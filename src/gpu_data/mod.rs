// gpu_data/mod.rs -- Aldaron's Device Interface / Screen
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

mod tristrip;

use adi_gpu;
use Window;
use ami::{ Mat4, Vec4 };

use adi_gpu::DisplayTrait;

/// The builder for `Model`.
pub struct ModelBuilder {
	vertices: Vec<f32>,
	face: Vec<f32>,
	ts: tristrip::TriStrip,
	mat4: Mat4,
}

impl ModelBuilder {
	/// Generate builder for `Model`.
	pub fn new() -> Self {
		ModelBuilder {
			vertices: vec![],
			face: vec![],
			ts: tristrip::TriStrip::new(),
			mat4: Mat4::new(),
		}
	}

	/// Set transformation matrix
	pub fn m(mut self, mat4: Mat4) -> Self {
		self.mat4 = mat4;

		self
	}

	/// Set the vertices for the following faces.
	pub fn v(mut self, vertices: &[[f32;4]]) -> Self {
		self.ts = tristrip::TriStrip::new();
		self.ts.push(vertices);

		println!("Face split in {}", self.ts.points.len());

		self
	}

	/// Add a face to the model, this unapplies the transformation matrix.
	pub fn f(mut self) -> Self {
		for i in 0..self.ts.points.len() {
			println!("{}", self.ts.points[i].len());
			let points = self.ts.points[i].clone();
			println!("{:?}", points);
			self = self.shape(points.as_slice());
		}

		self.mat4 = Mat4::new();

		self
	}

	/// Add a shape to the model.
	pub fn shape(mut self, vertices: &[[f32;4]]) -> Self {
		if vertices.len() == 0 { return self; }

		// If there's already a shape, separate by a degenerate triangle
		let s = if self.vertices.is_empty() == false {
			// Start the next degenerate triangle (previous vertex).
			for _ in 0..4 {
				let v = self.vertices[self.vertices.len()-4];
				self.vertices.push(v);
			}
			// Finish the degenerate triangle (next vertex).
			let v = self.mat4 * Vec4::new(
				vertices[0][0],
				vertices[0][1],
				vertices[0][2],
				vertices[0][3],
			);

			for _ in 0..2 {
				self.vertices.push(v.x);
				self.vertices.push(v.y);
				self.vertices.push(v.z);
				self.vertices.push(v.w);
			}

			1
		} else { 0 };

		// Add the vertices
		for i in vertices.iter().skip(s) {
			let v = self.mat4 * Vec4::new(
				i[0],
				i[1],
				i[2],
				i[3],
			);

			self.vertices.push(v.x);
			self.vertices.push(v.y);
			self.vertices.push(v.z);
			self.vertices.push(v.w);
		}
	
		self
	}

	/// Create the model
	pub fn finish(self, window: &mut Window) -> Model {
		println!("RAW:{:?}", self.vertices);

		Model(window.window.model(self.vertices.as_slice()))
	}
}

/// A collection of indices and vertices
#[derive(Copy,Clone)]
pub struct Model(pub(crate) adi_gpu::Model);

/// A collection of colors, one for each vertex.
#[derive(Copy,Clone)]
pub struct Gradient(pub(crate) adi_gpu::Gradient);

impl Gradient {
	/// Create new `Gradient` based on `data`.
	pub fn new(window: &mut Window, data: &[f32]) -> Gradient {
		Gradient(window.window.gradient(data))
	}
}

/// Texture Coordinates for a `Model`.
#[derive(Copy,Clone)]
pub struct TexCoords(pub(crate) adi_gpu::TexCoords);

impl TexCoords {
	/// Create new texture coordinates based on `data`.
	pub fn new(window: &mut Window, data: &[f32]) -> TexCoords {
		TexCoords(window.window.texcoords(data))
	}
}
