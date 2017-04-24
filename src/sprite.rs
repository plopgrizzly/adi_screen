/**
 * adi_screen - Aldaron's Device Interface - Screen - "sprite.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use transforms::Matrix;

use input::Input;
use Screen;
use vw::{ Texture, Style, Shape };

pub struct Sprite<T> {
	context: Vec<T>,
	index: usize,
	pub callback: fn(screen: &mut Screen, context: &mut Sprite<T>, i: usize,
		event: Input) -> isize,
}

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

impl<T> Sprite<T> {
	pub fn colored(screen: &mut Screen, v: &[f32], style: &Style,
		callback: fn(win: &mut Screen, context: &mut Sprite<T>,
			i: usize, event: Input) -> isize) -> Sprite<T>
	{
		let shape = Shape::colored(screen, v, style);
		let index = sprite(screen, shape);
		let ctx = Vec::new();
		Sprite { callback: callback, context: ctx, index: index }
	}

	pub fn textured(screen: &mut Screen, v: &[f32], style: &Style,
		callback: fn(screen: &mut Screen, context: &mut Sprite<T>,
			i: usize, event: Input) -> isize) -> Sprite<T>
	{
		let shape = Shape::textured(screen, v, style);
		let index = sprite(screen, shape);
		let ctx = Vec::new();
		Sprite { callback: callback, context: ctx, index: index }
	}

	pub fn texcopy(&mut self, screen: &mut Screen, matrix: &Matrix,
		texture: &Texture, context: T) -> ()
	{
		let sprite = screen.sprites.get_mut(self.index).unwrap();
		sprite.shape.texclone(&matrix, texture);
		self.context.push(context);
	}

	pub fn copy(&mut self, screen: &mut Screen, matrix: &Matrix, context: T)
		-> ()
	{
		let sprite = screen.sprites.get_mut(self.index).unwrap();
		sprite.shape.clone(&matrix);
		self.context.push(context);
	}

	pub fn enabled(&mut self, screen: &mut Screen, enabled: bool) -> () {
		screen.sprites[self.index].enabled = enabled;
	}

	pub fn run(&mut self, screen: &mut Screen, event: Input) -> isize {
		let mut a = 0;
		if screen.sprites[self.index].enabled {
			for i in 0..self.context.len() {
				let r = (self.callback)(screen, self, i, event);
				if r != 0 { a = r }
			}
		}
		a
	}

	pub fn count(&self) -> usize {
		self.context.len()
	}

	pub fn event(&mut self, screen: &mut Screen, i: usize, event: Input)
		-> isize
	{
		if screen.sprites[self.index].enabled {
			(self.callback)(screen, self, i, event)
		}else{
			-1
		}
	}

	pub fn matrix(&mut self, screen: &mut Screen, i: usize, matrix: &Matrix)
		-> ()
	{
		let sprite = screen.sprites.get_mut(self.index).unwrap();
		sprite.shape.matrix(i, matrix);
	}

	pub fn animate(&mut self, screen: &mut Screen, i: usize,
		texture: &Texture, matrix: &Matrix) -> ()
	{
		let sprite = screen.sprites.get_mut(self.index).unwrap();
		sprite.shape.animate(i, texture, matrix);
	}

	pub fn vertices(&mut self, screen: &mut Screen, v: &[f32]) -> () {
		let sprite = screen.sprites.get_mut(self.index).unwrap();
		sprite.shape.vertices(v);
	}

	pub fn context(&mut self, i: usize) -> &mut T {
		&mut self.context[i]
	}
}
