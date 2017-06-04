/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/xcb/window_update.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use ami::void_pointer::*;

extern {
	fn xcb_flush(c: VoidPointer) -> i32;
}

pub fn window_update(connection: VoidPointer) {
	unsafe {
		xcb_flush(connection);
	}
}
