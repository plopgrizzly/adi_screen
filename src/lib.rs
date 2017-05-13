/**
 * adi_screen - Aldaron's Device Interface - Screen - "lib.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

/// The version of adi_screen that's being used
pub const VERSION : &'static str = "adi_screen 0.1.0";

mod ffi;
mod image;
mod vw;

mod window;
mod sprite;
mod style;
mod input;
mod gui;

pub use window::Window;
pub use sprite::{ Sprite, Transform };
pub use style::Style;
pub use input::{ Input, Key };
pub use gui::Button;

pub extern crate adi_clock;
