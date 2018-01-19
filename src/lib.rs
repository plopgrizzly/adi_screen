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

pub extern crate adi_clock;

pub use adi_gpu::window::Input;
pub use adi_gpu::window::Key;
pub use adi_gpu::window::Button;
pub use adi_gpu::window::Joystick;
pub use adi_gpu::window::Msg;
pub use adi_gpu::window::Click;
