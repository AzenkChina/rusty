#![allow(non_camel_case_types)]

#[repr(C)]
struct __core {
	details: extern fn() -> *const u8,
	endian: extern fn() -> u8,
	init: extern fn(val: u8),
	status: extern fn() -> u8,
	sleep: extern fn(),
	reset: extern fn(),
	idle: extern fn(val: u16),
}

#[repr(C)]
struct __interrupt {
	disable: extern fn(),
	enable: extern fn(),
	status: extern fn() -> u8,
	request: extern fn(),
	release: extern fn(),
}

#[repr(C)]
struct __watchdog {
	overflow: u32,
	feed:  extern fn(),
}

#[repr(C)]
struct __cpu {
	core: __core,
	interrupt: __interrupt,
	watchdog: __watchdog,
}

extern {
	static cpu: __cpu;
}

pub enum __status {
	DISABLED,
	ENABLED,
}

pub fn disable() {
	unsafe { (cpu.interrupt.disable)() }
}

pub fn enable() {
	unsafe { (cpu.interrupt.enable)() }
}

pub fn status() -> __status {
	let val: u8;

	unsafe { val = (cpu.interrupt.status)() }

    match val {
        0x00 => __status::DISABLED,
        _ => __status::ENABLED,
    }
}
