/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/vulkan/create_queue.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

extern {
	fn vkGetDeviceQueue(physicalDevice: usize, queueFamilyIndex: u32,
		queueIndex: u32, pQueue: *mut usize) -> ();
}

pub fn create_queue(gpu_interface: usize, present_queue_index: u32) -> usize {
	let mut queue = 0;

	unsafe {
		vkGetDeviceQueue(gpu_interface, present_queue_index, 0,
			&mut queue)
	}

	queue
}
