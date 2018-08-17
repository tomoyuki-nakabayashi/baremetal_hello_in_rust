global start

section .text
bits 64

start:
  Mov Rsp, 0x0100

  ; Call the Rust
  extern rust_main
  call rust_main

  ; Never reach here, now.
  hlt
