// window.rs -- Aldaron's Device Interface / Screen
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

use adi_clock::{ Timer, Pulse, Clock };
use Input;
use adi_gpu::{ Display, DisplayTrait };
use aci_png;
use Texture;
use afi::Graphic;

/// A builder for `Window`.
pub struct WindowBuilder {
	fog: Option<(f32, f32)>,
	rgb: (f32, f32, f32),
	name: String,
	icon: Option<Graphic>,
}

impl WindowBuilder {
	/// A new `WindowBuilder`.
	pub fn new(name: &str, icon: Option<Graphic>) -> Self {
		WindowBuilder {
			fog: None,
			rgb: (1.0, 1.0, 1.0),
			name: name.to_string(),
			icon,
		}
	}

	/// Set fog distance and fog depth, off by default.
	pub fn fog(mut self, fog_distance: f32, fog_depth: f32) -> Self {
		self.fog = Some((fog_distance, fog_depth));
		self
	}

	/// Set background color, white by default.
	pub fn background(mut self, r: f32, g: f32, b: f32) -> Self {
		self.rgb = (r, g, b);
		self
	}

	/// Finish building the `Window`.
	pub fn finish(self) -> Window {
		let mut native = if let Some(i) = self.icon {
			Display::new(&self.name, i)
		} else {
			let logo = aci_png::decode(
				include_bytes!("res/logo.png"))
				.unwrap();
			Display::new(&self.name, logo)
		}.unwrap();

		let button = Texture(native.texture(
			aci_png::decode(include_bytes!("gui/res/button.png"))
				.unwrap()));

		native.color(self.rgb);
		native.fog(self.fog);

		Window {
			window: native, time: (Timer::new(1.0 / 60.0), 0.0),
			clock: Clock::new(), since_clock: 0.0, since_frame: 0.0,
			minsize: (64, (0.0, 0.0)), aspect: 0.0, button: button,
			seconds: 0.0, fps_counter: 0, fps: 0,
			font: ::gui::Font::new(::gui::DEFAULT_FONT)
		}
	}
}

/// Window represents a connection to a display that can also recieve input.
pub struct Window {
	pub(crate) window: Display,
	time: (Timer, f32),
	clock: Clock,
	since_clock: f32,
	since_frame: f32,
	minsize: (u32, (f32, f32)),
	aspect: f32,
	// Frame Rate Counting
	seconds: f32,
	fps_counter: u16,
	fps: u16,
	// Button Texture
	pub(crate) button: Texture,
	// Default Font
	pub(crate) font: ::gui::Font,
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
	/// Update fog distance.
	pub fn fog(&mut self, fog: Option<(f32, f32)>) {
		self.window.fog(fog);
	}

	/// Set the background color of the window.
	pub fn background(&mut self, rgb: (f32, f32, f32)) -> () {
		self.window.color(rgb);
	}

	/// Adjust the location and direction of the camera.
	pub fn camera(&mut self, xyz: (f32,f32,f32), rotate_xyz: (f32,f32,f32)) {
		self.window.camera(xyz, rotate_xyz);
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

		// Count FPS
		self.fps_counter += 1;

		if self.since_clock >= self.seconds {
			self.fps = self.fps_counter;
			self.fps_counter = 0;
			self.seconds += 1.0;
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

	/// Return the current number of FPS the window is running at.
	pub fn fps(&self) -> (u16, bool) {
		(self.fps, self.fps_counter == 0)
	}
}
