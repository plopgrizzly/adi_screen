/**
 * adi_screen - Aldaron's Device Interface - Screen - "style.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

const SHADER_COLOR : usize = 0;
const SHADER_TEXTURE : usize = 1;
const SHADER_CUSTOM : usize = 2;

use vw::NativeTexture;
use vw::Vw;
use image::Image;
use Window;

extern {
	fn vw_vulkan_texture(a: *mut Vw, b: u32, c: u32, d: *const u8, e: u8,
		f: u8, g: u8, h: u8) -> NativeTexture;
}

#[derive(Copy,Clone)]
pub enum Style {
	Invisible,
	Solid(usize),
	Opaque(usize, NativeTexture),
	Subtransparent(usize, NativeTexture),
	CustomSolid(usize),
	CustomTexture(usize, NativeTexture),
}

impl Style {
	pub fn create() -> Style {
		Style::Invisible
	}

	pub fn solid(self) -> Style {
		Style::Solid(SHADER_COLOR)
	}

	pub fn opaque(self, window: &mut Window, icon: &'static [u8]) -> Style {
		let icon = Image::load(icon);

		Style::Opaque(SHADER_TEXTURE, unsafe {
			vw_vulkan_texture(&mut window.vw, icon.size.0,
				icon.size.1, &icon.pixels[0], 0, 0, 0, 0)
		})
	}

	pub fn subtransparent(self, window: &mut Window, icon: &'static [u8],
		key: (u8,u8,u8)) -> Style
	{
		let icon = Image::load(icon).alpha_key(key);

		Style::Subtransparent(SHADER_TEXTURE, unsafe {
			vw_vulkan_texture(&mut window.vw, icon.size.0,
				icon.size.1, &icon.pixels[0], 1,
				key.0, key.1, key.2)
		})
	}

	pub fn shader(self, index: usize) -> Style {
		match self {
			Style::Invisible =>
				panic!("Can't customize invisible style."),
			Style::Solid(_) => Style::Solid(SHADER_CUSTOM + index),
			Style::Opaque(_, t) =>
				Style::Opaque(SHADER_CUSTOM + index, t),
			Style::Subtransparent(_, t) =>
				Style::Subtransparent(SHADER_CUSTOM + index, t),
			_ => panic!("Style's shader is already customized.")
		}
	}
}
