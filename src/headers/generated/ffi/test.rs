use crate::ffi::*;
use crate::*;
use std::os::raw::*;

pub const XCB_TEST_MAJOR_VERSION: u32 = 2;
pub const XCB_TEST_MINOR_VERSION: u32 = 2;

pub const XCB_TEST_GET_VERSION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_test_get_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub major_version: u8,
    pub pad0: u8,
    pub minor_version: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_test_get_version_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_test_get_version_reply_t {
    pub response_type: u8,
    pub major_version: u8,
    pub sequence: u16,
    pub length: u32,
    pub minor_version: u16,
}

pub type xcb_test_cursor_t = u32;
pub const XCB_TEST_CURSOR_NONE: xcb_test_cursor_t = 0x00;
pub const XCB_TEST_CURSOR_CURRENT: xcb_test_cursor_t = 0x01;

pub const XCB_TEST_COMPARE_CURSOR: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_test_compare_cursor_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub cursor: xcb_cursor_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_test_compare_cursor_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_test_compare_cursor_reply_t {
    pub response_type: u8,
    pub same: u8,
    pub sequence: u16,
    pub length: u32,
}

pub const XCB_TEST_FAKE_INPUT: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_test_fake_input_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub type_: u8,
    pub detail: u8,
    pub pad0: [u8; 2],
    pub time: u32,
    pub root: xcb_window_t,
    pub pad1: [u8; 8],
    pub root_x: i16,
    pub root_y: i16,
    pub pad2: [u8; 7],
    pub deviceid: u8,
}

pub const XCB_TEST_GRAB_CONTROL: u8 = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_test_grab_control_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub impervious: u8,
    pub pad0: [u8; 3],
}

impl XcbXtest {
    #[inline]
    pub unsafe fn xcb_test_id(&self) -> *mut xcb_extension_t {
        sym!(self, xcb_test_id)
    }

    #[inline]
    pub unsafe fn xcb_test_get_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_test_get_version_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_test_get_version_reply_t {
        sym!(self, xcb_test_get_version_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_test_get_version(
        &self,
        c: *mut xcb_connection_t,
        major_version: u8,
        minor_version: u16,
    ) -> xcb_test_get_version_cookie_t {
        sym!(self, xcb_test_get_version)(c, major_version, minor_version)
    }

    #[inline]
    pub unsafe fn xcb_test_get_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        major_version: u8,
        minor_version: u16,
    ) -> xcb_test_get_version_cookie_t {
        sym!(self, xcb_test_get_version_unchecked)(c, major_version, minor_version)
    }

    #[inline]
    pub unsafe fn xcb_test_compare_cursor_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_test_compare_cursor_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_test_compare_cursor_reply_t {
        sym!(self, xcb_test_compare_cursor_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_test_compare_cursor(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        cursor: xcb_cursor_t,
    ) -> xcb_test_compare_cursor_cookie_t {
        sym!(self, xcb_test_compare_cursor)(c, window, cursor)
    }

    #[inline]
    pub unsafe fn xcb_test_compare_cursor_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        cursor: xcb_cursor_t,
    ) -> xcb_test_compare_cursor_cookie_t {
        sym!(self, xcb_test_compare_cursor_unchecked)(c, window, cursor)
    }

    #[inline]
    pub unsafe fn xcb_test_fake_input(
        &self,
        c: *mut xcb_connection_t,
        type_: u8,
        detail: u8,
        time: u32,
        root: xcb_window_t,
        root_x: i16,
        root_y: i16,
        deviceid: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_test_fake_input)(c, type_, detail, time, root, root_x, root_y, deviceid)
    }

    #[inline]
    pub unsafe fn xcb_test_fake_input_checked(
        &self,
        c: *mut xcb_connection_t,
        type_: u8,
        detail: u8,
        time: u32,
        root: xcb_window_t,
        root_x: i16,
        root_y: i16,
        deviceid: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_test_fake_input_checked)(
            c, type_, detail, time, root, root_x, root_y, deviceid,
        )
    }

    #[inline]
    pub unsafe fn xcb_test_grab_control(
        &self,
        c: *mut xcb_connection_t,
        impervious: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_test_grab_control)(c, impervious)
    }

    #[inline]
    pub unsafe fn xcb_test_grab_control_checked(
        &self,
        c: *mut xcb_connection_t,
        impervious: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_test_grab_control_checked)(c, impervious)
    }
}
