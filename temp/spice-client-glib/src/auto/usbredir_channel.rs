// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Channel;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "SpiceUsbredirChannel")]
    pub struct UsbredirChannel(Object<ffi::SpiceUsbredirChannel, ffi::SpiceUsbredirChannelClass>) @extends Channel;

    match fn {
        type_ => || ffi::spice_usbredir_channel_get_type(),
    }
}

impl UsbredirChannel {}

impl fmt::Display for UsbredirChannel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("UsbredirChannel")
    }
}
