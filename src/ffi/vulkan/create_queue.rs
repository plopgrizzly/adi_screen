/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/vulkan/create_queue.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use std::ffi::CString;

pub fn create_queue(gpu_interface: usize, present_queue_index: u32) -> usize {
	let mut queue = 0;

	unsafe {
		extern "system" {
			fn vkGetDeviceProcAddr(instance: usize,
				name: *const i8)
			-> extern "system" fn(
				physicalDevice: usize, queueFamilyIndex: u32,
				queueIndex: u32, pQueue: *mut usize) -> ();
		}
		let name = CString::new("vkGetDeviceQueue").unwrap();
		(vkGetDeviceProcAddr(gpu_interface, name.as_ptr()))
		(gpu_interface, present_queue_index, 0, &mut queue)
	};

	queue
}