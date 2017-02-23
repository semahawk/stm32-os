//
// cmd.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 22 Feb 2017 17:54:08 +0100 (CET)
//

use core::str::Split;
use core::fmt::Write;

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
      write!(usart::USART2, "Usage: gpio <set|clear> <A> <0-15>\r\n");
      return;
    }
  };

  let port = match args.next() {
    Some("A") | Some("a") => gpio::Port::A,
    _ => {
      write!(usart::USART2, "Usage: gpio <set|clear> <A> <0-15>\r\n");
      return;
    }
  };

  let pin = match args.next() {
    Some(pin) => pin.parse::<u8>().unwrap(),
    None => {
      write!(usart::USART2, "Usage: gpio <set|clear> <A> <0-15>\r\n");
      return;
    }
  };

  if pin > 15 {
    write!(usart::USART2, "Usage: gpio <set|clear> <A> <0-15>\r\n");
    return;
  }

  let gpio = gpio::port(port);

  match op {
    Op::set   => {
      write!(usart::USART2, "Enabled pin {} in port {:?}\r\n", pin, port);
      gpio.enable_pin(pin);
    },
    Op::clear => {
      write!(usart::USART2, "Disabled pin {} in port {:?}\r\n", pin, port);
      gpio.disable_pin(pin);
    }
  }
}

/*
 * vi: ts=2 sw=2 expandtab
 */

