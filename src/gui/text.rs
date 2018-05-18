// "adi_screen" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use fonterator;

// use Texture;
use Sprite;
use ModelBuilder;
// use TexCoords;
use Window;
use window::WindowFunctions;
use Transform;

/// Macro to set text.
#[macro_export] macro_rules! text {
	( $window:expr, $text:expr, $font:expr, $( $x:expr ),*) => {
		let text: Text = $text;
		let window: Window = $window;

		// TODO don't use format!() 
		text.update(window, &format!(text), font);

//		&[ $( $crate::Texture::new($window,
//			$decode(include_bytes!($x)).unwrap()) ),* ]
	}
}

/// A font that's built into the library.
pub const DEFAULT_FONT: &'static [u8] =
	include_bytes!("res/font/DejaVuSansMono.ttf");

/// Text on the screen.
pub struct Text(Option<Sprite>, (f32, f32), (f32, f32));

impl Text {
	/// Add an empty text box to the screen.
	pub fn new(_: &mut Window, pos: (f32,f32), wh: (f32,f32)) -> Text {
		Text(None, pos, wh)
	}

	/// Update the texture.
	pub fn update(&mut self, window: &mut Window, text: &str,
		font: Option<&Font>)
	{
		let win_size = window.wh();
		let w = ((win_size.0 as f32 * self.2 .0) as u32) * (text.len() as u32) / 2;
		let h = (win_size.0 as f32 * self.2 .1) as u32;

		let model = font
			.unwrap_or(&window.font)
			.render(w as usize, h as f32, text);

		let model = model.c([1.0, 1.0, 1.0, 1.0]).finish(window);

		let sprite = Sprite::new(window, &model, None,
			Transform::new()
				.scale(0.2, 0.2, 0.2)
				.translate(self.1 .0, self.1 .1 + 1.0, 0.0),
			false, false, false);

		self.0 = Some(sprite);
	}

	/// Set the position for the text.
	pub fn position(&mut self, x: f32, y: f32) {
		self.1 = (x, y);
	}
}

use fonterator::{ PathOp };

/// A font.
pub struct Font(
	fonterator::Font<'static>,
);

impl Font {
	pub fn new(font_data: &'static [u8]) -> Font {
		Font(
			fonterator::Font::new(font_data)
				.unwrap()
		)
//		let mut reader = ::std::io::Cursor::new(font_data);

//		Font(fonterator::Font::read(&mut reader).unwrap())
	}

	fn render(&self, _width: usize, _height: f32, // TODO
		text: &str) -> ModelBuilder
	{
		let mut model = ModelBuilder::new();
		let mut verts = vec![];

		let size = (1.0f32).ceil();
		let mut x = 0.0;

		for glyph in self.0.glyphs(text, (size, size)) {
			for i in glyph.0.draw(x, 0.0) {
				match i {
					PathOp::MoveTo(x, y) => {
						verts.push([x, y, 0.0, 0.0]);
					},
					PathOp::LineTo(x, y) => {
						verts.push([x, y, 0.0, 0.0]);
					},
					PathOp::QuadTo(x, y, cx, cy) => {
						let sx = verts[verts.len() - 1][0];
						let sy = verts[verts.len() - 1][1];

						for i in 0+1..8+1 {
							let t = i as f32 / 8.0; // 0 - 1

							let dx = (1.0 - t) * (1.0 - t) * sx + 2.0 * (1.0 - t) * t * cx + t * t * x;
							let dy = (1.0 - t) * (1.0 - t) * sy + 2.0 * (1.0 - t) * t * cy + t * t * y;

							verts.push([dx, dy, 0.0, 0.0]);
						}
					},
					PathOp::Close => {
						model=model.v(verts.as_slice())
							.f();
						verts.clear();
					},
				}
			}
			x += glyph.1;
		}

		model
	}
}
