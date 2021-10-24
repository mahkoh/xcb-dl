use crate::lazy::*;
use crate::*;
use libloading::{Error, Library};
use std::os::raw::*;

#[rustfmt::skip]
pub struct XcbShape {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_shape_combine: LazySymbol<unsafe fn(c: *mut xcb_connection_t, operation: xcb_shape_op_t, destination_kind: xcb_shape_kind_t, source_kind: xcb_shape_kind_t, destination_window: xcb_window_t, x_offset: i16, y_offset: i16, source_window: xcb_window_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_shape_combine_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, operation: xcb_shape_op_t, destination_kind: xcb_shape_kind_t, source_kind: xcb_shape_kind_t, destination_window: xcb_window_t, x_offset: i16, y_offset: i16, source_window: xcb_window_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_shape_get_rectangles: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, source_kind: xcb_shape_kind_t) -> xcb_shape_get_rectangles_cookie_t>,
    pub(crate) xcb_shape_get_rectangles_rectangles: LazySymbol<unsafe fn(R: *const xcb_shape_get_rectangles_reply_t) -> *mut xcb_rectangle_t>,
    pub(crate) xcb_shape_get_rectangles_rectangles_iterator: LazySymbol<unsafe fn(R: *const xcb_shape_get_rectangles_reply_t) -> xcb_rectangle_iterator_t>,
    pub(crate) xcb_shape_get_rectangles_rectangles_length: LazySymbol<unsafe fn(R: *const xcb_shape_get_rectangles_reply_t) -> c_int>,
    pub(crate) xcb_shape_get_rectangles_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_shape_get_rectangles_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_shape_get_rectangles_reply_t>,
    pub(crate) xcb_shape_get_rectangles_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, source_kind: xcb_shape_kind_t) -> xcb_shape_get_rectangles_cookie_t>,
    pub(crate) xcb_shape_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_shape_input_selected: LazySymbol<unsafe fn(c: *mut xcb_connection_t, destination_window: xcb_window_t) -> xcb_shape_input_selected_cookie_t>,
    pub(crate) xcb_shape_input_selected_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_shape_input_selected_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_shape_input_selected_reply_t>,
    pub(crate) xcb_shape_input_selected_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, destination_window: xcb_window_t) -> xcb_shape_input_selected_cookie_t>,
    pub(crate) xcb_shape_kind_end: LazySymbol<unsafe fn(i: *mut xcb_shape_kind_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_shape_kind_next: LazySymbol<unsafe fn(i: *mut xcb_shape_kind_iterator_t)>,
    pub(crate) xcb_shape_mask: LazySymbol<unsafe fn(c: *mut xcb_connection_t, operation: xcb_shape_op_t, destination_kind: xcb_shape_kind_t, destination_window: xcb_window_t, x_offset: i16, y_offset: i16, source_bitmap: xcb_pixmap_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_shape_mask_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, operation: xcb_shape_op_t, destination_kind: xcb_shape_kind_t, destination_window: xcb_window_t, x_offset: i16, y_offset: i16, source_bitmap: xcb_pixmap_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_shape_offset: LazySymbol<unsafe fn(c: *mut xcb_connection_t, destination_kind: xcb_shape_kind_t, destination_window: xcb_window_t, x_offset: i16, y_offset: i16) -> xcb_void_cookie_t>,
    pub(crate) xcb_shape_offset_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, destination_kind: xcb_shape_kind_t, destination_window: xcb_window_t, x_offset: i16, y_offset: i16) -> xcb_void_cookie_t>,
    pub(crate) xcb_shape_op_end: LazySymbol<unsafe fn(i: *mut xcb_shape_op_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_shape_op_next: LazySymbol<unsafe fn(i: *mut xcb_shape_op_iterator_t)>,
    pub(crate) xcb_shape_query_extents: LazySymbol<unsafe fn(c: *mut xcb_connection_t, destination_window: xcb_window_t) -> xcb_shape_query_extents_cookie_t>,
    pub(crate) xcb_shape_query_extents_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_shape_query_extents_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_shape_query_extents_reply_t>,
    pub(crate) xcb_shape_query_extents_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, destination_window: xcb_window_t) -> xcb_shape_query_extents_cookie_t>,
    pub(crate) xcb_shape_query_version: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_shape_query_version_cookie_t>,
    pub(crate) xcb_shape_query_version_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_shape_query_version_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_shape_query_version_reply_t>,
    pub(crate) xcb_shape_query_version_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_shape_query_version_cookie_t>,
    pub(crate) xcb_shape_rectangles: LazySymbol<unsafe fn(c: *mut xcb_connection_t, operation: xcb_shape_op_t, destination_kind: xcb_shape_kind_t, ordering: u8, destination_window: xcb_window_t, x_offset: i16, y_offset: i16, rectangles_len: u32, rectangles: *const xcb_rectangle_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_shape_rectangles_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, operation: xcb_shape_op_t, destination_kind: xcb_shape_kind_t, ordering: u8, destination_window: xcb_window_t, x_offset: i16, y_offset: i16, rectangles_len: u32, rectangles: *const xcb_rectangle_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_shape_select_input: LazySymbol<unsafe fn(c: *mut xcb_connection_t, destination_window: xcb_window_t, enable: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_shape_select_input_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, destination_window: xcb_window_t, enable: u8) -> xcb_void_cookie_t>,
}
