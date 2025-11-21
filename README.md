# Browser

A production-ready web browser built in Rust with GPU-accelerated rendering.

## Features

- HTML5 parsing with `html5ever`
- CSS parsing and selectors
- GPU rendering with `wgpu`
- Async networking with `reqwest` and `tokio`
- Font rendering with `rusttype` and `fontdue`
- Cross-platform window management with `winit`

## Requirements

- Rust 1.70+
- System dependencies:
  - **Linux**: `libxkbcommon-dev`, `libwayland-dev`
  - **macOS**: Xcode Command Line Tools
  - **Windows**: Visual Studio Build Tools

## Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run
cargo run
```

## Testing

```bash
cargo test --all-features
```

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
