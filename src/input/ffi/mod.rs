/*
 * adi_screen - Aldaron's Device Interface - Screen - "input/ffi/mod.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
 */

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "android"))]
mod unix;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(not(any(target_os = "macos",target_os = "linux",target_os = "windows")))]
mod emulated;
