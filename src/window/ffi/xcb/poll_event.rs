/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "src/ffi/xcb/poll_event.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use ami::void_pointer::*;
use Input;
use input::keyboard::english;
use Key;
use super::{ convert_mouse_pos, should_resize };

const KEY_DOWN: u8 = 2;
const KEY_UP: u8 = 3;
const BUTTON_DOWN: u8 = 4;
const BUTTON_UP: u8 = 5;
const CURSOR_MOVE: u8 = 6;
const CURSOR_ENTER: u8 = 7;
const CURSOR_LEAVE: u8 = 8;
const GAIN_FOCUS: u8 = 9;
const LOSE_FOCUS: u8 = 10;
const WINDOW_RESIZE: u8 = 22;
const WINDOW_CLOSE: u8 = 33;

const UNKNOWN_CLICK: &'static str = "adi_screen: WARNING: Unknown Mouse Button";

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

struct Event {
	pointer: *mut XcbGenericEvent,
}

impl Event {
	fn create(connection: VoidPointer) -> Event {
		Event {
			pointer: unsafe {
				xcb_poll_for_event(connection)
			}
		}
	}

	fn id(&self) -> u8 {
		unsafe {
			(*self.pointer).response_type & 0x7f
		}
	}

	fn detail(&self) -> u32 {
		unsafe {
			(*self.pointer).detail as u32
		}
	}

	fn key(&self) -> Key {
		english(self.detail())
	}

	fn xy(&self) -> (i16, i16) {
		unsafe {
			((*self.pointer).event_x, (*self.pointer).event_y)
		}
	}

	fn wh(&self) -> (u32, u32) {
		unsafe {
			((*self.pointer).root_x as u32,
				(*self.pointer).root_y as u32)
		}
	}
}

impl Drop for Event {
	fn drop(&mut self) -> () {
		unsafe {
			free(self.pointer)
		};	
	}
}

#[link(name = "xcb")] // TODO: Attempt linking during run-time
extern {
	fn xcb_poll_for_event(c: VoidPointer) -> *mut XcbGenericEvent;
	fn free(event: *mut XcbGenericEvent) -> ();
}

pub fn poll_event(connection: VoidPointer, input: &mut Vec<Input>,
	wh: &mut (u32, u32)) -> bool
{
	let e = Event::create(connection);

	if e.pointer.is_null() {
		return false;
	}

	match e.id() {
		KEY_DOWN => {
			let key = e.key();
			let previous = input.pop().unwrap_or(Input::Redraw);

			if previous == Input::KeyUp(key) {
				match key {
					Key::Insert|Key::Compose => {},
					_ => input.push(Input::KeyRepeat(key)),
				};
			} else {
				if previous != Input::Redraw {
					input.push(previous);
				}
				input.push(Input::KeyDown(key));
			}
		},
		KEY_UP => {
			input.push(Input::KeyUp(e.key()));
		},
		BUTTON_DOWN => {
			let (x, y) = convert_mouse_pos(wh, e.xy());

			match e.detail() {
				1 => input.push(Input::LeftDown(x,y)),
				2 => input.push(Input::MiddleDown(x,y)),
				3 => input.push(Input::RightDown(x,y)),
				4 => input.push(Input::ScrollUp(x,y)),
				5 => input.push(Input::ScrollDown(x,y)),
				6 => input.push(Input::ScrollLeft(x,y)),
				7 => input.push(Input::ScrollRight(x,y)),
				u => println!("{} {}!", UNKNOWN_CLICK, u),
			}
		},
		BUTTON_UP => {
			let (x, y) = convert_mouse_pos(wh, e.xy());

			match e.detail() {
				1 => input.push(Input::LeftUp(x,y)),
				2 => input.push(Input::MiddleUp(x,y)),
				3 => input.push(Input::RightUp(x,y)),
				_ => {}, // ignore scrolling release
			}
		},
		CURSOR_MOVE => {
			let (x, y) = convert_mouse_pos(wh, e.xy());

			input.push(Input::Cursor(x,y));
		},
		CURSOR_ENTER => {
			input.push(Input::EnterWindow)
		},
		CURSOR_LEAVE => {
			input.push(Input::LeaveWindow)
		},
		GAIN_FOCUS => {
			input.push(Input::Resume)
		},
		LOSE_FOCUS => {
			input.push(Input::Pause)
		},
		WINDOW_RESIZE => {
			if should_resize(wh, e.wh()) {
				input.push(Input::Resize)
			}
		},
		WINDOW_CLOSE => {
			input.push(Input::Back)
		},
		_ => {}, // ignore all other messages
	}

	true
}
