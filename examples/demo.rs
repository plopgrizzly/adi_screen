/**
 * adi_screen - Aldaron's Device Interface - Screen - "examples/demo.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

extern crate adi_screen;

use adi_screen::Transform;
use adi_screen::{ Sprite, Screen, Style };
use adi_screen::gui::Button;
use adi_screen::input::Input;

struct Context {
	screen: Screen,
	image: Sprite,
	triangle: Sprite,
	logo: Sprite,
	square: Sprite,
	button: Button,
}

fn draw(context: &mut Context) {
	let disp = context.screen.full_smooth_pulse(8.0);
	let disp2 = context.screen.full_linear_pulse(4.0);

	Transform::create()
		.translate(-0.5, -0.5, 0.0)//5.0 * disp)
		.translate(disp * 1.0, 0.0, 0.0)
		.rotate(0.0, 0.0, disp)
		.perspective(&context.screen, 90.0)
		.on(&mut context.screen, &context.triangle, 0);
	Transform::create()
		.translate(-0.5, 0.5, 0.0)//5.0 * disp)
		.translate(disp2 * 1.0, 0.0, 0.0)
		.perspective(&context.screen, 90.0)
		.on(&mut context.screen, &context.triangle, 1);
	// Render onto the screen.
	context.screen.render((disp, 0.0, disp));
}

fn resize(context: &mut Context) {
	Transform::create().scale(0.1, 0.1, 1.0)
		.orthographic(&context.screen)
		.on(&mut context.screen, &context.square, 0);
	Transform::create().orthographic(&context.screen)
		.on(&mut context.screen, &context.image, 0);
	Transform::create().scale(0.5, 0.5, 1.0)
		.orthographic(&context.screen)
		.on(&mut context.screen, &context.logo, 0);
}

fn input(context: &mut Context) {
	let message = Input::get(&mut context.screen);

	match message {
		Input::None => draw(context),
		Input::Resize => resize(context),
		Input::Back => println!("Back"),
		Input::Resume => println!("Resume ( Gain Focus )"),
		Input::Pause => println!("Pause ( Lose Focus )"),
		Input::KeyDown(a) => println!("press {}", a),
		Input::KeyUp(a) => println!("release {}", a),
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
	};
	let pressed = context.button.get(&mut context.screen, message);
	if pressed {
		println!("button been pressed!");
	}
}

fn init() -> (Screen, Style) {
	// Load Resources - Images
	let image_logo = include_bytes!("res/logo.ppm");

	// Open window
	let mut screen = Screen::create("Demo", image_logo, &[]);

	// Create Textures
	let style_logo = Style::create().opaque(&mut screen, image_logo);

	(screen, style_logo)
}

fn init2() -> Context {
	let (mut screen, style_logo) = init();

	// Create Styles
	let style_solid = Style::create().solid();
	let style_bear = Style::create().subtransparent(&mut screen,
		include_bytes!("res/plopgrizzly.ppm"), (0, 255, 0));

	// Create Sprites
	let shape_image = include!("res/image.data");
	Context {
		logo: Sprite::create(&mut screen, &shape_image, style_logo, 1),
		button: Button::create(&mut screen, (-1.0, -1.0)),
		triangle: Sprite::create(&mut screen,
			&include!("res/triangle.data"), style_solid, 2),
		image: {
			let image = Sprite::create(&mut screen, &shape_image,
				style_logo, 1);
			image.animate(&mut screen, 0, &style_bear);
			image
		},
		square: Sprite::create(&mut screen,
			&include!("res/square.data"), style_solid, 1),
		screen: screen,
	}
}

fn main() {
	let mut context = init2();

	loop {
		input(&mut context);
	}
}
