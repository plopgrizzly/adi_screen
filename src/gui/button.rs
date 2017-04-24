/**
 * adi_screen - Aldaron's Device Interface - Screen - "gui/button.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use transforms::Matrix;

use input::Input;
use Screen;
use Sprite;
use Texture;
// use screen::{ Sprite, Screen, Texture };
use vw::Style;

pub struct Button {
	xmin: f32,
	xmax: f32,
	ymin: f32,
	ymax: f32,
	held: bool,
}

impl Button {
	pub fn add(screen: &mut Screen, style: &Style, pos: (f32,f32))
		-> Sprite<Button>
	{
		let v = [
			 0.0,  0.0, 0.0, 1.0,	0.0, 0.0, 1.0, 1.0,
			 1.0,  1.0, 0.0, 1.0,	1.0 / 3.0, 1.0, 1.0, 1.0,
			 1.0,  0.0, 0.0, 1.0,	1.0 / 3.0, 0.0, 1.0, 1.0,

			 1.0,  1.0, 0.0, 1.0,	1.0 / 3.0, 1.0, 1.0, 1.0,
			 0.0,  0.0, 0.0, 1.0,	0.0, 0.0, 1.0, 1.0,
			 0.0,  1.0, 0.0, 1.0,	0.0, 1.0, 1.0, 1.0,
		];
		let matrix = Matrix::identity()
			.scale((screen.minsize.1).0, (screen.minsize.1).1, 1.0)
			.translate(pos.0, pos.1, 0.0);
		let button = Button {
			xmax: pos.0 + (screen.minsize.1).0,
			xmin: pos.0,
			ymax: pos.1 + (screen.minsize.1).1,
			ymin: pos.1,
			held: false,
		};
		let texture = Texture::opaque(screen,
			include_bytes!("../res/button.ppm"));
		let mut rtn = Sprite::textured(screen, &v, style, button_event);
		rtn.texcopy(screen, &matrix, &texture, button);
		rtn
	}

	pub fn lx(&self) -> f32 {
		self.xmin
	}
}

impl Sprite<Button> {
	fn modify(&mut self, screen: &mut Screen, num: f32) {
		let (lx, ly) = {
			let context = self.context(0);
			(context.lx(), context.ymin)
		};
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
		let matrix = Matrix::identity()
			.scale((screen.minsize.1).0, (screen.minsize.1).1, 1.0)
			.translate(lx, ly, 0.0);
		self.vertices(screen, &v);
		self.matrix(screen, 0, &matrix);
	}

	fn away(&mut self, screen: &mut Screen) {
		self.modify(screen, 0.0)
	}

	fn over(&mut self, screen: &mut Screen) {
		self.modify(screen, 1.0)
	}

	fn down(&mut self, screen: &mut Screen) {
		self.modify(screen, 2.0)
	}
}

fn button_check(screen: &mut Screen, button: &mut Sprite<Button>, x: f32, y: f32) {
	let (xmax, xmin, ymax, ymin, held) = {
		let c = button.context(0);
		(c.xmax, c.xmin, c.ymax, c.ymin, c.held)
	};
	if x >= xmin && x <= xmax && y >= ymin && y <= ymax {
		if held {
			button.down(screen);
		}else{
			button.over(screen);
		}
	}else{
		button.away(screen);
	}
}

fn button_event(screen: &mut Screen, button: &mut Sprite<Button>, _: usize,
	event: Input) -> isize
{
	match event {
		Input::Cursor(x, y) => {
			button_check(screen, button, x, y);
		},
		Input::Resize(_, _) => {
			let pos = (button.context(0).xmin, button.context(0).ymin);
			*button.context(0) = Button {
				xmax: pos.0 + (screen.minsize.1).0,
				xmin: pos.0,
				ymax: pos.1 + (screen.minsize.1).1,
				ymin: pos.1,
				held: false,
			};
			button.away(screen);
		},
		Input::LeftDown(x, y) => {
			button.context(0).held = true;
			button_check(screen, button, x, y);
		},
		Input::LeftUp(x, y) => {
			button.context(0).held = false;
			button_check(screen, button, x, y);
		},
		Input::LeaveWindow => {
			button.context(0).held = false;
			button.away(screen);
		},
		_ => {},
	}
	0
}
