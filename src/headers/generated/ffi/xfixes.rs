use crate::*;
use std::os::raw::*;

pub const XCB_XFIXES_MAJOR_VERSION: u32 = 5;
pub const XCB_XFIXES_MINOR_VERSION: u32 = 0;

pub const XCB_XFIXES_QUERY_VERSION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client_major_version: u32,
    pub client_minor_version: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_query_version_cookie_t {
    pub sequence: c_uint,
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

pub type xcb_xfixes_save_set_mode_t = u32;
pub const XCB_XFIXES_SAVE_SET_MODE_INSERT: xcb_xfixes_save_set_mode_t = 0x00;
pub const XCB_XFIXES_SAVE_SET_MODE_DELETE: xcb_xfixes_save_set_mode_t = 0x01;

pub type xcb_xfixes_save_set_target_t = u32;
pub const XCB_XFIXES_SAVE_SET_TARGET_NEAREST: xcb_xfixes_save_set_target_t = 0x00;
pub const XCB_XFIXES_SAVE_SET_TARGET_ROOT: xcb_xfixes_save_set_target_t = 0x01;

pub type xcb_xfixes_save_set_mapping_t = u32;
pub const XCB_XFIXES_SAVE_SET_MAPPING_MAP: xcb_xfixes_save_set_mapping_t = 0x00;
pub const XCB_XFIXES_SAVE_SET_MAPPING_UNMAP: xcb_xfixes_save_set_mapping_t = 0x01;

pub const XCB_XFIXES_CHANGE_SAVE_SET: u8 = 1;

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

pub type xcb_xfixes_selection_event_t = u32;
pub const XCB_XFIXES_SELECTION_EVENT_SET_SELECTION_OWNER: xcb_xfixes_selection_event_t = 0x00;
pub const XCB_XFIXES_SELECTION_EVENT_SELECTION_WINDOW_DESTROY: xcb_xfixes_selection_event_t = 0x01;
pub const XCB_XFIXES_SELECTION_EVENT_SELECTION_CLIENT_CLOSE: xcb_xfixes_selection_event_t = 0x02;

pub type xcb_xfixes_selection_event_mask_t = u32;
pub const XCB_XFIXES_SELECTION_EVENT_MASK_SET_SELECTION_OWNER: xcb_xfixes_selection_event_mask_t =
    0x01;
pub const XCB_XFIXES_SELECTION_EVENT_MASK_SELECTION_WINDOW_DESTROY:
    xcb_xfixes_selection_event_mask_t = 0x02;
pub const XCB_XFIXES_SELECTION_EVENT_MASK_SELECTION_CLIENT_CLOSE:
    xcb_xfixes_selection_event_mask_t = 0x04;

pub const XCB_XFIXES_SELECTION_NOTIFY: u8 = 0;

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

pub const XCB_XFIXES_SELECT_SELECTION_INPUT: u8 = 2;

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

pub type xcb_xfixes_cursor_notify_t = u32;
pub const XCB_XFIXES_CURSOR_NOTIFY_DISPLAY_CURSOR: xcb_xfixes_cursor_notify_t = 0x00;

pub type xcb_xfixes_cursor_notify_mask_t = u32;
pub const XCB_XFIXES_CURSOR_NOTIFY_MASK_DISPLAY_CURSOR: xcb_xfixes_cursor_notify_mask_t = 0x01;

pub const XCB_XFIXES_CURSOR_NOTIFY: u8 = 1;

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

pub const XCB_XFIXES_SELECT_CURSOR_INPUT: u8 = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_select_cursor_input_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub event_mask: u32,
}

pub const XCB_XFIXES_GET_CURSOR_IMAGE: u8 = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_image_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_image_cookie_t {
    pub sequence: c_uint,
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

pub type xcb_xfixes_region_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_region_iterator_t {
    pub data: *mut xcb_xfixes_region_t,
    pub rem: c_int,
    pub index: c_int,
}

pub const XCB_XFIXES_BAD_REGION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_bad_region_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

pub type xcb_xfixes_region_enum_t = u32;
pub const XCB_XFIXES_REGION_NONE: xcb_xfixes_region_enum_t = 0x00;

pub const XCB_XFIXES_CREATE_REGION: u8 = 5;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_create_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
}

pub const XCB_XFIXES_CREATE_REGION_FROM_BITMAP: u8 = 6;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_create_region_from_bitmap_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
    pub bitmap: xcb_pixmap_t,
}

pub const XCB_XFIXES_CREATE_REGION_FROM_WINDOW: u8 = 7;

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

pub const XCB_XFIXES_CREATE_REGION_FROM_GC: u8 = 8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_create_region_from_gc_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
    pub gc: xcb_gcontext_t,
}

pub const XCB_XFIXES_CREATE_REGION_FROM_PICTURE: u8 = 9;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_create_region_from_picture_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
    pub picture: xcb_render_picture_t,
}

pub const XCB_XFIXES_DESTROY_REGION: u8 = 10;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_destroy_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
}

pub const XCB_XFIXES_SET_REGION: u8 = 11;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_set_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
}

pub const XCB_XFIXES_COPY_REGION: u8 = 12;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_copy_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub source: xcb_xfixes_region_t,
    pub destination: xcb_xfixes_region_t,
}

pub const XCB_XFIXES_UNION_REGION: u8 = 13;

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

pub const XCB_XFIXES_INTERSECT_REGION: u8 = 14;

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

pub const XCB_XFIXES_SUBTRACT_REGION: u8 = 15;

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

pub const XCB_XFIXES_INVERT_REGION: u8 = 16;

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

pub const XCB_XFIXES_TRANSLATE_REGION: u8 = 17;

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

pub const XCB_XFIXES_REGION_EXTENTS: u8 = 18;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_region_extents_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub source: xcb_xfixes_region_t,
    pub destination: xcb_xfixes_region_t,
}

pub const XCB_XFIXES_FETCH_REGION: u8 = 19;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_fetch_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_fetch_region_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_XFIXES_SET_GC_CLIP_REGION: u8 = 20;

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

pub const XCB_XFIXES_SET_WINDOW_SHAPE_REGION: u8 = 21;

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

pub const XCB_XFIXES_SET_PICTURE_CLIP_REGION: u8 = 22;

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

pub const XCB_XFIXES_SET_CURSOR_NAME: u8 = 23;

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

pub const XCB_XFIXES_GET_CURSOR_NAME: u8 = 24;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_name_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub cursor: xcb_cursor_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_name_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_XFIXES_GET_CURSOR_IMAGE_AND_NAME: u8 = 25;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_image_and_name_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_image_and_name_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_XFIXES_CHANGE_CURSOR: u8 = 26;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_change_cursor_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub source: xcb_cursor_t,
    pub destination: xcb_cursor_t,
}

pub const XCB_XFIXES_CHANGE_CURSOR_BY_NAME: u8 = 27;

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

pub const XCB_XFIXES_EXPAND_REGION: u8 = 28;

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

pub const XCB_XFIXES_HIDE_CURSOR: u8 = 29;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_hide_cursor_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

pub const XCB_XFIXES_SHOW_CURSOR: u8 = 30;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_show_cursor_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

pub type xcb_xfixes_barrier_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_barrier_iterator_t {
    pub data: *mut xcb_xfixes_barrier_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_xfixes_barrier_directions_t = u32;
pub const XCB_XFIXES_BARRIER_DIRECTIONS_POSITIVE_X: xcb_xfixes_barrier_directions_t = 0x01;
pub const XCB_XFIXES_BARRIER_DIRECTIONS_POSITIVE_Y: xcb_xfixes_barrier_directions_t = 0x02;
pub const XCB_XFIXES_BARRIER_DIRECTIONS_NEGATIVE_X: xcb_xfixes_barrier_directions_t = 0x04;
pub const XCB_XFIXES_BARRIER_DIRECTIONS_NEGATIVE_Y: xcb_xfixes_barrier_directions_t = 0x08;

pub const XCB_XFIXES_CREATE_POINTER_BARRIER: u8 = 31;

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

pub const XCB_XFIXES_DELETE_POINTER_BARRIER: u8 = 32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_delete_pointer_barrier_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub barrier: xcb_xfixes_barrier_t,
}

impl XcbXfixes {
    #[inline]
    pub fn xcb_xfixes_id(&self) -> *mut xcb_extension_t {
        call!(self, xcb_xfixes_id)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xfixes_query_version_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xfixes_query_version_reply_t {
        call!(self, xcb_xfixes_query_version_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_query_version(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u32,
        client_minor_version: u32,
    ) -> xcb_xfixes_query_version_cookie_t {
        call!(self, xcb_xfixes_query_version)(c, client_major_version, client_minor_version)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u32,
        client_minor_version: u32,
    ) -> xcb_xfixes_query_version_cookie_t {
        call!(self, xcb_xfixes_query_version_unchecked)(
            c,
            client_major_version,
            client_minor_version,
        )
    }

    #[inline]
    pub unsafe fn xcb_xfixes_change_save_set(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
        target: u8,
        map: u8,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_change_save_set)(c, mode, target, map, window)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_change_save_set_checked(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
        target: u8,
        map: u8,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_change_save_set_checked)(c, mode, target, map, window)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_select_selection_input(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        selection: xcb_atom_t,
        event_mask: u32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_select_selection_input)(c, window, selection, event_mask)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_select_selection_input_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        selection: xcb_atom_t,
        event_mask: u32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_select_selection_input_checked)(c, window, selection, event_mask)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_select_cursor_input(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        event_mask: u32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_select_cursor_input)(c, window, event_mask)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_select_cursor_input_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        event_mask: u32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_select_cursor_input_checked)(c, window, event_mask)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_cursor_image(
        &self,
        R: *const xcb_xfixes_get_cursor_image_reply_t,
    ) -> *mut u32 {
        call!(self, xcb_xfixes_get_cursor_image_cursor_image)(R)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_cursor_image_length(
        &self,
        R: *const xcb_xfixes_get_cursor_image_reply_t,
    ) -> c_int {
        call!(self, xcb_xfixes_get_cursor_image_cursor_image_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_cursor_image_end(
        &self,
        R: *const xcb_xfixes_get_cursor_image_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_xfixes_get_cursor_image_cursor_image_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xfixes_get_cursor_image_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xfixes_get_cursor_image_reply_t {
        call!(self, xcb_xfixes_get_cursor_image_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xfixes_get_cursor_image_cookie_t {
        call!(self, xcb_xfixes_get_cursor_image)(c)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xfixes_get_cursor_image_cookie_t {
        call!(self, xcb_xfixes_get_cursor_image_unchecked)(c)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_region_next(&self, i: *mut xcb_xfixes_region_iterator_t) {
        call!(self, xcb_xfixes_region_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_xfixes_region_end(
        &self,
        i: *mut xcb_xfixes_region_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_xfixes_region_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_create_region(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_create_region)(c, region, rectangles_len, rectangles)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_create_region_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_create_region_checked)(c, region, rectangles_len, rectangles)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_create_region_from_bitmap(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        bitmap: xcb_pixmap_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_create_region_from_bitmap)(c, region, bitmap)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_create_region_from_bitmap_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        bitmap: xcb_pixmap_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_create_region_from_bitmap_checked)(c, region, bitmap)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_create_region_from_window(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        window: xcb_window_t,
        kind: xcb_shape_kind_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_create_region_from_window)(c, region, window, kind)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_create_region_from_window_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        window: xcb_window_t,
        kind: xcb_shape_kind_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_create_region_from_window_checked)(c, region, window, kind)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_create_region_from_gc(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        gc: xcb_gcontext_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_create_region_from_gc)(c, region, gc)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_create_region_from_gc_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        gc: xcb_gcontext_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_create_region_from_gc_checked)(c, region, gc)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_create_region_from_picture(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        picture: xcb_render_picture_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_create_region_from_picture)(c, region, picture)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_create_region_from_picture_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        picture: xcb_render_picture_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_create_region_from_picture_checked)(c, region, picture)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_destroy_region(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_destroy_region)(c, region)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_destroy_region_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_destroy_region_checked)(c, region)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_set_region(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_set_region)(c, region, rectangles_len, rectangles)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_set_region_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_set_region_checked)(c, region, rectangles_len, rectangles)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_copy_region(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_copy_region)(c, source, destination)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_copy_region_checked(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_copy_region_checked)(c, source, destination)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_union_region(
        &self,
        c: *mut xcb_connection_t,
        source1: xcb_xfixes_region_t,
        source2: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_union_region)(c, source1, source2, destination)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_union_region_checked(
        &self,
        c: *mut xcb_connection_t,
        source1: xcb_xfixes_region_t,
        source2: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_union_region_checked)(c, source1, source2, destination)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_intersect_region(
        &self,
        c: *mut xcb_connection_t,
        source1: xcb_xfixes_region_t,
        source2: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_intersect_region)(c, source1, source2, destination)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_intersect_region_checked(
        &self,
        c: *mut xcb_connection_t,
        source1: xcb_xfixes_region_t,
        source2: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_intersect_region_checked)(c, source1, source2, destination)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_subtract_region(
        &self,
        c: *mut xcb_connection_t,
        source1: xcb_xfixes_region_t,
        source2: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_subtract_region)(c, source1, source2, destination)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_subtract_region_checked(
        &self,
        c: *mut xcb_connection_t,
        source1: xcb_xfixes_region_t,
        source2: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_subtract_region_checked)(c, source1, source2, destination)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_invert_region(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_xfixes_region_t,
        bounds: xcb_rectangle_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_invert_region)(c, source, bounds, destination)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_invert_region_checked(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_xfixes_region_t,
        bounds: xcb_rectangle_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_invert_region_checked)(c, source, bounds, destination)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_translate_region(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        dx: i16,
        dy: i16,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_translate_region)(c, region, dx, dy)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_translate_region_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        dx: i16,
        dy: i16,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_translate_region_checked)(c, region, dx, dy)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_region_extents(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_region_extents)(c, source, destination)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_region_extents_checked(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_region_extents_checked)(c, source, destination)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_fetch_region_rectangles(
        &self,
        R: *const xcb_xfixes_fetch_region_reply_t,
    ) -> *mut xcb_rectangle_t {
        call!(self, xcb_xfixes_fetch_region_rectangles)(R)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_fetch_region_rectangles_length(
        &self,
        R: *const xcb_xfixes_fetch_region_reply_t,
    ) -> c_int {
        call!(self, xcb_xfixes_fetch_region_rectangles_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_fetch_region_rectangles_iterator(
        &self,
        R: *const xcb_xfixes_fetch_region_reply_t,
    ) -> xcb_rectangle_iterator_t {
        call!(self, xcb_xfixes_fetch_region_rectangles_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_fetch_region_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xfixes_fetch_region_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xfixes_fetch_region_reply_t {
        call!(self, xcb_xfixes_fetch_region_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_fetch_region(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
    ) -> xcb_xfixes_fetch_region_cookie_t {
        call!(self, xcb_xfixes_fetch_region)(c, region)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_fetch_region_unchecked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
    ) -> xcb_xfixes_fetch_region_cookie_t {
        call!(self, xcb_xfixes_fetch_region_unchecked)(c, region)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_set_gc_clip_region(
        &self,
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        region: xcb_xfixes_region_t,
        x_origin: i16,
        y_origin: i16,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_set_gc_clip_region)(c, gc, region, x_origin, y_origin)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_set_gc_clip_region_checked(
        &self,
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        region: xcb_xfixes_region_t,
        x_origin: i16,
        y_origin: i16,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_set_gc_clip_region_checked)(c, gc, region, x_origin, y_origin)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_set_window_shape_region(
        &self,
        c: *mut xcb_connection_t,
        dest: xcb_window_t,
        dest_kind: xcb_shape_kind_t,
        x_offset: i16,
        y_offset: i16,
        region: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_set_window_shape_region)(
            c, dest, dest_kind, x_offset, y_offset, region,
        )
    }

    #[inline]
    pub unsafe fn xcb_xfixes_set_window_shape_region_checked(
        &self,
        c: *mut xcb_connection_t,
        dest: xcb_window_t,
        dest_kind: xcb_shape_kind_t,
        x_offset: i16,
        y_offset: i16,
        region: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_set_window_shape_region_checked)(
            c, dest, dest_kind, x_offset, y_offset, region,
        )
    }

    #[inline]
    pub unsafe fn xcb_xfixes_set_picture_clip_region(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        region: xcb_xfixes_region_t,
        x_origin: i16,
        y_origin: i16,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_set_picture_clip_region)(c, picture, region, x_origin, y_origin)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_set_picture_clip_region_checked(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        region: xcb_xfixes_region_t,
        x_origin: i16,
        y_origin: i16,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_set_picture_clip_region_checked)(
            c, picture, region, x_origin, y_origin,
        )
    }

    #[inline]
    pub unsafe fn xcb_xfixes_set_cursor_name(
        &self,
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
        nbytes: u16,
        name: *const c_char,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_set_cursor_name)(c, cursor, nbytes, name)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_set_cursor_name_checked(
        &self,
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
        nbytes: u16,
        name: *const c_char,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_set_cursor_name_checked)(c, cursor, nbytes, name)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_name_name(
        &self,
        R: *const xcb_xfixes_get_cursor_name_reply_t,
    ) -> *mut c_char {
        call!(self, xcb_xfixes_get_cursor_name_name)(R)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_name_name_length(
        &self,
        R: *const xcb_xfixes_get_cursor_name_reply_t,
    ) -> c_int {
        call!(self, xcb_xfixes_get_cursor_name_name_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_name_name_end(
        &self,
        R: *const xcb_xfixes_get_cursor_name_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_xfixes_get_cursor_name_name_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_name_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xfixes_get_cursor_name_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xfixes_get_cursor_name_reply_t {
        call!(self, xcb_xfixes_get_cursor_name_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_name(
        &self,
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
    ) -> xcb_xfixes_get_cursor_name_cookie_t {
        call!(self, xcb_xfixes_get_cursor_name)(c, cursor)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_name_unchecked(
        &self,
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
    ) -> xcb_xfixes_get_cursor_name_cookie_t {
        call!(self, xcb_xfixes_get_cursor_name_unchecked)(c, cursor)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_name(
        &self,
        R: *const xcb_xfixes_get_cursor_image_and_name_reply_t,
    ) -> *mut c_char {
        call!(self, xcb_xfixes_get_cursor_image_and_name_name)(R)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_name_length(
        &self,
        R: *const xcb_xfixes_get_cursor_image_and_name_reply_t,
    ) -> c_int {
        call!(self, xcb_xfixes_get_cursor_image_and_name_name_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_name_end(
        &self,
        R: *const xcb_xfixes_get_cursor_image_and_name_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_xfixes_get_cursor_image_and_name_name_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_cursor_image(
        &self,
        R: *const xcb_xfixes_get_cursor_image_and_name_reply_t,
    ) -> *mut u32 {
        call!(self, xcb_xfixes_get_cursor_image_and_name_cursor_image)(R)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_cursor_image_length(
        &self,
        R: *const xcb_xfixes_get_cursor_image_and_name_reply_t,
    ) -> c_int {
        call!(
            self,
            xcb_xfixes_get_cursor_image_and_name_cursor_image_length
        )(R)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_cursor_image_end(
        &self,
        R: *const xcb_xfixes_get_cursor_image_and_name_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_xfixes_get_cursor_image_and_name_cursor_image_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xfixes_get_cursor_image_and_name_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xfixes_get_cursor_image_and_name_reply_t {
        call!(self, xcb_xfixes_get_cursor_image_and_name_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_and_name(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xfixes_get_cursor_image_and_name_cookie_t {
        call!(self, xcb_xfixes_get_cursor_image_and_name)(c)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xfixes_get_cursor_image_and_name_cookie_t {
        call!(self, xcb_xfixes_get_cursor_image_and_name_unchecked)(c)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_change_cursor(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_cursor_t,
        destination: xcb_cursor_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_change_cursor)(c, source, destination)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_change_cursor_checked(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_cursor_t,
        destination: xcb_cursor_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_change_cursor_checked)(c, source, destination)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_change_cursor_by_name(
        &self,
        c: *mut xcb_connection_t,
        src: xcb_cursor_t,
        nbytes: u16,
        name: *const c_char,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_change_cursor_by_name)(c, src, nbytes, name)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_change_cursor_by_name_checked(
        &self,
        c: *mut xcb_connection_t,
        src: xcb_cursor_t,
        nbytes: u16,
        name: *const c_char,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_change_cursor_by_name_checked)(c, src, nbytes, name)
    }

    #[inline]
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
        call!(self, xcb_xfixes_expand_region)(c, source, destination, left, right, top, bottom)
    }

    #[inline]
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
        call!(self, xcb_xfixes_expand_region_checked)(
            c,
            source,
            destination,
            left,
            right,
            top,
            bottom,
        )
    }

    #[inline]
    pub unsafe fn xcb_xfixes_hide_cursor(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_hide_cursor)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_hide_cursor_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_hide_cursor_checked)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_show_cursor(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_show_cursor)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_show_cursor_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_show_cursor_checked)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_barrier_next(&self, i: *mut xcb_xfixes_barrier_iterator_t) {
        call!(self, xcb_xfixes_barrier_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_xfixes_barrier_end(
        &self,
        i: *mut xcb_xfixes_barrier_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_xfixes_barrier_end)(i)
    }

    #[inline]
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
        call!(self, xcb_xfixes_create_pointer_barrier)(
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

    #[inline]
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
        call!(self, xcb_xfixes_create_pointer_barrier_checked)(
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

    #[inline]
    pub unsafe fn xcb_xfixes_delete_pointer_barrier(
        &self,
        c: *mut xcb_connection_t,
        barrier: xcb_xfixes_barrier_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_delete_pointer_barrier)(c, barrier)
    }

    #[inline]
    pub unsafe fn xcb_xfixes_delete_pointer_barrier_checked(
        &self,
        c: *mut xcb_connection_t,
        barrier: xcb_xfixes_barrier_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xfixes_delete_pointer_barrier_checked)(c, barrier)
    }
}

pub struct XcbXfixes {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_xfixes_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_xfixes_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xfixes_query_version_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xfixes_query_version_reply_t,
    >,
    pub(crate) xcb_xfixes_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u32,
            client_minor_version: u32,
        ) -> xcb_xfixes_query_version_cookie_t,
    >,
    pub(crate) xcb_xfixes_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u32,
            client_minor_version: u32,
        ) -> xcb_xfixes_query_version_cookie_t,
    >,
    pub(crate) xcb_xfixes_change_save_set: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            mode: u8,
            target: u8,
            map: u8,
            window: xcb_window_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_change_save_set_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            mode: u8,
            target: u8,
            map: u8,
            window: xcb_window_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_select_selection_input: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            selection: xcb_atom_t,
            event_mask: u32,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_select_selection_input_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            selection: xcb_atom_t,
            event_mask: u32,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_select_cursor_input: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            event_mask: u32,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_select_cursor_input_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            event_mask: u32,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_get_cursor_image_cursor_image:
        LazySymbol<unsafe fn(R: *const xcb_xfixes_get_cursor_image_reply_t) -> *mut u32>,
    pub(crate) xcb_xfixes_get_cursor_image_cursor_image_length:
        LazySymbol<unsafe fn(R: *const xcb_xfixes_get_cursor_image_reply_t) -> c_int>,
    pub(crate) xcb_xfixes_get_cursor_image_cursor_image_end: LazySymbol<
        unsafe fn(R: *const xcb_xfixes_get_cursor_image_reply_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_xfixes_get_cursor_image_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xfixes_get_cursor_image_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xfixes_get_cursor_image_reply_t,
    >,
    pub(crate) xcb_xfixes_get_cursor_image:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xfixes_get_cursor_image_cookie_t>,
    pub(crate) xcb_xfixes_get_cursor_image_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xfixes_get_cursor_image_cookie_t>,
    pub(crate) xcb_xfixes_region_next: LazySymbol<unsafe fn(i: *mut xcb_xfixes_region_iterator_t)>,
    pub(crate) xcb_xfixes_region_end:
        LazySymbol<unsafe fn(i: *mut xcb_xfixes_region_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_xfixes_create_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_create_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_create_region_from_bitmap: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            bitmap: xcb_pixmap_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_create_region_from_bitmap_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            bitmap: xcb_pixmap_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_create_region_from_window: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            window: xcb_window_t,
            kind: xcb_shape_kind_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_create_region_from_window_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            window: xcb_window_t,
            kind: xcb_shape_kind_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_create_region_from_gc: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            gc: xcb_gcontext_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_create_region_from_gc_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            gc: xcb_gcontext_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_create_region_from_picture: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            picture: xcb_render_picture_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_create_region_from_picture_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            picture: xcb_render_picture_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_destroy_region: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, region: xcb_xfixes_region_t) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_destroy_region_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, region: xcb_xfixes_region_t) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_set_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_set_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_copy_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_copy_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_union_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source1: xcb_xfixes_region_t,
            source2: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_union_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source1: xcb_xfixes_region_t,
            source2: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_intersect_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source1: xcb_xfixes_region_t,
            source2: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_intersect_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source1: xcb_xfixes_region_t,
            source2: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_subtract_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source1: xcb_xfixes_region_t,
            source2: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_subtract_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source1: xcb_xfixes_region_t,
            source2: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_invert_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_xfixes_region_t,
            bounds: xcb_rectangle_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_invert_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_xfixes_region_t,
            bounds: xcb_rectangle_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_translate_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            dx: i16,
            dy: i16,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_translate_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            dx: i16,
            dy: i16,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_region_extents: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_region_extents_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_fetch_region_rectangles:
        LazySymbol<unsafe fn(R: *const xcb_xfixes_fetch_region_reply_t) -> *mut xcb_rectangle_t>,
    pub(crate) xcb_xfixes_fetch_region_rectangles_length:
        LazySymbol<unsafe fn(R: *const xcb_xfixes_fetch_region_reply_t) -> c_int>,
    pub(crate) xcb_xfixes_fetch_region_rectangles_iterator: LazySymbol<
        unsafe fn(R: *const xcb_xfixes_fetch_region_reply_t) -> xcb_rectangle_iterator_t,
    >,
    pub(crate) xcb_xfixes_fetch_region_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xfixes_fetch_region_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xfixes_fetch_region_reply_t,
    >,
    pub(crate) xcb_xfixes_fetch_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
        ) -> xcb_xfixes_fetch_region_cookie_t,
    >,
    pub(crate) xcb_xfixes_fetch_region_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
        ) -> xcb_xfixes_fetch_region_cookie_t,
    >,
    pub(crate) xcb_xfixes_set_gc_clip_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gc: xcb_gcontext_t,
            region: xcb_xfixes_region_t,
            x_origin: i16,
            y_origin: i16,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_set_gc_clip_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gc: xcb_gcontext_t,
            region: xcb_xfixes_region_t,
            x_origin: i16,
            y_origin: i16,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_set_window_shape_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            dest: xcb_window_t,
            dest_kind: xcb_shape_kind_t,
            x_offset: i16,
            y_offset: i16,
            region: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_set_window_shape_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            dest: xcb_window_t,
            dest_kind: xcb_shape_kind_t,
            x_offset: i16,
            y_offset: i16,
            region: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_set_picture_clip_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            region: xcb_xfixes_region_t,
            x_origin: i16,
            y_origin: i16,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_set_picture_clip_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            region: xcb_xfixes_region_t,
            x_origin: i16,
            y_origin: i16,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_set_cursor_name: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cursor: xcb_cursor_t,
            nbytes: u16,
            name: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_set_cursor_name_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cursor: xcb_cursor_t,
            nbytes: u16,
            name: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_get_cursor_name_name:
        LazySymbol<unsafe fn(R: *const xcb_xfixes_get_cursor_name_reply_t) -> *mut c_char>,
    pub(crate) xcb_xfixes_get_cursor_name_name_length:
        LazySymbol<unsafe fn(R: *const xcb_xfixes_get_cursor_name_reply_t) -> c_int>,
    pub(crate) xcb_xfixes_get_cursor_name_name_end: LazySymbol<
        unsafe fn(R: *const xcb_xfixes_get_cursor_name_reply_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_xfixes_get_cursor_name_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xfixes_get_cursor_name_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xfixes_get_cursor_name_reply_t,
    >,
    pub(crate) xcb_xfixes_get_cursor_name: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cursor: xcb_cursor_t,
        ) -> xcb_xfixes_get_cursor_name_cookie_t,
    >,
    pub(crate) xcb_xfixes_get_cursor_name_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cursor: xcb_cursor_t,
        ) -> xcb_xfixes_get_cursor_name_cookie_t,
    >,
    pub(crate) xcb_xfixes_get_cursor_image_and_name_name: LazySymbol<
        unsafe fn(R: *const xcb_xfixes_get_cursor_image_and_name_reply_t) -> *mut c_char,
    >,
    pub(crate) xcb_xfixes_get_cursor_image_and_name_name_length:
        LazySymbol<unsafe fn(R: *const xcb_xfixes_get_cursor_image_and_name_reply_t) -> c_int>,
    pub(crate) xcb_xfixes_get_cursor_image_and_name_name_end: LazySymbol<
        unsafe fn(R: *const xcb_xfixes_get_cursor_image_and_name_reply_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_xfixes_get_cursor_image_and_name_cursor_image:
        LazySymbol<unsafe fn(R: *const xcb_xfixes_get_cursor_image_and_name_reply_t) -> *mut u32>,
    pub(crate) xcb_xfixes_get_cursor_image_and_name_cursor_image_length:
        LazySymbol<unsafe fn(R: *const xcb_xfixes_get_cursor_image_and_name_reply_t) -> c_int>,
    pub(crate) xcb_xfixes_get_cursor_image_and_name_cursor_image_end: LazySymbol<
        unsafe fn(R: *const xcb_xfixes_get_cursor_image_and_name_reply_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_xfixes_get_cursor_image_and_name_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xfixes_get_cursor_image_and_name_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xfixes_get_cursor_image_and_name_reply_t,
    >,
    pub(crate) xcb_xfixes_get_cursor_image_and_name: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_xfixes_get_cursor_image_and_name_cookie_t,
    >,
    pub(crate) xcb_xfixes_get_cursor_image_and_name_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_xfixes_get_cursor_image_and_name_cookie_t,
    >,
    pub(crate) xcb_xfixes_change_cursor: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_cursor_t,
            destination: xcb_cursor_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_change_cursor_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_cursor_t,
            destination: xcb_cursor_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_change_cursor_by_name: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src: xcb_cursor_t,
            nbytes: u16,
            name: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_change_cursor_by_name_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src: xcb_cursor_t,
            nbytes: u16,
            name: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_expand_region: LazySymbol<
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
    pub(crate) xcb_xfixes_expand_region_checked: LazySymbol<
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
    pub(crate) xcb_xfixes_hide_cursor:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_xfixes_hide_cursor_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_xfixes_show_cursor:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_xfixes_show_cursor_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_xfixes_barrier_next:
        LazySymbol<unsafe fn(i: *mut xcb_xfixes_barrier_iterator_t)>,
    pub(crate) xcb_xfixes_barrier_end:
        LazySymbol<unsafe fn(i: *mut xcb_xfixes_barrier_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_xfixes_create_pointer_barrier: LazySymbol<
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
    pub(crate) xcb_xfixes_create_pointer_barrier_checked: LazySymbol<
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
    pub(crate) xcb_xfixes_delete_pointer_barrier: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, barrier: xcb_xfixes_barrier_t) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xfixes_delete_pointer_barrier_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, barrier: xcb_xfixes_barrier_t) -> xcb_void_cookie_t,
    >,
}
