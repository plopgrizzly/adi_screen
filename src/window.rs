// Aldaron's Device Interface / Screen
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/window.rs

use adi_clock::Timer;
use adi_clock::Pulse;
use Input;
use afi;
use adi_gpu;
use adi_gpu::Display;
use aci_png;
use Texture;

/// Window represents a connection to a display that can also recieve input.
pub struct Window {
	pub(crate) window: Display,
	time: (Timer, f32),
	minsize: (u32, (f32, f32)),
	aspect: f32,
	#[allow(unused)] // TODO: Unused
	pub(crate) joystick: ::Joystick,
	pub(crate) button: Texture,
}

pub trait WindowFunctions {
	fn unit_ratio(&self) -> f32;
	fn toggle_fullscreen(&mut self) -> ();
	fn wh(&self) -> (u32, u32);
}

impl WindowFunctions for Window {
	fn unit_ratio(&self) -> f32 {
		self.aspect
	}

	fn toggle_fullscreen(&mut self) -> () {
//		self.window.fullscreen();
	}

	fn wh(&self) -> (u32, u32) {
		self.window.wh()
	}
}

impl Window {
	/// Create a window for drawing to. name is the name of the window. icon
	/// is the window's icon in ppm format. shaders is a list of custom
	/// shaders. `fog` is a tuple: (distance, depth).
	pub fn new(name: &str, icon: afi::Graphic, background: (f32, f32, f32),
		fog: (f32, f32)) -> Window
	{
		let mut native = Display::new(name, icon, background, fog);
		let button = Texture(adi_gpu::Texture::new(&mut native,
			aci_png::decode(include_bytes!("gui/res/button.png"))
				.unwrap()));
		Window {
			window: native, time: (Timer::new(1.0 / 60.0), 0.0),
			minsize: (64, (0.0, 0.0)), aspect: 0.0,
			joystick: ::Joystick::new(), button: button,
		}
	}

	/// Adjust the location and direction of the camera.
	pub fn camera(&mut self, xyz: (f32,f32,f32), rotate_xyz: (f32,f32,f32)) {
		self.window.camera(xyz, rotate_xyz);
	}

	/// Set the background color of the window.
	pub fn background(&mut self, rgb: (f32, f32, f32)) -> () {
		self.window.bg_color(rgb);
	}

	/// Get the minimal x and y dimension for a widget.
	pub fn unit_size(&self) -> (f32, f32) {
		self.minsize.1
	}

	/// Get input if there is, otherwise return `None`.
	pub fn input(&mut self) -> Option<Input> {
		let mut input = self.window.input();

		if input == None && self.aspect == 0.0 {
			input = Some(Input::Resize);
		}

		if input == Some(Input::Resize) {
			let (w, h) = self.wh();
			let (w, h) = (w as f32, h as f32);

			(self.minsize.1).0 = 2.0 * (self.minsize.0 as f32) / w;
			(self.minsize.1).1 = 2.0 * (self.minsize.0 as f32) / h;
			self.aspect = h / w;
		}

		input
	}

	/// Update the window and return the user input.  This should run in a
	/// loop.
	pub fn update(&mut self) -> () {
		// TODO: Automatically decrease to 30fps if needed.
		self.time.1 = self.time.0.wait(); // 60 fps
		// Update Screen
		self.window.update();
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
//		renderer::close(&mut self.vw);
		println!("adi_screen: Quit.");
	}
}
