// Aldaron's Device Interface / Screen
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/lib.rs

mod window;
mod sprite;
mod gui;
mod texture;
mod gpu_data;

pub use window::{ Window };
pub use sprite::{ Sprite, SpriteBuilder, Transform };
pub use gui::Button as GuiButton;
pub use texture::Texture;
pub use gpu_data::{ Model, Gradient, TexCoords };

extern crate afi;
extern crate ami;
extern crate awi;
extern crate adi_gpu;
extern crate aci_png;

pub extern crate adi_clock;

pub use awi::Input;
pub use awi::Key;
pub use awi::Button;
pub use awi::Joystick;
pub use awi::Msg;
pub use awi::Click;
