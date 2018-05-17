// "adi_screen" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

mod button;
mod text;
pub use self::button::Button;
pub use self::text::Font;
pub(crate) use self::text::DEFAULT_FONT;
pub use self::text::Text;

use Texture;

// A GUI Widget
pub enum Widget<'a> {
	/// An empty widget.
	None,
	/// A widget that contains another widget (left, right, up, down margin)
	Container(&'a Widget<'a>, usize, usize, usize, usize),
	/// A widget that can be clicked, closure executes on button release
	Button(&'a Widget<'a>, &'a Fn() -> ()),
	/// A widget that displays text
	Text(&'a str),
	/// A widget that displays an image
	Image(&'a Texture),
	/// A widget that displays an icon (save, open, hamburger menu, etc.)
	Icon(&'a usize), // TODO: change type to actually be an icon
	/// A horizontal list container across the top of it's container.
	MenuBar(&'a Vec<&'a Widget<'a>>),
	/// A vertical list container on the left side of it's container.
	SideBar(&'a Vec<&'a Widget<'a>>),
	/// A horizontal list container across the bottom of it's container.
	InfoBar(&'a Vec<&'a Widget<'a>>),
	/// A vertical list container on the right side of it's container.
	DataBar(&'a Vec<&'a Widget<'a>>),
	/// A list of switchable tabs.
	Tabs(&'a Vec<&'a str>, &'a Fn(usize) -> ()),
	/// A pop-up menu
	PopUp(&'a Widget<'a>, &'a Fn(usize) -> ()),
	/// An editable text field
	TextField(&'a str, &'a usize),
}

// A Graphical User Interface
pub struct Gui<'a>(Widget<'a>);

impl<'a> Gui<'a> {
	/// A new GUI.
	pub fn new(widget: Widget<'a>) -> Self {
		Gui(widget)
	}
}
