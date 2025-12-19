#![no_std]
#![no_main]

const CHAR_WIDTH: usize = 12;
const CHAR_HEIGHT: usize = 24;

use core::arch::asm;
use core::fmt::{self, Write};

use lazy_static::lazy_static;
use limine::BaseRevision;
use limine::framebuffer::Framebuffer;
use limine::request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker};
use spleen_font::{FONT_12X24, PSF2Font};
use x86_64::instructions::tables::load_tss;
use x86_64::registers::segmentation::{CS, SS, Segment};
use x86_64::structures::gdt::{self, Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;

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

struct Selectors {
  code_selector: SegmentSelector,
  tss_selector: SegmentSelector,
  data_selector: SegmentSelector,
}

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
  font: &'a mut PSF2Font<'a>,
  col_x: usize,
  row_y: usize,
}

impl<'a, 'b> Writer<'a>
where
  'a: 'b,
{
  fn new(framebuffer: Option<&'a Framebuffer<'b>>, font: &'a mut PSF2Font<'b>) -> Self {
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
        self.write_char(char);
        self.col_x += 1;
      }
    }
    Ok(())
  }

  fn write_char(&mut self, text: char) -> Result<(), core::fmt::Error> {
    let mut tmp = [0u8, 2];

    if let Some(glyph) = self
      .font
      .glyph_for_utf8(text.encode_utf8(&mut tmp).as_bytes())
      && let Some(framebuffer) = self.framebuffer
    {
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
    Ok(())
  }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
  assert!(BASE_REVISION.is_supported());
  if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
    if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
      let mut font = PSF2Font::new(FONT_12X24).unwrap();
      let mut writer = Writer::new(Some(&framebuffer), &mut font);
      writeln!(writer, "dkos 0.0.1");
      writeln!(writer, "{:?}", 67.0 / 61.0);
      writeln!(writer, "{:?}", framebuffer.addr());
    }
  }
  hcf();
}

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
