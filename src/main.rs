#![feature(lang_items)]
#![feature(asm)]
#![no_main]
#![no_std]

mod rcc;
mod gpio;

#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
  rcc::enable(rcc::Periph::apb2_gpioa);

  let gpioa = gpio::port(gpio::Port::A);

  // Set GPIOA pin 5's mode to output/push-pull
  gpioa.set_pin_mode(5, gpio::PinMode::OutPP);

  loop {
    gpioa.enable_pin(5);
    for _ in 0..10_000 {}
    gpioa.disable_pin(5);
    for _ in 0..10_000 {}
  }
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
  #[no_mangle]
  extern "C" fn panic_fmt() { loop {} }
}

/*
 * vi: ts=2:sw=2 expandtab
 */

