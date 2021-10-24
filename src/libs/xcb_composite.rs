use crate::lazy::*;
use crate::*;
use libloading::{Error, Library};
use std::os::raw::*;

#[rustfmt::skip]
pub struct XcbComposite {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_composite_create_region_from_border_clip: LazySymbol<unsafe fn(c: *mut xcb_connection_t, region: xcb_xfixes_region_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_composite_create_region_from_border_clip_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, region: xcb_xfixes_region_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_composite_get_overlay_window: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_composite_get_overlay_window_cookie_t>,
    pub(crate) xcb_composite_get_overlay_window_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_composite_get_overlay_window_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_composite_get_overlay_window_reply_t>,
    pub(crate) xcb_composite_get_overlay_window_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_composite_get_overlay_window_cookie_t>,
    pub(crate) xcb_composite_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_composite_name_window_pixmap: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, pixmap: xcb_pixmap_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_composite_name_window_pixmap_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, pixmap: xcb_pixmap_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_composite_query_version: LazySymbol<unsafe fn(c: *mut xcb_connection_t, client_major_version: u32, client_minor_version: u32) -> xcb_composite_query_version_cookie_t>,
    pub(crate) xcb_composite_query_version_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_composite_query_version_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_composite_query_version_reply_t>,
    pub(crate) xcb_composite_query_version_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, client_major_version: u32, client_minor_version: u32) -> xcb_composite_query_version_cookie_t>,
    pub(crate) xcb_composite_redirect_subwindows: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, update: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_composite_redirect_subwindows_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, update: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_composite_redirect_window: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, update: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_composite_redirect_window_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, update: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_composite_release_overlay_window: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_composite_release_overlay_window_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_composite_unredirect_subwindows: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, update: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_composite_unredirect_subwindows_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, update: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_composite_unredirect_window: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, update: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_composite_unredirect_window_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, update: u8) -> xcb_void_cookie_t>,
}
