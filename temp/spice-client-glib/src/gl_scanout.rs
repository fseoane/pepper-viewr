// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(unix)]
use std::os::unix::io::{IntoRawFd, RawFd};

use glib::translate::*;

use crate::GlScanout;

impl GlScanout {
    #[cfg(unix)]
    pub fn fd(&self) -> RawFd {
        unsafe { (*self.as_ptr()).fd }
    }

    pub fn width(&self) -> u32 {
        unsafe { (*self.as_ptr()).width }
    }

    pub fn height(&self) -> u32 {
        unsafe { (*self.as_ptr()).height }
    }

    pub fn stride(&self) -> u32 {
        unsafe { (*self.as_ptr()).stride }
    }

    pub fn format(&self) -> u32 {
        unsafe { (*self.as_ptr()).format }
    }

    pub fn y0_top(&self) -> bool {
        unsafe { from_glib((*self.as_ptr()).y0top) }
    }
}

#[cfg(unix)]
impl IntoRawFd for GlScanout {
    fn into_raw_fd(mut self) -> RawFd {
        unsafe {
            let ffi = &mut *self.to_glib_none_mut().0;
            // close is unchecked in spice-gtk
            std::mem::replace(&mut ffi.fd, -1)
        }
    }
}
