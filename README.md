# adi_screen
Portable rust library for rendering onto a computer screen or phone screen.  Can
be used for video games or applications.

### support

| Platform              | Vulkan | OpenGL | Metal | Window | Touch | Touchpad | Mouse | Keyboard | Joystick |
|-----------------------|--------|--------|-------|--------|-------|----------|-------|----------|----------|
| XCB on Linux          | Yes    | No     | No    | Yes    | No    | Yes      | Yes   | Yes      | Yes      |
| Windows               | Yes    | No     | No    | Yes    | No    | No       | Yes   | Yes      | No       |

# next planned release

## adi_screen 0.2.1
* Give sprites auto depth-sort.

# current releases

## adi_screen 0.2.0
* Fixed 2 keys being pressed at same time causing problems on XCB
* Made scrolling fast work on Windows ( increments of 120 )
* Added joystick support for Linux
* Fixed cursor being reported in wrong place when maximized on Windows
* Fixed screen resolution being wrong when maximized on Windows

## adi_screen 0.1.0
* Initial Release
