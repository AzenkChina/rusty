#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate cortex_m;
extern crate stm32f0;
extern crate panic_halt;
#[cfg(debug_assertions)]
extern crate cortex_m_semihosting;

use cortex_m_rt::{entry, exception};
use stm32f0::stm32f0x1::{interrupt};
#[cfg(debug_assertions)]
use cortex_m_semihosting::hprintln;


#[link(name = "devices", kind = "static")]
extern {
	fn SVC_Handler();
	fn PendSV_Handler();
	fn SysTick_Handler();
	fn RTC_IRQHandler();
	fn USART3_8_IRQHandler();
	fn USART1_IRQHandler();
	fn udelay(v: u16);
	fn mdelay(v: u16);
}

#[entry]
fn main() -> ! {
#[cfg(debug_assertions)]
	hprintln!("Call C function.").unwrap();
	unsafe{ udelay(100); }

#[cfg(debug_assertions)]
	hprintln!("Call C function.").unwrap();
	unsafe{ mdelay(100); }

#[cfg(debug_assertions)]
	hprintln!("Hello, World!").unwrap();

    loop {
		unsafe{ mdelay(100); }
    }
}

#[exception]
fn DefaultHandler(irqn: i16) {
	panic!("Unhandled exception (IRQn = {})", irqn);
}

#[exception]
fn NonMaskableInt() {

}

#[exception]
fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
	panic!("HardFault at {:#?}", ef);
}

#[exception]
fn SVCall() {
	unsafe{ SVC_Handler(); }
}

#[exception]
fn PendSV() {
	unsafe{ PendSV_Handler(); }
}

#[exception]
fn SysTick() {
	unsafe{ SysTick_Handler(); }
}

#[interrupt]
fn RTC() {
	unsafe{ RTC_IRQHandler(); }
}

#[interrupt]
fn USART1() {
	unsafe{ USART1_IRQHandler(); }
}

#[interrupt]
fn USART3_4_5_6_7_8() {
	unsafe{ USART3_8_IRQHandler(); }
}