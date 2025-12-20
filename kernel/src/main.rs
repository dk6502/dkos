#![no_std]
#![no_main]

use core::arch::asm;
use core::fmt::Write;

use crate::display::Display;
use crate::writer::Writer;
use limine::BaseRevision;
use limine::request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker};
use spleen_font::{FONT_12X24, PSF2Font};
use uart_16550::SerialPort;
use x86_64::structures::gdt::{self, GlobalDescriptorTable};

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
  let mut font = PSF2Font::new(FONT_12X24).unwrap();
  serial_port.init();
  if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
    if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
      let mut writer = Writer::new(Some(&framebuffer), Some(&mut font));
      let _ = writeln!(writer, "dkos 0.1.0");
      let _ = writeln!(writer, "{:?}", 67.0 / 61.0);
      let _ = writeln!(writer, "{:?}", framebuffer.addr());
    }
  }
  let _ = writeln!(serial_port, "dkos 0.1.0");
  hcf();
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
  if let Some(framebuffer_request) = FRAMEBUFFER_REQUEST.get_response() {
    if let Some(framebuffer) = framebuffer_request.framebuffers().next() {
      for i in 0..framebuffer.width() {
        for j in 0..framebuffer.height() {
          unsafe {
            framebuffer.write_pixel(0xFF0000FF, i, j);
          }
        }
      }
      let mut font = PSF2Font::new(FONT_12X24).unwrap();
      let mut writer = Writer::new(Some(&framebuffer), Some(&mut font));
      let _ = write!(writer, "{}", info);
    }
  }
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
