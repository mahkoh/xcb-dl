use crate::ffi::*;
use crate::*;
use std::os::raw::*;

pub const XCB_XC_MISC_MAJOR_VERSION: u32 = 1;
pub const XCB_XC_MISC_MINOR_VERSION: u32 = 1;

pub const XCB_XC_MISC_GET_VERSION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xc_misc_get_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client_major_version: u16,
    pub client_minor_version: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xc_misc_get_version_cookie_t {
    pub sequence: c_uint,
}

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

pub const XCB_XC_MISC_GET_XID_RANGE: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xc_misc_get_xid_range_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xc_misc_get_xid_range_cookie_t {
    pub sequence: c_uint,
}

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

pub const XCB_XC_MISC_GET_XID_LIST: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xc_misc_get_xid_list_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub count: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xc_misc_get_xid_list_cookie_t {
    pub sequence: c_uint,
}

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

impl Xcb {
    #[inline]
    pub unsafe fn xcb_xc_misc_id(&self) -> *mut xcb_extension_t {
        sym!(self, xcb_xc_misc_id)
    }

    #[inline]
    pub unsafe fn xcb_xc_misc_get_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xc_misc_get_version_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xc_misc_get_version_reply_t {
        sym!(self, xcb_xc_misc_get_version_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xc_misc_get_version(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u16,
        client_minor_version: u16,
    ) -> xcb_xc_misc_get_version_cookie_t {
        sym!(self, xcb_xc_misc_get_version)(c, client_major_version, client_minor_version)
    }

    #[inline]
    pub unsafe fn xcb_xc_misc_get_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u16,
        client_minor_version: u16,
    ) -> xcb_xc_misc_get_version_cookie_t {
        sym!(self, xcb_xc_misc_get_version_unchecked)(c, client_major_version, client_minor_version)
    }

    #[inline]
    pub unsafe fn xcb_xc_misc_get_xid_range_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xc_misc_get_xid_range_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xc_misc_get_xid_range_reply_t {
        sym!(self, xcb_xc_misc_get_xid_range_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xc_misc_get_xid_range(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xc_misc_get_xid_range_cookie_t {
        sym!(self, xcb_xc_misc_get_xid_range)(c)
    }

    #[inline]
    pub unsafe fn xcb_xc_misc_get_xid_range_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xc_misc_get_xid_range_cookie_t {
        sym!(self, xcb_xc_misc_get_xid_range_unchecked)(c)
    }

    #[inline]
    pub unsafe fn xcb_xc_misc_get_xid_list_ids(
        &self,
        R: *const xcb_xc_misc_get_xid_list_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_xc_misc_get_xid_list_ids)(R)
    }

    #[inline]
    pub unsafe fn xcb_xc_misc_get_xid_list_ids_length(
        &self,
        R: *const xcb_xc_misc_get_xid_list_reply_t,
    ) -> c_int {
        sym!(self, xcb_xc_misc_get_xid_list_ids_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_xc_misc_get_xid_list_ids_end(
        &self,
        R: *const xcb_xc_misc_get_xid_list_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xc_misc_get_xid_list_ids_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_xc_misc_get_xid_list_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xc_misc_get_xid_list_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xc_misc_get_xid_list_reply_t {
        sym!(self, xcb_xc_misc_get_xid_list_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xc_misc_get_xid_list(
        &self,
        c: *mut xcb_connection_t,
        count: u32,
    ) -> xcb_xc_misc_get_xid_list_cookie_t {
        sym!(self, xcb_xc_misc_get_xid_list)(c, count)
    }

    #[inline]
    pub unsafe fn xcb_xc_misc_get_xid_list_unchecked(
        &self,
        c: *mut xcb_connection_t,
        count: u32,
    ) -> xcb_xc_misc_get_xid_list_cookie_t {
        sym!(self, xcb_xc_misc_get_xid_list_unchecked)(c, count)
    }
}
