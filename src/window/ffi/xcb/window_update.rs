/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/xcb/window_update.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use super::LazyPointer;

extern {
	fn xcb_flush(c: LazyPointer) -> i32;
}

pub fn window_update(connection: LazyPointer) {
	unsafe {
		xcb_flush(connection);
	}
}
