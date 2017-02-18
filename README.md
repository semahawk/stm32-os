What
====

My lousy attempt at creating something in Rust for an embedded device with a
Cortex M3 CPU.

The board I'm using is the STM32 Nucleo F103RB (http://www.st.com/content/st_com/en/products/evaluation-tools/product-evaluation-tools/mcu-eval-tools/stm32-mcu-eval-tools/stm32-mcu-nucleo/nucleo-f103rb.html).

Build
=====

```
xargo build --target thumbv7m-none-eabi
```

Install on target
=================

```
# openocd -f interface/stlink-v2-1.cfg -f board/st_nucleo_f103rb.cfg -f flash_image.ocd
```

Run
===

Via OpenOCD:

```
# openocd -f interface/stlink-v2-1.cfg -f board/st_nucleo_f103rb.cfg -f attach.ocd
```

And in another terminal:

```
$ telnet localhost 4444
Trying ::1...
telnet: connect to address ::1: Connection refused
Trying 127.0.0.1...
Connected to localhost.
Escape character is '^]'.
Open On-Chip Debugger
> resume
```

