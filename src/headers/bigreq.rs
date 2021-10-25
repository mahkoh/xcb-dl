// This file was generated using generate.py. Do not edit.

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_big_requests_enable_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_big_requests_enable_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_big_requests_enable.
pub const XCB_BIG_REQUESTS_ENABLE: u8 = 0i32 as u8;

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
        ) -> xcb_big_requests_enable_reply_t,
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
    pub fn xcb_big_requests_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_big_requests_id) }
    }

    /// Returns `true` iff the symbol `xcb_big_requests_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_big_requests_id(&self) -> bool {
        has_sym!(self, xcb_big_requests_id)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will cause
     * a reply to be generated. Any returned error will be
     * placed in the event queue.
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_big_requests_enable_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_big_requests_enable_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_big_requests_enable_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_big_requests_enable_reply_t {
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
