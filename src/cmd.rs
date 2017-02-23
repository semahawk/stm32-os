//
// cmd.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 22 Feb 2017 17:54:08 +0100 (CET)
//

use core::str::Split;
use core::fmt::Write;

use gpio::Gpio_trait;

use gpio;
use usart;

const commands: &'static [(&str, fn (Split<char>))] = &[
  ("gpio", gpio),
];

pub fn lookup_command(cmd: &str) -> Option<fn (Split<char>)> {
  for &(command, function) in commands {
    if command == cmd {
      return Some(function)
    }
  }

  None
}

fn gpio(mut args: Split<char>) {
  enum Op { set, clear };

  let op = match args.next() {
    Some("set") => Op::set,
    Some("clear") => Op::clear,
    _ => {
      print!("Usage: gpio <set|clear> <A> <0-15>\r\n");
      return;
    }
  };

  let (gpio, port) = match args.next() {
    Some("A") | Some("a") => (gpio::GPIOA, "A"),
    _ => {
      print!("Usage: gpio <set|clear> <A> <0-15>\r\n");
      return;
    }
  };

  let pin = match args.next() {
    Some(pin) => pin.parse::<u8>().unwrap(),
    None => {
      print!("Usage: gpio <set|clear> <A> <0-15>\r\n");
      return;
    }
  };

  if pin > 15 {
    print!("Usage: gpio <set|clear> <A> <0-15>\r\n");
    return;
  }

  match op {
    Op::set => {
      gpio.enable_pin(pin);
      print!("Enabled pin {} in GPIO port {}\r\n", pin, port);
    },
    Op::clear => {
      gpio.disable_pin(pin);
      print!("Disabled pin {} in GPIO port {}\r\n", pin, port);
    }
  }
}

/*
 * vi: ts=2 sw=2 expandtab
 */

