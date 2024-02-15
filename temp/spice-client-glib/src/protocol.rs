// Take a look at the license at the top of the repository in the LICENSE file.

use bitflags::bitflags;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ChannelType {
    Invalid = 0,
    Main = 1,
    Display = 2,
    Inputs = 3,
    Cursor = 4,
    Playback = 5,
    Record = 6,
    Tunnel = 7,
    Smartcard = 8,
    Usbredir = 9,
    Port = 10,
    Webdav = 11,
}

impl TryFrom<i32> for ChannelType {
    type Error = i32;

    fn try_from(v: i32) -> Result<Self, i32> {
        match v {
            x if x == ChannelType::Invalid as i32 => Ok(ChannelType::Invalid),
            x if x == ChannelType::Main as i32 => Ok(ChannelType::Main),
            x if x == ChannelType::Display as i32 => Ok(ChannelType::Display),
            x if x == ChannelType::Inputs as i32 => Ok(ChannelType::Inputs),
            x if x == ChannelType::Cursor as i32 => Ok(ChannelType::Cursor),
            x if x == ChannelType::Playback as i32 => Ok(ChannelType::Playback),
            x if x == ChannelType::Record as i32 => Ok(ChannelType::Record),
            x if x == ChannelType::Tunnel as i32 => Ok(ChannelType::Tunnel),
            x if x == ChannelType::Smartcard as i32 => Ok(ChannelType::Smartcard),
            x if x == ChannelType::Usbredir as i32 => Ok(ChannelType::Usbredir),
            x if x == ChannelType::Port as i32 => Ok(ChannelType::Port),
            x if x == ChannelType::Webdav as i32 => Ok(ChannelType::Webdav),
            x => Err(x),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CursorType {
    Alpha = 0,
    Mono = 1,
    Color4 = 2,
    Color8 = 3,
    Color16 = 4,
    Color24 = 5,
    Color32 = 6,
}

impl TryFrom<i32> for CursorType {
    type Error = i32;

    fn try_from(v: i32) -> Result<Self, i32> {
        match v {
            x if x == CursorType::Alpha as i32 => Ok(CursorType::Alpha),
            x if x == CursorType::Mono as i32 => Ok(CursorType::Mono),
            x if x == CursorType::Color4 as i32 => Ok(CursorType::Color4),
            x if x == CursorType::Color8 as i32 => Ok(CursorType::Color8),
            x if x == CursorType::Color16 as i32 => Ok(CursorType::Color16),
            x if x == CursorType::Color24 as i32 => Ok(CursorType::Color24),
            x if x == CursorType::Color32 as i32 => Ok(CursorType::Color32),
            x => Err(x),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SurfaceFormat {
    Invalid = 0,
    _1A = 1,
    _8A = 8,
    _16_555 = 16,
    _32XRGB = 32,
    _16_565 = 80,
    _32ARGB = 96,
}

impl TryFrom<i32> for SurfaceFormat {
    type Error = i32;

    fn try_from(v: i32) -> Result<Self, i32> {
        match v {
            x if x == SurfaceFormat::Invalid as i32 => Ok(SurfaceFormat::Invalid),
            x if x == SurfaceFormat::_1A as i32 => Ok(SurfaceFormat::_1A),
            x if x == SurfaceFormat::_8A as i32 => Ok(SurfaceFormat::_8A),
            x if x == SurfaceFormat::_16_555 as i32 => Ok(SurfaceFormat::_16_555),
            x if x == SurfaceFormat::_32XRGB as i32 => Ok(SurfaceFormat::_32XRGB),
            x if x == SurfaceFormat::_16_565 as i32 => Ok(SurfaceFormat::_16_565),
            x if x == SurfaceFormat::_32ARGB as i32 => Ok(SurfaceFormat::_32ARGB),
            x => Err(x),
        }
    }
}

bitflags! {
    pub struct KeyboardModifiersFlags: i32 {
        const NONE = 0;
        const SCROLL = 0b00000001;
        const NUM = 0b00000010;
        const CAPS = 0b00000100;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum MouseButton {
    Invalid = 0,
    Left = 1,
    Middle = 2,
    Right = 3,
    Up = 4,
    Down = 5,
    Side = 6,
    Extra = 7,
}

impl TryFrom<i32> for MouseButton {
    type Error = i32;

    fn try_from(v: i32) -> Result<Self, i32> {
        match v {
            x if x == MouseButton::Invalid as i32 => Ok(MouseButton::Invalid),
            x if x == MouseButton::Left as i32 => Ok(MouseButton::Left),
            x if x == MouseButton::Middle as i32 => Ok(MouseButton::Middle),
            x if x == MouseButton::Right as i32 => Ok(MouseButton::Right),
            x if x == MouseButton::Up as i32 => Ok(MouseButton::Up),
            x if x == MouseButton::Down as i32 => Ok(MouseButton::Down),
            x if x == MouseButton::Side as i32 => Ok(MouseButton::Side),
            x if x == MouseButton::Extra as i32 => Ok(MouseButton::Extra),
            x => Err(x),
        }
    }
}

bitflags! {
    pub struct MouseButtonMask: i32 {
        const NONE = 0;
        const LEFT = 0b00000001;
        const MIDDLE = 0b00000010;
        const RIGHT = 0b00000100;
        const UP = 0b00001000;
        const DOWN = 0b00010000;
        const SIDE = 0b00100000;
        const EXTRA = 0b01000000;
    }
}

bitflags! {
    pub struct MouseMode: i32 {
        const NONE = 0;
        const SERVER = 0b00000001;
        const CLIENT = 0b00000010;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ClipboardFormat {
    None = 0,
    Utf8 = 1,
    Png = 2,
    Bmp = 3,
    Tiff = 4,
    Jpg = 5,
    FileList = 6,
}

impl TryFrom<i32> for ClipboardFormat {
    type Error = i32;

    fn try_from(v: i32) -> Result<Self, i32> {
        match v {
            x if x == ClipboardFormat::None as i32 => Ok(ClipboardFormat::None),
            x if x == ClipboardFormat::Utf8 as i32 => Ok(ClipboardFormat::Utf8),
            x if x == ClipboardFormat::Png as i32 => Ok(ClipboardFormat::Png),
            x if x == ClipboardFormat::Bmp as i32 => Ok(ClipboardFormat::Bmp),
            x if x == ClipboardFormat::Tiff as i32 => Ok(ClipboardFormat::Tiff),
            x if x == ClipboardFormat::Jpg as i32 => Ok(ClipboardFormat::Jpg),
            x if x == ClipboardFormat::FileList as i32 => Ok(ClipboardFormat::FileList),
            x => Err(x),
        }
    }
}
