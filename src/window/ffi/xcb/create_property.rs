/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/xcb/create_property.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use ami::void_pointer::*;
use std::ffi::CString;

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
	fn free(p: *mut XcbInternAtomReply);
}

pub fn create_property(connection: VoidPointer, name: &str, fake: bool,
	name2: &str, fake2: bool) -> (u32, u32)
{
	let fake = if fake { 1 } else { 0 };
	let fake2 = if fake2 { 1 } else { 0 };
	let name_len = name.len() as u16;
	let name2_len = name2.len() as u16;
	let name = CString::new(name).unwrap();
	let name2 = CString::new(name2).unwrap();

	let cookie = unsafe {
		xcb_intern_atom(connection, fake, name_len, name.as_ptr())
	};

	let cookie2 = unsafe {
		xcb_intern_atom(connection, fake2, name2_len, name2.as_ptr())
	};

	let reply = unsafe {
		xcb_intern_atom_reply(connection, cookie, NULL)
	};

	let reply2 = unsafe {
		xcb_intern_atom_reply(connection, cookie2, NULL)
	};

	let atom = unsafe { (*reply).atom };
	let atom2 = unsafe { (*reply2).atom };

	unsafe {
		free(reply);
		free(reply2);
	}

	(atom, atom2)
}
