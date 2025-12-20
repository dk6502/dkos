use limine::framebuffer::Framebuffer;

pub trait Display {
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
