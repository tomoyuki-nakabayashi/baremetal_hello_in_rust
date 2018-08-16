#![feature(panic_implementation)]
#![feature(asm)]
#![no_std]

#[no_mangle]
pub extern fn rust_main() {
  unsafe {
    asm!("mov rax, 0x10000000" :::: "intel");
    asm!("mov rbx, 0x48" :::: "intel");
    asm!("mov [rax], rbx" :::: "intel");
    asm!("mov rbx, 0x65" :::: "intel");
    asm!("mov [rax], rbx" :::: "intel");
    asm!("mov rbx, 0x6c" :::: "intel");
    asm!("mov [rax], rbx" :::: "intel");
    asm!("mov rbx, 0x6c" :::: "intel");
    asm!("mov [rax], rbx" :::: "intel");
    asm!("mov rbx, 0x6f" :::: "intel");
    asm!("mov [rax], rbx" :::: "intel");
    asm!("mov rbx, 0x0a" :::: "intel");
    asm!("mov [rax], rbx" :::: "intel");
  }
}

use core::panic::PanicInfo;

#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}