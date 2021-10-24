use crate::ffi::*;
use crate::*;
use std::os::raw::*;

pub const XCB_SHAPE_MAJOR_VERSION: u32 = 1;
pub const XCB_SHAPE_MINOR_VERSION: u32 = 1;

pub type xcb_shape_op_t = u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_op_iterator_t {
    pub data: *mut xcb_shape_op_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_shape_kind_t = u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_kind_iterator_t {
    pub data: *mut xcb_shape_kind_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_shape_so_t = u32;
pub const XCB_SHAPE_SO_SET: xcb_shape_so_t = 0x00;
pub const XCB_SHAPE_SO_UNION: xcb_shape_so_t = 0x01;
pub const XCB_SHAPE_SO_INTERSECT: xcb_shape_so_t = 0x02;
pub const XCB_SHAPE_SO_SUBTRACT: xcb_shape_so_t = 0x03;
pub const XCB_SHAPE_SO_INVERT: xcb_shape_so_t = 0x04;

pub type xcb_shape_sk_t = u32;
pub const XCB_SHAPE_SK_BOUNDING: xcb_shape_sk_t = 0x00;
pub const XCB_SHAPE_SK_CLIP: xcb_shape_sk_t = 0x01;
pub const XCB_SHAPE_SK_INPUT: xcb_shape_sk_t = 0x02;

pub const XCB_SHAPE_NOTIFY: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_notify_event_t {
    pub response_type: u8,
    pub shape_kind: xcb_shape_kind_t,
    pub sequence: u16,
    pub affected_window: xcb_window_t,
    pub extents_x: i16,
    pub extents_y: i16,
    pub extents_width: u16,
    pub extents_height: u16,
    pub server_time: xcb_timestamp_t,
    pub shaped: u8,
    pub pad0: [u8; 11],
}

pub const XCB_SHAPE_QUERY_VERSION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_query_version_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u16,
    pub minor_version: u16,
}

pub const XCB_SHAPE_RECTANGLES: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_rectangles_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub operation: xcb_shape_op_t,
    pub destination_kind: xcb_shape_kind_t,
    pub ordering: u8,
    pub pad0: u8,
    pub destination_window: xcb_window_t,
    pub x_offset: i16,
    pub y_offset: i16,
}

pub const XCB_SHAPE_MASK: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_mask_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub operation: xcb_shape_op_t,
    pub destination_kind: xcb_shape_kind_t,
    pub pad0: [u8; 2],
    pub destination_window: xcb_window_t,
    pub x_offset: i16,
    pub y_offset: i16,
    pub source_bitmap: xcb_pixmap_t,
}

pub const XCB_SHAPE_COMBINE: u8 = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_combine_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub operation: xcb_shape_op_t,
    pub destination_kind: xcb_shape_kind_t,
    pub source_kind: xcb_shape_kind_t,
    pub pad0: u8,
    pub destination_window: xcb_window_t,
    pub x_offset: i16,
    pub y_offset: i16,
    pub source_window: xcb_window_t,
}

pub const XCB_SHAPE_OFFSET: u8 = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_offset_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub destination_kind: xcb_shape_kind_t,
    pub pad0: [u8; 3],
    pub destination_window: xcb_window_t,
    pub x_offset: i16,
    pub y_offset: i16,
}

pub const XCB_SHAPE_QUERY_EXTENTS: u8 = 5;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_query_extents_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub destination_window: xcb_window_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_query_extents_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_query_extents_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub bounding_shaped: u8,
    pub clip_shaped: u8,
    pub pad1: [u8; 2],
    pub bounding_shape_extents_x: i16,
    pub bounding_shape_extents_y: i16,
    pub bounding_shape_extents_width: u16,
    pub bounding_shape_extents_height: u16,
    pub clip_shape_extents_x: i16,
    pub clip_shape_extents_y: i16,
    pub clip_shape_extents_width: u16,
    pub clip_shape_extents_height: u16,
}

pub const XCB_SHAPE_SELECT_INPUT: u8 = 6;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_select_input_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub destination_window: xcb_window_t,
    pub enable: u8,
    pub pad0: [u8; 3],
}

pub const XCB_SHAPE_INPUT_SELECTED: u8 = 7;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_input_selected_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub destination_window: xcb_window_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_input_selected_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_input_selected_reply_t {
    pub response_type: u8,
    pub enabled: u8,
    pub sequence: u16,
    pub length: u32,
}

pub const XCB_SHAPE_GET_RECTANGLES: u8 = 8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_get_rectangles_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub source_kind: xcb_shape_kind_t,
    pub pad0: [u8; 3],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_get_rectangles_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_get_rectangles_reply_t {
    pub response_type: u8,
    pub ordering: u8,
    pub sequence: u16,
    pub length: u32,
    pub rectangles_len: u32,
    pub pad0: [u8; 20],
}

impl XcbShape {
    #[inline]
    pub unsafe fn xcb_shape_id(&self) -> *mut xcb_extension_t {
        sym!(self, xcb_shape_id)
    }

    #[inline]
    pub unsafe fn xcb_shape_op_next(&self, i: *mut xcb_shape_op_iterator_t) {
        sym!(self, xcb_shape_op_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_shape_op_end(
        &self,
        i: *mut xcb_shape_op_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_shape_op_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_shape_kind_next(&self, i: *mut xcb_shape_kind_iterator_t) {
        sym!(self, xcb_shape_kind_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_shape_kind_end(
        &self,
        i: *mut xcb_shape_kind_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_shape_kind_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_shape_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_shape_query_version_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_shape_query_version_reply_t {
        sym!(self, xcb_shape_query_version_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_shape_query_version(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_shape_query_version_cookie_t {
        sym!(self, xcb_shape_query_version)(c)
    }

    #[inline]
    pub unsafe fn xcb_shape_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_shape_query_version_cookie_t {
        sym!(self, xcb_shape_query_version_unchecked)(c)
    }

    #[inline]
    pub unsafe fn xcb_shape_rectangles(
        &self,
        c: *mut xcb_connection_t,
        operation: xcb_shape_op_t,
        destination_kind: xcb_shape_kind_t,
        ordering: u8,
        destination_window: xcb_window_t,
        x_offset: i16,
        y_offset: i16,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_shape_rectangles)(
            c,
            operation,
            destination_kind,
            ordering,
            destination_window,
            x_offset,
            y_offset,
            rectangles_len,
            rectangles,
        )
    }

    #[inline]
    pub unsafe fn xcb_shape_rectangles_checked(
        &self,
        c: *mut xcb_connection_t,
        operation: xcb_shape_op_t,
        destination_kind: xcb_shape_kind_t,
        ordering: u8,
        destination_window: xcb_window_t,
        x_offset: i16,
        y_offset: i16,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_shape_rectangles_checked)(
            c,
            operation,
            destination_kind,
            ordering,
            destination_window,
            x_offset,
            y_offset,
            rectangles_len,
            rectangles,
        )
    }

    #[inline]
    pub unsafe fn xcb_shape_mask(
        &self,
        c: *mut xcb_connection_t,
        operation: xcb_shape_op_t,
        destination_kind: xcb_shape_kind_t,
        destination_window: xcb_window_t,
        x_offset: i16,
        y_offset: i16,
        source_bitmap: xcb_pixmap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_shape_mask)(
            c,
            operation,
            destination_kind,
            destination_window,
            x_offset,
            y_offset,
            source_bitmap,
        )
    }

    #[inline]
    pub unsafe fn xcb_shape_mask_checked(
        &self,
        c: *mut xcb_connection_t,
        operation: xcb_shape_op_t,
        destination_kind: xcb_shape_kind_t,
        destination_window: xcb_window_t,
        x_offset: i16,
        y_offset: i16,
        source_bitmap: xcb_pixmap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_shape_mask_checked)(
            c,
            operation,
            destination_kind,
            destination_window,
            x_offset,
            y_offset,
            source_bitmap,
        )
    }

    #[inline]
    pub unsafe fn xcb_shape_combine(
        &self,
        c: *mut xcb_connection_t,
        operation: xcb_shape_op_t,
        destination_kind: xcb_shape_kind_t,
        source_kind: xcb_shape_kind_t,
        destination_window: xcb_window_t,
        x_offset: i16,
        y_offset: i16,
        source_window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_shape_combine)(
            c,
            operation,
            destination_kind,
            source_kind,
            destination_window,
            x_offset,
            y_offset,
            source_window,
        )
    }

    #[inline]
    pub unsafe fn xcb_shape_combine_checked(
        &self,
        c: *mut xcb_connection_t,
        operation: xcb_shape_op_t,
        destination_kind: xcb_shape_kind_t,
        source_kind: xcb_shape_kind_t,
        destination_window: xcb_window_t,
        x_offset: i16,
        y_offset: i16,
        source_window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_shape_combine_checked)(
            c,
            operation,
            destination_kind,
            source_kind,
            destination_window,
            x_offset,
            y_offset,
            source_window,
        )
    }

    #[inline]
    pub unsafe fn xcb_shape_offset(
        &self,
        c: *mut xcb_connection_t,
        destination_kind: xcb_shape_kind_t,
        destination_window: xcb_window_t,
        x_offset: i16,
        y_offset: i16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_shape_offset)(c, destination_kind, destination_window, x_offset, y_offset)
    }

    #[inline]
    pub unsafe fn xcb_shape_offset_checked(
        &self,
        c: *mut xcb_connection_t,
        destination_kind: xcb_shape_kind_t,
        destination_window: xcb_window_t,
        x_offset: i16,
        y_offset: i16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_shape_offset_checked)(
            c,
            destination_kind,
            destination_window,
            x_offset,
            y_offset,
        )
    }

    #[inline]
    pub unsafe fn xcb_shape_query_extents_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_shape_query_extents_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_shape_query_extents_reply_t {
        sym!(self, xcb_shape_query_extents_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_shape_query_extents(
        &self,
        c: *mut xcb_connection_t,
        destination_window: xcb_window_t,
    ) -> xcb_shape_query_extents_cookie_t {
        sym!(self, xcb_shape_query_extents)(c, destination_window)
    }

    #[inline]
    pub unsafe fn xcb_shape_query_extents_unchecked(
        &self,
        c: *mut xcb_connection_t,
        destination_window: xcb_window_t,
    ) -> xcb_shape_query_extents_cookie_t {
        sym!(self, xcb_shape_query_extents_unchecked)(c, destination_window)
    }

    #[inline]
    pub unsafe fn xcb_shape_select_input(
        &self,
        c: *mut xcb_connection_t,
        destination_window: xcb_window_t,
        enable: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_shape_select_input)(c, destination_window, enable)
    }

    #[inline]
    pub unsafe fn xcb_shape_select_input_checked(
        &self,
        c: *mut xcb_connection_t,
        destination_window: xcb_window_t,
        enable: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_shape_select_input_checked)(c, destination_window, enable)
    }

    #[inline]
    pub unsafe fn xcb_shape_input_selected_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_shape_input_selected_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_shape_input_selected_reply_t {
        sym!(self, xcb_shape_input_selected_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_shape_input_selected(
        &self,
        c: *mut xcb_connection_t,
        destination_window: xcb_window_t,
    ) -> xcb_shape_input_selected_cookie_t {
        sym!(self, xcb_shape_input_selected)(c, destination_window)
    }

    #[inline]
    pub unsafe fn xcb_shape_input_selected_unchecked(
        &self,
        c: *mut xcb_connection_t,
        destination_window: xcb_window_t,
    ) -> xcb_shape_input_selected_cookie_t {
        sym!(self, xcb_shape_input_selected_unchecked)(c, destination_window)
    }

    #[inline]
    pub unsafe fn xcb_shape_get_rectangles_rectangles(
        &self,
        R: *const xcb_shape_get_rectangles_reply_t,
    ) -> *mut xcb_rectangle_t {
        sym!(self, xcb_shape_get_rectangles_rectangles)(R)
    }

    #[inline]
    pub unsafe fn xcb_shape_get_rectangles_rectangles_length(
        &self,
        R: *const xcb_shape_get_rectangles_reply_t,
    ) -> c_int {
        sym!(self, xcb_shape_get_rectangles_rectangles_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_shape_get_rectangles_rectangles_iterator(
        &self,
        R: *const xcb_shape_get_rectangles_reply_t,
    ) -> xcb_rectangle_iterator_t {
        sym!(self, xcb_shape_get_rectangles_rectangles_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_shape_get_rectangles_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_shape_get_rectangles_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_shape_get_rectangles_reply_t {
        sym!(self, xcb_shape_get_rectangles_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_shape_get_rectangles(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        source_kind: xcb_shape_kind_t,
    ) -> xcb_shape_get_rectangles_cookie_t {
        sym!(self, xcb_shape_get_rectangles)(c, window, source_kind)
    }

    #[inline]
    pub unsafe fn xcb_shape_get_rectangles_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        source_kind: xcb_shape_kind_t,
    ) -> xcb_shape_get_rectangles_cookie_t {
        sym!(self, xcb_shape_get_rectangles_unchecked)(c, window, source_kind)
    }
}
