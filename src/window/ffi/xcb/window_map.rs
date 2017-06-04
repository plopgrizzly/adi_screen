/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/xcb/window_map.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use ami::void_pointer::*;

extern {
	fn xcb_map_window(c: VoidPointer, window: u32) -> u32;
}

pub fn window_map(window: u32, connection: VoidPointer) {
	unsafe {
		xcb_map_window(connection, window);
	}
}
