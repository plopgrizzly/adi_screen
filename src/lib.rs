// Aldaron's Device Interface / Screen
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/lib.rs

//! Aldaron's Device Interface / Screen is a library developed by Plop Grizzly
//! for interfacing with a computer screen or phone screen to render graphics.

#![warn(missing_docs)]
#![doc(html_logo_url = "http://plopgrizzly.com/adi_screen/icon.png",
	html_favicon_url = "http://plopgrizzly.com/adi_screen/icon.ico",
	html_root_url = "http://plopgrizzly.com/adi_screen/")]

mod window;
mod sprite;
mod gui;
mod texture;
mod gpu_data;

pub use window::{ Window };
pub use sprite::{ Sprite, SpriteBuilder, SpriteList, Transform };
pub use gui::Button as GuiButton;
pub use gui::{ Text, DEFAULT_FONT };
pub use texture::Texture;
pub use gpu_data::{ Model, Gradient, TexCoords };

extern crate afi;
extern crate ami;
extern crate adi_gpu;
extern crate aci_png;
extern crate rusttype;
extern crate awi;

pub extern crate adi_clock;

pub use awi::Input;
pub use awi::Key;
pub use awi::Button;
pub use awi::ControllerManager;
pub use awi::Msg;
pub use awi::Click;
