#![feature(lang_items)]
#![feature(asm)]
#![no_main]
#![no_std]

#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
  unsafe {
    let sram_boundary = *(0x0000_0000 as *const u32);
    let crash = *(sram_boundary as *const u32);
  }

  loop {}
}

mod exception {
  pub extern "C" fn dummy_handler() {
    unsafe { asm!("bkpt"); }
    loop {}
  }

  #[export_name = "_EXCEPTIONS"]
  pub static EXCEPTIONS: [Option<extern "C" fn()>; 14] = [
    Some(dummy_handler), // NMI
    Some(dummy_handler), // Hard fault
    Some(dummy_handler), // Memmanage fault
    Some(dummy_handler), // Bus fault
    Some(dummy_handler), // Usage fault
    None, // Reserved
    None, // Reserved
    None, // Reserved
    None, // Reserved
    Some(dummy_handler), // SVC call
    None, // Reserved for debug
    None, // Reserved
    Some(dummy_handler), // PendSV
    Some(dummy_handler), // Systick
  ];
}

mod lang_items {
  #[lang = "panic_fmt"]
  extern "C" fn panic_fmt() {}
}

/*
 * vi: ts=2:sw=2 expandtab
 */

