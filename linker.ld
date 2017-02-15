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
    text_end = .;
  } >flash

  . = ALIGN(4);
  data_start_in_flash = .;

  .data : AT(text_end)
  {
    . = ALIGN(4);
    data_start_in_sram = .;

    *(.data)
    *(.data.*)

    stack_bottom = .;
    . += 2K;
    stack_top = ORIGIN(sram) + LENGTH(sram);

    data_end_in_sram = .;
  } >sram

  data_end_in_flash = .;

  .bss (NOLOAD):
  {
    . = ALIGN(4);
    __bss_start = .;

    *(.bss)
    *(.bss.*)

    __bss_end = .;
  } >sram
}
