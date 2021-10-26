#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The libxcb identifier of an extension.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_extension_t {
    pub name: *const c_char,
    pub global_id: c_int,
}

/// A libxcb request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_protocol_request_t {
    pub count: usize,
    pub ext: *mut xcb_extension_t,
    pub opcode: u8,
    pub isvoid: u8,
}

/// The `xcb_send_request_flags_t` enum.
pub type xcb_send_request_flags_t = c_int;
/// The `XCB_REQUEST_CHECKED` variant of `xcb_send_request_flags_t`.
pub const XCB_REQUEST_CHECKED: xcb_send_request_flags_t = 0x01;
/// The `XCB_REQUEST_RAW` variant of `xcb_send_request_flags_t`.
pub const XCB_REQUEST_RAW: xcb_send_request_flags_t = 0x02;
/// The `XCB_REQUEST_DISCARD_REPLY` variant of `xcb_send_request_flags_t`.
pub const XCB_REQUEST_DISCARD_REPLY: xcb_send_request_flags_t = 0x04;
/// The `XCB_REQUEST_REPLY_FDS` variant of `xcb_send_request_flags_t`.
pub const XCB_REQUEST_REPLY_FDS: xcb_send_request_flags_t = 0x08;

pub(crate) struct XcbXcbext {
    xcb_send_request: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            flags: c_int,
            vector: *mut libc::iovec,
            request: *const xcb_protocol_request_t,
        ) -> c_uint,
    >,
    xcb_send_request_with_fds: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            flags: c_int,
            vector: *mut libc::iovec,
            request: *const xcb_protocol_request_t,
            num_fds: c_uint,
            fds: *mut c_int,
        ) -> c_uint,
    >,
    xcb_send_request64: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            flags: c_int,
            vector: *mut libc::iovec,
            request: *const xcb_protocol_request_t,
        ) -> u64,
    >,
    xcb_send_request_with_fds64: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            flags: c_int,
            vector: *mut libc::iovec,
            request: *const xcb_protocol_request_t,
            num_fds: c_uint,
            fds: *mut c_int,
        ) -> u64,
    >,
    xcb_send_fd: LazySymbol<unsafe fn(c: *mut xcb_connection_t, fd: c_int)>,
    xcb_take_socket: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            return_socket: extern "C" fn(closure: *mut c_void),
            closure: *mut c_void,
            flags: c_int,
            sent: *mut u64,
        ) -> c_int,
    >,
    xcb_writev: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            vector: *mut libc::iovec,
            count: c_int,
            requests: u64,
        ) -> c_int,
    >,
    xcb_wait_for_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            request: c_uint,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut c_void,
    >,
    xcb_wait_for_reply64: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            request: u64,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut c_void,
    >,
    xcb_poll_for_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            request: c_uint,
            reply: *mut *mut c_void,
            error: *mut *mut xcb_generic_error_t,
        ) -> c_int,
    >,
    xcb_poll_for_reply64: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            request: u64,
            reply: *mut *mut c_void,
            error: *mut *mut xcb_generic_error_t,
        ) -> c_int,
    >,
    xcb_popcount: LazySymbol<unsafe fn(mask: u32) -> c_int>,
    xcb_sumof: LazySymbol<unsafe fn(list: *mut u8, len: c_int) -> c_int>,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self
            .xcbext
            .$f
            .get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .xcbext
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

impl Xcb {
    /// Sends a request.
    #[inline]
    pub unsafe fn xcb_send_request(
        &self,
        c: *mut xcb_connection_t,
        flags: c_int,
        vector: *mut libc::iovec,
        request: *const xcb_protocol_request_t,
    ) -> c_uint {
        sym!(self, xcb_send_request)(c, flags, vector, request)
    }

    /// Returns `true` iff the symbol `xcb_send_request` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_send_request(&self) -> bool {
        has_sym!(self, xcb_send_request)
    }

    /// Sends a request with file descriptors.
    #[inline]
    pub unsafe fn xcb_send_request_with_fds(
        &self,
        c: *mut xcb_connection_t,
        flags: c_int,
        vector: *mut libc::iovec,
        request: *const xcb_protocol_request_t,
        num_fds: c_uint,
        fds: *mut c_int,
    ) -> c_uint {
        sym!(self, xcb_send_request_with_fds)(c, flags, vector, request, num_fds, fds)
    }

    /// Returns `true` iff the symbol `xcb_send_request_with_fds` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_send_request_with_fds(&self) -> bool {
        has_sym!(self, xcb_send_request_with_fds)
    }

    /// Sends a request with 64-bit sequence numbers.
    #[inline]
    pub unsafe fn xcb_send_request64(
        &self,
        c: *mut xcb_connection_t,
        flags: c_int,
        vector: *mut libc::iovec,
        request: *const xcb_protocol_request_t,
    ) -> u64 {
        sym!(self, xcb_send_request64)(c, flags, vector, request)
    }

    /// Returns `true` iff the symbol `xcb_send_request` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_send_request64(&self) -> bool {
        has_sym!(self, xcb_send_request64)
    }

    /// Sends a request with file descriptors with 64-bit sequence numbers.
    #[inline]
    pub unsafe fn xcb_send_request_with_fds64(
        &self,
        c: *mut xcb_connection_t,
        flags: c_int,
        vector: *mut libc::iovec,
        request: *const xcb_protocol_request_t,
        num_fds: c_uint,
        fds: *mut c_int,
    ) -> u64 {
        sym!(self, xcb_send_request_with_fds64)(c, flags, vector, request, num_fds, fds)
    }

    /// Returns `true` iff the symbol `xcb_send_request_with_fds64` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_send_request_with_fds64(&self) -> bool {
        has_sym!(self, xcb_send_request_with_fds64)
    }

    /// Send a file descriptor to the server in the next call to `xcb_send_request`.
    #[deprecated]
    #[inline]
    pub unsafe fn xcb_send_fd(&self, c: *mut xcb_connection_t, fd: c_int) {
        sym!(self, xcb_send_fd)(c, fd)
    }

    /// Returns `true` iff the symbol `xcb_send_fd` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_send_fd(&self) -> bool {
        has_sym!(self, xcb_send_fd)
    }

    /// Take over the write side of the socket.
    #[inline]
    pub unsafe fn xcb_take_socket(
        &self,
        c: *mut xcb_connection_t,
        return_socket: extern "C" fn(closure: *mut c_void),
        closure: *mut c_void,
        flags: c_int,
        sent: *mut u64,
    ) -> c_int {
        sym!(self, xcb_take_socket)(c, return_socket, closure, flags, sent)
    }

    /// Returns `true` iff the symbol `xcb_take_socket` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_take_socket(&self) -> bool {
        has_sym!(self, xcb_take_socket)
    }

    /// Send raw data to the X server.
    #[inline]
    pub unsafe fn xcb_writev(
        &self,
        c: *mut xcb_connection_t,
        vector: *mut libc::iovec,
        count: c_int,
        requests: u64,
    ) -> c_int {
        sym!(self, xcb_writev)(c, vector, count, requests)
    }

    /// Returns `true` iff the symbol `xcb_writev` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_writev(&self) -> bool {
        has_sym!(self, xcb_writev)
    }

    /// Wait for the reply of a given request.
    #[inline]
    pub unsafe fn xcb_wait_for_reply(
        &self,
        c: *mut xcb_connection_t,
        request: c_uint,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut c_void {
        sym!(self, xcb_wait_for_reply)(c, request, e)
    }

    /// Returns `true` iff the symbol `xcb_wait_for_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_wait_for_reply(&self) -> bool {
        has_sym!(self, xcb_wait_for_reply)
    }

    /// Wait for the reply of a given request, with 64-bit sequence number.
    #[inline]
    pub unsafe fn xcb_wait_for_reply64(
        &self,
        c: *mut xcb_connection_t,
        request: u64,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut c_void {
        sym!(self, xcb_wait_for_reply64)(c, request, e)
    }

    /// Returns `true` iff the symbol `xcb_wait_for_reply64` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_wait_for_reply64(&self) -> bool {
        has_sym!(self, xcb_wait_for_reply64)
    }

    /// Poll for the reply of a given request.
    #[inline]
    pub unsafe fn xcb_poll_for_reply(
        &self,
        c: *mut xcb_connection_t,
        request: c_uint,
        reply: *mut *mut c_void,
        e: *mut *mut xcb_generic_error_t,
    ) -> c_int {
        sym!(self, xcb_poll_for_reply)(c, request, reply, e)
    }

    /// Returns `true` iff the symbol `xcb_poll_for_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poll_for_reply(&self) -> bool {
        has_sym!(self, xcb_poll_for_reply)
    }

    /// Poll for the reply of a given request, with 64-bit sequence number.
    #[inline]
    pub unsafe fn xcb_poll_for_reply64(
        &self,
        c: *mut xcb_connection_t,
        request: u64,
        reply: *mut *mut c_void,
        e: *mut *mut xcb_generic_error_t,
    ) -> c_int {
        sym!(self, xcb_poll_for_reply64)(c, request, reply, e)
    }

    /// Returns `true` iff the symbol `xcb_poll_for_reply64` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poll_for_reply64(&self) -> bool {
        has_sym!(self, xcb_poll_for_reply64)
    }

    #[inline]
    pub unsafe fn xcb_popcount(&self, mask: u32) -> c_int {
        sym!(self, xcb_popcount)(mask)
    }

    /// Returns `true` iff the symbol `xcb_popcount` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_popcount(&self) -> bool {
        has_sym!(self, xcb_popcount)
    }

    #[inline]
    pub unsafe fn xcb_sumof(&self, list: *mut u8, len: c_int) -> c_int {
        sym!(self, xcb_sumof)(list, len)
    }

    /// Returns `true` iff the symbol `xcb_sumof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sumof(&self) -> bool {
        has_sym!(self, xcb_sumof)
    }
}

#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::Xcb::load().unwrap() };
        assert!(lib.has_xcb_send_request());
        assert!(lib.has_xcb_send_request_with_fds());
        assert!(lib.has_xcb_send_request64());
        assert!(lib.has_xcb_send_request_with_fds64());
        assert!(lib.has_xcb_send_fd());
        assert!(lib.has_xcb_take_socket());
        assert!(lib.has_xcb_writev());
        assert!(lib.has_xcb_wait_for_reply());
        assert!(lib.has_xcb_wait_for_reply64());
        assert!(lib.has_xcb_poll_for_reply());
        assert!(lib.has_xcb_poll_for_reply64());
        assert!(lib.has_xcb_popcount());
        assert!(lib.has_xcb_sumof());
    }
}
