/**
 * adi_screen - Aldaron's Device Interface - Screen - "lib.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

/// The version of adi_screen that's being used
pub const VERSION : &'static str = "adi_screen 0.2.2";

mod image;
mod renderer;

mod window;
mod sprite;
mod style;
mod gui;

pub use window::{ Window };
pub use sprite::{ Sprite, Transform };
pub use style::Style;
pub use gui::Button as GuiButton;

extern crate ami;
extern crate awi;

pub extern crate adi_clock;

use awi::Window as AwiWindow;
use awi::WindowConnection as AwiConnection;

pub use awi::Input;
pub use awi::Key;
pub use awi::Button;
pub use awi::Joystick;
pub use awi::Msg;
pub use awi::Click;
pub use awi::InputQueue;
