#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(asm)]
#![no_main]
#![no_std]

extern crate rlibc;

use core::fmt::Write;

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

  let mut usart2 = usart::new(usart::Port::Usart2, usart::Baudrate::_115200);

  // Set the LED pin as output/push-pull
  gpioa.set_pin_mode(5, gpio::PinMode::OutPP);

  // Set the USART pins
  gpioa.set_pin_mode(2, gpio::PinMode::OutAltPP);
  gpioa.set_pin_mode(3, gpio::PinMode::InFloat);

  write!(usart2, "Clocks initialized\r\n");
  write!(usart2, "SYSCLK = {} Hz\r\n", rcc::get_clock_speed(rcc::Clock::SYSCLK));
  write!(usart2, "HCLK   = {} Hz\r\n", rcc::get_clock_speed(rcc::Clock::HCLK));
  write!(usart2, "PCLK1  = {} Hz\r\n", rcc::get_clock_speed(rcc::Clock::PCLK1));
  write!(usart2, "PCLK2  = {} Hz\r\n", rcc::get_clock_speed(rcc::Clock::PCLK2));
  write!(usart2, "\r\n");
  write!(usart2, "Available commands are 'blink' and 'hello'\r\n");

  loop {
    let mut buf = [0u8; 32];
    write!(usart2, ": ");

    usart2.get_string(&mut buf);

    if buf.starts_with(b"blink") {
      gpioa.enable_pin(5);
      for _ in 0..10_000 {  };
      gpioa.disable_pin(5);
      for _ in 0..10_000 {  };
    } else if buf.starts_with(b"hello") {
      write!(usart2, "well hello to you too!\r\n");
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

#[no_mangle]
pub unsafe extern fn __aeabi_memclr4(s: *mut u8, n: usize) -> *mut u8 {
  let mut i = 0;

  while i < n {
    *s.offset(i as isize) = 0u8;
    i += 1;
  }

  return s;
}

mod lang_items {
  #[lang = "panic_fmt"]
  #[no_mangle]
  extern "C" fn panic_fmt() { loop {} }
}

/*
 * vi: ts=2:sw=2 expandtab
 */

