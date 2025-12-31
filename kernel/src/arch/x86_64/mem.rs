use core::arch::asm;

use crate::{println, requests::HHDM_RESPONSE};

pub static PML4: Pml4 = Pml4([0xFFFFFFFF; 512]);

pub fn init_paging() {
  let virt_addr: u64;
  let phys_addr: u64;
  unsafe {
    virt_addr = &PML4.0 as *const [u64; 512] as u64;
    phys_addr = virt_addr - HHDM_RESPONSE.offset();
    asm!("
      mov cr3, {}
    ", in(reg) phys_addr);
    println!("hi!!");
  }
}

#[repr(align(4096))]
struct Pml4([u64; 512]);
