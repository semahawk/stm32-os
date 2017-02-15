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
  /* just to see that PC is being incremented */
  mov r0, #0
  add_one:
  add r0, r0, 1
  b add_one

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

