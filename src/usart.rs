//
// usart.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 17 Feb 2017 18:08:54 +0100 (CET)
//

use core::fmt;

use rcc;
use gpio;

/// Read data register not empty (data ready to be read)
const USART_SR_RXNE: u32 = 1 << 5;
/// Transmission complete
const USART_SR_TC: u32 = 1 << 6;
/// Transmitter data register empty (ie. can send bytes?)
const USART_SR_TXE: u32 = 1 << 7;
/// UART enable bit
const USART_CR1_UE: u32 = 1 << 13;
/// Selects the word length:
///   0 - 1 start bit, 8 data bits, n stop bit
///   1 - 1 start bit, 9 data bits, n stop bit
const USART_CR1_M: u32 = 1 << 1;
/// Receiver enable
const USART_CR1_RE: u32 = 1 << 2;
/// Transmitter enable
const USART_CR1_TE: u32 = 1 << 3;

#[repr(packed)]
struct Usart_register_map {
  SR:   u32,
  DR:   u32,
  BRR:  u32,
  CR1:  u32,
  CR2:  u32,
  CR3:  u32,
  GTPR: u32,
}

/// Base address + the peripheral clock
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Usart(u32, rcc::Clock);

pub const USART1: Usart = Usart(0x4001_3800, rcc::Clock::PCLK2);
pub const USART2: Usart = Usart(0x4000_4400, rcc::Clock::PCLK1);
pub const USART3: Usart = Usart(0x4000_4800, rcc::Clock::PCLK1);
// TODO: UART4 and UART5

pub static mut current: Option<Usart> = Some(USART2);

#[derive(Copy,Clone)]
pub enum Baudrate {
  _9600   = 9600,
  _115200 = 115200,
}

pub trait Usart_trait {
  fn initialize(self, baudrate: Baudrate);
  fn send_byte(&self, byte: u8);
  fn get_byte(&self) -> u8;
  fn get_string(&self, buf: &mut [u8]);
}

impl Usart_trait for Usart {
  fn initialize(self, baudrate: Baudrate) {
    let regmap = self.0 as *mut Usart_register_map;

    let mut usartdiv = 0;
    let mut clock_speed = rcc::get_clock_speed(self.1);

    while clock_speed >= 16 * baudrate as u32 {
      usartdiv += 1;
      clock_speed -= 16 * baudrate as u32;
    }

    usartdiv = usartdiv << 4;

    unsafe {
      // Set the hardware flow control (0x0 is the reset value but what the hell)
      (*regmap).CR2 = 0x0;

      // Enable transmission and reception
      (*regmap).CR1 |= USART_CR1_TE | USART_CR1_RE;

      // Actually set the baud rate (it's not perfect since the fractional bit is not taken into
      // account)
      (*regmap).BRR = usartdiv;

      // Enable the UART
      (*regmap).CR1 |= USART_CR1_UE;
    }

    if self == USART2 {
      // Set the USART pins
      gpio::GPIOA.set_pin_mode(2, gpio::PinMode::OutAltPP);
      gpio::GPIOA.set_pin_mode(3, gpio::PinMode::InFloat);
    }
  }

  fn send_byte(&self, byte: u8) {
    let regmap = self.0 as *mut Usart_register_map;

    // Wait until there's space for transmission
    unsafe {
      while (*regmap).SR & USART_SR_TXE == 0 {}

      // Actually transmit the data
      (*regmap).DR = byte as u32;

      // Wait until the transmission is complete
      while (*regmap).SR & USART_SR_TC != 0 {}
    }
  }

  fn get_byte(&self) -> u8 {
    let regmap = self.0 as *mut Usart_register_map;

    unsafe {
      while (*regmap).SR & USART_SR_RXNE == 0 {}

      (*regmap).DR as u8
    }
  }

  fn get_string(&self, buf: &mut [u8]) {
    let mut i = 0;

    loop {
      let byte = self.get_byte();
      if i >= buf.len() { break; }

      if byte == 0x8 || byte == 0x7f {
        self.send_byte(0x8);  // backspace
        self.send_byte(0x20); // space
        self.send_byte(0x8);  // backspace again
        i -= 1;
        buf[i] = 0x0;
        continue;
      }

      if byte == 0xa || byte == 0xd {
        self.send_byte(0xa);
        self.send_byte(0xd);
        break;
      }

      self.send_byte(byte);

      buf[i] = byte;
      i += 1;
    }
  }
}

impl fmt::Write for Usart {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    for byte in s.bytes() {
      self.send_byte(byte)
    }

    Ok(())
  }
}

pub fn output_to(mut usart: Usart) {
  unsafe {
    current = Some(usart);
  }
}

macro_rules! print {
  ($($arg:tt)*) => ({
    use core::fmt::Write;
    unsafe {
      match $crate::usart::current {
        Some(ref mut usart) => {
          usart.write_fmt(format_args!($($arg)*)).unwrap();
        },
        None => (),
      }
    }
  });
}

/*
 * vi: ts=2 sw=2 expandtab
 */

