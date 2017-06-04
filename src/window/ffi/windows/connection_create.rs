/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "window/ffi/windows/connection_create.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use ami::void_pointer::*;

extern "system" {
	fn GetModuleHandleW(a: VoidPointer) -> VoidPointer;
}

pub fn connection_create() -> VoidPointer {
	unsafe {
		GetModuleHandleW(NULL)
	}
}
