/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/xcb/window_title.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use ami::void_pointer::*;
use std::ffi::CString;

extern {
	fn xcb_change_property(c: VoidPointer, mode: u8, window: u32,
		property: u32, t: u32, format: u8, data_len: u32,
		data: *const i8) -> u32;	
}

pub fn window_title(window: u32, connection: VoidPointer, title: &str) {
	let title_len = title.len() as u32;
	let title = CString::new(title).unwrap();

	unsafe {
		xcb_change_property(connection, 0, window, 39, 31, 8,
			title_len, title.as_ptr());
		xcb_change_property(connection, 0, window, 37, 31, 8,
			title_len, title.as_ptr());
	}
}
