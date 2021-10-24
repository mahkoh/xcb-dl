use crate::ffi::*;
use crate::lazy::*;
use libloading::{Error, Library};
use std::os::raw::*;

#[rustfmt::skip]
pub struct XcbPresent {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_present_event_end: LazySymbol<unsafe fn(i: *mut xcb_present_event_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_present_event_next: LazySymbol<unsafe fn(i: *mut xcb_present_event_iterator_t)>,
    pub(crate) xcb_present_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_present_notify_end: LazySymbol<unsafe fn(i: *mut xcb_present_notify_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_present_notify_msc: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, serial: u32, target_msc: u64, divisor: u64, remainder: u64) -> xcb_void_cookie_t>,
    pub(crate) xcb_present_notify_msc_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, serial: u32, target_msc: u64, divisor: u64, remainder: u64) -> xcb_void_cookie_t>,
    pub(crate) xcb_present_notify_next: LazySymbol<unsafe fn(i: *mut xcb_present_notify_iterator_t)>,
    pub(crate) xcb_present_pixmap: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, pixmap: xcb_pixmap_t, serial: u32, valid: xcb_xfixes_region_t, update: xcb_xfixes_region_t, x_off: i16, y_off: i16, target_crtc: xcb_randr_crtc_t, wait_fence: xcb_sync_fence_t, idle_fence: xcb_sync_fence_t, options: u32, target_msc: u64, divisor: u64, remainder: u64, notifies_len: u32, notifies: *const xcb_present_notify_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_present_pixmap_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, pixmap: xcb_pixmap_t, serial: u32, valid: xcb_xfixes_region_t, update: xcb_xfixes_region_t, x_off: i16, y_off: i16, target_crtc: xcb_randr_crtc_t, wait_fence: xcb_sync_fence_t, idle_fence: xcb_sync_fence_t, options: u32, target_msc: u64, divisor: u64, remainder: u64, notifies_len: u32, notifies: *const xcb_present_notify_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_present_query_capabilities: LazySymbol<unsafe fn(c: *mut xcb_connection_t, target: u32) -> xcb_present_query_capabilities_cookie_t>,
    pub(crate) xcb_present_query_capabilities_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_present_query_capabilities_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_present_query_capabilities_reply_t>,
    pub(crate) xcb_present_query_capabilities_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, target: u32) -> xcb_present_query_capabilities_cookie_t>,
    pub(crate) xcb_present_query_version: LazySymbol<unsafe fn(c: *mut xcb_connection_t, major_version: u32, minor_version: u32) -> xcb_present_query_version_cookie_t>,
    pub(crate) xcb_present_query_version_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_present_query_version_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_present_query_version_reply_t>,
    pub(crate) xcb_present_query_version_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, major_version: u32, minor_version: u32) -> xcb_present_query_version_cookie_t>,
    pub(crate) xcb_present_redirect_notify_notifies: LazySymbol<unsafe fn(R: *const xcb_present_redirect_notify_event_t) -> *mut xcb_present_notify_t>,
    pub(crate) xcb_present_redirect_notify_notifies_iterator: LazySymbol<unsafe fn(R: *const xcb_present_redirect_notify_event_t) -> xcb_present_notify_iterator_t>,
    pub(crate) xcb_present_redirect_notify_notifies_length: LazySymbol<unsafe fn(R: *const xcb_present_redirect_notify_event_t) -> c_int>,
    pub(crate) xcb_present_select_input: LazySymbol<unsafe fn(c: *mut xcb_connection_t, eid: xcb_present_event_t, window: xcb_window_t, event_mask: u32) -> xcb_void_cookie_t>,
    pub(crate) xcb_present_select_input_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, eid: xcb_present_event_t, window: xcb_window_t, event_mask: u32) -> xcb_void_cookie_t>,
}
