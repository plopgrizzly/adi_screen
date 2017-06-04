/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "window/ffi/xcb/mod.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

mod create_connection;
mod create_property;
mod create_screen;
mod create_window;
mod destroy;
mod poll_event;
mod use_property;
mod window_fullscreen;
mod window_icon;
mod window_map;
mod window_title;
mod window_update;

use Input;
use image::Image;
use ami::void_pointer::*;
use super::{ convert_mouse_pos, should_resize };

const MWW : u16 = super::MWW as u16;
const MWH : u16 = super::MWH as u16;

struct Connection { native: VoidPointer }
impl Connection {
	fn create() -> Connection {
		Connection { native: create_connection::create_connection() }
	}
}
impl Drop for Connection {
	fn drop(&mut self) -> () {
		let connection = self.native;

		destroy::connection(connection);
	}
}

struct Screen { root: u32, visual: u32, black: u32 }
impl Screen {
	fn create(connection: &Connection) -> Screen {
		let c = connection.native;
		let (root, visual, black) = create_screen::create_screen(c);

		Screen {
			root: root, visual: visual, black: black,
		}
	}
}

struct Window { native: u32 }
impl Window {
	fn create(connection: &Connection, screen: Screen) -> Window {
		let c = connection.native;
		let r = screen.root;
		let v = screen.visual;
		let b = screen.black;

		Window {
			native: create_window::create_window(c, r, v, b)
		}
	}
}

struct Property(u32, u32);
impl Property {
	fn create(connection: &Connection, name: &str, fake: bool, name2: &str)
		-> Property
	{
		let props = create_property::create_property(connection.native,
			name, fake, name2, false);

		Property(props.0, props.1)
	}
}

pub struct NativeWindow {
	window: Window,
	connection: Connection,
	fullscreen: Property,
}
impl NativeWindow {
	pub fn create(title: &str, icon: &'static [u8]) -> NativeWindow {
		let connection = Connection::create();
		let screen = Screen::create(&connection);
		let window = Window::create(&connection, screen);
		let fullscreen = Property::create(&connection,
			"_NET_WM_STATE", false, "_NET_WM_STATE_FULLSCREEN");
		let delete = Property::create(&connection,
			"WM_PROTOCOLS", true, "WM_DELETE_WINDOW");

		let window = NativeWindow {
			fullscreen: fullscreen,
			window: window,
			connection: connection,
		};

		window.property(delete); // Catch the window's 'X' button as an event.
		window.title(title); // Set the window title
		window.icon(Image::load(icon)); // Set the window icon
		window.map(); // Show the window

		window
	}

	pub fn fullscreen(&self) {
		window_fullscreen::window_fullscreen(self.window.native,
			self.connection.native, self.fullscreen.0,
			self.fullscreen.1)
	}

	pub fn poll_event(&self, input: &mut Vec<Input>, wh: &mut (u32, u32))
		-> bool
	{
		poll_event::poll_event(self.connection.native, input, wh)
	}

	pub fn update(&self) {
		window_update::window_update(self.connection.native);
	}

	pub fn get_window(&self) -> u32 {
		self.window.native
	}

	pub fn get_connection(&self) -> VoidPointer {
		self.connection.native
	}

	fn map(&self) {
		window_map::window_map(self.window.native,
			self.connection.native);
		window_update::window_update(self.connection.native);
	}

	fn property(&self, property: Property) {
		use_property::use_property(self.window.native,
			self.connection.native, property.0, property.1);
	}

	fn title(&self, title: &str) {
		window_title::window_title(self.window.native,
			self.connection.native, title)
	}

	fn icon(&self, image: Image) {
		window_icon::window_icon(self.window.native,
			self.connection.native, image)
	}
}
