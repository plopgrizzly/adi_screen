// gpu_data.rs -- Aldaron's Device Interface / Screen
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

use adi_gpu;
use Window;

use adi_gpu::DisplayTrait;

/// The builder for `Model`.
pub struct ModelBuilder(Vec<f32>);

impl ModelBuilder {
	/// Generate builder for `Model`.
	pub fn new() -> Self {
		ModelBuilder(vec![])
	}

	/// Add a shape to the model.
	pub fn shape(mut self, vertices: &[(f32,f32,f32,f32)]) -> Self {
		// If there's already a shape, separate by a degenerate triangle
		if self.0.is_empty() == false {
			// Start the next degenerate triangle (previous vertex).
			for i in 0..4 {
				let v = self.0[self.0.len()-4];
				self.0.push(v);
			}
			// Finish the degenerate triangle (next vertex).
			self.0.push(vertices[0].0); // X
			self.0.push(vertices[0].1); // Y
			self.0.push(vertices[0].2); // Z
			self.0.push(vertices[0].3); // W
		}

		// Add the vertices
		for i in vertices {
			self.0.push(i.0); // X
			self.0.push(i.1); // Y
			self.0.push(i.2); // Z
			self.0.push(i.3); // W
		}
	
		self
	}

	/// Create the model
	pub fn finish(mut self, window: &mut Window) -> Model {
		Model(window.window.model(self.0.as_slice()))
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
