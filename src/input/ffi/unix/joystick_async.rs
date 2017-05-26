/*
 * adi_screen - Aldaron's Device Interface
 * Screen - "input/ffi/unix/joystick_async.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
 */

extern {
	fn fcntl(fd: i32, cmd: i32, v: i32) -> i32;
}

pub fn joystick_async(fd: i32) -> () {
	let error = unsafe {
		fcntl(fd, 0x4, 0x800)
	} == -1;

	if error {
		panic!("Joystick unplugged 2!");
	}
}
