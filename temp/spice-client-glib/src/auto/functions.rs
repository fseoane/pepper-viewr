// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{MainChannel, Session};
use glib::{prelude::*, translate::*};
use std::ptr;

//#[doc(alias = "spice_g_signal_connect_object")]
//pub fn g_signal_connect_object<P: Fn() + Send + Sync + 'static>(instance: /*Unimplemented*/Option<Basic: Pointer>, detailed_signal: &str, c_handler: P, gobject: /*Unimplemented*/Option<Basic: Pointer>, connect_flags: /*Ignored*/glib::ConnectFlags) -> libc::c_ulong {
//    unsafe { TODO: call ffi:spice_g_signal_connect_object() }
//}

#[doc(alias = "spice_set_session_option")]
pub fn set_session_option(session: &Session) {
    skip_assert_initialized!();
    unsafe {
        ffi::spice_set_session_option(session.to_glib_none().0);
    }
}

#[doc(alias = "spice_util_get_debug")]
#[doc(alias = "util_get_debug")]
pub fn debug() -> bool {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::spice_util_get_debug()) }
}

#[doc(alias = "spice_util_get_version_string")]
#[doc(alias = "util_get_version_string")]
pub fn version() -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe { from_glib_none(ffi::spice_util_get_version_string()) }
}

#[doc(alias = "spice_util_set_debug")]
#[doc(alias = "util_set_debug")]
pub fn set_debug(enabled: bool) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::spice_util_set_debug(enabled.into_glib());
    }
}

//#[doc(alias = "spice_uuid_to_string")]
//pub fn uuid_to_string(uuid: /*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 3 }; 16) -> Option<glib::GString> {
//    unsafe { TODO: call ffi:spice_uuid_to_string() }
//}
