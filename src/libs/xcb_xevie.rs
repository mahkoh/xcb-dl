use crate::ffi::*;
use crate::lazy::*;
use libloading::{Error, Library};
use std::os::raw::*;

#[rustfmt::skip]
pub struct XcbXevie {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_xevie_end: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cmap: u32) -> xcb_xevie_end_cookie_t>,
    pub(crate) xcb_xevie_end_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_xevie_end_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_xevie_end_reply_t>,
    pub(crate) xcb_xevie_end_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cmap: u32) -> xcb_xevie_end_cookie_t>,
    pub(crate) xcb_xevie_event_end: LazySymbol<unsafe fn(i: *mut xcb_xevie_event_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_xevie_event_next: LazySymbol<unsafe fn(i: *mut xcb_xevie_event_iterator_t)>,
    pub(crate) xcb_xevie_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_xevie_query_version: LazySymbol<unsafe fn(c: *mut xcb_connection_t, client_major_version: u16, client_minor_version: u16) -> xcb_xevie_query_version_cookie_t>,
    pub(crate) xcb_xevie_query_version_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_xevie_query_version_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_xevie_query_version_reply_t>,
    pub(crate) xcb_xevie_query_version_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, client_major_version: u16, client_minor_version: u16) -> xcb_xevie_query_version_cookie_t>,
    pub(crate) xcb_xevie_select_input: LazySymbol<unsafe fn(c: *mut xcb_connection_t, event_mask: u32) -> xcb_xevie_select_input_cookie_t>,
    pub(crate) xcb_xevie_select_input_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_xevie_select_input_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_xevie_select_input_reply_t>,
    pub(crate) xcb_xevie_select_input_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, event_mask: u32) -> xcb_xevie_select_input_cookie_t>,
    pub(crate) xcb_xevie_send: LazySymbol<unsafe fn(c: *mut xcb_connection_t, event: xcb_xevie_event_t, data_type: u32) -> xcb_xevie_send_cookie_t>,
    pub(crate) xcb_xevie_send_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_xevie_send_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_xevie_send_reply_t>,
    pub(crate) xcb_xevie_send_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, event: xcb_xevie_event_t, data_type: u32) -> xcb_xevie_send_cookie_t>,
    pub(crate) xcb_xevie_start: LazySymbol<unsafe fn(c: *mut xcb_connection_t, screen: u32) -> xcb_xevie_start_cookie_t>,
    pub(crate) xcb_xevie_start_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_xevie_start_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_xevie_start_reply_t>,
    pub(crate) xcb_xevie_start_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, screen: u32) -> xcb_xevie_start_cookie_t>,
}
