/**
 * adi_screen - Aldaron's Device Interface - Screen - "window/mod.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

mod ffi;
pub mod input;

pub use self::ffi::{ NativeWindow, poll_events };
use self::input::Input;

use adi_clock::Timer;
use adi_clock::Pulse;
use renderer;
use renderer::{ Vw, Style, Shape };

/// Window represents a connection to a display that can also recieve input.
pub struct Window {
	pub vw: Vw, // TODO: pub
	pub window: NativeWindow, // TODO: pub
	size: (u32, u32),
	pub sprites: Vec<Shape>, // TODO: pub
	time: (Timer, f32),
	minsize: (u32, (f32, f32)),
	aspect: f32,
	ymultiply: f32,
	shaders: Vec<Style>,
	color: (f32, f32, f32),
	pub input: Vec<Input>, // TODO: pub
}

pub trait WindowFunctions {
	fn resize(&mut self, w: u32, h: u32) -> ();
	fn shader(&self, i: usize) -> Style;
	fn unit_ratio(&self) -> f32;
	fn toggle_fullscreen(&mut self) -> ();
	fn dim(&self) -> (u32, u32);
}

impl WindowFunctions for Window {
	fn resize(&mut self, w: u32, h: u32) -> () {
		self.size = (w, h);
		(self.minsize.1).0 = 2.0 * (self.minsize.0 as f32) / (w as f32);
		(self.minsize.1).1 = 2.0 * (self.minsize.0 as f32) / (h as f32);
		self.aspect = (h as f32) / (w as f32);
		self.ymultiply = 1.0 / self.aspect;
		renderer::resize(self);
	}

	fn shader(&self, i: usize) -> Style {
		self.shaders[i]
	}

	fn unit_ratio(&self) -> f32 {
		self.aspect
	}

	fn toggle_fullscreen(&mut self) -> () {
		self.window.fullscreen();
	}

	fn dim(&self) -> (u32, u32) {
		self.size
	}
}

impl Window {
	/// Create a window for drawing to. name is the name of the window. icon
	/// is the window's icon in ppm format. shaders is a list of custom
	/// shaders. 
	pub fn create(name: &str, icon: &'static [u8], shaders: &[renderer::Shader])
		-> Window
	{
		let native = NativeWindow::create(name, icon);
		let mut input = Vec::new();
		input.push(Input::Resize);
		let mut window = Window {
			vw: renderer::open(name, &native), window: native,
			size: (0,0), sprites: Vec::new(),
			time: (Timer::create(1.0 / 60.0), 0.0),
			minsize: (64, (0.0, 0.0)), aspect: 0.0, ymultiply: 0.0,
			shaders: Vec::new(), input: input,
			color: (0.0, 0.0, 0.0),
		};
		renderer::make_styles(&mut window.vw, shaders, &mut window.shaders);
		window
	}

	/// Set the background color of the window.
	pub fn background(&mut self, r: f32, g: f32, b: f32) -> () {
		self.color = (r, g, b);
	}

	/// Get the minimal x and y dimension for a widget.
	pub fn unit_size(&self) -> (f32, f32) {
		self.minsize.1
	}

	/// Update the window and return the user input.  This should run in a
	/// loop.
	pub fn update(&mut self) -> Input {
		if self.input.is_empty() {
			let color = self.color;

			renderer::draw_clear(self, color.0, color.1, color.2);
			for i in 0..self.sprites.len() {
				renderer::Shape::draw(self, i);
			}
			// TODO: Automatically decrease to 30fps if needed.
			self.time.1 = self.time.0.wait(); // 60 fps
			// Update Screen
			renderer::draw_update(self);
			self.window.update();
			// Now, get new input.
			Input::poll_events(self);
		}
		Input::get(self)
	}

	/// Returns a number between 0-1. This function is used for animations.
	/// It will take rate_spr seconds to go from 0 to 1. 
	pub fn pulse_half_linear(&self, rate_spr: f32) -> f32 {
		self.time.1.pulse_half_linear(rate_spr)
	}

	/// Returns a number between 0-1. This function is used for animations.
	/// It will take rate_spr seconds to go from 0 to 1 and back to 0.
	pub fn pulse_full_linear(&self, rate_spr: f32) -> f32 {
		self.time.1.pulse_full_linear(rate_spr)

	}

	/// Returns a number between 0-1. This function is used for animations.
	/// It will take rate_spr seconds to go from 0 to 1. It uses cosine
	/// underneath to make the animation look smooth, by making the
	/// beginning and end of the animation slower than the middle.
	pub fn pulse_half_smooth(&self, rate_spr: f32) -> f32 {
		self.time.1.pulse_half_smooth(rate_spr)
	}

	/// Returns a number between 0-1. This function is used for animations.
	/// It will take rate_spr seconds to go from 0 to 1 and back to 0. It
	/// uses cosine underneath to make the animation look smooth, by making
	/// the beginning and end of the animation slower than the middle.
	pub fn pulse_full_smooth(&self, rate_spr: f32) -> f32 {
		self.time.1.pulse_full_smooth(rate_spr)
	}
}

impl Drop for Window {
	fn drop(&mut self) -> () {
		renderer::close(&mut self.vw);
		println!("adi_screen: Quit.");
	}
}