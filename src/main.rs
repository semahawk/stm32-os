#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(asm)]
#![no_main]
#![no_std]

extern crate rlibc;

mod usart;
mod rcc;
mod gpio;
mod mmio;
mod conf;

#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
  rcc::initialize_clocks();

  rcc::enable(rcc::Periph::apb2_gpioa);
  rcc::enable(rcc::Periph::apb2_afio);
  rcc::enable(rcc::Periph::apb1_usart2);

  let gpioa = gpio::port(gpio::Port::A);

  let usart2 = usart::new(usart::Port::Usart2, usart::Baudrate::_115200);

  // Set the LED pin as output/push-pull
  gpioa.set_pin_mode(5, gpio::PinMode::OutPP);

  // Set the USART pins
  gpioa.set_pin_mode(2, gpio::PinMode::OutAltPP);
  gpioa.set_pin_mode(3, gpio::PinMode::InFloat);

  usart2.send_string("Clocks initialized (SYSCLK = 36MHz)\r\n");
  usart2.send_string("Available commands are 'blink' and 'hello'\r\n");

  loop {
    let mut buf = [0u8; 32];
    usart2.send_string(": ");

    usart2.get_string(&mut buf);

    if buf.starts_with(b"blink") {
      gpioa.enable_pin(5);
      for _ in 0..10_000 {  };
      gpioa.disable_pin(5);
      for _ in 0..10_000 {  };
    } else if buf.starts_with(b"hello") {
      usart2.send_string("well hello to you too!\r\n");
    }
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

