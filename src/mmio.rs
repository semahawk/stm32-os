//
// mmio.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 19 Feb 2017 15:07:42 +0100 (CET)
//

use core::intrinsics::volatile_store;
use core::intrinsics::volatile_load;

pub fn read(reg: u32) -> u32 {
  unsafe {
    volatile_load(reg as *const u32)
  }
}

pub fn write(reg: u32, val: u32) {
  unsafe {
    volatile_store(reg as *mut u32, val)
  }
}

pub fn write_u8(reg: u32, val: u8) {
  unsafe {
    volatile_store(reg as *mut u8, val)
  }
}

pub fn set_bit(reg: u32, bit: u32) {
  write(reg, read(reg) | bit)
}

/*
 * vi: ts=2 sw=2 expandtab
 */

