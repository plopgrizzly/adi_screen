/**
 * adi_screen - Aldaron's Device Interface - Screen - "lib.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

/// The version of adi_screen that's being used
pub const VERSION : &'static str = "adi_screen 0.1.0";

mod screen;
mod sprite;

pub use screen::Screen;
pub use screen::SHADER_COLOR;
pub use screen::SHADER_TEXTURE;
pub use screen::SHADER_CUSTOM;
pub use sprite::Sprite;
pub use sprite::Transform;

pub extern crate adi_clock;

pub mod input;
pub mod gui;

mod ffi; // Native window module
pub use self::ffi::{ running };

mod image;

#[link(name = "vulkan-1")]
mod vw;
pub use self::vw::{ Texture, Style, Shader };
