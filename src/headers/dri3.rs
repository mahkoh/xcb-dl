// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The name of the `DRI3` extension.
pub const XCB_DRI3_NAME: &[u8] = b"DRI3";

/// The name of the `DRI3` extension.
pub const XCB_DRI3_NAME_STR: &str = "DRI3";

/// The cookie for the reply to a `DRI3::QueryVersion` request.
///
/// Pass this cookie to [`xcb_dri3_query_version_reply`] to retrieve the reply.
///
/// [`xcb_dri3_query_version_reply`]: XcbDri3::xcb_dri3_query_version_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_query_version_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_dri3_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DRI3::QueryVersion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDri3::xcb_dri3_id()`], then the type of the request is
/// [`xcb_dri3_query_version_request_t`].
pub const XCB_DRI3_QUERY_VERSION: u8 = 0i32 as u8;

/// The `DRI3::QueryVersion` request.
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

/// The `DRI3::QueryVersion` reply.
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

/// The cookie for the reply to a `DRI3::Open` request.
///
/// Pass this cookie to [`xcb_dri3_open_reply`] to retrieve the reply.
///
/// [`xcb_dri3_open_reply`]: XcbDri3::xcb_dri3_open_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_open_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_dri3_open_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DRI3::Open` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDri3::xcb_dri3_id()`], then the type of the request is
/// [`xcb_dri3_open_request_t`].
pub const XCB_DRI3_OPEN: u8 = 1i32 as u8;

/// The `DRI3::Open` request.
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

/// The `DRI3::Open` reply.
///
/// This reply contains file descriptors that can be accessed with [`xcb_dri3_open_reply_fds`].
///
/// [`xcb_dri3_open_reply_fds`]: XcbDri3::xcb_dri3_open_reply_fds
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

/// The opcode for `DRI3::PixmapFromBuffer` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDri3::xcb_dri3_id()`], then the type of the request is
/// [`xcb_dri3_pixmap_from_buffer_request_t`].
pub const XCB_DRI3_PIXMAP_FROM_BUFFER: u8 = 2i32 as u8;

/// The `DRI3::PixmapFromBuffer` request.
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

/// The cookie for the reply to a `DRI3::BufferFromPixmap` request.
///
/// Pass this cookie to [`xcb_dri3_buffer_from_pixmap_reply`] to retrieve the reply.
///
/// [`xcb_dri3_buffer_from_pixmap_reply`]: XcbDri3::xcb_dri3_buffer_from_pixmap_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_buffer_from_pixmap_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_dri3_buffer_from_pixmap_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DRI3::BufferFromPixmap` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDri3::xcb_dri3_id()`], then the type of the request is
/// [`xcb_dri3_buffer_from_pixmap_request_t`].
pub const XCB_DRI3_BUFFER_FROM_PIXMAP: u8 = 3i32 as u8;

/// The `DRI3::BufferFromPixmap` request.
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

/// The `DRI3::BufferFromPixmap` reply.
///
/// This reply contains file descriptors that can be accessed with [`xcb_dri3_buffer_from_pixmap_reply_fds`].
///
/// [`xcb_dri3_buffer_from_pixmap_reply_fds`]: XcbDri3::xcb_dri3_buffer_from_pixmap_reply_fds
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

/// The opcode for `DRI3::FenceFromFD` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDri3::xcb_dri3_id()`], then the type of the request is
/// [`xcb_dri3_fence_from_fd_request_t`].
pub const XCB_DRI3_FENCE_FROM_FD: u8 = 4i32 as u8;

/// The `DRI3::FenceFromFD` request.
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

/// The cookie for the reply to a `DRI3::FDFromFence` request.
///
/// Pass this cookie to [`xcb_dri3_fd_from_fence_reply`] to retrieve the reply.
///
/// [`xcb_dri3_fd_from_fence_reply`]: XcbDri3::xcb_dri3_fd_from_fence_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_fd_from_fence_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_dri3_fd_from_fence_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DRI3::FDFromFence` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDri3::xcb_dri3_id()`], then the type of the request is
/// [`xcb_dri3_fd_from_fence_request_t`].
pub const XCB_DRI3_FD_FROM_FENCE: u8 = 5i32 as u8;

/// The `DRI3::FDFromFence` request.
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

/// The `DRI3::FDFromFence` reply.
///
/// This reply contains file descriptors that can be accessed with [`xcb_dri3_fd_from_fence_reply_fds`].
///
/// [`xcb_dri3_fd_from_fence_reply_fds`]: XcbDri3::xcb_dri3_fd_from_fence_reply_fds
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

/// The cookie for the reply to a `DRI3::GetSupportedModifiers` request.
///
/// Pass this cookie to [`xcb_dri3_get_supported_modifiers_reply`] to retrieve the reply.
///
/// [`xcb_dri3_get_supported_modifiers_reply`]: XcbDri3::xcb_dri3_get_supported_modifiers_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_get_supported_modifiers_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_dri3_get_supported_modifiers_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DRI3::GetSupportedModifiers` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDri3::xcb_dri3_id()`], then the type of the request is
/// [`xcb_dri3_get_supported_modifiers_request_t`].
pub const XCB_DRI3_GET_SUPPORTED_MODIFIERS: u8 = 6i32 as u8;

/// The `DRI3::GetSupportedModifiers` request.
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

/// The `DRI3::GetSupportedModifiers` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `window_modifiers`
/// - `screen_modifiers`
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

/// The opcode for `DRI3::PixmapFromBuffers` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDri3::xcb_dri3_id()`], then the type of the request is
/// [`xcb_dri3_pixmap_from_buffers_request_t`].
pub const XCB_DRI3_PIXMAP_FROM_BUFFERS: u8 = 7i32 as u8;

/// The `DRI3::PixmapFromBuffers` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `buffers`
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

/// The cookie for the reply to a `DRI3::BuffersFromPixmap` request.
///
/// Pass this cookie to [`xcb_dri3_buffers_from_pixmap_reply`] to retrieve the reply.
///
/// [`xcb_dri3_buffers_from_pixmap_reply`]: XcbDri3::xcb_dri3_buffers_from_pixmap_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_buffers_from_pixmap_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_dri3_buffers_from_pixmap_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DRI3::BuffersFromPixmap` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDri3::xcb_dri3_id()`], then the type of the request is
/// [`xcb_dri3_buffers_from_pixmap_request_t`].
pub const XCB_DRI3_BUFFERS_FROM_PIXMAP: u8 = 8i32 as u8;

/// The `DRI3::BuffersFromPixmap` request.
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

/// The `DRI3::BuffersFromPixmap` reply.
///
/// This reply contains file descriptors that can be accessed with [`xcb_dri3_buffers_from_pixmap_reply_fds`].
///
/// [`xcb_dri3_buffers_from_pixmap_reply_fds`]: XcbDri3::xcb_dri3_buffers_from_pixmap_reply_fds
///
/// The following fields can be accessed via accessor functions:
///
/// - `strides`
/// - `offsets`
/// - `buffers`
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

#[cfg(feature = "xcb_dri3")]
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
        ) -> *mut xcb_dri3_query_version_reply_t,
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
        ) -> *mut xcb_dri3_open_reply_t,
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
        ) -> *mut xcb_dri3_buffer_from_pixmap_reply_t,
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
        ) -> *mut xcb_dri3_fd_from_fence_reply_t,
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
        ) -> *mut xcb_dri3_get_supported_modifiers_reply_t,
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
        ) -> *mut xcb_dri3_buffers_from_pixmap_reply_t,
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

#[cfg(feature = "xcb_dri3")]
impl XcbDri3 {
    /// The libxcb identifier of the `DRI3` extension.
    #[inline]
    pub fn xcb_dri3_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_dri3_id) }
    }

    /// Returns `true` iff the symbol `xcb_dri3_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_id(&self) -> bool {
        has_sym!(self, xcb_dri3_id)
    }

    /// Sends a `DRI3::QueryVersion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri3_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri3_query_version_reply`]: Self::xcb_dri3_query_version_reply
    #[inline]
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

    /// Sends a `DRI3::QueryVersion` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri3_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri3_query_version_reply`]: Self::xcb_dri3_query_version_reply
    #[inline]
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

    /// Waits for the reply to a `DRI3::QueryVersion` request.
    #[inline]
    pub unsafe fn xcb_dri3_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri3_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri3_query_version_reply_t {
        sym!(self, xcb_dri3_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dri3_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_dri3_query_version_reply)
    }

    /// Sends a `DRI3::Open` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri3_open_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri3_open_reply`]: Self::xcb_dri3_open_reply
    #[inline]
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

    /// Sends a `DRI3::Open` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri3_open_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri3_open_reply`]: Self::xcb_dri3_open_reply
    #[inline]
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

    /// Waits for the reply to a `DRI3::Open` request.
    #[inline]
    pub unsafe fn xcb_dri3_open_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri3_open_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri3_open_reply_t {
        sym!(self, xcb_dri3_open_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dri3_open_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_open_reply(&self) -> bool {
        has_sym!(self, xcb_dri3_open_reply)
    }

    /// Retrieves the file descriptors from the reply to a `DRI3::Open` request.
    ///
    /// The returned pointer must be freed with `libc::free`.
    #[inline]
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

    /// Sends a `DRI3::PixmapFromBuffer` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
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

    /// Sends a `DRI3::PixmapFromBuffer` request (unchecked).
    #[inline]
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

    /// Sends a `DRI3::BufferFromPixmap` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri3_buffer_from_pixmap_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri3_buffer_from_pixmap_reply`]: Self::xcb_dri3_buffer_from_pixmap_reply
    #[inline]
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

    /// Sends a `DRI3::BufferFromPixmap` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri3_buffer_from_pixmap_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri3_buffer_from_pixmap_reply`]: Self::xcb_dri3_buffer_from_pixmap_reply
    #[inline]
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

    /// Waits for the reply to a `DRI3::BufferFromPixmap` request.
    #[inline]
    pub unsafe fn xcb_dri3_buffer_from_pixmap_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri3_buffer_from_pixmap_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri3_buffer_from_pixmap_reply_t {
        sym!(self, xcb_dri3_buffer_from_pixmap_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dri3_buffer_from_pixmap_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_buffer_from_pixmap_reply(&self) -> bool {
        has_sym!(self, xcb_dri3_buffer_from_pixmap_reply)
    }

    /// Retrieves the file descriptors from the reply to a `DRI3::BufferFromPixmap` request.
    ///
    /// The returned pointer must be freed with `libc::free`.
    #[inline]
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

    /// Sends a `DRI3::FenceFromFD` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
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

    /// Sends a `DRI3::FenceFromFD` request (unchecked).
    #[inline]
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

    /// Sends a `DRI3::FDFromFence` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri3_fd_from_fence_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri3_fd_from_fence_reply`]: Self::xcb_dri3_fd_from_fence_reply
    #[inline]
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

    /// Sends a `DRI3::FDFromFence` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri3_fd_from_fence_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri3_fd_from_fence_reply`]: Self::xcb_dri3_fd_from_fence_reply
    #[inline]
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

    /// Waits for the reply to a `DRI3::FDFromFence` request.
    #[inline]
    pub unsafe fn xcb_dri3_fd_from_fence_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri3_fd_from_fence_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri3_fd_from_fence_reply_t {
        sym!(self, xcb_dri3_fd_from_fence_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dri3_fd_from_fence_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_fd_from_fence_reply(&self) -> bool {
        has_sym!(self, xcb_dri3_fd_from_fence_reply)
    }

    /// Retrieves the file descriptors from the reply to a `DRI3::FDFromFence` request.
    ///
    /// The returned pointer must be freed with `libc::free`.
    #[inline]
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

    /// Computes the size of a `xcb_dri3_get_supported_modifiers_reply_t` object.
    #[inline]
    pub unsafe fn xcb_dri3_get_supported_modifiers_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_dri3_get_supported_modifiers_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_dri3_get_supported_modifiers_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_get_supported_modifiers_sizeof(&self) -> bool {
        has_sym!(self, xcb_dri3_get_supported_modifiers_sizeof)
    }

    /// Sends a `DRI3::GetSupportedModifiers` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri3_get_supported_modifiers_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri3_get_supported_modifiers_reply`]: Self::xcb_dri3_get_supported_modifiers_reply
    #[inline]
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

    /// Sends a `DRI3::GetSupportedModifiers` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri3_get_supported_modifiers_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri3_get_supported_modifiers_reply`]: Self::xcb_dri3_get_supported_modifiers_reply
    #[inline]
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

    /// Returns a pointer to the `window_modifiers` field of a `xcb_dri3_get_supported_modifiers_reply_t` struct.
    #[inline]
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

    /// Returns the number of elements of the `window_modifiers` field of a `xcb_dri3_get_supported_modifiers_reply_t` struct.
    #[inline]
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

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `window_modifiers` field of a `xcb_dri3_get_supported_modifiers_reply_t` struct.
    #[inline]
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

    /// Returns a pointer to the `screen_modifiers` field of a `xcb_dri3_get_supported_modifiers_reply_t` struct.
    #[inline]
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

    /// Returns the number of elements of the `screen_modifiers` field of a `xcb_dri3_get_supported_modifiers_reply_t` struct.
    #[inline]
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

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `screen_modifiers` field of a `xcb_dri3_get_supported_modifiers_reply_t` struct.
    #[inline]
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

    /// Waits for the reply to a `DRI3::GetSupportedModifiers` request.
    #[inline]
    pub unsafe fn xcb_dri3_get_supported_modifiers_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri3_get_supported_modifiers_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri3_get_supported_modifiers_reply_t {
        sym!(self, xcb_dri3_get_supported_modifiers_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dri3_get_supported_modifiers_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_get_supported_modifiers_reply(&self) -> bool {
        has_sym!(self, xcb_dri3_get_supported_modifiers_reply)
    }

    /// Sends a `DRI3::PixmapFromBuffers` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
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

    /// Sends a `DRI3::PixmapFromBuffers` request (unchecked).
    #[inline]
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

    /// Computes the size of a `xcb_dri3_buffers_from_pixmap_reply_t` object.
    #[inline]
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

    /// Sends a `DRI3::BuffersFromPixmap` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri3_buffers_from_pixmap_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri3_buffers_from_pixmap_reply`]: Self::xcb_dri3_buffers_from_pixmap_reply
    #[inline]
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

    /// Sends a `DRI3::BuffersFromPixmap` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri3_buffers_from_pixmap_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri3_buffers_from_pixmap_reply`]: Self::xcb_dri3_buffers_from_pixmap_reply
    #[inline]
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

    /// Returns a pointer to the `strides` field of a `xcb_dri3_buffers_from_pixmap_reply_t` struct.
    #[inline]
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

    /// Returns the number of elements of the `strides` field of a `xcb_dri3_buffers_from_pixmap_reply_t` struct.
    #[inline]
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

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `strides` field of a `xcb_dri3_buffers_from_pixmap_reply_t` struct.
    #[inline]
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

    /// Returns a pointer to the `offsets` field of a `xcb_dri3_buffers_from_pixmap_reply_t` struct.
    #[inline]
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

    /// Returns the number of elements of the `offsets` field of a `xcb_dri3_buffers_from_pixmap_reply_t` struct.
    #[inline]
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

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `offsets` field of a `xcb_dri3_buffers_from_pixmap_reply_t` struct.
    #[inline]
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

    /// Returns a pointer to the `buffers` field of a `xcb_dri3_buffers_from_pixmap_reply_t` struct.
    #[inline]
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

    /// Returns the number of elements of the `buffers` field of a `xcb_dri3_buffers_from_pixmap_reply_t` struct.
    #[inline]
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

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `buffers` field of a `xcb_dri3_buffers_from_pixmap_reply_t` struct.
    #[inline]
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

    /// Waits for the reply to a `DRI3::BuffersFromPixmap` request.
    #[inline]
    pub unsafe fn xcb_dri3_buffers_from_pixmap_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri3_buffers_from_pixmap_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri3_buffers_from_pixmap_reply_t {
        sym!(self, xcb_dri3_buffers_from_pixmap_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dri3_buffers_from_pixmap_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri3_buffers_from_pixmap_reply(&self) -> bool {
        has_sym!(self, xcb_dri3_buffers_from_pixmap_reply)
    }

    /// Retrieves the file descriptors from the reply to a `DRI3::BuffersFromPixmap` request.
    ///
    /// The returned pointer must be freed with `libc::free`.
    #[inline]
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

#[cfg(feature = "xcb_dri3")]
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
        assert!(lib.has_xcb_dri3_open_reply_fds());
        assert!(lib.has_xcb_dri3_pixmap_from_buffer_checked());
        assert!(lib.has_xcb_dri3_pixmap_from_buffer());
        assert!(lib.has_xcb_dri3_buffer_from_pixmap());
        assert!(lib.has_xcb_dri3_buffer_from_pixmap_unchecked());
        assert!(lib.has_xcb_dri3_buffer_from_pixmap_reply());
        assert!(lib.has_xcb_dri3_buffer_from_pixmap_reply_fds());
        assert!(lib.has_xcb_dri3_fence_from_fd_checked());
        assert!(lib.has_xcb_dri3_fence_from_fd());
        assert!(lib.has_xcb_dri3_fd_from_fence());
        assert!(lib.has_xcb_dri3_fd_from_fence_unchecked());
        assert!(lib.has_xcb_dri3_fd_from_fence_reply());
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
        assert!(lib.has_xcb_dri3_buffers_from_pixmap_reply_fds());
    }
}
