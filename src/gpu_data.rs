// Aldaron's Device Interface / Screen
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/gpu_data.rs

use adi_gpu;
use Window;

/// A collection of vertices
#[derive(Copy,Clone)]
pub struct Model(pub(crate) adi_gpu::Model);

impl Model {
	pub fn new(window: &mut Window, data: (&[u32], &[f32])) -> Model {
		Model(adi_gpu::Model::new(&mut window.window, data.1, data.0))
	}
}

/// A collection of vertices
#[derive(Copy,Clone)]
pub struct Gradient(pub(crate) adi_gpu::Gradient);

impl Gradient {
	pub fn new(window: &mut Window, data: &[f32]) -> Gradient {
		Gradient(adi_gpu::Gradient::new(&mut window.window, data))
	}
}

/// A collection of vertices
#[derive(Copy,Clone)]
pub struct TexCoords(pub(crate) adi_gpu::TexCoords);

impl TexCoords {
	pub fn new(window: &mut Window, data: &[f32]) -> TexCoords {
		TexCoords(adi_gpu::TexCoords::new(&mut window.window, data))
	}
}
