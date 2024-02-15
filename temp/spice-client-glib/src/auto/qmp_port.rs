// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{PortChannel, QmpPortVmAction, QmpStatus};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute, pin::Pin, ptr};

glib::wrapper! {
    #[doc(alias = "SpiceQmpPort")]
    pub struct QmpPort(Object<ffi::SpiceQmpPort, ffi::SpiceQmpPortClass>);

    match fn {
        type_ => || ffi::spice_qmp_port_get_type(),
    }
}

impl QmpPort {
    #[doc(alias = "spice_qmp_port_query_status_async")]
    pub fn query_status_async<P: FnOnce(Result<QmpStatus, glib::Error>) + 'static>(
        &self,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn query_status_async_trampoline<
            P: FnOnce(Result<QmpStatus, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::spice_qmp_port_query_status_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = query_status_async_trampoline::<P>;
        unsafe {
            ffi::spice_qmp_port_query_status_async(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn query_status_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<QmpStatus, glib::Error>> + 'static>> {
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.query_status_async(Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "spice_qmp_port_vm_action_async")]
    pub fn vm_action_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        action: QmpPortVmAction,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn vm_action_async_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::spice_qmp_port_vm_action_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = vm_action_async_trampoline::<P>;
        unsafe {
            ffi::spice_qmp_port_vm_action_async(
                self.to_glib_none().0,
                action.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn vm_action_future(
        &self,
        action: QmpPortVmAction,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.vm_action_async(action, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    pub fn channel(&self) -> Option<PortChannel> {
        ObjectExt::property(self, "channel")
    }

    pub fn is_ready(&self) -> bool {
        ObjectExt::property(self, "ready")
    }

    #[doc(alias = "spice_qmp_port_get")]
    pub fn get(channel: &impl IsA<PortChannel>) -> Option<QmpPort> {
        skip_assert_initialized!();
        unsafe { from_glib_none(ffi::spice_qmp_port_get(channel.as_ref().to_glib_none().0)) }
    }

    //#[doc(alias = "event")]
    //pub fn connect_event<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Unimplemented node: *.Pointer
    //}

    #[doc(alias = "ready")]
    pub fn connect_ready_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ready_trampoline<F: Fn(&QmpPort) + 'static>(
            this: *mut ffi::SpiceQmpPort,
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
                b"notify::ready\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ready_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for QmpPort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("QmpPort")
    }
}
