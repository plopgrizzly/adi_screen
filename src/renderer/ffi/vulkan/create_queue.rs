/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/vulkan/create_queue.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use ami::void_pointer::*;
use std::ffi::CString;

pub fn create_queue(gpu_interface: VoidPointer, present_queue_index: u32)
	-> usize
{
	let mut queue = 0;

	unsafe {
		extern "system" {
			fn vkGetDeviceProcAddr(instance: VoidPointer,
				name: *const i8)
			-> extern "system" fn(
				physicalDevice: VoidPointer,
				queueFamilyIndex: u32, queueIndex: u32,
				pQueue: *mut usize) -> ();
		}
		let name = CString::new("vkGetDeviceQueue").unwrap();
		(vkGetDeviceProcAddr(gpu_interface, name.as_ptr()))
		(gpu_interface, present_queue_index, 0, &mut queue)
	};

	queue
}
