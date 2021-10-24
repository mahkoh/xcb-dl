use crate::ffi::*;
use crate::*;
use std::os::raw::*;

pub const XCB_GENERICEVENT_MAJOR_VERSION: u32 = 1;
pub const XCB_GENERICEVENT_MINOR_VERSION: u32 = 0;

pub const XCB_GENERICEVENT_QUERY_VERSION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_genericevent_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client_major_version: u16,
    pub client_minor_version: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_genericevent_query_version_cookie_t {
    pub sequence: c_uint,
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

impl XcbGe {
    #[inline]
    pub unsafe fn xcb_genericevent_id(&self) -> *mut xcb_extension_t {
        sym!(self, xcb_genericevent_id)
    }

    #[inline]
    pub unsafe fn xcb_genericevent_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_genericevent_query_version_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_genericevent_query_version_reply_t {
        sym!(self, xcb_genericevent_query_version_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_genericevent_query_version(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u16,
        client_minor_version: u16,
    ) -> xcb_genericevent_query_version_cookie_t {
        sym!(self, xcb_genericevent_query_version)(c, client_major_version, client_minor_version)
    }

    #[inline]
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
}
