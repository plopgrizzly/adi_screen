// Aldaron's Device Interface / Screen
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/sprite.rs

use std::ptr::null_mut;
use std::f32::consts::PI;

use Window;
use { Texture, Model, Gradient, TexCoords };
use window::WindowFunctions;
use adi_gpu::{ Shape, ShapeBuilder };
use adi_gpu;

#[must_use]
/// Sprite represents anything that is rendered onto the screen.
pub struct Sprite(Shape);

#[must_use]
pub struct SpriteBuilder(ShapeBuilder, bool, bool);

/// Transform represents a transformation matrix.
pub struct Transform(adi_gpu::Transform);

/*fn sprite(window: &mut Window, shape: Shape) -> usize {
	window.sprites.push(shape); // Add sprite to end of vector
	window.sprites.len() - 1 // Length - 1 to get index of sprite.
}*/

impl SpriteBuilder {
	/// Create a new `SpriteBuilder`.
	#[inline(always)]
	pub fn new(vertices: Model) -> Self {
		SpriteBuilder(ShapeBuilder::new(vertices.0), false, false)
	}

	/// Enable alpha blending for this sprite.
	#[inline(always)]
	pub fn alpha(self) -> Self {
		SpriteBuilder(self.0, true, false)
	}

	/// Enable per-fragment alpha blending for this sprite.
	pub fn blend(self) -> Self {
		SpriteBuilder(self.0, true, true)
	}

	/// Create a sprite with a solid color.
	#[inline(always)]
	pub fn solid(&self, window: &mut Window, color: [f32; 4]) -> Sprite {
		Sprite(self.0.push_solid(&mut window.window, color, self.1,
			self.2))
	}

	/// Create a sprite shaded by a gradient (1 color per vertex).
	#[inline(always)]
	pub fn gradient(&self, window: &mut Window, colors: Gradient) -> Sprite {
		Sprite(self.0.push_gradient(&mut window.window, colors.0,
			self.1, self.2))
	}

	/// Create a sprite with a texture and texture coordinates.
	#[inline(always)]
	pub fn texture(&self, window: &mut Window, texture: Texture, tc: TexCoords)
		-> Sprite
	{
		Sprite(self.0.push_texture(&mut window.window, texture.0, tc.0,
			self.1, self.2))
	}

	/// Create a sprite with a texture, texture coordinates and alpha.
	/// Automatically Enables Alpha Blending. (no need to call `alpha()`)
	#[inline(always)]
	pub fn faded(&self, window: &mut Window, texture: Texture, tc: TexCoords,
		alpha: f32) -> Sprite
	{
		Sprite(self.0.push_faded(&mut window.window, texture.0, tc.0,
			alpha, self.2))
	}

	/// Create a sprite with a texture and texture coordinates and tint.
	#[inline(always)]
	pub fn tinted(&self, window: &mut Window, texture: Texture,
		tc: TexCoords, tint: [f32; 4]) -> Sprite
	{
		Sprite(self.0.push_tinted(&mut window.window, texture.0, tc.0,
			tint, self.1, self.2))
	}

	/// Create a sprite with a texture and texture coordinates and tint per
	/// vertex.
	#[inline(always)]
	pub fn complex(&self, window: &mut Window, texture: Texture,
		tc: TexCoords, tint_pv: Gradient) -> Sprite
	{
		Sprite(self.0.push_complex(&mut window.window, texture.0, tc.0,
			tint_pv.0, self.1, self.2))
	}
}

/*	/// Change the style of self to style for instance i.
	pub fn style(&self, window: &mut Window, i: usize, style: &Style) -> (){
		match *style {
			Style::Invisible => {
				Shape::enable(window, self.0, i, false);
			}
			Style::Texture(s, ref tx) => {
				let shader = window.shader(s);
				Shape::animate(window, self.0, i, tx, shader);
			}
			Style::Solid(s) => {
				let shader = window.shader(s);
				Shape::animate(window, self.0, i, null_mut(),
					shader);
			}
		}
	}*/

/*	/// Change the vertices of self to v.
	pub fn vertices(&mut self, window: &mut Window, v: &[f32]) -> () {
		self.0.vertices(window, self.0, v);
	}*/
//}

impl Transform {
	/// Create a transform that does nothing. ( Underneath, this is an
	/// identity matrix ).
	pub fn new() -> Transform {
		Transform (adi_gpu::Transform::new())
	}

	/// Translate self by x, y and z.
	pub fn translate(mut self, x:f32, y:f32, z:f32) -> Transform {
		Transform(self.0.translate(x, y, z))
	}

	/// Scale self by x, y and z.
	pub fn scale(mut self, x:f32, y:f32, z:f32) -> Transform {
		Transform(self.0.scale(x, y, z))
	}

	/// Rotate self by yaw, pitch and roll.
	pub fn rotate(self, yaw:f32, pitch:f32, roll:f32) -> Transform {
		Transform(self.0.rotate(yaw, pitch, roll))
	}

	/// Multiply by a projection that scales width and height by the
	/// smallest widget size. The widget is put at position pos. Position
	/// isn't affected by aspect ratio.
	pub fn auto(self, window: &mut Window, pos: (f32, f32)) -> Transform {
		let size = window.unit_size();
		let t = self.scale(size.0, size.1, 1.0)
			.translate(pos.0, pos.1, 0.0);
		t
	}

	/// Apply a TransformApply onto instance i of Sprite.
	pub fn apply(self, window: &mut Window, sprite: &Sprite) -> Transform {
		sprite.0.transform(&mut window.window, &self.0);

		self
	}
}
