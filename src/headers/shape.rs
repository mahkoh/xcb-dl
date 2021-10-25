// This file was generated using generate.py. Do not edit.

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

pub type xcb_shape_op_t = u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_op_iterator_t {
    pub data: *mut xcb_shape_op_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_shape_op_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_shape_kind_t = u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_kind_iterator_t {
    pub data: *mut xcb_shape_kind_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_shape_kind_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_shape_so_t = u32;
pub const XCB_SHAPE_SO_SET: xcb_shape_so_t = 0;
pub const XCB_SHAPE_SO_UNION: xcb_shape_so_t = 1;
pub const XCB_SHAPE_SO_INTERSECT: xcb_shape_so_t = 2;
pub const XCB_SHAPE_SO_SUBTRACT: xcb_shape_so_t = 3;
pub const XCB_SHAPE_SO_INVERT: xcb_shape_so_t = 4;

pub type xcb_shape_sk_t = u32;
pub const XCB_SHAPE_SK_BOUNDING: xcb_shape_sk_t = 0;
pub const XCB_SHAPE_SK_CLIP: xcb_shape_sk_t = 1;
pub const XCB_SHAPE_SK_INPUT: xcb_shape_sk_t = 2;

/// Opcode for xcb_shape_notify.
pub const XCB_SHAPE_NOTIFY: u8 = 0i32 as u8;

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

impl Default for xcb_shape_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_query_version_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_shape_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_shape_query_version.
pub const XCB_SHAPE_QUERY_VERSION: u8 = 0i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_shape_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_shape_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_shape_rectangles.
pub const XCB_SHAPE_RECTANGLES: u8 = 1i32 as u8;

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

impl Default for xcb_shape_rectangles_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_shape_mask.
pub const XCB_SHAPE_MASK: u8 = 2i32 as u8;

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

impl Default for xcb_shape_mask_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_shape_combine.
pub const XCB_SHAPE_COMBINE: u8 = 3i32 as u8;

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

impl Default for xcb_shape_combine_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_shape_offset.
pub const XCB_SHAPE_OFFSET: u8 = 4i32 as u8;

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

impl Default for xcb_shape_offset_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_query_extents_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_shape_query_extents_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_shape_query_extents.
pub const XCB_SHAPE_QUERY_EXTENTS: u8 = 5i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_query_extents_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub destination_window: xcb_window_t,
}

impl Default for xcb_shape_query_extents_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_shape_query_extents_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_shape_select_input.
pub const XCB_SHAPE_SELECT_INPUT: u8 = 6i32 as u8;

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

impl Default for xcb_shape_select_input_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_input_selected_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_shape_input_selected_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_shape_input_selected.
pub const XCB_SHAPE_INPUT_SELECTED: u8 = 7i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_input_selected_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub destination_window: xcb_window_t,
}

impl Default for xcb_shape_input_selected_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_input_selected_reply_t {
    pub response_type: u8,
    pub enabled: u8,
    pub sequence: u16,
    pub length: u32,
}

impl Default for xcb_shape_input_selected_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shape_get_rectangles_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_shape_get_rectangles_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_shape_get_rectangles.
pub const XCB_SHAPE_GET_RECTANGLES: u8 = 8i32 as u8;

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

impl Default for xcb_shape_get_rectangles_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_shape_get_rectangles_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub(crate) struct XcbShapeShape {
    xcb_shape_id: LazySymbol<*mut xcb_extension_t>,
    xcb_shape_op_next: LazySymbol<unsafe fn(i: *mut xcb_shape_op_iterator_t)>,
    xcb_shape_op_end: LazySymbol<unsafe fn(i: xcb_shape_op_iterator_t) -> xcb_generic_iterator_t>,
    xcb_shape_kind_next: LazySymbol<unsafe fn(i: *mut xcb_shape_kind_iterator_t)>,
    xcb_shape_kind_end:
        LazySymbol<unsafe fn(i: xcb_shape_kind_iterator_t) -> xcb_generic_iterator_t>,
    xcb_shape_query_version:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_shape_query_version_cookie_t>,
    xcb_shape_query_version_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_shape_query_version_cookie_t>,
    xcb_shape_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_shape_query_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_shape_query_version_reply_t,
    >,
    xcb_shape_rectangles_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, rectangles_len: u32) -> c_int>,
    xcb_shape_rectangles_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            operation: xcb_shape_op_t,
            destination_kind: xcb_shape_kind_t,
            ordering: u8,
            destination_window: xcb_window_t,
            x_offset: i16,
            y_offset: i16,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_shape_rectangles: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            operation: xcb_shape_op_t,
            destination_kind: xcb_shape_kind_t,
            ordering: u8,
            destination_window: xcb_window_t,
            x_offset: i16,
            y_offset: i16,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_shape_rectangles_rectangles:
        LazySymbol<unsafe fn(r: *const xcb_shape_rectangles_request_t) -> *mut xcb_rectangle_t>,
    xcb_shape_rectangles_rectangles_length:
        LazySymbol<unsafe fn(r: *const xcb_shape_rectangles_request_t) -> c_int>,
    xcb_shape_rectangles_rectangles_iterator:
        LazySymbol<unsafe fn(r: *const xcb_shape_rectangles_request_t) -> xcb_rectangle_iterator_t>,
    xcb_shape_mask_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            operation: xcb_shape_op_t,
            destination_kind: xcb_shape_kind_t,
            destination_window: xcb_window_t,
            x_offset: i16,
            y_offset: i16,
            source_bitmap: xcb_pixmap_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_shape_mask: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            operation: xcb_shape_op_t,
            destination_kind: xcb_shape_kind_t,
            destination_window: xcb_window_t,
            x_offset: i16,
            y_offset: i16,
            source_bitmap: xcb_pixmap_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_shape_combine_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            operation: xcb_shape_op_t,
            destination_kind: xcb_shape_kind_t,
            source_kind: xcb_shape_kind_t,
            destination_window: xcb_window_t,
            x_offset: i16,
            y_offset: i16,
            source_window: xcb_window_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_shape_combine: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            operation: xcb_shape_op_t,
            destination_kind: xcb_shape_kind_t,
            source_kind: xcb_shape_kind_t,
            destination_window: xcb_window_t,
            x_offset: i16,
            y_offset: i16,
            source_window: xcb_window_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_shape_offset_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            destination_kind: xcb_shape_kind_t,
            destination_window: xcb_window_t,
            x_offset: i16,
            y_offset: i16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_shape_offset: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            destination_kind: xcb_shape_kind_t,
            destination_window: xcb_window_t,
            x_offset: i16,
            y_offset: i16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_shape_query_extents: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            destination_window: xcb_window_t,
        ) -> xcb_shape_query_extents_cookie_t,
    >,
    xcb_shape_query_extents_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            destination_window: xcb_window_t,
        ) -> xcb_shape_query_extents_cookie_t,
    >,
    xcb_shape_query_extents_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_shape_query_extents_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_shape_query_extents_reply_t,
    >,
    xcb_shape_select_input_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            destination_window: xcb_window_t,
            enable: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_shape_select_input: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            destination_window: xcb_window_t,
            enable: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_shape_input_selected: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            destination_window: xcb_window_t,
        ) -> xcb_shape_input_selected_cookie_t,
    >,
    xcb_shape_input_selected_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            destination_window: xcb_window_t,
        ) -> xcb_shape_input_selected_cookie_t,
    >,
    xcb_shape_input_selected_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_shape_input_selected_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_shape_input_selected_reply_t,
    >,
    xcb_shape_get_rectangles_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_shape_get_rectangles: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            source_kind: xcb_shape_kind_t,
        ) -> xcb_shape_get_rectangles_cookie_t,
    >,
    xcb_shape_get_rectangles_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            source_kind: xcb_shape_kind_t,
        ) -> xcb_shape_get_rectangles_cookie_t,
    >,
    xcb_shape_get_rectangles_rectangles:
        LazySymbol<unsafe fn(r: *const xcb_shape_get_rectangles_reply_t) -> *mut xcb_rectangle_t>,
    xcb_shape_get_rectangles_rectangles_length:
        LazySymbol<unsafe fn(r: *const xcb_shape_get_rectangles_reply_t) -> c_int>,
    xcb_shape_get_rectangles_rectangles_iterator: LazySymbol<
        unsafe fn(r: *const xcb_shape_get_rectangles_reply_t) -> xcb_rectangle_iterator_t,
    >,
    xcb_shape_get_rectangles_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_shape_get_rectangles_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_shape_get_rectangles_reply_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self
            .shape
            .$f
            .get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .shape
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

impl XcbShape {
    pub fn xcb_shape_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_shape_id) }
    }

    /// Returns `true` iff the symbol `xcb_shape_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_id(&self) -> bool {
        has_sym!(self, xcb_shape_id)
    }

    pub unsafe fn xcb_shape_op_next(&self, i: *mut xcb_shape_op_iterator_t) {
        sym!(self, xcb_shape_op_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_shape_op_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_op_next(&self) -> bool {
        has_sym!(self, xcb_shape_op_next)
    }

    pub unsafe fn xcb_shape_op_end(&self, i: xcb_shape_op_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_shape_op_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_shape_op_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_op_end(&self) -> bool {
        has_sym!(self, xcb_shape_op_end)
    }

    pub unsafe fn xcb_shape_kind_next(&self, i: *mut xcb_shape_kind_iterator_t) {
        sym!(self, xcb_shape_kind_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_shape_kind_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_kind_next(&self) -> bool {
        has_sym!(self, xcb_shape_kind_next)
    }

    pub unsafe fn xcb_shape_kind_end(
        &self,
        i: xcb_shape_kind_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_shape_kind_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_shape_kind_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_kind_end(&self) -> bool {
        has_sym!(self, xcb_shape_kind_end)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_shape_query_version(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_shape_query_version_cookie_t {
        sym!(self, xcb_shape_query_version)(c)
    }

    /// Returns `true` iff the symbol `xcb_shape_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_query_version(&self) -> bool {
        has_sym!(self, xcb_shape_query_version)
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
    pub unsafe fn xcb_shape_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_shape_query_version_cookie_t {
        sym!(self, xcb_shape_query_version_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_shape_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_query_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_shape_query_version_unchecked)
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
     * xcb_shape_query_version_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_shape_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_shape_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_shape_query_version_reply_t {
        sym!(self, xcb_shape_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_shape_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_shape_query_version_reply)
    }

    pub unsafe fn xcb_shape_rectangles_sizeof(
        &self,
        _buffer: *const c_void,
        rectangles_len: u32,
    ) -> c_int {
        sym!(self, xcb_shape_rectangles_sizeof)(_buffer, rectangles_len)
    }

    /// Returns `true` iff the symbol `xcb_shape_rectangles_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_rectangles_sizeof(&self) -> bool {
        has_sym!(self, xcb_shape_rectangles_sizeof)
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

    /// Returns `true` iff the symbol `xcb_shape_rectangles_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_rectangles_checked(&self) -> bool {
        has_sym!(self, xcb_shape_rectangles_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_shape_rectangles` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_rectangles(&self) -> bool {
        has_sym!(self, xcb_shape_rectangles)
    }

    pub unsafe fn xcb_shape_rectangles_rectangles(
        &self,
        r: *const xcb_shape_rectangles_request_t,
    ) -> *mut xcb_rectangle_t {
        sym!(self, xcb_shape_rectangles_rectangles)(r)
    }

    /// Returns `true` iff the symbol `xcb_shape_rectangles_rectangles` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_rectangles_rectangles(&self) -> bool {
        has_sym!(self, xcb_shape_rectangles_rectangles)
    }

    pub unsafe fn xcb_shape_rectangles_rectangles_length(
        &self,
        r: *const xcb_shape_rectangles_request_t,
    ) -> c_int {
        sym!(self, xcb_shape_rectangles_rectangles_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_shape_rectangles_rectangles_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_rectangles_rectangles_length(&self) -> bool {
        has_sym!(self, xcb_shape_rectangles_rectangles_length)
    }

    pub unsafe fn xcb_shape_rectangles_rectangles_iterator(
        &self,
        r: *const xcb_shape_rectangles_request_t,
    ) -> xcb_rectangle_iterator_t {
        sym!(self, xcb_shape_rectangles_rectangles_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_shape_rectangles_rectangles_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_rectangles_rectangles_iterator(&self) -> bool {
        has_sym!(self, xcb_shape_rectangles_rectangles_iterator)
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

    /// Returns `true` iff the symbol `xcb_shape_mask_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_mask_checked(&self) -> bool {
        has_sym!(self, xcb_shape_mask_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_shape_mask` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_mask(&self) -> bool {
        has_sym!(self, xcb_shape_mask)
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

    /// Returns `true` iff the symbol `xcb_shape_combine_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_combine_checked(&self) -> bool {
        has_sym!(self, xcb_shape_combine_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_shape_combine` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_combine(&self) -> bool {
        has_sym!(self, xcb_shape_combine)
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

    /// Returns `true` iff the symbol `xcb_shape_offset_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_offset_checked(&self) -> bool {
        has_sym!(self, xcb_shape_offset_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_shape_offset` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_offset(&self) -> bool {
        has_sym!(self, xcb_shape_offset)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_shape_query_extents(
        &self,
        c: *mut xcb_connection_t,
        destination_window: xcb_window_t,
    ) -> xcb_shape_query_extents_cookie_t {
        sym!(self, xcb_shape_query_extents)(c, destination_window)
    }

    /// Returns `true` iff the symbol `xcb_shape_query_extents` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_query_extents(&self) -> bool {
        has_sym!(self, xcb_shape_query_extents)
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
    pub unsafe fn xcb_shape_query_extents_unchecked(
        &self,
        c: *mut xcb_connection_t,
        destination_window: xcb_window_t,
    ) -> xcb_shape_query_extents_cookie_t {
        sym!(self, xcb_shape_query_extents_unchecked)(c, destination_window)
    }

    /// Returns `true` iff the symbol `xcb_shape_query_extents_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_query_extents_unchecked(&self) -> bool {
        has_sym!(self, xcb_shape_query_extents_unchecked)
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
     * xcb_shape_query_extents_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_shape_query_extents_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_shape_query_extents_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_shape_query_extents_reply_t {
        sym!(self, xcb_shape_query_extents_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_shape_query_extents_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_query_extents_reply(&self) -> bool {
        has_sym!(self, xcb_shape_query_extents_reply)
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
    pub unsafe fn xcb_shape_select_input_checked(
        &self,
        c: *mut xcb_connection_t,
        destination_window: xcb_window_t,
        enable: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_shape_select_input_checked)(c, destination_window, enable)
    }

    /// Returns `true` iff the symbol `xcb_shape_select_input_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_select_input_checked(&self) -> bool {
        has_sym!(self, xcb_shape_select_input_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_shape_select_input(
        &self,
        c: *mut xcb_connection_t,
        destination_window: xcb_window_t,
        enable: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_shape_select_input)(c, destination_window, enable)
    }

    /// Returns `true` iff the symbol `xcb_shape_select_input` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_select_input(&self) -> bool {
        has_sym!(self, xcb_shape_select_input)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_shape_input_selected(
        &self,
        c: *mut xcb_connection_t,
        destination_window: xcb_window_t,
    ) -> xcb_shape_input_selected_cookie_t {
        sym!(self, xcb_shape_input_selected)(c, destination_window)
    }

    /// Returns `true` iff the symbol `xcb_shape_input_selected` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_input_selected(&self) -> bool {
        has_sym!(self, xcb_shape_input_selected)
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
    pub unsafe fn xcb_shape_input_selected_unchecked(
        &self,
        c: *mut xcb_connection_t,
        destination_window: xcb_window_t,
    ) -> xcb_shape_input_selected_cookie_t {
        sym!(self, xcb_shape_input_selected_unchecked)(c, destination_window)
    }

    /// Returns `true` iff the symbol `xcb_shape_input_selected_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_input_selected_unchecked(&self) -> bool {
        has_sym!(self, xcb_shape_input_selected_unchecked)
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
     * xcb_shape_input_selected_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_shape_input_selected_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_shape_input_selected_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_shape_input_selected_reply_t {
        sym!(self, xcb_shape_input_selected_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_shape_input_selected_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_input_selected_reply(&self) -> bool {
        has_sym!(self, xcb_shape_input_selected_reply)
    }

    pub unsafe fn xcb_shape_get_rectangles_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_shape_get_rectangles_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_shape_get_rectangles_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_get_rectangles_sizeof(&self) -> bool {
        has_sym!(self, xcb_shape_get_rectangles_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_shape_get_rectangles(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        source_kind: xcb_shape_kind_t,
    ) -> xcb_shape_get_rectangles_cookie_t {
        sym!(self, xcb_shape_get_rectangles)(c, window, source_kind)
    }

    /// Returns `true` iff the symbol `xcb_shape_get_rectangles` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_get_rectangles(&self) -> bool {
        has_sym!(self, xcb_shape_get_rectangles)
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
    pub unsafe fn xcb_shape_get_rectangles_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        source_kind: xcb_shape_kind_t,
    ) -> xcb_shape_get_rectangles_cookie_t {
        sym!(self, xcb_shape_get_rectangles_unchecked)(c, window, source_kind)
    }

    /// Returns `true` iff the symbol `xcb_shape_get_rectangles_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_get_rectangles_unchecked(&self) -> bool {
        has_sym!(self, xcb_shape_get_rectangles_unchecked)
    }

    pub unsafe fn xcb_shape_get_rectangles_rectangles(
        &self,
        r: *const xcb_shape_get_rectangles_reply_t,
    ) -> *mut xcb_rectangle_t {
        sym!(self, xcb_shape_get_rectangles_rectangles)(r)
    }

    /// Returns `true` iff the symbol `xcb_shape_get_rectangles_rectangles` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_get_rectangles_rectangles(&self) -> bool {
        has_sym!(self, xcb_shape_get_rectangles_rectangles)
    }

    pub unsafe fn xcb_shape_get_rectangles_rectangles_length(
        &self,
        r: *const xcb_shape_get_rectangles_reply_t,
    ) -> c_int {
        sym!(self, xcb_shape_get_rectangles_rectangles_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_shape_get_rectangles_rectangles_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_get_rectangles_rectangles_length(&self) -> bool {
        has_sym!(self, xcb_shape_get_rectangles_rectangles_length)
    }

    pub unsafe fn xcb_shape_get_rectangles_rectangles_iterator(
        &self,
        r: *const xcb_shape_get_rectangles_reply_t,
    ) -> xcb_rectangle_iterator_t {
        sym!(self, xcb_shape_get_rectangles_rectangles_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_shape_get_rectangles_rectangles_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_get_rectangles_rectangles_iterator(&self) -> bool {
        has_sym!(self, xcb_shape_get_rectangles_rectangles_iterator)
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
     * xcb_shape_get_rectangles_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_shape_get_rectangles_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_shape_get_rectangles_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_shape_get_rectangles_reply_t {
        sym!(self, xcb_shape_get_rectangles_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_shape_get_rectangles_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shape_get_rectangles_reply(&self) -> bool {
        has_sym!(self, xcb_shape_get_rectangles_reply)
    }
}

#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbShape::load().unwrap() };
        assert!(lib.has_xcb_shape_id());
        assert!(lib.has_xcb_shape_op_next());
        assert!(lib.has_xcb_shape_op_end());
        assert!(lib.has_xcb_shape_kind_next());
        assert!(lib.has_xcb_shape_kind_end());
        assert!(lib.has_xcb_shape_query_version());
        assert!(lib.has_xcb_shape_query_version_unchecked());
        assert!(lib.has_xcb_shape_query_version_reply());
        assert!(lib.has_xcb_shape_rectangles_sizeof());
        assert!(lib.has_xcb_shape_rectangles_checked());
        assert!(lib.has_xcb_shape_rectangles());
        assert!(lib.has_xcb_shape_rectangles_rectangles());
        assert!(lib.has_xcb_shape_rectangles_rectangles_length());
        assert!(lib.has_xcb_shape_rectangles_rectangles_iterator());
        assert!(lib.has_xcb_shape_mask_checked());
        assert!(lib.has_xcb_shape_mask());
        assert!(lib.has_xcb_shape_combine_checked());
        assert!(lib.has_xcb_shape_combine());
        assert!(lib.has_xcb_shape_offset_checked());
        assert!(lib.has_xcb_shape_offset());
        assert!(lib.has_xcb_shape_query_extents());
        assert!(lib.has_xcb_shape_query_extents_unchecked());
        assert!(lib.has_xcb_shape_query_extents_reply());
        assert!(lib.has_xcb_shape_select_input_checked());
        assert!(lib.has_xcb_shape_select_input());
        assert!(lib.has_xcb_shape_input_selected());
        assert!(lib.has_xcb_shape_input_selected_unchecked());
        assert!(lib.has_xcb_shape_input_selected_reply());
        assert!(lib.has_xcb_shape_get_rectangles_sizeof());
        assert!(lib.has_xcb_shape_get_rectangles());
        assert!(lib.has_xcb_shape_get_rectangles_unchecked());
        assert!(lib.has_xcb_shape_get_rectangles_rectangles());
        assert!(lib.has_xcb_shape_get_rectangles_rectangles_length());
        assert!(lib.has_xcb_shape_get_rectangles_rectangles_iterator());
        assert!(lib.has_xcb_shape_get_rectangles_reply());
    }
}
