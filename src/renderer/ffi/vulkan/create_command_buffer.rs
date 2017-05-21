/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/vulkan/create_command_buffer.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use std::ffi::CString;
use super::{ LazyPointer, VkResult, VkStructureType, check_error };

#[repr(C)]
enum VkCommandBufferLevel {
	Primary = 0,
}

#[repr(C)]
struct VkCommandPoolCreateInfo {
	s_type: VkStructureType,
	p_next: LazyPointer,
	flags: u32,
	queue_family_index: u32,
}

#[repr(C)]
struct VkCommandBufferAllocateInfo {
	s_type: VkStructureType,
	p_next: LazyPointer,
	command_pool: u64,
	level: VkCommandBufferLevel,
	command_buffer_count: u32,
}

pub fn create_command_buffer(gpu_interface: usize, present_queue_index: u32)
	-> (usize, u64)
{
	let mut command_pool = 0;
	let mut command_buffer = 0;

	let create_info = VkCommandPoolCreateInfo {
		s_type: VkStructureType::CommandPoolCreateInfo,
		p_next: 0,
		flags: 0x00000002, // Reset Command Buffer
		queue_family_index: present_queue_index,
	};

	unsafe {
		extern "system" {
			fn vkGetDeviceProcAddr(instance: LazyPointer,
				name: *const i8)
			-> extern "system" fn(
				device: usize,
				pCreateInfo: *const VkCommandPoolCreateInfo,
				pAllocator: LazyPointer,
				pCommandPool: *mut u64) -> VkResult;
		}
		let name = CString::new("vkCreateCommandPool").unwrap();
		check_error("Failed to create vulkan instance.",
			(vkGetDeviceProcAddr(gpu_interface, name.as_ptr()))
			(gpu_interface, &create_info, 0, &mut command_pool)
		);
	};

	let allocate_info = VkCommandBufferAllocateInfo {
		s_type: VkStructureType::CommandBufferAllocateInfo,
		p_next: 0,
		command_pool: command_pool,
		level: VkCommandBufferLevel::Primary,
		command_buffer_count: 1,
	};

	unsafe {
		extern "system" {
			fn vkGetDeviceProcAddr(instance: LazyPointer,
				name: *const i8)
			-> extern "system" fn(
				device: usize,
				ai: *const VkCommandBufferAllocateInfo,
				cmd_buffs: *mut usize) -> VkResult;
		}
		let name = CString::new("vkAllocateCommandBuffers").unwrap();
		check_error("Failed to create vulkan instance.",
			(vkGetDeviceProcAddr(gpu_interface, name.as_ptr()))
			(gpu_interface, &allocate_info, &mut command_buffer)
		);
	};

	(command_buffer, command_pool)
}
