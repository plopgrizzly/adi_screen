/**
 * adi_screen - Aldaron's Device Interface - Screen - "input/mod.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use std::fmt;

use Window;
use window::WindowFunctions;
use running;

pub mod keyboard;

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Key {
	Char(char),
	Backspace,
	Delete,
	Ctrl(bool),
	Shift(bool),
	Alt(bool),
	F(u8),
	CapsLock,
	NumLock,
	Home,
	End,
	PageUp,
	PageDown,
	Up,
	Down,
	Left,
	Right,
	Insert,
	Escape,
}

impl fmt::Display for Key {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Key::Char(a) => match a {
				' ' => write!(f, "space"),
				'\t' => write!(f, "tab"),
				'\n' => write!(f, "newline"),
				b => write!(f, "{}", b),
			},
			Key::Backspace => write!(f, "Backspace"),
			Key::Delete => write!(f, "Delete"),
			Key::Ctrl(false) => write!(f, "Left Ctrl (false)"),
			Key::Ctrl(true) => write!(f, "Right Ctrl (true)"),
			Key::Shift(false) => write!(f, "Left Shift (false)"),
			Key::Shift(true) => write!(f, "Right Shift (true)"),
			Key::Alt(false) => write!(f, "Left Alt (false)"),
			Key::Alt(true) => write!(f, "Right Alt (true)"),
			Key::F(a) => write!(f, "F{}", a),
			Key::CapsLock => write!(f, "CapsLock"),
			Key::NumLock => write!(f, "NumLock"),
			Key::Home => write!(f, "Home"),
			Key::End => write!(f, "End"),
			Key::PageUp => write!(f, "PageUp"),
			Key::PageDown => write!(f, "PageDown"),
			Key::Up => write!(f, "Up"),
			Key::Down => write!(f, "Down"),
			Key::Left => write!(f, "Left"),
			Key::Right => write!(f, "Right"),
			Key::Insert => write!(f, "Insert"),
			Key::Escape => write!(f, "Escape"),
		}
	}
}

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Input {
	Draw,
	Resize,
	Back,
	Resume,
	Pause,
	EnterWindow,
	LeaveWindow,
	KeyDown(Key),
	KeyRepeat(Key),
	KeyUp(Key),
	Cursor(f32,f32),
	LeftDown(f32,f32),
	MiddleDown(f32,f32),
	RightDown(f32,f32),
	LeftUp(f32,f32),
	MiddleUp(f32,f32),
	RightUp(f32,f32),
	ScrollUp(f32,f32),
	ScrollDown(f32,f32),
	ScrollLeft(f32,f32),
	ScrollRight(f32,f32),
}

fn key(window: &mut Window, input: Input, a: Key) -> Input {
	match a {
		Key::F(11) => {
			if input == Input::KeyDown(a) {
				window.toggle_fullscreen();
			}
			Input::get(window)
		}
		Key::Escape => {
			if input == Input::KeyDown(a) {
				Input::Back
			} else {
				Input::get(window)
			}
		}
		Key::Char('\x00') => {
			Input::get(window)
		}
		_ => input
	}
}

impl Input {
	pub fn get(window: &mut Window) -> Input {
		match running(window) {
			Input::KeyDown(a) => key(window, Input::KeyDown(a), a),
			Input::KeyRepeat(a) =>
				key(window, Input::KeyRepeat(a), a),
			Input::KeyUp(a) => key(window, Input::KeyUp(a), a),
			a => a,
		}
	}
}
