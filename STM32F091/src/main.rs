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
use devices::{*};

#[entry]
fn main() -> ! {
#[cfg(debug_assertions)]
	hprintln!("Hello, World!").unwrap();

	delay::millisecond(100);

	power::init();

	cpu::core::init(cpu::core::__level::NORMAL);

	led::relay::init(led::relay::__dev_state::NORMAL);
	led::warning::init(led::warning::__dev_state::NORMAL);

    loop {
		cpu::watchdog::feed();
		delay::millisecond(1000);
		led::relay::set(led::relay::__status::ON);
		led::warning::set(led::warning::__status::OFF);

		cpu::watchdog::feed();
		delay::millisecond(1000);
		led::relay::set(led::relay::__status::OFF);
		led::warning::set(led::warning::__status::ON);

#[cfg(debug_assertions)]
		let status = power::status();
#[cfg(debug_assertions)]
		match status {
			power::__status::AC => hprintln!("AC mode!").unwrap(),
			_ => hprintln!("BATTERY mode!").unwrap(),
		}
	}
}
