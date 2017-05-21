/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/vulkan/create_gpu_interface.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use std::ffi::CString;
use super::{ LazyPointer, VkResult, VkStructureType, check_error };

#[cfg(feature = "checks")]
const NUM_LAYERS : u32 = 1;
#[cfg(not(feature = "checks"))]
const NUM_LAYERS : u32 = 0;

#[repr(C)]
struct VkDeviceQueueCreateInfo {
	s_type: VkStructureType,
	p_next: LazyPointer,
	flags: u32,
	queue_family_index: u32,
	queue_count: u32,
	p_queue_priorities: *const f32,
}

#[repr(C)]
struct VkDeviceCreateInfo {
	s_type: VkStructureType,
	p_next: LazyPointer,
	flags: u32,
	queue_create_info_count: u32,
	p_queue_create_infos: *const VkDeviceQueueCreateInfo,
	enabled_layer_count: u32,
	enabled_layer_names: *const [*const i8; NUM_LAYERS as usize],
	enabled_extension_count: u32,
	enabled_extension_names: *const *const i8,
	enabled_features: LazyPointer,
}

extern {
//	fn vkCreateDevice(;
}

#[cfg(feature = "checks")]
fn layers() -> (CString) {
	let s1 = CString::new("VK_LAYER_LUNARG_standard_validation").unwrap();
	(s1)
}

#[cfg(feature = "checks")]
fn layer_names(layer: &(CString)) -> [*const i8; NUM_LAYERS as usize] {
	[ layer.as_ptr() ]
}

#[cfg(not(feature = "checks"))]
fn layers() -> ( ) {
	( )
}

#[cfg(not(feature = "checks"))]
fn layer_names(_: &()) -> [*const i8; NUM_LAYERS as usize] {
	[ ]
}

pub fn create_gpu_interface(instance: usize, gpu: usize,
	present_queue_index: u32) -> usize
{
	let mut device = 0;
	let ext = CString::new("VK_KHR_swapchain").unwrap();
	let lay = layers();
	let create_info = VkDeviceCreateInfo {
		s_type: VkStructureType::DeviceCreateInfo,
		p_next: 0,
		flags: 0,
		queue_create_info_count: 1,
		p_queue_create_infos: &VkDeviceQueueCreateInfo {
			s_type: VkStructureType::DeviceQueueCreateInfo,
			p_next: 0,
			flags: 0,
			queue_family_index: present_queue_index,
			queue_count: 1,
			p_queue_priorities: &1.0,
		},
		enabled_layer_count: NUM_LAYERS,
		enabled_layer_names: &layer_names(&lay),
		enabled_extension_count: 1,
		enabled_extension_names: &ext.as_ptr(),
		enabled_features: 0,
	};

	unsafe {
		extern "system" {
			fn vkGetInstanceProcAddr(instance: LazyPointer,
				name: *const i8)
			-> extern "system" fn(
				physicalDevice: usize,
				pCreateInfo: *const VkDeviceCreateInfo,
				pAllocator: LazyPointer,
				pDevice: *mut usize) -> VkResult;
		}
		let name = CString::new("vkCreateDevice").unwrap();
		check_error("vkCreateDevice failure.",
			(vkGetInstanceProcAddr(instance, name.as_ptr()))
			(gpu, &create_info, 0, &mut device)
		);
	};

	device
}
