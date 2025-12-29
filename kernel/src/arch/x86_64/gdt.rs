use core::arch::asm;

use crate::println;

/// The static GDT
pub static GDT: [u64; 3] = {
  // The entries here are u64s b/c they will stay unchanged
  let gdt = [
    // This is the initial null GDT segment.
    0,
    // This is the kernel code segment
    0x00af9b000000ffff,
    // This is the kernel data segment
    0x00cf93000000ffff,
  ];
  gdt
};

/// Self-explanatory. This initializes the GDT via `lgdt` and then reloads the segment registers.
pub fn init_gdt() {
  let gp = GlobalDescriptorTablePointer {
    limit: (size_of_val(&GDT) - 1) as u16,
    base: &GDT as *const _ as u64,
  };
  unsafe {
    asm!(
      "
      lgdt [{}]
      push 0x08
      lea rax, [rip +  2f]
      push rax
      retfq
      2:
      mov ax, 0x10
      mov ds, ax
      mov es, ax
      mov fs, ax
      mov gs, ax
      mov ss, ax
      "
      , in(reg) &gp)
  }
  println!("GDT init OK");
}

/// Struct that should be referenced in `lgdt`
#[repr(C, packed)]
struct GlobalDescriptorTablePointer {
  limit: u16,
  base: u64,
}
