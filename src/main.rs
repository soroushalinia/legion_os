#![no_std]
#![no_main]

mod console;

use console::FrameBuffer;
use core::fmt::Write;
use core::panic::PanicInfo;
use raw_cpuid;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn _start(frame_buffer: &mut FrameBuffer, _mem_map_buf: &mut [u8]) -> ! {
    frame_buffer.clear();
    let mut console = console::Console::new(*frame_buffer);
    let cpuid = raw_cpuid::CpuId::new();
    let cpu_string = cpuid.get_processor_brand_string().unwrap();
    writeln!(console, "Legion OS v0.1.0").unwrap();
    writeln!(
        console,
        "Resolution: {}x{}",
        frame_buffer.width(),
        frame_buffer.height()
    )
    .unwrap();
    writeln!(console, "CPU: {}", cpu_string.as_str()).unwrap();
    loop {}
}
