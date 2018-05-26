// "adi_screen" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

use Window;
use adi_gpu::{ Shape };
use ami::{ Mat4, IDENTITY };

/// Macro to create multiple sprites in an array.
///
/// # Example
/// ```
/// let mut window = WindowBuilder::new("Window Name", None).finish();
/// 
/// textures!(window, aci_png::decode,
/// 	"res/texture0.png", // 0
/// 	"res/texture1.png", // 1
/// );
/// 
/// models!(window, "res/model.data");
/// 
/// let sprites = sprites!(window,
/// 		(0/*model 0*/, Some(0/*texture 0*/),
/// 	Transform::new().translate(0.0, -0.5, 2.0), false),
/// 		(0/*model 0*/, Some(0/*texture 0*/),
/// 	Transform::new().translate(0.0, -4.5, 2.0), false));
/// ```
#[macro_export] macro_rules! sprites {
	($window:expr, $( $x:expr ),*) => {
		[ $( $crate::Sprite::new($window, $x.0, $x.1, $x.2, $x.3, false,
			true) ),* ]
	}
}

/// Macro to create multiple fog-affected sprites in an array.
/// # Example
/// See [`sprites!()`](macro.sprites.html)
#[macro_export] macro_rules! sprites_fog {
	($window:expr, $( $x:expr ),*) => {
		[ $( $crate::Sprite::new($window, $x.0, $x.1, $x.2, $x.3, true,
			true) ),* ]
	}
}

/// Macro to create multiple non-camera affected sprites in an array.
/// # Example
/// See [`sprites!()`](macro.sprites.html)
#[macro_export] macro_rules! sprites_gui {
	($window:expr, $( $x:expr ),*) => {
		[ $( $crate::Sprite::new($window, $x.0, $x.1, $x.2, $x.3, false,
			false) ),* ]
	}
}

#[must_use]
/// Sprite represents anything that is rendered onto the screen.
pub struct Sprite(pub(crate) Shape);

impl Sprite {
	#[doc(hidden)]
	pub fn new(window: &mut Window, model: usize,
		texture: Option<usize>, transform: Transform, alpha: bool,
		fog: bool, camera: bool) -> Self
	{
		if let Some(gradient) = window.models[model].1 {
			if let Some(texcoords) = window.models[model].2 {
				// Complex
				Sprite(window.window.shape_complex(
					&window.models[model].0, transform.0,
					&window.textures[texture.unwrap()].0,
					texcoords, gradient, alpha, fog, camera)
				)
			} else {
				// Gradient
				Sprite(window.window.shape_gradient(
					&window.models[model].0, transform.0,
					gradient, alpha, fog, camera)
				)
			}
		} else if let Some(texcoords) = window.models[model].2 {
			if let Some(color) = window.models[model].3 {
				// Tinted
				Sprite(window.window.shape_tinted(
					&window.models[model].0, transform.0,
					&window.textures[texture.unwrap()].0,
					texcoords, color, alpha, fog, camera))
			} else if let Some(opacity) = window.models[model].4 {
				// Faded
				Sprite(window.window.shape_faded(
					&window.models[model].0, transform.0,
					&window.textures[texture.unwrap()].0,
					texcoords, opacity, fog, camera))
			} else {
				// Texture
				Sprite(window.window.shape_texture(
					&window.models[model].0, transform.0,
					&window.textures[texture.unwrap()].0,
					texcoords, alpha, fog, camera))
			}
		} else if let Some(color) = window.models[model].3 {
			// Solid
			Sprite(window.window.shape_solid(&window.models[model].0,
				transform.0, color, alpha, fog, camera))
		} else {
			panic!("Not enough information to make Sprite!")
		}
	}
}

/// Transform represents a transformation matrix.
pub struct Transform(Mat4);

impl Transform {
	/// Create a transform that does nothing. ( Underneath, this is an
	/// identity matrix ).
	pub fn new() -> Transform {
		Transform (IDENTITY)
	}

	/// Translate self by x, y and z.
	pub fn translate(self, x:f32, y:f32, z:f32) -> Transform {
		Transform(self.0.translate(x, y, z))
	}

	/// Scale self by x, y and z.
	pub fn scale(self, x:f32, y:f32, z:f32) -> Transform {
		Transform(self.0.scale(x, y, z))
	}

	/// Rotate self by yaw, pitch and roll.
	pub fn rotate(self, yaw:f32, pitch:f32, roll:f32) -> Transform {
		Transform(self.0.rotate(yaw, pitch, roll))
	}

	/// Multiply by a projection that scales width and height by the
	/// smallest widget size. The widget is put at position pos. Position
	/// isn't affected by aspect ratio.
	pub fn auto(self, window: &mut Window, pos: (f32, f32)) -> Transform {
		let size = window.unit_size();
		let t = self.scale(size.0, size.1, 1.0)
			.translate(pos.0, pos.1, 0.0);
		t
	}

	/// Apply a TransformApply onto instance i of Sprite.
	pub fn apply(self, window: &mut Window, sprite: &mut Sprite)
		-> Transform
	{
		window.window.transform(&mut sprite.0, self.0);

		self
	}
}
