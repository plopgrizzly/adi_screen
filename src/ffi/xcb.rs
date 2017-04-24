/**
 * adi_screen - Aldaron's Device Interface - Screen - "ffi/xcb.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use std;
use std::ffi::CString;
use Screen;
use input::Input;
use image::Image;
use input::keyboard::{ english };
use input::Key;
use std::vec::Vec;
use super::shared;

type XcbVoid = u8;
pub type XcbConnection = XcbVoid;

#[repr(C)]
pub struct XcbScreen {
	root: u32,
	default_colormap: u32,
	white_pixel: u32,
	black_pixel: u32,
	current_input_masks: u32,
	width_in_pixels: u16,
	height_in_pixels: u16,
	width_in_millimeters: u16,
	height_in_millimeters: u16,
	min_installed_maps: u16,
	max_installed_maps: u16,
	root_visual: u32,
	backing_stores: u8,
	save_unders: u8,
	root_depth: u8,
	allowed_depths_len: u8,
}

#[repr(C)]
struct XcbScreenIterator {
	data: *mut XcbScreen,
	rem: i32,
	index: i32,
}

#[repr(C)]
struct XcbInternAtomReply {
	response_type: u8,
	pad0: u8,
	sequence: u16,
	length: u32,
	atom: u32,
}

#[repr(C)]
struct XcbGenericEvent {
	response_type: u8,
	detail: u8,
	sequence: u16,
	timestamp: u32,
	root: u32,
	event: u32,
	child: u32,
	root_x: i16,
	root_y: i16,
	event_x: i16,
	event_y: i16,
	state: u16,
	same_screen: u8,
	pad0: u8,
}

#[repr(C)]
struct ValueList {
	a: u32,
	b: u32,
}

#[repr(C)]
struct XcbClientMessageEvent {
	response_type: u8,
	format: u8,
	sequence: u16,
	window: u32,
	stype: u32,
	data32: [u32; 5],
}

const XCB_KEY_PRESS: u8 = 2;
const XCB_KEY_RELEASE: u8  = 3;
const XCB_BUTTON_PRESS: u8  = 4;
const XCB_BUTTON_RELEASE: u8  = 5;
const XCB_MOTION_NOTIFY: u8  = 6;
const XCB_ENTER_NOTIFY: u8  = 7;
const XCB_LEAVE_NOTIFY: u8  = 8;
const XCB_FOCUS_IN: u8  = 9;
const XCB_FOCUS_OUT: u8  = 10;
const XCB_EXPOSE: u8 = 12;
const XCB_CONFIGURE_NOTIFY: u8 = 22;
const XCB_CLIENT_MESSAGE: u8  = 33;

const MWW : u16 = shared::MWW as u16;
const MWH : u16 = shared::MWH as u16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NativeWindow {
	pub connection: *mut XcbConnection,
	pub screen: *mut XcbScreen,
	pub window: u32,
}

#[link(name = "xcb")]
extern {
	fn xcb_connect(displayname: *const i8, screenp: *const i32)
		-> *mut XcbConnection;
	fn xcb_get_setup(c: *mut XcbConnection) -> *mut XcbVoid;
	fn xcb_setup_roots_iterator(setup: *mut XcbVoid) -> XcbScreenIterator;
	fn xcb_generate_id(c: *mut XcbConnection) -> u32;
	fn xcb_create_window(c: *mut XcbConnection, depth: u8, wid: u32,
		parent: u32, x: i16, y: i16, width: u16, height: u16,
		border_width: u16, _class: u16, visual: u32, value_mask: u32,
		value_list: *mut ValueList) -> u32;
	fn xcb_intern_atom(c: *mut XcbConnection, only_if_exists: u8,
		name_len: u16, name: *const i8) -> u32;
	fn xcb_intern_atom_reply(c: *mut XcbConnection, cookie: u32,
		e: *mut XcbVoid) -> *mut XcbInternAtomReply;
	fn xcb_change_property(c: *mut XcbConnection, mode: u8, window: u32,
		property: u32, t: u32, format: u8, data_len: u32,
		data: *const u32) -> u32;
	fn xcb_send_event(c: *mut XcbConnection, p: u8, dest: u32,
		event_mask: u32, event: *const XcbClientMessageEvent) -> ();
	fn xcb_map_window(c: *mut XcbConnection, window: u32) -> u32;
	fn xcb_flush(c: *mut XcbConnection) -> i32;
	fn xcb_poll_for_event(c: *mut XcbConnection) -> *mut XcbGenericEvent;
	fn xcb_disconnect(c: *mut XcbConnection) -> ();
	//
	fn free(p: *mut XcbVoid);
}

pub fn toggle_fullscreen(window: &NativeWindow) {
	let cookie = unsafe { xcb_intern_atom(window.connection, 0, 13,
		CString::new("_NET_WM_STATE").unwrap().as_ptr()) };
	let cookie2 = unsafe { xcb_intern_atom(window.connection, 0, 24,
		CString::new("_NET_WM_STATE_FULLSCREEN").unwrap().as_ptr()) };
	let reply = unsafe {
		xcb_intern_atom_reply(window.connection, cookie,
			std::ptr::null_mut())
	};
	let reply2 = unsafe {
		xcb_intern_atom_reply(window.connection, cookie2,
			std::ptr::null_mut())
	};
	let event = XcbClientMessageEvent {
		response_type: 33, // Client Message
		format: 32,
		sequence: 0,
		window: window.window,
		stype: unsafe { (*reply).atom },
		data32: [2, unsafe { (*reply2).atom }, 0, 0, 0],
	};
	unsafe {
		xcb_send_event(window.connection, 1, window.window,
			1048576 | 524288, &event);
		free(reply as *mut _);
		free(reply2 as *mut _);
	}
	println!("FULLSCREN!");
}

pub fn native_window(title: &str, icon:&'static [u8]) -> NativeWindow {
	let icon = Image::load(icon);
	let connection : *mut _ = unsafe {
		xcb_connect(std::ptr::null(), std::ptr::null())
	};
	if connection.is_null() {
		panic!("Couldn't connect to X Server.");
	}
	let setup = unsafe { xcb_get_setup(connection) };
	let screen = unsafe { xcb_setup_roots_iterator(setup).data };
	let window = unsafe { xcb_generate_id(connection) };

	let black = unsafe { (*screen).black_pixel };
	let event_mask : u32 = 1|2|4|8|16|32|64|32768|131072|2097152;
	let mut value_list = ValueList {
		a: black, b: event_mask
	};

	unsafe {
		xcb_create_window(connection, 0, window,
			(*screen).root, 0, 0, MWW, MWH, 0, 1,
			(*screen).root_visual, 2|2048,
			&mut value_list);
	}

	let cookie = unsafe { xcb_intern_atom(connection, 1, 12,
		CString::new("WM_PROTOCOLS").unwrap().as_ptr()) };
	let reply = unsafe {
		xcb_intern_atom_reply(connection, cookie, std::ptr::null_mut())
	};
	let cookie2 = unsafe { xcb_intern_atom(connection, 0, 16,
		CString::new("WM_DELETE_WINDOW").unwrap().as_ptr()) };
	let reply2 = unsafe {
		xcb_intern_atom_reply(connection, cookie2, std::ptr::null_mut())
	};
	unsafe {
		xcb_change_property(connection, 0, window,
			(*reply).atom, 4, 32, 1, & ((*reply2).atom));
		xcb_change_property(connection, 0, window,
			39, 31, 8, title.len() as u32,
			CString::new(title).unwrap().as_ptr() as *const _);
		xcb_change_property(connection, 0, window,
			37, 31, 8, title.len() as u32,
			CString::new(title).unwrap().as_ptr() as *const _);
		free(reply as *mut _);
		free(reply2 as *mut _);
	}
	let cookie = unsafe { xcb_intern_atom(connection, 1, 12,
		CString::new("_NET_WM_ICON").unwrap().as_ptr()) };
	let reply = unsafe {
		xcb_intern_atom_reply(connection, cookie, std::ptr::null_mut())
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
		xcb_map_window(connection, window);
	}
	let native_window = NativeWindow {
		connection: connection, screen: screen, window: window
	};
	unsafe {
		xcb_flush(connection);
	}
	native_window
}

fn poll_event(screen: &Screen) -> (u8, u8, (i16, i16)) {
	let e = unsafe { xcb_poll_for_event(screen.window.connection) };
	if e == std::ptr::null_mut() {
		return (0, 0, (0, 0));
	}
	let xcb_event = unsafe { (*e).response_type & 0x7f };
	let dim = match xcb_event {
		XCB_CONFIGURE_NOTIFY => unsafe { ((*e).root_x, (*e).root_y) },
		XCB_MOTION_NOTIFY | XCB_BUTTON_RELEASE | XCB_BUTTON_PRESS =>
			unsafe { ((*e).event_x, (*e).event_y) },
		_ => (0, 0)
	};
	let details = unsafe { (xcb_event, (*e).detail, dim) };
	unsafe { free(e as *mut _) };
	details
}

fn convert_event(screen: &Screen, event_out: &mut Input) -> bool {
	let (xcb_event, detail, dim) = poll_event(screen);
	*event_out = match xcb_event {
		0 => Input::None,
		XCB_KEY_PRESS => Input::KeyDown(english(detail as u32)),
		XCB_KEY_RELEASE => {
			let detail = english(detail as u32);
			if detail == Key::F(11) {
				return true // ignore.
			}else{
				let e = unsafe {
					xcb_poll_for_event(screen.window.connection)
				};
				if e == std::ptr::null_mut() {
					Input::KeyUp(detail)
				}else{
					unsafe { free(e as *mut _) };
					Input::KeyRepeat(detail)
				}
			}
		},
		XCB_BUTTON_PRESS => {
			let (x, y) = shared::convert_mouse_pos(&screen, dim);
			match detail {
				1 => Input::LeftDown(x,y),
				2 => Input::MiddleDown(x,y),
				3 => Input::RightDown(x,y),
				4 => Input::ScrollUp(x,y),
				5 => Input::ScrollDown(x,y),
				6 => Input::ScrollLeft(x,y),
				7 => Input::ScrollRight(x,y),
				u => panic!("Unknown Mouse Button {}!", u),
			}
		},
		XCB_BUTTON_RELEASE => {
			let (x, y) = shared::convert_mouse_pos(&screen, dim);
			match detail {
				1 => Input::LeftUp(x,y),
				2 => Input::MiddleUp(x,y),
				3 => Input::RightUp(x,y),
				4 ... 7 => return true, // ignore
				u => panic!("Unknown Mouse Button {}!", u),
			}
		},
		XCB_MOTION_NOTIFY => {
			let (x, y) = shared::convert_mouse_pos(&screen, dim);
			Input::Cursor(x,y)
		},
		XCB_ENTER_NOTIFY => Input::EnterWindow,
		XCB_LEAVE_NOTIFY => Input::LeaveWindow,
		XCB_FOCUS_IN => Input::Resume,
		XCB_FOCUS_OUT => Input::Pause,
		XCB_CONFIGURE_NOTIFY =>
			Input::Resize(dim.0 as u32, dim.1 as u32),
		XCB_CLIENT_MESSAGE => Input::Back,
		XCB_EXPOSE => return true, // ignore.
		35 => { return true }, // TODO: why so many?  Temporary ignore
		x => {
			println!("Unknown event: {}", x);
			return true; // ignore
		},
	};
	false
}

pub fn running(screen: &Screen) -> Input {
	let mut converted = Input::None;

	while convert_event(screen, &mut converted) {}
	converted
}

pub fn cleanup(window: &mut NativeWindow) {
	unsafe { xcb_disconnect(window.connection); }
}
