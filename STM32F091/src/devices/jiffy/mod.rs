#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[repr(C)]
struct __jiffy {
	value: extern fn() -> u32,
	after: extern fn(val: u32) -> u32,
}

extern {
	static jiffy: __jiffy;
}

pub fn value()  -> u32 {
	unsafe { (jiffy.value)() }
}

pub fn after(val: u32)  -> u32 {
	unsafe { (jiffy.after)(val) }
}
