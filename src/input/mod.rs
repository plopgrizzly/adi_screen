/**
 * adi_screen - Aldaron's Device Interface - Screen - "input/mod.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

mod ffi;

pub mod keyboard;
pub mod joystick;

use Window;
use window::{ WindowFunctions, poll_events };
use input::keyboard::Key;
use input::joystick::Button;

#[derive(PartialEq)]
#[derive(Copy, Clone)]
/// Input represents user input.
pub enum Input {
	Redraw,
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
	JoystickMove(f32, f32),
	JoystickPov(f32, f32),
	JoystickThrottle(f32),
	JoystickButtonDown(Button),
	JoystickButtonUp(Button),
}

fn key(window: &mut Window, input: Input, a: Key) -> Input {
	match a {
		Key::Char(keyboard::FSC) => {
			if input == Input::KeyDown(a) {
				window.toggle_fullscreen();
			}
			Input::get(window)
		}
		Key::Char(keyboard::ESC) => {
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
		match window.input.remove(0) {
			Input::KeyDown(a) => key(window, Input::KeyDown(a), a),
			Input::KeyRepeat(a) =>
				key(window, Input::KeyRepeat(a), a),
			Input::KeyUp(a) => key(window, Input::KeyUp(a), a),
			a => a,
		}
	}

	pub fn poll_events(window: &mut Window) -> () {
		let old_size = window.dim();
		let mut size = window.dim();
		window.input = poll_events(&mut window.window, &mut size);
		if old_size != size {
			window.resize(size.0, size.1);
		}
		window.joystick.update(&mut window.input);
	}
}
