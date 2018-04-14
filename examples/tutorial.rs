// tutorial.rs -- Aldaron's Device Interface / Screen
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

extern crate adi_screen;
extern crate aci_png;

use adi_screen::{ Window, Input, Sprite, Transform, Key, Msg, InputQueue,
	Texture, SpriteBuilder };

struct Player {
	sprite: Sprite,
	pos: (f32, f32),
	up: bool,
	down: bool,
	right: bool,
	left: bool,
}

impl Player {
	fn update(&mut self, window: &mut Window, input: &Input) {
		let mut resize = false;
		match *input {
			Input::KeyPress(Key::Up) => self.up = true,
			Input::KeyPress(Key::Down) => self.down = true,
			Input::KeyPress(Key::Left) => self.left = true,
			Input::KeyPress(Key::Right) => self.right = true,
			Input::KeyRelease(Key::Up) => { self.up = false; },
			Input::KeyRelease(Key::Down) => { self.down = false; },
			Input::KeyRelease(Key::Left) => { self.left = false; },
			Input::KeyRelease(Key::Right) => { self.right = false; },
			Input::Resize => resize = true,
			_ => {},
		}
		if self.up {
			self.pos.1 -= 0.005;
		}
		if self.down {
			self.pos.1 += 0.005;
		}
		if self.right {
			self.pos.0 += 0.005;
		}
		if self.left {
			self.pos.0 -= 0.005;
		}
		if !self.up && !self.down && !self.right && !self.left
			&& !resize
		{
			return;
		}
		Transform::create()
			.translate(self.pos.0, self.pos.1, 0.0)
			.perspective(window, 90.0)
			.apply(window, &self.sprite, 0);
	}
}

struct Context {
	window: Window,
	player: Player,
}

fn redraw(context: &mut Context) {
	let disp2 = context.window.pulse_full_linear(2.0);

	context.window.background(disp2, disp2, disp2);
}

fn resize(window: &mut Window, tex_logo: Texture) -> Sprite {
	SpriteBuilder::new(&include!("res/sprite.data")).texture(
		window, tex_logo, &include!("res/sprite.texc"))
}

fn main() {
	let mut window = Window::create("project_name",
		&aci_png::decode(include_bytes!("res/logo.png")).unwrap(),
		(0.25, 0.25, 1.0), (20.0, 10.0));
	let mut queue = InputQueue::new();

	let tex_logo = Texture::new(&mut window, aci_png::decode(
		include_bytes!("res/logo.png")
	).unwrap().as_slice());

	let sprite = resize(&mut window, tex_logo);

	let mut context = Context {
		player: Player {
			sprite: sprite,
			pos: (0.0, 0.0),
			up: false, down: false, left: false, right: false,
		},
		window: window,
	};

	'mainloop: loop {
		redraw(&mut context);

		context.window.update(&mut queue);

		for input in queue.iter() {
			match *input {
				Input::Msg(Msg::Back) | Input::Msg(Msg::Quit)
					=> break 'mainloop,
				Input::Resize => context.player.sprite
					= resize(&mut context.window, tex_logo),
				_ => {},
			}

			// Run the update functions
			context.player.update(&mut context.window, input);
		}
	}
}
