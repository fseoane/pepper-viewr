use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem::transmute;

use crate::{DisplayChannel, DisplayMonitorConfig, DisplayPrimary};

impl DisplayChannel {
    pub fn connect_display_primary_create<F: Fn(&DisplayChannel) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn display_primary_destroy_trampoline<
            F: Fn(&DisplayChannel) + 'static,
        >(
            this: *mut ffi::SpiceDisplayChannel,
            _format: libc::c_int,
            _width: libc::c_int,
            _height: libc::c_int,
            _stride: libc::c_int,
            _shmid: libc::c_int,
            _imgdata: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"display-primary-create\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    display_primary_destroy_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn monitors(&self) -> Option<Vec<DisplayMonitorConfig>> {
        unsafe {
            let mut value = glib::Value::from_type(from_glib(glib::ffi::g_array_get_type()));
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"monitors\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            let array = glib::gobject_ffi::g_value_get_boxed(value.to_glib_none().0)
                as *const glib::ffi::GArray;
            if array.is_null() {
                None
            } else {
                let array = &*array;
                let data = array.data as *const ffi::SpiceDisplayMonitorConfig;
                let len = array.len as usize;

                let mut vec = Vec::with_capacity(len);
                for i in 0..len {
                    vec.push(from_glib_none(data.add(i)))
                }
                Some(vec)
            }
        }
    }

    pub fn primary(&self, id: u32) -> Option<DisplayPrimary> {
        let mut primary = DisplayPrimary::new();

        if self.get_primary(id, &mut primary).is_ok() {
            Some(primary)
        } else {
            None
        }
    }
}
