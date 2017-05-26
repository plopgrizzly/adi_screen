/*
 * adi_screen - Aldaron's Device Interface
 * Screen - "input/ffi/unix/joystick_create.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
 */

use std::ffi::CString;

extern {
	fn open(pathname: *const i8, flags: i32) -> i32;
}

fn open_joystick(name: &str) -> i32 {
	let file_name = CString::new(name).unwrap();

	unsafe {
		open(file_name.as_ptr(), 0)
	}
}

pub fn joystick_create() -> i32 {
	let joystick = open_joystick("/dev/js0");

	if joystick != -1 {
		return joystick;
	}

	let joystick = open_joystick("/dev/input/js0");

	joystick
}
