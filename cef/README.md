# CEF Browser

A production-ready web browser built using the Chromium Embedded Framework (CEF).

## Features

- **Full Chromium Engine**: Complete HTML5, CSS3, and JavaScript support
- **GPU Acceleration**: Hardware-accelerated rendering via Chromium's compositor
- **Modern Web Standards**: ES6+, WebGL, WebAssembly, Service Workers, and more
- **Developer Tools**: Built-in Chrome DevTools (F12 or Ctrl+Shift+I)
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Multi-Process Architecture**: Separate processes for browser, renderer, and GPU
- **Customizable**: Full control over browser behavior through handlers

## Requirements

### Linux (Debian/Ubuntu)
```bash
sudo apt-get install build-essential cmake libgtk-3-dev libglib2.0-dev \
    libnss3-dev libatk1.0-dev libatk-bridge2.0-dev libcups2-dev \
    libxcomposite-dev libxdamage-dev libxrandr-dev libgbm-dev libasound2-dev
```

### Linux (Fedora/RHEL)
```bash
sudo dnf install cmake gtk3-devel nss-devel atk-devel at-spi2-atk-devel \
    cups-devel alsa-lib-devel
```

### macOS
```bash
xcode-select --install
```

### Windows
- Visual Studio 2019 or newer with C++ workload
- CMake 3.19 or newer

## Building

### Quick Start
```bash
cd cef
chmod +x build.sh
./build.sh Release
```

### Manual Build
```bash
mkdir build && cd build
cmake -DCMAKE_BUILD_TYPE=Release ..
cmake --build . --config Release -j$(nproc)
```

### Build Options
- `CMAKE_BUILD_TYPE`: Debug or Release (default: Release)
- `CEF_VERSION`: CEF version to download (default: 120.1.10+...)

## Running

```bash
cd build
./cef_browser
```

### Command Line Options
- Remote debugging is enabled by default at `http://localhost:9222`

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| Ctrl+R / F5 | Reload page |
| Ctrl+Shift+R | Reload ignoring cache |
| Ctrl+Shift+I / F12 | Open DevTools |
| Ctrl+L | Focus address bar |
| Ctrl+W | Close tab |
| Alt+Left | Go back |
| Alt+Right | Go forward |

## Project Structure

```
cef/
├── CMakeLists.txt          # Build configuration
├── build.sh                # Build script
├── README.md               # This file
├── src/
│   ├── main.cpp            # Application entry point
│   ├── app.h/cpp           # CEF application handler
│   ├── browser_client.h/cpp # Browser event handlers
│   ├── browser_window.h/cpp # Window management
│   ├── resource_util.h/cpp  # Resource utilities
│   └── helper_main.cpp      # Subprocess entry point
└── resources/
    └── macos/
        └── Info.plist       # macOS app bundle info
```

## Architecture

The browser uses CEF's multi-process architecture:

1. **Browser Process**: Main process handling UI and coordination
2. **Renderer Process**: Handles HTML/CSS parsing and JavaScript execution
3. **GPU Process**: Hardware-accelerated compositing and WebGL
4. **Network Service**: Handles all network requests

## Customization

### Adding JavaScript Bindings
Edit `app.cpp` in the `OnContextCreated` method to inject custom JavaScript objects.

### Custom Context Menu
Edit `browser_client.cpp` in the `OnBeforeContextMenu` and `OnContextMenuCommand` methods.

### Custom Keyboard Shortcuts
Edit `browser_client.cpp` in the `OnPreKeyEvent` method.

## License

MIT License - See LICENSE file for details.

## CEF Resources

- [CEF Project](https://bitbucket.org/chromiumembedded/cef)
- [CEF Builds](https://cef-builds.spotifycdn.com/index.html)
- [CEF Documentation](https://bitbucket.org/chromiumembedded/cef/wiki/Home)
