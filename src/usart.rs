//
// usart.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 17 Feb 2017 18:08:54 +0100 (CET)
//

use mmio;

/// Status Register
const USART_SR: u32 = 0x00;
/// Transmission complete
const USART_SR_TC: u32 = 1 << 6;
/// Transmitter data register empty (ie. can send bytes?)
const USART_SR_TXE: u32 = 1 << 7;
/// Data Register
const USART_DR: u32 = 0x04;
/// Baud Rate Register
const USART_BRR: u32 = 0x08;
/// Control Register (1)
const USART_CR1: u32 = 0x0c;
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
/// UART Control Register (2)
const USART_CR2: u32 = 0x10;

pub enum Baudrate {
  _9600,
  _115200,
}

pub enum Port {
  Usart1,
  Usart2,
  Usart3,
}

pub struct Usart {
  base_addr: u32,
}

pub fn new(port: Port, baudrate: Baudrate) -> Usart {
  let base_addr = match port {
    Port::Usart1 => 0x4001_3800,
    Port::Usart2 => 0x4000_4400,
    Port::Usart3 => 0x4000_4800,
  };

  let usart_cr1 = base_addr + USART_CR1;
  let usart_cr2 = base_addr + USART_CR2;
  let usart_brr = base_addr + USART_BRR;

  unsafe {
    // Configure stop bits (00 - 1 stop bit)
    mmio::write(usart_cr2, 0x0);

    // Enable transmition and reception
    mmio::set_bits(usart_cr1, USART_CR1_TE | USART_CR1_RE);

    // Assuming 32MHz of PCLK1's frequency and 115200 baudrate
    // TODO calculate this
    mmio::write(usart_brr, 0b100010110);

    // Enable the UART
    mmio::set_bits(usart_cr1, USART_CR1_UE);
  }

  Usart {
    base_addr: base_addr,
  }
}

impl Usart {
  pub fn send_byte(&self, data: u8) {
    let usart_sr = self.base_addr + USART_SR;
    let usart_dr = self.base_addr + USART_DR;

    // Wait until there's space for transmission
    while mmio::read(usart_sr) & USART_SR_TXE == 0 { }

    // Actually transmit the data
    mmio::write_u8(usart_dr, data);

    // Wait until the transmission is complete
    while mmio::read(usart_sr) & USART_SR_TC != 0 { }
  }
}

/*
 * vi: ts=2 sw=2 expandtab
 */

