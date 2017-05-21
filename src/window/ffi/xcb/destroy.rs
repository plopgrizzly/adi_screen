/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/vulkan/destroy.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use super::LazyPointer;

extern {
	fn xcb_disconnect(c: LazyPointer) -> ();
}

pub fn connection(connection: LazyPointer) -> () {
	unsafe {
		xcb_disconnect(connection);
	}
}
