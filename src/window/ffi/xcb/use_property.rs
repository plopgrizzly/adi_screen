/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/xcb/use_property.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use ami::void_pointer::*;

extern {
	fn xcb_change_property(c: VoidPointer, mode: u8, window: u32,
		property: u32, t: u32, format: u8, data_len: u32,
		data: *const u32) -> u32;
}

pub fn use_property(window: u32, connection: VoidPointer, a: u32, a2: u32) {
	unsafe {
		xcb_change_property(connection, 0, window, a, 4, 32, 1, &a2);
	}
}
