// Take a look at the license at the top of the repository in the LICENSE file.

use std::convert::TryInto;

use crate::{CursorShape, CursorType};

impl CursorShape {
    pub fn cursor_type(&self) -> Result<CursorType, i32> {
        unsafe { (*self.as_ptr()).type_.try_into() }
    }

    pub fn width(&self) -> i32 {
        unsafe { (*self.as_ptr()).width as _ }
    }

    pub fn height(&self) -> i32 {
        unsafe { (*self.as_ptr()).height as _ }
    }

    pub fn hot_x(&self) -> i32 {
        unsafe { (*self.as_ptr()).hot_spot_x as _ }
    }

    pub fn hot_y(&self) -> i32 {
        unsafe { (*self.as_ptr()).hot_spot_y as _ }
    }

    pub fn data(&self) -> Result<&[u8], String> {
        let ps = match self.cursor_type() {
            Ok(CursorType::Color32) | Ok(CursorType::Alpha) => 4,
            Ok(CursorType::Color16) => 2,
            _ => return Err(format!("Unhandled cursor type: {:?}", self.cursor_type())),
        };

        unsafe {
            Ok(std::slice::from_raw_parts(
                (*self.as_ptr()).data as _,
                (self.width() * self.height() * ps) as _,
            ))
        }
    }
}
