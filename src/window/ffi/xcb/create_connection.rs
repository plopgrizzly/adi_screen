/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/xcb/create_connection.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use ami::void_pointer::*;
use std;

extern {
	fn xcb_connect(displayname: *const i8, screenp: *const i32)
		-> VoidPointer;
}

pub fn create_connection() -> VoidPointer {
	let connection = unsafe {
		xcb_connect(std::ptr::null(), std::ptr::null())
	};

	if connection == NULL {
		panic!("Couldn't connect to X Server.");
	}

	connection
}
