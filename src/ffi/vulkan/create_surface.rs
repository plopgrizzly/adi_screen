/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/vulkan/create_surface.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use ffi::NativeWindow;
use super::{ LazyPointer, VkResult, VkStructureType, check_error };

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[repr(C)]
struct SurfaceCreateInfo {
	s_type: VkStructureType,
	p_next: LazyPointer,
	flags: u32,
	connection: LazyPointer,
	window: u32,
}

#[cfg(target_os = "windows")]
#[repr(C)]
struct SurfaceCreateInfo {
	s_type: VkStructureType,
	p_next: LazyPointer,
	flags: u32,
	// TODO
	hinstance: usize,
	hwnd: usize,
}

#[cfg(target_os = "android")]
#[repr(C)]
struct SurfaceCreateInfo {
	s_type: VkStructureType,
	p_next: LazyPointer,
	flags: u32,
	window: *mut ANativeWindow,
}

const ERROR : &'static str = "Failed to create surface.";

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn create_surface(instance: usize, native_window: &NativeWindow) -> u64 {
	let mut surface = 0;
	let surface_create_info = SurfaceCreateInfo {
		s_type: VkStructureType::SurfaceCreateInfo,
		p_next: 0,
		flags: 0,
		connection: native_window.connection,
		window: native_window.window,
	};

	unsafe {
		extern "system" {
			fn vkCreateXcbSurfaceKHR(
				instance: usize,
				pCreateInfo: *const SurfaceCreateInfo,
				pAllocator: LazyPointer,
				surface: *mut u64) -> VkResult;
		}
		check_error(ERROR,
			vkCreateXcbSurfaceKHR(instance, &surface_create_info, 0, &mut surface)
		);
	};

	surface
}

#[cfg(target_os = "windows")]
pub fn create_surface(instance: usize, native_window: &NativeWindow) -> u64 {
	let mut surface = 0;
	let surface_create_info = SurfaceCreateInfo {
		s_type: VkStructureType::SurfaceCreateInfo,
		p_next: 0,
		flags: 0,
		hinstance: native_window.connection,
		hwnd: native_window.window,
	};

	unsafe {
		extern "system" {
			fn vkCreateWin32SurfaceKHR(
				instance: usize,
				pCreateInfo: *const SurfaceCreateInfo,
				pAllocator: LazyPointer,
				surface: *mut u64) -> VkResult;
		}
		check_error(ERROR,
			vkCreateWin32SurfaceKHR(instance, &surface_create_info, 0, &mut surface)
		);
	};

	surface
}

#[cfg(target_os = "android")]
pub fn create_surface(instance: usize, native_window: &NativeWindow) -> u64 {
	let mut surface = 0;
	let surface_create_info = SurfaceCreateInfo {
		s_type: VkStructureType::SurfaceCreateInfo,
		p_next: 0,
		flags: 0,
		window: native_window.window,
	};

	unsafe {
		extern "system" {
			fn vkCreateAndroidSurfaceKHR(instance: usize,
				pCreateInfo: *const SurfaceCreateInfo,
				pAllocator: LazyPointer,
				surface: *mut u64) -> VkResult;
		}
		check_error(ERROR,
			vkCreateAndroidSurfaceKHR(instance, &surface_create_info, 0, &mut surface)
		);
	};

	surface
}