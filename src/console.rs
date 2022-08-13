use noto_sans_mono_bitmap::{get_bitmap, BitmapHeight, FontWeight};

static FONT_HEIGHT: usize = 18;
static FONT_WEIGHT: usize = 11;

#[derive(Clone, Copy)]
pub struct FrameBuffer {
    base_address: *mut u32,
    _size: usize,
    width: usize,
    height: usize,
    stride: usize,
}

pub struct Console {
    framebuffer: FrameBuffer,
    pub row: usize,
    pub col: usize,
    pub height: usize,
    pub width: usize,
}

impl FrameBuffer {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn clear(&mut self) {
        //! Clears Screen
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel =
                    unsafe { &mut *self.base_address.offset(((y * self.stride) + x) as isize) };
                *pixel = 0x00000000;
            }
        }
    }

    pub fn shift_vertical(&mut self, rows: usize) {
        //! Shift console vertically

        // Redraw every thing except first line on top
        for y in 0..(self.height - rows) {
            for x in 0..self.width {
                unsafe {
                    self.base_address
                        .offset((y * self.stride + x) as isize)
                        .write(
                            *self
                                .base_address
                                .offset(((y + rows) * self.stride + x) as isize),
                        );
                }
            }
        }

        // Clear last line
        for y in (self.height - rows)..self.height {
            for x in 0..self.width {
                unsafe {
                    self.base_address
                        .offset((y * self.stride + x) as isize)
                        .write(0x00000000);
                }
            }
        }
    }

    pub fn draw_char(&mut self, character: char, x: usize, y: usize) {
        //! Draws character on screen

        let bitmap_char = get_bitmap(character, FontWeight::Regular, BitmapHeight::Size18)
            .expect("unsupported char");
        let pixel_data = bitmap_char.bitmap();
        for row in 0..FONT_HEIGHT {
            for col in 0..FONT_WEIGHT {
                let pixel = unsafe {
                    &mut *self
                        .base_address
                        .offset(((y + row) * self.stride + x + col) as isize)
                };
                let color = pixel_data[row][col];

                let rgb = [color, color, color];
                let code: u32 =
                    ((rgb[0] as u32) << 16) + ((rgb[1] as u32) << 8) + ((rgb[2] as u32) << 0);
                *pixel = code;
            }
        }
    }
}

impl Console {
    pub fn new(framebuffer: FrameBuffer) -> Self {
        //! Create a terminal
        let width = (framebuffer.width() / FONT_WEIGHT) - 1;
        let height = (framebuffer.height() / FONT_WEIGHT) - 1;
        Console {
            framebuffer: framebuffer,
            row: 0,
            col: 0,
            height: height,
            width: width,
        }
    }

    fn scroll(&mut self) {
        self.framebuffer.shift_vertical(FONT_HEIGHT);
    }

    pub fn write_char(&mut self, ch: char) {
        if ch == '\n' {
            self.row += 1;
            self.col = 0;
        } else {
            let y = self.row * FONT_HEIGHT;
            let x = self.col * FONT_WEIGHT;

            self.framebuffer.draw_char(ch, x, y);
            self.col += 1;
        }

        if self.col >= self.width {
            self.row += 1;
            self.col = 0;
        }

        if self.row >= self.height {
            self.scroll();
            self.row = self.height - 1;
        }
    }
}

impl core::fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.bytes() {
            self.write_char(ch as _);
        }
        Ok(())
    }
}
