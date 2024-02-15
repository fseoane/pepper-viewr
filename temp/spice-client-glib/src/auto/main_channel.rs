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
    #[doc(alias = "SpiceMainChannel")]
    pub struct MainChannel(Object<ffi::SpiceMainChannel, ffi::SpiceMainChannelClass>) @extends Channel;

    match fn {
        type_ => || ffi::spice_main_channel_get_type(),
    }
}

impl MainChannel {
    #[doc(alias = "spice_main_channel_agent_test_capability")]
    pub fn agent_test_capability(&self, cap: u32) -> bool {
        unsafe {
            from_glib(ffi::spice_main_channel_agent_test_capability(
                self.to_glib_none().0,
                cap,
            ))
        }
    }

    #[doc(alias = "spice_main_channel_clipboard_selection_grab")]
    pub fn clipboard_selection_grab(&self, selection: u32, types: &[u32]) {
        let ntypes = types.len() as _;
        unsafe {
            ffi::spice_main_channel_clipboard_selection_grab(
                self.to_glib_none().0,
                selection,
                types.to_glib_none().0,
                ntypes,
            );
        }
    }

    #[doc(alias = "spice_main_channel_clipboard_selection_notify")]
    pub fn clipboard_selection_notify(&self, selection: u32, type_: u32, data: &[u8]) {
        let size = data.len() as _;
        unsafe {
            ffi::spice_main_channel_clipboard_selection_notify(
                self.to_glib_none().0,
                selection,
                type_,
                data.to_glib_none().0,
                size,
            );
        }
    }

    #[doc(alias = "spice_main_channel_clipboard_selection_release")]
    pub fn clipboard_selection_release(&self, selection: u32) {
        unsafe {
            ffi::spice_main_channel_clipboard_selection_release(self.to_glib_none().0, selection);
        }
    }

    #[doc(alias = "spice_main_channel_clipboard_selection_request")]
    pub fn clipboard_selection_request(&self, selection: u32, type_: u32) {
        unsafe {
            ffi::spice_main_channel_clipboard_selection_request(
                self.to_glib_none().0,
                selection,
                type_,
            );
        }
    }

    //#[doc(alias = "spice_main_channel_file_copy_async")]
    //pub fn file_copy_async<P: FnOnce(Result<(), glib::Error>) + 'static, Q: FnOnce(Result<(), glib::Error>) + 'static>(&self, sources: &[gio::File], flags: gio::FileCopyFlags, cancellable: Option<&impl IsA<gio::Cancellable>>, progress_callback: P, callback: Q) {
    //    unsafe { TODO: call ffi:spice_main_channel_file_copy_async() }
    //}

    //
    //pub fn file_copy_future<P: FnOnce(Result<(), glib::Error>) + 'static>(&self, sources: &[gio::File], flags: gio::FileCopyFlags, progress_callback: P) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {

    //let sources = sources.clone();
    //let progress_callback = progress_callback.map(ToOwned::to_owned);
    //Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
    //    obj.file_copy_async(
    //        &sources,
    //        flags,
    //        Some(cancellable),
    //        progress_callback.as_ref().map(::std::borrow::Borrow::borrow),
    //        move |res| {
    //            send.resolve(res);
    //        },
    //    );
    //}))
    //}

    #[doc(alias = "spice_main_channel_request_mouse_mode")]
    pub fn request_mouse_mode(&self, mode: i32) {
        unsafe {
            ffi::spice_main_channel_request_mouse_mode(self.to_glib_none().0, mode);
        }
    }

    #[doc(alias = "spice_main_channel_send_monitor_config")]
    pub fn send_monitor_config(&self) -> bool {
        unsafe {
            from_glib(ffi::spice_main_channel_send_monitor_config(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "spice_main_channel_update_display")]
    pub fn update_display(&self, id: i32, x: i32, y: i32, width: i32, height: i32, update: bool) {
        unsafe {
            ffi::spice_main_channel_update_display(
                self.to_glib_none().0,
                id,
                x,
                y,
                width,
                height,
                update.into_glib(),
            );
        }
    }

    #[doc(alias = "spice_main_channel_update_display_enabled")]
    pub fn update_display_enabled(&self, id: i32, enabled: bool, update: bool) {
        unsafe {
            ffi::spice_main_channel_update_display_enabled(
                self.to_glib_none().0,
                id,
                enabled.into_glib(),
                update.into_glib(),
            );
        }
    }

    #[doc(alias = "spice_main_channel_update_display_mm")]
    pub fn update_display_mm(&self, id: i32, width_mm: i32, height_mm: i32, update: bool) {
        unsafe {
            ffi::spice_main_channel_update_display_mm(
                self.to_glib_none().0,
                id,
                width_mm,
                height_mm,
                update.into_glib(),
            );
        }
    }

    #[doc(alias = "agent-caps-0")]
    pub fn agent_caps_0(&self) -> i32 {
        ObjectExt::property(self, "agent-caps-0")
    }

    #[doc(alias = "agent-connected")]
    pub fn is_agent_connected(&self) -> bool {
        ObjectExt::property(self, "agent-connected")
    }

    #[doc(alias = "disable-animation")]
    pub fn is_disable_animation(&self) -> bool {
        ObjectExt::property(self, "disable-animation")
    }

    #[doc(alias = "disable-animation")]
    pub fn set_disable_animation(&self, disable_animation: bool) {
        ObjectExt::set_property(self, "disable-animation", disable_animation)
    }

    #[doc(alias = "disable-display-align")]
    pub fn is_disable_display_align(&self) -> bool {
        ObjectExt::property(self, "disable-display-align")
    }

    #[doc(alias = "disable-display-align")]
    pub fn set_disable_display_align(&self, disable_display_align: bool) {
        ObjectExt::set_property(self, "disable-display-align", disable_display_align)
    }

    #[doc(alias = "disable-display-position")]
    pub fn is_disable_display_position(&self) -> bool {
        ObjectExt::property(self, "disable-display-position")
    }

    #[doc(alias = "disable-display-position")]
    pub fn set_disable_display_position(&self, disable_display_position: bool) {
        ObjectExt::set_property(self, "disable-display-position", disable_display_position)
    }

    #[doc(alias = "disable-font-smooth")]
    pub fn is_disable_font_smooth(&self) -> bool {
        ObjectExt::property(self, "disable-font-smooth")
    }

    #[doc(alias = "disable-font-smooth")]
    pub fn set_disable_font_smooth(&self, disable_font_smooth: bool) {
        ObjectExt::set_property(self, "disable-font-smooth", disable_font_smooth)
    }

    #[doc(alias = "disable-wallpaper")]
    pub fn is_disable_wallpaper(&self) -> bool {
        ObjectExt::property(self, "disable-wallpaper")
    }

    #[doc(alias = "disable-wallpaper")]
    pub fn set_disable_wallpaper(&self, disable_wallpaper: bool) {
        ObjectExt::set_property(self, "disable-wallpaper", disable_wallpaper)
    }

    #[doc(alias = "max-clipboard")]
    pub fn max_clipboard(&self) -> i32 {
        ObjectExt::property(self, "max-clipboard")
    }

    #[doc(alias = "max-clipboard")]
    pub fn set_max_clipboard(&self, max_clipboard: i32) {
        ObjectExt::set_property(self, "max-clipboard", max_clipboard)
    }

    #[doc(alias = "mouse-mode")]
    pub fn mouse_mode(&self) -> i32 {
        ObjectExt::property(self, "mouse-mode")
    }

    #[doc(alias = "main-agent-update")]
    pub fn connect_main_agent_update<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn main_agent_update_trampoline<F: Fn(&MainChannel) + 'static>(
            this: *mut ffi::SpiceMainChannel,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"main-agent-update\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    main_agent_update_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "main-clipboard-selection-release")]
    pub fn connect_main_clipboard_selection_release<F: Fn(&Self, u32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn main_clipboard_selection_release_trampoline<
            F: Fn(&MainChannel, u32) + 'static,
        >(
            this: *mut ffi::SpiceMainChannel,
            selection: libc::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), selection)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"main-clipboard-selection-release\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    main_clipboard_selection_release_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "main-clipboard-selection-request")]
    pub fn connect_main_clipboard_selection_request<F: Fn(&Self, u32, u32) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn main_clipboard_selection_request_trampoline<
            F: Fn(&MainChannel, u32, u32) -> bool + 'static,
        >(
            this: *mut ffi::SpiceMainChannel,
            selection: libc::c_uint,
            types: libc::c_uint,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), selection, types).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"main-clipboard-selection-request\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    main_clipboard_selection_request_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "main-mouse-update")]
    pub fn connect_main_mouse_update<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn main_mouse_update_trampoline<F: Fn(&MainChannel) + 'static>(
            this: *mut ffi::SpiceMainChannel,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"main-mouse-update\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    main_mouse_update_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "migration-started")]
    pub fn connect_migration_started<F: Fn(&Self, &glib::Object) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn migration_started_trampoline<
            F: Fn(&MainChannel, &glib::Object) + 'static,
        >(
            this: *mut ffi::SpiceMainChannel,
            session: *mut glib::gobject_ffi::GObject,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(session))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"migration-started\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    migration_started_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "new-file-transfer")]
    pub fn connect_new_file_transfer<F: Fn(&Self, &glib::Object) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn new_file_transfer_trampoline<
            F: Fn(&MainChannel, &glib::Object) + 'static,
        >(
            this: *mut ffi::SpiceMainChannel,
            task: *mut glib::gobject_ffi::GObject,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(task))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"new-file-transfer\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    new_file_transfer_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "agent-caps-0")]
    pub fn connect_agent_caps_0_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_agent_caps_0_trampoline<F: Fn(&MainChannel) + 'static>(
            this: *mut ffi::SpiceMainChannel,
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
                b"notify::agent-caps-0\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_agent_caps_0_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "agent-connected")]
    pub fn connect_agent_connected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_agent_connected_trampoline<F: Fn(&MainChannel) + 'static>(
            this: *mut ffi::SpiceMainChannel,
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
                b"notify::agent-connected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_agent_connected_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "disable-animation")]
    pub fn connect_disable_animation_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_disable_animation_trampoline<F: Fn(&MainChannel) + 'static>(
            this: *mut ffi::SpiceMainChannel,
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
                b"notify::disable-animation\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_disable_animation_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "disable-display-align")]
    pub fn connect_disable_display_align_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_disable_display_align_trampoline<
            F: Fn(&MainChannel) + 'static,
        >(
            this: *mut ffi::SpiceMainChannel,
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
                b"notify::disable-display-align\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_disable_display_align_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "disable-display-position")]
    pub fn connect_disable_display_position_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_disable_display_position_trampoline<
            F: Fn(&MainChannel) + 'static,
        >(
            this: *mut ffi::SpiceMainChannel,
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
                b"notify::disable-display-position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_disable_display_position_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "disable-font-smooth")]
    pub fn connect_disable_font_smooth_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_disable_font_smooth_trampoline<
            F: Fn(&MainChannel) + 'static,
        >(
            this: *mut ffi::SpiceMainChannel,
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
                b"notify::disable-font-smooth\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_disable_font_smooth_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "disable-wallpaper")]
    pub fn connect_disable_wallpaper_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_disable_wallpaper_trampoline<F: Fn(&MainChannel) + 'static>(
            this: *mut ffi::SpiceMainChannel,
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
                b"notify::disable-wallpaper\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_disable_wallpaper_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "max-clipboard")]
    pub fn connect_max_clipboard_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_clipboard_trampoline<F: Fn(&MainChannel) + 'static>(
            this: *mut ffi::SpiceMainChannel,
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
                b"notify::max-clipboard\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_clipboard_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "mouse-mode")]
    pub fn connect_mouse_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mouse_mode_trampoline<F: Fn(&MainChannel) + 'static>(
            this: *mut ffi::SpiceMainChannel,
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
                b"notify::mouse-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mouse_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for MainChannel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MainChannel")
    }
}