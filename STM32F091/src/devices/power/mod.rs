#![allow(non_camel_case_types)]

#[repr(C)]
struct __power {
	init: extern fn(),
	status: extern fn() -> u8,
	alter: extern fn(val: u8) -> u8,
}

extern {
	static power: __power;
}

pub enum __status
{
	AC,
	DC,
	AUX,
	BATTERY,
	WAKEUP,
	REBOOT,
	RESET,
}

pub fn init() {
	unsafe { (power.init)(); }
}

pub fn status()  -> __status {
	let val: u8;

	unsafe { val = (power.status)(); }

    match val {
		0x00 => __status::AC,
		0x01 => __status::DC,
		0x02 => __status::AUX,
		0x03 => __status::BATTERY,
		0x04 => __status::WAKEUP,
		0xf3 => __status::REBOOT,
		0xfc => __status::RESET,
		_ => __status::BATTERY,
    }
}

pub fn alter(val: __status)  -> __status {
	let result: u8;

    match val {
		__status::AC => unsafe { result = (power.alter)(0x00) },
		__status::DC => unsafe { result = (power.alter)(0x01) },
		__status::AUX => unsafe { result = (power.alter)(0x02) },
		__status::BATTERY => unsafe { result = (power.alter)(0x03) },
		__status::WAKEUP => unsafe { result = (power.alter)(0x04) },
		__status::REBOOT => unsafe { result = (power.alter)(0xf3) },
		__status::RESET => unsafe { result = (power.alter)(0xfc) },
    }

    match result {
		0x00 => __status::AC,
		0x01 => __status::DC,
		0x02 => __status::AUX,
		0x03 => __status::BATTERY,
		0x04 => __status::WAKEUP,
		0xf3 => __status::REBOOT,
		0xfc => __status::RESET,
		_ => __status::BATTERY,
    }
}