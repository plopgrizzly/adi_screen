/**
 * adi_screen - Aldaron's Device Interface - Screen - "ffi/string.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

#[cfg(not(target_os = "windows"))]
use std::ffi::CString;

// Windows uses UTF-16
#[cfg(target_os = "windows")]
pub fn native(what: &str) -> Vec<u8> {
	let mut rtn : Vec<u8> = Vec::new();
	for c in what.encode_utf16() {
		rtn.push((c % 255) as u8);
		rtn.push((c / 255) as u8);
	}
	rtn.push(0);
	rtn.push(0);
	rtn
}

// Everything else uses UTF-8
#[cfg(not(target_os = "windows"))]
pub fn native(what: &str) -> Vec<u8> {
	CString::new(what).unwrap().into_bytes()
}