// This file was generated using generate.py. Do not edit.

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_query_version_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_dri3_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_dri3_query_version.
pub const XCB_DRI3_QUERY_VERSION: u8 = 0i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub major_version: u32,
    pub minor_version: u32,
}

impl Default for xcb_dri3_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
}

impl Default for xcb_dri3_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_open_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_dri3_open_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_dri3_open.
pub const XCB_DRI3_OPEN: u8 = 1i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_open_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub provider: u32,
}

impl Default for xcb_dri3_open_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_open_reply_t {
    pub response_type: u8,
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad0: [u8; 24],
}

impl Default for xcb_dri3_open_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_dri3_pixmap_from_buffer.
pub const XCB_DRI3_PIXMAP_FROM_BUFFER: u8 = 2i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_pixmap_from_buffer_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub pixmap: xcb_pixmap_t,
    pub drawable: xcb_drawable_t,
    pub size: u32,
    pub width: u16,
    pub height: u16,
    pub stride: u16,
    pub depth: u8,
    pub bpp: u8,
}

impl Default for xcb_dri3_pixmap_from_buffer_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_buffer_from_pixmap_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_dri3_buffer_from_pixmap_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_dri3_buffer_from_pixmap.
pub const XCB_DRI3_BUFFER_FROM_PIXMAP: u8 = 3i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_buffer_from_pixmap_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub pixmap: xcb_pixmap_t,
}

impl Default for xcb_dri3_buffer_from_pixmap_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_buffer_from_pixmap_reply_t {
    pub response_type: u8,
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub size: u32,
    pub width: u16,
    pub height: u16,
    pub stride: u16,
    pub depth: u8,
    pub bpp: u8,
    pub pad0: [u8; 12],
}

impl Default for xcb_dri3_buffer_from_pixmap_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_dri3_fence_from_fd.
pub const XCB_DRI3_FENCE_FROM_FD: u8 = 4i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_fence_from_fd_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub fence: u32,
    pub initially_triggered: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_dri3_fence_from_fd_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_fd_from_fence_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_dri3_fd_from_fence_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_dri3_fd_from_fence.
pub const XCB_DRI3_FD_FROM_FENCE: u8 = 5i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_fd_from_fence_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub fence: u32,
}

impl Default for xcb_dri3_fd_from_fence_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_fd_from_fence_reply_t {
    pub response_type: u8,
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad0: [u8; 24],
}

impl Default for xcb_dri3_fd_from_fence_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_get_supported_modifiers_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_dri3_get_supported_modifiers_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_dri3_get_supported_modifiers.
pub const XCB_DRI3_GET_SUPPORTED_MODIFIERS: u8 = 6i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_get_supported_modifiers_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: u32,
    pub depth: u8,
    pub bpp: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_dri3_get_supported_modifiers_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_get_supported_modifiers_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_window_modifiers: u32,
    pub num_screen_modifiers: u32,
    pub pad1: [u8; 16],
}

impl Default for xcb_dri3_get_supported_modifiers_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_dri3_pixmap_from_buffers.
pub const XCB_DRI3_PIXMAP_FROM_BUFFERS: u8 = 7i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_pixmap_from_buffers_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub pixmap: xcb_pixmap_t,
    pub window: xcb_window_t,
    pub num_buffers: u8,
    pub pad0: [u8; 3],
    pub width: u16,
    pub height: u16,
    pub stride0: u32,
    pub offset0: u32,
    pub stride1: u32,
    pub offset1: u32,
    pub stride2: u32,
    pub offset2: u32,
    pub stride3: u32,
    pub offset3: u32,
    pub depth: u8,
    pub bpp: u8,
    pub pad1: [u8; 2],
    pub modifier: u64,
}

impl Default for xcb_dri3_pixmap_from_buffers_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_buffers_from_pixmap_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_dri3_buffers_from_pixmap_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_dri3_buffers_from_pixmap.
pub const XCB_DRI3_BUFFERS_FROM_PIXMAP: u8 = 8i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_buffers_from_pixmap_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub pixmap: xcb_pixmap_t,
}

impl Default for xcb_dri3_buffers_from_pixmap_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_buffers_from_pixmap_reply_t {
    pub response_type: u8,
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: u16,
    pub height: u16,
    pub pad0: [u8; 4],
    pub modifier: u64,
    pub depth: u8,
    pub bpp: u8,
    pub pad1: [u8; 6],
}

impl Default for xcb_dri3_buffers_from_pixmap_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub(crate) struct XcbDri3Dri3 {
    xcb_dri3_id: LazySymbol<*mut xcb_extension_t>,
    xcb_dri3_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u32,
            minor_version: u32,
        ) -> xcb_dri3_query_version_cookie_t,
    >,
    xcb_dri3_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u32,
            minor_version: u32,
        ) -> xcb_dri3_query_version_cookie_t,
    >,
    xcb_dri3_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dri3_query_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_dri3_query_version_reply_t,
    >,
    xcb_dri3_open: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            provider: u32,
        ) -> xcb_dri3_open_cookie_t,
    >,
    xcb_dri3_open_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            provider: u32,
        ) -> xcb_dri3_open_cookie_t,
    >,
    xcb_dri3_open_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dri3_open_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_dri3_open_reply_t,
    >,
    xcb_dri3_open_reply_fds: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, reply: *mut xcb_dri3_open_reply_t) -> *mut c_int,
    >,
    xcb_dri3_pixmap_from_buffer_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            pixmap: xcb_pixmap_t,
            drawable: xcb_drawable_t,
            size: u32,
            width: u16,
            height: u16,
            stride: u16,
            depth: u8,
            bpp: u8,
            pixmap_fd: i32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_dri3_pixmap_from_buffer: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            pixmap: xcb_pixmap_t,
            drawable: xcb_drawable_t,
            size: u32,
            width: u16,
            height: u16,
            stride: u16,
            depth: u8,
            bpp: u8,
            pixmap_fd: i32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_dri3_buffer_from_pixmap: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            pixmap: xcb_pixmap_t,
        ) -> xcb_dri3_buffer_from_pixmap_cookie_t,
    >,
    xcb_dri3_buffer_from_pixmap_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            pixmap: xcb_pixmap_t,
        ) -> xcb_dri3_buffer_from_pixmap_cookie_t,
    >,
    xcb_dri3_buffer_from_pixmap_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dri3_buffer_from_pixmap_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_dri3_buffer_from_pixmap_reply_t,
    >,
    xcb_dri3_buffer_from_pixmap_reply_fds: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            reply: *mut xcb_dri3_buffer_from_pixmap_reply_t,
        ) -> *mut c_int,
    >,
    xcb_dri3_fence_from_fd_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            fence: u32,
            initially_triggered: u8,
            fence_fd: i32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_dri3_fence_from_fd: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            fence: u32,
            initially_triggered: u8,
            fence_fd: i32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_dri3_fd_from_fence: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            fence: u32,
        ) -> xcb_dri3_fd_from_fence_cookie_t,
    >,
    xcb_dri3_fd_from_fence_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            fence: u32,
        ) -> xcb_dri3_fd_from_fence_cookie_t,
    >,
    xcb_dri3_fd_from_fence_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dri3_fd_from_fence_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_dri3_fd_from_fence_reply_t,
    >,
    xcb_dri3_fd_from_fence_reply_fds: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            reply: *mut xcb_dri3_fd_from_fence_reply_t,
        ) -> *mut c_int,
    >,
    xcb_dri3_get_supported_modifiers_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_dri3_get_supported_modifiers: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: u32,
            depth: u8,
            bpp: u8,
        ) -> xcb_dri3_get_supported_modifiers_cookie_t,
    >,
    xcb_dri3_get_supported_modifiers_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: u32,
            depth: u8,
            bpp: u8,
        ) -> xcb_dri3_get_supported_modifiers_cookie_t,
    >,
    xcb_dri3_get_supported_modifiers_window_modifiers:
        LazySymbol<unsafe fn(r: *const xcb_dri3_get_supported_modifiers_reply_t) -> *mut u64>,
    xcb_dri3_get_supported_modifiers_window_modifiers_length:
        LazySymbol<unsafe fn(r: *const xcb_dri3_get_supported_modifiers_reply_t) -> c_int>,
    xcb_dri3_get_supported_modifiers_window_modifiers_end: LazySymbol<
        unsafe fn(r: *const xcb_dri3_get_supported_modifiers_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_dri3_get_supported_modifiers_screen_modifiers:
        LazySymbol<unsafe fn(r: *const xcb_dri3_get_supported_modifiers_reply_t) -> *mut u64>,
    xcb_dri3_get_supported_modifiers_screen_modifiers_length:
        LazySymbol<unsafe fn(r: *const xcb_dri3_get_supported_modifiers_reply_t) -> c_int>,
    xcb_dri3_get_supported_modifiers_screen_modifiers_end: LazySymbol<
        unsafe fn(r: *const xcb_dri3_get_supported_modifiers_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_dri3_get_supported_modifiers_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dri3_get_supported_modifiers_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_dri3_get_supported_modifiers_reply_t,
    >,
    xcb_dri3_pixmap_from_buffers_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            pixmap: xcb_pixmap_t,
            window: xcb_window_t,
            num_buffers: u8,
            width: u16,
            height: u16,
            stride0: u32,
            offset0: u32,
            stride1: u32,
            offset1: u32,
            stride2: u32,
            offset2: u32,
            stride3: u32,
            offset3: u32,
            depth: u8,
            bpp: u8,
            modifier: u64,
            buffers: *const i32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_dri3_pixmap_from_buffers: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            pixmap: xcb_pixmap_t,
            window: xcb_window_t,
            num_buffers: u8,
            width: u16,
            height: u16,
            stride0: u32,
            offset0: u32,
            stride1: u32,
            offset1: u32,
            stride2: u32,
            offset2: u32,
            stride3: u32,
            offset3: u32,
            depth: u8,
            bpp: u8,
            modifier: u64,
            buffers: *const i32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_dri3_buffers_from_pixmap_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, buffers: i32) -> c_int>,
    xcb_dri3_buffers_from_pixmap: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            pixmap: xcb_pixmap_t,
        ) -> xcb_dri3_buffers_from_pixmap_cookie_t,
    >,
    xcb_dri3_buffers_from_pixmap_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            pixmap: xcb_pixmap_t,
        ) -> xcb_dri3_buffers_from_pixmap_cookie_t,
    >,
    xcb_dri3_buffers_from_pixmap_strides:
        LazySymbol<unsafe fn(r: *const xcb_dri3_buffers_from_pixmap_reply_t) -> *mut u32>,
    xcb_dri3_buffers_from_pixmap_strides_length:
        LazySymbol<unsafe fn(r: *const xcb_dri3_buffers_from_pixmap_reply_t) -> c_int>,
    xcb_dri3_buffers_from_pixmap_strides_end: LazySymbol<
        unsafe fn(r: *const xcb_dri3_buffers_from_pixmap_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_dri3_buffers_from_pixmap_offsets:
        LazySymbol<unsafe fn(r: *const xcb_dri3_buffers_from_pixmap_reply_t) -> *mut u32>,
    xcb_dri3_buffers_from_pixmap_offsets_length:
        LazySymbol<unsafe fn(r: *const xcb_dri3_buffers_from_pixmap_reply_t) -> c_int>,
    xcb_dri3_buffers_from_pixmap_offsets_end: LazySymbol<
        unsafe fn(r: *const xcb_dri3_buffers_from_pixmap_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_dri3_buffers_from_pixmap_buffers:
        LazySymbol<unsafe fn(r: *const xcb_dri3_buffers_from_pixmap_reply_t) -> *mut i32>,
    xcb_dri3_buffers_from_pixmap_buffers_length:
        LazySymbol<unsafe fn(r: *const xcb_dri3_buffers_from_pixmap_reply_t) -> c_int>,
    xcb_dri3_buffers_from_pixmap_buffers_end: LazySymbol<
        unsafe fn(r: *const xcb_dri3_buffers_from_pixmap_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_dri3_buffers_from_pixmap_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dri3_buffers_from_pixmap_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_dri3_buffers_from_pixmap_reply_t,
    >,
    xcb_dri3_buffers_from_pixmap_reply_fds: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            reply: *mut xcb_dri3_buffers_from_pixmap_reply_t,
        ) -> *mut c_int,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self.dri3.$f.get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .dri3
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

impl XcbDri3 {
    pub fn xcb_dri3_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_dri3_id) }
    }

    /// Returns `true` iff the symbol `xcb_dri3_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_id(&self) -> bool {
        has_sym!(self, xcb_dri3_id)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_dri3_query_version(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
    ) -> xcb_dri3_query_version_cookie_t {
        sym!(self, xcb_dri3_query_version)(c, major_version, minor_version)
    }

    /// Returns `true` iff the symbol `xcb_dri3_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_query_version(&self) -> bool {
        has_sym!(self, xcb_dri3_query_version)
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
    pub unsafe fn xcb_dri3_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
    ) -> xcb_dri3_query_version_cookie_t {
        sym!(self, xcb_dri3_query_version_unchecked)(c, major_version, minor_version)
    }

    /// Returns `true` iff the symbol `xcb_dri3_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_query_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_dri3_query_version_unchecked)
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
     * xcb_dri3_query_version_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_dri3_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri3_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_dri3_query_version_reply_t {
        sym!(self, xcb_dri3_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dri3_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_dri3_query_version_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_dri3_open(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        provider: u32,
    ) -> xcb_dri3_open_cookie_t {
        sym!(self, xcb_dri3_open)(c, drawable, provider)
    }

    /// Returns `true` iff the symbol `xcb_dri3_open` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_open(&self) -> bool {
        has_sym!(self, xcb_dri3_open)
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
    pub unsafe fn xcb_dri3_open_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        provider: u32,
    ) -> xcb_dri3_open_cookie_t {
        sym!(self, xcb_dri3_open_unchecked)(c, drawable, provider)
    }

    /// Returns `true` iff the symbol `xcb_dri3_open_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_open_unchecked(&self) -> bool {
        has_sym!(self, xcb_dri3_open_unchecked)
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
     * xcb_dri3_open_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_dri3_open_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri3_open_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_dri3_open_reply_t {
        sym!(self, xcb_dri3_open_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dri3_open_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_open_reply(&self) -> bool {
        has_sym!(self, xcb_dri3_open_reply)
    }
    pub unsafe fn xcb_dri3_open_reply_fds(
        &self,
        c: *mut xcb_connection_t,
        reply: *mut xcb_dri3_open_reply_t,
    ) -> *mut c_int {
        sym!(self, xcb_dri3_open_reply_fds)(c, reply)
    }

    /// Returns `true` iff the symbol `xcb_dri3_open_reply_fds` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_open_reply_fds(&self) -> bool {
        has_sym!(self, xcb_dri3_open_reply_fds)
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
    pub unsafe fn xcb_dri3_pixmap_from_buffer_checked(
        &self,
        c: *mut xcb_connection_t,
        pixmap: xcb_pixmap_t,
        drawable: xcb_drawable_t,
        size: u32,
        width: u16,
        height: u16,
        stride: u16,
        depth: u8,
        bpp: u8,
        pixmap_fd: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dri3_pixmap_from_buffer_checked)(
            c, pixmap, drawable, size, width, height, stride, depth, bpp, pixmap_fd,
        )
    }

    /// Returns `true` iff the symbol `xcb_dri3_pixmap_from_buffer_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_pixmap_from_buffer_checked(&self) -> bool {
        has_sym!(self, xcb_dri3_pixmap_from_buffer_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_dri3_pixmap_from_buffer(
        &self,
        c: *mut xcb_connection_t,
        pixmap: xcb_pixmap_t,
        drawable: xcb_drawable_t,
        size: u32,
        width: u16,
        height: u16,
        stride: u16,
        depth: u8,
        bpp: u8,
        pixmap_fd: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dri3_pixmap_from_buffer)(
            c, pixmap, drawable, size, width, height, stride, depth, bpp, pixmap_fd,
        )
    }

    /// Returns `true` iff the symbol `xcb_dri3_pixmap_from_buffer` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_pixmap_from_buffer(&self) -> bool {
        has_sym!(self, xcb_dri3_pixmap_from_buffer)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_dri3_buffer_from_pixmap(
        &self,
        c: *mut xcb_connection_t,
        pixmap: xcb_pixmap_t,
    ) -> xcb_dri3_buffer_from_pixmap_cookie_t {
        sym!(self, xcb_dri3_buffer_from_pixmap)(c, pixmap)
    }

    /// Returns `true` iff the symbol `xcb_dri3_buffer_from_pixmap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_buffer_from_pixmap(&self) -> bool {
        has_sym!(self, xcb_dri3_buffer_from_pixmap)
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
    pub unsafe fn xcb_dri3_buffer_from_pixmap_unchecked(
        &self,
        c: *mut xcb_connection_t,
        pixmap: xcb_pixmap_t,
    ) -> xcb_dri3_buffer_from_pixmap_cookie_t {
        sym!(self, xcb_dri3_buffer_from_pixmap_unchecked)(c, pixmap)
    }

    /// Returns `true` iff the symbol `xcb_dri3_buffer_from_pixmap_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_buffer_from_pixmap_unchecked(&self) -> bool {
        has_sym!(self, xcb_dri3_buffer_from_pixmap_unchecked)
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
     * xcb_dri3_buffer_from_pixmap_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_dri3_buffer_from_pixmap_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri3_buffer_from_pixmap_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_dri3_buffer_from_pixmap_reply_t {
        sym!(self, xcb_dri3_buffer_from_pixmap_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dri3_buffer_from_pixmap_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_buffer_from_pixmap_reply(&self) -> bool {
        has_sym!(self, xcb_dri3_buffer_from_pixmap_reply)
    }
    pub unsafe fn xcb_dri3_buffer_from_pixmap_reply_fds(
        &self,
        c: *mut xcb_connection_t,
        reply: *mut xcb_dri3_buffer_from_pixmap_reply_t,
    ) -> *mut c_int {
        sym!(self, xcb_dri3_buffer_from_pixmap_reply_fds)(c, reply)
    }

    /// Returns `true` iff the symbol `xcb_dri3_buffer_from_pixmap_reply_fds` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_buffer_from_pixmap_reply_fds(&self) -> bool {
        has_sym!(self, xcb_dri3_buffer_from_pixmap_reply_fds)
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
    pub unsafe fn xcb_dri3_fence_from_fd_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        fence: u32,
        initially_triggered: u8,
        fence_fd: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dri3_fence_from_fd_checked)(
            c,
            drawable,
            fence,
            initially_triggered,
            fence_fd,
        )
    }

    /// Returns `true` iff the symbol `xcb_dri3_fence_from_fd_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_fence_from_fd_checked(&self) -> bool {
        has_sym!(self, xcb_dri3_fence_from_fd_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_dri3_fence_from_fd(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        fence: u32,
        initially_triggered: u8,
        fence_fd: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dri3_fence_from_fd)(c, drawable, fence, initially_triggered, fence_fd)
    }

    /// Returns `true` iff the symbol `xcb_dri3_fence_from_fd` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_fence_from_fd(&self) -> bool {
        has_sym!(self, xcb_dri3_fence_from_fd)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_dri3_fd_from_fence(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        fence: u32,
    ) -> xcb_dri3_fd_from_fence_cookie_t {
        sym!(self, xcb_dri3_fd_from_fence)(c, drawable, fence)
    }

    /// Returns `true` iff the symbol `xcb_dri3_fd_from_fence` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_fd_from_fence(&self) -> bool {
        has_sym!(self, xcb_dri3_fd_from_fence)
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
    pub unsafe fn xcb_dri3_fd_from_fence_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        fence: u32,
    ) -> xcb_dri3_fd_from_fence_cookie_t {
        sym!(self, xcb_dri3_fd_from_fence_unchecked)(c, drawable, fence)
    }

    /// Returns `true` iff the symbol `xcb_dri3_fd_from_fence_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_fd_from_fence_unchecked(&self) -> bool {
        has_sym!(self, xcb_dri3_fd_from_fence_unchecked)
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
     * xcb_dri3_fd_from_fence_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_dri3_fd_from_fence_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri3_fd_from_fence_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_dri3_fd_from_fence_reply_t {
        sym!(self, xcb_dri3_fd_from_fence_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dri3_fd_from_fence_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_fd_from_fence_reply(&self) -> bool {
        has_sym!(self, xcb_dri3_fd_from_fence_reply)
    }
    pub unsafe fn xcb_dri3_fd_from_fence_reply_fds(
        &self,
        c: *mut xcb_connection_t,
        reply: *mut xcb_dri3_fd_from_fence_reply_t,
    ) -> *mut c_int {
        sym!(self, xcb_dri3_fd_from_fence_reply_fds)(c, reply)
    }

    /// Returns `true` iff the symbol `xcb_dri3_fd_from_fence_reply_fds` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_fd_from_fence_reply_fds(&self) -> bool {
        has_sym!(self, xcb_dri3_fd_from_fence_reply_fds)
    }

    pub unsafe fn xcb_dri3_get_supported_modifiers_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_dri3_get_supported_modifiers_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_dri3_get_supported_modifiers_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_get_supported_modifiers_sizeof(&self) -> bool {
        has_sym!(self, xcb_dri3_get_supported_modifiers_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_dri3_get_supported_modifiers(
        &self,
        c: *mut xcb_connection_t,
        window: u32,
        depth: u8,
        bpp: u8,
    ) -> xcb_dri3_get_supported_modifiers_cookie_t {
        sym!(self, xcb_dri3_get_supported_modifiers)(c, window, depth, bpp)
    }

    /// Returns `true` iff the symbol `xcb_dri3_get_supported_modifiers` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_get_supported_modifiers(&self) -> bool {
        has_sym!(self, xcb_dri3_get_supported_modifiers)
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
    pub unsafe fn xcb_dri3_get_supported_modifiers_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: u32,
        depth: u8,
        bpp: u8,
    ) -> xcb_dri3_get_supported_modifiers_cookie_t {
        sym!(self, xcb_dri3_get_supported_modifiers_unchecked)(c, window, depth, bpp)
    }

    /// Returns `true` iff the symbol `xcb_dri3_get_supported_modifiers_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_get_supported_modifiers_unchecked(&self) -> bool {
        has_sym!(self, xcb_dri3_get_supported_modifiers_unchecked)
    }

    pub unsafe fn xcb_dri3_get_supported_modifiers_window_modifiers(
        &self,
        r: *const xcb_dri3_get_supported_modifiers_reply_t,
    ) -> *mut u64 {
        sym!(self, xcb_dri3_get_supported_modifiers_window_modifiers)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri3_get_supported_modifiers_window_modifiers` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_get_supported_modifiers_window_modifiers(&self) -> bool {
        has_sym!(self, xcb_dri3_get_supported_modifiers_window_modifiers)
    }

    pub unsafe fn xcb_dri3_get_supported_modifiers_window_modifiers_length(
        &self,
        r: *const xcb_dri3_get_supported_modifiers_reply_t,
    ) -> c_int {
        sym!(
            self,
            xcb_dri3_get_supported_modifiers_window_modifiers_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_dri3_get_supported_modifiers_window_modifiers_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_get_supported_modifiers_window_modifiers_length(&self) -> bool {
        has_sym!(
            self,
            xcb_dri3_get_supported_modifiers_window_modifiers_length
        )
    }

    pub unsafe fn xcb_dri3_get_supported_modifiers_window_modifiers_end(
        &self,
        r: *const xcb_dri3_get_supported_modifiers_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_dri3_get_supported_modifiers_window_modifiers_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri3_get_supported_modifiers_window_modifiers_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_get_supported_modifiers_window_modifiers_end(&self) -> bool {
        has_sym!(self, xcb_dri3_get_supported_modifiers_window_modifiers_end)
    }

    pub unsafe fn xcb_dri3_get_supported_modifiers_screen_modifiers(
        &self,
        r: *const xcb_dri3_get_supported_modifiers_reply_t,
    ) -> *mut u64 {
        sym!(self, xcb_dri3_get_supported_modifiers_screen_modifiers)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri3_get_supported_modifiers_screen_modifiers` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_get_supported_modifiers_screen_modifiers(&self) -> bool {
        has_sym!(self, xcb_dri3_get_supported_modifiers_screen_modifiers)
    }

    pub unsafe fn xcb_dri3_get_supported_modifiers_screen_modifiers_length(
        &self,
        r: *const xcb_dri3_get_supported_modifiers_reply_t,
    ) -> c_int {
        sym!(
            self,
            xcb_dri3_get_supported_modifiers_screen_modifiers_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_dri3_get_supported_modifiers_screen_modifiers_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_get_supported_modifiers_screen_modifiers_length(&self) -> bool {
        has_sym!(
            self,
            xcb_dri3_get_supported_modifiers_screen_modifiers_length
        )
    }

    pub unsafe fn xcb_dri3_get_supported_modifiers_screen_modifiers_end(
        &self,
        r: *const xcb_dri3_get_supported_modifiers_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_dri3_get_supported_modifiers_screen_modifiers_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri3_get_supported_modifiers_screen_modifiers_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_get_supported_modifiers_screen_modifiers_end(&self) -> bool {
        has_sym!(self, xcb_dri3_get_supported_modifiers_screen_modifiers_end)
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
     * xcb_dri3_get_supported_modifiers_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_dri3_get_supported_modifiers_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri3_get_supported_modifiers_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_dri3_get_supported_modifiers_reply_t {
        sym!(self, xcb_dri3_get_supported_modifiers_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dri3_get_supported_modifiers_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_get_supported_modifiers_reply(&self) -> bool {
        has_sym!(self, xcb_dri3_get_supported_modifiers_reply)
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
    pub unsafe fn xcb_dri3_pixmap_from_buffers_checked(
        &self,
        c: *mut xcb_connection_t,
        pixmap: xcb_pixmap_t,
        window: xcb_window_t,
        num_buffers: u8,
        width: u16,
        height: u16,
        stride0: u32,
        offset0: u32,
        stride1: u32,
        offset1: u32,
        stride2: u32,
        offset2: u32,
        stride3: u32,
        offset3: u32,
        depth: u8,
        bpp: u8,
        modifier: u64,
        buffers: *const i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dri3_pixmap_from_buffers_checked)(
            c,
            pixmap,
            window,
            num_buffers,
            width,
            height,
            stride0,
            offset0,
            stride1,
            offset1,
            stride2,
            offset2,
            stride3,
            offset3,
            depth,
            bpp,
            modifier,
            buffers,
        )
    }

    /// Returns `true` iff the symbol `xcb_dri3_pixmap_from_buffers_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_pixmap_from_buffers_checked(&self) -> bool {
        has_sym!(self, xcb_dri3_pixmap_from_buffers_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_dri3_pixmap_from_buffers(
        &self,
        c: *mut xcb_connection_t,
        pixmap: xcb_pixmap_t,
        window: xcb_window_t,
        num_buffers: u8,
        width: u16,
        height: u16,
        stride0: u32,
        offset0: u32,
        stride1: u32,
        offset1: u32,
        stride2: u32,
        offset2: u32,
        stride3: u32,
        offset3: u32,
        depth: u8,
        bpp: u8,
        modifier: u64,
        buffers: *const i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dri3_pixmap_from_buffers)(
            c,
            pixmap,
            window,
            num_buffers,
            width,
            height,
            stride0,
            offset0,
            stride1,
            offset1,
            stride2,
            offset2,
            stride3,
            offset3,
            depth,
            bpp,
            modifier,
            buffers,
        )
    }

    /// Returns `true` iff the symbol `xcb_dri3_pixmap_from_buffers` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_pixmap_from_buffers(&self) -> bool {
        has_sym!(self, xcb_dri3_pixmap_from_buffers)
    }

    pub unsafe fn xcb_dri3_buffers_from_pixmap_sizeof(
        &self,
        _buffer: *const c_void,
        buffers: i32,
    ) -> c_int {
        sym!(self, xcb_dri3_buffers_from_pixmap_sizeof)(_buffer, buffers)
    }

    /// Returns `true` iff the symbol `xcb_dri3_buffers_from_pixmap_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_buffers_from_pixmap_sizeof(&self) -> bool {
        has_sym!(self, xcb_dri3_buffers_from_pixmap_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_dri3_buffers_from_pixmap(
        &self,
        c: *mut xcb_connection_t,
        pixmap: xcb_pixmap_t,
    ) -> xcb_dri3_buffers_from_pixmap_cookie_t {
        sym!(self, xcb_dri3_buffers_from_pixmap)(c, pixmap)
    }

    /// Returns `true` iff the symbol `xcb_dri3_buffers_from_pixmap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_buffers_from_pixmap(&self) -> bool {
        has_sym!(self, xcb_dri3_buffers_from_pixmap)
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
    pub unsafe fn xcb_dri3_buffers_from_pixmap_unchecked(
        &self,
        c: *mut xcb_connection_t,
        pixmap: xcb_pixmap_t,
    ) -> xcb_dri3_buffers_from_pixmap_cookie_t {
        sym!(self, xcb_dri3_buffers_from_pixmap_unchecked)(c, pixmap)
    }

    /// Returns `true` iff the symbol `xcb_dri3_buffers_from_pixmap_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_buffers_from_pixmap_unchecked(&self) -> bool {
        has_sym!(self, xcb_dri3_buffers_from_pixmap_unchecked)
    }

    pub unsafe fn xcb_dri3_buffers_from_pixmap_strides(
        &self,
        r: *const xcb_dri3_buffers_from_pixmap_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_dri3_buffers_from_pixmap_strides)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri3_buffers_from_pixmap_strides` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_buffers_from_pixmap_strides(&self) -> bool {
        has_sym!(self, xcb_dri3_buffers_from_pixmap_strides)
    }

    pub unsafe fn xcb_dri3_buffers_from_pixmap_strides_length(
        &self,
        r: *const xcb_dri3_buffers_from_pixmap_reply_t,
    ) -> c_int {
        sym!(self, xcb_dri3_buffers_from_pixmap_strides_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri3_buffers_from_pixmap_strides_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_buffers_from_pixmap_strides_length(&self) -> bool {
        has_sym!(self, xcb_dri3_buffers_from_pixmap_strides_length)
    }

    pub unsafe fn xcb_dri3_buffers_from_pixmap_strides_end(
        &self,
        r: *const xcb_dri3_buffers_from_pixmap_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_dri3_buffers_from_pixmap_strides_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri3_buffers_from_pixmap_strides_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_buffers_from_pixmap_strides_end(&self) -> bool {
        has_sym!(self, xcb_dri3_buffers_from_pixmap_strides_end)
    }

    pub unsafe fn xcb_dri3_buffers_from_pixmap_offsets(
        &self,
        r: *const xcb_dri3_buffers_from_pixmap_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_dri3_buffers_from_pixmap_offsets)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri3_buffers_from_pixmap_offsets` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_buffers_from_pixmap_offsets(&self) -> bool {
        has_sym!(self, xcb_dri3_buffers_from_pixmap_offsets)
    }

    pub unsafe fn xcb_dri3_buffers_from_pixmap_offsets_length(
        &self,
        r: *const xcb_dri3_buffers_from_pixmap_reply_t,
    ) -> c_int {
        sym!(self, xcb_dri3_buffers_from_pixmap_offsets_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri3_buffers_from_pixmap_offsets_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_buffers_from_pixmap_offsets_length(&self) -> bool {
        has_sym!(self, xcb_dri3_buffers_from_pixmap_offsets_length)
    }

    pub unsafe fn xcb_dri3_buffers_from_pixmap_offsets_end(
        &self,
        r: *const xcb_dri3_buffers_from_pixmap_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_dri3_buffers_from_pixmap_offsets_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri3_buffers_from_pixmap_offsets_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_buffers_from_pixmap_offsets_end(&self) -> bool {
        has_sym!(self, xcb_dri3_buffers_from_pixmap_offsets_end)
    }

    pub unsafe fn xcb_dri3_buffers_from_pixmap_buffers(
        &self,
        r: *const xcb_dri3_buffers_from_pixmap_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_dri3_buffers_from_pixmap_buffers)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri3_buffers_from_pixmap_buffers` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_buffers_from_pixmap_buffers(&self) -> bool {
        has_sym!(self, xcb_dri3_buffers_from_pixmap_buffers)
    }

    pub unsafe fn xcb_dri3_buffers_from_pixmap_buffers_length(
        &self,
        r: *const xcb_dri3_buffers_from_pixmap_reply_t,
    ) -> c_int {
        sym!(self, xcb_dri3_buffers_from_pixmap_buffers_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri3_buffers_from_pixmap_buffers_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_buffers_from_pixmap_buffers_length(&self) -> bool {
        has_sym!(self, xcb_dri3_buffers_from_pixmap_buffers_length)
    }

    pub unsafe fn xcb_dri3_buffers_from_pixmap_buffers_end(
        &self,
        r: *const xcb_dri3_buffers_from_pixmap_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_dri3_buffers_from_pixmap_buffers_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri3_buffers_from_pixmap_buffers_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_buffers_from_pixmap_buffers_end(&self) -> bool {
        has_sym!(self, xcb_dri3_buffers_from_pixmap_buffers_end)
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
     * xcb_dri3_buffers_from_pixmap_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_dri3_buffers_from_pixmap_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri3_buffers_from_pixmap_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_dri3_buffers_from_pixmap_reply_t {
        sym!(self, xcb_dri3_buffers_from_pixmap_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dri3_buffers_from_pixmap_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_buffers_from_pixmap_reply(&self) -> bool {
        has_sym!(self, xcb_dri3_buffers_from_pixmap_reply)
    }
    pub unsafe fn xcb_dri3_buffers_from_pixmap_reply_fds(
        &self,
        c: *mut xcb_connection_t,
        reply: *mut xcb_dri3_buffers_from_pixmap_reply_t,
    ) -> *mut c_int {
        sym!(self, xcb_dri3_buffers_from_pixmap_reply_fds)(c, reply)
    }

    /// Returns `true` iff the symbol `xcb_dri3_buffers_from_pixmap_reply_fds` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_buffers_from_pixmap_reply_fds(&self) -> bool {
        has_sym!(self, xcb_dri3_buffers_from_pixmap_reply_fds)
    }
}

#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbDri3::load().unwrap() };
        assert!(lib.has_xcb_dri3_id());
        assert!(lib.has_xcb_dri3_query_version());
        assert!(lib.has_xcb_dri3_query_version_unchecked());
        assert!(lib.has_xcb_dri3_query_version_reply());
        assert!(lib.has_xcb_dri3_open());
        assert!(lib.has_xcb_dri3_open_unchecked());
        assert!(lib.has_xcb_dri3_open_reply());

        /**
         * Return the reply fds
         * @param c      The connection
         * @param reply  The reply
         *
         * Returns the array of reply fds of the request asked by
         *
         * The returned value must be freed by the caller using free().
         */
        assert!(lib.has_xcb_dri3_open_reply_fds());
        assert!(lib.has_xcb_dri3_pixmap_from_buffer_checked());
        assert!(lib.has_xcb_dri3_pixmap_from_buffer());
        assert!(lib.has_xcb_dri3_buffer_from_pixmap());
        assert!(lib.has_xcb_dri3_buffer_from_pixmap_unchecked());
        assert!(lib.has_xcb_dri3_buffer_from_pixmap_reply());

        /**
         * Return the reply fds
         * @param c      The connection
         * @param reply  The reply
         *
         * Returns the array of reply fds of the request asked by
         *
         * The returned value must be freed by the caller using free().
         */
        assert!(lib.has_xcb_dri3_buffer_from_pixmap_reply_fds());
        assert!(lib.has_xcb_dri3_fence_from_fd_checked());
        assert!(lib.has_xcb_dri3_fence_from_fd());
        assert!(lib.has_xcb_dri3_fd_from_fence());
        assert!(lib.has_xcb_dri3_fd_from_fence_unchecked());
        assert!(lib.has_xcb_dri3_fd_from_fence_reply());

        /**
         * Return the reply fds
         * @param c      The connection
         * @param reply  The reply
         *
         * Returns the array of reply fds of the request asked by
         *
         * The returned value must be freed by the caller using free().
         */
        assert!(lib.has_xcb_dri3_fd_from_fence_reply_fds());
        assert!(lib.has_xcb_dri3_get_supported_modifiers_sizeof());
        assert!(lib.has_xcb_dri3_get_supported_modifiers());
        assert!(lib.has_xcb_dri3_get_supported_modifiers_unchecked());
        assert!(lib.has_xcb_dri3_get_supported_modifiers_window_modifiers());
        assert!(lib.has_xcb_dri3_get_supported_modifiers_window_modifiers_length());
        assert!(lib.has_xcb_dri3_get_supported_modifiers_window_modifiers_end());
        assert!(lib.has_xcb_dri3_get_supported_modifiers_screen_modifiers());
        assert!(lib.has_xcb_dri3_get_supported_modifiers_screen_modifiers_length());
        assert!(lib.has_xcb_dri3_get_supported_modifiers_screen_modifiers_end());
        assert!(lib.has_xcb_dri3_get_supported_modifiers_reply());
        assert!(lib.has_xcb_dri3_pixmap_from_buffers_checked());
        assert!(lib.has_xcb_dri3_pixmap_from_buffers());
        assert!(lib.has_xcb_dri3_buffers_from_pixmap_sizeof());
        assert!(lib.has_xcb_dri3_buffers_from_pixmap());
        assert!(lib.has_xcb_dri3_buffers_from_pixmap_unchecked());
        assert!(lib.has_xcb_dri3_buffers_from_pixmap_strides());
        assert!(lib.has_xcb_dri3_buffers_from_pixmap_strides_length());
        assert!(lib.has_xcb_dri3_buffers_from_pixmap_strides_end());
        assert!(lib.has_xcb_dri3_buffers_from_pixmap_offsets());
        assert!(lib.has_xcb_dri3_buffers_from_pixmap_offsets_length());
        assert!(lib.has_xcb_dri3_buffers_from_pixmap_offsets_end());
        assert!(lib.has_xcb_dri3_buffers_from_pixmap_buffers());
        assert!(lib.has_xcb_dri3_buffers_from_pixmap_buffers_length());
        assert!(lib.has_xcb_dri3_buffers_from_pixmap_buffers_end());
        assert!(lib.has_xcb_dri3_buffers_from_pixmap_reply());

        /**
         * Return the reply fds
         * @param c      The connection
         * @param reply  The reply
         *
         * Returns the array of reply fds of the request asked by
         *
         * The returned value must be freed by the caller using free().
         */
        assert!(lib.has_xcb_dri3_buffers_from_pixmap_reply_fds());
    }
}
