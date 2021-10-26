// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The cookie for the reply to a `DPMS::GetVersion` request.
///
/// Pass this cookie to [`xcb_dpms_get_version_reply`] to retrieve the reply.
///
/// [`xcb_dpms_get_version_reply`]: XcbDpms::xcb_dpms_get_version_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_get_version_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_dpms_get_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DPMS::GetVersion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDpms::xcb_dpms_id()`], then the type of the request is
/// [`xcb_dpms_get_version_request_t`].
pub const XCB_DPMS_GET_VERSION: u8 = 0i32 as u8;

/// The `DPMS::GetVersion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_get_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client_major_version: u16,
    pub client_minor_version: u16,
}

impl Default for xcb_dpms_get_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `DPMS::GetVersion` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_get_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major_version: u16,
    pub server_minor_version: u16,
}

impl Default for xcb_dpms_get_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `DPMS::Capable` request.
///
/// Pass this cookie to [`xcb_dpms_capable_reply`] to retrieve the reply.
///
/// [`xcb_dpms_capable_reply`]: XcbDpms::xcb_dpms_capable_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_capable_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_dpms_capable_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DPMS::Capable` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDpms::xcb_dpms_id()`], then the type of the request is
/// [`xcb_dpms_capable_request_t`].
pub const XCB_DPMS_CAPABLE: u8 = 1i32 as u8;

/// The `DPMS::Capable` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_capable_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_dpms_capable_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `DPMS::Capable` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_capable_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub capable: u8,
    pub pad1: [u8; 23],
}

impl Default for xcb_dpms_capable_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `DPMS::GetTimeouts` request.
///
/// Pass this cookie to [`xcb_dpms_get_timeouts_reply`] to retrieve the reply.
///
/// [`xcb_dpms_get_timeouts_reply`]: XcbDpms::xcb_dpms_get_timeouts_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_get_timeouts_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_dpms_get_timeouts_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DPMS::GetTimeouts` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDpms::xcb_dpms_id()`], then the type of the request is
/// [`xcb_dpms_get_timeouts_request_t`].
pub const XCB_DPMS_GET_TIMEOUTS: u8 = 2i32 as u8;

/// The `DPMS::GetTimeouts` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_get_timeouts_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_dpms_get_timeouts_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `DPMS::GetTimeouts` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_get_timeouts_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub standby_timeout: u16,
    pub suspend_timeout: u16,
    pub off_timeout: u16,
    pub pad1: [u8; 18],
}

impl Default for xcb_dpms_get_timeouts_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DPMS::SetTimeouts` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDpms::xcb_dpms_id()`], then the type of the request is
/// [`xcb_dpms_set_timeouts_request_t`].
pub const XCB_DPMS_SET_TIMEOUTS: u8 = 3i32 as u8;

/// The `DPMS::SetTimeouts` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_set_timeouts_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub standby_timeout: u16,
    pub suspend_timeout: u16,
    pub off_timeout: u16,
}

impl Default for xcb_dpms_set_timeouts_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DPMS::Enable` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDpms::xcb_dpms_id()`], then the type of the request is
/// [`xcb_dpms_enable_request_t`].
pub const XCB_DPMS_ENABLE: u8 = 4i32 as u8;

/// The `DPMS::Enable` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_enable_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_dpms_enable_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DPMS::Disable` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDpms::xcb_dpms_id()`], then the type of the request is
/// [`xcb_dpms_disable_request_t`].
pub const XCB_DPMS_DISABLE: u8 = 5i32 as u8;

/// The `DPMS::Disable` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_disable_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_dpms_disable_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `DPMS::DPMSMode` enum.
///
/// This enum has the following variants:
///
/// - [`DPMS::DPMSMode::On`](XCB_DPMS_DPMS_MODE_ON)
/// - [`DPMS::DPMSMode::Standby`](XCB_DPMS_DPMS_MODE_STANDBY)
/// - [`DPMS::DPMSMode::Suspend`](XCB_DPMS_DPMS_MODE_SUSPEND)
/// - [`DPMS::DPMSMode::Off`](XCB_DPMS_DPMS_MODE_OFF)
pub type xcb_dpms_dpms_mode_t = u32;
/// The `DPMS::DPMSMode::On` enum variant.
///
/// This is a variant of [`xcb_dpms_dpms_mode_t`].
pub const XCB_DPMS_DPMS_MODE_ON: xcb_dpms_dpms_mode_t = 0;
/// The `DPMS::DPMSMode::Standby` enum variant.
///
/// This is a variant of [`xcb_dpms_dpms_mode_t`].
pub const XCB_DPMS_DPMS_MODE_STANDBY: xcb_dpms_dpms_mode_t = 1;
/// The `DPMS::DPMSMode::Suspend` enum variant.
///
/// This is a variant of [`xcb_dpms_dpms_mode_t`].
pub const XCB_DPMS_DPMS_MODE_SUSPEND: xcb_dpms_dpms_mode_t = 2;
/// The `DPMS::DPMSMode::Off` enum variant.
///
/// This is a variant of [`xcb_dpms_dpms_mode_t`].
pub const XCB_DPMS_DPMS_MODE_OFF: xcb_dpms_dpms_mode_t = 3;

/// The opcode for `DPMS::ForceLevel` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDpms::xcb_dpms_id()`], then the type of the request is
/// [`xcb_dpms_force_level_request_t`].
pub const XCB_DPMS_FORCE_LEVEL: u8 = 6i32 as u8;

/// The `DPMS::ForceLevel` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_force_level_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub power_level: u16,
}

impl Default for xcb_dpms_force_level_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `DPMS::Info` request.
///
/// Pass this cookie to [`xcb_dpms_info_reply`] to retrieve the reply.
///
/// [`xcb_dpms_info_reply`]: XcbDpms::xcb_dpms_info_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_info_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_dpms_info_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DPMS::Info` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDpms::xcb_dpms_id()`], then the type of the request is
/// [`xcb_dpms_info_request_t`].
pub const XCB_DPMS_INFO: u8 = 7i32 as u8;

/// The `DPMS::Info` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_info_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_dpms_info_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `DPMS::Info` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_info_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub power_level: u16,
    pub state: u8,
    pub pad1: [u8; 21],
}

impl Default for xcb_dpms_info_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_dpms")]
pub(crate) struct XcbDpmsDpms {
    xcb_dpms_id: LazySymbol<*mut xcb_extension_t>,
    xcb_dpms_get_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u16,
            client_minor_version: u16,
        ) -> xcb_dpms_get_version_cookie_t,
    >,
    xcb_dpms_get_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u16,
            client_minor_version: u16,
        ) -> xcb_dpms_get_version_cookie_t,
    >,
    xcb_dpms_get_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dpms_get_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_dpms_get_version_reply_t,
    >,
    xcb_dpms_capable: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_dpms_capable_cookie_t>,
    xcb_dpms_capable_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_dpms_capable_cookie_t>,
    xcb_dpms_capable_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dpms_capable_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_dpms_capable_reply_t,
    >,
    xcb_dpms_get_timeouts:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_dpms_get_timeouts_cookie_t>,
    xcb_dpms_get_timeouts_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_dpms_get_timeouts_cookie_t>,
    xcb_dpms_get_timeouts_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dpms_get_timeouts_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_dpms_get_timeouts_reply_t,
    >,
    xcb_dpms_set_timeouts_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            standby_timeout: u16,
            suspend_timeout: u16,
            off_timeout: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_dpms_set_timeouts: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            standby_timeout: u16,
            suspend_timeout: u16,
            off_timeout: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_dpms_enable_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    xcb_dpms_enable: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    xcb_dpms_disable_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    xcb_dpms_disable: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    xcb_dpms_force_level_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, power_level: u16) -> xcb_void_cookie_t>,
    xcb_dpms_force_level:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, power_level: u16) -> xcb_void_cookie_t>,
    xcb_dpms_info: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_dpms_info_cookie_t>,
    xcb_dpms_info_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_dpms_info_cookie_t>,
    xcb_dpms_info_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dpms_info_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_dpms_info_reply_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self.dpms.$f.get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .dpms
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_dpms")]
impl XcbDpms {
    /// The libxcb identifier of the `DPMS` extension.
    #[inline]
    pub fn xcb_dpms_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_dpms_id) }
    }

    /// Returns `true` iff the symbol `xcb_dpms_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_id(&self) -> bool {
        has_sym!(self, xcb_dpms_id)
    }

    /// Sends a `DPMS::GetVersion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dpms_get_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dpms_get_version_reply`]: Self::xcb_dpms_get_version_reply
    #[inline]
    pub unsafe fn xcb_dpms_get_version(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u16,
        client_minor_version: u16,
    ) -> xcb_dpms_get_version_cookie_t {
        sym!(self, xcb_dpms_get_version)(c, client_major_version, client_minor_version)
    }

    /// Returns `true` iff the symbol `xcb_dpms_get_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_get_version(&self) -> bool {
        has_sym!(self, xcb_dpms_get_version)
    }

    /// Sends a `DPMS::GetVersion` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dpms_get_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dpms_get_version_reply`]: Self::xcb_dpms_get_version_reply
    #[inline]
    pub unsafe fn xcb_dpms_get_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u16,
        client_minor_version: u16,
    ) -> xcb_dpms_get_version_cookie_t {
        sym!(self, xcb_dpms_get_version_unchecked)(c, client_major_version, client_minor_version)
    }

    /// Returns `true` iff the symbol `xcb_dpms_get_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_get_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_dpms_get_version_unchecked)
    }

    /// Waits for the reply to a `DPMS::GetVersion` request.
    #[inline]
    pub unsafe fn xcb_dpms_get_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dpms_get_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dpms_get_version_reply_t {
        sym!(self, xcb_dpms_get_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dpms_get_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_get_version_reply(&self) -> bool {
        has_sym!(self, xcb_dpms_get_version_reply)
    }

    /// Sends a `DPMS::Capable` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dpms_capable_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dpms_capable_reply`]: Self::xcb_dpms_capable_reply
    #[inline]
    pub unsafe fn xcb_dpms_capable(&self, c: *mut xcb_connection_t) -> xcb_dpms_capable_cookie_t {
        sym!(self, xcb_dpms_capable)(c)
    }

    /// Returns `true` iff the symbol `xcb_dpms_capable` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_capable(&self) -> bool {
        has_sym!(self, xcb_dpms_capable)
    }

    /// Sends a `DPMS::Capable` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dpms_capable_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dpms_capable_reply`]: Self::xcb_dpms_capable_reply
    #[inline]
    pub unsafe fn xcb_dpms_capable_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_dpms_capable_cookie_t {
        sym!(self, xcb_dpms_capable_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_dpms_capable_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_capable_unchecked(&self) -> bool {
        has_sym!(self, xcb_dpms_capable_unchecked)
    }

    /// Waits for the reply to a `DPMS::Capable` request.
    #[inline]
    pub unsafe fn xcb_dpms_capable_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dpms_capable_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dpms_capable_reply_t {
        sym!(self, xcb_dpms_capable_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dpms_capable_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_capable_reply(&self) -> bool {
        has_sym!(self, xcb_dpms_capable_reply)
    }

    /// Sends a `DPMS::GetTimeouts` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dpms_get_timeouts_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dpms_get_timeouts_reply`]: Self::xcb_dpms_get_timeouts_reply
    #[inline]
    pub unsafe fn xcb_dpms_get_timeouts(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_dpms_get_timeouts_cookie_t {
        sym!(self, xcb_dpms_get_timeouts)(c)
    }

    /// Returns `true` iff the symbol `xcb_dpms_get_timeouts` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_get_timeouts(&self) -> bool {
        has_sym!(self, xcb_dpms_get_timeouts)
    }

    /// Sends a `DPMS::GetTimeouts` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dpms_get_timeouts_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dpms_get_timeouts_reply`]: Self::xcb_dpms_get_timeouts_reply
    #[inline]
    pub unsafe fn xcb_dpms_get_timeouts_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_dpms_get_timeouts_cookie_t {
        sym!(self, xcb_dpms_get_timeouts_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_dpms_get_timeouts_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_get_timeouts_unchecked(&self) -> bool {
        has_sym!(self, xcb_dpms_get_timeouts_unchecked)
    }

    /// Waits for the reply to a `DPMS::GetTimeouts` request.
    #[inline]
    pub unsafe fn xcb_dpms_get_timeouts_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dpms_get_timeouts_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dpms_get_timeouts_reply_t {
        sym!(self, xcb_dpms_get_timeouts_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dpms_get_timeouts_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_get_timeouts_reply(&self) -> bool {
        has_sym!(self, xcb_dpms_get_timeouts_reply)
    }

    /// Sends a `DPMS::SetTimeouts` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_dpms_set_timeouts_checked(
        &self,
        c: *mut xcb_connection_t,
        standby_timeout: u16,
        suspend_timeout: u16,
        off_timeout: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dpms_set_timeouts_checked)(c, standby_timeout, suspend_timeout, off_timeout)
    }

    /// Returns `true` iff the symbol `xcb_dpms_set_timeouts_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_set_timeouts_checked(&self) -> bool {
        has_sym!(self, xcb_dpms_set_timeouts_checked)
    }

    /// Sends a `DPMS::SetTimeouts` request (unchecked).
    #[inline]
    pub unsafe fn xcb_dpms_set_timeouts(
        &self,
        c: *mut xcb_connection_t,
        standby_timeout: u16,
        suspend_timeout: u16,
        off_timeout: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dpms_set_timeouts)(c, standby_timeout, suspend_timeout, off_timeout)
    }

    /// Returns `true` iff the symbol `xcb_dpms_set_timeouts` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_set_timeouts(&self) -> bool {
        has_sym!(self, xcb_dpms_set_timeouts)
    }

    /// Sends a `DPMS::Enable` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_dpms_enable_checked(&self, c: *mut xcb_connection_t) -> xcb_void_cookie_t {
        sym!(self, xcb_dpms_enable_checked)(c)
    }

    /// Returns `true` iff the symbol `xcb_dpms_enable_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_enable_checked(&self) -> bool {
        has_sym!(self, xcb_dpms_enable_checked)
    }

    /// Sends a `DPMS::Enable` request (unchecked).
    #[inline]
    pub unsafe fn xcb_dpms_enable(&self, c: *mut xcb_connection_t) -> xcb_void_cookie_t {
        sym!(self, xcb_dpms_enable)(c)
    }

    /// Returns `true` iff the symbol `xcb_dpms_enable` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_enable(&self) -> bool {
        has_sym!(self, xcb_dpms_enable)
    }

    /// Sends a `DPMS::Disable` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_dpms_disable_checked(&self, c: *mut xcb_connection_t) -> xcb_void_cookie_t {
        sym!(self, xcb_dpms_disable_checked)(c)
    }

    /// Returns `true` iff the symbol `xcb_dpms_disable_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_disable_checked(&self) -> bool {
        has_sym!(self, xcb_dpms_disable_checked)
    }

    /// Sends a `DPMS::Disable` request (unchecked).
    #[inline]
    pub unsafe fn xcb_dpms_disable(&self, c: *mut xcb_connection_t) -> xcb_void_cookie_t {
        sym!(self, xcb_dpms_disable)(c)
    }

    /// Returns `true` iff the symbol `xcb_dpms_disable` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_disable(&self) -> bool {
        has_sym!(self, xcb_dpms_disable)
    }

    /// Sends a `DPMS::ForceLevel` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_dpms_force_level_checked(
        &self,
        c: *mut xcb_connection_t,
        power_level: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dpms_force_level_checked)(c, power_level)
    }

    /// Returns `true` iff the symbol `xcb_dpms_force_level_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_force_level_checked(&self) -> bool {
        has_sym!(self, xcb_dpms_force_level_checked)
    }

    /// Sends a `DPMS::ForceLevel` request (unchecked).
    #[inline]
    pub unsafe fn xcb_dpms_force_level(
        &self,
        c: *mut xcb_connection_t,
        power_level: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dpms_force_level)(c, power_level)
    }

    /// Returns `true` iff the symbol `xcb_dpms_force_level` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_force_level(&self) -> bool {
        has_sym!(self, xcb_dpms_force_level)
    }

    /// Sends a `DPMS::Info` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dpms_info_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dpms_info_reply`]: Self::xcb_dpms_info_reply
    #[inline]
    pub unsafe fn xcb_dpms_info(&self, c: *mut xcb_connection_t) -> xcb_dpms_info_cookie_t {
        sym!(self, xcb_dpms_info)(c)
    }

    /// Returns `true` iff the symbol `xcb_dpms_info` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_info(&self) -> bool {
        has_sym!(self, xcb_dpms_info)
    }

    /// Sends a `DPMS::Info` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dpms_info_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dpms_info_reply`]: Self::xcb_dpms_info_reply
    #[inline]
    pub unsafe fn xcb_dpms_info_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_dpms_info_cookie_t {
        sym!(self, xcb_dpms_info_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_dpms_info_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_info_unchecked(&self) -> bool {
        has_sym!(self, xcb_dpms_info_unchecked)
    }

    /// Waits for the reply to a `DPMS::Info` request.
    #[inline]
    pub unsafe fn xcb_dpms_info_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dpms_info_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dpms_info_reply_t {
        sym!(self, xcb_dpms_info_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dpms_info_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_info_reply(&self) -> bool {
        has_sym!(self, xcb_dpms_info_reply)
    }
}

#[cfg(feature = "xcb_dpms")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbDpms::load().unwrap() };
        assert!(lib.has_xcb_dpms_id());
        assert!(lib.has_xcb_dpms_get_version());
        assert!(lib.has_xcb_dpms_get_version_unchecked());
        assert!(lib.has_xcb_dpms_get_version_reply());
        assert!(lib.has_xcb_dpms_capable());
        assert!(lib.has_xcb_dpms_capable_unchecked());
        assert!(lib.has_xcb_dpms_capable_reply());
        assert!(lib.has_xcb_dpms_get_timeouts());
        assert!(lib.has_xcb_dpms_get_timeouts_unchecked());
        assert!(lib.has_xcb_dpms_get_timeouts_reply());
        assert!(lib.has_xcb_dpms_set_timeouts_checked());
        assert!(lib.has_xcb_dpms_set_timeouts());
        assert!(lib.has_xcb_dpms_enable_checked());
        assert!(lib.has_xcb_dpms_enable());
        assert!(lib.has_xcb_dpms_disable_checked());
        assert!(lib.has_xcb_dpms_disable());
        assert!(lib.has_xcb_dpms_force_level_checked());
        assert!(lib.has_xcb_dpms_force_level());
        assert!(lib.has_xcb_dpms_info());
        assert!(lib.has_xcb_dpms_info_unchecked());
        assert!(lib.has_xcb_dpms_info_reply());
    }
}
