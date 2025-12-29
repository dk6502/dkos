.text
.globl interrupt_stub
interrupt_stub:
  pushq %rax
  pushq $0
  call interrupt_dispatch
  addq $16, %rsp
  iretq
