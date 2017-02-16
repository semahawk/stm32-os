#![feature(lang_items)]
#![no_main]
#![no_std]

#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
  let y;
  let x = 42;

  y = x;

  loop {}
}

#[export_name = "_nmi_isr"]
pub extern "C" fn nmi_isr() -> ! {
  loop {}
}

#[export_name = "_hard_fault_isr"]
pub extern "C" fn hard_fault_isr() -> ! {
  loop {}
}

mod lang_items {
  #[lang = "panic_fmt"]
  extern "C" fn panic_fmt() {}
}

/*
 * vi: ts=2:sw=2 expandtab
 */

