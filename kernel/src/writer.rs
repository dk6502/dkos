use core::fmt;

use lazy_static::lazy_static;
use limine::framebuffer::Framebuffer;
use spin::mutex::Mutex;
use spleen_font::{FONT_12X24, PSF2Font};

use crate::{FRAMEBUFFER_REQUEST, display::Display};

lazy_static! {
  pub static ref WRITER: Mutex<Writer<'static>> = {
    let font = PSF2Font::new(FONT_12X24).unwrap();
    let framebuffer_response = FRAMEBUFFER_REQUEST.get_response().unwrap();
    let framebuffer = framebuffer_response.framebuffers().next().unwrap();
    let writer = Mutex::new(Writer::new(framebuffer, font));
    writer
  };
}

pub struct Writer<'a> {
  pub framebuffer: Framebuffer<'a>,
  pub font: PSF2Font<'a>,
  col_x: usize,
  row_y: usize,
}

impl<'a> Writer<'a> {
  pub fn new(framebuffer: Framebuffer<'a>, font: PSF2Font<'a>) -> Self {
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
    if let Some(glyph) = self.font.glyph_for_utf8(bytes) {
      for (row_y, row) in glyph.enumerate() {
        for (col_x, on) in row.enumerate() {
          unsafe {
            if on {
              self.framebuffer.write_pixel(
                0xFFFFFFFF,
                (self.col_x * self.font.width as usize + col_x) as u64,
                (self.row_y * self.font.header_size as usize + row_y) as u64,
              );
            }
          }
        }
      }
    }

    Ok(())
  }
}
