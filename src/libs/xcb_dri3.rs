use crate::ffi::*;
use crate::lazy::*;
use libloading::{Error, Library};
use std::os::raw::*;

#[rustfmt::skip]
pub struct XcbDri3 {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_dri3_buffer_from_pixmap: LazySymbol<unsafe fn(c: *mut xcb_connection_t, pixmap: xcb_pixmap_t) -> xcb_dri3_buffer_from_pixmap_cookie_t>,
    pub(crate) xcb_dri3_buffer_from_pixmap_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_dri3_buffer_from_pixmap_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_dri3_buffer_from_pixmap_reply_t>,
    pub(crate) xcb_dri3_buffer_from_pixmap_reply_fds: LazySymbol<unsafe fn(c: *mut xcb_connection_t, reply: *mut xcb_dri3_buffer_from_pixmap_reply_t) -> *mut c_int>,
    pub(crate) xcb_dri3_buffer_from_pixmap_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, pixmap: xcb_pixmap_t) -> xcb_dri3_buffer_from_pixmap_cookie_t>,
    pub(crate) xcb_dri3_fd_from_fence: LazySymbol<unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t, fence: u32) -> xcb_dri3_fd_from_fence_cookie_t>,
    pub(crate) xcb_dri3_fd_from_fence_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_dri3_fd_from_fence_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_dri3_fd_from_fence_reply_t>,
    pub(crate) xcb_dri3_fd_from_fence_reply_fds: LazySymbol<unsafe fn(c: *mut xcb_connection_t, reply: *mut xcb_dri3_fd_from_fence_reply_t) -> *mut c_int>,
    pub(crate) xcb_dri3_fd_from_fence_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t, fence: u32) -> xcb_dri3_fd_from_fence_cookie_t>,
    pub(crate) xcb_dri3_fence_from_fd: LazySymbol<unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t, fence: u32, initially_triggered: u8, fence_fd: i32) -> xcb_void_cookie_t>,
    pub(crate) xcb_dri3_fence_from_fd_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t, fence: u32, initially_triggered: u8, fence_fd: i32) -> xcb_void_cookie_t>,
    pub(crate) xcb_dri3_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_dri3_open: LazySymbol<unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t, provider: u32) -> xcb_dri3_open_cookie_t>,
    pub(crate) xcb_dri3_open_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_dri3_open_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_dri3_open_reply_t>,
    pub(crate) xcb_dri3_open_reply_fds: LazySymbol<unsafe fn(c: *mut xcb_connection_t, reply: *mut xcb_dri3_open_reply_t) -> *mut c_int>,
    pub(crate) xcb_dri3_open_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t, provider: u32) -> xcb_dri3_open_cookie_t>,
    pub(crate) xcb_dri3_pixmap_from_buffer: LazySymbol<unsafe fn(c: *mut xcb_connection_t, pixmap: xcb_pixmap_t, drawable: xcb_drawable_t, size: u32, width: u16, height: u16, stride: u16, depth: u8, bpp: u8, pixmap_fd: i32) -> xcb_void_cookie_t>,
    pub(crate) xcb_dri3_pixmap_from_buffer_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, pixmap: xcb_pixmap_t, drawable: xcb_drawable_t, size: u32, width: u16, height: u16, stride: u16, depth: u8, bpp: u8, pixmap_fd: i32) -> xcb_void_cookie_t>,
    pub(crate) xcb_dri3_query_version: LazySymbol<unsafe fn(c: *mut xcb_connection_t, major_version: u32, minor_version: u32) -> xcb_dri3_query_version_cookie_t>,
    pub(crate) xcb_dri3_query_version_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_dri3_query_version_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_dri3_query_version_reply_t>,
    pub(crate) xcb_dri3_query_version_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, major_version: u32, minor_version: u32) -> xcb_dri3_query_version_cookie_t>,
}
