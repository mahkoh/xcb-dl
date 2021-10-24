use crate::lazy::*;
use crate::*;
use libloading::{Error, Library};
use std::os::raw::*;

#[rustfmt::skip]
pub struct XcbDpms {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_dpms_capable: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_dpms_capable_cookie_t>,
    pub(crate) xcb_dpms_capable_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_dpms_capable_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_dpms_capable_reply_t>,
    pub(crate) xcb_dpms_capable_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_dpms_capable_cookie_t>,
    pub(crate) xcb_dpms_disable: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_dpms_disable_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_dpms_enable: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_dpms_enable_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_dpms_force_level: LazySymbol<unsafe fn(c: *mut xcb_connection_t, power_level: u16) -> xcb_void_cookie_t>,
    pub(crate) xcb_dpms_force_level_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, power_level: u16) -> xcb_void_cookie_t>,
    pub(crate) xcb_dpms_get_timeouts: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_dpms_get_timeouts_cookie_t>,
    pub(crate) xcb_dpms_get_timeouts_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_dpms_get_timeouts_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_dpms_get_timeouts_reply_t>,
    pub(crate) xcb_dpms_get_timeouts_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_dpms_get_timeouts_cookie_t>,
    pub(crate) xcb_dpms_get_version: LazySymbol<unsafe fn(c: *mut xcb_connection_t, client_major_version: u16, client_minor_version: u16) -> xcb_dpms_get_version_cookie_t>,
    pub(crate) xcb_dpms_get_version_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_dpms_get_version_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_dpms_get_version_reply_t>,
    pub(crate) xcb_dpms_get_version_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, client_major_version: u16, client_minor_version: u16) -> xcb_dpms_get_version_cookie_t>,
    pub(crate) xcb_dpms_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_dpms_info: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_dpms_info_cookie_t>,
    pub(crate) xcb_dpms_info_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_dpms_info_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_dpms_info_reply_t>,
    pub(crate) xcb_dpms_info_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_dpms_info_cookie_t>,
    pub(crate) xcb_dpms_set_timeouts: LazySymbol<unsafe fn(c: *mut xcb_connection_t, standby_timeout: u16, suspend_timeout: u16, off_timeout: u16) -> xcb_void_cookie_t>,
    pub(crate) xcb_dpms_set_timeouts_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, standby_timeout: u16, suspend_timeout: u16, off_timeout: u16) -> xcb_void_cookie_t>,
}
