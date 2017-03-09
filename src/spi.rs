//
// spi.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 01 Mar 2017 20:07:34 +0100 (CET)
//

use gpio;

#[repr(packed)]
struct SPI_register_map {
  CR1: u32,
  CR2: u32,
  SR: u32,
  DR: u32,
  CRCPR: u32,
  RXCRCR: u32,
  TXCRCR: u32,
  I2SCFGR: u32,
  I2SPR: u32,
}

/// Data frame format
///  0: 8-bit data frame
///  1: 16-bit data frame
const SPI_CR1_DFF: u32 = 1 << 11;
/// Software slave management
const SPI_CR1_SSM: u32 = 1 << 9;
/// Internal slave select
const SPI_CR1_SSI: u32 = 1 << 8;
/// Does LSB go first?
const SPI_CR1_LSBFIRST: u32 = 1 << 7;
/// SPI enable
const SPI_CR1_SPE: u32 = 1 << 6;
/// Baud rate control
///  000: fPCLK/2
///  001: fPCLK/4
///  010: fPCLK/8
///  011: fPCLK/16
///  100: fPCLK/32
///  101: fPCLK/64
///  110: fPCLK/128
///  111: fPCLK/256
const SPI_CR1_BR: u32 = 0b111 << 3;
/// Are we the master?
const SPI_CR1_MSTR: u32 = 1 << 2;
/// Clock polarity
///  0: CK to 0 when idle
///  1: CK to 1 when idle
const SPI_CR1_CPOL: u32 = 1 << 1;
/// Clock phase
///  0: The first clock transition is the first data capture edge
///  1: The second clock transition is the first data capture edge
const SPI_CR1_CPHA: u32 = 1 << 0;

/// Transmition buffer empty (can transmit?)
const SPI_SR_TXE: u32 = 1 << 1;
/// Reception buffer not empty (is there data to be received?)
const SPI_SR_RXNE: u32 = 1 << 0;

#[derive(PartialEq)]
pub struct SPI {
  regmap: *mut SPI_register_map,
}

pub const SPI1: SPI = SPI { regmap: 0x4001_3000 as *mut SPI_register_map };

impl SPI {
  pub fn initialize(self) {
    if self == SPI1 {
      // SCK
      gpio::GPIOA.set_pin_mode(5, gpio::PinMode::OutAltPP);
      gpio::GPIOA.set_pin_speed(5, gpio::PinSpeed::Max50MHz);
      // MOSI
      gpio::GPIOA.set_pin_mode(7, gpio::PinMode::OutAltPP);
      gpio::GPIOA.set_pin_speed(7, gpio::PinSpeed::Max50MHz);
      // MISO
      gpio::GPIOA.set_pin_mode(6, gpio::PinMode::InFloat);
      // CS
      gpio::GPIOC.set_pin_mode(0, gpio::PinMode::OutPP);
      // Disable the CS line (it's inactive when high)
      gpio::GPIOC.enable_pin(0);
    }

    unsafe {
      // Scale the baud rate by 16 => 64MHz / 16 = 4MHz (4Mbps) which
      // is appropriate for the SPI
      (*self.regmap).CR1 = 0b011 << 3;

      // Clock polarity -> 0, clock phase -> 0
      (*self.regmap).CR1 &= !(SPI_CR1_CPOL | SPI_CR1_CPHA);

      // 8-bits of data in a frame
      (*self.regmap).CR1 &= !SPI_CR1_DFF;

      // MSB goes first
      (*self.regmap).CR1 &= !SPI_CR1_LSBFIRST;

      // The CS line will be controlled by software
      (*self.regmap).CR1 |= SPI_CR1_SSM | SPI_CR1_SSI;

      // Let's be the master
      (*self.regmap).CR1 |= SPI_CR1_MSTR;

      // Actually enable the SPI device
      (*self.regmap).CR1 |= SPI_CR1_SPE;
    }
  }

  pub fn send_recv_byte(&self, byte: u8) -> u8 {
    unsafe {
      while (*self.regmap).SR & SPI_SR_TXE == 0 {}
      (*self.regmap).DR = byte as u32;

      while (*self.regmap).SR & SPI_SR_RXNE == 0 {}
      return (*self.regmap).DR as u8;
    }
  }

  //pub fn recv_byte(&self) -> u8 {
    //unsafe {
      //while (*self.regmap).SR & SPI_SR_RXNE == 0 {}

      //return (*self.regmap).DR as u8;
    //}
  //}
}

/*
 * vi: ts=2 sw=2 expandtab
 */

