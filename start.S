.syntax unified
.cpu cortex-m3
.fpu softvfp
.thumb

.text

/* defined in linker.ld */
.extern stack_top

/*
 * The vector table
 *
 * Has to start at address 0x0
 *
 * reference:
 * http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0337e/ch05s09s01.html
 */
.word stack_top
/* bit[0] set means treat as Thumb (which we want)
 *
 * reference:
 * http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.faqs/ka12545.html
 */
.word reset + 1
.word nmi_isr + 1
.word hard_fault_isr + 1

reset:
  /* clear the .bss section */
  ldr r1, =__bss_start
  ldr r2, =__bss_end
  movs r0, #0x00
  b bss_clear_loop
bss_clear_init:
  str r0, [r1], #4
bss_clear_loop:
  cmp r1, r2
  bcc bss_clear_init

  b .

/*
 * Handler for the Non-Maskable Interrupts
 */
nmi_isr:
  b .

/*
 * Handler for Hard Faults
 */
hard_fault_isr:
  b .

/*
 * vi: ft=arm:ts=2:sw=2 expandtab
 */

