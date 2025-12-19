#![no_std]
#![no_main]

const CHAR_WIDTH: usize = 12;
const CHAR_HEIGHT: usize = 24;

use core::arch::asm;
use core::fmt::{self, Write};

use limine::BaseRevision;
use limine::framebuffer::Framebuffer;
use limine::request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker};
use spleen_font::{FONT_12X24, PSF2Font};
use uart_16550::SerialPort;

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

trait Display {
  unsafe fn write_pixel(&self, code: u32, x: u64, y: u64);
}

impl<'a> Display for Framebuffer<'a> {
  unsafe fn write_pixel(&self, color: u32, x: u64, y: u64) {
    let pixel_offset = y * self.pitch() + x * 4;
    unsafe {
      self
        .addr()
        .add(pixel_offset as usize)
        .cast::<u32>()
        .write(color);
    }
  }
}

struct Writer<'a> {
  framebuffer: Option<&'a Framebuffer<'a>>,
  font: Option<&'a mut PSF2Font<'a>>,
  col_x: usize,
  row_y: usize,
}

impl<'a, 'b> Writer<'a>
where
  'a: 'b,
{
  fn new(framebuffer: Option<&'a Framebuffer<'b>>, font: Option<&'a mut PSF2Font<'b>>) -> Self {
    Self {
      framebuffer,
      font,
      col_x: 0,
      row_y: 0,
    }
  }
}

impl<'a> fmt::Write for Writer<'a> {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    let mut tmp = [0u8, 2];
    for char in s.chars() {
      let bytes = char.encode_utf8(&mut tmp).as_bytes();
      if bytes == &[0x000A_u8][..] {
        self.row_y += 1;
        self.col_x = 0;
      } else {
        let _ = self.write_char(char);
        self.col_x += 1;
      }
    }
    Ok(())
  }

  fn write_char(&mut self, text: char) -> Result<(), core::fmt::Error> {
    let mut tmp = [0u8, 2];
    let bytes = text.encode_utf8(&mut tmp).as_bytes();
    if let Some(framebuffer) = self.framebuffer
      && let Some(ref mut font) = self.font
    {
      if let Some(glyph) = font.glyph_for_utf8(bytes) {
        for (row_y, row) in glyph.enumerate() {
          for (col_x, on) in row.enumerate() {
            unsafe {
              if on {
                framebuffer.write_pixel(
                  0xFFFFFFFF,
                  (self.col_x * CHAR_WIDTH + col_x) as u64,
                  (self.row_y * CHAR_HEIGHT + row_y) as u64,
                );
              }
            }
          }
        }
      }
    }

    Ok(())
  }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
  assert!(BASE_REVISION.is_supported());
  let mut serial_port = unsafe { SerialPort::new(SERIAL_IO_PORT) };
  let mut font = PSF2Font::new(FONT_12X24).unwrap();
  serial_port.init();
  if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
    if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
      let mut writer = Writer::new(Some(&framebuffer), Some(&mut font));
      let _ = writeln!(writer, "dkos 0.0.1");
      let _ = writeln!(writer, "{:?}", 67.0 / 61.0);
      let _ = writeln!(writer, "{:?}", framebuffer.addr());
    }
  }
  hcf();
}

unsafe fn iret() {}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
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
