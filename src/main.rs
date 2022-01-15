#![forbid(unsafe_code)]
//#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]
mod hex_input;
mod micro_code_view;
mod ram_view;
mod register_view;
mod view;
mod vm;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = crate::view::VMView::init();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
