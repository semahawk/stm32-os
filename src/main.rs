#![feature(lang_items)]
#![feature(asm)]
#![no_main]
#![no_std]

#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
  /// Base address of the RCC block
  const RCC: u32 = 0x4002_1000;
  /// Address of the APB2ENR register
  const RCC_APB2ENR: u32 = RCC + 0x18;
  /// Mask of the bit that is in charge of enabling/disabling the GPIOA port
  const RCC_APB2ENR_IOPAEN: u32 = 1 << 2;

  unsafe {
    // Enable the GPIO port A block
    *(RCC_APB2ENR as *mut u32) |= RCC_APB2ENR_IOPAEN;
  }

  /// Base address of the GPIO port A block
  const GPIOA: u32 = 0x4001_0800;
  /// Address of the CRL register of the GPIO port A block
  const GPIOA_CRL: u32 = GPIOA + 0x00;
  /// Address of the BSRR register of the GPIO port A block
  const GPIOA_BSRR: u32 = GPIOA + 0x10;

  unsafe {
    // Set GPIOA pin 5's mode to output/push-pull
    let gpioa_crl = GPIOA_CRL as *mut u32;
    *gpioa_crl = (*gpioa_crl & !(0b1111 << 20)) | (0b0001 << 20);
  }

  loop {
    unsafe {
      // Drive GPIOA pin 5 high
      *(GPIOA_BSRR as *mut u32) = 1 << 5;
      for _ in 0..10_000 {}
      // Drive GPIOA pin 5 low
      *(GPIOA_BSRR as *mut u32) = 1 << (16 + 5);
      for _ in 0..10_000 {}
    }
  }
}

mod exception {
  pub extern "C" fn dummy_handler() {
    unsafe { asm!("bkpt"); }
    loop {}
  }

  #[export_name = "_EXCEPTIONS"]
  pub static EXCEPTIONS: [Option<extern "C" fn()>; 14] = [
    Some(dummy_handler), // NMI
    Some(dummy_handler), // Hard fault
    Some(dummy_handler), // Memmanage fault
    Some(dummy_handler), // Bus fault
    Some(dummy_handler), // Usage fault
    None, // Reserved
    None, // Reserved
    None, // Reserved
    None, // Reserved
    Some(dummy_handler), // SVC call
    None, // Reserved for debug
    None, // Reserved
    Some(dummy_handler), // PendSV
    Some(dummy_handler), // Systick
  ];
}

mod lang_items {
  #[lang = "panic_fmt"]
  #[no_mangle]
  extern "C" fn panic_fmt() { loop {} }
}

/*
 * vi: ts=2:sw=2 expandtab
 */

