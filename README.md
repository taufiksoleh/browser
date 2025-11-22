# Browser

A web browser project with two implementations:

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

## CI/CD Pipeline

The project includes GitHub Actions workflows that:

- **Lint**: Check formatting and run clippy
- **Test**: Run unit tests
- **Build**: Build for Linux and macOS targets

### Supported Platforms

| Platform | Architecture | Status |
|----------|-------------|--------|
| Linux    | x86_64      | ✅     |
| macOS    | x86_64      | ✅     |
| macOS    | ARM64       | ✅     |

## License

MIT
