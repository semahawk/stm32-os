//
// mcp23s08.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 02 Mar 2017 21:37:36 +0100 (CET)
//

use spi;
use gpio;

pub const IODIR:   u8 = 0x00;
pub const IPOL:    u8 = 0x01;
pub const GPINTEN: u8 = 0x02;
pub const DEFVAL:  u8 = 0x03;
pub const INTCON:  u8 = 0x04;
pub const IOCON:   u8 = 0x05;
pub const GPPU:    u8 = 0x06;
pub const INTF:    u8 = 0x07;
pub const INTCAP:  u8 = 0x08;
pub const GPIO:    u8 = 0x09;
pub const OLAT:    u8 = 0x0a;

pub fn write_reg(spi: spi::SPI, reg: u8, value: u8) {
  gpio::GPIOC.disable_pin(0);
  spi.send_recv_byte(0x40);
  spi.send_recv_byte(reg);
  spi.send_recv_byte(value);
  gpio::GPIOC.enable_pin(0);
}

/*
 * vi: ts=2 sw=2 expandtab
 */

