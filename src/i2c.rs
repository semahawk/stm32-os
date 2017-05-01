//
// i2c.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 01 Mar 2017 19:29:46 +0100 (CET)
//

use gpio;

#[repr(packed)]
struct I2C_register_map {
  CR1: u32,
  CR2: u32,
  OAR1: u32,
  OAR2: u32,
  DR: u32,
  SR1: u32,
  SR2: u32,
  CCR: u32,
  TRISE: u32,
}

#[derive(Debug, PartialEq)]
pub struct I2C(*mut I2C_register_map);

pub const I2C1: I2C = I2C(0x4000_5400 as *mut I2C_register_map);
pub const I2C2: I2C = I2C(0x4000_5800 as *mut I2C_register_map);

impl I2C {
  pub fn initialize(&self) {
    if *self == I2C1 {
      // SCL
      gpio::GPIOB.set_pin_mode(6, gpio::PinMode::OutAltDrain);
      gpio::GPIOB.set_pin_speed(6, gpio::PinSpeed::Max50MHz);
      // SDA
      gpio::GPIOB.set_pin_mode(7, gpio::PinMode::OutAltDrain);
      gpio::GPIOB.set_pin_speed(7, gpio::PinSpeed::Max50MHz);
    } else {
      unimplemented!();
    }

    unsafe {
      // FREQ - Set the peripheral clock frequency to 8MHz
      (*self.0).CR2 |= 0b001000 << 0;

      // CCR
      (*self.0).CCR |= 0x28 << 0;

      // TRISE
      (*self.0).TRISE |= 0x9;

      // PE - Enable the device
      (*self.0).CR1 |= 1 << 0;
    }
  }

  pub fn generate_start(&self) {
    unsafe {
      (*self.0).CR1 |= 1 << 8;

      print!("i2c: generating start (sr1: 0x{:x})\r\n", (*self.0).SR1);
      // Wait until the start condition was actually generated
      while (*self.0).SR1 & (1 << 0) == 0 {}
    }
  }

  pub fn generate_stop(&self) {
    unsafe {
      (*self.0).CR1 |= 1 << 9;

      print!("i2c: generating stop\r\n");
      // Wait until the start condition was actually generated
      while (*self.0).SR1 & (1 << 4) == 0 {}
    }
  }

  pub fn send_address(&self, addr: u8) {
    unsafe {
      (*self.0).DR = addr as u32 & 0b01111111;

      print!("i2c: sending address\r\n");
      // Wait until the address was actually sent
      while (*self.0).SR1 & (1 << 1) == 0 {}
    }
  }

  pub fn send_byte(&self, data: u8) {
    unsafe {
      (*self.0).DR = data as u32;

      print!("i2c: sending byte\r\n");
      // Wait until the data was sent successfully
      while (*self.0).SR1 & (1 << 2) == 0 {}
    }
  }
}

/*
 * vi: ts=2 sw=2 expandtab
 */

