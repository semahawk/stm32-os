//
// rcc.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 17 Feb 2017 17:05:04 +0100 (CET)
//

use mmio;
use conf;

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
const RCC_CFGR_SWS_HSI: u32 = 0b00 << 2;
const RCC_CFGR_SWS_HSE: u32 = 0b01 << 2;
const RCC_CFGR_SWS_PLL: u32 = 0b10 << 2;
const RCC_CFGR_HPRE: u32 = 0b1111 << 4;
const RCC_CFGR_PPRE1: u32 = 0b111 << 8;
const RCC_CFGR_PPRE2: u32 = 0b111 << 11;
const RCC_CFGR_ADCPRE: u32 = 0b11 << 14;
const RCC_CFGR_PLLSRC: u32 = 0b1 << 16;
const RCC_CFGR_PLLSRC_HSI: u32 = 0b0 << 16;
const RCC_CFGR_PLLSRC_HSE: u32 = 0b1 << 16;
const RCC_CFGR_PLLMUL: u32 = 0b1111 << 18;
const RCC_CFGR_MCO: u32 = 0b1111 << 24;
/// RCC Clock Interrupt Register address
const RCC_CIR: u32 = RCC + 0x08;
/// Address of the APB1ENR register
const RCC_APB1ENR: u32 = RCC + 0x1c;
/// Bit that is in charge of enabling/disabling the USART2 port
const RCC_APB1ENR_USART2EN: u32 = 1 << 17;
/// Address of the APB2ENR register
const RCC_APB2ENR: u32 = RCC + 0x18;
/// Bit that is in charge of enabling/disabling the GPIOA port
const RCC_APB2ENR_IOPAEN: u32 = 1 << 2;
/// Bit that is in charge of enabling/disabling the GPIOB port
const RCC_APB2ENR_IOPBEN: u32 = 1 << 3;
/// Bit that is in charge of enabling/disabling the GPIOC port
const RCC_APB2ENR_IOPCEN: u32 = 1 << 4;
/// Bit that is in charge of enabling/disabling the GPIOD port
const RCC_APB2ENR_IOPDEN: u32 = 1 << 5;
/// Bit that is in charge of setting the alternate function of the IO clock
const RCC_APB2ENR_AFIOEN: u32 = 1 << 0;

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
  apb1_usart2,
  apb2_afio,
  apb2_gpioa,
  apb2_gpiob,
  apb2_gpioc,
  apb2_gpiod,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Clock {
  SYSCLK,
  HCLK,
  PCLK1,
  PCLK2,
  // TODO SDIOCLK, FSMCCLK, FCLK, TIMXCLK, ADCCLK etc.
}

pub fn enable(periph: Periph) {
  let (reg, bit) = match periph {
    Periph::apb1_usart2 => (RCC_APB1ENR, RCC_APB1ENR_USART2EN),
    Periph::apb2_afio   => (RCC_APB2ENR, RCC_APB2ENR_AFIOEN),
    Periph::apb2_gpioa  => (RCC_APB2ENR, RCC_APB2ENR_IOPAEN),
    Periph::apb2_gpiob  => (RCC_APB2ENR, RCC_APB2ENR_IOPBEN),
    Periph::apb2_gpioc  => (RCC_APB2ENR, RCC_APB2ENR_IOPCEN),
    Periph::apb2_gpiod  => (RCC_APB2ENR, RCC_APB2ENR_IOPDEN),
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

pub fn get_clock_speed(clock: Clock) -> u32 {
  let sysclk = match mmio::read(RCC_CFGR) & RCC_CFGR_SWS {
    /* HSI oscillator used as system clock */
    RCC_CFGR_SWS_HSI => conf::HSI_BASE_FREQUENCY,
    /* HSE oscillator used as system clock */
    RCC_CFGR_SWS_HSE => conf::HSE_BASE_FREQUENCY,
    /* PLL used as system clock */
    RCC_CFGR_SWS_PLL => {
      let pll_in_freq = match mmio::read(RCC_CFGR) & RCC_CFGR_PLLSRC {
        RCC_CFGR_PLLSRC_HSI => conf::HSI_BASE_FREQUENCY / 2,
        RCC_CFGR_PLLSRC_HSE => conf::HSE_BASE_FREQUENCY,
        _ => panic!(),
      };

      // Use the fact that 0b0000 => 2, 0b0001 => 3, 0b0010 => 4, etc.
      let pll_multiplier = ((mmio::read(RCC_CFGR) & RCC_CFGR_PLLMUL) >> 18) + 2;

      pll_in_freq * pll_multiplier
    },
    _ => panic!(),
  };

  let hpre = (mmio::read(RCC_CFGR) & RCC_CFGR_HPRE) >> 4;
  let hclk = sysclk >> (((hpre & 0b1000) >> 3) * ((hpre & 0b111) + 1));

  let ppre1 = (mmio::read(RCC_CFGR) & RCC_CFGR_PPRE1) >> 8;
  let pclk1 = hclk >> (((ppre1 & 0b100) >> 2) * ((ppre1 & 0b11) + 1));

  let ppre2 = (mmio::read(RCC_CFGR) & RCC_CFGR_PPRE2) >> 11;
  let pclk2 = hclk >> (((ppre2 & 0b100) >> 2) * ((ppre2 & 0b11) + 1));

  // Return the requested value here so that we exhaust all input patterns
  return match clock {
    Clock::SYSCLK => sysclk,
    Clock::HCLK   => hclk,
    Clock::PCLK1  => pclk1,
    Clock::PCLK2  => pclk2,
  };
}

/*
 * vi: ts=2 sw=2 expandtab
 */

