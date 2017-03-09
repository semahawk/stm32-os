#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(asm)]
#![no_main]
#![no_std]

extern crate rlibc;

use core::fmt::Write;
use core::slice;
use core::str;

#[macro_use]
mod usart;
mod rcc;
mod gpio;
mod mmio;
mod conf;
mod cmd;
mod spi;
mod mcp23s08;

#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
  rcc::initialize_clocks();

  move_data_section_to_ram();

  rcc::enable(rcc::Periph::apb2_gpioa);
  rcc::enable(rcc::Periph::apb2_gpiob);
  rcc::enable(rcc::Periph::apb2_gpioc);
  rcc::enable(rcc::Periph::apb2_gpiod);
  rcc::enable(rcc::Periph::apb2_gpioe);
  rcc::enable(rcc::Periph::apb2_gpiof);
  rcc::enable(rcc::Periph::apb2_gpiog);
  rcc::enable(rcc::Periph::apb2_afio);
  rcc::enable(rcc::Periph::apb2_spi1);
  rcc::enable(rcc::Periph::apb1_usart2);

  // Initialize USART2 (the one that goes through the debugger/the USB cable)
  usart::USART2.initialize(usart::Baudrate::_115200);

  usart::output_to(usart::USART2);

  // Configure SPI1
  spi::SPI1.initialize();

  print!("Clocks initialized\r\n");
  print!("SYSCLK = {} Hz\r\n", rcc::get_clock_speed(rcc::Clock::SYSCLK));
  print!("HCLK   = {} Hz\r\n", rcc::get_clock_speed(rcc::Clock::HCLK));
  print!("PCLK1  = {} Hz\r\n", rcc::get_clock_speed(rcc::Clock::PCLK1));
  print!("PCLK2  = {} Hz\r\n", rcc::get_clock_speed(rcc::Clock::PCLK2));
  print!("\r\n");

  print!("Using MCP23S08 through SPI1 to enable port GP0\r\n");
  mcp23s08::write_reg(spi::SPI1, mcp23s08::IODIR, !0x01);
  mcp23s08::write_reg(spi::SPI1, mcp23s08::OLAT, 0x01);

  print!("Available command is 'gpio <set|clear> <port> <pin>'\r\n");

  loop {
    let mut buf = [0u8; 32];
    print!(": ");

    usart::USART2.get_string(&mut buf);

    let input = unsafe {
      str::from_utf8_unchecked(slice::from_raw_parts(buf.as_ptr(), buf.len()))
    };

    let mut args = input.split('\u{0}').next().unwrap().split(' ');

    match args.next() {
      Some(command) => {
        match cmd::lookup_command(command) {
          Some(handler) => {
            handler(args);
          },
          None => {
            print!("Unknown command: {}\r\n", command);
          },
        }
      },
      None => (),
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

fn move_data_section_to_ram() {
  // oh man..
  extern {
    static data_in_flash_start: u32;
    static data_in_ram_start: u32;
    static data_in_ram_end: u32;
  }

  let from = &data_in_flash_start as *const u32;
  let mut to = &data_in_ram_start as *const u32;
  let mut size = &data_in_ram_end as *const u32 as u32 - &data_in_ram_start as *const u32 as u32;

  for i in 0..size {
    unsafe {
      core::ptr::write((&mut to as *mut _ as u32 + i) as *mut u8,
              *((from as u32 + i) as *const u8));
    }
  }
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

