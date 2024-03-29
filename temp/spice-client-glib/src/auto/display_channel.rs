// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{Channel, DisplayPrimary, GlScanout};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute, ptr};

glib::wrapper! {
    #[doc(alias = "SpiceDisplayChannel")]
    pub struct DisplayChannel(Object<ffi::SpiceDisplayChannel, ffi::SpiceDisplayChannelClass>) @extends Channel;

    match fn {
        type_ => || ffi::spice_display_channel_get_type(),
    }
}

impl DisplayChannel {
    #[doc(alias = "spice_display_channel_change_preferred_compression")]
    #[doc(alias = "display_channel_change_preferred_compression")]
    pub fn change_preferred_compression(&self, compression: i32) {
        unsafe {
            ffi::spice_display_channel_change_preferred_compression(
                self.to_glib_none().0,
                compression,
            );
        }
    }

    #[doc(alias = "spice_display_channel_change_preferred_video_codec_types")]
    #[doc(alias = "display_channel_change_preferred_video_codec_types")]
    pub fn change_preferred_video_codec_types(&self, codecs: &[i32]) -> Result<(), glib::Error> {
        let ncodecs = codecs.len() as _;
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::spice_display_channel_change_preferred_video_codec_types(
                self.to_glib_none().0,
                codecs.to_glib_none().0,
                ncodecs,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "spice_display_channel_get_primary")]
    #[doc(alias = "display_channel_get_primary")]
    pub fn get_primary(
        &self,
        surface_id: u32,
        primary: &mut DisplayPrimary,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::spice_display_channel_get_primary(
                    self.to_glib_none().0,
                    surface_id,
                    primary.to_glib_none_mut().0
                ),
                "No primary surface"
            )
        }
    }

    #[doc(alias = "spice_display_channel_get_gl_scanout")]
    #[doc(alias = "get_gl_scanout")]
    pub fn gl_scanout(&self) -> Option<GlScanout> {
        unsafe {
            from_glib_none(ffi::spice_display_channel_get_gl_scanout(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "spice_display_channel_gl_draw_done")]
    pub fn gl_draw_done(&self) {
        unsafe {
            ffi::spice_display_channel_gl_draw_done(self.to_glib_none().0);
        }
    }

    pub fn height(&self) -> u32 {
        ObjectExt::property(self, "height")
    }

    #[doc(alias = "monitors-max")]
    pub fn monitors_max(&self) -> u32 {
        ObjectExt::property(self, "monitors-max")
    }

    pub fn width(&self) -> u32 {
        ObjectExt::property(self, "width")
    }

    #[doc(alias = "display-invalidate")]
    pub fn connect_display_invalidate<F: Fn(&Self, i32, i32, i32, i32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn display_invalidate_trampoline<
            F: Fn(&DisplayChannel, i32, i32, i32, i32) + 'static,
        >(
            this: *mut ffi::SpiceDisplayChannel,
            x: libc::c_int,
            y: libc::c_int,
            width: libc::c_int,
            height: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y, width, height)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"display-invalidate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    display_invalidate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "display-mark")]
    pub fn connect_display_mark<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn display_mark_trampoline<F: Fn(&DisplayChannel, i32) + 'static>(
            this: *mut ffi::SpiceDisplayChannel,
            mark: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), mark)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"display-mark\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    display_mark_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "display-primary-destroy")]
    pub fn connect_display_primary_destroy<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn display_primary_destroy_trampoline<
            F: Fn(&DisplayChannel) + 'static,
        >(
            this: *mut ffi::SpiceDisplayChannel,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"display-primary-destroy\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    display_primary_destroy_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "gl-draw")]
    pub fn connect_gl_draw<F: Fn(&Self, u32, u32, u32, u32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn gl_draw_trampoline<
            F: Fn(&DisplayChannel, u32, u32, u32, u32) + 'static,
        >(
            this: *mut ffi::SpiceDisplayChannel,
            x: libc::c_uint,
            y: libc::c_uint,
            width: libc::c_uint,
            height: libc::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y, width, height)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"gl-draw\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    gl_draw_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    //#[doc(alias = "gst-video-overlay")]
    //pub fn connect_gst_video_overlay<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored pipeline: Gst.Pipeline
    //}

    #[doc(alias = "gl-scanout")]
    pub fn connect_gl_scanout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gl_scanout_trampoline<F: Fn(&DisplayChannel) + 'static>(
            this: *mut ffi::SpiceDisplayChannel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::gl-scanout\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_gl_scanout_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "height")]
    pub fn connect_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_height_trampoline<F: Fn(&DisplayChannel) + 'static>(
            this: *mut ffi::SpiceDisplayChannel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_height_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "monitors")]
    pub fn connect_monitors_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_monitors_trampoline<F: Fn(&DisplayChannel) + 'static>(
            this: *mut ffi::SpiceDisplayChannel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::monitors\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_monitors_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "monitors-max")]
    pub fn connect_monitors_max_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_monitors_max_trampoline<F: Fn(&DisplayChannel) + 'static>(
            this: *mut ffi::SpiceDisplayChannel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::monitors-max\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_monitors_max_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "width")]
    pub fn connect_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_trampoline<F: Fn(&DisplayChannel) + 'static>(
            this: *mut ffi::SpiceDisplayChannel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_width_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DisplayChannel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DisplayChannel")
    }
}
