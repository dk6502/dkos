.text
.globl interrupt_stub
interrupt_stub:
  call interrupt_dispatch
  iretq
