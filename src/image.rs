/**
 * adi_screen - Aldaron's Device Interface - Screen - "image.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

pub struct Image {
	pub pixels: &'static [u8],
	pub size: (u32, u32),
	has_alpha: bool,
	alpha_key: (u8,u8,u8),
}

fn skip_line(ppm: &'static [u8], index: &mut usize) {
	loop {
		if ppm[*index] == '\n' as u8 {
			break;
		}
		*index += 1;
	}
	*index += 1;
}

fn utf8_to_u32(ppm: &'static [u8], index: &mut usize, until: char) -> u32 {
	let mut number = 0;
	while ppm[*index] != until as u8 {
		number *= 10;
		if ppm[*index] != '0' as u8 {
			number += ppm[*index] as u32 - 48;
		}
		*index += 1;
	}
	*index += 1;
	number
}

impl Image {
	pub fn load(ppm: &'static [u8]) -> Image {
		let mut index = 3;
		let size;

		// Header
		if ppm[0] != 'P' as u8 || ppm[1] != '6' as u8 {
			panic!("Not a PPM file.");
		}

		// Optional Comment
		if ppm[index] == '#' as u8 {
			skip_line(ppm, &mut index);
		}

		// Width & Height
		size = (
			utf8_to_u32(ppm, &mut index, ' '),
			utf8_to_u32(ppm, &mut index, '\n'),
		);

		// We don't care about this.  In ppm format 255 is normally here
		skip_line(ppm, &mut index);

		Image {
			pixels: &ppm[index..],
			size: size,
			has_alpha: false,
			alpha_key: (0,0,0),
		}
	}

	pub fn alpha_key(mut self, alpha_key: (u8,u8,u8)) -> Image {
		self.has_alpha = true;
		self.alpha_key = alpha_key;
		self
	}
}
