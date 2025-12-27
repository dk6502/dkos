.global flush_gdt
set_gdt:
  .extern(GP)
  lgdt [GP]

reload_segments:
  push 0x08
  lea rax, [rel .reload_cs]
  push rax
  retfq
.reload_cs:
  mov ax, 0x10
  mov ds, ax
  mov es, ax
  mov fs, ax
  mov gs, ax
  ret
