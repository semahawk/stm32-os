MEMORY {
  flash (rx): ORIGIN = 0x08000000, LENGTH = 128K
  sram (rwx): ORIGIN = 0x20000000, LENGTH = 20K
}

SECTIONS {
  . = ORIGIN(flash);

  .text :
  {
    . = ALIGN(4);
    *(.text*)
  } >flash

  .data :
  {
    . = ALIGN(4);

    *(.data)
    *(.data.*)

    stack_bottom = .;
    . += 2K;
    stack_top = ORIGIN(sram) + LENGTH(sram);
  } >sram
}
