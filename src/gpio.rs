//
// gpio.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 16 Feb 2017 21:36:10 +0100 (CET)
//

#[derive(Debug, Clone, Copy)]
pub enum PinMode {
  Analog,
  InFloat,
  InPP,
  OutPP, // output - push/pull
  OutDrain,
  OutAltPP,
  OutAltDrain,
}

#[derive(Debug, Clone, Copy)]
pub enum PinSpeed {
  Max2MHz,
  Max10MHz,
  Max50MHz,
}

#[repr(packed)]
struct Gpio_register_map {
  CRL:  u32,
  CRH:  u32,
  IDR:  u32,
  ODR:  u32,
  BSRR: u32,
  BRR:  u32,
  LCK:  u32,
}

#[derive(Debug)]
pub struct Gpio(u32);

pub const GPIOA: Gpio = Gpio(0x4001_0800);
pub const GPIOB: Gpio = Gpio(0x4001_0C00);
pub const GPIOC: Gpio = Gpio(0x4001_1000);
pub const GPIOD: Gpio = Gpio(0x4001_1400);
pub const GPIOE: Gpio = Gpio(0x4001_1800);
pub const GPIOF: Gpio = Gpio(0x4001_1C00);
pub const GPIOG: Gpio = Gpio(0x4001_2000);

impl Gpio {
  pub fn enable_pin(&self, pin: u8) {
    /* FIXME sanitize 'num' (possible values: 0-15 inclusive) */
    let regmap = self.0 as *mut Gpio_register_map;

    unsafe {
      (*regmap).BSRR |= 1u32 << pin;
    }
  }

  pub fn disable_pin(&self, pin: u8) {
    /* FIXME sanitize 'num' (possible values: 0-15 inclusive) */
    let regmap = self.0 as *mut Gpio_register_map;

    unsafe {
      (*regmap).BSRR |= 1u32 << (16u32 + pin as u32);
    }
  }

  pub fn set_pin_mode(&self, pin: u8, mode: PinMode) {
    let regmap = self.0 as *mut Gpio_register_map;

    let bits = match mode {
      PinMode::Analog      => 0b0000,
      PinMode::InFloat     => 0b0100,
      PinMode::InPP        => 0b1000,
      PinMode::OutPP       => 0b0010,
      PinMode::OutDrain    => 0b0110,
      PinMode::OutAltPP    => 0b1010,
      PinMode::OutAltDrain => 0b1110,
    };

    unsafe {
      (*regmap).CRL = ((*regmap).CRL & !(0b1111 << (4 * pin))) | (bits << (4 * pin));
    }
  }

  pub fn set_pin_speed(&self, pin: u8, speed: PinSpeed) {
    let regmap = self.0 as *mut Gpio_register_map;

    // TODO only makes sense for output pins - sanitize!
    let bits = match speed {
      PinSpeed::Max2MHz  => 0b10,
      PinSpeed::Max10MHz => 0b01,
      PinSpeed::Max50MHz => 0b11,
    };

    unsafe {
      (*regmap).CRL = ((*regmap).CRL & !(0b11 << (4 * pin))) | (bits << (4 * pin));
    }
  }
}

/*
 * vi: ts=2 sw=2 expandtab
 */

