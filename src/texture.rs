// Aldaron's Device Interface / Screen
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/texture.rs

use adi_gpu;
use Window;

#[macro_export] macro_rules! textures {
	( $window:expr, $decode:expr, $( $x:expr ),*) => {
		&[ $( $crate::Texture::new($window,
			$decode(include_bytes!($x)).unwrap()) ),* ]
	}
}

/// A reference to an image in GPU memory.
#[derive(Copy,Clone)]
pub struct Texture(pub(crate) adi_gpu::Texture);

impl Texture {
	/// Load a texture from graphic data into gpu memory.
	pub fn new(window: &mut Window, image_data: ::afi::Graphic) -> Texture {
		Texture(adi_gpu::Texture::new(&mut window.window, image_data))
	}

	/// Load an empty texture into gpu memory.
	pub fn empty(window: &mut Window, w: u32, h: u32) -> Texture {
		Texture(adi_gpu::Texture::empty(&mut window.window, w, h))
	}

	/// Load multiple texture from graphic data into gpu memory.
	pub fn new_vec<F>(window: &mut Window,
		loader: F, files: &[&[u8]])
		-> Result<Vec<Texture>, ::afi::GraphicDecodeErr>
		where F: Fn(&[u8]) -> Result<::afi::Graphic, ::afi::GraphicDecodeErr>
	{
		let mut textures = Vec::new();

		for i in files {
			textures.push(Texture::new(window, loader(i)?));
		}

		Ok(textures)
	}

	/// Get the width
	pub fn w(&self) -> u32 {
		self.0.w()
	}

	/// Get the height
	pub fn h(&self) -> u32 {
		self.0.h()
	}

	/// Set the pixels for the texture.
	pub fn set(&mut self, window: &mut Window, data: &[u32]) -> () {
		self.0.set(&mut window.window, data)
	}
}
