/**
 * adi_screen - Aldaron's Device Interface - Screen - "sprite.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use std::ptr::null_mut;
use std::f32::consts::PI;

use Screen;
use vw::{ Shape };
use Style;

#[must_use]
pub struct Sprite(usize);

#[must_use]
pub struct Transform([f32; 16]);

pub struct TransformApply(Transform);

pub struct SpriteData {
	pub enabled: bool, // Is the sprite going to be drawn and used?
	pub shape: Shape, // The shape to render.
}

fn sprite(screen: &mut Screen, shape: Shape) -> usize {
	let sprite = SpriteData {
		enabled: true,
		shape: shape,
	};
	screen.sprites.push(sprite); // Add sprite to end of vector
	screen.sprites.len() - 1 // Length - 1 to get index of sprite.
}

impl Sprite {
	pub fn create(screen: &mut Screen, v: &[f32], style: Style,
		instances: u32) -> Sprite
	{
		let shape = Shape::create(screen, v, style);
		let index = sprite(screen, shape);

		match style {
			Style::Opaque(_, ref tx) |
				Style::Subtransparent(_, ref tx) =>
			{
				for _ in 0..instances {
					Shape::add(screen, index, tx);
				}
			}
			Style::Solid(_) => {
				for _ in 0..instances {
					Shape::add(screen, index, null_mut());
				}
			}
			_ => panic!("This style type is unsupported.")
		}

		Sprite(index)
	}

	pub fn animate(&self, screen: &mut Screen, i: usize, style: &Style)->(){
		match *style {
			Style::Invisible => {
				screen.sprites[i].enabled = false;
			}
			Style::Opaque(_,ref tx)|Style::Subtransparent(_,ref tx)
				=>
			{
				Shape::animate(screen, self.0, i, tx);
			}
			_ => {
				panic!("Can't animate with this style.")
			}
		}
	}

	pub fn vertices(&mut self, screen: &mut Screen, v: &[f32]) -> () {
		Shape::vertices(screen, self.0, v);
	}
}

impl Transform {
	fn combine(mut self, matrix: [f32; 16]) -> Transform {
		self.0 = [
			(self.0[0] * matrix[0]) + (self.0[1] * matrix[4]) +
			(self.0[2] * matrix[8]) + (self.0[3] * matrix[12]),
			(self.0[0] * matrix[1]) + (self.0[1] * matrix[5]) +
			(self.0[2] * matrix[9]) + (self.0[3] * matrix[13]),
			(self.0[0] * matrix[2]) + (self.0[1] * matrix[6]) +
			(self.0[2] * matrix[10]) + (self.0[3] * matrix[14]),
			(self.0[0] * matrix[3]) + (self.0[1] * matrix[7]) +
			(self.0[2] * matrix[11]) + (self.0[3] * matrix[15]),

			(self.0[4] * matrix[0]) + (self.0[5] * matrix[4]) +
			(self.0[6] * matrix[8]) + (self.0[7] * matrix[12]),
			(self.0[4] * matrix[1]) + (self.0[5] * matrix[5]) +
			(self.0[6] * matrix[9]) + (self.0[7] * matrix[13]),
			(self.0[4] * matrix[2]) + (self.0[5] * matrix[6]) +
			(self.0[6] * matrix[10]) + (self.0[7] * matrix[14]),
			(self.0[4] * matrix[3]) + (self.0[5] * matrix[7]) +
			(self.0[6] * matrix[11]) + (self.0[7] * matrix[15]),

			(self.0[8] * matrix[0]) + (self.0[9] * matrix[4]) +
			(self.0[10] * matrix[8]) + (self.0[11] * matrix[12]),
			(self.0[8] * matrix[1]) + (self.0[9] * matrix[5]) +
			(self.0[10] * matrix[9]) + (self.0[11] * matrix[13]),
			(self.0[8] * matrix[2]) + (self.0[9] * matrix[6]) +
			(self.0[10] * matrix[10]) + (self.0[11] * matrix[14]),
			(self.0[8] * matrix[3]) + (self.0[9] * matrix[7]) +
			(self.0[10] * matrix[11]) + (self.0[11] * matrix[15]),

			(self.0[12] * matrix[0]) + (self.0[13] * matrix[4]) +
			(self.0[14] * matrix[8]) + (self.0[15] * matrix[12]),
			(self.0[12] * matrix[1]) + (self.0[13] * matrix[5]) +
			(self.0[14] * matrix[9]) + (self.0[15] * matrix[13]),
			(self.0[12] * matrix[2]) + (self.0[13] * matrix[6]) +
			(self.0[14] * matrix[10]) + (self.0[15] * matrix[14]),
			(self.0[12] * matrix[3]) + (self.0[13] * matrix[7]) +
			(self.0[14] * matrix[11]) + (self.0[15] * matrix[15])
		];
		self
	}

	pub fn create() -> Transform {
		Transform ([
			1., 0., 0., 0.,
			0., 1., 0., 0.,
			0., 0., 1., 0.,
			0., 0., 0., 1.,
		])
	}

	pub fn translate(mut self, x:f32, y:f32, z:f32) -> Transform {
		self.0[12] += x;
		self.0[13] += y;
		self.0[14] += z;
		self
	}

	pub fn scale(mut self, x:f32, y:f32, z:f32) -> Transform {
		self.0[0] *= x;
		self.0[5] *= y;
		self.0[15] *= z;
		self
	}

	pub fn rotate(self, yaw:f32, pitch:f32, roll:f32) -> Transform {
		let num9 = roll * PI;
		let num6 = num9.sin();
		let num5 = num9.cos();
		let num8 = pitch * PI;
		let num4 = num8.sin();
		let num3 = num8.cos();
		let num7 = yaw * PI;
		let num2 = num7.sin();
		let num = num7.cos();

		let qx = ((num * num4) * num5) + ((num2 * num3) * num6);
		let qy = ((num2 * num3) * num5) - ((num * num4) * num6);
		let qz = ((num * num3) * num6) - ((num2 * num4) * num5);
		let qw = ((num * num3) * num5) + ((num2 * num4) * num6);

		let m1 = [
			qw, qz, -qy, qx,
			-qz, qw, qx, qy,
			qy, -qx, qw, qz,
			-qx, -qy, -qz, qw,
		];
		let m2 = [
			qw, qz, -qy, -qx,
			-qz, qw, qx, -qy,
			qy, -qx, qw, -qz,
			qx, qy, qz, qw,
		];
		self.combine(m1).combine(m2)
	}

	pub fn perspective(self, screen: &Screen, fov: f32) -> TransformApply {
		let scale = (fov * 0.5 * PI / 180.).tan().recip();
		let xscale = scale * screen.unit_ratio();
		let t = self.combine([
				xscale,	0.,	0.,	0.,
				0.,	scale,	0.,	0.,
				0.,	0.,	1.,	1.,
				0.,	0.,	0., 	1.,
			]);

		TransformApply(t)
	}

	pub fn orthographic(self, screen: &Screen) -> TransformApply {
		TransformApply(self.scale(screen.unit_ratio(), 1.0, 1.0))
	}

	pub fn auto(self, screen: &mut Screen, pos: (f32, f32))
		-> TransformApply
	{
		let t = self.scale(screen.unit_width(),screen.unit_height(),1.0)
			.translate(pos.0, pos.1, 0.0);
		TransformApply(t)
	}
}

impl TransformApply {
	pub fn on(self, screen: &mut Screen, sprite: &Sprite, i: usize)
		-> TransformApply
	{
		Shape::matrix(screen, sprite.0, i, self.0 .0);
		self
	}
}
