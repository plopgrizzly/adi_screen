/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/vulkan/create_instance.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use std::ptr::null;
use std::ffi::CString;
use super::{ LazyPointer, VkResult, VkStructureType, check_error };
use VERSION;

#[cfg(any(target_os = "linux", target_os = "macos"))]
const EXTENSION : &'static str = "VK_KHR_xcb_surface";

#[cfg(target_os = "android")]
const EXTENSION : &'static str = "VK_KHR_android_surface";

#[cfg(target_os = "windows")]
const EXTENSION : &'static str = "VK_KHR_win32_surface";

const VULKAN_VERSION : (u32, &'static str) = (4194304, "VK_API_VERSION_1_0");

#[repr(C)]
struct ExtensionNames {
	s1: *const i8,
	s2: *const i8,
}

#[repr(C)]
struct VkApplicationInfo {
	s_type: VkStructureType,
	p_next: LazyPointer,
	p_application_name: *const i8,
	application_version: u32,
	p_engine_name: *const i8,
	engine_version: u32,
	api_version: u32,
}

#[repr(C)]
struct VkInstanceCreateInfo {
	s_type: VkStructureType,
	p_next: LazyPointer,
	flags: u32,
	p_application_info: *const VkApplicationInfo,
	enabled_layer_count: u32,
	pp_enabled_layer_names: *const *const i8,
	enabled_extension_count: u32,
	pp_enabled_extension_names: *const ExtensionNames,
}

extern {
	fn vkCreateInstance(pCreateInfo: *const VkInstanceCreateInfo,
		pAllocator: LazyPointer, pInstance: *mut usize)
		-> VkResult;
}

pub fn create_instance(app_name: &str) -> usize {
	let mut instance = 0;

	let program_name : *const i8 = CString::new(app_name)
		.unwrap().as_ptr();
	let engine_name : *const i8 = CString::new(VERSION)
		.unwrap().as_ptr();

	// These 2 variables must be defined separately so they stay in scope.
	let s1 = CString::new("VK_KHR_surface").unwrap();
	let s2 = CString::new(EXTENSION).unwrap();

	let instance_create_info = VkInstanceCreateInfo {
		s_type: VkStructureType::InstanceCreateInfo,
		p_next: 0,
		flags: 0,
		p_application_info: &VkApplicationInfo {
			s_type: VkStructureType::ApplicationInfo,
			p_next: 0,
			p_application_name: program_name,
			application_version: 2,
			p_engine_name: engine_name,
			engine_version: 2,
			api_version: VULKAN_VERSION.0,
		},
		enabled_layer_count: 0,
		pp_enabled_layer_names: null(),
		enabled_extension_count: 2,
		pp_enabled_extension_names: &ExtensionNames {
			s1: s1.as_ptr(),
			s2: s2.as_ptr()
		},
	};

	check_error("Failed to create vulkan instance.", unsafe {
		vkCreateInstance(&instance_create_info, 0, &mut instance)
	});

	println!("adi_screen: Program: {}", app_name);
	println!("adi_screen: Engine: {}", VERSION);
	println!("adi_screen: Backend: {}", VULKAN_VERSION.1);

	instance
}
