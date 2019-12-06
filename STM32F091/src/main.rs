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

	devices::pinit();

	devices::usdelay(100);

	devices::rledinit(0x03);
	devices::wledinit(0x03);

    loop {
		devices::msdelay(10);
		devices::pstatus();
		devices::rledset(0x01);
		devices::wledset(0xff);

		devices::msdelay(10);
		devices::pstatus();
		devices::rledset(0xff);
		devices::wledset(0x01);
	}
}