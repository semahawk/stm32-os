#![feature(lang_items)]
#![feature(asm)]
#![no_main]
#![no_std]

mod gpio;

#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
  /// Base address of the RCC block
  const RCC: u32 = 0x4002_1000;
  /// Address of the APB2ENR register
  const RCC_APB2ENR: u32 = RCC + 0x18;
  /// Mask of the bit that is in charge of enabling/disabling the GPIOA port
  const RCC_APB2ENR_IOPAEN: u32 = 1 << 2;

  unsafe {
    // Enable the GPIO port A block
    *(RCC_APB2ENR as *mut u32) |= RCC_APB2ENR_IOPAEN;
  }

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

