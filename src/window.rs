// window.rs -- Aldaron's Device Interface / Screen
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

use adi_clock::Timer;
use adi_clock::Pulse;
use adi_clock::Clock;
use Input;
use afi;
use adi_gpu::Display;
use adi_gpu::DisplayTrait;
use aci_png;
use Texture;

/// Window represents a connection to a display that can also recieve input.
pub struct Window {
	pub(crate) window: Display,
	time: (Timer, f32),
	clock: Clock,
	since_clock: f32,
	since_frame: f32,
	minsize: (u32, (f32, f32)),
	aspect: f32,
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
	pub fn new(name: &str, icon: &afi::Graphic, background: (f32, f32, f32),
		fog: Option<(f32, f32)>) -> Window
	{
		let mut native = Display::new(name, icon).unwrap();
		let button = Texture(native.texture(
			aci_png::decode(include_bytes!("gui/res/button.png"))
				.unwrap()));

		native.color(background);
		native.fog(fog);

		Window {
			window: native, time: (Timer::new(1.0 / 60.0), 0.0),
			clock: Clock::new(), since_clock: 0.0, since_frame: 0.0,
			minsize: (64, (0.0, 0.0)), aspect: 0.0, button: button,
		}
	}

	/// Adjust the location and direction of the camera.
	pub fn camera(&mut self, xyz: (f32,f32,f32), rotate_xyz: (f32,f32,f32)) {
		self.window.camera(xyz, rotate_xyz);
	}

	/// Set the background color of the window.
	pub fn background(&mut self, rgb: (f32, f32, f32)) -> () {
		self.window.color(rgb);
	}

	/// Get the minimal x and y dimension for a widget.
	pub fn unit_size(&self) -> (f32, f32) {
		self.minsize.1
	}

	/// Update the window and return the user input.  This should run in a
	/// loop.  Returns `None` when done looping through input.  After `None`
	/// is returned, the next call will update the screen.
	pub fn update(&mut self) -> Option<Input> {
		// TODO: Automatically decrease to 30fps if needed.
		// self.time.1 = self.time.0.wait(); // 60 fps
		// Update Screen
		let mut input = self.window.update();

		if input == None && self.aspect == 0.0 {
			input = Some(Input::Resize);
		}

		if input == Some(Input::Resize) {
			let wh = self.wh();
			let (w, h) = (wh.0 as f32, wh.1 as f32);

			self.window.resize(wh);

			(self.minsize.1).0 = 2.0 * (self.minsize.0 as f32) / w;
			(self.minsize.1).1 = 2.0 * (self.minsize.0 as f32) / h;
			self.aspect = h / w;
		}

		// Update how much time has passed since previous frame.
		if input.is_none() {
			let old_time = self.since_clock;
			self.since_clock = self.clock.since();
			self.since_frame = self.since_clock - old_time;
		}

		input
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

	/// Get the time passed since the previous window frame.
	pub fn since(&self) -> f32 {
		self.since_frame
	}
}

impl Drop for Window {
	fn drop(&mut self) -> () {
//		renderer::close(&mut self.vw);
		println!("adi_screen: Quit.");
	}
}
