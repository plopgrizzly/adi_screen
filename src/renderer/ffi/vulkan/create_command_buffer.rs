/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/vulkan/create_command_buffer.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use ami::void_pointer::*;
use std::ffi::CString;
use super::{ VkResult, VkStructureType, check_error };

#[repr(C)]
enum VkCommandBufferLevel {
	Primary = 0,
}

#[repr(C)]
struct VkCommandPoolCreateInfo {
	s_type: VkStructureType,
	p_next: VoidPointer,
	flags: u32,
	queue_family_index: u32,
}

#[repr(C)]
struct VkCommandBufferAllocateInfo {
	s_type: VkStructureType,
	p_next: VoidPointer,
	command_pool: u64,
	level: VkCommandBufferLevel,
	command_buffer_count: u32,
}

pub fn create_command_buffer(gpu_interface: VoidPointer,
	present_queue_index: u32) -> (VoidPointer, u64)
{
	let mut command_pool = 0;
	let mut command_buffer = NULL;

	let create_info = VkCommandPoolCreateInfo {
		s_type: VkStructureType::CommandPoolCreateInfo,
		p_next: NULL,
		flags: 0x00000002, // Reset Command Buffer
		queue_family_index: present_queue_index,
	};

	unsafe {
		extern "system" {
			fn vkGetDeviceProcAddr(instance: VoidPointer,
				name: *const i8)
			-> extern "system" fn(
				device: VoidPointer,
				pCreateInfo: *const VkCommandPoolCreateInfo,
				pAllocator: VoidPointer,
				pCommandPool: *mut u64) -> VkResult;
		}
		let name = CString::new("vkCreateCommandPool").unwrap();
		check_error("Failed to create vulkan instance.",
			(vkGetDeviceProcAddr(gpu_interface, name.as_ptr()))
			(gpu_interface, &create_info, NULL, &mut command_pool)
		);
	};

	let allocate_info = VkCommandBufferAllocateInfo {
		s_type: VkStructureType::CommandBufferAllocateInfo,
		p_next: NULL,
		command_pool: command_pool,
		level: VkCommandBufferLevel::Primary,
		command_buffer_count: 1,
	};

	unsafe {
		extern "system" {
			fn vkGetDeviceProcAddr(instance: VoidPointer,
				name: *const i8)
			-> extern "system" fn(
				device: VoidPointer,
				ai: *const VkCommandBufferAllocateInfo,
				cmd_buffs: *mut VoidPointer) -> VkResult;
		}
		let name = CString::new("vkAllocateCommandBuffers").unwrap();
		check_error("Failed to create vulkan instance.",
			(vkGetDeviceProcAddr(gpu_interface, name.as_ptr()))
			(gpu_interface, &allocate_info, &mut command_buffer)
		);
	};

	(command_buffer, command_pool)
}
