/**
 * adi_screen - Aldaron's Device Interface - Screen - "lib.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

pub const VERSION : &'static str = "adi_screen 0.1.0";

pub extern crate adi_clock;
use adi_clock::Timer;
use adi_clock::Pulse;

pub mod input;
pub mod transforms;

pub mod gui;

mod ffi; // Native window module
pub use self::ffi::{ running };

mod image;

mod sprite;
pub use self::sprite::Sprite;

#[link(name = "vulkan-1")]
mod vw;
pub use self::vw::{ Texture, Style, Shader };

pub struct Screen {
	pub vw: vw::Vw,
	pub rqexit: bool,
	window: ffi::NativeWindow,
	pub size: (u32, u32),
//	back_fn: CallbackFn,
	sprites: Vec<sprite::SpriteData>,
	time: (Timer, f32),
	minsize: (u32, (f32, f32)),
	aspect: f32,
	ymultiply: f32,
}

impl Screen {
	pub fn new(name: &str, icon: &'static [u8], shaders: &[vw::Shader])
		-> (Screen, Vec<Style>)
	{
		let native = ffi::native_window(name, icon);
		let vw = vw::open(name, &native);
		let size = (640, 360);
		let aspect = (size.1 as f32) / (size.0 as f32);
		let mut screen = Screen { vw: vw, window: native, size: size,
			/*back_fn: default_back_fn,*/ sprites: Vec::new(),
			time: (Timer::new(1.0 / 60.0), 0.0),
			minsize: (64, (2.0 * 64.0 / 640.0, 2.0 * 64.0 / 360.0)),
			aspect: aspect, ymultiply: 1.0 / aspect, rqexit: false };
		let pipelines = vw::make_styles(&mut screen, shaders);
		println!("new window is done!");
		(screen, pipelines)
	}

	pub fn scalex(&self) -> f32 {
		(self.minsize.1).0
	}

	pub fn scaley(&self) -> f32 {
		(self.minsize.1).1 // * self.ymultiply
	}

	pub fn render(&mut self, color: (f32, f32, f32)) -> () {
		self.clear(color.0, color.1, color.2);
		for i in 0..self.sprites.len() {
			if self.sprites[i].enabled {
				self.sprites[i].shape.draw();
			}
		}
		// TODO: Automatically decrease to 30fps if needed.
		self.time.1 = self.time.0.wait(); // 60 fps
		// Update Screen
		vw::draw_update(self);
	}

	pub fn cleanup(&mut self) -> () {
		vw::close(&mut self.vw);
		ffi::cleanup(&mut self.window);
	}

	fn clear(&mut self, r:f32, g:f32, b:f32) -> () {
		vw::draw_clear(self, r, g, b);
	}

	pub fn resize(&mut self, w: u32, h: u32) {
		self.size = (w, h);
		(self.minsize.1).0 = 2.0 * (self.minsize.0 as f32) / (w as f32);
		(self.minsize.1).1 = 2.0 * (self.minsize.0 as f32) / (h as f32);
		self.aspect = (h as f32) / (w as f32);
		self.ymultiply = 1.0 / self.aspect;
		vw::resize(self);
	}

	pub fn toggle_fullscreen(&mut self) {
		ffi::toggle_fullscreen(&mut self.window);
	}

	pub fn stop(&mut self) {
		self.rqexit = true;
	}

	pub fn keep(&mut self) {
		self.rqexit = false;
	}
	
	pub fn half_linear_pulse(&self, rate_spr: f32) -> f32 {
		self.time.1.half_linear_pulse(rate_spr)
	}
	
	pub fn full_linear_pulse(&self, rate_spr: f32) -> f32 {
		self.time.1.full_linear_pulse(rate_spr)

	}
	
	pub fn full_smooth_pulse(&self, rate_spr: f32) -> f32 {
		self.time.1.full_smooth_pulse(rate_spr)

	}
	
	pub fn half_smooth_pulse(&self, rate_spr: f32) -> f32 {
		self.time.1.half_smooth_pulse(rate_spr)
	}
}
