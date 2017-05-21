# adi_screen
Portable rust library for rendering onto a computer screen or phone screen.  Can
be used for video games or applications.

# releases

## adi_screen 0.1.0
* Initial Release

### support

| Platform              | Vulkan | OpenGL | Metal | Window | Touch | Touchpad | Mouse | Keyboard | Joystick |
|-----------------------|--------|--------|-------|--------|-------|----------|-------|----------|----------|
| XCB on Linux          | Yes    | No     | No    | Yes    | No    | Yes      | Yes   | Yes      | No       |
| Windows               | Yes    | No     | No    | Yes    | No    | No       | Yes   | Yes      | No       |

# next planned release

## adi_screen 0.1.1
* Fix 2 keys being pressed at same time causing problems on XCB.

* Make scrolling fast work on windows ( increments of 120 )

* Add Joystick Support on Linux
