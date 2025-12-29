use crate::writer::WRITER;
use core::arch::asm;
use core::fmt::Write;

pub type HandlerFunc = unsafe extern "C" fn() -> !;
static mut IDT: [InterruptDescriptorTableEntry; 32] = [InterruptDescriptorTableEntry::empty(); 32];

#[repr(u8)]
enum GateType {
  Interrupt = 0x0E,
  Trap = 0x0F,
}

pub fn lidt() {
  unsafe {
    IDT[0] = InterruptDescriptorTableEntry::new(interrupt_stub, GateType::Interrupt);
    IDT[3] = InterruptDescriptorTableEntry::new(interrupt_stub, GateType::Trap);
  }
  let idtp = InterruptDescriptorTablePtr {
    limit: (size_of::<InterruptDescriptorTableEntry>() * 32 - 1) as u16,
    base: &raw const IDT,
  };
  unsafe {
    asm!(
      "
      lidt [{}]
      ",
      in(reg) &idtp
    )
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
  ist: u8,
  options: u8,
  offset_mid: u16,
  offset_high: u32,
  reserved: u32,
}
impl InterruptDescriptorTableEntry {
  fn new(handler: HandlerFunc, gate_type: GateType) -> Self {
    let offset = handler as u64;
    Self {
      offset_low: offset as u16,
      selector: 0x08,
      ist: 0,
      options: 0x80 | gate_type as u8,
      offset_mid: (offset >> 16) as u16,
      offset_high: (offset >> 32) as u32,
      reserved: 0,
    }
  }
  const fn empty() -> Self {
    Self {
      offset_low: 0,
      selector: 0,
      ist: 0,
      options: 0xE0,
      offset_mid: 0,
      offset_high: 0,
      reserved: 0,
    }
  }
}

// Handler functions

unsafe extern "C" {
  fn interrupt_stub() -> !;
}

#[unsafe(no_mangle)]
extern "C" fn interrupt_dispatch() {
  let _ = writeln!(WRITER.lock(), "INTERRUPT: CODE {}", "x");
}
