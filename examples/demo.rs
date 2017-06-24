/**
 * adi_screen - Aldaron's Device Interface - Screen - "examples/demo.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

extern crate adi_screen;
extern crate aci_ppm;

use adi_screen::{
	Transform,
	Sprite,
	Window,
	Style,
	Input,
	GuiButton,
	Msg,
	InputQueue
};

struct DemoApp {
	window: Window,
	image: Sprite,
	triangle: Sprite,
	logo: Sprite,
	square: Sprite,
	button: GuiButton,
	running: bool,
}

impl DemoApp {
	fn animate(&mut self) {
		let disp = self.window.pulse_full_smooth(8.0);
		let disp2 = self.window.pulse_full_linear(4.0);

		Transform::create()
			.translate(-0.5, -0.5, 0.0)//5.0 * disp)
			.translate(disp * 1.0, 0.0, 0.0)
			.rotate(0.0, 0.0, disp)
			.perspective(&self.window, 90.0)
			.apply(&mut self.window, &self.triangle, 0);
		Transform::create()
			.translate(-0.5, 0.5, 0.0)//5.0 * disp)
			.translate(disp2 * 1.0, 0.0, 0.0)
			.perspective(&self.window, 90.0)
			.apply(&mut self.window, &self.triangle, 1);

		self.window.background(disp, 0.0, disp);
	}

	fn input(&mut self, input: Input) {
		use Input::*;
		use Msg::*;

		match input {
			Msg(Quit) | Msg(Back) => self.running = false,
			Resize => resize(self),
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
		};
		let pressed = self.button.update(&mut self.window, input);
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

fn resize(context: &mut DemoApp) {
	Transform::create().scale(0.1, 0.1, 1.0)
		.orthographic(&context.window)
		.apply(&mut context.window, &context.square, 0);
	Transform::create().orthographic(&context.window)
		.apply(&mut context.window, &context.image, 0);
	Transform::create().scale(0.5, 0.5, 1.0)
		.orthographic(&context.window)
		.apply(&mut context.window, &context.logo, 0);
}

fn init2() -> DemoApp {
	// Load Resources - Images
	let icon = aci_ppm::decode(include_bytes!("res/logo.ppm")).unwrap();
	let image_logo = include_bytes!("res/logo.ppm");

	// Create Window
	let mut window = Window::create("Demo", icon, &[]);

	// Create Styles
	let style_logo = Style::create().opaque(&mut window, image_logo);
	let style_gradient = Style::create().gradient();
	let style_bear = Style::create().subtransparent(&mut window,
		include_bytes!("res/plopgrizzly.ppm"), (0, 255, 0));

	// Create Sprites
	let shape_image = include!("res/image.data");
	DemoApp {
		logo: Sprite::create(&mut window, &shape_image, style_logo, 1),
		button: GuiButton::create(&mut window, (-1.0, -1.0)),
		triangle: Sprite::create(&mut window,
			&include!("res/triangle.data"), style_gradient, 2),
		image: {
			let image = Sprite::create(&mut window, &shape_image,
				style_logo, 1);
			image.style(&mut window, 0, &style_bear);
			image
		},
		square: Sprite::create(&mut window,
			&include!("res/square.data"), style_gradient, 1),
		window: window,
		running: true,
	}
}

fn main() {
	let mut app = init2();
	let mut input_queue = InputQueue::create();

	while app.running {
		app.update(&mut input_queue);
	}
}
