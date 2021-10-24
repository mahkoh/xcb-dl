use crate::lazy::*;
use crate::*;
use libloading::{Error, Library};
use std::os::raw::*;

#[rustfmt::skip]
pub struct XcbScreensaver {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_screensaver_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_screensaver_query_info: LazySymbol<unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t) -> xcb_screensaver_query_info_cookie_t>,
    pub(crate) xcb_screensaver_query_info_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_screensaver_query_info_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_screensaver_query_info_reply_t>,
    pub(crate) xcb_screensaver_query_info_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t) -> xcb_screensaver_query_info_cookie_t>,
    pub(crate) xcb_screensaver_query_version: LazySymbol<unsafe fn(c: *mut xcb_connection_t, client_major_version: u8, client_minor_version: u8) -> xcb_screensaver_query_version_cookie_t>,
    pub(crate) xcb_screensaver_query_version_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_screensaver_query_version_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_screensaver_query_version_reply_t>,
    pub(crate) xcb_screensaver_query_version_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, client_major_version: u8, client_minor_version: u8) -> xcb_screensaver_query_version_cookie_t>,
    pub(crate) xcb_screensaver_select_input: LazySymbol<unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t, event_mask: u32) -> xcb_void_cookie_t>,
    pub(crate) xcb_screensaver_select_input_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t, event_mask: u32) -> xcb_void_cookie_t>,
    pub(crate) xcb_screensaver_set_attributes: LazySymbol<unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t, x: i16, y: i16, width: u16, height: u16, border_width: u16, class: u8, depth: u8, visual: xcb_visualid_t, value_mask: u32, value_list: *const u32) -> xcb_void_cookie_t>,
    pub(crate) xcb_screensaver_set_attributes_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t, x: i16, y: i16, width: u16, height: u16, border_width: u16, class: u8, depth: u8, visual: xcb_visualid_t, value_mask: u32, value_list: *const u32) -> xcb_void_cookie_t>,
    pub(crate) xcb_screensaver_suspend: LazySymbol<unsafe fn(c: *mut xcb_connection_t, suspend: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_screensaver_suspend_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, suspend: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_screensaver_unset_attributes: LazySymbol<unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_screensaver_unset_attributes_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t) -> xcb_void_cookie_t>,
}
