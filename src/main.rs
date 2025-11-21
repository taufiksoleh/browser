// Allow dead code for library-style codebase where not all APIs are used in main()
#![allow(dead_code)]
#![allow(unused_imports)]

//! Browser - A production-ready web browser in Rust
//!
//! Architecture follows multi-process model for security:
//! - Browser Process: UI, network, storage coordination
//! - Renderer Process: HTML/CSS parsing, layout, paint
//! - GPU Process: Hardware-accelerated compositing

mod browser;
mod css;
mod dom;
mod layout;
mod network;
mod render;
mod ui;

use browser::Browser;
use log::info;

fn main() {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    info!("Starting Browser v{}", env!("CARGO_PKG_VERSION"));

    // Run the browser
    if let Err(e) = Browser::run() {
        log::error!("Browser error: {}", e);
        std::process::exit(1);
    }
}
