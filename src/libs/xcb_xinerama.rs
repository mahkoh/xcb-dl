use crate::ffi::*;
use crate::lazy::*;
use libloading::{Error, Library};
use std::os::raw::*;

#[rustfmt::skip]
pub struct XcbXinerama {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_xinerama_get_screen_count: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_xinerama_get_screen_count_cookie_t>,
    pub(crate) xcb_xinerama_get_screen_count_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_xinerama_get_screen_count_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_xinerama_get_screen_count_reply_t>,
    pub(crate) xcb_xinerama_get_screen_count_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_xinerama_get_screen_count_cookie_t>,
    pub(crate) xcb_xinerama_get_screen_size: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, screen: u32) -> xcb_xinerama_get_screen_size_cookie_t>,
    pub(crate) xcb_xinerama_get_screen_size_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_xinerama_get_screen_size_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_xinerama_get_screen_size_reply_t>,
    pub(crate) xcb_xinerama_get_screen_size_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, screen: u32) -> xcb_xinerama_get_screen_size_cookie_t>,
    pub(crate) xcb_xinerama_get_state: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_xinerama_get_state_cookie_t>,
    pub(crate) xcb_xinerama_get_state_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_xinerama_get_state_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_xinerama_get_state_reply_t>,
    pub(crate) xcb_xinerama_get_state_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_xinerama_get_state_cookie_t>,
    pub(crate) xcb_xinerama_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_xinerama_is_active: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xinerama_is_active_cookie_t>,
    pub(crate) xcb_xinerama_is_active_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_xinerama_is_active_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_xinerama_is_active_reply_t>,
    pub(crate) xcb_xinerama_is_active_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xinerama_is_active_cookie_t>,
    pub(crate) xcb_xinerama_query_screens: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xinerama_query_screens_cookie_t>,
    pub(crate) xcb_xinerama_query_screens_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_xinerama_query_screens_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_xinerama_query_screens_reply_t>,
    pub(crate) xcb_xinerama_query_screens_screen_info: LazySymbol<unsafe fn(R: *const xcb_xinerama_query_screens_reply_t) -> *mut xcb_xinerama_screen_info_t>,
    pub(crate) xcb_xinerama_query_screens_screen_info_iterator: LazySymbol<unsafe fn(R: *const xcb_xinerama_query_screens_reply_t) -> xcb_xinerama_screen_info_iterator_t>,
    pub(crate) xcb_xinerama_query_screens_screen_info_length: LazySymbol<unsafe fn(R: *const xcb_xinerama_query_screens_reply_t) -> c_int>,
    pub(crate) xcb_xinerama_query_screens_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xinerama_query_screens_cookie_t>,
    pub(crate) xcb_xinerama_query_version: LazySymbol<unsafe fn(c: *mut xcb_connection_t, major: u8, minor: u8) -> xcb_xinerama_query_version_cookie_t>,
    pub(crate) xcb_xinerama_query_version_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_xinerama_query_version_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_xinerama_query_version_reply_t>,
    pub(crate) xcb_xinerama_query_version_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, major: u8, minor: u8) -> xcb_xinerama_query_version_cookie_t>,
    pub(crate) xcb_xinerama_screen_info_end: LazySymbol<unsafe fn(i: *mut xcb_xinerama_screen_info_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_xinerama_screen_info_next: LazySymbol<unsafe fn(i: *mut xcb_xinerama_screen_info_iterator_t)>,
}
