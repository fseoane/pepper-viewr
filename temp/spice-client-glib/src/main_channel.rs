use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem::transmute;

use crate::MainChannel;

impl MainChannel {
    pub fn connect_main_clipboard_selection<F: Fn(&MainChannel, u32, u32, &[u8]) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn main_clipboard_selection_trampoline<
            F: Fn(&MainChannel, u32, u32, &[u8]) + 'static,
        >(
            this: *mut ffi::SpiceMainChannel,
            selection: libc::c_uint,
            type_: libc::c_uint,
            data: glib::ffi::gpointer,
            size: libc::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                selection,
                type_,
                &FromGlibContainerAsVec::from_glib_none_num_as_vec(data as *const u8, size as _),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"main-clipboard-selection\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    main_clipboard_selection_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_main_clipboard_selection_grab<F: Fn(&MainChannel, u32, &[u32]) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn main_clipboard_selection_trampoline<
            F: Fn(&MainChannel, u32, &[u32]) + 'static,
        >(
            this: *mut ffi::SpiceMainChannel,
            selection: libc::c_uint,
            types: glib::ffi::gpointer,
            ntypes: libc::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                selection,
                &FromGlibContainerAsVec::from_glib_none_num_as_vec(
                    types as *const u32,
                    ntypes as _,
                ),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"main-clipboard-selection-grab\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    main_clipboard_selection_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
