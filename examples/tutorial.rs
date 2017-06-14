extern crate adi_screen;
extern crate aci_ppm;

use adi_screen::{ Window, Input, Sprite, Style, Transform, Key };

struct Player {
	sprite: Sprite,
	pos: (f32, f32),
	up: bool,
	down: bool,
	right: bool,
	left: bool,
}

impl Player {
	fn update(&mut self, window: &mut Window, input: Input) {
		let mut resize = false;
		match input {
			Input::KeyDown(Key::Up) => self.up = true,
			Input::KeyDown(Key::Down) => self.down = true,
			Input::KeyDown(Key::Left) => self.left = true,
			Input::KeyDown(Key::Right) => self.right = true,
			Input::KeyUp(Key::Up) => { self.up = false; },
			Input::KeyUp(Key::Down) => { self.down = false; },
			Input::KeyUp(Key::Left) => { self.left = false; },
			Input::KeyUp(Key::Right) => { self.right = false; },
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

fn main() {
	let mut window = Window::create("project_name",
		aci_ppm::decode(include_bytes!("res/logo.ppm")).unwrap(), &[]);
	let style = Style::create().subtransparent(&mut window,
		include_bytes!("res/logo.ppm"), (0, 0, 0));
	let sprite = Sprite::create(&mut window, &include!("res/sprite.data"),
		style, 1);

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

		let queue = context.window.update();

		for input in queue {
			match input {
				Input::Back => break 'mainloop,
				_ => {},
			}

			// Run the update functions
			context.player.update(&mut context.window, input);
		}
	}
}
