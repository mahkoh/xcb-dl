use crate::ffi::*;
use crate::lazy::*;
use libloading::{Error, Library};
use std::os::raw::*;

#[rustfmt::skip]
pub struct XcbShm {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_shm_attach: LazySymbol<unsafe fn(c: *mut xcb_connection_t, shmseg: xcb_shm_seg_t, shmid: u32, read_only: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_shm_attach_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, shmseg: xcb_shm_seg_t, shmid: u32, read_only: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_shm_attach_fd: LazySymbol<unsafe fn(c: *mut xcb_connection_t, shmseg: xcb_shm_seg_t, shm_fd: i32, read_only: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_shm_attach_fd_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, shmseg: xcb_shm_seg_t, shm_fd: i32, read_only: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_shm_create_pixmap: LazySymbol<unsafe fn(c: *mut xcb_connection_t, pid: xcb_pixmap_t, drawable: xcb_drawable_t, width: u16, height: u16, depth: u8, shmseg: xcb_shm_seg_t, offset: u32) -> xcb_void_cookie_t>,
    pub(crate) xcb_shm_create_pixmap_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, pid: xcb_pixmap_t, drawable: xcb_drawable_t, width: u16, height: u16, depth: u8, shmseg: xcb_shm_seg_t, offset: u32) -> xcb_void_cookie_t>,
    pub(crate) xcb_shm_create_segment: LazySymbol<unsafe fn(c: *mut xcb_connection_t, shmseg: xcb_shm_seg_t, size: u32, read_only: u8) -> xcb_shm_create_segment_cookie_t>,
    pub(crate) xcb_shm_create_segment_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_shm_create_segment_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_shm_create_segment_reply_t>,
    pub(crate) xcb_shm_create_segment_reply_fds: LazySymbol<unsafe fn(c: *mut xcb_connection_t, reply: *mut xcb_shm_create_segment_reply_t) -> *mut c_int>,
    pub(crate) xcb_shm_create_segment_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, shmseg: xcb_shm_seg_t, size: u32, read_only: u8) -> xcb_shm_create_segment_cookie_t>,
    pub(crate) xcb_shm_detach: LazySymbol<unsafe fn(c: *mut xcb_connection_t, shmseg: xcb_shm_seg_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_shm_detach_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, shmseg: xcb_shm_seg_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_shm_get_image: LazySymbol<unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t, x: i16, y: i16, width: u16, height: u16, plane_mask: u32, format: u8, shmseg: xcb_shm_seg_t, offset: u32) -> xcb_shm_get_image_cookie_t>,
    pub(crate) xcb_shm_get_image_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_shm_get_image_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_shm_get_image_reply_t>,
    pub(crate) xcb_shm_get_image_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t, x: i16, y: i16, width: u16, height: u16, plane_mask: u32, format: u8, shmseg: xcb_shm_seg_t, offset: u32) -> xcb_shm_get_image_cookie_t>,
    pub(crate) xcb_shm_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_shm_put_image: LazySymbol<unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t, gc: xcb_gcontext_t, total_width: u16, total_height: u16, src_x: u16, src_y: u16, src_width: u16, src_height: u16, dst_x: i16, dst_y: i16, depth: u8, format: u8, send_event: u8, shmseg: xcb_shm_seg_t, offset: u32) -> xcb_void_cookie_t>,
    pub(crate) xcb_shm_put_image_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t, gc: xcb_gcontext_t, total_width: u16, total_height: u16, src_x: u16, src_y: u16, src_width: u16, src_height: u16, dst_x: i16, dst_y: i16, depth: u8, format: u8, send_event: u8, shmseg: xcb_shm_seg_t, offset: u32) -> xcb_void_cookie_t>,
    pub(crate) xcb_shm_query_version: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_shm_query_version_cookie_t>,
    pub(crate) xcb_shm_query_version_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_shm_query_version_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_shm_query_version_reply_t>,
    pub(crate) xcb_shm_query_version_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_shm_query_version_cookie_t>,
    pub(crate) xcb_shm_seg_end: LazySymbol<unsafe fn(i: *mut xcb_shm_seg_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_shm_seg_next: LazySymbol<unsafe fn(i: *mut xcb_shm_seg_iterator_t)>,
}
