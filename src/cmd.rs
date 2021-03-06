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
use spi;
use mcp23s08;

const commands: &'static [(&str, fn (Split<char>))] = &[
  ("gpio", gpio),
  ("spi", spi),
  ("mcp", mcp),
  ("loadb", loadb),
];

pub fn lookup_command(cmd: &str) -> Option<fn (Split<char>)> {
  for &(command, function) in commands {
    if command == cmd {
      return Some(function)
    }
  }

  None
}

fn loadb(mut args: Split<char>) {
  let address = match args.next() {
    Some(address) => i32::from_str_radix(address, 16).ok().unwrap(),
    None => {
      print!("Usage: loadb <address>\r\n");
      return;
    }
  };

  let mut bytes_received = 0;
  let mut address = address as *mut u8;
  print!("Ready to receive file; will write to {:p}\r\n", address);

  loop {
    let byte = usart::USART2.get_byte();

    if byte == 0x3 {
      print!("Ending..\r\n");
      break;
    }

    print!("Got byte: 0x{:x}\r\n", byte);

    unsafe {
      *address.offset(bytes_received) = byte;
    }

    bytes_received += 1;
  }

  print!("Received {} bytes\r\n", bytes_received);
}

fn gpio(mut args: Split<char>) {
  #[derive(PartialEq)]
  enum Op { set, clear, mode };

  let op = match args.next() {
    Some("set") => Op::set,
    Some("clear") => Op::clear,
    Some("mode") => Op::mode,
    _ => {
      print!("Usage: gpio <set|clear> <A|B|C|D|E|F|G> <0-15>\r\n");
      print!("Usage: gpio <mode> <A|B|C|D|E|F|G> <0-15> <analog|infloat|inpp|outpp|outdrain|outaltpp|outaltdrain>\r\n");
      return;
    }
  };

  let (gpio, port) = match args.next() {
    Some("A") | Some("a") => (gpio::GPIOA, "A"),
    Some("B") | Some("b") => (gpio::GPIOB, "B"),
    Some("C") | Some("c") => (gpio::GPIOC, "C"),
    Some("D") | Some("d") => (gpio::GPIOD, "D"),
    Some("E") | Some("e") => (gpio::GPIOE, "E"),
    Some("F") | Some("f") => (gpio::GPIOF, "F"),
    Some("G") | Some("g") => (gpio::GPIOG, "G"),
    _ => {
      print!("Usage: gpio <set|clear> <A|B|C|D|E|F|G> <0-15>\r\n");
      print!("Usage: gpio <mode> <A|B|C|D|E|F|G> <0-15> <analog|infloat|inpp|outpp|outdrain|outaltpp|outaltdrain>\r\n");
      return;
    }
  };

  let pin = match args.next() {
    Some(pin) => pin.parse::<u8>().unwrap(),
    None => {
      print!("Usage: gpio <set|clear> <A|B|C|D|E|F|G> <0-15>\r\n");
      print!("Usage: gpio <mode> <A|B|C|D|E|F|G> <0-15> <analog|infloat|inpp|outpp|outdrain|outaltpp|outaltdrain>\r\n");
      return;
    }
  };

  if pin > 15 {
    print!("Usage: gpio <set|clear> <A|B|C|D|E|F|G> <0-15>\r\n");
    print!("Usage: gpio <mode> <A|B|C|D|E|F|G> <0-15> <analog|infloat|inpp|outpp|outdrain|outaltpp|outaltdrain>\r\n");
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
    },
    Op::mode => {
      let mode = {
        match args.next() {
          Some("analog") => gpio::PinMode::Analog,
          Some("infloat") => gpio::PinMode::InFloat,
          Some("inpp") => gpio::PinMode::InPP,
          Some("outpp") => gpio::PinMode::OutPP,
          Some("outdrain") => gpio::PinMode::OutDrain,
          Some("outaltpp") => gpio::PinMode::OutAltPP,
          Some("outaltdrain") => gpio::PinMode::OutAltDrain,
          Some(_) | None => {
            print!("Usage: gpio <mode> <A> <0-15> <analog|infloat|inpp|outpp|outdrain|outaltpp|outaltdrain>\r\n");
            return;
          }
        }
      };

      gpio.set_pin_mode(pin, mode);
      print!("Mode set to {:?} for pin {} in GPIO port {}\r\n", mode, pin, port);
    },
  }
}

fn spi(mut args: Split<char>) {
  let spi = match args.next() {
    Some("1") => spi::SPI1,
    Some(_) | None => {
      print!("Usage: spi <1> <output value>\r\n");
      return;
    },
  };

  let value = match args.next() {
    Some(value) => u8::from_str_radix(value, 16).ok().unwrap(),
    None => {
      print!("Usage: spi <1> <output value>\r\n");
      return;
    },
  };

  let input_value = spi.send_recv_byte(value);

  print!("Returned: {:x}\r\n", input_value);
}

fn mcp(mut args: Split<char>) {
  #[derive(PartialEq)]
  enum Op { Write };

  let op = match args.next() {
    Some("write") => Op::Write,
    Some (_) | None => {
      print!("Usage: mcp write <reg> <value>\r\n");
      return;
    },
  };

  let reg = match args.next() {
    Some(reg) => u8::from_str_radix(reg, 16).ok().unwrap(),
    None => {
      print!("Usage: mcp write <reg> <value>\r\n");
      return;
    },
  };

  let value = match args.next() {
    Some(value) => u8::from_str_radix(value, 16).ok().unwrap(),
    None => {
      print!("Usage: mcp write <reg> <value>\r\n");
      return;
    },
  };

  mcp23s08::write_reg(spi::SPI1, reg, value);
}

/*
 * vi: ts=2 sw=2 expandtab
 */

