use crate::ffi::*;
use crate::*;
use std::os::raw::*;

pub const XCB_BIG_REQUESTS_MAJOR_VERSION: u32 = 0;
pub const XCB_BIG_REQUESTS_MINOR_VERSION: u32 = 0;

pub const XCB_BIG_REQUESTS_ENABLE: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_big_requests_enable_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_big_requests_enable_cookie_t {
    pub sequence: c_uint,
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

impl Xcb {
    #[inline]
    pub unsafe fn xcb_big_requests_id(&self) -> *mut xcb_extension_t {
        sym!(self, xcb_big_requests_id)
    }

    #[inline]
    pub unsafe fn xcb_big_requests_enable_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_big_requests_enable_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_big_requests_enable_reply_t {
        sym!(self, xcb_big_requests_enable_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_big_requests_enable(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_big_requests_enable_cookie_t {
        sym!(self, xcb_big_requests_enable)(c)
    }

    #[inline]
    pub unsafe fn xcb_big_requests_enable_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_big_requests_enable_cookie_t {
        sym!(self, xcb_big_requests_enable_unchecked)(c)
    }
}
