#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::arch::asm;

use limine::BaseRevision;
use limine::request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker};
use uart_16550::SerialPort;

mod arch;
mod fbcon;

// #[used] lets the compiler know not to remove these
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

/// Debugging function that tests the #DE CPU exception
#[allow(unused)]
fn divide_by_zero() {
  unsafe { asm!("mov dx, 0; div dx") }
}

/// Debugger breakpoint that can be broken out of in GDB with `set $pc += 2`
#[allow(unused)]
fn breakpoint() {
  unsafe { asm!("2: jmp 2b") }
}

/// Halt & catch fire
fn hcf() -> ! {
  loop {
    unsafe {
      #[cfg(target_arch = "x86_64")]
      asm!("hlt");
    }
  }
}

/// This is the kernel entrypoint
#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
  assert!(BASE_REVISION.is_supported());
  let mut serial_port = unsafe { SerialPort::new(SERIAL_IO_PORT) };
  serial_port.init();
  println!("dkos 0.1.0");
  #[cfg(target_arch = "x86_64")]
  {
    arch::x86_64::gdt::init_gdt();
    arch::x86_64::idt::init_idt();
    unsafe { asm!("int3") }
  }
  divide_by_zero();

  hcf();
}

/// Custom panic handler
#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
  println!("{}", info);
  hcf();
}
