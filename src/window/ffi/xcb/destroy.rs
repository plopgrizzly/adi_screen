/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/vulkan/destroy.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use ami::void_pointer::*;

extern {
	fn xcb_disconnect(c: VoidPointer) -> ();
}

pub fn connection(connection: VoidPointer) -> () {
	unsafe {
		xcb_disconnect(connection);
	}
}
