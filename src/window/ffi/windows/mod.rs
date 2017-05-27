/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "window/ffi/windows/mod.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

mod class_create;
mod connection_create;
mod string; // for UTF-16 conversions
mod window_create;
mod window_fullscreen;
mod window_poll_event;

use Input;
use image::Image;
use super::{ LazyPointer, convert_mouse_pos, should_resize };

const MWW : isize = super::MWW as isize;
const MWH : isize = super::MWH as isize;

pub struct Connection { pub native: LazyPointer }
impl Connection {
	fn create() -> Connection {
		Connection { native: connection_create::connection_create() }
	}
}
pub struct Class { name: [u8; 80] }
impl Class {
	fn create(connection: &Connection, name: &str, image: Image, wnd_proc:
		extern "C" fn(a: LazyPointer, b: u32, c: LazyPointer,
			d: LazyPointer) -> isize)
		-> Class
	{
		Class {
			name: class_create::class_create(connection.native,
				name, image, wnd_proc)
		}
	}
}
pub struct Window { pub native: LazyPointer }
impl Window {
	fn create(connection: &Connection, size: (isize, isize), class: Class) -> Window {
		let c = connection.native;
		let name = class.name;

		Window { native: window_create::window_create(c, size, name) }
	}
}

pub struct NativeWindow {
	pub window: Window,
	pub connection: Connection,
	miw: bool, // Mouse In Window
	restore_size: (i32, i32, i32, i32),
	fullscreen: bool,
	restore_style: usize,
}
impl NativeWindow {
	pub fn create(title: &str, icon: &'static [u8]) -> NativeWindow {
		let connection = Connection::create();
		let class = Class::create(&connection, title, Image::load(icon),
			window_poll_event::wnd_proc);
		let window = Window::create(&connection, (MWW, MWH), class);

		NativeWindow { connection: connection, window: window, miw: true,
			restore_size: (0, 0, 0, 0),
			fullscreen: false, restore_style: 0,
		}
	}

	pub fn fullscreen(&mut self) {
		window_fullscreen::window_fullscreen(self.window.native,
			&mut self.fullscreen, &mut self.restore_size,
			&mut self.restore_style);
	}

	pub fn poll_event(&mut self, input: &mut Vec<Input>, wh: &mut (u32, u32))
		-> bool
	{
		let miw = &mut self.miw;
		let window = self.window.native;

		window_poll_event::window_poll_event(window, input, miw, wh)
	}

	pub fn update(&self) {
	}
}