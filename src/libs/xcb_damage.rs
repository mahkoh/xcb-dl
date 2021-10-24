use crate::ffi::*;
use crate::lazy::*;
use libloading::{Error, Library};
use std::os::raw::*;

#[rustfmt::skip]
pub struct XcbDamage {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_damage_add: LazySymbol<unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t, region: xcb_xfixes_region_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_damage_add_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t, region: xcb_xfixes_region_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_damage_create: LazySymbol<unsafe fn(c: *mut xcb_connection_t, damage: xcb_damage_damage_t, drawable: xcb_drawable_t, level: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_damage_create_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, damage: xcb_damage_damage_t, drawable: xcb_drawable_t, level: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_damage_damage_end: LazySymbol<unsafe fn(i: *mut xcb_damage_damage_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_damage_damage_next: LazySymbol<unsafe fn(i: *mut xcb_damage_damage_iterator_t)>,
    pub(crate) xcb_damage_destroy: LazySymbol<unsafe fn(c: *mut xcb_connection_t, damage: xcb_damage_damage_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_damage_destroy_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, damage: xcb_damage_damage_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_damage_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_damage_query_version: LazySymbol<unsafe fn(c: *mut xcb_connection_t, client_major_version: u32, client_minor_version: u32) -> xcb_damage_query_version_cookie_t>,
    pub(crate) xcb_damage_query_version_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_damage_query_version_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_damage_query_version_reply_t>,
    pub(crate) xcb_damage_query_version_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, client_major_version: u32, client_minor_version: u32) -> xcb_damage_query_version_cookie_t>,
    pub(crate) xcb_damage_subtract: LazySymbol<unsafe fn(c: *mut xcb_connection_t, damage: xcb_damage_damage_t, repair: xcb_xfixes_region_t, parts: xcb_xfixes_region_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_damage_subtract_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, damage: xcb_damage_damage_t, repair: xcb_xfixes_region_t, parts: xcb_xfixes_region_t) -> xcb_void_cookie_t>,
}
