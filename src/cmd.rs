//
// cmd.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 22 Feb 2017 17:54:08 +0100 (CET)
//

use gpio;

const commands: &'static [(&str, fn ())] = &[
  ("blink", blink),
];

pub fn lookup_command(cmd: &str) -> Option<fn ()> {
  for &(command, function) in commands {
    if command == cmd {
      return Some(function)
    }
  }

  None
}

fn blink() {
  let gpioa = gpio::port(gpio::Port::A);
  gpioa.enable_pin(5);
  for _ in 0..10_000 {  };
  gpioa.disable_pin(5);
  for _ in 0..10_000 {  };
}

/*
 * vi: ts=2 sw=2 expandtab
 */

