// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::{prelude::*, translate::*};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "SpiceChannelEvent")]
pub enum ChannelEvent {
    #[doc(alias = "SPICE_CHANNEL_NONE")]
    None,
    #[doc(alias = "SPICE_CHANNEL_OPENED")]
    Opened,
    #[doc(alias = "SPICE_CHANNEL_SWITCHING")]
    Switching,
    #[doc(alias = "SPICE_CHANNEL_CLOSED")]
    Closed,
    #[doc(alias = "SPICE_CHANNEL_ERROR_CONNECT")]
    ErrorConnect,
    #[doc(alias = "SPICE_CHANNEL_ERROR_TLS")]
    ErrorTls,
    #[doc(alias = "SPICE_CHANNEL_ERROR_LINK")]
    ErrorLink,
    #[doc(alias = "SPICE_CHANNEL_ERROR_AUTH")]
    ErrorAuth,
    #[doc(alias = "SPICE_CHANNEL_ERROR_IO")]
    ErrorIo,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ChannelEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ChannelEvent::{}",
            match *self {
                Self::None => "None",
                Self::Opened => "Opened",
                Self::Switching => "Switching",
                Self::Closed => "Closed",
                Self::ErrorConnect => "ErrorConnect",
                Self::ErrorTls => "ErrorTls",
                Self::ErrorLink => "ErrorLink",
                Self::ErrorAuth => "ErrorAuth",
                Self::ErrorIo => "ErrorIo",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for ChannelEvent {
    type GlibType = ffi::SpiceChannelEvent;

    #[inline]
    fn into_glib(self) -> ffi::SpiceChannelEvent {
        match self {
            Self::None => ffi::SPICE_CHANNEL_NONE,
            Self::Opened => ffi::SPICE_CHANNEL_OPENED,
            Self::Switching => ffi::SPICE_CHANNEL_SWITCHING,
            Self::Closed => ffi::SPICE_CHANNEL_CLOSED,
            Self::ErrorConnect => ffi::SPICE_CHANNEL_ERROR_CONNECT,
            Self::ErrorTls => ffi::SPICE_CHANNEL_ERROR_TLS,
            Self::ErrorLink => ffi::SPICE_CHANNEL_ERROR_LINK,
            Self::ErrorAuth => ffi::SPICE_CHANNEL_ERROR_AUTH,
            Self::ErrorIo => ffi::SPICE_CHANNEL_ERROR_IO,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::SpiceChannelEvent> for ChannelEvent {
    #[inline]
    unsafe fn from_glib(value: ffi::SpiceChannelEvent) -> Self {
        skip_assert_initialized!();

        match value {
            ffi::SPICE_CHANNEL_NONE => Self::None,
            ffi::SPICE_CHANNEL_OPENED => Self::Opened,
            ffi::SPICE_CHANNEL_SWITCHING => Self::Switching,
            ffi::SPICE_CHANNEL_CLOSED => Self::Closed,
            ffi::SPICE_CHANNEL_ERROR_CONNECT => Self::ErrorConnect,
            ffi::SPICE_CHANNEL_ERROR_TLS => Self::ErrorTls,
            ffi::SPICE_CHANNEL_ERROR_LINK => Self::ErrorLink,
            ffi::SPICE_CHANNEL_ERROR_AUTH => Self::ErrorAuth,
            ffi::SPICE_CHANNEL_ERROR_IO => Self::ErrorIo,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for ChannelEvent {
    #[inline]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::spice_channel_event_get_type()) }
    }
}

impl glib::HasParamSpec for ChannelEvent {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
    }
}

impl glib::value::ValueType for ChannelEvent {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for ChannelEvent {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for ChannelEvent {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<ChannelEvent> for glib::Value {
    #[inline]
    fn from(v: ChannelEvent) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "SpiceClientError")]
pub enum ClientError {
    #[doc(alias = "SPICE_CLIENT_ERROR_FAILED")]
    Failed,
    #[doc(alias = "SPICE_CLIENT_ERROR_USB_DEVICE_REJECTED")]
    UsbDeviceRejected,
    #[doc(alias = "SPICE_CLIENT_ERROR_USB_DEVICE_LOST")]
    UsbDeviceLost,
    #[doc(alias = "SPICE_CLIENT_ERROR_AUTH_NEEDS_PASSWORD")]
    AuthNeedsPassword,
    #[doc(alias = "SPICE_CLIENT_ERROR_AUTH_NEEDS_USERNAME")]
    AuthNeedsUsername,
    #[doc(alias = "SPICE_CLIENT_ERROR_AUTH_NEEDS_PASSWORD_AND_USERNAME")]
    AuthNeedsPasswordAndUsername,
    #[doc(alias = "SPICE_CLIENT_ERROR_USB_SERVICE")]
    UsbService,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ClientError::{}",
            match *self {
                Self::Failed => "Failed",
                Self::UsbDeviceRejected => "UsbDeviceRejected",
                Self::UsbDeviceLost => "UsbDeviceLost",
                Self::AuthNeedsPassword => "AuthNeedsPassword",
                Self::AuthNeedsUsername => "AuthNeedsUsername",
                Self::AuthNeedsPasswordAndUsername => "AuthNeedsPasswordAndUsername",
                Self::UsbService => "UsbService",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for ClientError {
    type GlibType = ffi::SpiceClientError;

    #[inline]
    fn into_glib(self) -> ffi::SpiceClientError {
        match self {
            Self::Failed => ffi::SPICE_CLIENT_ERROR_FAILED,
            Self::UsbDeviceRejected => ffi::SPICE_CLIENT_ERROR_USB_DEVICE_REJECTED,
            Self::UsbDeviceLost => ffi::SPICE_CLIENT_ERROR_USB_DEVICE_LOST,
            Self::AuthNeedsPassword => ffi::SPICE_CLIENT_ERROR_AUTH_NEEDS_PASSWORD,
            Self::AuthNeedsUsername => ffi::SPICE_CLIENT_ERROR_AUTH_NEEDS_USERNAME,
            Self::AuthNeedsPasswordAndUsername => {
                ffi::SPICE_CLIENT_ERROR_AUTH_NEEDS_PASSWORD_AND_USERNAME
            }
            Self::UsbService => ffi::SPICE_CLIENT_ERROR_USB_SERVICE,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::SpiceClientError> for ClientError {
    #[inline]
    unsafe fn from_glib(value: ffi::SpiceClientError) -> Self {
        skip_assert_initialized!();

        match value {
            ffi::SPICE_CLIENT_ERROR_FAILED => Self::Failed,
            ffi::SPICE_CLIENT_ERROR_USB_DEVICE_REJECTED => Self::UsbDeviceRejected,
            ffi::SPICE_CLIENT_ERROR_USB_DEVICE_LOST => Self::UsbDeviceLost,
            ffi::SPICE_CLIENT_ERROR_AUTH_NEEDS_PASSWORD => Self::AuthNeedsPassword,
            ffi::SPICE_CLIENT_ERROR_AUTH_NEEDS_USERNAME => Self::AuthNeedsUsername,
            ffi::SPICE_CLIENT_ERROR_AUTH_NEEDS_PASSWORD_AND_USERNAME => {
                Self::AuthNeedsPasswordAndUsername
            }
            ffi::SPICE_CLIENT_ERROR_USB_SERVICE => Self::UsbService,
            value => Self::__Unknown(value),
        }
    }
}

impl glib::error::ErrorDomain for ClientError {
    #[inline]
    fn domain() -> glib::Quark {
        skip_assert_initialized!();

        unsafe { from_glib(ffi::spice_client_error_quark()) }
    }

    #[inline]
    fn code(self) -> i32 {
        self.into_glib()
    }

    #[inline]
    #[allow(clippy::match_single_binding)]
    fn from(code: i32) -> Option<Self> {
        skip_assert_initialized!();
        match unsafe { from_glib(code) } {
            Self::__Unknown(_) => Some(Self::Failed),
            value => Some(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "SpiceQmpPortVmAction")]
pub enum QmpPortVmAction {
    #[doc(alias = "SPICE_QMP_PORT_VM_ACTION_QUIT")]
    Quit,
    #[doc(alias = "SPICE_QMP_PORT_VM_ACTION_RESET")]
    Reset,
    #[doc(alias = "SPICE_QMP_PORT_VM_ACTION_POWER_DOWN")]
    PowerDown,
    #[doc(alias = "SPICE_QMP_PORT_VM_ACTION_PAUSE")]
    Pause,
    #[doc(alias = "SPICE_QMP_PORT_VM_ACTION_CONTINUE")]
    Continue,
    #[doc(alias = "SPICE_QMP_PORT_VM_ACTION_LAST")]
    Last,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for QmpPortVmAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "QmpPortVmAction::{}",
            match *self {
                Self::Quit => "Quit",
                Self::Reset => "Reset",
                Self::PowerDown => "PowerDown",
                Self::Pause => "Pause",
                Self::Continue => "Continue",
                Self::Last => "Last",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for QmpPortVmAction {
    type GlibType = ffi::SpiceQmpPortVmAction;

    #[inline]
    fn into_glib(self) -> ffi::SpiceQmpPortVmAction {
        match self {
            Self::Quit => ffi::SPICE_QMP_PORT_VM_ACTION_QUIT,
            Self::Reset => ffi::SPICE_QMP_PORT_VM_ACTION_RESET,
            Self::PowerDown => ffi::SPICE_QMP_PORT_VM_ACTION_POWER_DOWN,
            Self::Pause => ffi::SPICE_QMP_PORT_VM_ACTION_PAUSE,
            Self::Continue => ffi::SPICE_QMP_PORT_VM_ACTION_CONTINUE,
            Self::Last => ffi::SPICE_QMP_PORT_VM_ACTION_LAST,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::SpiceQmpPortVmAction> for QmpPortVmAction {
    #[inline]
    unsafe fn from_glib(value: ffi::SpiceQmpPortVmAction) -> Self {
        skip_assert_initialized!();

        match value {
            ffi::SPICE_QMP_PORT_VM_ACTION_QUIT => Self::Quit,
            ffi::SPICE_QMP_PORT_VM_ACTION_RESET => Self::Reset,
            ffi::SPICE_QMP_PORT_VM_ACTION_POWER_DOWN => Self::PowerDown,
            ffi::SPICE_QMP_PORT_VM_ACTION_PAUSE => Self::Pause,
            ffi::SPICE_QMP_PORT_VM_ACTION_CONTINUE => Self::Continue,
            ffi::SPICE_QMP_PORT_VM_ACTION_LAST => Self::Last,
            value => Self::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "SpiceSessionMigration")]
pub enum SessionMigration {
    #[doc(alias = "SPICE_SESSION_MIGRATION_NONE")]
    None,
    #[doc(alias = "SPICE_SESSION_MIGRATION_SWITCHING")]
    Switching,
    #[doc(alias = "SPICE_SESSION_MIGRATION_MIGRATING")]
    Migrating,
    #[doc(alias = "SPICE_SESSION_MIGRATION_CONNECTING")]
    Connecting,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for SessionMigration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SessionMigration::{}",
            match *self {
                Self::None => "None",
                Self::Switching => "Switching",
                Self::Migrating => "Migrating",
                Self::Connecting => "Connecting",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for SessionMigration {
    type GlibType = ffi::SpiceSessionMigration;

    #[inline]
    fn into_glib(self) -> ffi::SpiceSessionMigration {
        match self {
            Self::None => ffi::SPICE_SESSION_MIGRATION_NONE,
            Self::Switching => ffi::SPICE_SESSION_MIGRATION_SWITCHING,
            Self::Migrating => ffi::SPICE_SESSION_MIGRATION_MIGRATING,
            Self::Connecting => ffi::SPICE_SESSION_MIGRATION_CONNECTING,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::SpiceSessionMigration> for SessionMigration {
    #[inline]
    unsafe fn from_glib(value: ffi::SpiceSessionMigration) -> Self {
        skip_assert_initialized!();

        match value {
            ffi::SPICE_SESSION_MIGRATION_NONE => Self::None,
            ffi::SPICE_SESSION_MIGRATION_SWITCHING => Self::Switching,
            ffi::SPICE_SESSION_MIGRATION_MIGRATING => Self::Migrating,
            ffi::SPICE_SESSION_MIGRATION_CONNECTING => Self::Connecting,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for SessionMigration {
    #[inline]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::spice_session_migration_get_type()) }
    }
}

impl glib::HasParamSpec for SessionMigration {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
    }
}

impl glib::value::ValueType for SessionMigration {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for SessionMigration {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for SessionMigration {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<SessionMigration> for glib::Value {
    #[inline]
    fn from(v: SessionMigration) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}
