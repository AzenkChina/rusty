extern crate cortex_m_rt;
extern crate cortex_m;
extern crate stm32f0;
extern crate panic_halt;

use cortex_m_rt::exception;
use stm32f0::stm32f0x1::interrupt;

#[link(name = "devices", kind = "static")]

extern {
	fn SVC_Handler();
	fn PendSV_Handler();
	fn SysTick_Handler();
	fn RTC_IRQHandler();
	fn USART3_8_IRQHandler();
	fn USART1_IRQHandler();
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