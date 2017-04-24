/**
 * Aldaron's Device Interface - "demo.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the GNU GENERAL PUBLIC LICENSE
**/

extern crate adi_screen;

use adi_screen::transforms::{ Matrix };
use adi_screen::{ Sprite, Screen, Texture };
use adi_screen::gui::{ Button };
use adi_screen::input::Input;

struct SpriteContext { }

struct Context {
	screen: Screen,
	image: Sprite<SpriteContext>,
	triangle: Sprite<SpriteContext>,
	button: Sprite<Button>,
}

fn draw(context: &mut Context) {
	let disp = context.screen.full_smooth_pulse(4.0);
	let disp2 = context.screen.full_linear_pulse(4.0);
	let matrix = Matrix::identity()
//		.rotate(disp, 0.0, 0.0)
		.translate(-0.5, -0.5, 0.0)//5.0 * disp)
		.translate(disp * 1.0, 0.0, 0.0)
		.set_perspective(90.0);

	let matrix2 = Matrix::identity()
		.translate(-0.5, 0.5, 0.0)//5.0 * disp)
		.translate(disp2 * 1.0, 0.0, 0.0)
		.set_perspective(90.0);

	context.triangle.matrix(&mut context.screen, 0, &matrix);
	context.triangle.matrix(&mut context.screen, 1, &matrix2);
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
	context.button.run(&mut context.screen, message);
}

fn logo_input(_: &mut Screen, _: &mut Sprite<SpriteContext>, _: usize, _: Input)
	-> isize
{
	-1
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
	let sm = Matrix::identity().scale(0.1, 0.1, 1.0);
	let im = Matrix::identity();
	let jm = Matrix::identity().scale(0.5, 0.5, 1.0);
	// Open window
	let (mut screen, styles) = Screen::new("Demo",
		include_bytes!("res/logo.ppm"), &[]);

	let texture = [
		Texture::opaque(&mut screen, include_bytes!("res/logo.ppm")),
		Texture::akeyed(&mut screen,
			include_bytes!("res/plopgrizzly.ppm"), (0, 255, 0))
	];
println!("TEX..");
	// Make sprites
	Sprite::textured(&mut screen, &v_image, &styles[1], logo_input)
		.texcopy(&mut screen, &jm, &texture[0], SpriteContext {});
	let mut context = Context {
		button: Button::add(&mut screen, &styles[1], (0.5, 0.5)),
		triangle: Sprite::colored(&mut screen, &v_triangle,
			&styles[0], logo_input),
		image: Sprite::textured(&mut screen, &v_image, &styles[1],
			logo_input),
		screen: screen,
//		time: Time::now(),
	};
	Sprite::colored(&mut context.screen, &v_square, &styles[0], logo_input)
		.copy(&mut context.screen, &sm, SpriteContext {});
	context.triangle.copy(&mut context.screen, &im, SpriteContext {});
	context.triangle.copy(&mut context.screen, &im, SpriteContext {});

	context.image.texcopy(&mut context.screen, &im, &texture[1], SpriteContext {});
	context.image.animate(&mut context.screen, 0, &texture[0], &im);

	loop {
		let message = Input::get(&mut context.screen);
		input(&mut context, message);
	}
}
