/**
 * adi_screen - Aldaron's Device Interface
 * Screen - "ffi/xcb/create_screen.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use super::LazyPointer;

#[repr(C)]
pub struct XcbScreen {
	root: u32,
	default_colormap: u32,
	white_pixel: u32,
	black_pixel: u32,
	current_input_masks: u32,
	width_in_pixels: u16,
	height_in_pixels: u16,
	width_in_millimeters: u16,
	height_in_millimeters: u16,
	min_installed_maps: u16,
	max_installed_maps: u16,
	root_visual: u32,
	backing_stores: u8,
	save_unders: u8,
	root_depth: u8,
	allowed_depths_len: u8,
}

#[repr(C)]
struct XcbScreenIterator {
	data: *mut XcbScreen,
	rem: i32,
	index: i32,
}

extern {
	fn xcb_get_setup(c: LazyPointer) -> LazyPointer;
	fn xcb_setup_roots_iterator(setup: LazyPointer) -> XcbScreenIterator;
}

pub fn create_screen(connection: LazyPointer) -> (u32, u32, u32) {
	let setup = unsafe { xcb_get_setup(connection) };
	let screen = unsafe { xcb_setup_roots_iterator(setup).data };
	let root = unsafe { (*screen).root };
	let visual = unsafe { (*screen).root_visual };
	let black = unsafe { (*screen).black_pixel };

	(root, visual, black)
}
