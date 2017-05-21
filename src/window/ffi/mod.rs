/**
 * adi_screen - Aldaron's Device Interface - Screen - "ffi/mod.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::NativeWindow;

// TODO: Add wayland support
//#[cfg(target_os = "linux")]
//mod wayland;
//#[cfg(target_os = "linux")]
//pub use self::wayland::NativeWindow;

#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "android")]
pub use self::android::NativeWindow;

#[cfg(target_os = "ios")]
mod ios;
#[cfg(target_os = "ios")]
pub use self::ios::NativeWindow;

// MacOS, Linux and all other OS's that might support XCB.
#[cfg(not(any(target_os = "windows",target_os = "android",target_os = "ios")))]
mod xcb;
#[cfg(not(any(target_os = "windows",target_os = "android",target_os = "ios")))]
pub use self::xcb::NativeWindow;

// mod aldaros;
// mod arduino;
// mod nintendo_switch;
// mod raspberry_pi;
// mod web_assembly;
// mod no_os;

// // // // // // // // // // // // // // // // // // // // // // // // // // //

// Platform Independant

use Input;

type LazyPointer = usize;

const MWW : u32 = 640;
const MWH : u32 = 360;

//pub struct WindowFFI {
//	poll_event: fn(window: &mut Window) -> bool,
//}

//#[cfg(target_os = "linux")]
//pub fn load_windowffi() -> WindowFFI {
//	// Try loading XCB
//	WindowFFI {
//		poll_event: xcb::poll_event,
//	}
//}

pub fn poll_events(window: &mut NativeWindow,
	wh: &mut (u32, u32)/*, windowffi: &WindowFFI*/) -> Vec<Input>
{
	let mut input = Vec::new();

	while window.poll_event(&mut input, wh) { }

	input.push(Input::Redraw);

	input
}

fn convert_mouse_pos(wh: &(u32, u32), c: (i16, i16)) -> (f32,f32) {
	let x = ((c.0 as f32) / (wh.0 as f32) * 2.0) - 1.0;
	let y = ((c.1 as f32) / (wh.1 as f32) * 2.0) - 1.0;
	(x, y)
}

fn should_resize(wh: &mut (u32, u32), d: (u32, u32)) -> bool {
	if *wh != d {
		*wh = d;
		true
	}else{
		// ignore, didn't actually resize.
		false
	}
}
