/**
 * adi_screen - Aldaron's Device Interface - Screen - "lib.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

/// The version of adi_screen that's being used
pub const VERSION : &'static str = "adi_screen 0.1.0";

mod window;
mod sprite;
mod style;

pub use window::Window;
pub use sprite::Sprite;
pub use sprite::Transform;
pub use style::Style;

pub extern crate adi_clock;

pub mod input;
pub mod gui;

mod ffi; // Native window module
pub use self::ffi::{ running };

mod image;

#[link(name = "vulkan-1")]
mod vw;
pub use self::vw::{ Shader };
