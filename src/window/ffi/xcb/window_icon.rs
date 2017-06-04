/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/xcb/window_icon.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use ami::void_pointer::*;
use std::ffi::CString;

use image::Image;

#[repr(C)]
struct XcbInternAtomReply {
	response_type: u8,
	pad0: u8,
	sequence: u16,
	length: u32,
	atom: u32,
}

extern {
	fn xcb_intern_atom(c: VoidPointer, only_if_exists: u8,
		name_len: u16, name: *const i8) -> u32;
	fn xcb_intern_atom_reply(c: VoidPointer, cookie: u32,
		e: VoidPointer) -> *mut XcbInternAtomReply;
	fn xcb_change_property(c: VoidPointer, mode: u8, window: u32,
		property: u32, t: u32, format: u8, data_len: u32,
		data: *const u32) -> u32;
}

pub fn window_icon(window: u32, connection: VoidPointer, icon: Image) {
	let cookie = unsafe {
		xcb_intern_atom(connection, 1, 12,
			CString::new("_NET_WM_ICON").unwrap().as_ptr())
	};
	let reply = unsafe {
		xcb_intern_atom_reply(connection, cookie, NULL)
	};
	unsafe {
		let width = icon.size.0 as usize;
		let iconsize = 2 + ( icon.size.0 * icon.size.1 );
		let mut vector : Vec<u32> = Vec::new();
		vector.push(icon.size.0);
		vector.push(icon.size.1);
		for i in 0usize..icon.size.0 as usize{
			for j in 0usize..icon.size.1 as usize {
				let mut pixel = 0xff000000;
				pixel |= 0x00010000 * icon.pixels[
					0 + 3 * (j + (width * i))] as u32;
				pixel |= 0x00000100 * icon.pixels[
					1 + 3 * (j + (width * i))] as u32;
				pixel |= 0x00000001 * icon.pixels[
					2 + 3 * (j + (width * i))] as u32;
				vector.push(pixel);
			}
		}
		xcb_change_property(connection, 0, window,
			(*reply).atom, 6, 32, iconsize, &vector[0]);
	}
}
