/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "window/ffi/windows/connection_create.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use super::LazyPointer;

extern "system" {
	fn GetModuleHandleW(a: LazyPointer) -> LazyPointer;
}

pub fn connection_create() -> LazyPointer {
	unsafe {
		GetModuleHandleW(0)
	}
}