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
    LONG(_nmi_isr + 1);
    LONG(_hard_fault_isr + 1);

    /* reset handler (the main entry point) */
    _reset = .;
    *(.text._reset)

    *(.text*)
  } > FLASH

  /DISCARD/ :
  {
    *(.ARM.exidx*)
    *(.note.gnu.build-id*)
  }
}
