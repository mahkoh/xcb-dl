// This file was generated using generate.py. Do not edit.

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_query_version_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xfixes_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_query_version.
pub const XCB_XFIXES_QUERY_VERSION: u8 = 0i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client_major_version: u32,
    pub client_minor_version: u32,
}

impl Default for xcb_xfixes_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
    pub pad1: [u8; 16],
}

impl Default for xcb_xfixes_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xfixes_save_set_mode_t = u32;
pub const XCB_XFIXES_SAVE_SET_MODE_INSERT: xcb_xfixes_save_set_mode_t = 0;
pub const XCB_XFIXES_SAVE_SET_MODE_DELETE: xcb_xfixes_save_set_mode_t = 1;

pub type xcb_xfixes_save_set_target_t = u32;
pub const XCB_XFIXES_SAVE_SET_TARGET_NEAREST: xcb_xfixes_save_set_target_t = 0;
pub const XCB_XFIXES_SAVE_SET_TARGET_ROOT: xcb_xfixes_save_set_target_t = 1;

pub type xcb_xfixes_save_set_mapping_t = u32;
pub const XCB_XFIXES_SAVE_SET_MAPPING_MAP: xcb_xfixes_save_set_mapping_t = 0;
pub const XCB_XFIXES_SAVE_SET_MAPPING_UNMAP: xcb_xfixes_save_set_mapping_t = 1;

/// Opcode for xcb_xfixes_change_save_set.
pub const XCB_XFIXES_CHANGE_SAVE_SET: u8 = 1i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_change_save_set_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub mode: u8,
    pub target: u8,
    pub map: u8,
    pub pad0: u8,
    pub window: xcb_window_t,
}

impl Default for xcb_xfixes_change_save_set_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xfixes_selection_event_t = u32;
pub const XCB_XFIXES_SELECTION_EVENT_SET_SELECTION_OWNER: xcb_xfixes_selection_event_t = 0;
pub const XCB_XFIXES_SELECTION_EVENT_SELECTION_WINDOW_DESTROY: xcb_xfixes_selection_event_t = 1;
pub const XCB_XFIXES_SELECTION_EVENT_SELECTION_CLIENT_CLOSE: xcb_xfixes_selection_event_t = 2;

pub type xcb_xfixes_selection_event_mask_t = u32;
pub const XCB_XFIXES_SELECTION_EVENT_MASK_SET_SELECTION_OWNER: xcb_xfixes_selection_event_mask_t =
    1;
pub const XCB_XFIXES_SELECTION_EVENT_MASK_SELECTION_WINDOW_DESTROY:
    xcb_xfixes_selection_event_mask_t = 2;
pub const XCB_XFIXES_SELECTION_EVENT_MASK_SELECTION_CLIENT_CLOSE:
    xcb_xfixes_selection_event_mask_t = 4;

/// Opcode for xcb_xfixes_selection_notify.
pub const XCB_XFIXES_SELECTION_NOTIFY: u8 = 0i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_selection_notify_event_t {
    pub response_type: u8,
    pub subtype: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub owner: xcb_window_t,
    pub selection: xcb_atom_t,
    pub timestamp: xcb_timestamp_t,
    pub selection_timestamp: xcb_timestamp_t,
    pub pad0: [u8; 8],
}

impl Default for xcb_xfixes_selection_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_select_selection_input.
pub const XCB_XFIXES_SELECT_SELECTION_INPUT: u8 = 2i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_select_selection_input_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub selection: xcb_atom_t,
    pub event_mask: u32,
}

impl Default for xcb_xfixes_select_selection_input_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xfixes_cursor_notify_t = u32;
pub const XCB_XFIXES_CURSOR_NOTIFY_DISPLAY_CURSOR: xcb_xfixes_cursor_notify_t = 0;

pub type xcb_xfixes_cursor_notify_mask_t = u32;
pub const XCB_XFIXES_CURSOR_NOTIFY_MASK_DISPLAY_CURSOR: xcb_xfixes_cursor_notify_mask_t = 1;

/// Opcode for xcb_xfixes_cursor_notify.
pub const XCB_XFIXES_CURSOR_NOTIFY: u8 = 1i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_cursor_notify_event_t {
    pub response_type: u8,
    pub subtype: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub cursor_serial: u32,
    pub timestamp: xcb_timestamp_t,
    pub name: xcb_atom_t,
    pub pad0: [u8; 12],
}

impl Default for xcb_xfixes_cursor_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_select_cursor_input.
pub const XCB_XFIXES_SELECT_CURSOR_INPUT: u8 = 3i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_select_cursor_input_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub event_mask: u32,
}

impl Default for xcb_xfixes_select_cursor_input_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_image_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xfixes_get_cursor_image_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_get_cursor_image.
pub const XCB_XFIXES_GET_CURSOR_IMAGE: u8 = 4i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_image_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_xfixes_get_cursor_image_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_image_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub xhot: u16,
    pub yhot: u16,
    pub cursor_serial: u32,
    pub pad1: [u8; 8],
}

impl Default for xcb_xfixes_get_cursor_image_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xfixes_region_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_region_iterator_t {
    pub data: *mut xcb_xfixes_region_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xfixes_region_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_bad_region.
pub const XCB_XFIXES_BAD_REGION: u8 = 0i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_bad_region_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_xfixes_bad_region_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xfixes_region_enum_t = u32;
pub const XCB_XFIXES_REGION_NONE: xcb_xfixes_region_enum_t = 0;

/// Opcode for xcb_xfixes_create_region.
pub const XCB_XFIXES_CREATE_REGION: u8 = 5i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_create_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
}

impl Default for xcb_xfixes_create_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_create_region_from_bitmap.
pub const XCB_XFIXES_CREATE_REGION_FROM_BITMAP: u8 = 6i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_create_region_from_bitmap_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
    pub bitmap: xcb_pixmap_t,
}

impl Default for xcb_xfixes_create_region_from_bitmap_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_create_region_from_window.
pub const XCB_XFIXES_CREATE_REGION_FROM_WINDOW: u8 = 7i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_create_region_from_window_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
    pub window: xcb_window_t,
    pub kind: xcb_shape_kind_t,
    pub pad0: [u8; 3],
}

impl Default for xcb_xfixes_create_region_from_window_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_create_region_from_gc.
pub const XCB_XFIXES_CREATE_REGION_FROM_GC: u8 = 8i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_create_region_from_gc_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
    pub gc: xcb_gcontext_t,
}

impl Default for xcb_xfixes_create_region_from_gc_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_create_region_from_picture.
pub const XCB_XFIXES_CREATE_REGION_FROM_PICTURE: u8 = 9i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_create_region_from_picture_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
    pub picture: xcb_render_picture_t,
}

impl Default for xcb_xfixes_create_region_from_picture_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_destroy_region.
pub const XCB_XFIXES_DESTROY_REGION: u8 = 10i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_destroy_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
}

impl Default for xcb_xfixes_destroy_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_set_region.
pub const XCB_XFIXES_SET_REGION: u8 = 11i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_set_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
}

impl Default for xcb_xfixes_set_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_copy_region.
pub const XCB_XFIXES_COPY_REGION: u8 = 12i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_copy_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub source: xcb_xfixes_region_t,
    pub destination: xcb_xfixes_region_t,
}

impl Default for xcb_xfixes_copy_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_union_region.
pub const XCB_XFIXES_UNION_REGION: u8 = 13i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_union_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub source1: xcb_xfixes_region_t,
    pub source2: xcb_xfixes_region_t,
    pub destination: xcb_xfixes_region_t,
}

impl Default for xcb_xfixes_union_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_intersect_region.
pub const XCB_XFIXES_INTERSECT_REGION: u8 = 14i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_intersect_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub source1: xcb_xfixes_region_t,
    pub source2: xcb_xfixes_region_t,
    pub destination: xcb_xfixes_region_t,
}

impl Default for xcb_xfixes_intersect_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_subtract_region.
pub const XCB_XFIXES_SUBTRACT_REGION: u8 = 15i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_subtract_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub source1: xcb_xfixes_region_t,
    pub source2: xcb_xfixes_region_t,
    pub destination: xcb_xfixes_region_t,
}

impl Default for xcb_xfixes_subtract_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_invert_region.
pub const XCB_XFIXES_INVERT_REGION: u8 = 16i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_invert_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub source: xcb_xfixes_region_t,
    pub bounds: xcb_rectangle_t,
    pub destination: xcb_xfixes_region_t,
}

impl Default for xcb_xfixes_invert_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_translate_region.
pub const XCB_XFIXES_TRANSLATE_REGION: u8 = 17i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_translate_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
    pub dx: i16,
    pub dy: i16,
}

impl Default for xcb_xfixes_translate_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_region_extents.
pub const XCB_XFIXES_REGION_EXTENTS: u8 = 18i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_region_extents_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub source: xcb_xfixes_region_t,
    pub destination: xcb_xfixes_region_t,
}

impl Default for xcb_xfixes_region_extents_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_fetch_region_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xfixes_fetch_region_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_fetch_region.
pub const XCB_XFIXES_FETCH_REGION: u8 = 19i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_fetch_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
}

impl Default for xcb_xfixes_fetch_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_fetch_region_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub extents: xcb_rectangle_t,
    pub pad1: [u8; 16],
}

impl Default for xcb_xfixes_fetch_region_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_set_gc_clip_region.
pub const XCB_XFIXES_SET_GC_CLIP_REGION: u8 = 20i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_set_gc_clip_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub gc: xcb_gcontext_t,
    pub region: xcb_xfixes_region_t,
    pub x_origin: i16,
    pub y_origin: i16,
}

impl Default for xcb_xfixes_set_gc_clip_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_set_window_shape_region.
pub const XCB_XFIXES_SET_WINDOW_SHAPE_REGION: u8 = 21i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_set_window_shape_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub dest: xcb_window_t,
    pub dest_kind: xcb_shape_kind_t,
    pub pad0: [u8; 3],
    pub x_offset: i16,
    pub y_offset: i16,
    pub region: xcb_xfixes_region_t,
}

impl Default for xcb_xfixes_set_window_shape_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_set_picture_clip_region.
pub const XCB_XFIXES_SET_PICTURE_CLIP_REGION: u8 = 22i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_set_picture_clip_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub picture: xcb_render_picture_t,
    pub region: xcb_xfixes_region_t,
    pub x_origin: i16,
    pub y_origin: i16,
}

impl Default for xcb_xfixes_set_picture_clip_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_set_cursor_name.
pub const XCB_XFIXES_SET_CURSOR_NAME: u8 = 23i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_set_cursor_name_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub cursor: xcb_cursor_t,
    pub nbytes: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_xfixes_set_cursor_name_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_name_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xfixes_get_cursor_name_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_get_cursor_name.
pub const XCB_XFIXES_GET_CURSOR_NAME: u8 = 24i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_name_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub cursor: xcb_cursor_t,
}

impl Default for xcb_xfixes_get_cursor_name_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_name_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub atom: xcb_atom_t,
    pub nbytes: u16,
    pub pad1: [u8; 18],
}

impl Default for xcb_xfixes_get_cursor_name_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_image_and_name_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xfixes_get_cursor_image_and_name_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_get_cursor_image_and_name.
pub const XCB_XFIXES_GET_CURSOR_IMAGE_AND_NAME: u8 = 25i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_image_and_name_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_xfixes_get_cursor_image_and_name_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_image_and_name_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub xhot: u16,
    pub yhot: u16,
    pub cursor_serial: u32,
    pub cursor_atom: xcb_atom_t,
    pub nbytes: u16,
    pub pad1: [u8; 2],
}

impl Default for xcb_xfixes_get_cursor_image_and_name_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_change_cursor.
pub const XCB_XFIXES_CHANGE_CURSOR: u8 = 26i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_change_cursor_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub source: xcb_cursor_t,
    pub destination: xcb_cursor_t,
}

impl Default for xcb_xfixes_change_cursor_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_change_cursor_by_name.
pub const XCB_XFIXES_CHANGE_CURSOR_BY_NAME: u8 = 27i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_change_cursor_by_name_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub src: xcb_cursor_t,
    pub nbytes: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_xfixes_change_cursor_by_name_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_expand_region.
pub const XCB_XFIXES_EXPAND_REGION: u8 = 28i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_expand_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub source: xcb_xfixes_region_t,
    pub destination: xcb_xfixes_region_t,
    pub left: u16,
    pub right: u16,
    pub top: u16,
    pub bottom: u16,
}

impl Default for xcb_xfixes_expand_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_hide_cursor.
pub const XCB_XFIXES_HIDE_CURSOR: u8 = 29i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_hide_cursor_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_xfixes_hide_cursor_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_show_cursor.
pub const XCB_XFIXES_SHOW_CURSOR: u8 = 30i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_show_cursor_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_xfixes_show_cursor_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xfixes_barrier_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_barrier_iterator_t {
    pub data: *mut xcb_xfixes_barrier_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xfixes_barrier_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xfixes_barrier_directions_t = u32;
pub const XCB_XFIXES_BARRIER_DIRECTIONS_POSITIVE_X: xcb_xfixes_barrier_directions_t = 1;
pub const XCB_XFIXES_BARRIER_DIRECTIONS_POSITIVE_Y: xcb_xfixes_barrier_directions_t = 2;
pub const XCB_XFIXES_BARRIER_DIRECTIONS_NEGATIVE_X: xcb_xfixes_barrier_directions_t = 4;
pub const XCB_XFIXES_BARRIER_DIRECTIONS_NEGATIVE_Y: xcb_xfixes_barrier_directions_t = 8;

/// Opcode for xcb_xfixes_create_pointer_barrier.
pub const XCB_XFIXES_CREATE_POINTER_BARRIER: u8 = 31i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_create_pointer_barrier_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub barrier: xcb_xfixes_barrier_t,
    pub window: xcb_window_t,
    pub x1: u16,
    pub y1: u16,
    pub x2: u16,
    pub y2: u16,
    pub directions: u32,
    pub pad0: [u8; 2],
    pub num_devices: u16,
}

impl Default for xcb_xfixes_create_pointer_barrier_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xfixes_delete_pointer_barrier.
pub const XCB_XFIXES_DELETE_POINTER_BARRIER: u8 = 32i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_delete_pointer_barrier_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub barrier: xcb_xfixes_barrier_t,
}

impl Default for xcb_xfixes_delete_pointer_barrier_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub(crate) struct XcbXfixesXfixes {
    xcb_xfixes_id: LazySymbol<*mut xcb_extension_t>,
    xcb_xfixes_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u32,
            client_minor_version: u32,
        ) -> xcb_xfixes_query_version_cookie_t,
    >,
    xcb_xfixes_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u32,
            client_minor_version: u32,
        ) -> xcb_xfixes_query_version_cookie_t,
    >,
    xcb_xfixes_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xfixes_query_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_xfixes_query_version_reply_t,
    >,
    xcb_xfixes_change_save_set_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            mode: u8,
            target: u8,
            map: u8,
            window: xcb_window_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_change_save_set: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            mode: u8,
            target: u8,
            map: u8,
            window: xcb_window_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_select_selection_input_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            selection: xcb_atom_t,
            event_mask: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_select_selection_input: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            selection: xcb_atom_t,
            event_mask: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_select_cursor_input_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            event_mask: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_select_cursor_input: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            event_mask: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_get_cursor_image_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xfixes_get_cursor_image:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xfixes_get_cursor_image_cookie_t>,
    xcb_xfixes_get_cursor_image_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xfixes_get_cursor_image_cookie_t>,
    xcb_xfixes_get_cursor_image_cursor_image:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_get_cursor_image_reply_t) -> *mut u32>,
    xcb_xfixes_get_cursor_image_cursor_image_length:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_get_cursor_image_reply_t) -> c_int>,
    xcb_xfixes_get_cursor_image_cursor_image_end: LazySymbol<
        unsafe fn(r: *const xcb_xfixes_get_cursor_image_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_xfixes_get_cursor_image_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xfixes_get_cursor_image_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_xfixes_get_cursor_image_reply_t,
    >,
    xcb_xfixes_region_next: LazySymbol<unsafe fn(i: *mut xcb_xfixes_region_iterator_t)>,
    xcb_xfixes_region_end:
        LazySymbol<unsafe fn(i: xcb_xfixes_region_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xfixes_create_region_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, rectangles_len: u32) -> c_int>,
    xcb_xfixes_create_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_create_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_create_region_rectangles:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_create_region_request_t) -> *mut xcb_rectangle_t>,
    xcb_xfixes_create_region_rectangles_length:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_create_region_request_t) -> c_int>,
    xcb_xfixes_create_region_rectangles_iterator: LazySymbol<
        unsafe fn(r: *const xcb_xfixes_create_region_request_t) -> xcb_rectangle_iterator_t,
    >,
    xcb_xfixes_create_region_from_bitmap_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            bitmap: xcb_pixmap_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_create_region_from_bitmap: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            bitmap: xcb_pixmap_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_create_region_from_window_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            window: xcb_window_t,
            kind: xcb_shape_kind_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_create_region_from_window: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            window: xcb_window_t,
            kind: xcb_shape_kind_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_create_region_from_gc_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            gc: xcb_gcontext_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_create_region_from_gc: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            gc: xcb_gcontext_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_create_region_from_picture_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            picture: xcb_render_picture_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_create_region_from_picture: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            picture: xcb_render_picture_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_destroy_region_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, region: xcb_xfixes_region_t) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_destroy_region: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, region: xcb_xfixes_region_t) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_set_region_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, rectangles_len: u32) -> c_int>,
    xcb_xfixes_set_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_set_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_set_region_rectangles:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_set_region_request_t) -> *mut xcb_rectangle_t>,
    xcb_xfixes_set_region_rectangles_length:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_set_region_request_t) -> c_int>,
    xcb_xfixes_set_region_rectangles_iterator: LazySymbol<
        unsafe fn(r: *const xcb_xfixes_set_region_request_t) -> xcb_rectangle_iterator_t,
    >,
    xcb_xfixes_copy_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_copy_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_union_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source1: xcb_xfixes_region_t,
            source2: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_union_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source1: xcb_xfixes_region_t,
            source2: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_intersect_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source1: xcb_xfixes_region_t,
            source2: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_intersect_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source1: xcb_xfixes_region_t,
            source2: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_subtract_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source1: xcb_xfixes_region_t,
            source2: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_subtract_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source1: xcb_xfixes_region_t,
            source2: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_invert_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_xfixes_region_t,
            bounds: xcb_rectangle_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_invert_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_xfixes_region_t,
            bounds: xcb_rectangle_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_translate_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            dx: i16,
            dy: i16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_translate_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            dx: i16,
            dy: i16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_region_extents_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_region_extents: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_fetch_region_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xfixes_fetch_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
        ) -> xcb_xfixes_fetch_region_cookie_t,
    >,
    xcb_xfixes_fetch_region_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
        ) -> xcb_xfixes_fetch_region_cookie_t,
    >,
    xcb_xfixes_fetch_region_rectangles:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_fetch_region_reply_t) -> *mut xcb_rectangle_t>,
    xcb_xfixes_fetch_region_rectangles_length:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_fetch_region_reply_t) -> c_int>,
    xcb_xfixes_fetch_region_rectangles_iterator: LazySymbol<
        unsafe fn(r: *const xcb_xfixes_fetch_region_reply_t) -> xcb_rectangle_iterator_t,
    >,
    xcb_xfixes_fetch_region_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xfixes_fetch_region_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_xfixes_fetch_region_reply_t,
    >,
    xcb_xfixes_set_gc_clip_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gc: xcb_gcontext_t,
            region: xcb_xfixes_region_t,
            x_origin: i16,
            y_origin: i16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_set_gc_clip_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gc: xcb_gcontext_t,
            region: xcb_xfixes_region_t,
            x_origin: i16,
            y_origin: i16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_set_window_shape_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            dest: xcb_window_t,
            dest_kind: xcb_shape_kind_t,
            x_offset: i16,
            y_offset: i16,
            region: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_set_window_shape_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            dest: xcb_window_t,
            dest_kind: xcb_shape_kind_t,
            x_offset: i16,
            y_offset: i16,
            region: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_set_picture_clip_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            region: xcb_xfixes_region_t,
            x_origin: i16,
            y_origin: i16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_set_picture_clip_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            region: xcb_xfixes_region_t,
            x_origin: i16,
            y_origin: i16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_set_cursor_name_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xfixes_set_cursor_name_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cursor: xcb_cursor_t,
            nbytes: u16,
            name: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_set_cursor_name: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cursor: xcb_cursor_t,
            nbytes: u16,
            name: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_set_cursor_name_name:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_set_cursor_name_request_t) -> *mut c_char>,
    xcb_xfixes_set_cursor_name_name_length:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_set_cursor_name_request_t) -> c_int>,
    xcb_xfixes_set_cursor_name_name_end: LazySymbol<
        unsafe fn(r: *const xcb_xfixes_set_cursor_name_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_xfixes_get_cursor_name_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xfixes_get_cursor_name: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cursor: xcb_cursor_t,
        ) -> xcb_xfixes_get_cursor_name_cookie_t,
    >,
    xcb_xfixes_get_cursor_name_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cursor: xcb_cursor_t,
        ) -> xcb_xfixes_get_cursor_name_cookie_t,
    >,
    xcb_xfixes_get_cursor_name_name:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_get_cursor_name_reply_t) -> *mut c_char>,
    xcb_xfixes_get_cursor_name_name_length:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_get_cursor_name_reply_t) -> c_int>,
    xcb_xfixes_get_cursor_name_name_end: LazySymbol<
        unsafe fn(r: *const xcb_xfixes_get_cursor_name_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_xfixes_get_cursor_name_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xfixes_get_cursor_name_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_xfixes_get_cursor_name_reply_t,
    >,
    xcb_xfixes_get_cursor_image_and_name_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xfixes_get_cursor_image_and_name: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_xfixes_get_cursor_image_and_name_cookie_t,
    >,
    xcb_xfixes_get_cursor_image_and_name_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_xfixes_get_cursor_image_and_name_cookie_t,
    >,
    xcb_xfixes_get_cursor_image_and_name_cursor_image:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_get_cursor_image_and_name_reply_t) -> *mut u32>,
    xcb_xfixes_get_cursor_image_and_name_cursor_image_length:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_get_cursor_image_and_name_reply_t) -> c_int>,
    xcb_xfixes_get_cursor_image_and_name_cursor_image_end: LazySymbol<
        unsafe fn(r: *const xcb_xfixes_get_cursor_image_and_name_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_xfixes_get_cursor_image_and_name_name: LazySymbol<
        unsafe fn(r: *const xcb_xfixes_get_cursor_image_and_name_reply_t) -> *mut c_char,
    >,
    xcb_xfixes_get_cursor_image_and_name_name_length:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_get_cursor_image_and_name_reply_t) -> c_int>,
    xcb_xfixes_get_cursor_image_and_name_name_end: LazySymbol<
        unsafe fn(r: *const xcb_xfixes_get_cursor_image_and_name_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_xfixes_get_cursor_image_and_name_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xfixes_get_cursor_image_and_name_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_xfixes_get_cursor_image_and_name_reply_t,
    >,
    xcb_xfixes_change_cursor_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_cursor_t,
            destination: xcb_cursor_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_change_cursor: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_cursor_t,
            destination: xcb_cursor_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_change_cursor_by_name_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xfixes_change_cursor_by_name_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src: xcb_cursor_t,
            nbytes: u16,
            name: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_change_cursor_by_name: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src: xcb_cursor_t,
            nbytes: u16,
            name: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_change_cursor_by_name_name:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_change_cursor_by_name_request_t) -> *mut c_char>,
    xcb_xfixes_change_cursor_by_name_name_length:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_change_cursor_by_name_request_t) -> c_int>,
    xcb_xfixes_change_cursor_by_name_name_end: LazySymbol<
        unsafe fn(r: *const xcb_xfixes_change_cursor_by_name_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_xfixes_expand_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
            left: u16,
            right: u16,
            top: u16,
            bottom: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_expand_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
            left: u16,
            right: u16,
            top: u16,
            bottom: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_hide_cursor_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_xfixes_hide_cursor:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_xfixes_show_cursor_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_xfixes_show_cursor:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_xfixes_barrier_next: LazySymbol<unsafe fn(i: *mut xcb_xfixes_barrier_iterator_t)>,
    xcb_xfixes_barrier_end:
        LazySymbol<unsafe fn(i: xcb_xfixes_barrier_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xfixes_create_pointer_barrier_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xfixes_create_pointer_barrier_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            barrier: xcb_xfixes_barrier_t,
            window: xcb_window_t,
            x1: u16,
            y1: u16,
            x2: u16,
            y2: u16,
            directions: u32,
            num_devices: u16,
            devices: *const u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_create_pointer_barrier: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            barrier: xcb_xfixes_barrier_t,
            window: xcb_window_t,
            x1: u16,
            y1: u16,
            x2: u16,
            y2: u16,
            directions: u32,
            num_devices: u16,
            devices: *const u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_create_pointer_barrier_devices:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_create_pointer_barrier_request_t) -> *mut u16>,
    xcb_xfixes_create_pointer_barrier_devices_length:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_create_pointer_barrier_request_t) -> c_int>,
    xcb_xfixes_create_pointer_barrier_devices_end: LazySymbol<
        unsafe fn(r: *const xcb_xfixes_create_pointer_barrier_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_xfixes_delete_pointer_barrier_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, barrier: xcb_xfixes_barrier_t) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_delete_pointer_barrier: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, barrier: xcb_xfixes_barrier_t) -> xcb_void_cookie_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self
            .xfixes
            .$f
            .get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .xfixes
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

impl XcbXfixes {
    pub fn xcb_xfixes_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_xfixes_id) }
    }

    /// Returns `true` iff the symbol `xcb_xfixes_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_id(&self) -> bool {
        has_sym!(self, xcb_xfixes_id)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_query_version(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u32,
        client_minor_version: u32,
    ) -> xcb_xfixes_query_version_cookie_t {
        sym!(self, xcb_xfixes_query_version)(c, client_major_version, client_minor_version)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_query_version(&self) -> bool {
        has_sym!(self, xcb_xfixes_query_version)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will cause
     * a reply to be generated. Any returned error will be
     * placed in the event queue.
     */
    pub unsafe fn xcb_xfixes_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u32,
        client_minor_version: u32,
    ) -> xcb_xfixes_query_version_cookie_t {
        sym!(self, xcb_xfixes_query_version_unchecked)(
            c,
            client_major_version,
            client_minor_version,
        )
    }

    /// Returns `true` iff the symbol `xcb_xfixes_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_query_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_xfixes_query_version_unchecked)
    }

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_xfixes_query_version_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xfixes_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xfixes_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xfixes_query_version_reply_t {
        sym!(self, xcb_xfixes_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_xfixes_query_version_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_change_save_set_checked(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
        target: u8,
        map: u8,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_change_save_set_checked)(c, mode, target, map, window)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_change_save_set_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_change_save_set_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_change_save_set_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_change_save_set(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
        target: u8,
        map: u8,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_change_save_set)(c, mode, target, map, window)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_change_save_set` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_change_save_set(&self) -> bool {
        has_sym!(self, xcb_xfixes_change_save_set)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_select_selection_input_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        selection: xcb_atom_t,
        event_mask: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_select_selection_input_checked)(c, window, selection, event_mask)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_select_selection_input_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_select_selection_input_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_select_selection_input_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_select_selection_input(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        selection: xcb_atom_t,
        event_mask: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_select_selection_input)(c, window, selection, event_mask)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_select_selection_input` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_select_selection_input(&self) -> bool {
        has_sym!(self, xcb_xfixes_select_selection_input)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_select_cursor_input_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        event_mask: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_select_cursor_input_checked)(c, window, event_mask)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_select_cursor_input_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_select_cursor_input_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_select_cursor_input_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_select_cursor_input(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        event_mask: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_select_cursor_input)(c, window, event_mask)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_select_cursor_input` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_select_cursor_input(&self) -> bool {
        has_sym!(self, xcb_xfixes_select_cursor_input)
    }

    pub unsafe fn xcb_xfixes_get_cursor_image_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xfixes_get_cursor_image_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_sizeof(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_get_cursor_image(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xfixes_get_cursor_image_cookie_t {
        sym!(self, xcb_xfixes_get_cursor_image)(c)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will cause
     * a reply to be generated. Any returned error will be
     * placed in the event queue.
     */
    pub unsafe fn xcb_xfixes_get_cursor_image_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xfixes_get_cursor_image_cookie_t {
        sym!(self, xcb_xfixes_get_cursor_image_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_unchecked(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_unchecked)
    }

    pub unsafe fn xcb_xfixes_get_cursor_image_cursor_image(
        &self,
        r: *const xcb_xfixes_get_cursor_image_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_xfixes_get_cursor_image_cursor_image)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_cursor_image` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_cursor_image(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_cursor_image)
    }

    pub unsafe fn xcb_xfixes_get_cursor_image_cursor_image_length(
        &self,
        r: *const xcb_xfixes_get_cursor_image_reply_t,
    ) -> c_int {
        sym!(self, xcb_xfixes_get_cursor_image_cursor_image_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_cursor_image_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_cursor_image_length(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_cursor_image_length)
    }

    pub unsafe fn xcb_xfixes_get_cursor_image_cursor_image_end(
        &self,
        r: *const xcb_xfixes_get_cursor_image_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xfixes_get_cursor_image_cursor_image_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_cursor_image_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_cursor_image_end(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_cursor_image_end)
    }

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_xfixes_get_cursor_image_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xfixes_get_cursor_image_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xfixes_get_cursor_image_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xfixes_get_cursor_image_reply_t {
        sym!(self, xcb_xfixes_get_cursor_image_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_reply(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_reply)
    }

    pub unsafe fn xcb_xfixes_region_next(&self, i: *mut xcb_xfixes_region_iterator_t) {
        sym!(self, xcb_xfixes_region_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_region_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_region_next(&self) -> bool {
        has_sym!(self, xcb_xfixes_region_next)
    }

    pub unsafe fn xcb_xfixes_region_end(
        &self,
        i: xcb_xfixes_region_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xfixes_region_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_region_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_region_end(&self) -> bool {
        has_sym!(self, xcb_xfixes_region_end)
    }

    pub unsafe fn xcb_xfixes_create_region_sizeof(
        &self,
        _buffer: *const c_void,
        rectangles_len: u32,
    ) -> c_int {
        sym!(self, xcb_xfixes_create_region_sizeof)(_buffer, rectangles_len)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_sizeof(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_create_region_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_create_region_checked)(c, region, rectangles_len, rectangles)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_create_region(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_create_region)(c, region, rectangles_len, rectangles)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region)
    }

    pub unsafe fn xcb_xfixes_create_region_rectangles(
        &self,
        r: *const xcb_xfixes_create_region_request_t,
    ) -> *mut xcb_rectangle_t {
        sym!(self, xcb_xfixes_create_region_rectangles)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_rectangles` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_rectangles(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_rectangles)
    }

    pub unsafe fn xcb_xfixes_create_region_rectangles_length(
        &self,
        r: *const xcb_xfixes_create_region_request_t,
    ) -> c_int {
        sym!(self, xcb_xfixes_create_region_rectangles_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_rectangles_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_rectangles_length(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_rectangles_length)
    }

    pub unsafe fn xcb_xfixes_create_region_rectangles_iterator(
        &self,
        r: *const xcb_xfixes_create_region_request_t,
    ) -> xcb_rectangle_iterator_t {
        sym!(self, xcb_xfixes_create_region_rectangles_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_rectangles_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_rectangles_iterator(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_rectangles_iterator)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_create_region_from_bitmap_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        bitmap: xcb_pixmap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_create_region_from_bitmap_checked)(c, region, bitmap)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_from_bitmap_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_from_bitmap_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_from_bitmap_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_create_region_from_bitmap(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        bitmap: xcb_pixmap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_create_region_from_bitmap)(c, region, bitmap)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_from_bitmap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_from_bitmap(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_from_bitmap)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_create_region_from_window_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        window: xcb_window_t,
        kind: xcb_shape_kind_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_create_region_from_window_checked)(c, region, window, kind)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_from_window_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_from_window_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_from_window_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_create_region_from_window(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        window: xcb_window_t,
        kind: xcb_shape_kind_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_create_region_from_window)(c, region, window, kind)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_from_window` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_from_window(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_from_window)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_create_region_from_gc_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        gc: xcb_gcontext_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_create_region_from_gc_checked)(c, region, gc)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_from_gc_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_from_gc_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_from_gc_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_create_region_from_gc(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        gc: xcb_gcontext_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_create_region_from_gc)(c, region, gc)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_from_gc` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_from_gc(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_from_gc)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_create_region_from_picture_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        picture: xcb_render_picture_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_create_region_from_picture_checked)(c, region, picture)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_from_picture_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_from_picture_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_from_picture_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_create_region_from_picture(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        picture: xcb_render_picture_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_create_region_from_picture)(c, region, picture)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_from_picture` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_from_picture(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_from_picture)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_destroy_region_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_destroy_region_checked)(c, region)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_destroy_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_destroy_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_destroy_region_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_destroy_region(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_destroy_region)(c, region)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_destroy_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_destroy_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_destroy_region)
    }

    pub unsafe fn xcb_xfixes_set_region_sizeof(
        &self,
        _buffer: *const c_void,
        rectangles_len: u32,
    ) -> c_int {
        sym!(self, xcb_xfixes_set_region_sizeof)(_buffer, rectangles_len)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_region_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_region_sizeof(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_region_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_set_region_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_set_region_checked)(c, region, rectangles_len, rectangles)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_region_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_set_region(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_set_region)(c, region, rectangles_len, rectangles)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_region)
    }

    pub unsafe fn xcb_xfixes_set_region_rectangles(
        &self,
        r: *const xcb_xfixes_set_region_request_t,
    ) -> *mut xcb_rectangle_t {
        sym!(self, xcb_xfixes_set_region_rectangles)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_region_rectangles` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_region_rectangles(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_region_rectangles)
    }

    pub unsafe fn xcb_xfixes_set_region_rectangles_length(
        &self,
        r: *const xcb_xfixes_set_region_request_t,
    ) -> c_int {
        sym!(self, xcb_xfixes_set_region_rectangles_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_region_rectangles_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_region_rectangles_length(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_region_rectangles_length)
    }

    pub unsafe fn xcb_xfixes_set_region_rectangles_iterator(
        &self,
        r: *const xcb_xfixes_set_region_request_t,
    ) -> xcb_rectangle_iterator_t {
        sym!(self, xcb_xfixes_set_region_rectangles_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_region_rectangles_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_region_rectangles_iterator(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_region_rectangles_iterator)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_copy_region_checked(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_copy_region_checked)(c, source, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_copy_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_copy_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_copy_region_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_copy_region(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_copy_region)(c, source, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_copy_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_copy_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_copy_region)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_union_region_checked(
        &self,
        c: *mut xcb_connection_t,
        source1: xcb_xfixes_region_t,
        source2: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_union_region_checked)(c, source1, source2, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_union_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_union_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_union_region_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_union_region(
        &self,
        c: *mut xcb_connection_t,
        source1: xcb_xfixes_region_t,
        source2: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_union_region)(c, source1, source2, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_union_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_union_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_union_region)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_intersect_region_checked(
        &self,
        c: *mut xcb_connection_t,
        source1: xcb_xfixes_region_t,
        source2: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_intersect_region_checked)(c, source1, source2, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_intersect_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_intersect_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_intersect_region_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_intersect_region(
        &self,
        c: *mut xcb_connection_t,
        source1: xcb_xfixes_region_t,
        source2: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_intersect_region)(c, source1, source2, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_intersect_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_intersect_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_intersect_region)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_subtract_region_checked(
        &self,
        c: *mut xcb_connection_t,
        source1: xcb_xfixes_region_t,
        source2: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_subtract_region_checked)(c, source1, source2, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_subtract_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_subtract_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_subtract_region_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_subtract_region(
        &self,
        c: *mut xcb_connection_t,
        source1: xcb_xfixes_region_t,
        source2: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_subtract_region)(c, source1, source2, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_subtract_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_subtract_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_subtract_region)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_invert_region_checked(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_xfixes_region_t,
        bounds: xcb_rectangle_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_invert_region_checked)(c, source, bounds, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_invert_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_invert_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_invert_region_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_invert_region(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_xfixes_region_t,
        bounds: xcb_rectangle_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_invert_region)(c, source, bounds, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_invert_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_invert_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_invert_region)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_translate_region_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        dx: i16,
        dy: i16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_translate_region_checked)(c, region, dx, dy)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_translate_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_translate_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_translate_region_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_translate_region(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        dx: i16,
        dy: i16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_translate_region)(c, region, dx, dy)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_translate_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_translate_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_translate_region)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_region_extents_checked(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_region_extents_checked)(c, source, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_region_extents_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_region_extents_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_region_extents_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_region_extents(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_region_extents)(c, source, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_region_extents` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_region_extents(&self) -> bool {
        has_sym!(self, xcb_xfixes_region_extents)
    }

    pub unsafe fn xcb_xfixes_fetch_region_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xfixes_fetch_region_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_fetch_region_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_fetch_region_sizeof(&self) -> bool {
        has_sym!(self, xcb_xfixes_fetch_region_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_fetch_region(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
    ) -> xcb_xfixes_fetch_region_cookie_t {
        sym!(self, xcb_xfixes_fetch_region)(c, region)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_fetch_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_fetch_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_fetch_region)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will cause
     * a reply to be generated. Any returned error will be
     * placed in the event queue.
     */
    pub unsafe fn xcb_xfixes_fetch_region_unchecked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
    ) -> xcb_xfixes_fetch_region_cookie_t {
        sym!(self, xcb_xfixes_fetch_region_unchecked)(c, region)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_fetch_region_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_fetch_region_unchecked(&self) -> bool {
        has_sym!(self, xcb_xfixes_fetch_region_unchecked)
    }

    pub unsafe fn xcb_xfixes_fetch_region_rectangles(
        &self,
        r: *const xcb_xfixes_fetch_region_reply_t,
    ) -> *mut xcb_rectangle_t {
        sym!(self, xcb_xfixes_fetch_region_rectangles)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_fetch_region_rectangles` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_fetch_region_rectangles(&self) -> bool {
        has_sym!(self, xcb_xfixes_fetch_region_rectangles)
    }

    pub unsafe fn xcb_xfixes_fetch_region_rectangles_length(
        &self,
        r: *const xcb_xfixes_fetch_region_reply_t,
    ) -> c_int {
        sym!(self, xcb_xfixes_fetch_region_rectangles_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_fetch_region_rectangles_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_fetch_region_rectangles_length(&self) -> bool {
        has_sym!(self, xcb_xfixes_fetch_region_rectangles_length)
    }

    pub unsafe fn xcb_xfixes_fetch_region_rectangles_iterator(
        &self,
        r: *const xcb_xfixes_fetch_region_reply_t,
    ) -> xcb_rectangle_iterator_t {
        sym!(self, xcb_xfixes_fetch_region_rectangles_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_fetch_region_rectangles_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_fetch_region_rectangles_iterator(&self) -> bool {
        has_sym!(self, xcb_xfixes_fetch_region_rectangles_iterator)
    }

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_xfixes_fetch_region_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xfixes_fetch_region_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xfixes_fetch_region_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xfixes_fetch_region_reply_t {
        sym!(self, xcb_xfixes_fetch_region_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_fetch_region_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_fetch_region_reply(&self) -> bool {
        has_sym!(self, xcb_xfixes_fetch_region_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_set_gc_clip_region_checked(
        &self,
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        region: xcb_xfixes_region_t,
        x_origin: i16,
        y_origin: i16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_set_gc_clip_region_checked)(c, gc, region, x_origin, y_origin)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_gc_clip_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_gc_clip_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_gc_clip_region_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_set_gc_clip_region(
        &self,
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        region: xcb_xfixes_region_t,
        x_origin: i16,
        y_origin: i16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_set_gc_clip_region)(c, gc, region, x_origin, y_origin)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_gc_clip_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_gc_clip_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_gc_clip_region)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_set_window_shape_region_checked(
        &self,
        c: *mut xcb_connection_t,
        dest: xcb_window_t,
        dest_kind: xcb_shape_kind_t,
        x_offset: i16,
        y_offset: i16,
        region: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_set_window_shape_region_checked)(
            c, dest, dest_kind, x_offset, y_offset, region,
        )
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_window_shape_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_window_shape_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_window_shape_region_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_set_window_shape_region(
        &self,
        c: *mut xcb_connection_t,
        dest: xcb_window_t,
        dest_kind: xcb_shape_kind_t,
        x_offset: i16,
        y_offset: i16,
        region: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_set_window_shape_region)(
            c, dest, dest_kind, x_offset, y_offset, region,
        )
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_window_shape_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_window_shape_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_window_shape_region)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_set_picture_clip_region_checked(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        region: xcb_xfixes_region_t,
        x_origin: i16,
        y_origin: i16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_set_picture_clip_region_checked)(
            c, picture, region, x_origin, y_origin,
        )
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_picture_clip_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_picture_clip_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_picture_clip_region_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_set_picture_clip_region(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        region: xcb_xfixes_region_t,
        x_origin: i16,
        y_origin: i16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_set_picture_clip_region)(c, picture, region, x_origin, y_origin)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_picture_clip_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_picture_clip_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_picture_clip_region)
    }

    pub unsafe fn xcb_xfixes_set_cursor_name_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xfixes_set_cursor_name_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_cursor_name_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_cursor_name_sizeof(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_cursor_name_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_set_cursor_name_checked(
        &self,
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
        nbytes: u16,
        name: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_set_cursor_name_checked)(c, cursor, nbytes, name)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_cursor_name_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_cursor_name_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_cursor_name_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_set_cursor_name(
        &self,
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
        nbytes: u16,
        name: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_set_cursor_name)(c, cursor, nbytes, name)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_cursor_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_cursor_name(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_cursor_name)
    }

    pub unsafe fn xcb_xfixes_set_cursor_name_name(
        &self,
        r: *const xcb_xfixes_set_cursor_name_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_xfixes_set_cursor_name_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_cursor_name_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_cursor_name_name(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_cursor_name_name)
    }

    pub unsafe fn xcb_xfixes_set_cursor_name_name_length(
        &self,
        r: *const xcb_xfixes_set_cursor_name_request_t,
    ) -> c_int {
        sym!(self, xcb_xfixes_set_cursor_name_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_cursor_name_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_cursor_name_name_length(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_cursor_name_name_length)
    }

    pub unsafe fn xcb_xfixes_set_cursor_name_name_end(
        &self,
        r: *const xcb_xfixes_set_cursor_name_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xfixes_set_cursor_name_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_cursor_name_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_cursor_name_name_end(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_cursor_name_name_end)
    }

    pub unsafe fn xcb_xfixes_get_cursor_name_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xfixes_get_cursor_name_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_name_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_name_sizeof(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_name_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_get_cursor_name(
        &self,
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
    ) -> xcb_xfixes_get_cursor_name_cookie_t {
        sym!(self, xcb_xfixes_get_cursor_name)(c, cursor)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_name(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_name)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will cause
     * a reply to be generated. Any returned error will be
     * placed in the event queue.
     */
    pub unsafe fn xcb_xfixes_get_cursor_name_unchecked(
        &self,
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
    ) -> xcb_xfixes_get_cursor_name_cookie_t {
        sym!(self, xcb_xfixes_get_cursor_name_unchecked)(c, cursor)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_name_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_name_unchecked(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_name_unchecked)
    }

    pub unsafe fn xcb_xfixes_get_cursor_name_name(
        &self,
        r: *const xcb_xfixes_get_cursor_name_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_xfixes_get_cursor_name_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_name_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_name_name(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_name_name)
    }

    pub unsafe fn xcb_xfixes_get_cursor_name_name_length(
        &self,
        r: *const xcb_xfixes_get_cursor_name_reply_t,
    ) -> c_int {
        sym!(self, xcb_xfixes_get_cursor_name_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_name_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_name_name_length(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_name_name_length)
    }

    pub unsafe fn xcb_xfixes_get_cursor_name_name_end(
        &self,
        r: *const xcb_xfixes_get_cursor_name_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xfixes_get_cursor_name_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_name_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_name_name_end(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_name_name_end)
    }

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_xfixes_get_cursor_name_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xfixes_get_cursor_name_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xfixes_get_cursor_name_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xfixes_get_cursor_name_reply_t {
        sym!(self, xcb_xfixes_get_cursor_name_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_name_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_name_reply(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_name_reply)
    }

    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_xfixes_get_cursor_image_and_name_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_and_name_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_and_name_sizeof(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_and_name_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_get_cursor_image_and_name(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xfixes_get_cursor_image_and_name_cookie_t {
        sym!(self, xcb_xfixes_get_cursor_image_and_name)(c)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_and_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_and_name(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_and_name)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will cause
     * a reply to be generated. Any returned error will be
     * placed in the event queue.
     */
    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xfixes_get_cursor_image_and_name_cookie_t {
        sym!(self, xcb_xfixes_get_cursor_image_and_name_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_and_name_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_and_name_unchecked(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_and_name_unchecked)
    }

    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_cursor_image(
        &self,
        r: *const xcb_xfixes_get_cursor_image_and_name_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_xfixes_get_cursor_image_and_name_cursor_image)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_and_name_cursor_image` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_and_name_cursor_image(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_and_name_cursor_image)
    }

    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_cursor_image_length(
        &self,
        r: *const xcb_xfixes_get_cursor_image_and_name_reply_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xfixes_get_cursor_image_and_name_cursor_image_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_and_name_cursor_image_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_and_name_cursor_image_length(&self) -> bool {
        has_sym!(
            self,
            xcb_xfixes_get_cursor_image_and_name_cursor_image_length
        )
    }

    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_cursor_image_end(
        &self,
        r: *const xcb_xfixes_get_cursor_image_and_name_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xfixes_get_cursor_image_and_name_cursor_image_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_and_name_cursor_image_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_and_name_cursor_image_end(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_and_name_cursor_image_end)
    }

    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_name(
        &self,
        r: *const xcb_xfixes_get_cursor_image_and_name_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_xfixes_get_cursor_image_and_name_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_and_name_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_and_name_name(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_and_name_name)
    }

    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_name_length(
        &self,
        r: *const xcb_xfixes_get_cursor_image_and_name_reply_t,
    ) -> c_int {
        sym!(self, xcb_xfixes_get_cursor_image_and_name_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_and_name_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_and_name_name_length(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_and_name_name_length)
    }

    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_name_end(
        &self,
        r: *const xcb_xfixes_get_cursor_image_and_name_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xfixes_get_cursor_image_and_name_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_and_name_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_and_name_name_end(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_and_name_name_end)
    }

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_xfixes_get_cursor_image_and_name_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xfixes_get_cursor_image_and_name_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xfixes_get_cursor_image_and_name_reply_t {
        sym!(self, xcb_xfixes_get_cursor_image_and_name_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_and_name_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_and_name_reply(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_and_name_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_change_cursor_checked(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_cursor_t,
        destination: xcb_cursor_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_change_cursor_checked)(c, source, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_change_cursor_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_change_cursor_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_change_cursor_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_change_cursor(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_cursor_t,
        destination: xcb_cursor_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_change_cursor)(c, source, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_change_cursor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_change_cursor(&self) -> bool {
        has_sym!(self, xcb_xfixes_change_cursor)
    }

    pub unsafe fn xcb_xfixes_change_cursor_by_name_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xfixes_change_cursor_by_name_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_change_cursor_by_name_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_change_cursor_by_name_sizeof(&self) -> bool {
        has_sym!(self, xcb_xfixes_change_cursor_by_name_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_change_cursor_by_name_checked(
        &self,
        c: *mut xcb_connection_t,
        src: xcb_cursor_t,
        nbytes: u16,
        name: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_change_cursor_by_name_checked)(c, src, nbytes, name)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_change_cursor_by_name_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_change_cursor_by_name_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_change_cursor_by_name_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_change_cursor_by_name(
        &self,
        c: *mut xcb_connection_t,
        src: xcb_cursor_t,
        nbytes: u16,
        name: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_change_cursor_by_name)(c, src, nbytes, name)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_change_cursor_by_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_change_cursor_by_name(&self) -> bool {
        has_sym!(self, xcb_xfixes_change_cursor_by_name)
    }

    pub unsafe fn xcb_xfixes_change_cursor_by_name_name(
        &self,
        r: *const xcb_xfixes_change_cursor_by_name_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_xfixes_change_cursor_by_name_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_change_cursor_by_name_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_change_cursor_by_name_name(&self) -> bool {
        has_sym!(self, xcb_xfixes_change_cursor_by_name_name)
    }

    pub unsafe fn xcb_xfixes_change_cursor_by_name_name_length(
        &self,
        r: *const xcb_xfixes_change_cursor_by_name_request_t,
    ) -> c_int {
        sym!(self, xcb_xfixes_change_cursor_by_name_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_change_cursor_by_name_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_change_cursor_by_name_name_length(&self) -> bool {
        has_sym!(self, xcb_xfixes_change_cursor_by_name_name_length)
    }

    pub unsafe fn xcb_xfixes_change_cursor_by_name_name_end(
        &self,
        r: *const xcb_xfixes_change_cursor_by_name_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xfixes_change_cursor_by_name_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_change_cursor_by_name_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_change_cursor_by_name_name_end(&self) -> bool {
        has_sym!(self, xcb_xfixes_change_cursor_by_name_name_end)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_expand_region_checked(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
        left: u16,
        right: u16,
        top: u16,
        bottom: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_expand_region_checked)(
            c,
            source,
            destination,
            left,
            right,
            top,
            bottom,
        )
    }

    /// Returns `true` iff the symbol `xcb_xfixes_expand_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_expand_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_expand_region_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_expand_region(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
        left: u16,
        right: u16,
        top: u16,
        bottom: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_expand_region)(c, source, destination, left, right, top, bottom)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_expand_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_expand_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_expand_region)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_hide_cursor_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_hide_cursor_checked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_hide_cursor_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_hide_cursor_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_hide_cursor_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_hide_cursor(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_hide_cursor)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_hide_cursor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_hide_cursor(&self) -> bool {
        has_sym!(self, xcb_xfixes_hide_cursor)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_show_cursor_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_show_cursor_checked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_show_cursor_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_show_cursor_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_show_cursor_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_show_cursor(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_show_cursor)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_show_cursor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_show_cursor(&self) -> bool {
        has_sym!(self, xcb_xfixes_show_cursor)
    }

    pub unsafe fn xcb_xfixes_barrier_next(&self, i: *mut xcb_xfixes_barrier_iterator_t) {
        sym!(self, xcb_xfixes_barrier_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_barrier_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_barrier_next(&self) -> bool {
        has_sym!(self, xcb_xfixes_barrier_next)
    }

    pub unsafe fn xcb_xfixes_barrier_end(
        &self,
        i: xcb_xfixes_barrier_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xfixes_barrier_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_barrier_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_barrier_end(&self) -> bool {
        has_sym!(self, xcb_xfixes_barrier_end)
    }

    pub unsafe fn xcb_xfixes_create_pointer_barrier_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xfixes_create_pointer_barrier_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_pointer_barrier_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_pointer_barrier_sizeof(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_pointer_barrier_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_create_pointer_barrier_checked(
        &self,
        c: *mut xcb_connection_t,
        barrier: xcb_xfixes_barrier_t,
        window: xcb_window_t,
        x1: u16,
        y1: u16,
        x2: u16,
        y2: u16,
        directions: u32,
        num_devices: u16,
        devices: *const u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_create_pointer_barrier_checked)(
            c,
            barrier,
            window,
            x1,
            y1,
            x2,
            y2,
            directions,
            num_devices,
            devices,
        )
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_pointer_barrier_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_pointer_barrier_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_pointer_barrier_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_create_pointer_barrier(
        &self,
        c: *mut xcb_connection_t,
        barrier: xcb_xfixes_barrier_t,
        window: xcb_window_t,
        x1: u16,
        y1: u16,
        x2: u16,
        y2: u16,
        directions: u32,
        num_devices: u16,
        devices: *const u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_create_pointer_barrier)(
            c,
            barrier,
            window,
            x1,
            y1,
            x2,
            y2,
            directions,
            num_devices,
            devices,
        )
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_pointer_barrier` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_pointer_barrier(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_pointer_barrier)
    }

    pub unsafe fn xcb_xfixes_create_pointer_barrier_devices(
        &self,
        r: *const xcb_xfixes_create_pointer_barrier_request_t,
    ) -> *mut u16 {
        sym!(self, xcb_xfixes_create_pointer_barrier_devices)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_pointer_barrier_devices` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_pointer_barrier_devices(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_pointer_barrier_devices)
    }

    pub unsafe fn xcb_xfixes_create_pointer_barrier_devices_length(
        &self,
        r: *const xcb_xfixes_create_pointer_barrier_request_t,
    ) -> c_int {
        sym!(self, xcb_xfixes_create_pointer_barrier_devices_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_pointer_barrier_devices_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_pointer_barrier_devices_length(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_pointer_barrier_devices_length)
    }

    pub unsafe fn xcb_xfixes_create_pointer_barrier_devices_end(
        &self,
        r: *const xcb_xfixes_create_pointer_barrier_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xfixes_create_pointer_barrier_devices_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_pointer_barrier_devices_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_pointer_barrier_devices_end(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_pointer_barrier_devices_end)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_xfixes_delete_pointer_barrier_checked(
        &self,
        c: *mut xcb_connection_t,
        barrier: xcb_xfixes_barrier_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_delete_pointer_barrier_checked)(c, barrier)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_delete_pointer_barrier_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_delete_pointer_barrier_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_delete_pointer_barrier_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xfixes_delete_pointer_barrier(
        &self,
        c: *mut xcb_connection_t,
        barrier: xcb_xfixes_barrier_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_delete_pointer_barrier)(c, barrier)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_delete_pointer_barrier` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_delete_pointer_barrier(&self) -> bool {
        has_sym!(self, xcb_xfixes_delete_pointer_barrier)
    }
}

#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbXfixes::load().unwrap() };
        assert!(lib.has_xcb_xfixes_id());
        assert!(lib.has_xcb_xfixes_query_version());
        assert!(lib.has_xcb_xfixes_query_version_unchecked());
        assert!(lib.has_xcb_xfixes_query_version_reply());
        assert!(lib.has_xcb_xfixes_change_save_set_checked());
        assert!(lib.has_xcb_xfixes_change_save_set());
        assert!(lib.has_xcb_xfixes_select_selection_input_checked());
        assert!(lib.has_xcb_xfixes_select_selection_input());
        assert!(lib.has_xcb_xfixes_select_cursor_input_checked());
        assert!(lib.has_xcb_xfixes_select_cursor_input());
        assert!(lib.has_xcb_xfixes_get_cursor_image_sizeof());
        assert!(lib.has_xcb_xfixes_get_cursor_image());
        assert!(lib.has_xcb_xfixes_get_cursor_image_unchecked());
        assert!(lib.has_xcb_xfixes_get_cursor_image_cursor_image());
        assert!(lib.has_xcb_xfixes_get_cursor_image_cursor_image_length());
        assert!(lib.has_xcb_xfixes_get_cursor_image_cursor_image_end());
        assert!(lib.has_xcb_xfixes_get_cursor_image_reply());
        assert!(lib.has_xcb_xfixes_region_next());
        assert!(lib.has_xcb_xfixes_region_end());
        assert!(lib.has_xcb_xfixes_create_region_sizeof());
        assert!(lib.has_xcb_xfixes_create_region_checked());
        assert!(lib.has_xcb_xfixes_create_region());
        assert!(lib.has_xcb_xfixes_create_region_rectangles());
        assert!(lib.has_xcb_xfixes_create_region_rectangles_length());
        assert!(lib.has_xcb_xfixes_create_region_rectangles_iterator());
        assert!(lib.has_xcb_xfixes_create_region_from_bitmap_checked());
        assert!(lib.has_xcb_xfixes_create_region_from_bitmap());
        assert!(lib.has_xcb_xfixes_create_region_from_window_checked());
        assert!(lib.has_xcb_xfixes_create_region_from_window());
        assert!(lib.has_xcb_xfixes_create_region_from_gc_checked());
        assert!(lib.has_xcb_xfixes_create_region_from_gc());
        assert!(lib.has_xcb_xfixes_create_region_from_picture_checked());
        assert!(lib.has_xcb_xfixes_create_region_from_picture());
        assert!(lib.has_xcb_xfixes_destroy_region_checked());
        assert!(lib.has_xcb_xfixes_destroy_region());
        assert!(lib.has_xcb_xfixes_set_region_sizeof());
        assert!(lib.has_xcb_xfixes_set_region_checked());
        assert!(lib.has_xcb_xfixes_set_region());
        assert!(lib.has_xcb_xfixes_set_region_rectangles());
        assert!(lib.has_xcb_xfixes_set_region_rectangles_length());
        assert!(lib.has_xcb_xfixes_set_region_rectangles_iterator());
        assert!(lib.has_xcb_xfixes_copy_region_checked());
        assert!(lib.has_xcb_xfixes_copy_region());
        assert!(lib.has_xcb_xfixes_union_region_checked());
        assert!(lib.has_xcb_xfixes_union_region());
        assert!(lib.has_xcb_xfixes_intersect_region_checked());
        assert!(lib.has_xcb_xfixes_intersect_region());
        assert!(lib.has_xcb_xfixes_subtract_region_checked());
        assert!(lib.has_xcb_xfixes_subtract_region());
        assert!(lib.has_xcb_xfixes_invert_region_checked());
        assert!(lib.has_xcb_xfixes_invert_region());
        assert!(lib.has_xcb_xfixes_translate_region_checked());
        assert!(lib.has_xcb_xfixes_translate_region());
        assert!(lib.has_xcb_xfixes_region_extents_checked());
        assert!(lib.has_xcb_xfixes_region_extents());
        assert!(lib.has_xcb_xfixes_fetch_region_sizeof());
        assert!(lib.has_xcb_xfixes_fetch_region());
        assert!(lib.has_xcb_xfixes_fetch_region_unchecked());
        assert!(lib.has_xcb_xfixes_fetch_region_rectangles());
        assert!(lib.has_xcb_xfixes_fetch_region_rectangles_length());
        assert!(lib.has_xcb_xfixes_fetch_region_rectangles_iterator());
        assert!(lib.has_xcb_xfixes_fetch_region_reply());
        assert!(lib.has_xcb_xfixes_set_gc_clip_region_checked());
        assert!(lib.has_xcb_xfixes_set_gc_clip_region());
        assert!(lib.has_xcb_xfixes_set_window_shape_region_checked());
        assert!(lib.has_xcb_xfixes_set_window_shape_region());
        assert!(lib.has_xcb_xfixes_set_picture_clip_region_checked());
        assert!(lib.has_xcb_xfixes_set_picture_clip_region());
        assert!(lib.has_xcb_xfixes_set_cursor_name_sizeof());
        assert!(lib.has_xcb_xfixes_set_cursor_name_checked());
        assert!(lib.has_xcb_xfixes_set_cursor_name());
        assert!(lib.has_xcb_xfixes_set_cursor_name_name());
        assert!(lib.has_xcb_xfixes_set_cursor_name_name_length());
        assert!(lib.has_xcb_xfixes_set_cursor_name_name_end());
        assert!(lib.has_xcb_xfixes_get_cursor_name_sizeof());
        assert!(lib.has_xcb_xfixes_get_cursor_name());
        assert!(lib.has_xcb_xfixes_get_cursor_name_unchecked());
        assert!(lib.has_xcb_xfixes_get_cursor_name_name());
        assert!(lib.has_xcb_xfixes_get_cursor_name_name_length());
        assert!(lib.has_xcb_xfixes_get_cursor_name_name_end());
        assert!(lib.has_xcb_xfixes_get_cursor_name_reply());
        assert!(lib.has_xcb_xfixes_get_cursor_image_and_name_sizeof());
        assert!(lib.has_xcb_xfixes_get_cursor_image_and_name());
        assert!(lib.has_xcb_xfixes_get_cursor_image_and_name_unchecked());
        assert!(lib.has_xcb_xfixes_get_cursor_image_and_name_cursor_image());
        assert!(lib.has_xcb_xfixes_get_cursor_image_and_name_cursor_image_length());
        assert!(lib.has_xcb_xfixes_get_cursor_image_and_name_cursor_image_end());
        assert!(lib.has_xcb_xfixes_get_cursor_image_and_name_name());
        assert!(lib.has_xcb_xfixes_get_cursor_image_and_name_name_length());
        assert!(lib.has_xcb_xfixes_get_cursor_image_and_name_name_end());
        assert!(lib.has_xcb_xfixes_get_cursor_image_and_name_reply());
        assert!(lib.has_xcb_xfixes_change_cursor_checked());
        assert!(lib.has_xcb_xfixes_change_cursor());
        assert!(lib.has_xcb_xfixes_change_cursor_by_name_sizeof());
        assert!(lib.has_xcb_xfixes_change_cursor_by_name_checked());
        assert!(lib.has_xcb_xfixes_change_cursor_by_name());
        assert!(lib.has_xcb_xfixes_change_cursor_by_name_name());
        assert!(lib.has_xcb_xfixes_change_cursor_by_name_name_length());
        assert!(lib.has_xcb_xfixes_change_cursor_by_name_name_end());
        assert!(lib.has_xcb_xfixes_expand_region_checked());
        assert!(lib.has_xcb_xfixes_expand_region());
        assert!(lib.has_xcb_xfixes_hide_cursor_checked());
        assert!(lib.has_xcb_xfixes_hide_cursor());
        assert!(lib.has_xcb_xfixes_show_cursor_checked());
        assert!(lib.has_xcb_xfixes_show_cursor());
        assert!(lib.has_xcb_xfixes_barrier_next());
        assert!(lib.has_xcb_xfixes_barrier_end());
        assert!(lib.has_xcb_xfixes_create_pointer_barrier_sizeof());
        assert!(lib.has_xcb_xfixes_create_pointer_barrier_checked());
        assert!(lib.has_xcb_xfixes_create_pointer_barrier());
        assert!(lib.has_xcb_xfixes_create_pointer_barrier_devices());
        assert!(lib.has_xcb_xfixes_create_pointer_barrier_devices_length());
        assert!(lib.has_xcb_xfixes_create_pointer_barrier_devices_end());
        assert!(lib.has_xcb_xfixes_delete_pointer_barrier_checked());
        assert!(lib.has_xcb_xfixes_delete_pointer_barrier());
    }
}
