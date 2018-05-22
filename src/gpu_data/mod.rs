// "adi_screen" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

mod tristrip;

use adi_gpu;
use Window;
use ami::{ Mat4, IDENTITY, Vec4 };

/// Macro to load multiple models into an array.
#[macro_export] macro_rules! models {
	($models:ident, $window:expr, $( $x:expr ),*) => {
		let $models = {
			use $crate::ModelBuilder as model;
			use $crate::IDENTITY;

			&[ $( include!($x).finish($window) ),* ]
		};
	}
}

/// The builder for `Model`.
pub struct ModelBuilder {
	// Should face double?
	double: bool,
	// Final output
	vertices: Vec<f32>,
	// Build a tristrip
	ts: Vec<[f32; 4]>,
	// Final output
	colors: Vec<f32>,
	// Build a tristrip
	colors_ts: Vec<[f32; 4]>,
	// Final output
	tcs: Vec<f32>,
	// Build a tristrip
	tcs_ts: Vec<[f32; 4]>,
	//
	color: Option<[f32; 4]>,
	//
	opacity: Option<f32>,
	//
	mat4: Mat4,
}

impl ModelBuilder {
	#[doc(hidden)]
	pub fn new() -> Self {
		ModelBuilder {
			double: false,
			vertices: vec![],
			ts: vec![],
			colors: vec![],
			colors_ts: vec![],
			tcs: vec![],
			tcs_ts: vec![],
			color: None,
			opacity: None,
			mat4: IDENTITY,
		}
	}

	/// Set transformation matrix
	pub fn m(mut self, mat4: Mat4) -> Self {
		self.mat4 = mat4;

		self
	}

	/// Set one color for the whole model.
	pub fn c(mut self, color: [f32;4]) -> Self {
		self.color = Some(color);
		self
	}

	/// Set the opacity for the whole model.
	pub fn o(mut self, opacity: f32) -> Self {
		self.opacity = Some(opacity);
		self
	}

	/// Set the colors for the following faces.
	pub fn g(mut self, vertices: &[[f32;4]]) -> Self {
		self.double = false;
		self.colors_ts = vec![];
		self.colors_ts.extend(vertices);
		self
	}

	/// Set the texture coordinates for the following faces.
	pub fn t(mut self, vertices: &[[f32;4]]) -> Self {
		self.double = false;
		self.tcs_ts = vec![];
		self.tcs_ts.extend(vertices);
		self
	}

	/// Set the vertices for the following faces.
	pub fn v(mut self, vertices: &[[f32;4]]) -> Self {
		let double = self.double;
		self.double = false;

		self.ts = vec![];
		self.ts.extend(vertices);

		if double {
			let half_colors = self.colors_ts.len() / 2;
			let half_tcs = self.tcs_ts.len() / 2;

			self.colors_ts.truncate(half_colors);
			self.tcs_ts.truncate(half_tcs);
		}

		tristrip::convert(self.ts.as_mut_slice(),
			if self.colors_ts.is_empty() { None }
			else { Some(self.colors_ts.as_mut_slice()) },
			if self.tcs_ts.is_empty() { None }
			else { Some(self.tcs_ts.as_mut_slice()) });

		self
	}

	/// Set the vertices for a double-sided face (actually 2 faces)
	pub fn d(mut self, vertices: &[[f32;4]]) -> Self {
		let single = !self.double;
		self.double = true;

		self.ts = vec![];
		self.ts.extend(vertices);
		self.ts.reverse();
		self.ts.extend(vertices);

		if single {
			self.colors_ts = [self.colors_ts.as_slice(), self.colors_ts.as_slice()].concat();
			self.tcs_ts = [self.tcs_ts.as_slice(), self.tcs_ts.as_slice()].concat();
		}

		tristrip::convert(&mut self.ts[0..vertices.len()],
			if self.colors_ts.is_empty() { None }
			else { Some(&mut self.colors_ts[..vertices.len()]) },
			if self.tcs_ts.is_empty() { None }
			else { Some(&mut self.tcs_ts[..vertices.len()]) });

		tristrip::convert(&mut self.ts[vertices.len()..],
			if self.colors_ts.is_empty() { None }
			else { Some(&mut self.colors_ts[vertices.len()..]) },
			if self.tcs_ts.is_empty() { None }
			else { Some(&mut self.tcs_ts[vertices.len()..]) });

		self
	}

	/// Add a face to the model, this unapplies the transformation matrix.
	pub fn f(mut self) -> Self {
		let len = self.ts.len();
//		if self.double {
//			let sidelen = len / 2;
//			self = self.shape(0, sidelen);
//			self = self.shape(sidelen, len);
//		} else {
			self = self.shape(0, len);
//		}

		self.mat4 = IDENTITY;

		self
	}

	/// Add a shape to the model.
	pub fn shape(mut self, b: usize, e: usize) -> Self {
		if self.ts.len() == 0 { return self; }

		// If there's already a shape, separate by a degenerate triangle
		let s = if self.vertices.is_empty() == false {
			// Start the next degenerate triangle (previous vertex).
			for _ in 0..4 {
				let v = self.vertices[self.vertices.len()-4];
				self.vertices.push(v);
			}
			// Finish the degenerate triangle (next vertex).
			let v = self.mat4 * Vec4::new(
				self.ts[0][0],
				self.ts[0][1],
				self.ts[0][2],
				self.ts[0][3],
			);

			if self.colors_ts.len() != 0 {
				for _ in 0..4 {
					let v = self.colors[self.colors.len()-4];
					self.colors.push(v);
				}
				for _ in 0..2 {
					self.colors.push(self.colors_ts[0][0]);
					self.colors.push(self.colors_ts[0][1]);
					self.colors.push(self.colors_ts[0][2]);
					self.colors.push(self.colors_ts[0][3]);
				}
			}
			if self.tcs_ts.len() != 0 {
				for _ in 0..4 {
					let v = self.tcs[self.tcs.len()-4];
					self.tcs.push(v);
				}
				for _ in 0..2 {
					self.tcs.push(self.tcs_ts[0][0]);
					self.tcs.push(self.tcs_ts[0][1]);
					self.tcs.push(self.tcs_ts[0][2]);
					self.tcs.push(self.tcs_ts[0][3]);
				}
			}

			for _ in 0..2 {
				self.vertices.push(v.x);
				self.vertices.push(v.y);
				self.vertices.push(v.z);
				self.vertices.push(v.w);
			}

			1
		} else { 0 };

		// Add the vertices
		for i in s+b..e {
			let v = self.mat4 * Vec4::new(
				self.ts[i][0],
				self.ts[i][1],
				self.ts[i][2],
				self.ts[i][3],
			);

			self.vertices.push(v.x);
			self.vertices.push(v.y);
			self.vertices.push(v.z);
			self.vertices.push(v.w);
		}

		if self.colors_ts.len() != 0 {
			for i in s+b..e {
				self.colors.push(self.colors_ts[i][0]);
				self.colors.push(self.colors_ts[i][1]);
				self.colors.push(self.colors_ts[i][2]);
				self.colors.push(self.colors_ts[i][3]);
			}
		}

		if self.tcs_ts.len() != 0 {
			for i in s+b..e {
				self.tcs.push(self.tcs_ts[i][0]);
				self.tcs.push(self.tcs_ts[i][1]);
				self.tcs.push(self.tcs_ts[i][2]);
				self.tcs.push(self.tcs_ts[i][3]);
			}
		}
	
		self
	}

	/// Create the model
	pub fn finish(self, window: &mut Window) -> Model {
		Model(window.window.model(self.vertices.as_slice()),
			if self.colors.is_empty() {
				None
			} else {
				assert!(self.colors.len() == self.vertices.len());
				Some(window.window.gradient(self.colors.as_slice()))
			},
			if self.tcs.is_empty() {
				None
			} else {
				println!("{} {}", self.tcs.len() / 4, self.vertices.len() / 4);
				assert_eq!(self.tcs.len(), self.vertices.len());
				Some(window.window.texcoords(self.tcs.as_slice()))
			}, self.color, self.opacity)
	}
}

/// A collection of indices and vertices
#[derive(Copy,Clone)]
pub struct Model(pub(crate) adi_gpu::Model,
	pub(crate) Option<adi_gpu::Gradient>,
	pub(crate) Option<adi_gpu::TexCoords>,
	pub(crate) Option<[f32; 4]>,
	pub(crate) Option<f32>);
