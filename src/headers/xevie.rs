// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The name of the `Xevie` extension.
pub const XCB_XEVIE_NAME: &[u8] = b"XEVIE";

/// The name of the `Xevie` extension.
pub const XCB_XEVIE_NAME_STR: &str = "XEVIE";

/// The cookie for the reply to a `Xevie::QueryVersion` request.
///
/// Pass this cookie to [`xcb_xevie_query_version_reply`] to retrieve the reply.
///
/// [`xcb_xevie_query_version_reply`]: XcbXevie::xcb_xevie_query_version_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_query_version_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xevie_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xevie::QueryVersion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXevie::xcb_xevie_id()`], then the type of the request is
/// [`xcb_xevie_query_version_request_t`].
pub const XCB_XEVIE_QUERY_VERSION: u8 = 0i32 as u8;

/// The `Xevie::QueryVersion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client_major_version: u16,
    pub client_minor_version: u16,
}

impl Default for xcb_xevie_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xevie::QueryVersion` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major_version: u16,
    pub server_minor_version: u16,
    pub pad1: [u8; 20],
}

impl Default for xcb_xevie_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Xevie::Start` request.
///
/// Pass this cookie to [`xcb_xevie_start_reply`] to retrieve the reply.
///
/// [`xcb_xevie_start_reply`]: XcbXevie::xcb_xevie_start_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_start_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xevie_start_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xevie::Start` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXevie::xcb_xevie_id()`], then the type of the request is
/// [`xcb_xevie_start_request_t`].
pub const XCB_XEVIE_START: u8 = 1i32 as u8;

/// The `Xevie::Start` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_start_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
}

impl Default for xcb_xevie_start_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xevie::Start` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_start_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 24],
}

impl Default for xcb_xevie_start_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Xevie::End` request.
///
/// Pass this cookie to [`xcb_xevie_end_reply`] to retrieve the reply.
///
/// [`xcb_xevie_end_reply`]: XcbXevie::xcb_xevie_end_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_end_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xevie_end_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xevie::End` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXevie::xcb_xevie_id()`], then the type of the request is
/// [`xcb_xevie_end_request_t`].
pub const XCB_XEVIE_END: u8 = 2i32 as u8;

/// The `Xevie::End` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_end_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub cmap: u32,
}

impl Default for xcb_xevie_end_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xevie::End` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_end_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 24],
}

impl Default for xcb_xevie_end_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xevie::Datatype` enum.
///
/// This enum has the following variants:
///
/// - [`Xevie::Datatype::Unmodified`](XCB_XEVIE_DATATYPE_UNMODIFIED)
/// - [`Xevie::Datatype::Modified`](XCB_XEVIE_DATATYPE_MODIFIED)
pub type xcb_xevie_datatype_t = u32;
/// The `Xevie::Datatype::Unmodified` enum variant.
///
/// This is a variant of [`xcb_xevie_datatype_t`].
pub const XCB_XEVIE_DATATYPE_UNMODIFIED: xcb_xevie_datatype_t = 0;
/// The `Xevie::Datatype::Modified` enum variant.
///
/// This is a variant of [`xcb_xevie_datatype_t`].
pub const XCB_XEVIE_DATATYPE_MODIFIED: xcb_xevie_datatype_t = 1;

/// The `Xevie::Event` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_event_t {
    pub pad0: [u8; 32],
}

impl Default for xcb_xevie_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Xevie::Event` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_event_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xevie_event_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xevie_event_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Xevie::Send` request.
///
/// Pass this cookie to [`xcb_xevie_send_reply`] to retrieve the reply.
///
/// [`xcb_xevie_send_reply`]: XcbXevie::xcb_xevie_send_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_send_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xevie_send_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xevie::Send` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXevie::xcb_xevie_id()`], then the type of the request is
/// [`xcb_xevie_send_request_t`].
pub const XCB_XEVIE_SEND: u8 = 3i32 as u8;

/// The `Xevie::Send` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_send_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub event: xcb_xevie_event_t,
    pub data_type: u32,
    pub pad0: [u8; 64],
}

impl Default for xcb_xevie_send_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xevie::Send` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_send_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 24],
}

impl Default for xcb_xevie_send_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Xevie::SelectInput` request.
///
/// Pass this cookie to [`xcb_xevie_select_input_reply`] to retrieve the reply.
///
/// [`xcb_xevie_select_input_reply`]: XcbXevie::xcb_xevie_select_input_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_select_input_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xevie_select_input_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xevie::SelectInput` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXevie::xcb_xevie_id()`], then the type of the request is
/// [`xcb_xevie_select_input_request_t`].
pub const XCB_XEVIE_SELECT_INPUT: u8 = 4i32 as u8;

/// The `Xevie::SelectInput` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_select_input_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub event_mask: u32,
}

impl Default for xcb_xevie_select_input_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xevie::SelectInput` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_select_input_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 24],
}

impl Default for xcb_xevie_select_input_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_xevie")]
pub(crate) struct XcbXevieXevie {
    xcb_xevie_id: LazySymbol<*mut xcb_extension_t>,
    xcb_xevie_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u16,
            client_minor_version: u16,
        ) -> xcb_xevie_query_version_cookie_t,
    >,
    xcb_xevie_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u16,
            client_minor_version: u16,
        ) -> xcb_xevie_query_version_cookie_t,
    >,
    xcb_xevie_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xevie_query_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xevie_query_version_reply_t,
    >,
    xcb_xevie_start:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, screen: u32) -> xcb_xevie_start_cookie_t>,
    xcb_xevie_start_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, screen: u32) -> xcb_xevie_start_cookie_t>,
    xcb_xevie_start_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xevie_start_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xevie_start_reply_t,
    >,
    xcb_xevie_end:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cmap: u32) -> xcb_xevie_end_cookie_t>,
    xcb_xevie_end_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cmap: u32) -> xcb_xevie_end_cookie_t>,
    xcb_xevie_end_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xevie_end_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xevie_end_reply_t,
    >,
    xcb_xevie_event_next: LazySymbol<unsafe fn(i: *mut xcb_xevie_event_iterator_t)>,
    xcb_xevie_event_end:
        LazySymbol<unsafe fn(i: xcb_xevie_event_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xevie_send: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            event: xcb_xevie_event_t,
            data_type: u32,
        ) -> xcb_xevie_send_cookie_t,
    >,
    xcb_xevie_send_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            event: xcb_xevie_event_t,
            data_type: u32,
        ) -> xcb_xevie_send_cookie_t,
    >,
    xcb_xevie_send_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xevie_send_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xevie_send_reply_t,
    >,
    xcb_xevie_select_input: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, event_mask: u32) -> xcb_xevie_select_input_cookie_t,
    >,
    xcb_xevie_select_input_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, event_mask: u32) -> xcb_xevie_select_input_cookie_t,
    >,
    xcb_xevie_select_input_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xevie_select_input_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xevie_select_input_reply_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self
            .xevie
            .$f
            .get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .xevie
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_xevie")]
impl XcbXevie {
    /// The libxcb identifier of the `Xevie` extension.
    #[inline]
    pub fn xcb_xevie_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_xevie_id) }
    }

    /// Returns `true` iff the symbol `xcb_xevie_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xevie_id(&self) -> bool {
        has_sym!(self, xcb_xevie_id)
    }

    /// Sends a `Xevie::QueryVersion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xevie_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xevie_query_version_reply`]: Self::xcb_xevie_query_version_reply
    #[inline]
    pub unsafe fn xcb_xevie_query_version(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u16,
        client_minor_version: u16,
    ) -> xcb_xevie_query_version_cookie_t {
        sym!(self, xcb_xevie_query_version)(c, client_major_version, client_minor_version)
    }

    /// Returns `true` iff the symbol `xcb_xevie_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xevie_query_version(&self) -> bool {
        has_sym!(self, xcb_xevie_query_version)
    }

    /// Sends a `Xevie::QueryVersion` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xevie_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xevie_query_version_reply`]: Self::xcb_xevie_query_version_reply
    #[inline]
    pub unsafe fn xcb_xevie_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u16,
        client_minor_version: u16,
    ) -> xcb_xevie_query_version_cookie_t {
        sym!(self, xcb_xevie_query_version_unchecked)(c, client_major_version, client_minor_version)
    }

    /// Returns `true` iff the symbol `xcb_xevie_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xevie_query_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_xevie_query_version_unchecked)
    }

    /// Waits for the reply to a `Xevie::QueryVersion` request.
    #[inline]
    pub unsafe fn xcb_xevie_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xevie_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xevie_query_version_reply_t {
        sym!(self, xcb_xevie_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xevie_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xevie_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_xevie_query_version_reply)
    }

    /// Sends a `Xevie::Start` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xevie_start_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xevie_start_reply`]: Self::xcb_xevie_start_reply
    #[inline]
    pub unsafe fn xcb_xevie_start(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_xevie_start_cookie_t {
        sym!(self, xcb_xevie_start)(c, screen)
    }

    /// Returns `true` iff the symbol `xcb_xevie_start` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xevie_start(&self) -> bool {
        has_sym!(self, xcb_xevie_start)
    }

    /// Sends a `Xevie::Start` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xevie_start_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xevie_start_reply`]: Self::xcb_xevie_start_reply
    #[inline]
    pub unsafe fn xcb_xevie_start_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_xevie_start_cookie_t {
        sym!(self, xcb_xevie_start_unchecked)(c, screen)
    }

    /// Returns `true` iff the symbol `xcb_xevie_start_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xevie_start_unchecked(&self) -> bool {
        has_sym!(self, xcb_xevie_start_unchecked)
    }

    /// Waits for the reply to a `Xevie::Start` request.
    #[inline]
    pub unsafe fn xcb_xevie_start_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xevie_start_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xevie_start_reply_t {
        sym!(self, xcb_xevie_start_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xevie_start_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xevie_start_reply(&self) -> bool {
        has_sym!(self, xcb_xevie_start_reply)
    }

    /// Sends a `Xevie::End` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xevie_end_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xevie_end_reply`]: Self::xcb_xevie_end_reply
    #[inline]
    pub unsafe fn xcb_xevie_end(
        &self,
        c: *mut xcb_connection_t,
        cmap: u32,
    ) -> xcb_xevie_end_cookie_t {
        sym!(self, xcb_xevie_end)(c, cmap)
    }

    /// Returns `true` iff the symbol `xcb_xevie_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xevie_end(&self) -> bool {
        has_sym!(self, xcb_xevie_end)
    }

    /// Sends a `Xevie::End` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xevie_end_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xevie_end_reply`]: Self::xcb_xevie_end_reply
    #[inline]
    pub unsafe fn xcb_xevie_end_unchecked(
        &self,
        c: *mut xcb_connection_t,
        cmap: u32,
    ) -> xcb_xevie_end_cookie_t {
        sym!(self, xcb_xevie_end_unchecked)(c, cmap)
    }

    /// Returns `true` iff the symbol `xcb_xevie_end_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xevie_end_unchecked(&self) -> bool {
        has_sym!(self, xcb_xevie_end_unchecked)
    }

    /// Waits for the reply to a `Xevie::End` request.
    #[inline]
    pub unsafe fn xcb_xevie_end_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xevie_end_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xevie_end_reply_t {
        sym!(self, xcb_xevie_end_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xevie_end_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xevie_end_reply(&self) -> bool {
        has_sym!(self, xcb_xevie_end_reply)
    }

    /// Advances a `xcb_xevie_event_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xevie_event_next(&self, i: *mut xcb_xevie_event_iterator_t) {
        sym!(self, xcb_xevie_event_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xevie_event_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xevie_event_next(&self) -> bool {
        has_sym!(self, xcb_xevie_event_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xevie_event_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xevie_event_end(
        &self,
        i: xcb_xevie_event_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xevie_event_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xevie_event_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xevie_event_end(&self) -> bool {
        has_sym!(self, xcb_xevie_event_end)
    }

    /// Sends a `Xevie::Send` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xevie_send_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xevie_send_reply`]: Self::xcb_xevie_send_reply
    #[inline]
    pub unsafe fn xcb_xevie_send(
        &self,
        c: *mut xcb_connection_t,
        event: xcb_xevie_event_t,
        data_type: u32,
    ) -> xcb_xevie_send_cookie_t {
        sym!(self, xcb_xevie_send)(c, event, data_type)
    }

    /// Returns `true` iff the symbol `xcb_xevie_send` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xevie_send(&self) -> bool {
        has_sym!(self, xcb_xevie_send)
    }

    /// Sends a `Xevie::Send` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xevie_send_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xevie_send_reply`]: Self::xcb_xevie_send_reply
    #[inline]
    pub unsafe fn xcb_xevie_send_unchecked(
        &self,
        c: *mut xcb_connection_t,
        event: xcb_xevie_event_t,
        data_type: u32,
    ) -> xcb_xevie_send_cookie_t {
        sym!(self, xcb_xevie_send_unchecked)(c, event, data_type)
    }

    /// Returns `true` iff the symbol `xcb_xevie_send_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xevie_send_unchecked(&self) -> bool {
        has_sym!(self, xcb_xevie_send_unchecked)
    }

    /// Waits for the reply to a `Xevie::Send` request.
    #[inline]
    pub unsafe fn xcb_xevie_send_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xevie_send_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xevie_send_reply_t {
        sym!(self, xcb_xevie_send_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xevie_send_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xevie_send_reply(&self) -> bool {
        has_sym!(self, xcb_xevie_send_reply)
    }

    /// Sends a `Xevie::SelectInput` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xevie_select_input_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xevie_select_input_reply`]: Self::xcb_xevie_select_input_reply
    #[inline]
    pub unsafe fn xcb_xevie_select_input(
        &self,
        c: *mut xcb_connection_t,
        event_mask: u32,
    ) -> xcb_xevie_select_input_cookie_t {
        sym!(self, xcb_xevie_select_input)(c, event_mask)
    }

    /// Returns `true` iff the symbol `xcb_xevie_select_input` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xevie_select_input(&self) -> bool {
        has_sym!(self, xcb_xevie_select_input)
    }

    /// Sends a `Xevie::SelectInput` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xevie_select_input_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xevie_select_input_reply`]: Self::xcb_xevie_select_input_reply
    #[inline]
    pub unsafe fn xcb_xevie_select_input_unchecked(
        &self,
        c: *mut xcb_connection_t,
        event_mask: u32,
    ) -> xcb_xevie_select_input_cookie_t {
        sym!(self, xcb_xevie_select_input_unchecked)(c, event_mask)
    }

    /// Returns `true` iff the symbol `xcb_xevie_select_input_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xevie_select_input_unchecked(&self) -> bool {
        has_sym!(self, xcb_xevie_select_input_unchecked)
    }

    /// Waits for the reply to a `Xevie::SelectInput` request.
    #[inline]
    pub unsafe fn xcb_xevie_select_input_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xevie_select_input_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xevie_select_input_reply_t {
        sym!(self, xcb_xevie_select_input_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xevie_select_input_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xevie_select_input_reply(&self) -> bool {
        has_sym!(self, xcb_xevie_select_input_reply)
    }
}

#[cfg(feature = "xcb_xevie")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbXevie::load().unwrap() };
        assert!(lib.has_xcb_xevie_id());
        assert!(lib.has_xcb_xevie_query_version());
        assert!(lib.has_xcb_xevie_query_version_unchecked());
        assert!(lib.has_xcb_xevie_query_version_reply());
        assert!(lib.has_xcb_xevie_start());
        assert!(lib.has_xcb_xevie_start_unchecked());
        assert!(lib.has_xcb_xevie_start_reply());
        assert!(lib.has_xcb_xevie_end());
        assert!(lib.has_xcb_xevie_end_unchecked());
        assert!(lib.has_xcb_xevie_end_reply());
        assert!(lib.has_xcb_xevie_event_next());
        assert!(lib.has_xcb_xevie_event_end());
        assert!(lib.has_xcb_xevie_send());
        assert!(lib.has_xcb_xevie_send_unchecked());
        assert!(lib.has_xcb_xevie_send_reply());
        assert!(lib.has_xcb_xevie_select_input());
        assert!(lib.has_xcb_xevie_select_input_unchecked());
        assert!(lib.has_xcb_xevie_select_input_reply());
    }
}
