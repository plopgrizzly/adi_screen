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
	pub fn new(window: &mut Window, image_data: ::afi::Graphic) -> Texture {
		Texture(adi_gpu::Texture::new(&mut window.window, image_data))
	}
}
