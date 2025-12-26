#![no_std]
#![no_main]

use core::arch::asm;
use core::fmt::Write;

use crate::writer::WRITER;
use lazy_static::lazy_static;
use limine::BaseRevision;
use limine::request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker};
use spin::mutex::Mutex;
use spleen_font::{FONT_12X24, PSF2Font};
use uart_16550::SerialPort;

mod display;
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

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
  assert!(BASE_REVISION.is_supported());
  let mut serial_port = unsafe { SerialPort::new(SERIAL_IO_PORT) };
  serial_port.init();
  let _ = writeln!(serial_port, "dkos 0.1.0");
  let _ = writeln!(WRITER.lock(), "dkos 0.1.");

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
