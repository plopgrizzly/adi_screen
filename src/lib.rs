/**
 * adi_screen - Aldaron's Device Interface - Screen - "lib.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

/// The version of adi_screen that's being used
pub const VERSION : &'static str = "adi_screen 0.1.0";

mod image;
mod renderer;
mod input;

mod window;
mod sprite;
mod style;
mod gui;

pub use input::keyboard::Key;
pub use input::joystick::Button;
pub use input::{ Input };
pub use window::{ Window };
pub use sprite::{ Sprite, Transform };
pub use style::Style;
pub use gui::Button as GuiButton;

pub extern crate adi_clock;
