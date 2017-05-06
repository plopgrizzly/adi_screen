/**
 * adi_screen - Aldaron's Device Interface - Screen - "gui/button.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use input::Input;
use Screen;
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
	pub fn create(screen: &mut Screen, pos: (f32,f32)) -> Button {
		let v = [
			 0.0,  0.0, 0.0, 1.0,	0.0, 0.0, 1.0, 1.0,
			 1.0,  1.0, 0.0, 1.0,	1.0 / 3.0, 1.0, 1.0, 1.0,
			 1.0,  0.0, 0.0, 1.0,	1.0 / 3.0, 0.0, 1.0, 1.0,

			 1.0,  1.0, 0.0, 1.0,	1.0 / 3.0, 1.0, 1.0, 1.0,
			 0.0,  0.0, 0.0, 1.0,	0.0, 0.0, 1.0, 1.0,
			 0.0,  1.0, 0.0, 1.0,	0.0, 1.0, 1.0, 1.0,
		];

		let style = Style::create().opaque(screen,
			include_bytes!("../res/button.ppm"));
		let spr = Sprite::create(screen, &v, style, 1);
//		let transform = Transform::create().auto(screen, pos);
//		spr.copy(screen, &transform);

		let button = Button {
			sprite: spr,
			xmax: pos.0 + screen.unit_width(),
			xmin: pos.0,
			ymax: pos.1 + screen.unit_height(),
			ymin: pos.1,
			held: false,
		};

		button
	}

	pub fn get(&mut self, screen: &mut Screen, input: Input) -> bool {
		match input {
			Input::Cursor(x, y) => {
				self.button_check(screen, x, y);
			},
			Input::Resize => {
				let pos = (self.xmin, self.ymin);
				self.xmax = pos.0 + screen.unit_width();
				self.xmin = pos.0;
				self.ymax = pos.1 + screen.unit_height();
				self.ymin = pos.1;
				self.held = false;
				self.resize(screen);
				self.away(screen);
			},
			Input::LeftDown(x, y) => {
				self.held = true;
				self.button_check(screen, x, y);
			},
			Input::LeftUp(x, y) => {
				if self.held {
					self.held = false;
					return self.button_check(screen, x, y);
				}
			},
			Input::LeaveWindow => {
				self.held = false;
				self.away(screen);
			},
			_ => {},
		}
		false
	}

	// Private Functions

	fn resize(&mut self, screen: &mut Screen) {
		Transform::create().auto(screen, (self.xmin, self.ymin))
			.on(screen, &self.sprite, 0);
	}

	fn modify(&mut self, screen: &mut Screen, num: f32) {
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
		self.sprite.vertices(screen, &v);
	}

	fn away(&mut self, screen: &mut Screen) {
		self.modify(screen, 0.0);
	}

	fn over(&mut self, screen: &mut Screen) {
		self.modify(screen, 1.0);
	}

	fn down(&mut self, screen: &mut Screen) {
		self.modify(screen, 2.0);
	}


	fn button_check(&mut self, screen: &mut Screen, x: f32, y: f32) -> bool{
		let (xmax, xmin, ymax, ymin, held) = {
			(self.xmax, self.xmin, self.ymax, self.ymin, self.held)
		};
		if x >= xmin && x <= xmax && y >= ymin && y <= ymax {
			if held {
				self.down(screen);
			}else{
				self.over(screen);
			}
			true
		}else{
			self.held = false;
			self.away(screen);
			false
		}
	}
}
