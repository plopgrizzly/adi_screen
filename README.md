# adi_screen
Portable rust library for rendering onto a computer screen or phone screen.  Can
be used for video games or applications.

### To Use Vulkan on Windows
Download the vulkan runtime installer from
https://codeload.github.com/plopgrizzly/vulkan-runtime-installer/zip/master
Then, run VulkanRT-1.0.46.0-Installer.exe inside the downloaded zip file, and go
through the install process.

## support
Aldaron's Device Interface / Screen supports:
* XCB + Vulkan, XCB Input, Linux Joystick Input
* WinAPI + Vulkan, WinAPI Input (except TouchPad),

Aldaron's Device Interface / Screen will support:
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

## Features
Aldaron's Device Interface / Screen can:
* Create a window
* Render graphics with sprites
* Obtain user input

Aldaron's Device Interface / Screen will be able to:
* Give sprites auto depth-sort.
* Render text
* Use only Rust, no C code