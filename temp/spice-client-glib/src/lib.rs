// Take a look at the license at the top of the repository in the LICENSE file.

//! # Spice GLib client bindings
//!
//! This library contains safe Rust bindings for spice-client-glib.

#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;
pub use gio;
pub use glib;

// no runtime to initialize
macro_rules! assert_initialized_main_thread {
    () => {};
}

// No-op
macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(unused_imports)]
mod auto;
pub use auto::functions::*;
pub use auto::*;

pub mod prelude;
pub mod subclass;

mod protocol;
pub use protocol::*;

mod cursor_shape;

mod display_channel;
mod main_channel;

mod display_primary;
pub use display_primary::DisplayPrimary;

mod display_monitor_config;
pub use display_monitor_config::DisplayMonitorConfig;

mod gl_scanout;

mod usb_device;
