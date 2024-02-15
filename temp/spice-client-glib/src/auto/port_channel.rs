// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Channel;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute, ptr};

glib::wrapper! {
    #[doc(alias = "SpicePortChannel")]
    pub struct PortChannel(Object<ffi::SpicePortChannel, ffi::SpicePortChannelClass>) @extends Channel;

    match fn {
        type_ => || ffi::spice_port_channel_get_type(),
    }
}

impl PortChannel {
    pub const NONE: Option<&'static PortChannel> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::PortChannel>> Sealed for T {}
}

pub trait PortChannelExt: IsA<PortChannel> + sealed::Sealed + 'static {
    #[doc(alias = "spice_port_channel_event")]
    fn event(&self, event: u8) {
        unsafe {
            ffi::spice_port_channel_event(self.as_ref().to_glib_none().0, event);
        }
    }

    #[doc(alias = "port-name")]
    fn port_name(&self) -> Option<glib::GString> {
        ObjectExt::property(self.as_ref(), "port-name")
    }

    #[doc(alias = "port-opened")]
    fn is_port_opened(&self) -> bool {
        ObjectExt::property(self.as_ref(), "port-opened")
    }

    //#[doc(alias = "port-data")]
    //fn connect_port_data<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Unimplemented data: *.Pointer
    //}

    #[doc(alias = "port-event")]
    fn connect_port_event<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn port_event_trampoline<
            P: IsA<PortChannel>,
            F: Fn(&P, i32) + 'static,
        >(
            this: *mut ffi::SpicePortChannel,
            event: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(PortChannel::from_glib_borrow(this).unsafe_cast_ref(), event)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"port-event\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    port_event_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "port-name")]
    fn connect_port_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_port_name_trampoline<
            P: IsA<PortChannel>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::SpicePortChannel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(PortChannel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::port-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_port_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "port-opened")]
    fn connect_port_opened_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_port_opened_trampoline<
            P: IsA<PortChannel>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::SpicePortChannel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(PortChannel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::port-opened\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_port_opened_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<PortChannel>> PortChannelExt for O {}

impl fmt::Display for PortChannel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PortChannel")
    }
}