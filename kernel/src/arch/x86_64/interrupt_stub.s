# this makes 256 functions, where the function name is matched w the rust function name.
# \+ is the index in the .rept macro
.rept 256
.text
.globl interrupt_stub\+
interrupt_stub\+:
  # the interrupt vector is the 1st argument
  movq $\+, %rdi
  # if the exception has an error code, pop it off the stack
  .if \+ > 9 && \+ < 15 || \+ == 17 || \+ == 21 || \+ == 8
  popq %rsi
  # otherwise add a dummy
  .else
  movq $0, %rsi
  .endif
  # call interrupt dispatch
  call interrupt_dispatch
  iretq
.endr
