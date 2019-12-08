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

	devices::delay::millisecond(100);

	devices::power::init();

	devices::cpu::core::init(devices::cpu::core::__level::NORMAL);

    loop {
		devices::delay::millisecond(1000);
		devices::cpu::watchdog::feed();

		let status = devices::power::status();

#[cfg(debug_assertions)]
		match status {
			devices::power::__status::AC => hprintln!("AC mode!").unwrap(),
			_ => hprintln!("BATTERY mode!").unwrap(),
		}
	}
}
