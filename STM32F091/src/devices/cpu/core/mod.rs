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

pub enum __endian {
	LITTLE_ENDIAN,
	BIG_ENDIAN,
	UNKNOWN_ENDIAN,
}

pub enum __level {
    NORMAL,
    POWERSAVE,
}

pub fn endian() -> __endian {
	let val: u8;

	unsafe { val = (cpu.core.endian)() }

    match val {
        0x55 => __endian::LITTLE_ENDIAN,
        0xAA => __endian::BIG_ENDIAN,
		_ => __endian::UNKNOWN_ENDIAN,
    }
}

pub fn init(val: __level) {
    match val {
        __level::NORMAL => unsafe { (cpu.core.init)(0x00) },
        __level::POWERSAVE => unsafe { (cpu.core.init)(0x01) },
    }
}

pub fn status() -> __level {
	let val: u8;

	unsafe { val = (cpu.core.status)() }

    match val {
		0 => __level::NORMAL,
		_ => __level::POWERSAVE,
    }
}

pub fn sleep() {
	unsafe { (cpu.core.sleep)(); }
}

pub fn reset() {
	unsafe { (cpu.core.reset)(); }
}

pub fn idle(val: u16) {
	unsafe { (cpu.core.idle)(val); }
}
