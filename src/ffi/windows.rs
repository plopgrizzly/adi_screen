/**
 * adi_screen - Aldaron's Device Interface - Screen - "ffi/windows.rs"
 * Copyright 2017 (c) Jeron Lau - Licensed under the MIT LICENSE
**/

use std::mem;
use std::{ usize, isize };

use input::Input;
use screen::{ Screen };
use screen::ffi::string;
use screen::image::Image;

use input::keyboard::{ english };
use input::Key;

use super::shared;

type DWord = u32; // 32-bit unsigned integer.
type Handle = usize; // Handle is a pointer.
type HInstance = Handle; // HInstance is a pointer.
type HWnd = Handle; // HWnd is a pointer to a window.
type HMenu = Handle;
type Atom = u16;

type HIcon = Handle;
type HBrush = Handle;
type HCursor = HIcon;

type UInt = u32;
type LResult = isize;
type WParam = usize;
type LParam = usize;

type Void = u8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NativeWindow {
	pub connection: HInstance,
	pub window: HWnd,
	border_y: isize,
	border_x: isize,
	border_w: isize,
	border_h: isize,
	miw: bool, // Mouse In Window
	non_fullscreen: Rect,
	fullscreen: bool,
}

#[repr(C)]
struct WndClassEx {
	cb_size: UInt,
	style: UInt,
	lpfn_wnd_proc: WndProc,
	cb_cls_extra: i32,
	cb_wnd_extra: i32,
	h_instance: HInstance,
	h_icon: HIcon,
	h_cursor: HCursor,
	hbr_background: HBrush,
	lpsz_menu_name: usize, // Char *
	lpsz_class_name: *const [u8;80],
	h_icon_sm: HIcon,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Rect {
	left: isize,
	top: isize,
	right: isize,
	bottom: isize,
}

#[repr(C)]
struct Point {
	x: isize, // long
	y: isize, // long
}

#[repr(C)]
struct MinMaxInfo {
	pt_reserved: Point,
	pt_max_size: Point,
	pt_max_position: Point,
	pt_min_track_size: Point,
	pt_max_track_size: Point,
}

#[repr(C)]
struct Msg {
	hwnd: HWnd,
	message: UInt,
	w_param: WParam,
	l_param: LParam,
	time: DWord,
	pt: Point,
}

type WndProc = extern "C" fn(h_wnd: HWnd, u_msg: UInt, w_param: WParam,
	l_param: *mut Void) -> LResult;

const WS_OVERLAPPEDWINDOW : DWord = 0x00C00000 | 0x00080000 | 0x00040000
	| 0x00010000 | 0x00020000;
const WS_VISIBLE : DWord = 0x10000000;
const WS_SYSMENU : DWord = 0x00080000;
const WS_FLAGS : DWord = WS_OVERLAPPEDWINDOW | WS_VISIBLE | WS_SYSMENU;

const MWW : isize = shared::MWW as isize;
const MWH : isize = shared::MWH as isize;
const MFW : Point = Point { x: MWW + 16, y: MWH + 39 };

static mut RESIZED : bool = false;
static mut DIM : Rect = Rect { left: 0, top: 0, right: MWW, bottom: MWH };
static mut PAUSED : bool = false;
static mut RESUMED : bool = false;

#[link(name = "gdi32")]
extern "system" {
	fn CreateWindowExW(a: DWord, class_name: *const [u8;80],
		window_name: *const [u8;80], style: DWord, x: i32, y: i32,
		w: i32, h: i32, parent: HWnd, menu: HMenu, hInstance: HInstance,
		param: Handle) -> HWnd;
	fn GetModuleHandleW(a: usize) -> HInstance;
	fn CreateIcon(hInstance: HInstance, w: i32, h: i32, planes: u8,
		bitspixel: u8, and: *const u8, xor: *const u8) -> HIcon;
	fn LoadCursorW(hInstance: HInstance, cursorName: usize) -> HCursor;
	fn DefWindowProcW(hWnd: HWnd, uMsg: UInt, wParam: WParam,
		lParam: *mut Void) -> LResult;
	fn GetStockObject(fnObject: i32) -> HBrush;
	fn RegisterClassExW(a: *const WndClassEx) -> Atom;
	fn AdjustWindowRect(a: *mut Rect, dwStyle: DWord, bMenu: i32) -> i32;
	fn PeekMessageW(lpMsg: *mut Msg, h_wnd: HWnd, msg_filter_min: u32,
		msg_filter_max: u32, remove_msg: u32) -> i32;
	fn TranslateMessage(lpMsg: *const Msg) -> i32;
	fn DispatchMessageW(lpMsg: *const Msg) -> usize;
	fn PostQuitMessage(exit_code: i32) -> ();
	fn GetWindowRect(h_wnd: HWnd, out: *mut Rect) -> i32;
	fn GetCursorPos(point: *mut Point) -> i32;
//	fn SetWindowLongPtrW(h_wnd: HWnd, n_index: i32, new: usize) -> usize; // 64-bit
	fn SetWindowLongW(h_wnd: HWnd, n_index: i32, new: usize) -> usize;
	fn SetWindowPos(h_wnd: HWnd, insert_after: HWnd, x: i32, y: i32, w: i32, h: i32, flags: u32) -> i32;
	fn GetSystemMetrics(index: i32) -> i32;
}

extern "C" fn wnd_proc(h_wnd: HWnd, u_msg: UInt, w_param: WParam,
	l_param: *mut Void) -> LResult
{
	match u_msg {
		0x0007 => unsafe { RESUMED = true },
		0x0008 => unsafe { PAUSED = true },
		0x0010 => {
			unsafe { PostQuitMessage(0) }; // Successful exit
			return 1; // TRUE = Don't Close Window Yet
		},
		0x0024 => {
			let lp = l_param as *mut _ as *mut MinMaxInfo;
			// Minimum Size
			unsafe {
				(*lp).pt_min_track_size = MFW;
			}
			// Get new dimensions
			unsafe {
				RESIZED = true;
				GetWindowRect(h_wnd, &mut DIM);
			}
			return 0;
		},
		0x0083 => {
			if w_param == 1 {
				let lp = l_param as *mut _ as *mut Rect;
				unsafe { DIM = *lp; }
			}
		},
		_ => {},
	}

	unsafe {
		DefWindowProcW(h_wnd, u_msg, w_param, l_param)
	}
}

pub fn toggle_fullscreen(window: &mut NativeWindow) {
	if window.fullscreen {
		unsafe {
			SetWindowLongW(window.window, -16, WS_FLAGS as usize);
			SetWindowPos(window.window, usize::MAX - 1,
				window.non_fullscreen.left as i32,
				window.non_fullscreen.top as i32,
				(window.non_fullscreen.right -
					window.non_fullscreen.left) as i32,
				(window.non_fullscreen.bottom -
					window.non_fullscreen.top) as i32,
				0x0040 | 0x0020);
		}
	} else {
		let w = unsafe { GetSystemMetrics(0) };
		let h = unsafe { GetSystemMetrics(1) };
		println!("w {} h {}", w, h);
		window.non_fullscreen = unsafe { DIM };
		unsafe {
			SetWindowLongW(window.window, -16, WS_VISIBLE as usize);
			SetWindowPos(window.window, usize::MAX, 0, 0, w, h,
				0x0040 | 0x0020);
		}
	}
	window.fullscreen = !window.fullscreen;
}

pub fn native_window(title: &str, icon:&'static [u8]) -> NativeWindow {
	let icon = Image::load(icon);

	let mut name : [u8; 80] = [0u8; 80];
	let nam = string::native(title);

	for i in 0..nam.len() {
		name[i] = nam[i];
	}
	let h_instance = unsafe { GetModuleHandleW(0) };

	let width = icon.size.0 as i32;
	let height = icon.size.1 as i32;

	let mut and : Vec<u8> = Vec::new();
	let mut xor : Vec<u8> = Vec::new();

	let w = icon.size.0 as usize;
	for i in 0usize..icon.size.0 as usize{
		for j in 0usize..icon.size.1 as usize {
			// Xor
			xor.push(icon.pixels[1 + 3 * (j + (w * i))]);
			xor.push(icon.pixels[0 + 3 * (j + (w * i))]);
			xor.push(icon.pixels[2 + 3 * (j + (w * i))]);
			xor.push(0xFF);
			// And
			and.push(0xFF);
			and.push(0xFF);
			and.push(0xFF);
			and.push(0xFF);
		}
	}

	let new_icon = unsafe {
		CreateIcon(h_instance, width, height, 1, 32, &and[0],
			&xor[0])
	};
	
	let window_class = WndClassEx {
		cb_size: mem::size_of::<WndClassEx>() as u32,
		style: 0x0002 | 0x0001,
		lpfn_wnd_proc: wnd_proc,
		cb_cls_extra: 0,
		cb_wnd_extra: 0,
		h_instance: h_instance,
		h_icon: new_icon,
		h_cursor: unsafe { LoadCursorW(0, 32512) },
		hbr_background: unsafe { GetStockObject(0) },
		lpsz_menu_name: 0,
		lpsz_class_name: &name,
		h_icon_sm: new_icon,
	};
	
	if unsafe { RegisterClassExW(&window_class) } == 0 {
		panic!("Failed to register windows class.");
	}

	println!("hah!");
	let mut wr = Rect { left: 0, top: 0, right: MWW, bottom: MWH };
	unsafe { AdjustWindowRect(&mut wr, WS_OVERLAPPEDWINDOW, 0) };
	let width = wr.right - wr.left;
	let height = wr.bottom - wr.top;
	let border : (isize, isize, isize, isize) = (width - MWW, height - MWH,
		wr.right - MWW, 0 - wr.top);
	println!("W: {} H: {}", width, height);
	println!("top:{},bottom:{},right:{},left:{}", wr.top, wr.bottom, wr.right, wr.left);
	let window = unsafe { CreateWindowExW(0,
		&name,		// class name
		&name,		// app name
		WS_FLAGS,	// window style
		0, 0,		// x/y coords
		width as i32,	// width
		height as i32,	// height
		0,		// handle to parent
		0,		// handle to menu
		h_instance,	// hInstance
		0)		// no extra parameters
	};
	if window == 0 {
		panic!("Couldn't Create a Window!");
	}

	NativeWindow { connection: h_instance, window: window,
		border_w: border.0, border_h: border.1, border_x: border.2,
		border_y: border.3, miw: true,
		non_fullscreen: Rect { left: 0,top: 0,right: MWW,bottom: MWH },
		fullscreen: false }
}

fn get_mouse(screen: &mut Screen) -> (f32, f32, bool) {
	let mut pos = Point { x: 0, y: 0 };
	unsafe { GetCursorPos(&mut pos); }

	let winx = screen.window.border_x + unsafe { DIM.left };
	let winy = screen.window.border_y + unsafe { DIM.top };
	
	let dim = ((pos.x - winx) as i16, (pos.y - winy) as i16);
	let pos = shared::convert_mouse_pos(&screen, dim);

	let miw = pos.0 >= -1.0 && pos.0 <= 1.0 && pos.1 >= -1.0
		&& pos.1 <= 1.0;

	let miw_changed = if screen.window.miw != miw {
		screen.window.miw = miw;
		true
	} else {
		false
	};
	(pos.0, pos.1, miw_changed)
}

fn convert_event(screen: &mut Screen, event_out: &mut Input) -> bool {
	let mut msg = Msg { hwnd: 0, message: 0, w_param: 0, l_param: 0, time: 0,
		pt: Point { x: 0, y: 0 } };
		
	if unsafe { RESIZED } {
		let w = unsafe { DIM.right - DIM.left } - screen.window.border_w;
		let h = unsafe { DIM.bottom - DIM.top } - screen.window.border_h;

		if shared::should_resize(screen, (w as u32, h as u32)) {
			*event_out = Input::Resize;
		}
		unsafe { RESIZED = false };
		return false; // Send Event
	}
	
	if unsafe { PAUSED } {
		*event_out = Input::Pause;
		unsafe { PAUSED = false };
		return false; // Send Event
	}
	
	if unsafe { RESUMED } {
		*event_out = Input::Resume;
		unsafe { RESUMED = false };
		return false; // Send Event
	}

	let (x, y, miw_changed) = get_mouse(screen);
	if miw_changed {
		*event_out = if screen.window.miw {
			Input::EnterWindow
		} else {
			Input::LeaveWindow
		};
		return false; // Send Event
	}

	if unsafe {
		PeekMessageW(&mut msg, 0, 0, 0, 0x0001)
	} == 0 { // no messages available
		*event_out = Input::None;
		return false;
	}
	*event_out = match msg.message {
		0x0012 => Input::Back, // Quit
		0x0200 => Input::Cursor(x,y), // WM_MOUSEMOVE
		0x0201 => Input::LeftDown(x,y), // WM_LBUTTONDOWN
		0x0202 => Input::LeftUp(x,y), // WM_LBUTTONUP
		0x0207 => Input::MiddleDown(x,y),
		0x0208 => Input::MiddleUp(x, y),
		0x0204 => Input::RightDown(x, y),
		0x0205 => Input::RightUp(x, y),
		0x0100 | 0x0104 => {
			let scan = ((msg.l_param
				& 0b00000001_11111111_00000000_00000000) >> 16)
				as u16;
			let chr = english(msg.w_param as u16, scan);
			if msg.l_param & 0b01000000_00000000_00000000_00000000
				!= 0
			{
				match chr {
					// These keys shouldn't repeat.
					Key::Escape | Key::F(_) | Key::Insert |
						Key::CapsLock | Key::NumLock |
						Key::Shift(_) | Key::Ctrl(_) |
						Key::Alt(_)
					=> {
						return true;
					}
					_ => Input::KeyRepeat(chr)
				}
			} else {
				Input::KeyDown(chr)
			}
		}
		0x0101 | 0x0105 => {
			let scan = ((msg.l_param
				& 0b00000001_11111111_00000000_00000000) >> 16)
				as u16;
			Input::KeyUp(english(msg.w_param as u16, scan))
		}
		0x020A => {
			// TODO: Make a 240 count as 2 scrolls and 360 as 3.
			let a = (((msg.w_param as u32) >> 16) & 0xFFFF) as i16;
			if a > 0 {
				Input::ScrollUp(x, y)
			} else {
				Input::ScrollDown(x, y)
			}
		},
		_ => {
			unsafe {
				TranslateMessage(&msg);
				DispatchMessageW(&msg);
			}
//			println!("Unknown event: {0:x}", x);
			return true; // ignore
		}
	};
	false
}

pub fn running(screen: &mut Screen) -> Input {
	let mut converted = Input::None;

	while convert_event(screen, &mut converted) {}	
	converted
}

pub fn cleanup(_: &mut NativeWindow) { }
