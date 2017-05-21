/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/xcb/create_connection.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use std;
use super::LazyPointer;

extern {
	fn xcb_connect(displayname: *const i8, screenp: *const i32)
		-> LazyPointer;
}

pub fn create_connection() -> LazyPointer {
	let connection = unsafe {
		xcb_connect(std::ptr::null(), std::ptr::null())
	};

	if connection == 0 {
		panic!("Couldn't connect to X Server.");
	}

	connection
}
