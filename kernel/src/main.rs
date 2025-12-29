#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::arch::asm;
use core::fmt::Write;

use crate::gdt::{reload_segments, set_gdt};
use crate::idt::lidt;
use crate::writer::WRITER;
use limine::BaseRevision;
use limine::request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker};
use uart_16550::SerialPort;

mod display;
mod gdt;
mod idt;
mod writer;

// #[used] lets the compiler know not to remove
#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

const SERIAL_IO_PORT: u16 = 0x3F8;

fn divide_by_zero() {
  unsafe { asm!("mov dx, 0; div dx") }
}

#[allow(unused)]
fn breakpoint() {
  unsafe { asm!("2: jmp 2b") }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
  assert!(BASE_REVISION.is_supported());
  let mut serial_port = unsafe { SerialPort::new(SERIAL_IO_PORT) };
  serial_port.init();
  let _ = writeln!(WRITER.lock(), "dkos 0.1.0");
  set_gdt();
  reload_segments();
  let _ = writeln!(WRITER.lock(), "GDT init OK");
  lidt();
  let _ = writeln!(WRITER.lock(), "IDT init OK");

  unsafe { asm!("int3") }
  let _ = writeln!(WRITER.lock(), "IDT init OK 2");
  divide_by_zero();
  hcf();
}
#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
  let _ = writeln!(WRITER.lock(), "{}", info);
  hcf();
}

fn hcf() -> ! {
  loop {
    unsafe {
      #[cfg(target_arch = "x86_64")]
      asm!("hlt");
    }
  }
}
