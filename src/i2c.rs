//
// i2c.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 01 Mar 2017 19:29:46 +0100 (CET)
//

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

/*
 * vi: ts=2 sw=2 expandtab
 */

