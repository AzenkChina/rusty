extern crate cortex_m_rt;
extern crate cortex_m;
extern crate stm32f0;
extern crate panic_halt;

use cortex_m_rt::exception;
use stm32f0::stm32f0x1::interrupt;

#[repr(C)]
struct __device_ctrl
{
	name: *const u8,
	status: extern fn() -> u8,
	init: extern fn(val: u8) ->u8,
	suspend: extern fn(),
}

#[repr(C)]
struct __led
{
	control: __device_ctrl,
	get: extern fn() -> u8,
	set: extern fn(val: u8) -> u8,
}

#[repr(C)]
struct __power
{
	init: extern fn(),
	status: extern fn() -> u8,
	alter: extern fn(val: u8) -> u8,
}

#[link(name = "devices", kind = "static")]
extern {
	fn SVC_Handler();
	fn PendSV_Handler();
	fn SysTick_Handler();
	fn RTC_IRQHandler();
	fn USART3_8_IRQHandler();
	fn USART1_IRQHandler();
	fn udelay(val:u16);
	fn mdelay(val:u16);
	static power: __power;
	static led:[__led; 2];
}

pub fn usdelay(val: u16) {
	unsafe{ udelay(val); }
}

pub fn msdelay(val: u16) {
	unsafe{ mdelay(val); }
}

pub fn pinit() {
	unsafe{ (power.init)(); }
}

pub fn pstatus() -> u8 {
	unsafe{ (power.status)() }
}

pub fn rledinit(val: u8) {
	unsafe{ (led[0].control.init)(val); }
}

pub fn rledset(val: u8) -> u8 {
	unsafe{ (led[0].set)(val) }
}

pub fn wledinit(val: u8) {
	unsafe{ (led[1].control.init)(val); }
}

pub fn wledset(val: u8) -> u8 {
	unsafe{ (led[1].set)(val) }
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