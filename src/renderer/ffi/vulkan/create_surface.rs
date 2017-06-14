/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/vulkan/create_surface.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use ami::void_pointer::*;
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
	hinstance: VoidPointer,
	hwnd: VoidPointer,
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
pub fn create_surface_xcb(instance: VoidPointer, connection: VoidPointer,
	window: u32) -> u64
{
	let mut surface = 0;
	let surface_create_info = SurfaceCreateInfo {
		s_type: VkStructureType::SurfaceCreateInfo,
		p_next: NULL,
		flags: 0,
		connection: connection,
		window: window,
	};

	unsafe {
		extern "system" {
			fn vkCreateXcbSurfaceKHR(
				instance: VoidPointer,
				pCreateInfo: *const SurfaceCreateInfo,
				pAllocator: VoidPointer,
				surface: *mut u64) -> VkResult;
		}
		check_error(ERROR, vkCreateXcbSurfaceKHR(
			instance, &surface_create_info, NULL, &mut surface
		));
	};

	surface
}

#[cfg(target_os = "windows")]
pub fn create_surface(instance: VoidPointer, native_window: &::AwiWindow) -> u64
{
	let mut surface = 0;
	let surface_create_info = SurfaceCreateInfo {
		s_type: VkStructureType::SurfaceCreateInfo,
		p_next: NULL,
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
		check_error(ERROR, vkCreateWin32SurfaceKHR(
			instance, &surface_create_info, NULL, &mut surface
		));
	};

	surface
}

#[cfg(target_os = "android")]
pub fn create_surface(instance: VoidPointer, native_window: &::AwiWindow) -> u64
{
	let mut surface = 0;
	let surface_create_info = SurfaceCreateInfo {
		s_type: VkStructureType::SurfaceCreateInfo,
		p_next: NULL,
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
		check_error(ERROR, vkCreateAndroidSurfaceKHR(
			instance, &surface_create_info, NULL, &mut surface
		));
	};

	surface
}

pub fn create_surface(instance: VoidPointer, native_window: &::AwiWindow) -> u64
{
	let connection = native_window.get_connection();

	match connection {
		::AwiConnection::Xcb(connection,window) => {
			create_surface_xcb(instance, connection, window)
		}
		::AwiConnection::Wayland => panic!("Wayland Rendering Not Supported Yet"),
		::AwiConnection::DirectFB => panic!("DirectFB Rendering Not Supported Yet"),
		::AwiConnection::Windows => panic!("Windows Rendering Not Supported Yet"),
		::AwiConnection::Android => panic!("Android Rendering Not Supported Yet"),
		::AwiConnection::IOS => panic!("IOS Rendering Not Supported Yet"),
		::AwiConnection::AldaronsOS => panic!("AldaronsOS Rendering Not Supported Yet"),
		::AwiConnection::Arduino => panic!("Arduino Rendering Not Supported Yet"),
		::AwiConnection::Switch => panic!("Switch Rendering Not Supported Yet"),
		::AwiConnection::Web => panic!("Web Assembly Rendering Not Supported Yet"),
		::AwiConnection::NoOS => panic!("No OS Rendering Not Supported Yet"),
	}
}
