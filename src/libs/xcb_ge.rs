use crate::lazy::*;
use crate::*;
use libloading::{Error, Library};
use std::os::raw::*;

#[rustfmt::skip]
pub struct XcbGe {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_genericevent_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_genericevent_query_version: LazySymbol<unsafe fn(c: *mut xcb_connection_t, client_major_version: u16, client_minor_version: u16) -> xcb_genericevent_query_version_cookie_t>,
    pub(crate) xcb_genericevent_query_version_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_genericevent_query_version_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_genericevent_query_version_reply_t>,
    pub(crate) xcb_genericevent_query_version_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, client_major_version: u16, client_minor_version: u16) -> xcb_genericevent_query_version_cookie_t>,
}
