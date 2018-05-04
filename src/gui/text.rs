// gui/text.rs -- Aldaron's Device Interface / Screen
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

use fonterator;

// use Texture;
use Sprite;
use ModelBuilder;
// use TexCoords;
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
		/*let model = Model::new(window,
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
		]);*/
		let model = font
			.unwrap_or(&window.font)
			.render(w as usize, h as f32, text);

//		println!("{:?}", vertices);

		let model = model.finish(window);

		let sprite = SpriteList::new(model)
			.transform(Transform::new()
				.scale(0.2, 0.2, 0.2)
				.translate(self.1 .0, self.1 .1 + 1.0, 0.0)
			)
			.gui()
			.solid(window, [1.0, 1.0, 1.0, 1.0]) // (255, 255, 255, 255)
//			.texture(window, texture, tc)
			.only();

		// Actually render the text.
//		let mut buf = vec![0; (texture.wh().0 * texture.wh().1) as usize];

		self.0 = Some(sprite);
	}

	/// Set the position for the text.
	pub fn position(&mut self, x: f32, y: f32) {
		self.1 = (x, y);
	}
}

use fonterator::{ PathOp, Scale };

/// A font.
pub struct Font(
	fonterator::Font<'static>,
);

impl Font {
	pub fn new(font_data: &'static [u8]) -> Font {
		Font(
			fonterator::FontCollection::from_bytes(font_data)
				.unwrap()
				.into_font()
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
		let scale = Scale { x: size, y: size };
		let v_metrics = self.0.v_metrics(scale);
		let offset = fonterator::point(0.0, v_metrics.ascent);

		for glyph in self.0.layout(text, scale, offset) {
			for i in glyph.draw() {
				match i {
					PathOp::MoveTo(x, y) => {
						verts.push([x, y, 0.0, 0.0]);
						println!("Move({}, {})", x, y)
					},
					PathOp::LineTo(x, y) => {
						verts.push([x, y, 0.0, 0.0]);
						println!("Line({}, {})", x, y)
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

						println!("Quad({}, {}, {}, {})", x, y, cx, cy)
					},
					PathOp::LineClose => {
						model=model.v(verts.as_slice())
							.f();
						verts.clear();
					},
					PathOp::QuadClose(cx, cy) => {
						let x = verts[0][0];
						let y = verts[0][1];
						let sx = verts[verts.len() - 1][0];
						let sy = verts[verts.len() - 1][1];

						for i in 0+1..7+1 {
							let t = i as f32 / 8.0; // 0 - 1

							let dx = (1.0 - t) * (1.0 - t) * sx + 2.0 * (1.0 - t) * t * cx + t * t * x;
							let dy = (1.0 - t) * (1.0 - t) * sy + 2.0 * (1.0 - t) * t * cy + t * t * y;

							verts.push([dx, dy, 0.0, 0.0]);
						}

						model=model.v(verts.as_slice())
							.f();
						verts.clear();
					},
				}
			}
		}

		model




/*//		let mut wv = 0;
		let mut s = -1.0;

		// iterate over the characters in the string.
		for i in text.chars() {
			s += 1.0;
			let glyph = self.0.draw(i).unwrap();
			if glyph.is_none() { continue }
			let glyph = glyph.unwrap();
//			let mut a = font::Offset::default()
//				+ (font::Offset(glyph.advance_width(), 0.0) * s);
			let mut z = (0.0, 0.0);
			'a: for contour in glyph.iter() {
				let mut a = (
					glyph.advance_width() * s + contour.offset.0,
					contour.offset.1
				);

//				let offset = contour.offset;
				println!("new contour {:?}", a);*/
//				let mut prev = font::Offset(0.0, 0.0);
//				let mut direction = None;
//				z+= (0.0, contour.offset.1);
//				a = z;
				// vertices.push([a.0, -a.1, 0.0, 0.0]);
//				let mut origin = wv;
//				wv += 1;
//				let mut side = true;
/*				for segment in contour.iter() {
					match segment {
						&Segment::Linear(xy) => {
							a.0 += xy.0;
							a.1 += xy.1;
							println!("linear [{}, {}]", a.0, -a.1);
							vertices.push([a.0,
								-a.1,
								0.0,
								0.0]);*/
							/*if side == false {
								let normalized = normalize(a - xy);
								let new_d = dot_product(normalized, perp(prev));
								if let Some(d) = direction {
									if d != new_d { // concave
										origin = wv - 1;
									} else {
										indices.push(wv - 1);
										indices.push(origin);
										indices.push(wv);
									}
								}
								direction = Some(new_d);
								prev = normalized; // reset
							} else {
								prev = normalize(a - xy);
								side = false;
							}
							wv += 1;*//*
						},
						&Segment::Quadratic(xy, pa) => {
							let c = a;
							let b = {
								(a.0 + xy.0, a.1 + xy.1)
							};
							let pa = {
								(pa.0 + xy.0, pa.1 + xy.1)
							};
							a = {
								(a.0 + pa.0, a.1 + pa.1)
							};

							vertices.push([
								a.0,
								-a.1,
								0.0,
								0.0]);

							// interpolation size 8
							for i in 0+1..8+1 {
								let i = i as f32;
								let d = (Offset(c.0*i/8.0,c.1*i/8.0)+Offset(b.0*(1.0-i/8.0),b.1*(1.0-i/8.0))
								 + Offset(b.0*i/8.0,b.1*i/8.0)+Offset(a.0*(1.0-i/8.0),a.1*(1.0-i/8.0))) / 2.0;

								*///vertices.push([
								//	d.0,
								//	-d.1,
								//	0.0,
								//	0.0]);

								/*if side == false {
									let normalized = normalize(a - xy);
									let new_d = dot_product(normalized, perp(prev));
									if let Some(d) = direction {
										if d != new_d { // concave
											origin = wv - 1;
										} else {
											indices.push(wv - 1);
											indices.push(origin);
											indices.push(wv);
										}
									}
									direction = Some(new_d);
									prev = normalized; // reset
								} else {
									prev = normalize(a - xy);
									side = false;
								}
								wv += 1;*//*
							}
						},
						&Segment::Cubic(_, _, _) => {
							panic!("cubic curve in \
								font is\
								unsupported");
						},
					}
				}
				// Last vertex = first vertex in a contour
				//if *vertices.last().unwrap() == vertices[0] {
				//	vertices.pop();
				//}
				model = model.v(vertices.as_slice()).f();
				vertices.clear();
//				break 'a;
			}
		}

		model*/

		/*let pixel_height = height.ceil() as usize;
		let scale = rusttype::Scale { x: height, y: height };
		let v_metrics = self.0.v_metrics(scale);
		let offset = rusttype::point(0.0, v_metrics.ascent);

		let glyphs: Vec<rusttype::PositionedGlyph> = self.0
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
		}*/
	}
}
