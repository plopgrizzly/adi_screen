/**
 * adi_screen - Aldaron's Device Interface - Screen - "gui/button.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use window::input::Input;
use Window;
use Sprite;
use Style;
use Transform;

pub struct Button {
	sprite: Sprite,
	xmin: f32,
	xmax: f32,
	ymin: f32,
	ymax: f32,
	held: bool,
}

impl Button {
	pub fn create(window: &mut Window, pos: (f32,f32)) -> Button {
		let v = [
			 0.0,  0.0, 0.0, 1.0,	0.0, 0.0, 1.0, 1.0,
			 1.0,  1.0, 0.0, 1.0,	1.0 / 3.0, 1.0, 1.0, 1.0,
			 1.0,  0.0, 0.0, 1.0,	1.0 / 3.0, 0.0, 1.0, 1.0,

			 1.0,  1.0, 0.0, 1.0,	1.0 / 3.0, 1.0, 1.0, 1.0,
			 0.0,  0.0, 0.0, 1.0,	0.0, 0.0, 1.0, 1.0,
			 0.0,  1.0, 0.0, 1.0,	0.0, 1.0, 1.0, 1.0,
		];

		let style = Style::create().opaque(window,
			include_bytes!("res/button.ppm"));
		let spr = Sprite::create(window, &v, style, 1);
		let size = window.unit_size();

		let button = Button {
			sprite: spr,
			xmax: pos.0 + size.0, xmin: pos.0,
			ymax: pos.1 + size.1, ymin: pos.1,
			held: false,
		};

		button
	}

	pub fn update(&mut self, window: &mut Window, input: Input) -> bool {
		match input {
			Input::Cursor(x, y) => {
				self.button_check(window, x, y);
			},
			Input::Resize => {
				let pos = (self.xmin, self.ymin);
				let size = window.unit_size();

				self.xmax = pos.0 + size.0;
				self.xmin = pos.0;
				self.ymax = pos.1 + size.1;
				self.ymin = pos.1;
				self.held = false;
				self.resize(window);
				self.away(window);
			},
			Input::LeftDown(x, y) => {
				self.held = true;
				self.button_check(window, x, y);
			},
			Input::LeftUp(x, y) => {
				if self.held {
					self.held = false;
					return self.button_check(window, x, y);
				}
			},
			Input::LeaveWindow => {
				self.held = false;
				self.away(window);
			},
			_ => {},
		}
		false
	}

	// Private Functions

	fn resize(&mut self, window: &mut Window) {
		Transform::create().auto(window, (self.xmin, self.ymin))
			.apply(window, &self.sprite, 0);
	}

	fn modify(&mut self, window: &mut Window, num: f32) {
		let xmin = num / 3.0;
		let xmax = (num + 1.0) / 3.0;
		let v = [
			 0.0,  0.0, 0.0, 1.0,	xmin, 0.0, 1.0, 1.0,
			 1.0,  1.0, 0.0, 1.0,	xmax, 1.0, 1.0, 1.0,
			 1.0,  0.0, 0.0, 1.0,	xmax, 0.0, 1.0, 1.0,

			 1.0,  1.0, 0.0, 1.0,	xmax, 1.0, 1.0, 1.0,
			 0.0,  0.0, 0.0, 1.0,	xmin, 0.0, 1.0, 1.0,
			 0.0,  1.0, 0.0, 1.0,	xmin, 1.0, 1.0, 1.0,
		];
		self.sprite.vertices(window, &v);
	}

	fn away(&mut self, window: &mut Window) {
		self.modify(window, 0.0);
	}

	fn over(&mut self, window: &mut Window) {
		self.modify(window, 1.0);
	}

	fn down(&mut self, window: &mut Window) {
		self.modify(window, 2.0);
	}


	fn button_check(&mut self, window: &mut Window, x: f32, y: f32) -> bool{
		let (xmax, xmin, ymax, ymin, held) = {
			(self.xmax, self.xmin, self.ymax, self.ymin, self.held)
		};
		if x >= xmin && x <= xmax && y >= ymin && y <= ymax {
			if held {
				self.down(window);
			}else{
				self.over(window);
			}
			true
		}else{
			self.held = false;
			self.away(window);
			false
		}
	}
}
