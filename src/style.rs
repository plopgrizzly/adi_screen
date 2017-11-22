// Aldaron's Device Interface / Screen
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/style.rs

/*const SHADER_COLOR : usize = 0;
const SHADER_TEXTURE : usize = 1;
const SHADER_CUSTOM : usize = 2;

use renderer::NativeTexture;
use renderer::Vw;
use image::Image;
use Window;

extern {
	fn vw_vulkan_texture(a: *mut Vw, b: u32, c: u32, d: *const u8, e: u8,
		f: u8, g: u8, h: u8) -> NativeTexture;
}

#[derive(Copy,Clone)]
/// Style represents a shader with an optionally attached texture.
pub enum Style {
	Invisible,
	Solid(usize),
	Texture(usize, NativeTexture),
}

impl Style {
	/// Create a new style.  Used on it's own, the style is invisible.
	pub fn create() -> Style {
		Style::Invisible
	}

	/// Use the second 4 values on a vertex as RGB colors.
	pub fn gradient(self) -> Style {
		Style::Solid(SHADER_COLOR)
	}

	/// Set the style to opaque image from ppm data, image.
	pub fn opaque(self, window: &mut Window, image: &'static [u8]) -> Style{
		let image = Image::load(image);

		Style::Texture(SHADER_TEXTURE, unsafe {
			vw_vulkan_texture(&mut window.vw, image.size.0,
				image.size.1, &image.pixels[0], 0, 0, 0, 0)
		})
	}

	/// Set the style to ppm image, image, with pixels with the color, key,
	/// replaced with a transparent pixel.
	pub fn subtransparent(self, window: &mut Window, image: &'static [u8],
		key: (u8,u8,u8)) -> Style
	{
		let image = Image::load(image).alpha_key(key);

		Style::Texture(SHADER_TEXTURE, unsafe {
			vw_vulkan_texture(&mut window.vw, image.size.0,
				image.size.1, &image.pixels[0], 1,
				key.0, key.1, key.2)
		})
	}

	/// Apply custom shader at index, index, to self.
	pub fn apply(self, index: usize) -> Style {
		match self {
			Style::Invisible =>
				panic!("Can't customize invisible style."),
			Style::Solid(_) => Style::Solid(SHADER_CUSTOM + index),
			Style::Texture(_, t) =>
				Style::Texture(SHADER_CUSTOM + index, t),
		}
	}
}*/
