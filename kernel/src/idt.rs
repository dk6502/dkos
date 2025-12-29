use crate::writer::WRITER;
use core::arch::asm;
use core::fmt::Write;

pub type HandlerFunc = extern "C" fn() -> !;
static mut IDT: [InterruptDescriptorTableEntry; 32] = [InterruptDescriptorTableEntry::empty(); 32];

pub fn lidt() {
  unsafe {
    IDT[0] = InterruptDescriptorTableEntry::new(divide_by_zero_handler, EntryOptions::new());
  }
  let idtp = InterruptDescriptorTablePtr {
    limit: (size_of::<InterruptDescriptorTableEntry>() * 32 - 1) as u16,
    base: &raw const IDT,
  };
  let _ = writeln!(WRITER.lock(), "{:?}", idtp);
  unsafe {
    let _ = writeln!(WRITER.lock(), "{:?}", IDT[0]);

    asm!(
      "
      lidt [{}]
      ",
      in(reg) &idtp
    )
  }
}

#[derive(Debug, Clone, Copy)]
pub struct EntryOptions(pub u16);

impl EntryOptions {
  const fn minimal() -> Self {
    let mut options = 0;
    options |= 0b111 << 9;
    Self(options)
  }

  const fn new() -> Self {
    let mut options = Self::minimal();
    options.set_present(true).disable_interrupts(true);
    options
  }

  const fn set_present(&mut self, present: bool) -> &mut Self {
    self.0 = (present as u16) << 15;
    self
  }

  const fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
    self.0 = (!disable as u16) << 8;
    self
  }

  const fn set_privilege_level(&mut self, dpl: u16) -> &mut Self {
    let dpl = dpl & 0b111;
    self.0 = dpl << 13;
    self
  }
}

#[repr(C, packed)]
#[derive(Debug)]
struct InterruptDescriptorTablePtr {
  limit: u16,
  base: *const [InterruptDescriptorTableEntry; 32],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct InterruptDescriptorTableEntry {
  offset_low: u16,
  selector: u16,
  options: EntryOptions,
  offset_mid: u16,
  offset_high: u32,
  reserved: u32,
}
impl InterruptDescriptorTableEntry {
  fn new(handler: HandlerFunc, options: EntryOptions) -> Self {
    let offset = handler as u64;
    let selector = 0x8;
    Self {
      offset_low: offset as u16,
      selector,
      options: options,
      offset_mid: (offset >> 16) as u16,
      offset_high: (offset >> 32) as u32,
      reserved: 0,
    }
  }
  const fn empty() -> Self {
    Self {
      offset_low: 0,
      selector: 0,
      options: EntryOptions::minimal(),
      offset_mid: 0,
      offset_high: 0,
      reserved: 0,
    }
  }
}

// Handler functions

extern "C" fn divide_by_zero_handler() -> ! {
  panic!("sure")
}
