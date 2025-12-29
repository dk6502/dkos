use core::arch::asm;

pub static GDT: [u64; 3] = {
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

pub fn set_gdt() {
  let gp = GlobalDescriptorTablePointer {
    limit: (size_of_val(&GDT) - 1) as u16,
    base: &GDT as *const _ as u64,
  };
  unsafe {
    asm!(
      "
      lgdt [{}]
      "
      , in(reg) &gp)
  }
}
pub fn reload_segments() {
  unsafe {
    asm!(
      "
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
    )
  }
}

#[repr(C, packed)]
pub struct GlobalDescriptorTableEntry {
  limit_low: u16,
  base_low: u16,
  base_middle: u8,
  access: u8,
  granularity: u8,
  base_high: u8,
}

impl GlobalDescriptorTableEntry {
  const fn new(base: u32, limit: u32, access: u8, gran: u8) -> Self {
    let mut granularity: u8;
    granularity = ((limit >> 16) & 0x0F) as u8;
    granularity |= gran & 0xF0;
    Self {
      limit_low: (limit & 0xFFFF) as u16,
      base_low: (base & 0xFFFF) as u16,
      base_middle: ((base >> 16) & 0xFF) as u8,
      access,
      granularity,
      base_high: ((base >> 24) & 0xFF) as u8,
    }
  }
}

#[repr(C, packed)]
struct GlobalDescriptorTablePointer {
  limit: u16,
  base: u64,
}
