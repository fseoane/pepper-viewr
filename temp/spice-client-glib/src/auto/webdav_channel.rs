// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{Channel, PortChannel};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "SpiceWebdavChannel")]
    pub struct WebdavChannel(Object<ffi::SpiceWebdavChannel, ffi::SpiceWebdavChannelClass>) @extends PortChannel, Channel;

    match fn {
        type_ => || ffi::spice_webdav_channel_get_type(),
    }
}

impl WebdavChannel {}

impl fmt::Display for WebdavChannel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WebdavChannel")
    }
}