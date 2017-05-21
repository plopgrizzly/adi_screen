/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/xcb/create_window.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use super::{ LazyPointer, MWW, MWH };

#[repr(C)]
struct ValueList {
	a: u32,
	b: u32,
}

extern {
	fn xcb_generate_id(c: LazyPointer) -> u32;
	fn xcb_create_window(c: LazyPointer, depth: u8, wid: u32,
		parent: u32, x: i16, y: i16, width: u16, height: u16,
		border_width: u16, _class: u16, visual: u32, value_mask: u32,
		value_list: *mut ValueList) -> u32;
}

pub fn create_window(connection: LazyPointer, root: u32, visual: u32,
	black: u32) -> u32
{
	let window = unsafe {
		xcb_generate_id(connection)
	};

	let mut value_list = ValueList {
		a: black, b: 1|2|4|8|16|32|64|131072|2097152,
	};

	unsafe {
		xcb_create_window(connection, 0, window, root, 0, 0, MWW, MWH,
			0, 1, visual, 2|2048, &mut value_list);
	}

	window
}
