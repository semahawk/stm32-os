//
// rcc.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 17 Feb 2017 17:05:04 +0100 (CET)
//

/// Base address of the RCC block
const RCC: u32 = 0x4002_1000;
/// Address of the APB2ENR register
const RCC_APB2ENR: u32 = RCC + 0x18;
/// Mask of the bit that is in charge of enabling/disabling the GPIOA port
const RCC_APB2ENR_IOPAEN: u32 = 1 << 2;

pub enum Periph {
  apb2_gpioa,
}

pub fn enable(periph: Periph) {
  let (reg, bit) = match periph {
    apb2_gpioa => (RCC_APB2ENR, RCC_APB2ENR_IOPAEN),
  };

  unsafe {
    *(reg as *mut u32) |= bit;
  }
}

/*
 * vi: ts=2 sw=2 expandtab
 */

