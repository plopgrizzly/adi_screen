/**
 * adi_screen - Aldaron's Device Interface - Screen - "build.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

extern crate gcc;

#[cfg(target_os = "linux")]
fn link() {
	// TODO: Implement the Vulkan Loader in Rust.
	println!("cargo:rustc-link-lib=vulkan");
}

#[cfg(target_os = "windows")]
fn link() {
	// TODO: Link Statically
	println!("cargo:rustc-link-lib=vulkan-1");
//	println!("cargo:rustc-link-lib=static=vulkan-1");
}

fn main() {
	gcc::Config::new().file("native/vw.c").flag("-Wall").flag("-Werror").compile("libaldaronvw.a");
//	println!("cargo:rustc-link-search=native=src/third-party/");
//	println!("cargo:rustc-link-args=-Wl,--subsystem,windows");
	link();
}
