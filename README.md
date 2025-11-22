# Browser

A web browser project with three implementations across multiple platforms:

## CEF Browser (Recommended)

A production-ready web browser built using the **Chromium Embedded Framework (CEF)**. This provides full Chromium functionality including:

- Complete HTML5, CSS3, and JavaScript support
- GPU-accelerated rendering
- Modern web standards (ES6+, WebGL, WebAssembly, Service Workers)
- Built-in Chrome DevTools
- Cross-platform support (Windows, macOS, Linux)

**See [cef/README.md](cef/README.md) for build instructions.**

### Quick Start (CEF)
```bash
cd cef
./build.sh Release
cd build
./cef_browser
```

---

## Rust Browser (Experimental)

An experimental web browser built from scratch in Rust with GPU-accelerated rendering.

### Features

- HTML5 parsing with `html5ever`
- CSS parsing and selectors
- GPU rendering with `wgpu`
- Async networking with `reqwest` and `tokio`
- Font rendering with `rusttype` and `fontdue`
- Cross-platform window management with `winit`

### Requirements

- Rust 1.70+
- System dependencies:
  - **Linux**: `libxkbcommon-dev`, `libwayland-dev`
  - **macOS**: Xcode Command Line Tools
  - **Windows**: Visual Studio Build Tools

### Building (Rust)

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run
cargo run
```

### Testing

```bash
cargo test --all-features
```

---

## Android Browser

A native Android web browser built using **Android WebView** with Chromium rendering engine.

### Features

- Full Chromium Engine via Android WebView
- Native Android UI with Material Design
- Modern web standards (ES6+, WebGL, WebAssembly)
- Fast and lightweight
- Privacy focused

**See [android/README.md](android/README.md) for build instructions.**

### Quick Start (Android)

```bash
cd android
./gradlew assembleDebug
```

Install on device:
```bash
adb install app/build/outputs/apk/debug/app-debug.apk
```

### Requirements

- Android 7.0 (API level 24) or higher
- JDK 17 or newer
- Android SDK (for building)

---

## CI/CD Pipeline

The project includes GitHub Actions workflows that:

- **CEF Browser**: Lint, build (Linux/macOS), unit tests, smoke tests, security scan
- **Rust Browser**: Lint, test, build for Linux and macOS targets
- **Android Browser**: Lint, build debug/release APKs, unit tests, security scan

### Supported Platforms

| Platform | Architecture | Implementation | Status |
|----------|-------------|----------------|--------|
| Linux    | x86_64      | CEF, Rust      | ✅     |
| macOS    | x86_64      | CEF, Rust      | ✅     |
| macOS    | ARM64       | Rust           | ✅     |
| Windows  | x86_64      | CEF, Rust      | ✅     |
| Android  | ARM64, ARMv7, x86_64 | Android WebView | ✅     |

## License

MIT
