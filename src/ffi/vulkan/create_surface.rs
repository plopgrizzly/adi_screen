/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/vulkan/create_surface.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use ffi::NativeWindow;
use super::{ LazyPointer, VkResult, VkStructureType, check_error };

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[repr(C)]
struct VkXcbSurfaceCreateInfo {
	s_type: VkStructureType,
	p_next: LazyPointer,
	flags: u32,
	connection: LazyPointer,
	window: u32,
}

#[cfg(target_os = "windows")]
#[repr(C)]
struct VkWin32SurfaceCreateInfo {
	s_type: VkStructureType,
	p_next: LazyPointer,
	flags: u32,
	// TODO
	hinstance: HINSTANCE,
	hwnd: HWND,
}

#[cfg(target_os = "android")]
#[repr(C)]
struct VkAndroidSurfaceCreateInfo {
	s_type: VkStructureType,
	p_next: LazyPointer,
	flags: u32,
	window: *mut ANativeWindow,
}

const ERROR : &'static str = "Failed to create surface.";

#[cfg(any(target_os = "linux", target_os = "macos"))]
extern {
	fn vkCreateXcbSurfaceKHR(instance: usize,
		pCreateInfo: *const VkXcbSurfaceCreateInfo,
		pAllocator: LazyPointer,
		surface: *mut u64) -> VkResult;
}

#[cfg(target_os = "windows")]
extern {
	fn vkCreateWin32SurfaceKHR(instance: usize,
		pCreateInfo: *const VkWin32SurfaceCreateInfo,
		pAllocator: LazyPointer,
		surface: *mut u64) -> VkResult;
}

#[cfg(target_os = "android")]
extern {
	fn vkCreateAndroidSurfaceKHR(instance: usize,
		pCreateInfo: *const VkAndroidSurfaceCreateInfo,
		pAllocator: LazyPointer,
		surface: *mut u64) -> VkResult;
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn create_surface(instance: usize, native_window: &NativeWindow) -> u64 {
	let mut surface = 0;
	let surface_create_info = VkXcbSurfaceCreateInfo {
		s_type: VkStructureType::SurfaceCreateInfo,
		p_next: 0,
		flags: 0,
		connection: native_window.connection,
		window: native_window.window,
	};

	check_error(ERROR, unsafe {
		vkCreateXcbSurfaceKHR(instance, &surface_create_info, 0,
			&mut surface)
	});

	surface
}

#[cfg(target_os = "windows")]
pub fn create_surface(instance: usize, native_window: &NativeWindow) -> u64 {
	let mut surface = 0;
	let surface_create_info = VkWin32SurfaceCreateInfo {
		s_type: VkStructureType::SurfaceCreateInfo,
		p_next: 0,
		flags: 0,
		hinstance: native_window.connection,
		hwnd: native_window.window,
	};

	check_error(ERROR, unsafe {
		vkCreateWin32SurfaceKHR(instance.native, &surface_create_info,
			0, &mut surface);
	});

	surface
}

#[cfg(target_os = "android")]
pub fn create_surface(instance: usize, native_window: &NativeWindow) -> u64 {
	let mut surface = 0;
	let surface_create_info = VkAndroidSurfaceCreateInfo {
		s_type: VkStructureType::SurfaceCreateInfo,
		p_next: 0,
		flags: 0,
		window: native_window.window,
	};

	check_error(ERROR, unsafe {
		vkCreateAndroidSurfaceKHR(instance.native, &surface_create_info,
			0, &mut surface)
	});

	surface
}
