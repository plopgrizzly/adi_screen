/**
 * adi_screen - Aldaron's Device Interface - Screen - "examples/demo.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

extern crate adi_screen;
extern crate aci_ppm;

use adi_screen::{ Transform, Sprite, Window, Style, Input, GuiButton };

struct Context {
	window: Window,
	image: Sprite,
	triangle: Sprite,
	logo: Sprite,
	square: Sprite,
	button: GuiButton,
}

fn redraw(context: &mut Context) {
	let disp = context.window.pulse_full_smooth(8.0);
	let disp2 = context.window.pulse_full_linear(4.0);

	Transform::create()
		.translate(-0.5, -0.5, 0.0)//5.0 * disp)
		.translate(disp * 1.0, 0.0, 0.0)
		.rotate(0.0, 0.0, disp)
		.perspective(&context.window, 90.0)
		.apply(&mut context.window, &context.triangle, 0);
	Transform::create()
		.translate(-0.5, 0.5, 0.0)//5.0 * disp)
		.translate(disp2 * 1.0, 0.0, 0.0)
		.perspective(&context.window, 90.0)
		.apply(&mut context.window, &context.triangle, 1);

	context.window.background(disp, 0.0, disp);
}

fn resize(context: &mut Context) {
	Transform::create().scale(0.1, 0.1, 1.0)
		.orthographic(&context.window)
		.apply(&mut context.window, &context.square, 0);
	Transform::create().orthographic(&context.window)
		.apply(&mut context.window, &context.image, 0);
	Transform::create().scale(0.5, 0.5, 1.0)
		.orthographic(&context.window)
		.apply(&mut context.window, &context.logo, 0);
}

fn update(context: &mut Context, message: Input) -> bool {
	match message {
		Input::Redraw => redraw(context),
		Input::Resize => resize(context),
		Input::Back => return true, // Quit
		Input::Resume => println!("Resume ( Gain Focus )"),
		Input::Pause => println!("Pause ( Lose Focus )"),
		Input::KeyPress(a) => println!("press {}", a),
		Input::KeyRelease(a) => println!("release {}", a),
		Input::KeyRepeat(a) => println!("repeat {}", a),
		Input::Cursor(x, y) => println!("Cursor({}, {})", x, y),
		Input::LeftDown(x, y) => println!("Left Down ({}, {})", x, y),
		Input::LeftUp(x, y) => println!("Left Up ({}, {})", x, y),
		Input::MiddleDown(x, y) => println!("Middle Down ({}, {})", x, y),
		Input::MiddleUp(x, y) => println!("Middle Up ({}, {})", x, y),
		Input::RightDown(x, y) => println!("Right Down ({}, {})", x, y),
		Input::RightUp(x, y) => println!("Right Up ({}, {})", x, y),
		Input::ScrollUp(x, y) => println!("Scroll Up ({}, {})", x, y),
		Input::ScrollDown(x, y) => println!("Scroll Down ({}, {})", x, y),
		Input::ScrollRight(x, y) => println!("Scroll Right ({}, {})", x, y),
		Input::ScrollLeft(x, y) => println!("Scroll Left ({}, {})", x, y),
		Input::EnterWindow => println!("Enter Window"),
		Input::LeaveWindow => println!("Leave Window"),
		Input::JoystickMove(x, y) => println!("Joystick ({}, {})", x, y),
		Input::JoystickPov(x, y) => println!("POV Hat ({}, {})", x, y),
		Input::JoystickThrottle(x) => println!("Throttle ({})", x),
		Input::JoystickButtonDown(a) => println!("Button Down ({})", a),
		Input::JoystickButtonUp(a) =>  println!("Button Up ({})", a),
		Input::Text(a) => println!("Text: {}", a),
		Input::Msg(_) => println!("Message"),
//		Input::Msg(a) => println!("Message: {}", a),
	};
	let pressed = context.button.update(&mut context.window, message);
	if pressed {
		println!("button been pressed!");
	}
	false
}

fn init2() -> Context {
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
	Context {
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
	}
}

fn main() {
	let mut context = init2();

	'mainloop: loop {
		for message in context.window.update() {
			if update(&mut context, message) {
				break 'mainloop;
			}
		}
	}
}
