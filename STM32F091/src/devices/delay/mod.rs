#![allow(non_camel_case_types)]

extern {
	fn udelay(val: u16);
	fn mdelay(val: u16);
}

pub fn microsecond(val: u16) {
	unsafe { udelay(val); }
}

pub fn millisecond(val: u16) {
	unsafe { mdelay(val); }
}