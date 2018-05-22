// "adi_screen" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
//
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

pub use window::{ WindowBuilder, Window };
pub use sprite::{ Sprite, Transform };
pub use gui::{ Text };
pub use texture::Texture;
pub use gpu_data::{ Model, ModelBuilder };

extern crate ami;
extern crate adi_gpu;
extern crate aci_png;
extern crate fonterator;

pub extern crate adi_clock;

pub use adi_gpu::{ afi, Input, Key, Click, Msg };
pub use ami::{ Mat4, IDENTITY };
