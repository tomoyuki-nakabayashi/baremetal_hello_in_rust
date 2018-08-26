#![feature(panic_implementation)]
#![feature(asm)]
#![no_std]

static HELLO: &[u8] = b"Hello from Rust!";

#[no_mangle]
pub extern fn rust_main() {
  let vga_buffer = 0xb8000 as *mut u16;

  for (i, &byte) in HELLO.iter().enumerate() {
    unsafe {
      *vga_buffer.offset(i as isize) = 0x3u16 << 8 | byte as u16;
    }
  }
}

use core::panic::PanicInfo;
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
