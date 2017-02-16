//
// gpio.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 16 Feb 2017 21:36:10 +0100 (CET)
//

pub enum Port {
  A,
  B,
  C,
  D,
  E,
  F,
  G,
}

pub enum PinMode {
  Analog,
  InFloat,
  InPP,
  OutPP, // output - push/pull
  OutDrain,
  OutAltPP,
  OutAltDrain,
}

pub struct GpioPort {
  base_addr: u32,
}

pub fn port(port: Port) -> GpioPort {
  let base_addr = match port {
    Port::A => 0x4001_0800,
    Port::B => 0x4001_0C00,
    Port::C => 0x4001_1000,
    Port::D => 0x4001_1400,
    Port::E => 0x4001_1800,
    Port::F => 0x4001_1C00,
    Port::G => 0x4001_2000,
  };

  GpioPort {
    base_addr: base_addr,
  }
}

impl GpioPort {
  pub fn enable_pin(&self, pin: u8) {
    /* FIXME sanitize 'num' (possible values: 0-15 inclusive) */
    unsafe {
      *((self.base_addr + 0x10) as *mut u32) = 1 << pin;
    }
  }

  pub fn disable_pin(&self, pin: u8) {
    /* FIXME sanitize 'num' (possible values: 0-15 inclusive) */
    unsafe {
      *((self.base_addr + 0x10) as *mut u32) = 1 << (16 + pin);
    }
  }

  pub fn set_pin_mode(&self, pin: u8, mode: PinMode) {
    /* 0x0 is the offset of the CRL register */
    let crl = (self.base_addr + 0x0) as *mut u32;

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
      *crl = (*crl & !(0b1111 << (4 * pin))) | (bits << (4 * pin));
    }
  }
}

/*
 * vi: ts=2 sw=2 expandtab
 */

