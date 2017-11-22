// Aldaron's Device Interface / Screen
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// examples/demo.rs

extern crate adi_screen;
extern crate aci_png;

use adi_screen::{
	Transform,
	Sprite,
	Window,
	Input,
	GuiButton,
	Msg,
	InputQueue,
	SpriteBuilder,
	Texture
};

const SHAPE_IMAGE: &[f32] = &include!("res/image.data");
const SHAPE_TEXC: &[f32] = &include!("res/image.texc");

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

		Transform::create()
			.translate(-0.5, -0.5, 0.0)//5.0 * disp)
			.translate(disp * 1.0, 0.0, 0.0)
			.rotate(0.0, 0.0, disp)
			.projection()
			.apply(&mut self.window, &self.drawable.triangle_a);
		Transform::create()
			.translate(-0.5, 0.5, 0.0)//5.0 * disp)
			.translate(disp2 * 1.0, 0.0, 0.0)
			.projection()
			.apply(&mut self.window, &self.drawable.triangle_b);

		self.window.background(disp, 0.0, disp);
	}

	fn input(&mut self, input: Input) {
		use Input::*;
		use Msg::*;

		match input {
			Msg(Quit) | Msg(Back) => self.running = false,
			Resize => self.drawable = resize(&mut self.window, self.gpu_data.tex_logo),
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

	fn update(&mut self, input_queue: &mut InputQueue) {
		self.animate();
		self.window.update(input_queue);

		for input in input_queue.iter() {
			self.input(*input);
		}
	}
}

fn resize(window: &mut Window, tex_logo: Texture) -> Drawable {
	let logo = SpriteBuilder::new(&SHAPE_IMAGE)
		.texture(window, tex_logo, &SHAPE_TEXC);
	let button = GuiButton::create(window, (-1.0, -1.0));
	let triangle_a = SpriteBuilder::new(&include!("res/triangle.data"))
		.gradient(window, &include!("res/triangle.texc"));
	let triangle_b = SpriteBuilder::new(&include!("res/triangle.data"))
		.gradient(window, &include!("res/triangle.texc"));
//	let _ = triangle_a.clone();
	let image = {
		let image = SpriteBuilder::new(&SHAPE_IMAGE)
			.texture(window, tex_logo, &SHAPE_TEXC);
// TODO
//		image.style(&mut window, 0, &style_bear);
		image
	};
	let square = SpriteBuilder::new(&include!("res/square.data"))
		.solid(window, [0.5, 0.5, 0.5, 0.5]);

	Transform::create().scale(0.1, 0.1, 1.0).projection()
		.apply(window, &square);
	Transform::create().projection().apply(window, &image);
	Transform::create().scale(0.5, 0.5, 1.0).projection()
		.apply(window, &logo);

	Drawable { triangle_a, triangle_b, button }
}

fn init2() -> DemoApp {
	// Load Resources - Images
	let icon = aci_png::decode(include_bytes!("res/logo.png")).unwrap();

	// Create Window
	let mut window = Window::new("Demo", icon);

	// Create Styles
//	let style_logo = Style::create().opaque(&mut window, icon);
//	let style_bear = Style::create().subtransparent(&mut window,
//		include_bytes!("res/plopgrizzly.ppm"), (0, 255, 0));

	let tex_logo = Texture::new(&mut window, aci_png::decode(
		include_bytes!("res/logo.png")
	).unwrap());

	DemoApp {
		drawable: resize(&mut window, tex_logo),
		window: window,
		running: true,
		gpu_data: GpuData { tex_logo },
	}
}

fn main() {
	let mut app = init2();
	let mut input_queue = InputQueue::new();

	while app.running {
		app.update(&mut input_queue);
	}
}
