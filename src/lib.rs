#![feature(panic_implementation)]
#![feature(asm)]
#![no_std]

extern crate volatile;
use volatile::Volatile;

static HELLO: &[u8] = b"Hello from Rust!";

#[no_mangle]
pub extern fn rust_main() {
  let uart16550 = 0x10000000 as *mut Volatile<u8>;

  for &byte in HELLO.iter() {
    unsafe {
      (*uart16550).write(byte);
    }
  }
}

use core::panic::PanicInfo;

#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}