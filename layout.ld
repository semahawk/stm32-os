ENTRY(_reset);

MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 128K
  RAM   : ORIGIN = 0x20000000, LENGTH = 20K
}

SECTIONS
{
  .text :
  {
    /* the ARM vector table */
    /* address of the stack (starts at end of ram) */
    LONG(ORIGIN(RAM) + LENGTH(RAM))
    /* adding 1, because bit 0 has to be set, to indicate Thumb mode */
    LONG(_reset + 1);
    KEEP(*(.rodata._EXCEPTIONS));

    /* reset handler (the main entry point) */
    _reset = .;
    *(.text._reset)

    *(.text*)
  } > FLASH

  data_in_flash_start = .;

  .data :
  {
    data_in_ram_start = .;
    *(.data*)
    data_in_ram_end = .;
  } > RAM

  /DISCARD/ :
  {
    *(.ARM.exidx*)
    *(.note.gnu.build-id*)
  }
}
