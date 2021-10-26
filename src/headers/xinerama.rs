// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The `Xinerama::ScreenInfo` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_screen_info_t {
    pub x_org: i16,
    pub y_org: i16,
    pub width: u16,
    pub height: u16,
}

impl Default for xcb_xinerama_screen_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Xinerama::ScreenInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_screen_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xinerama_screen_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xinerama_screen_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Xinerama::QueryVersion` request.
///
/// Pass this cookie to [`xcb_xinerama_query_version_reply`] to retrieve the reply.
///
/// [`xcb_xinerama_query_version_reply`]: XcbXinerama::xcb_xinerama_query_version_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_query_version_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xinerama_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xinerama::QueryVersion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinerama::xcb_xinerama_id()`], then the type of the request is
/// [`xcb_xinerama_query_version_request_t`].
pub const XCB_XINERAMA_QUERY_VERSION: u8 = 0i32 as u8;

/// The `Xinerama::QueryVersion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub major: u8,
    pub minor: u8,
}

impl Default for xcb_xinerama_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xinerama::QueryVersion` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major: u16,
    pub minor: u16,
}

impl Default for xcb_xinerama_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Xinerama::GetState` request.
///
/// Pass this cookie to [`xcb_xinerama_get_state_reply`] to retrieve the reply.
///
/// [`xcb_xinerama_get_state_reply`]: XcbXinerama::xcb_xinerama_get_state_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_get_state_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xinerama_get_state_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xinerama::GetState` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinerama::xcb_xinerama_id()`], then the type of the request is
/// [`xcb_xinerama_get_state_request_t`].
pub const XCB_XINERAMA_GET_STATE: u8 = 1i32 as u8;

/// The `Xinerama::GetState` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_get_state_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_xinerama_get_state_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xinerama::GetState` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_get_state_reply_t {
    pub response_type: u8,
    pub state: u8,
    pub sequence: u16,
    pub length: u32,
    pub window: xcb_window_t,
}

impl Default for xcb_xinerama_get_state_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Xinerama::GetScreenCount` request.
///
/// Pass this cookie to [`xcb_xinerama_get_screen_count_reply`] to retrieve the reply.
///
/// [`xcb_xinerama_get_screen_count_reply`]: XcbXinerama::xcb_xinerama_get_screen_count_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_get_screen_count_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xinerama_get_screen_count_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xinerama::GetScreenCount` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinerama::xcb_xinerama_id()`], then the type of the request is
/// [`xcb_xinerama_get_screen_count_request_t`].
pub const XCB_XINERAMA_GET_SCREEN_COUNT: u8 = 2i32 as u8;

/// The `Xinerama::GetScreenCount` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_get_screen_count_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_xinerama_get_screen_count_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xinerama::GetScreenCount` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_get_screen_count_reply_t {
    pub response_type: u8,
    pub screen_count: u8,
    pub sequence: u16,
    pub length: u32,
    pub window: xcb_window_t,
}

impl Default for xcb_xinerama_get_screen_count_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Xinerama::GetScreenSize` request.
///
/// Pass this cookie to [`xcb_xinerama_get_screen_size_reply`] to retrieve the reply.
///
/// [`xcb_xinerama_get_screen_size_reply`]: XcbXinerama::xcb_xinerama_get_screen_size_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_get_screen_size_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xinerama_get_screen_size_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xinerama::GetScreenSize` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinerama::xcb_xinerama_id()`], then the type of the request is
/// [`xcb_xinerama_get_screen_size_request_t`].
pub const XCB_XINERAMA_GET_SCREEN_SIZE: u8 = 3i32 as u8;

/// The `Xinerama::GetScreenSize` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_get_screen_size_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub screen: u32,
}

impl Default for xcb_xinerama_get_screen_size_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xinerama::GetScreenSize` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_get_screen_size_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: u32,
    pub height: u32,
    pub window: xcb_window_t,
    pub screen: u32,
}

impl Default for xcb_xinerama_get_screen_size_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Xinerama::IsActive` request.
///
/// Pass this cookie to [`xcb_xinerama_is_active_reply`] to retrieve the reply.
///
/// [`xcb_xinerama_is_active_reply`]: XcbXinerama::xcb_xinerama_is_active_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_is_active_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xinerama_is_active_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xinerama::IsActive` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinerama::xcb_xinerama_id()`], then the type of the request is
/// [`xcb_xinerama_is_active_request_t`].
pub const XCB_XINERAMA_IS_ACTIVE: u8 = 4i32 as u8;

/// The `Xinerama::IsActive` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_is_active_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_xinerama_is_active_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xinerama::IsActive` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_is_active_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub state: u32,
}

impl Default for xcb_xinerama_is_active_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Xinerama::QueryScreens` request.
///
/// Pass this cookie to [`xcb_xinerama_query_screens_reply`] to retrieve the reply.
///
/// [`xcb_xinerama_query_screens_reply`]: XcbXinerama::xcb_xinerama_query_screens_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_query_screens_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xinerama_query_screens_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xinerama::QueryScreens` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinerama::xcb_xinerama_id()`], then the type of the request is
/// [`xcb_xinerama_query_screens_request_t`].
pub const XCB_XINERAMA_QUERY_SCREENS: u8 = 5i32 as u8;

/// The `Xinerama::QueryScreens` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_query_screens_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_xinerama_query_screens_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xinerama::QueryScreens` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `screen_info`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_query_screens_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub number: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_xinerama_query_screens_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_xinerama")]
pub(crate) struct XcbXineramaXinerama {
    xcb_xinerama_id: LazySymbol<*mut xcb_extension_t>,
    xcb_xinerama_screen_info_next:
        LazySymbol<unsafe fn(i: *mut xcb_xinerama_screen_info_iterator_t)>,
    xcb_xinerama_screen_info_end:
        LazySymbol<unsafe fn(i: xcb_xinerama_screen_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xinerama_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major: u8,
            minor: u8,
        ) -> xcb_xinerama_query_version_cookie_t,
    >,
    xcb_xinerama_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major: u8,
            minor: u8,
        ) -> xcb_xinerama_query_version_cookie_t,
    >,
    xcb_xinerama_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xinerama_query_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xinerama_query_version_reply_t,
    >,
    xcb_xinerama_get_state: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_xinerama_get_state_cookie_t,
    >,
    xcb_xinerama_get_state_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_xinerama_get_state_cookie_t,
    >,
    xcb_xinerama_get_state_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xinerama_get_state_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xinerama_get_state_reply_t,
    >,
    xcb_xinerama_get_screen_count: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_xinerama_get_screen_count_cookie_t,
    >,
    xcb_xinerama_get_screen_count_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_xinerama_get_screen_count_cookie_t,
    >,
    xcb_xinerama_get_screen_count_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xinerama_get_screen_count_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xinerama_get_screen_count_reply_t,
    >,
    xcb_xinerama_get_screen_size: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            screen: u32,
        ) -> xcb_xinerama_get_screen_size_cookie_t,
    >,
    xcb_xinerama_get_screen_size_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            screen: u32,
        ) -> xcb_xinerama_get_screen_size_cookie_t,
    >,
    xcb_xinerama_get_screen_size_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xinerama_get_screen_size_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xinerama_get_screen_size_reply_t,
    >,
    xcb_xinerama_is_active:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xinerama_is_active_cookie_t>,
    xcb_xinerama_is_active_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xinerama_is_active_cookie_t>,
    xcb_xinerama_is_active_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xinerama_is_active_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xinerama_is_active_reply_t,
    >,
    xcb_xinerama_query_screens_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xinerama_query_screens:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xinerama_query_screens_cookie_t>,
    xcb_xinerama_query_screens_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xinerama_query_screens_cookie_t>,
    xcb_xinerama_query_screens_screen_info: LazySymbol<
        unsafe fn(r: *const xcb_xinerama_query_screens_reply_t) -> *mut xcb_xinerama_screen_info_t,
    >,
    xcb_xinerama_query_screens_screen_info_length:
        LazySymbol<unsafe fn(r: *const xcb_xinerama_query_screens_reply_t) -> c_int>,
    xcb_xinerama_query_screens_screen_info_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xinerama_query_screens_reply_t,
        ) -> xcb_xinerama_screen_info_iterator_t,
    >,
    xcb_xinerama_query_screens_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xinerama_query_screens_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xinerama_query_screens_reply_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self
            .xinerama
            .$f
            .get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .xinerama
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_xinerama")]
impl XcbXinerama {
    /// The libxcb identifier of the `Xinerama` extension.
    #[inline]
    pub fn xcb_xinerama_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_xinerama_id) }
    }

    /// Returns `true` iff the symbol `xcb_xinerama_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_id(&self) -> bool {
        has_sym!(self, xcb_xinerama_id)
    }

    /// Advances a `xcb_xinerama_screen_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xinerama_screen_info_next(
        &self,
        i: *mut xcb_xinerama_screen_info_iterator_t,
    ) {
        sym!(self, xcb_xinerama_screen_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_screen_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_screen_info_next(&self) -> bool {
        has_sym!(self, xcb_xinerama_screen_info_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xinerama_screen_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xinerama_screen_info_end(
        &self,
        i: xcb_xinerama_screen_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xinerama_screen_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_screen_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_screen_info_end(&self) -> bool {
        has_sym!(self, xcb_xinerama_screen_info_end)
    }

    /// Sends a `Xinerama::QueryVersion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xinerama_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xinerama_query_version_reply`]: Self::xcb_xinerama_query_version_reply
    #[inline]
    pub unsafe fn xcb_xinerama_query_version(
        &self,
        c: *mut xcb_connection_t,
        major: u8,
        minor: u8,
    ) -> xcb_xinerama_query_version_cookie_t {
        sym!(self, xcb_xinerama_query_version)(c, major, minor)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_query_version(&self) -> bool {
        has_sym!(self, xcb_xinerama_query_version)
    }

    /// Sends a `Xinerama::QueryVersion` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xinerama_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xinerama_query_version_reply`]: Self::xcb_xinerama_query_version_reply
    #[inline]
    pub unsafe fn xcb_xinerama_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        major: u8,
        minor: u8,
    ) -> xcb_xinerama_query_version_cookie_t {
        sym!(self, xcb_xinerama_query_version_unchecked)(c, major, minor)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_query_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_xinerama_query_version_unchecked)
    }

    /// Waits for the reply to a `Xinerama::QueryVersion` request.
    #[inline]
    pub unsafe fn xcb_xinerama_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xinerama_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xinerama_query_version_reply_t {
        sym!(self, xcb_xinerama_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_xinerama_query_version_reply)
    }

    /// Sends a `Xinerama::GetState` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xinerama_get_state_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xinerama_get_state_reply`]: Self::xcb_xinerama_get_state_reply
    #[inline]
    pub unsafe fn xcb_xinerama_get_state(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_xinerama_get_state_cookie_t {
        sym!(self, xcb_xinerama_get_state)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_get_state` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_get_state(&self) -> bool {
        has_sym!(self, xcb_xinerama_get_state)
    }

    /// Sends a `Xinerama::GetState` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xinerama_get_state_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xinerama_get_state_reply`]: Self::xcb_xinerama_get_state_reply
    #[inline]
    pub unsafe fn xcb_xinerama_get_state_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_xinerama_get_state_cookie_t {
        sym!(self, xcb_xinerama_get_state_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_get_state_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_get_state_unchecked(&self) -> bool {
        has_sym!(self, xcb_xinerama_get_state_unchecked)
    }

    /// Waits for the reply to a `Xinerama::GetState` request.
    #[inline]
    pub unsafe fn xcb_xinerama_get_state_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xinerama_get_state_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xinerama_get_state_reply_t {
        sym!(self, xcb_xinerama_get_state_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_get_state_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_get_state_reply(&self) -> bool {
        has_sym!(self, xcb_xinerama_get_state_reply)
    }

    /// Sends a `Xinerama::GetScreenCount` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xinerama_get_screen_count_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xinerama_get_screen_count_reply`]: Self::xcb_xinerama_get_screen_count_reply
    #[inline]
    pub unsafe fn xcb_xinerama_get_screen_count(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_xinerama_get_screen_count_cookie_t {
        sym!(self, xcb_xinerama_get_screen_count)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_get_screen_count` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_get_screen_count(&self) -> bool {
        has_sym!(self, xcb_xinerama_get_screen_count)
    }

    /// Sends a `Xinerama::GetScreenCount` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xinerama_get_screen_count_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xinerama_get_screen_count_reply`]: Self::xcb_xinerama_get_screen_count_reply
    #[inline]
    pub unsafe fn xcb_xinerama_get_screen_count_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_xinerama_get_screen_count_cookie_t {
        sym!(self, xcb_xinerama_get_screen_count_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_get_screen_count_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_get_screen_count_unchecked(&self) -> bool {
        has_sym!(self, xcb_xinerama_get_screen_count_unchecked)
    }

    /// Waits for the reply to a `Xinerama::GetScreenCount` request.
    #[inline]
    pub unsafe fn xcb_xinerama_get_screen_count_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xinerama_get_screen_count_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xinerama_get_screen_count_reply_t {
        sym!(self, xcb_xinerama_get_screen_count_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_get_screen_count_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_get_screen_count_reply(&self) -> bool {
        has_sym!(self, xcb_xinerama_get_screen_count_reply)
    }

    /// Sends a `Xinerama::GetScreenSize` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xinerama_get_screen_size_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xinerama_get_screen_size_reply`]: Self::xcb_xinerama_get_screen_size_reply
    #[inline]
    pub unsafe fn xcb_xinerama_get_screen_size(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        screen: u32,
    ) -> xcb_xinerama_get_screen_size_cookie_t {
        sym!(self, xcb_xinerama_get_screen_size)(c, window, screen)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_get_screen_size` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_get_screen_size(&self) -> bool {
        has_sym!(self, xcb_xinerama_get_screen_size)
    }

    /// Sends a `Xinerama::GetScreenSize` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xinerama_get_screen_size_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xinerama_get_screen_size_reply`]: Self::xcb_xinerama_get_screen_size_reply
    #[inline]
    pub unsafe fn xcb_xinerama_get_screen_size_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        screen: u32,
    ) -> xcb_xinerama_get_screen_size_cookie_t {
        sym!(self, xcb_xinerama_get_screen_size_unchecked)(c, window, screen)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_get_screen_size_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_get_screen_size_unchecked(&self) -> bool {
        has_sym!(self, xcb_xinerama_get_screen_size_unchecked)
    }

    /// Waits for the reply to a `Xinerama::GetScreenSize` request.
    #[inline]
    pub unsafe fn xcb_xinerama_get_screen_size_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xinerama_get_screen_size_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xinerama_get_screen_size_reply_t {
        sym!(self, xcb_xinerama_get_screen_size_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_get_screen_size_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_get_screen_size_reply(&self) -> bool {
        has_sym!(self, xcb_xinerama_get_screen_size_reply)
    }

    /// Sends a `Xinerama::IsActive` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xinerama_is_active_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xinerama_is_active_reply`]: Self::xcb_xinerama_is_active_reply
    #[inline]
    pub unsafe fn xcb_xinerama_is_active(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xinerama_is_active_cookie_t {
        sym!(self, xcb_xinerama_is_active)(c)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_is_active` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_is_active(&self) -> bool {
        has_sym!(self, xcb_xinerama_is_active)
    }

    /// Sends a `Xinerama::IsActive` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xinerama_is_active_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xinerama_is_active_reply`]: Self::xcb_xinerama_is_active_reply
    #[inline]
    pub unsafe fn xcb_xinerama_is_active_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xinerama_is_active_cookie_t {
        sym!(self, xcb_xinerama_is_active_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_is_active_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_is_active_unchecked(&self) -> bool {
        has_sym!(self, xcb_xinerama_is_active_unchecked)
    }

    /// Waits for the reply to a `Xinerama::IsActive` request.
    #[inline]
    pub unsafe fn xcb_xinerama_is_active_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xinerama_is_active_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xinerama_is_active_reply_t {
        sym!(self, xcb_xinerama_is_active_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_is_active_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_is_active_reply(&self) -> bool {
        has_sym!(self, xcb_xinerama_is_active_reply)
    }

    /// Computes the size of a `xcb_xinerama_query_screens_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xinerama_query_screens_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xinerama_query_screens_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_query_screens_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_query_screens_sizeof(&self) -> bool {
        has_sym!(self, xcb_xinerama_query_screens_sizeof)
    }

    /// Sends a `Xinerama::QueryScreens` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xinerama_query_screens_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xinerama_query_screens_reply`]: Self::xcb_xinerama_query_screens_reply
    #[inline]
    pub unsafe fn xcb_xinerama_query_screens(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xinerama_query_screens_cookie_t {
        sym!(self, xcb_xinerama_query_screens)(c)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_query_screens` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_query_screens(&self) -> bool {
        has_sym!(self, xcb_xinerama_query_screens)
    }

    /// Sends a `Xinerama::QueryScreens` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xinerama_query_screens_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xinerama_query_screens_reply`]: Self::xcb_xinerama_query_screens_reply
    #[inline]
    pub unsafe fn xcb_xinerama_query_screens_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xinerama_query_screens_cookie_t {
        sym!(self, xcb_xinerama_query_screens_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_query_screens_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_query_screens_unchecked(&self) -> bool {
        has_sym!(self, xcb_xinerama_query_screens_unchecked)
    }

    /// Returns a pointer to the `screen_info` field of a `xcb_xinerama_query_screens_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xinerama_query_screens_screen_info(
        &self,
        r: *const xcb_xinerama_query_screens_reply_t,
    ) -> *mut xcb_xinerama_screen_info_t {
        sym!(self, xcb_xinerama_query_screens_screen_info)(r)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_query_screens_screen_info` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_query_screens_screen_info(&self) -> bool {
        has_sym!(self, xcb_xinerama_query_screens_screen_info)
    }

    /// Returns the number of elements of the `screen_info` field of a `xcb_xinerama_query_screens_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xinerama_query_screens_screen_info_length(
        &self,
        r: *const xcb_xinerama_query_screens_reply_t,
    ) -> c_int {
        sym!(self, xcb_xinerama_query_screens_screen_info_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_query_screens_screen_info_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_query_screens_screen_info_length(&self) -> bool {
        has_sym!(self, xcb_xinerama_query_screens_screen_info_length)
    }

    /// Returns an iterator over the elements of the
    /// `screen_info` field of a `xcb_xinerama_query_screens_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xinerama_query_screens_screen_info_iterator(
        &self,
        r: *const xcb_xinerama_query_screens_reply_t,
    ) -> xcb_xinerama_screen_info_iterator_t {
        sym!(self, xcb_xinerama_query_screens_screen_info_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_query_screens_screen_info_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_query_screens_screen_info_iterator(&self) -> bool {
        has_sym!(self, xcb_xinerama_query_screens_screen_info_iterator)
    }

    /// Waits for the reply to a `Xinerama::QueryScreens` request.
    #[inline]
    pub unsafe fn xcb_xinerama_query_screens_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xinerama_query_screens_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xinerama_query_screens_reply_t {
        sym!(self, xcb_xinerama_query_screens_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_query_screens_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_query_screens_reply(&self) -> bool {
        has_sym!(self, xcb_xinerama_query_screens_reply)
    }
}

#[cfg(feature = "xcb_xinerama")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbXinerama::load().unwrap() };
        assert!(lib.has_xcb_xinerama_id());
        assert!(lib.has_xcb_xinerama_screen_info_next());
        assert!(lib.has_xcb_xinerama_screen_info_end());
        assert!(lib.has_xcb_xinerama_query_version());
        assert!(lib.has_xcb_xinerama_query_version_unchecked());
        assert!(lib.has_xcb_xinerama_query_version_reply());
        assert!(lib.has_xcb_xinerama_get_state());
        assert!(lib.has_xcb_xinerama_get_state_unchecked());
        assert!(lib.has_xcb_xinerama_get_state_reply());
        assert!(lib.has_xcb_xinerama_get_screen_count());
        assert!(lib.has_xcb_xinerama_get_screen_count_unchecked());
        assert!(lib.has_xcb_xinerama_get_screen_count_reply());
        assert!(lib.has_xcb_xinerama_get_screen_size());
        assert!(lib.has_xcb_xinerama_get_screen_size_unchecked());
        assert!(lib.has_xcb_xinerama_get_screen_size_reply());
        assert!(lib.has_xcb_xinerama_is_active());
        assert!(lib.has_xcb_xinerama_is_active_unchecked());
        assert!(lib.has_xcb_xinerama_is_active_reply());
        assert!(lib.has_xcb_xinerama_query_screens_sizeof());
        assert!(lib.has_xcb_xinerama_query_screens());
        assert!(lib.has_xcb_xinerama_query_screens_unchecked());
        assert!(lib.has_xcb_xinerama_query_screens_screen_info());
        assert!(lib.has_xcb_xinerama_query_screens_screen_info_length());
        assert!(lib.has_xcb_xinerama_query_screens_screen_info_iterator());
        assert!(lib.has_xcb_xinerama_query_screens_reply());
    }
}
