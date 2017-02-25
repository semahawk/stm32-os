//
// timer.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 24 Feb 2017 18:09:34 +0100 (CET)
//

use usart::Usart_trait;
use usart;

//
// NOTE: this module only applies to the general purpose timers (2 through 5)
//

// Counter enable
const TIMx_CR1_CEN: u32 = 1 << 0;

#[repr(packed)]
struct Timer_register_map {
  CR1: u32,
  CR2: u32,
  SMCR: u32,
  DIER: u32,
  SR: u32,
  EGR: u32,
  CCMR1_OUT: u32,
  CCMR1_IN: u32,
  CCMR2_OUT: u32,
  CCMR2_IN: u32,
  CCER: u32,
  CNT: u32,
  PSC: u32,
  ARR: u32,
  _rsvd0: u32,
  CCR1: u32,
  CCR2: u32,
  CCR3: u32,
  CCR4: u32,
  _rsvd1: u32,
  DCR: u32,
  DMAR: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Timer {
  regmap: *mut Timer_register_map,
  pub irq_handler: extern "C" fn (),
}

pub const TIM2: Timer = Timer { regmap: 0x4000_0000 as *mut Timer_register_map, irq_handler: tim2_handler };

impl Timer {
  pub fn enable(self, period_ms: u32) {
    (*self.regmap).CR1 |= TIMx_CR1_CEN;
  }
}

pub extern "C" fn tim2_handler() {
  print!("TIM2 handler called!\r\n");
  loop {}
}

/*
 * vi: ts=2 sw=2 expandtab
 */

