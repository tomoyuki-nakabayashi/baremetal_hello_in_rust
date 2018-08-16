global start

section .text
bits 64

start:
  Mov Rax, 0

  ; Call the Rust
  extern rust_main
  jmp rust_main

  ; Never reach here, now.
  hlt