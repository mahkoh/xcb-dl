use crate::ffi::*;
use crate::lazy::*;
use libloading::{Error, Library};
use std::os::raw::*;

#[rustfmt::skip]
pub struct XcbXtest {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_test_compare_cursor: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, cursor: xcb_cursor_t) -> xcb_test_compare_cursor_cookie_t>,
    pub(crate) xcb_test_compare_cursor_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_test_compare_cursor_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_test_compare_cursor_reply_t>,
    pub(crate) xcb_test_compare_cursor_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, cursor: xcb_cursor_t) -> xcb_test_compare_cursor_cookie_t>,
    pub(crate) xcb_test_fake_input: LazySymbol<unsafe fn(c: *mut xcb_connection_t, type_: u8, detail: u8, time: u32, root: xcb_window_t, root_x: i16, root_y: i16, deviceid: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_test_fake_input_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, type_: u8, detail: u8, time: u32, root: xcb_window_t, root_x: i16, root_y: i16, deviceid: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_test_get_version: LazySymbol<unsafe fn(c: *mut xcb_connection_t, major_version: u8, minor_version: u16) -> xcb_test_get_version_cookie_t>,
    pub(crate) xcb_test_get_version_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_test_get_version_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_test_get_version_reply_t>,
    pub(crate) xcb_test_get_version_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, major_version: u8, minor_version: u16) -> xcb_test_get_version_cookie_t>,
    pub(crate) xcb_test_grab_control: LazySymbol<unsafe fn(c: *mut xcb_connection_t, impervious: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_test_grab_control_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, impervious: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_test_id: LazySymbol<*mut xcb_extension_t>,
}
