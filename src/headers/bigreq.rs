// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The name of the `BigRequests` extension.
pub const XCB_BIG_REQUESTS_NAME: &[u8] = b"BIG-REQUESTS";

/// The name of the `BigRequests` extension.
pub const XCB_BIG_REQUESTS_NAME_STR: &str = "BIG-REQUESTS";

/// The cookie for the reply to a `BigRequests::Enable` request.
///
/// Pass this cookie to [`xcb_big_requests_enable_reply`] to retrieve the reply.
///
/// [`xcb_big_requests_enable_reply`]: Xcb::xcb_big_requests_enable_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_big_requests_enable_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_big_requests_enable_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `BigRequests::Enable` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`Xcb::xcb_big_requests_id()`], then the type of the request is
/// [`xcb_big_requests_enable_request_t`].
pub const XCB_BIG_REQUESTS_ENABLE: u8 = 0i32 as u8;

/// The `BigRequests::Enable` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_big_requests_enable_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_big_requests_enable_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `BigRequests::Enable` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_big_requests_enable_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub maximum_request_length: u32,
}

impl Default for xcb_big_requests_enable_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub(crate) struct XcbBigreq {
    xcb_big_requests_id: LazySymbol<*mut xcb_extension_t>,
    xcb_big_requests_enable:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_big_requests_enable_cookie_t>,
    xcb_big_requests_enable_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_big_requests_enable_cookie_t>,
    xcb_big_requests_enable_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_big_requests_enable_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_big_requests_enable_reply_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self
            .bigreq
            .$f
            .get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .bigreq
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

impl Xcb {
    /// The libxcb identifier of the `BigRequests` extension.
    #[inline]
    pub fn xcb_big_requests_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_big_requests_id) }
    }

    /// Returns `true` iff the symbol `xcb_big_requests_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_big_requests_id(&self) -> bool {
        has_sym!(self, xcb_big_requests_id)
    }

    /// Sends a `BigRequests::Enable` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_big_requests_enable_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_big_requests_enable_reply`]: Self::xcb_big_requests_enable_reply
    #[inline]
    pub unsafe fn xcb_big_requests_enable(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_big_requests_enable_cookie_t {
        sym!(self, xcb_big_requests_enable)(c)
    }

    /// Returns `true` iff the symbol `xcb_big_requests_enable` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_big_requests_enable(&self) -> bool {
        has_sym!(self, xcb_big_requests_enable)
    }

    /// Sends a `BigRequests::Enable` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_big_requests_enable_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_big_requests_enable_reply`]: Self::xcb_big_requests_enable_reply
    #[inline]
    pub unsafe fn xcb_big_requests_enable_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_big_requests_enable_cookie_t {
        sym!(self, xcb_big_requests_enable_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_big_requests_enable_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_big_requests_enable_unchecked(&self) -> bool {
        has_sym!(self, xcb_big_requests_enable_unchecked)
    }

    /// Waits for the reply to a `BigRequests::Enable` request.
    #[inline]
    pub unsafe fn xcb_big_requests_enable_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_big_requests_enable_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_big_requests_enable_reply_t {
        sym!(self, xcb_big_requests_enable_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_big_requests_enable_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_big_requests_enable_reply(&self) -> bool {
        has_sym!(self, xcb_big_requests_enable_reply)
    }
}

#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::Xcb::load().unwrap() };
        assert!(lib.has_xcb_big_requests_id());
        assert!(lib.has_xcb_big_requests_enable());
        assert!(lib.has_xcb_big_requests_enable_unchecked());
        assert!(lib.has_xcb_big_requests_enable_reply());
    }
}
