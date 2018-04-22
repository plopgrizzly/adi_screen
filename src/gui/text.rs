// gui/text.rs -- Aldaron's Device Interface / Screen
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

use rusttype;
use rusttype::FontCollection;

use Texture;
use Sprite;
use Model;
use TexCoords;
use SpriteList;
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
	include_bytes!("res/font/SourceCodePro-Regular.ttf");

/// Text on the screen.
pub struct Text(Option<(Texture, Sprite)>, (f32, f32), (f32, f32));

impl Text {
	/// Add an empty text box to the screen.
	pub fn new(_: &mut Window, pos: (f32,f32), wh: (f32,f32)) -> Text {
		Text(None, pos, wh)
	}

	/// Update the texture.
	pub fn update(&mut self, window: &mut Window, text: &str, font: &[u8]) {
		let win_size = window.wh();
		let w = ((win_size.0 as f32 * self.2 .0) as u32) * (text.len() as u32) / 2;
		let h = (win_size.0 as f32 * self.2 .1) as u32;
		let mut texture = Texture::empty(window, w, h);
		let model = Model::new(window,
			(&[0, 1, 2, 1, 0, 3],
			&[
				0.0,  0.0, 0.0, 1.0,
				self.2 .0 * 0.5 * (text.len() as f32), self.2 .1, 0.0, 1.0,
				self.2 .0 * 0.5 * (text.len() as f32), 0.0, 0.0, 1.0,
				0.0,  self.2 .1, 0.0, 1.0,
			])
		);
		let tc = TexCoords::new(window, &[
			0.0, 0.0, 1.0, 1.0,
			1.0, 1.0, 1.0, 1.0,
			1.0, 0.0, 1.0, 1.0,
			0.0, 1.0, 1.0, 1.0,
		]);
		let sprite = SpriteList::new(model)
			.transform(Transform::new().translate(self.1 .0, self.1 .1, 0.0))
			.gui()
			.texture(window, texture, tc)
			.only();

		// Actually render the text.
		let font = Font::new(font);
		let mut buf = vec![0; (texture.wh().0 * texture.wh().1) as usize];

		font.render(w as usize, h as f32, buf.as_mut_slice(),
			(255, 255, 255, 255), text);
		texture.set(window, buf.as_slice());

		self.0 = Some((texture, sprite));
	}

	/// Set the position for the text.
	pub fn position(&mut self, x: f32, y: f32) {
		self.1 = (x, y);
	}
}

/// A font.
pub struct Font<'a>(&'a [u8], rusttype::Font<'a>);

impl<'a> Font<'a> {
	pub fn new(font_data: &'a [u8]) -> Font<'a> {
		Font(font_data, FontCollection::from_bytes(font_data).unwrap()
			.into_font().unwrap())
	}

	pub(crate) fn render(&self, width: usize, height: f32, buffer: &mut [u32],
		color: (u8, u8, u8, u8), text: &str) -> ()
	{
		let pixel_height = height.ceil() as usize;
		let scale = rusttype::Scale { x: height, y: height };
		let v_metrics = self.1.v_metrics(scale);
		let offset = rusttype::point(0.0, v_metrics.ascent);

		let glyphs: Vec<rusttype::PositionedGlyph> = self.1
			.layout(text, scale, offset)
			.collect();

		for g in glyphs {
			if let Some(bb) = g.pixel_bounding_box() {
				g.draw(|x, y, v| {
					let c = unsafe { ::std::mem::transmute([
						(color.0 as f32 * v) as u8,
						(color.1 as f32 * v) as u8,
						(color.2 as f32 * v) as u8,
						(color.3 as f32 * v) as u8
					]) };

					let x = x as i32 + bb.min.x;
					let y = y as i32 + bb.min.y;
					// There's still a possibility that the glyph clips the boundaries of the bitmap
					if x >= 0 && x < width as i32 && y >= 0 && y < pixel_height as i32 {
						let x = x as usize;
						let y = y as usize;
						buffer[(x + y * width)] = c;
					}
				});
			}
		}
	}
}
