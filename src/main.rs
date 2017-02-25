#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(asm)]
#![no_main]
#![no_std]

extern crate rlibc;

use core::fmt::Write;
use core::slice;
use core::str;

use usart::Usart_trait;
use gpio::Gpio_trait;

#[macro_use]
mod usart;
mod rcc;
mod gpio;
mod mmio;
mod conf;
mod cmd;
mod timer;

#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
  rcc::initialize_clocks();

  move_data_section_to_ram();

  rcc::enable(rcc::Periph::apb2_gpioa);
  rcc::enable(rcc::Periph::apb2_afio);
  rcc::enable(rcc::Periph::apb1_usart2);
  rcc::enable(rcc::Periph::apb1_tim2);

  // Initialize the 'heartbeet' timer (1 million milliseconds for a 1 second period)
  timer::TIM2.enable(1_000);

  // Initialize USART2 (the one that goes through the debugger/the USB cable)
  usart::USART2.initialize(usart::Baudrate::_115200);

  usart::output_to(usart::USART2);

  // Set the LED pin as output/push-pull
  gpio::GPIOA.set_pin_mode(5, gpio::PinMode::OutPP);

  print!("Clocks initialized\r\n");
  print!("SYSCLK = {} Hz\r\n", rcc::get_clock_speed(rcc::Clock::SYSCLK));
  print!("HCLK   = {} Hz\r\n", rcc::get_clock_speed(rcc::Clock::HCLK));
  print!("PCLK1  = {} Hz\r\n", rcc::get_clock_speed(rcc::Clock::PCLK1));
  print!("PCLK2  = {} Hz\r\n", rcc::get_clock_speed(rcc::Clock::PCLK2));
  print!("\r\n");
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
  use timer;
  pub extern "C" fn dummy_handler() {
    unsafe { asm!("bkpt"); }
    loop {}
  }

  #[export_name = "_EXCEPTIONS"]
  pub static EXCEPTIONS: [Option<extern "C" fn()>; 45] = [
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
    Some(dummy_handler), // Window Watchdog
    Some(dummy_handler), // PVD through EXTI line detection
    Some(dummy_handler), // Tamper
    Some(dummy_handler), // RTC global interrupt
    Some(dummy_handler), // Flash global interrupt
    Some(dummy_handler), // RCC global interrupt
    Some(dummy_handler), // EXTI line 0 interrupt
    Some(dummy_handler), // EXTI line 1 interrupt
    Some(dummy_handler), // EXTI line 2 interrupt
    Some(dummy_handler), // EXTI line 3 interrupt
    Some(dummy_handler), // EXTI line 4 interrupt
    Some(dummy_handler), // DMA1 channel 1
    Some(dummy_handler), // DMA1 channel 2
    Some(dummy_handler), // DMA1 channel 3
    Some(dummy_handler), // DMA1 channel 4
    Some(dummy_handler), // DMA1 channel 5
    Some(dummy_handler), // DMA1 channel 6
    Some(dummy_handler), // DMA1 channel 7
    Some(dummy_handler), // ADC1 and ADC2
    Some(dummy_handler), // CAN1 TX interrupt
    Some(dummy_handler), // CAN1 RX0 interrupt
    Some(dummy_handler), // CAN1 RX1 interrupt
    Some(dummy_handler), // CAN1 SCE interrupt
    Some(dummy_handler), // EXTI line[9:5] interrupts
    Some(dummy_handler), // TIM1 break interrupt
    Some(dummy_handler), // TIM1 update interrupt
    Some(dummy_handler), // TIM1 trigger and communication interrupts
    Some(dummy_handler), // TIM1 capture compare interrupt
    Some(timer::TIM2.irq_handler), // TIM2 global interrupt
    Some(dummy_handler), // TIM3 global interrupt
    Some(dummy_handler), // TIM4 global interrupt
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

