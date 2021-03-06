// "adi_screen" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use Graphic;
use GraphicBuilder;
use GraphicDecodeErr;
use adi_gpu;
use Window;

/// Macro to load textures from files for the window.
///
/// The first texture file listed is indexed 0, and each subsequent texture
/// increases by 1.  See: [`sprites!()`](macro.sprites.html) for example.
#[macro_export] macro_rules! textures {
	($window:expr, $decode:expr, $( $x:expr ),*) => { {
		let a = vec![ $( $crate::Texture::new($window,
			$decode(include_bytes!($x)).unwrap()) ),* ];

		$window.textures(a);
	} }
}

/// A reference to an image in GPU memory.
pub struct Texture(pub(crate) adi_gpu::Texture, pub(crate) u32, pub(crate) u32);

impl Texture {
	#[doc(hidden)]
	pub fn new(window: &mut Window, image_data: Graphic) -> Texture {
		let (w, h, _) = image_data.as_slice();

		Texture(window.window.texture(&image_data), w, h)
	}

	/// Load an empty texture into gpu memory.
	pub fn empty(window: &mut Window, w: u32, h: u32) -> Texture {
		let size = (w as usize) * (h as usize);
		let graphic = GraphicBuilder::new().rgba(w, h, vec![0; size]);

		Texture(window.window.texture(&graphic), w, h)
	}

	/// Load multiple texture from graphic data into gpu memory.
	pub fn new_vec<F>(window: &mut Window,
		loader: F, files: &[&[u8]])
		-> Result<Vec<Texture>, GraphicDecodeErr>
		where F: Fn(&[u8]) -> Result<Graphic, GraphicDecodeErr>
	{
		let mut textures = Vec::new();

		for i in files {
			textures.push(Texture::new(window, loader(i)?));
		}

		Ok(textures)
	}

	/// Get the width and height of the texture.
	pub fn wh(&self) -> (u32, u32) {
		(self.1, self.2)
	}

	/// Set the pixels for the texture.
	pub fn set(&mut self, window: &mut Window, data: &[u32]) -> () {
		window.window.set_texture(&mut self.0, data);
	}
}
