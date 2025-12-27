use core::arch::global_asm;

use lazy_static::lazy_static;

lazy_static! {
  static ref GDT: GlobalDescriptorTable = {
    let gdt = [
      GlobalDescriptorTableEntry::new(0, 0, 0, 0),
      GlobalDescriptorTableEntry::new(0, 0xFFFFFFFF, 0x9A, 0xCF),
      GlobalDescriptorTableEntry::new(0, 0xFFFFFFFF, 0x92, 0xCF),
    ];
    gdt
  };
  static ref GP: GlobalDescriptorTablePointer = {
    GlobalDescriptorTablePointer {
      limit: (size_of::<GlobalDescriptorTable>() * 3 - 1) as u16,
      base: &GDT as *const _ as u32,
    }
  };
}

pub extern "C" fn set_gdt() {}
pub extern "C" fn reload_segments() {}

type GlobalDescriptorTable = [GlobalDescriptorTableEntry; 3];

#[repr(C, packed)]
struct GlobalDescriptorTableEntry {
  limit_low: u16,
  base_low: u16,
  base_middle: u8,
  access: u8,
  granularity: u8,
  base_high: u8,
}

impl GlobalDescriptorTableEntry {
  fn new(base: u32, limit: u32, access: u8, gran: u8) -> Self {
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
  base: u32,
}
