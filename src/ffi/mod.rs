/**
 * adi_screen - Aldaron's Device Interface - Screen - "ffi/mod.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

// Platform Depedant
#[cfg(any(target_os = "linux", target_os = "macos"))]
#[link(name = "xcb")]
mod xcb;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "android")]
mod android;
// TODO: Implement WAYLAND and MIR
//#[link(name = "wayland-client")]
//mod wayland;

// native window manager functions
#[cfg(any(target_os = "linux", target_os = "macos"))]
use self::xcb as native;
#[cfg(target_os = "windows")]
use self::windows as native;
#[cfg(target_os = "android")]
use self::android as native;

// Platform Independant

#[link(name = "vulkan-1")]
pub mod vulkan;

pub mod string;
mod shared;

pub use self::native::{ NativeWindow, convert_event, native_window, cleanup, toggle_fullscreen };
