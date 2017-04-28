/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/vulkan/select_gpu.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use std::ptr::null_mut;
use super::{ VkResult, check_error };

#[repr(C)]
#[derive(Copy, Clone)]
struct VkExtent3D {
	width: u32,
	height: u32,
	depth: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct VkQueueFamilyProperties {
	queue_flags: u32,
	queue_count: u32,
	timestamp_valid_bits: u32,
	min_image_transfer_granularity: VkExtent3D,
}

extern {
	fn vkEnumeratePhysicalDevices(instance: usize,
		pPhysicalDeviceCount : *mut u32,
		pPhysicalDevices: *mut usize) -> VkResult;
	fn vkGetPhysicalDeviceQueueFamilyProperties(
		physicalDevice: usize,
		pQueueFamilyPropertyCount: *mut u32,
		pQueueFamilyProperties: *mut VkQueueFamilyProperties) -> ();
	fn vkGetPhysicalDeviceSurfaceSupportKHR(
		physicalDevice: usize,
		queueFamilyIndex: u32,
		surface: u64,
		psupported: *mut u32) -> VkResult;
}

pub fn create_gpu(instance: usize, surface: u64) -> (usize, u32) {
	let mut num_gpus = 0;
	let mut gpus;

	unsafe {
		check_error("vkEnumeratePhysicalDevices(null) failed!",
			vkEnumeratePhysicalDevices(instance, &mut num_gpus,
				null_mut()));
		gpus = vec![0; num_gpus as usize];
		check_error("vkEnumeratePhysicalDevices(gpus) failed!",
			vkEnumeratePhysicalDevices(instance, &mut num_gpus,
				gpus.as_mut_ptr()));
	}

	for i in 0..(num_gpus as usize) {
		let mut num_queue_families = 0;

		unsafe {
			vkGetPhysicalDeviceQueueFamilyProperties(gpus[i],
				&mut num_queue_families, null_mut());
		}

		let mut properties = vec![VkQueueFamilyProperties {
			queue_flags: 0,
			queue_count: 0,
			timestamp_valid_bits: 0,
			min_image_transfer_granularity: VkExtent3D {
				width: 0, height: 0, depth: 0,
			},
		}; num_queue_families as usize];

		unsafe {
			vkGetPhysicalDeviceQueueFamilyProperties(gpus[i],
				&mut num_queue_families,
				properties.as_mut_ptr());		
		}

		for j in 0..(num_queue_families as usize) {
			let k = j as u32;
			let mut supports_present = 0;

			unsafe {
				check_error("vkGetPhysicalDeviceSurfaceSupport",
					vkGetPhysicalDeviceSurfaceSupportKHR(
						gpus[i], k, surface,
						&mut supports_present));
			}
			if supports_present != 0 &&
				(properties[j].queue_flags & 0x00000001) != 0
			{
				return (gpus[i], k);
			}
		}
	}

	panic!("Couldn't Create Gpu.");
}
