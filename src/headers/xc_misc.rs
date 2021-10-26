// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The cookie for the reply to a `XCMisc::GetVersion` request.
///
/// Pass this cookie to [`xcb_xc_misc_get_version_reply`] to retrieve the reply.
///
/// [`xcb_xc_misc_get_version_reply`]: Xcb::xcb_xc_misc_get_version_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xc_misc_get_version_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xc_misc_get_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XCMisc::GetVersion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`Xcb::xcb_xc_misc_id()`], then the type of the request is
/// [`xcb_xc_misc_get_version_request_t`].
pub const XCB_XC_MISC_GET_VERSION: u8 = 0i32 as u8;

/// The `XCMisc::GetVersion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xc_misc_get_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client_major_version: u16,
    pub client_minor_version: u16,
}

impl Default for xcb_xc_misc_get_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XCMisc::GetVersion` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xc_misc_get_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major_version: u16,
    pub server_minor_version: u16,
}

impl Default for xcb_xc_misc_get_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `XCMisc::GetXIDRange` request.
///
/// Pass this cookie to [`xcb_xc_misc_get_xid_range_reply`] to retrieve the reply.
///
/// [`xcb_xc_misc_get_xid_range_reply`]: Xcb::xcb_xc_misc_get_xid_range_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xc_misc_get_xid_range_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xc_misc_get_xid_range_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XCMisc::GetXIDRange` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`Xcb::xcb_xc_misc_id()`], then the type of the request is
/// [`xcb_xc_misc_get_xid_range_request_t`].
pub const XCB_XC_MISC_GET_XID_RANGE: u8 = 1i32 as u8;

/// The `XCMisc::GetXIDRange` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xc_misc_get_xid_range_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_xc_misc_get_xid_range_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XCMisc::GetXIDRange` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xc_misc_get_xid_range_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub start_id: u32,
    pub count: u32,
}

impl Default for xcb_xc_misc_get_xid_range_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `XCMisc::GetXIDList` request.
///
/// Pass this cookie to [`xcb_xc_misc_get_xid_list_reply`] to retrieve the reply.
///
/// [`xcb_xc_misc_get_xid_list_reply`]: Xcb::xcb_xc_misc_get_xid_list_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xc_misc_get_xid_list_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xc_misc_get_xid_list_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XCMisc::GetXIDList` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`Xcb::xcb_xc_misc_id()`], then the type of the request is
/// [`xcb_xc_misc_get_xid_list_request_t`].
pub const XCB_XC_MISC_GET_XID_LIST: u8 = 2i32 as u8;

/// The `XCMisc::GetXIDList` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xc_misc_get_xid_list_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub count: u32,
}

impl Default for xcb_xc_misc_get_xid_list_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XCMisc::GetXIDList` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `ids`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xc_misc_get_xid_list_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub ids_len: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_xc_misc_get_xid_list_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub(crate) struct XcbXc_Misc {
    xcb_xc_misc_id: LazySymbol<*mut xcb_extension_t>,
    xcb_xc_misc_get_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u16,
            client_minor_version: u16,
        ) -> xcb_xc_misc_get_version_cookie_t,
    >,
    xcb_xc_misc_get_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u16,
            client_minor_version: u16,
        ) -> xcb_xc_misc_get_version_cookie_t,
    >,
    xcb_xc_misc_get_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xc_misc_get_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xc_misc_get_version_reply_t,
    >,
    xcb_xc_misc_get_xid_range:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xc_misc_get_xid_range_cookie_t>,
    xcb_xc_misc_get_xid_range_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xc_misc_get_xid_range_cookie_t>,
    xcb_xc_misc_get_xid_range_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xc_misc_get_xid_range_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xc_misc_get_xid_range_reply_t,
    >,
    xcb_xc_misc_get_xid_list_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xc_misc_get_xid_list: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, count: u32) -> xcb_xc_misc_get_xid_list_cookie_t,
    >,
    xcb_xc_misc_get_xid_list_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, count: u32) -> xcb_xc_misc_get_xid_list_cookie_t,
    >,
    xcb_xc_misc_get_xid_list_ids:
        LazySymbol<unsafe fn(r: *const xcb_xc_misc_get_xid_list_reply_t) -> *mut u32>,
    xcb_xc_misc_get_xid_list_ids_length:
        LazySymbol<unsafe fn(r: *const xcb_xc_misc_get_xid_list_reply_t) -> c_int>,
    xcb_xc_misc_get_xid_list_ids_end:
        LazySymbol<unsafe fn(r: *const xcb_xc_misc_get_xid_list_reply_t) -> xcb_generic_iterator_t>,
    xcb_xc_misc_get_xid_list_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xc_misc_get_xid_list_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xc_misc_get_xid_list_reply_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self
            .xc_misc
            .$f
            .get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .xc_misc
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

impl Xcb {
    /// The libxcb identifier of the `XCMisc` extension.
    #[inline]
    pub fn xcb_xc_misc_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_xc_misc_id) }
    }

    /// Returns `true` iff the symbol `xcb_xc_misc_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xc_misc_id(&self) -> bool {
        has_sym!(self, xcb_xc_misc_id)
    }

    /// Sends a `XCMisc::GetVersion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xc_misc_get_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xc_misc_get_version_reply`]: Self::xcb_xc_misc_get_version_reply
    #[inline]
    pub unsafe fn xcb_xc_misc_get_version(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u16,
        client_minor_version: u16,
    ) -> xcb_xc_misc_get_version_cookie_t {
        sym!(self, xcb_xc_misc_get_version)(c, client_major_version, client_minor_version)
    }

    /// Returns `true` iff the symbol `xcb_xc_misc_get_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xc_misc_get_version(&self) -> bool {
        has_sym!(self, xcb_xc_misc_get_version)
    }

    /// Sends a `XCMisc::GetVersion` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xc_misc_get_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xc_misc_get_version_reply`]: Self::xcb_xc_misc_get_version_reply
    #[inline]
    pub unsafe fn xcb_xc_misc_get_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u16,
        client_minor_version: u16,
    ) -> xcb_xc_misc_get_version_cookie_t {
        sym!(self, xcb_xc_misc_get_version_unchecked)(c, client_major_version, client_minor_version)
    }

    /// Returns `true` iff the symbol `xcb_xc_misc_get_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xc_misc_get_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_xc_misc_get_version_unchecked)
    }

    /// Waits for the reply to a `XCMisc::GetVersion` request.
    #[inline]
    pub unsafe fn xcb_xc_misc_get_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xc_misc_get_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xc_misc_get_version_reply_t {
        sym!(self, xcb_xc_misc_get_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xc_misc_get_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xc_misc_get_version_reply(&self) -> bool {
        has_sym!(self, xcb_xc_misc_get_version_reply)
    }

    /// Sends a `XCMisc::GetXIDRange` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xc_misc_get_xid_range_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xc_misc_get_xid_range_reply`]: Self::xcb_xc_misc_get_xid_range_reply
    #[inline]
    pub unsafe fn xcb_xc_misc_get_xid_range(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xc_misc_get_xid_range_cookie_t {
        sym!(self, xcb_xc_misc_get_xid_range)(c)
    }

    /// Returns `true` iff the symbol `xcb_xc_misc_get_xid_range` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xc_misc_get_xid_range(&self) -> bool {
        has_sym!(self, xcb_xc_misc_get_xid_range)
    }

    /// Sends a `XCMisc::GetXIDRange` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xc_misc_get_xid_range_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xc_misc_get_xid_range_reply`]: Self::xcb_xc_misc_get_xid_range_reply
    #[inline]
    pub unsafe fn xcb_xc_misc_get_xid_range_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xc_misc_get_xid_range_cookie_t {
        sym!(self, xcb_xc_misc_get_xid_range_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_xc_misc_get_xid_range_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xc_misc_get_xid_range_unchecked(&self) -> bool {
        has_sym!(self, xcb_xc_misc_get_xid_range_unchecked)
    }

    /// Waits for the reply to a `XCMisc::GetXIDRange` request.
    #[inline]
    pub unsafe fn xcb_xc_misc_get_xid_range_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xc_misc_get_xid_range_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xc_misc_get_xid_range_reply_t {
        sym!(self, xcb_xc_misc_get_xid_range_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xc_misc_get_xid_range_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xc_misc_get_xid_range_reply(&self) -> bool {
        has_sym!(self, xcb_xc_misc_get_xid_range_reply)
    }

    /// Computes the size of a `xcb_xc_misc_get_xid_list_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xc_misc_get_xid_list_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xc_misc_get_xid_list_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xc_misc_get_xid_list_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xc_misc_get_xid_list_sizeof(&self) -> bool {
        has_sym!(self, xcb_xc_misc_get_xid_list_sizeof)
    }

    /// Sends a `XCMisc::GetXIDList` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xc_misc_get_xid_list_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xc_misc_get_xid_list_reply`]: Self::xcb_xc_misc_get_xid_list_reply
    #[inline]
    pub unsafe fn xcb_xc_misc_get_xid_list(
        &self,
        c: *mut xcb_connection_t,
        count: u32,
    ) -> xcb_xc_misc_get_xid_list_cookie_t {
        sym!(self, xcb_xc_misc_get_xid_list)(c, count)
    }

    /// Returns `true` iff the symbol `xcb_xc_misc_get_xid_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xc_misc_get_xid_list(&self) -> bool {
        has_sym!(self, xcb_xc_misc_get_xid_list)
    }

    /// Sends a `XCMisc::GetXIDList` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xc_misc_get_xid_list_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xc_misc_get_xid_list_reply`]: Self::xcb_xc_misc_get_xid_list_reply
    #[inline]
    pub unsafe fn xcb_xc_misc_get_xid_list_unchecked(
        &self,
        c: *mut xcb_connection_t,
        count: u32,
    ) -> xcb_xc_misc_get_xid_list_cookie_t {
        sym!(self, xcb_xc_misc_get_xid_list_unchecked)(c, count)
    }

    /// Returns `true` iff the symbol `xcb_xc_misc_get_xid_list_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xc_misc_get_xid_list_unchecked(&self) -> bool {
        has_sym!(self, xcb_xc_misc_get_xid_list_unchecked)
    }

    /// Returns a pointer to the `ids` field of a `xcb_xc_misc_get_xid_list_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xc_misc_get_xid_list_ids(
        &self,
        r: *const xcb_xc_misc_get_xid_list_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_xc_misc_get_xid_list_ids)(r)
    }

    /// Returns `true` iff the symbol `xcb_xc_misc_get_xid_list_ids` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xc_misc_get_xid_list_ids(&self) -> bool {
        has_sym!(self, xcb_xc_misc_get_xid_list_ids)
    }

    /// Returns the number of elements of the `ids` field of a `xcb_xc_misc_get_xid_list_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xc_misc_get_xid_list_ids_length(
        &self,
        r: *const xcb_xc_misc_get_xid_list_reply_t,
    ) -> c_int {
        sym!(self, xcb_xc_misc_get_xid_list_ids_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xc_misc_get_xid_list_ids_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xc_misc_get_xid_list_ids_length(&self) -> bool {
        has_sym!(self, xcb_xc_misc_get_xid_list_ids_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `ids` field of a `xcb_xc_misc_get_xid_list_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xc_misc_get_xid_list_ids_end(
        &self,
        r: *const xcb_xc_misc_get_xid_list_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xc_misc_get_xid_list_ids_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xc_misc_get_xid_list_ids_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xc_misc_get_xid_list_ids_end(&self) -> bool {
        has_sym!(self, xcb_xc_misc_get_xid_list_ids_end)
    }

    /// Waits for the reply to a `XCMisc::GetXIDList` request.
    #[inline]
    pub unsafe fn xcb_xc_misc_get_xid_list_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xc_misc_get_xid_list_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xc_misc_get_xid_list_reply_t {
        sym!(self, xcb_xc_misc_get_xid_list_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xc_misc_get_xid_list_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xc_misc_get_xid_list_reply(&self) -> bool {
        has_sym!(self, xcb_xc_misc_get_xid_list_reply)
    }
}

#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::Xcb::load().unwrap() };
        assert!(lib.has_xcb_xc_misc_id());
        assert!(lib.has_xcb_xc_misc_get_version());
        assert!(lib.has_xcb_xc_misc_get_version_unchecked());
        assert!(lib.has_xcb_xc_misc_get_version_reply());
        assert!(lib.has_xcb_xc_misc_get_xid_range());
        assert!(lib.has_xcb_xc_misc_get_xid_range_unchecked());
        assert!(lib.has_xcb_xc_misc_get_xid_range_reply());
        assert!(lib.has_xcb_xc_misc_get_xid_list_sizeof());
        assert!(lib.has_xcb_xc_misc_get_xid_list());
        assert!(lib.has_xcb_xc_misc_get_xid_list_unchecked());
        assert!(lib.has_xcb_xc_misc_get_xid_list_ids());
        assert!(lib.has_xcb_xc_misc_get_xid_list_ids_length());
        assert!(lib.has_xcb_xc_misc_get_xid_list_ids_end());
        assert!(lib.has_xcb_xc_misc_get_xid_list_reply());
    }
}
