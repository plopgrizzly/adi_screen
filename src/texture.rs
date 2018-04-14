// texture.rs -- Aldaron's Device Interface / Screen
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

use afi::GraphicBuilder;
use adi_gpu;
use Window;

pub use adi_gpu::DisplayTrait;
pub use adi_gpu::TextureTrait;

/// Macro to load multiple textures into an array.
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
		Texture(window.window.texture(image_data))
	}

	/// Load an empty texture into gpu memory.
	pub fn empty(window: &mut Window, w: u32, h: u32) -> Texture {
		let size = (w as usize) * (h as usize);
		let graphic = GraphicBuilder::new().rgba(w, h, vec![0; size]);

		Texture(window.window.texture(graphic))
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

	/// Get the width and height of the texture.
	pub fn wh(&self) -> (u32, u32) {
		self.0.wh()
	}

	/// Set the pixels for the texture.
	pub fn set(&mut self, window: &mut Window, data: &[u32]) -> () {
		window.window.set_texture(&mut self.0, data);
	}
}
