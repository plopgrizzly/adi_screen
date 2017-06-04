/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "window/ffi/windows/connection_create.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use ami::VoidPointer;

extern "system" {
	fn GetModuleHandleW(a: VoidPointer) -> VoidPointer;
}

pub fn connection_create() -> VoidPointer {
	unsafe {
		GetModuleHandleW(0)
	}
}
