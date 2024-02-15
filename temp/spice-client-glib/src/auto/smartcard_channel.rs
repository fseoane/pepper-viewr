// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Channel;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "SpiceSmartcardChannel")]
    pub struct SmartcardChannel(Object<ffi::SpiceSmartcardChannel, ffi::SpiceSmartcardChannelClass>) @extends Channel;

    match fn {
        type_ => || ffi::spice_smartcard_channel_get_type(),
    }
}

impl SmartcardChannel {}

impl fmt::Display for SmartcardChannel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SmartcardChannel")
    }
}
