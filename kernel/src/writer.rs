use core::fmt;

use limine::framebuffer::Framebuffer;
use spleen_font::PSF2Font;

use crate::display::Display;

pub struct Writer<'a> {
  framebuffer: Option<&'a Framebuffer<'a>>,
  font: Option<&'a mut PSF2Font<'a>>,
  col_x: usize,
  row_y: usize,
}

impl<'a, 'b> Writer<'a>
where
  'a: 'b,
{
  pub fn new(framebuffer: Option<&'a Framebuffer<'b>>, font: Option<&'a mut PSF2Font<'b>>) -> Self {
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
                  (self.col_x * font.width as usize + col_x) as u64,
                  (self.row_y * font.header_size as usize + row_y) as u64,
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
