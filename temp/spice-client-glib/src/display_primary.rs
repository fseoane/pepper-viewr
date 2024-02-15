// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::SurfaceFormat;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DisplayPrimary(ffi::SpiceDisplayPrimary);

impl DisplayPrimary {
    pub fn new() -> Self {
        Self(unsafe { std::mem::zeroed() })
    }

    pub fn format(&self) -> Result<SurfaceFormat, i32> {
        self.0.format.try_into()
    }

    pub fn width(&self) -> usize {
        self.0.width as _
    }

    pub fn height(&self) -> usize {
        self.0.height as _
    }

    pub fn stride(&self) -> usize {
        self.0.stride as _
    }

    pub fn shmid(&self) -> i32 {
        self.0.shmid
    }

    pub fn data(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.0.data, self.stride() * self.height()) }
    }

    pub fn marked(&self) -> bool {
        unsafe { from_glib(self.0.marked) }
    }

    pub fn as_ptr(&self) -> *const ffi::SpiceDisplayPrimary {
        &self.0
    }
}

impl From<ffi::SpiceDisplayPrimary> for DisplayPrimary {
    fn from(p: ffi::SpiceDisplayPrimary) -> Self {
        skip_assert_initialized!();
        Self(p)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::SpiceDisplayPrimary> for DisplayPrimary {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::SpiceDisplayPrimary, Self> {
        Stash(&self.0, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::SpiceDisplayPrimary> for DisplayPrimary {
    type Storage = &'a mut Self;

    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::SpiceDisplayPrimary, Self> {
        StashMut(&mut self.0, self)
    }
}

impl Default for DisplayPrimary {
    fn default() -> Self {
        Self::new()
    }
}
