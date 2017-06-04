/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/xcb/window_fullscreen.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use ami::void_pointer::*;

#[repr(C)]
struct XcbClientMessageEvent {
	response_type: u8,
	format: u8,
	sequence: u16,
	window: u32,
	stype: u32,
	data32: [u32; 5],
}

extern {
	fn xcb_send_event(c: VoidPointer, p: u8, dest: u32,
		event_mask: u32, event: *const XcbClientMessageEvent) -> ();
}

pub fn window_fullscreen(window: u32, connection: VoidPointer, a: u32, b: u32) {
	let event = XcbClientMessageEvent {
		response_type: 33, // Client Message
		format: 32,
		sequence: 0,
		window: window,
		stype: a,
		data32: [2, b, 0, 0, 0],
	};
	unsafe {
		xcb_send_event(connection, 1, window, 1048576 | 524288, &event);
	}
}
