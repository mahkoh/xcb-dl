// This file was generated using generate.py. Do not edit.

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_genericevent_query_version_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_genericevent_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_genericevent_query_version.
pub const XCB_GENERICEVENT_QUERY_VERSION: u8 = 0i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_genericevent_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client_major_version: u16,
    pub client_minor_version: u16,
}

impl Default for xcb_genericevent_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_genericevent_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub pad1: [u8; 20],
}

impl Default for xcb_genericevent_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_ge")]
pub(crate) struct XcbGeGe {
    xcb_genericevent_id: LazySymbol<*mut xcb_extension_t>,
    xcb_genericevent_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u16,
            client_minor_version: u16,
        ) -> xcb_genericevent_query_version_cookie_t,
    >,
    xcb_genericevent_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u16,
            client_minor_version: u16,
        ) -> xcb_genericevent_query_version_cookie_t,
    >,
    xcb_genericevent_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_genericevent_query_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_genericevent_query_version_reply_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self.ge.$f.get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .ge
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_ge")]
impl XcbGe {
    pub fn xcb_genericevent_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_genericevent_id) }
    }

    /// Returns `true` iff the symbol `xcb_genericevent_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_genericevent_id(&self) -> bool {
        has_sym!(self, xcb_genericevent_id)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_genericevent_query_version(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u16,
        client_minor_version: u16,
    ) -> xcb_genericevent_query_version_cookie_t {
        sym!(self, xcb_genericevent_query_version)(c, client_major_version, client_minor_version)
    }

    /// Returns `true` iff the symbol `xcb_genericevent_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_genericevent_query_version(&self) -> bool {
        has_sym!(self, xcb_genericevent_query_version)
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
    pub unsafe fn xcb_genericevent_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u16,
        client_minor_version: u16,
    ) -> xcb_genericevent_query_version_cookie_t {
        sym!(self, xcb_genericevent_query_version_unchecked)(
            c,
            client_major_version,
            client_minor_version,
        )
    }

    /// Returns `true` iff the symbol `xcb_genericevent_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_genericevent_query_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_genericevent_query_version_unchecked)
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
     * xcb_genericevent_query_version_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_genericevent_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_genericevent_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_genericevent_query_version_reply_t {
        sym!(self, xcb_genericevent_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_genericevent_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_genericevent_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_genericevent_query_version_reply)
    }
}

#[cfg(feature = "xcb_ge")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbGe::load().unwrap() };
        assert!(lib.has_xcb_genericevent_id());
        assert!(lib.has_xcb_genericevent_query_version());
        assert!(lib.has_xcb_genericevent_query_version_unchecked());
        assert!(lib.has_xcb_genericevent_query_version_reply());
    }
}
