// "adi_screen" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

/*use Input;
use Window;
use Sprite;
use Transform;
use ModelBuilder;

use ami::Mat4;*/

/*
/// A GUI Button Sprite.
pub struct Button {
	sprite: Sprite,
	xmin: f32,
	xmax: f32,
	ymin: f32,
	ymax: f32,
	held: bool,
}

impl Button {
	/// Add a new button to the screen.
	pub fn new(window: &mut Window, pos: (f32,f32)) -> Button {
		let model = ModelBuilder::new()
			.shape(&[
				[1.0, 0.0, 0.0, 1.0],
				[0.0, 0.0, 0.0, 1.0],
				[1.0, 1.0, 0.0, 1.0],
				[0.0, 1.0, 0.0, 1.0]],
				&[],
				&[
					[0.0, 0.0, 1.0, 1.0],
					[1.0 / 3.0, 1.0, 1.0, 1.0],
					[1.0 / 3.0, 0.0, 1.0, 1.0],
					[0.0, 1.0, 1.0, 1.0],
				])
			.finish(window);

//		let style = Shape::Texture(&v, 0/*include_bytes!("res/button.ppm")*/,&tc);
//		let spr = Sprite::create(window, style, 1);

//		let button = &window.button;
//		let spr = SpriteList::new(model).texture(window, button, tc)
//			.only();

		let spr = Sprite(window.window.shape_texture(&model.0,
			Mat4::new(), &window.button.0, false, true, true));


		let size = window.unit_size();

		let button = Button {
			sprite: spr,
			xmax: pos.0 + size.0, xmin: pos.0,
			ymax: pos.1 + size.1, ymin: pos.1,
			held: false,
		};

		button
	}

	/// Update appearance of button, depending on state of mouse, and return
	/// `true` when the button is pressed.
	pub fn update(&mut self, window: &mut Window, input: Input) -> bool {
		match input {
			Input::Cursor(xy) => {
				if let Some((x, y)) = xy {
					self.button_check(window, x, y);
				} else {
					self.held = false;
					self.away(window);
				}
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
			Input::CursorPress(_, (x, y)) => {
				self.held = true;
				self.button_check(window, x, y);
			},
			Input::CursorRelease(_, xy) => if self.held {
				if let Some((x, y)) = xy {
					self.held = false;
					return self.button_check(window, x, y);
				}
			},
			_ => {},
		}
		false
	}

	// Private Functions

	fn resize(&mut self, window: &mut Window) {
		Transform::new().auto(window, (self.xmin, self.ymin))
			.apply(window, &mut self.sprite);
	}

	fn modify(&mut self, _window: &mut Window, num: f32) {
		let xmin = num / 3.0;
		let xmax = (num + 1.0) / 3.0;
		let _v = [
			 0.0,  0.0, 0.0, 1.0,	xmin, 0.0, 1.0, 1.0,
			 1.0,  1.0, 0.0, 1.0,	xmax, 1.0, 1.0, 1.0,
			 1.0,  0.0, 0.0, 1.0,	xmax, 0.0, 1.0, 1.0,

			 1.0,  1.0, 0.0, 1.0,	xmax, 1.0, 1.0, 1.0,
			 0.0,  0.0, 0.0, 1.0,	xmin, 0.0, 1.0, 1.0,
			 0.0,  1.0, 0.0, 1.0,	xmin, 1.0, 1.0, 1.0,
		];
// TODO
//		self.sprite.vertices(window, &v);
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
		} else {
			self.held = false;
			self.away(window);
			false
		}
	}
}*/
