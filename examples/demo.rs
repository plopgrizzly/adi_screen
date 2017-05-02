/**
 * adi_screen - Aldaron's Device Interface - Screen - "examples/demo.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

extern crate adi_screen;

use adi_screen::Transform;
use adi_screen::{ Sprite, Screen, Texture };
use adi_screen::gui::Button;
use adi_screen::input::Input;
use adi_screen::{ SHADER_COLOR, SHADER_TEXTURE };

struct Context {
	screen: Screen,
	image: Sprite,
	triangle: Sprite,
	button: Button,
}

fn draw(context: &mut Context) {
	let disp = context.screen.full_smooth_pulse(8.0);
	let disp2 = context.screen.full_linear_pulse(4.0);

	Transform::create()
		.translate(-0.5, -0.5, 0.0)//5.0 * disp)
		.translate(disp * 1.0, 0.0, 0.0)
		.rotate(0.0, 0.0, disp)
		.perspective(90.0)
		.on(&mut context.screen, &context.triangle, 0);
	Transform::create()
		.translate(-0.5, 0.5, 0.0)//5.0 * disp)
		.translate(disp2 * 1.0, 0.0, 0.0)
		.perspective(90.0)
		.on(&mut context.screen, &context.triangle, 1);
	// Render onto the screen.
	context.screen.render((disp, 0.0, disp));
}

fn input(context: &mut Context, message: Input) {
	match message {
		Input::None => draw(context),
		Input::KeyDown(a) => println!("press {}", a),
		Input::KeyUp(a) => println!("release {}", a),
		Input::KeyRepeat(a) => println!("repeat {}", a),
		Input::Cursor(x, y) => println!("Cursor({}, {})", x, y),
		Input::Resize(w, h) => println!("Resize({}, {})", w, h),
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
		Input::Resume => println!("Resume ( Gain Focus )"),
		Input::Pause => println!("Pause ( Lose Focus )"),
		Input::Back => println!("Back"),
	};
	let pressed = context.button.get(&mut context.screen, message);
	if pressed {
		println!("button been pressed!");
	}
}

fn main() {
	// Vertices
	let v_triangle = [
		// Front Side
		-0.5,  0.5, 0., 1.0,	1.0, 0.0, 0.0, 1.0,
		 0.5,  0.5, 0., 1.0,	0.0, 1.0, 0.0, 1.0,
		 0.0, -0.5, 0., 1.0,	0.0, 0.0, 1.0, 1.0,

		// Back Side
		-0.5,  0.5, 0., 1.0,	1.0, 0.0, 0.0, 1.0,
		 0.0, -0.5, 0., 1.0,	0.0, 0.0, 1.0, 1.0,
		 0.5,  0.5, 0., 1.0,	0.0, 1.0, 0.0, 1.0,
	];
	let v_square = [
		-1.0, -1.0, 0.0, 1.0,	0.5, 0.5, 0.5, 0.5,
		1.0, 1.0, 0.0, 1.0,	0.5, 0.5, 0.5, 0.5,
		1.0, -1.0, 0.0, 1.0,	0.5, 0.5, 0.5, 0.5,

		1.0, 1.0, 0.0, 1.0,	0.5, 0.5, 0.5, 0.5,
		-1.0, -1.0, 0.0, 1.0,	0.5, 0.5, 0.5, 0.5,
		-1.0, 1.0, 0.0, 1.0,	0.5, 0.5, 0.5, 0.5,
	];
	let v_image = [
		-1.0, -1.0, 0.0, 1.0,	0.0, 0.0, 1.0, 1.0,
		-0.5, -0.5, 0.0, 1.0,	1.0, 1.0, 1.0, 1.0,
		-0.5, -1.0, 0.0, 1.0,	1.0, 0.0, 1.0, 1.0,

		-0.5, -0.5, 0.0, 1.0,	1.0, 1.0, 1.0, 1.0,
		-1.0, -1.0, 0.0, 1.0,	0.0, 0.0, 1.0, 1.0,
		-1.0, -0.5, 0.0, 1.0,	0.0, 1.0, 1.0, 1.0,
	];
	// Matrices
	let sm = Transform::create().scale(0.1, 0.1, 1.0).orthographic();
	let im = Transform::create().orthographic();
	let jm = Transform::create().scale(0.5, 0.5, 1.0).orthographic();
	// Open window
	let icon = include_bytes!("res/logo.ppm");
	let mut screen = Screen::create("Demo", icon, &[]);

	let texture = [
		Texture::opaque(&mut screen, include_bytes!("res/logo.ppm")),
		Texture::akeyed(&mut screen,
			include_bytes!("res/plopgrizzly.ppm"), (0, 255, 0))
	];
println!("TEX..");
	// Make sprites
	Sprite::textured(&mut screen, &v_image, SHADER_TEXTURE)
		.texcopy(&mut screen, &jm, &texture[0]);
	let mut context = Context {
		button: Button::create(&mut screen, (0.5, 0.5)),
		triangle: Sprite::colored(&mut screen, &v_triangle,
			SHADER_COLOR),
		image: Sprite::textured(&mut screen, &v_image, SHADER_TEXTURE),
		screen: screen,
//		time: Time::now(),
	};
	Sprite::colored(&mut context.screen, &v_square, SHADER_COLOR)
		.copy(&mut context.screen, &sm);
	context.triangle.copy(&mut context.screen, &im);
	context.triangle.copy(&mut context.screen, &im);

	context.image.texcopy(&mut context.screen, &im, &texture[1]);
	context.image.animate(&mut context.screen, 0, &texture[0]);

	loop {
		let message = Input::get(&mut context.screen);
		input(&mut context, message);
	}
}
