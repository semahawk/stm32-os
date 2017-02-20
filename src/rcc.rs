//
// rcc.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 17 Feb 2017 17:05:04 +0100 (CET)
//

use mmio;

/// Base address of the RCC block
const RCC: u32 = 0x4002_1000;
/// RCC Control Register address
const RCC_CR: u32 = RCC + 0x0;
const RCC_CR_HSION: u32 = 1 << 1;
const RCC_CR_HSEON: u32 = 1 << 16;
const RCC_CR_CSSON: u32 = 1 << 19;
const RCC_CR_HSEBYP: u32 = 1 << 18;
const RCC_CR_PLLON: u32 = 1 << 24;
const RCC_CR_PLLRDY: u32 = 1 << 25;
/// RCC Clock Configuration Register address
const RCC_CFGR: u32 = RCC + 0x04;
const RCC_CFGR_SW: u32 = 0b11 << 0;
const RCC_CFGR_SWS: u32 = 0b11 << 2;
const RCC_CFGR_HPRE: u32 = 0b1111 << 4;
const RCC_CFGR_PPRE1: u32 = 0b111 << 8;
const RCC_CFGR_PPRE2: u32 = 0b111 << 11;
const RCC_CFGR_ADCPRE: u32 = 0b11 << 14;
const RCC_CFGR_PLLSRC: u32 = 0b1 << 16;
const RCC_CFGR_MCO: u32 = 0b1111 << 24;
/// RCC Clock Interrupt Register address
const RCC_CIR: u32 = RCC + 0x08;
/// Address of the APB2ENR register
const RCC_APB2ENR: u32 = RCC + 0x18;
/// Mask of the bit that is in charge of enabling/disabling the GPIOA port
const RCC_APB2ENR_IOPAEN: u32 = 1 << 2;

/// TODO: move this out to a separate flash-specific module
/// Base address of the flash memory interface
const FLASH: u32 = 0x4002_2000;
/// Flash Access Control Register
const FLASH_ACR: u32 = FLASH + 0x0;
/// Flash latency
/// 000 Zero wait state,  if  0 MHz < SYSCLK ≤ 24 MHz
/// 001 One  wait state,  if 24 MHz < SYSCLK ≤ 48 MHz
/// 010 Two  wait states, if 48 MHz < SYSCLK ≤ 72 MHz
const FLASH_ACR_LATENCY: u32 = 0b111 << 0;
/// Prefetch buffer
const FLASH_ACR_PRFBTE: u32 = 0b1 << 4;

pub enum Periph {
  apb2_gpioa,
}

pub fn enable(periph: Periph) {
  let (reg, bit) = match periph {
    Periph::apb2_gpioa => (RCC_APB2ENR, RCC_APB2ENR_IOPAEN),
  };

  unsafe {
    *(reg as *mut u32) |= bit;
  }
}

pub fn initialize_clocks() {
  mmio::set_bits(RCC_CR, RCC_CR_HSION);
  mmio::unset_bits(RCC_CR, RCC_CR_HSEON | RCC_CR_CSSON | RCC_CR_PLLON);
  mmio::unset_bits(RCC_CR, RCC_CR_HSEBYP);

  // Reset RCC_CFGR to it's reset value
  mmio::write(RCC_CFGR, 0x0);

  // Disable all interrupts and clear pending bits
  mmio::write(RCC_CIR, 0x009f0000);

  // Set PLL's input to be the HSI clock (have to clear the bit)
  mmio::unset_bits(RCC_CFGR, RCC_CFGR_PLLSRC);
  // Set PLL's multiplication factor to 8
  // HSI's frequency is 8MHz so:
  // SYSCLK = PLLCLK = 8MHz / 2 * 16 = 64MHz
  mmio::set_bits(RCC_CFGR, 0b1110 << 18);
  // Set HPRE division to 1 (ie. HCLK = SYSCLK)
  mmio::set_bits(RCC_CFGR, 0b0000 << 4);
  // Set PPRE2 division to 1 (ie. PCLK2 = HCLK)
  mmio::set_bits(RCC_CFGR, 0b000 << 11);
  // Set PPRE1 division to 2 (ie. PCLK1 = HCLK / 2 (can't exceed 36MHz))
  mmio::set_bits(RCC_CFGR, 0b100 << 8);

  // Enable flash's prefetch buffer
  mmio::set_bits(FLASH_ACR, FLASH_ACR_PRFBTE);
  // If we're going with 64MHz SYSCLK we better set latency to two wait states
  mmio::set_bits(FLASH_ACR, 0b010 << 0);

  // Enable the PLL
  mmio::set_bits(RCC_CR, RCC_CR_PLLON);

  // Wait until the PLL is ready
  while mmio::read(RCC_CR) & RCC_CR_PLLRDY == 0 { }

  // Actually select the PLL as the system clock
  mmio::set_bits(RCC_CFGR, 0b10 << 0);

  // Wait till PLL is actually used as the system clock
  while mmio::read(RCC_CFGR) & RCC_CFGR_SWS != 0b10 << 2 { }
}

/*
 * vi: ts=2 sw=2 expandtab
 */

