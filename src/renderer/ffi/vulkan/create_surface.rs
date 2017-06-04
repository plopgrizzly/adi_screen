/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/vulkan/create_surface.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use ami::void_pointer::*;
use window::NativeWindow;
use super::{ VkResult, VkStructureType, check_error };

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[repr(C)]
struct SurfaceCreateInfo {
	s_type: VkStructureType,
	p_next: VoidPointer,
	flags: u32,
	connection: VoidPointer,
	window: u32,
}

#[cfg(target_os = "windows")]
#[repr(C)]
struct SurfaceCreateInfo {
	s_type: VkStructureType,
	p_next: VoidPointer,
	flags: u32,
	// TODO
	hinstance: usize,
	hwnd: usize,
}

#[cfg(target_os = "android")]
#[repr(C)]
struct SurfaceCreateInfo {
	s_type: VkStructureType,
	p_next: VoidPointer,
	flags: u32,
	window: *mut ANativeWindow,
}

const ERROR : &'static str = "Failed to create surface.";

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn create_surface(instance: VoidPointer, native_window: &NativeWindow)
	-> u64
{
	let mut surface = 0;
	let surface_create_info = SurfaceCreateInfo {
		s_type: VkStructureType::SurfaceCreateInfo,
		p_next: NULL,
		flags: 0,
		connection: native_window.get_connection(),
		window: native_window.get_window(),
	};

	unsafe {
		extern "system" {
			fn vkCreateXcbSurfaceKHR(
				instance: VoidPointer,
				pCreateInfo: *const SurfaceCreateInfo,
				pAllocator: VoidPointer,
				surface: *mut u64) -> VkResult;
		}
		check_error(ERROR,
			vkCreateXcbSurfaceKHR(instance, &surface_create_info, NULL, &mut surface)
		);
	};

	surface
}

#[cfg(target_os = "windows")]
pub fn create_surface(instance: VoidPointer, native_window: &NativeWindow)
	-> u64
{
	let mut surface = NULL;
	let surface_create_info = SurfaceCreateInfo {
		s_type: VkStructureType::SurfaceCreateInfo,
		p_next: 0,
		flags: 0,
		hinstance: native_window.get_connection(),
		hwnd: native_window.get_window(),
	};

	unsafe {
		extern "system" {
			fn vkCreateWin32SurfaceKHR(
				instance: VoidPointer,
				pCreateInfo: *const SurfaceCreateInfo,
				pAllocator: VoidPointer,
				surface: *mut u64) -> VkResult;
		}
		check_error(ERROR,
			vkCreateWin32SurfaceKHR(instance, &surface_create_info, 0, &mut surface)
		);
	};

	surface
}

#[cfg(target_os = "android")]
pub fn create_surface(instance: VoidPointer, native_window: &NativeWindow)
	-> u64
{
	let mut surface = NULL;
	let surface_create_info = SurfaceCreateInfo {
		s_type: VkStructureType::SurfaceCreateInfo,
		p_next: 0,
		flags: 0,
		window: native_window.get_window(),
	};

	unsafe {
		extern "system" {
			fn vkCreateAndroidSurfaceKHR(instance: VoidPointer,
				pCreateInfo: *const SurfaceCreateInfo,
				pAllocator: VoidPointer,
				surface: *mut u64) -> VkResult;
		}
		check_error(ERROR,
			vkCreateAndroidSurfaceKHR(instance, &surface_create_info, 0, &mut surface)
		);
	};

	surface
}
