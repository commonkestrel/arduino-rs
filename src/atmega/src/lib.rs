#![no_std]
#![feature(lang_items, asm_experimental_arch, abi_avr_interrupt, error_in_core)]

pub mod pins;
pub mod registers;
pub mod prelude;
pub mod timer;
pub mod volatile;
pub mod interrupt;
#[cfg(feature = "interrupt-macro")]
pub use atmega_macros::interrupt;

use core::panic::PanicInfo;

#[doc(hidden)]
pub fn init() {
    #[cfg(feature = "millis")]
    timer::begin_systick();
}

/// Takes two arguments, `setup()` and `run()`
#[macro_export]
macro_rules! run {
    ($setup: ident, $loop: ident) => {
        #[no_mangle]
        pub extern "C" fn main() -> ! {
            $crate::init();
            
            let mut state = $setup();
            loop{ $loop(&mut state) }
        }
    }
}

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}
