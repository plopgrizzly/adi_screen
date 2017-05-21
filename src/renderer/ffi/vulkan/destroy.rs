/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/vulkan/destroy.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use super::LazyPointer;

extern {
	fn vkDestroyInstance(instance: usize, pAllocator: LazyPointer) -> ();
	fn vkDestroySurfaceKHR(instance: usize, surface: u64,
		pAllocator: LazyPointer) -> ();
}

#[allow(dead_code)]
pub fn instance(instance: usize) -> () {
	unsafe {
		vkDestroyInstance(instance, 0);
	}
}

#[allow(dead_code)]
pub fn surface(instance: usize, surface: u64) -> () {
	unsafe {
		vkDestroySurfaceKHR(instance, surface, 0);
	}
}
