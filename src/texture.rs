// Aldaron's Device Interface / Screen
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/texture.rs

use adi_gpu;
use Window;

/// A reference to an image in GPU memory.
#[derive(Copy,Clone)]
pub struct Texture(pub(crate) adi_gpu::Texture);

impl Texture {
	/// Load a texture from graphic data into gpu memory.
	pub fn new(window: &mut Window, image_data: ::afi::Graphic) -> Texture {
		Texture(adi_gpu::Texture::new(&mut window.window, image_data))
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
}
