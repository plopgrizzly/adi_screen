# Aldaron's Device Interface - Screen (adi_screen 0.3.0)
[Aldaron's Device Interface / Screen (adi_screen)](http://plopgrizzly.com/adi_screen)
is a library developed by [Plop Grizzly](http://plopgrizzly.com)
for interfacing with a computer screen or phone screen to render graphics.
It can be used for either making video games or applications.

[Cargo](https://crates.io/crates/adi_screen) /
[Documentation](https://docs.rs/adi_screen)

## Features
**adi_screen**'s current features:
* Create a window
* Render graphics with sprites
* Obtain user input
* Sprites auto depth-sort for fast rendering.
* Render text

**adi_screen**'s planned features:
* Fully functioning octree
* 2D Text (not affected by camera)

## Support
**adi_screen**'s current support:
* XCB + Vulkan, XCB Input, Linux Joystick Input
* WinAPI + Vulkan, WinAPI Input (except TouchPad),

**adi_screen**'s planned support:
* XCB + OpenGLES
* MacOS Window + OpenGL
* MacOS Window + Metal
* Android + OpenGLES
* Android + Vulkan
* WinAPI TouchPad
* WinAPI Joystick
* WinAPI Touchscreen
* XCB Touchscreen
* Wayland + OpenGLES
* Wayland + Vulkan
* Wayland Touchscreen
* WinAPI + OpenGL
* Raspberry Pi Direct To Display + Vulkan
* Web Assembly + WebGL

### To Use Vulkan on Windows
Download the vulkan runtime installer from
https://codeload.github.com/plopgrizzly/vulkan-runtime-installer/zip/master
Then, run VulkanRT-1.0.46.0-Installer.exe inside the downloaded zip file, and go
through the install process.

# Contributing

If you'd like to help implement functions for unsupported platforms, fix bugs,
improve the API or improve the Documentation, then contact me at
jeron.lau@plopgrizzly.com. I'll appreciate any help.