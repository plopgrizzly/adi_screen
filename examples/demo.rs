// Aldaron's Device Interface / Screen
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// examples/demo.rs

#[macro_use]
extern crate adi_screen;
extern crate aci_png;

use adi_screen::{
	Transform,
	Sprite,
	Window,
	Input,
	GuiButton,
	Msg,
	Texture,
	SpriteList,
};

const TRIANGLE_MODEL: (&'static [u32], &'static [f32]) = include!("res/triangle.data");
const SQUARE_MODEL: (&'static [u32], &'static [f32]) = include!("res/square.data");
const SHAPE_VERTICES: &'static [f32] = &include!("res/image.vertices");
const SHAPE_INDICES: &'static [u32] = &include!("res/image.indices");
const SHAPE_TEXC: &'static [f32] = &include!("res/image.texc");

struct DemoApp {
	window: Window,
	drawable: Drawable,
	running: bool,
	gpu_data: GpuData,
}

struct Drawable {
	triangle_a: Sprite,
	triangle_b: Sprite,
	button: GuiButton,
}

struct GpuData {
	tex_logo: Texture,
}

impl DemoApp {
	fn animate(&mut self) {
		let disp = self.window.pulse_full_smooth(8.0);
		let disp2 = self.window.pulse_full_linear(4.0);

		Transform::new()
			.translate(-0.5, -0.5, 0.0)//5.0 * disp)
			.translate(disp * 1.0, 0.0, 0.0)
			.rotate(0.0, 0.0, disp)
			.apply(&mut self.window, &mut self.drawable.triangle_a);
		Transform::new()
			.translate(-0.5, 0.5, 0.0)//5.0 * disp)
			.translate(disp2 * 1.0, 0.0, 0.0)
			.apply(&mut self.window, &mut self.drawable.triangle_b);

		self.window.background((disp, 0.0, disp));
	}

	fn input(&mut self, input: Input) {
		use Input::*;
		use Msg::*;

		match input {
			Msg(Quit) | Msg(Back) => self.running = false,
//			Resize => self.drawable = resize(&mut self.window, self.gpu_data.tex_logo),
			Resume => println!("Resume ( Gain Focus )"),
			Pause => println!("Pause ( Lose Focus )"),
			KeyPress(a) => println!("Key Press: {}", a),
			KeyRelease(a) => println!("Key Release: {}", a),
			Cursor(xy) => {
				if let Some((x, y)) = xy {
					println!("Cursor: ({}, {})", x, y)
				} else {
					println!("Cursor: Out of window")
				}
			},
			CursorPress(a, (x, y)) => {
				println!("Cursor Press {}: ({}, {})", a, x, y)
			},
			CursorRelease(a, xy) => {
				if let Some((x, y)) = xy {
					println!("CursRel {} ({}, {})", a, x, y)
				} else {
					println!("CursRel {}: Out of window", a)
				}
			},
			ScrollUp(x, y) => println!("Scroll Up ({}, {})", x, y),
			ScrollDown(x, y) => println!("Scroll Down ({}, {})", x, y),
			ScrollRight(x, y) => println!("Scroll Right ({}, {})", x, y),
			ScrollLeft(x, y) => println!("Scroll Left ({}, {})", x, y),
			JoystickMove(x, y) => println!("Joystick ({}, {})", x, y),
			JoystickPov(x, y) => println!("POV Hat ({}, {})", x, y),
			JoystickThrottle(x) => println!("Throttle ({})", x),
			JoystickButtonDown(a) => println!("Button Down ({})", a),
			JoystickButtonUp(a) =>  println!("Button Up ({})", a),
			Text(a) => match a {
				'\u{7f}' => println!("Delete"),
				'\u{08}' => println!("Backspace"),
				'\u{91}' => println!("Left"),
				'\u{92}' => println!("Right"),
				'\u{9e}' => println!("Up"),
				'\u{9f}' => println!("Down"),
				'\n' => println!("New Line"),
				_ => println!("Text Input: {} {}", a, a as u32)
			},
			Msg(a) => println!("Message: {}", a),
			_ => println!("Other")
		};
		let pressed = self.drawable.button.update(&mut self.window, input);
		if pressed {
			println!("button been pressed!");
		}
	}

	fn update(&mut self) {
		self.animate();

		// Go through this frame's input.
		while let Some(input) = self.window.input() {
			self.input(input);

/*			use Input::*;
			use Msg::*;

			match input {
				Msg(Quit) | Msg(Back) => break 'app,
				_ => {},
			}*/
		}

		self.window.update();
	}
}

fn resize(window: &mut Window, tex_logo: Texture) -> Drawable {
	let logo_model = adi_screen::Model::new(window,
		(SHAPE_INDICES, SHAPE_VERTICES));
	let square_model = adi_screen::Model::new(window, SQUARE_MODEL);
	let triangle_model = adi_screen::Model::new(window, TRIANGLE_MODEL);

	let image_texcoords = adi_screen::TexCoords::new(window, SHAPE_TEXC);

	let gradient = adi_screen::Gradient::new(window,
		&include!("res/triangle.texc"));

	let mut sprites = SpriteList::new(triangle_model)
		.gradient(window, gradient)
		.gradient(window, gradient)
		// image
		.model(logo_model)
			.texture(window, tex_logo, image_texcoords) 
		.transform(Transform::new().scale(0.5, 0.5, 1.0))
			.texture(window, tex_logo, image_texcoords)
		.transform(Transform::new().scale(0.1, 0.1, 1.0))
			.model(square_model)
			.solid(window, [0.5, 0.5, 0.5, 0.5])
		.to_vec();

	let button = GuiButton::new(window, (-1.0, -1.0));

	let triangle_a = sprites.remove(0);
	let triangle_b = sprites.remove(0);

// TODO
//		image.style(&mut window, 0, &style_bear);

	Drawable { triangle_a, triangle_b, button }
}

fn init2() -> DemoApp {
	println!("WIN");
	// Create Window
	let mut window = Window::new("Demo",
		aci_png::decode(include_bytes!("res/logo.png")).unwrap(),
		(0.25, 0.25, 1.0), (20.0, 10.0));

	// Create Styles
//	let style_logo = Style::create().opaque(&mut window, icon);
//	let style_bear = Style::create().subtransparent(&mut window,
//		include_bytes!("res/plopgrizzly.ppm"), (0, 255, 0));

	println!("TEX");
	let tex_logo = textures!(&mut window, aci_png::decode, "res/logo.png");

	DemoApp {
		drawable: resize(&mut window, tex_logo[0]),
		window: window,
		running: true,
		gpu_data: GpuData { tex_logo: tex_logo[0] },
	}
}

fn main() {
	let mut app = init2();

	while app.running {
		app.update();
	}
}
