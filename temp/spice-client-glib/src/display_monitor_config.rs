// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

#[derive(Debug, Clone, Copy)]
pub struct DisplayMonitorConfig(ffi::SpiceDisplayMonitorConfig);

impl DisplayMonitorConfig {
    pub fn id(&self) -> u32 {
        self.0.id
    }

    pub fn surface_id(&self) -> u32 {
        self.0.surface_id
    }

    pub fn geometry(&self) -> (usize, usize, usize, usize) {
        (
            self.0.x as _,
            self.0.y as _,
            self.0.width as _,
            self.0.height as _,
        )
    }

    pub fn as_ptr(&self) -> *const ffi::SpiceDisplayMonitorConfig {
        &self.0
    }
}

impl From<ffi::SpiceDisplayMonitorConfig> for DisplayMonitorConfig {
    fn from(p: ffi::SpiceDisplayMonitorConfig) -> Self {
        skip_assert_initialized!();
        Self(p)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::SpiceDisplayMonitorConfig> for DisplayMonitorConfig {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::SpiceDisplayMonitorConfig, Self> {
        Stash(&self.0, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::SpiceDisplayMonitorConfig> for DisplayMonitorConfig {
    #[inline]
    unsafe fn from_glib_none(ptr: *const ffi::SpiceDisplayMonitorConfig) -> Self {
        Self(*ptr)
    }
}
