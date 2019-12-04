#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate stm32f0;
#[cfg(debug_assertions)]
extern crate cortex_m_semihosting;

use cortex_m_rt::entry;
#[cfg(debug_assertions)]
use cortex_m_semihosting::hprintln;

mod devices;

#[entry]
fn main() -> ! {
#[cfg(debug_assertions)]
	hprintln!("Hello, World!").unwrap();

	devices::msdelay(100);

    loop {
		devices::usdelay(1000);
	}
}