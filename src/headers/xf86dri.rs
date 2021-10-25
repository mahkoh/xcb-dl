// This file was generated using generate.py. Do not edit.

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_drm_clip_rect_t {
    pub x1: i16,
    pub y1: i16,
    pub x2: i16,
    pub x3: i16,
}

impl Default for xcb_xf86dri_drm_clip_rect_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_drm_clip_rect_iterator_t {
    pub data: *mut xcb_xf86dri_drm_clip_rect_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xf86dri_drm_clip_rect_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_query_version_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xf86dri_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xf86dri_query_version.
pub const XCB_XF86DRI_QUERY_VERSION: u8 = 0i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_xf86dri_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub dri_major_version: u16,
    pub dri_minor_version: u16,
    pub dri_minor_patch: u32,
}

impl Default for xcb_xf86dri_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_query_direct_rendering_capable_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xf86dri_query_direct_rendering_capable_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xf86dri_query_direct_rendering_capable.
pub const XCB_XF86DRI_QUERY_DIRECT_RENDERING_CAPABLE: u8 = 1i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_query_direct_rendering_capable_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
}

impl Default for xcb_xf86dri_query_direct_rendering_capable_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_query_direct_rendering_capable_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub is_capable: u8,
}

impl Default for xcb_xf86dri_query_direct_rendering_capable_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_open_connection_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xf86dri_open_connection_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xf86dri_open_connection.
pub const XCB_XF86DRI_OPEN_CONNECTION: u8 = 2i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_open_connection_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
}

impl Default for xcb_xf86dri_open_connection_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_open_connection_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub sarea_handle_low: u32,
    pub sarea_handle_high: u32,
    pub bus_id_len: u32,
    pub pad1: [u8; 12],
}

impl Default for xcb_xf86dri_open_connection_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xf86dri_close_connection.
pub const XCB_XF86DRI_CLOSE_CONNECTION: u8 = 3i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_close_connection_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
}

impl Default for xcb_xf86dri_close_connection_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_get_client_driver_name_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xf86dri_get_client_driver_name_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xf86dri_get_client_driver_name.
pub const XCB_XF86DRI_GET_CLIENT_DRIVER_NAME: u8 = 4i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_get_client_driver_name_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
}

impl Default for xcb_xf86dri_get_client_driver_name_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_get_client_driver_name_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub client_driver_major_version: u32,
    pub client_driver_minor_version: u32,
    pub client_driver_patch_version: u32,
    pub client_driver_name_len: u32,
    pub pad1: [u8; 8],
}

impl Default for xcb_xf86dri_get_client_driver_name_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_create_context_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xf86dri_create_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xf86dri_create_context.
pub const XCB_XF86DRI_CREATE_CONTEXT: u8 = 5i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_create_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
    pub visual: u32,
    pub context: u32,
}

impl Default for xcb_xf86dri_create_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_create_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub hw_context: u32,
}

impl Default for xcb_xf86dri_create_context_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xf86dri_destroy_context.
pub const XCB_XF86DRI_DESTROY_CONTEXT: u8 = 6i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_destroy_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
    pub context: u32,
}

impl Default for xcb_xf86dri_destroy_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_create_drawable_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xf86dri_create_drawable_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xf86dri_create_drawable.
pub const XCB_XF86DRI_CREATE_DRAWABLE: u8 = 7i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_create_drawable_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
    pub drawable: u32,
}

impl Default for xcb_xf86dri_create_drawable_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_create_drawable_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub hw_drawable_handle: u32,
}

impl Default for xcb_xf86dri_create_drawable_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xf86dri_destroy_drawable.
pub const XCB_XF86DRI_DESTROY_DRAWABLE: u8 = 8i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_destroy_drawable_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
    pub drawable: u32,
}

impl Default for xcb_xf86dri_destroy_drawable_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_get_drawable_info_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xf86dri_get_drawable_info_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xf86dri_get_drawable_info.
pub const XCB_XF86DRI_GET_DRAWABLE_INFO: u8 = 9i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_get_drawable_info_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
    pub drawable: u32,
}

impl Default for xcb_xf86dri_get_drawable_info_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_get_drawable_info_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub drawable_table_index: u32,
    pub drawable_table_stamp: u32,
    pub drawable_origin__x: i16,
    pub drawable_origin__y: i16,
    pub drawable_size__w: i16,
    pub drawable_size__h: i16,
    pub num_clip_rects: u32,
    pub back_x: i16,
    pub back_y: i16,
    pub num_back_clip_rects: u32,
}

impl Default for xcb_xf86dri_get_drawable_info_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_get_device_info_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xf86dri_get_device_info_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xf86dri_get_device_info.
pub const XCB_XF86DRI_GET_DEVICE_INFO: u8 = 10i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_get_device_info_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
}

impl Default for xcb_xf86dri_get_device_info_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_get_device_info_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub framebuffer_handle_low: u32,
    pub framebuffer_handle_high: u32,
    pub framebuffer_origin_offset: u32,
    pub framebuffer_size: u32,
    pub framebuffer_stride: u32,
    pub device_private_size: u32,
}

impl Default for xcb_xf86dri_get_device_info_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_auth_connection_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xf86dri_auth_connection_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xf86dri_auth_connection.
pub const XCB_XF86DRI_AUTH_CONNECTION: u8 = 11i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_auth_connection_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
    pub magic: u32,
}

impl Default for xcb_xf86dri_auth_connection_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_auth_connection_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub authenticated: u32,
}

impl Default for xcb_xf86dri_auth_connection_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub(crate) struct XcbXf86driXf86Dri {
    xcb_xf86dri_id: LazySymbol<*mut xcb_extension_t>,
    xcb_xf86dri_drm_clip_rect_next:
        LazySymbol<unsafe fn(i: *mut xcb_xf86dri_drm_clip_rect_iterator_t)>,
    xcb_xf86dri_drm_clip_rect_end:
        LazySymbol<unsafe fn(i: xcb_xf86dri_drm_clip_rect_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xf86dri_query_version:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xf86dri_query_version_cookie_t>,
    xcb_xf86dri_query_version_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xf86dri_query_version_cookie_t>,
    xcb_xf86dri_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xf86dri_query_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_xf86dri_query_version_reply_t,
    >,
    xcb_xf86dri_query_direct_rendering_capable: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
        ) -> xcb_xf86dri_query_direct_rendering_capable_cookie_t,
    >,
    xcb_xf86dri_query_direct_rendering_capable_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
        ) -> xcb_xf86dri_query_direct_rendering_capable_cookie_t,
    >,
    xcb_xf86dri_query_direct_rendering_capable_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xf86dri_query_direct_rendering_capable_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_xf86dri_query_direct_rendering_capable_reply_t,
    >,
    xcb_xf86dri_open_connection_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xf86dri_open_connection: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, screen: u32) -> xcb_xf86dri_open_connection_cookie_t,
    >,
    xcb_xf86dri_open_connection_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, screen: u32) -> xcb_xf86dri_open_connection_cookie_t,
    >,
    xcb_xf86dri_open_connection_bus_id:
        LazySymbol<unsafe fn(r: *const xcb_xf86dri_open_connection_reply_t) -> *mut c_char>,
    xcb_xf86dri_open_connection_bus_id_length:
        LazySymbol<unsafe fn(r: *const xcb_xf86dri_open_connection_reply_t) -> c_int>,
    xcb_xf86dri_open_connection_bus_id_end: LazySymbol<
        unsafe fn(r: *const xcb_xf86dri_open_connection_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_xf86dri_open_connection_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xf86dri_open_connection_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_xf86dri_open_connection_reply_t,
    >,
    xcb_xf86dri_close_connection_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, screen: u32) -> xcb_void_cookie_t>,
    xcb_xf86dri_close_connection:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, screen: u32) -> xcb_void_cookie_t>,
    xcb_xf86dri_get_client_driver_name_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xf86dri_get_client_driver_name: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
        ) -> xcb_xf86dri_get_client_driver_name_cookie_t,
    >,
    xcb_xf86dri_get_client_driver_name_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
        ) -> xcb_xf86dri_get_client_driver_name_cookie_t,
    >,
    xcb_xf86dri_get_client_driver_name_client_driver_name:
        LazySymbol<unsafe fn(r: *const xcb_xf86dri_get_client_driver_name_reply_t) -> *mut c_char>,
    xcb_xf86dri_get_client_driver_name_client_driver_name_length:
        LazySymbol<unsafe fn(r: *const xcb_xf86dri_get_client_driver_name_reply_t) -> c_int>,
    xcb_xf86dri_get_client_driver_name_client_driver_name_end: LazySymbol<
        unsafe fn(r: *const xcb_xf86dri_get_client_driver_name_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_xf86dri_get_client_driver_name_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xf86dri_get_client_driver_name_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_xf86dri_get_client_driver_name_reply_t,
    >,
    xcb_xf86dri_create_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            visual: u32,
            context: u32,
        ) -> xcb_xf86dri_create_context_cookie_t,
    >,
    xcb_xf86dri_create_context_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            visual: u32,
            context: u32,
        ) -> xcb_xf86dri_create_context_cookie_t,
    >,
    xcb_xf86dri_create_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xf86dri_create_context_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_xf86dri_create_context_reply_t,
    >,
    xcb_xf86dri_destroy_context_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, screen: u32, context: u32) -> xcb_void_cookie_t,
    >,
    xcb_xf86dri_destroy_context: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, screen: u32, context: u32) -> xcb_void_cookie_t,
    >,
    xcb_xf86dri_create_drawable: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            drawable: u32,
        ) -> xcb_xf86dri_create_drawable_cookie_t,
    >,
    xcb_xf86dri_create_drawable_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            drawable: u32,
        ) -> xcb_xf86dri_create_drawable_cookie_t,
    >,
    xcb_xf86dri_create_drawable_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xf86dri_create_drawable_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_xf86dri_create_drawable_reply_t,
    >,
    xcb_xf86dri_destroy_drawable_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, screen: u32, drawable: u32) -> xcb_void_cookie_t,
    >,
    xcb_xf86dri_destroy_drawable: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, screen: u32, drawable: u32) -> xcb_void_cookie_t,
    >,
    xcb_xf86dri_get_drawable_info_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xf86dri_get_drawable_info: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            drawable: u32,
        ) -> xcb_xf86dri_get_drawable_info_cookie_t,
    >,
    xcb_xf86dri_get_drawable_info_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            drawable: u32,
        ) -> xcb_xf86dri_get_drawable_info_cookie_t,
    >,
    xcb_xf86dri_get_drawable_info_clip_rects: LazySymbol<
        unsafe fn(
            r: *const xcb_xf86dri_get_drawable_info_reply_t,
        ) -> *mut xcb_xf86dri_drm_clip_rect_t,
    >,
    xcb_xf86dri_get_drawable_info_clip_rects_length:
        LazySymbol<unsafe fn(r: *const xcb_xf86dri_get_drawable_info_reply_t) -> c_int>,
    xcb_xf86dri_get_drawable_info_clip_rects_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xf86dri_get_drawable_info_reply_t,
        ) -> xcb_xf86dri_drm_clip_rect_iterator_t,
    >,
    xcb_xf86dri_get_drawable_info_back_clip_rects: LazySymbol<
        unsafe fn(
            r: *const xcb_xf86dri_get_drawable_info_reply_t,
        ) -> *mut xcb_xf86dri_drm_clip_rect_t,
    >,
    xcb_xf86dri_get_drawable_info_back_clip_rects_length:
        LazySymbol<unsafe fn(r: *const xcb_xf86dri_get_drawable_info_reply_t) -> c_int>,
    xcb_xf86dri_get_drawable_info_back_clip_rects_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xf86dri_get_drawable_info_reply_t,
        ) -> xcb_xf86dri_drm_clip_rect_iterator_t,
    >,
    xcb_xf86dri_get_drawable_info_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xf86dri_get_drawable_info_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_xf86dri_get_drawable_info_reply_t,
    >,
    xcb_xf86dri_get_device_info_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xf86dri_get_device_info: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, screen: u32) -> xcb_xf86dri_get_device_info_cookie_t,
    >,
    xcb_xf86dri_get_device_info_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, screen: u32) -> xcb_xf86dri_get_device_info_cookie_t,
    >,
    xcb_xf86dri_get_device_info_device_private:
        LazySymbol<unsafe fn(r: *const xcb_xf86dri_get_device_info_reply_t) -> *mut u32>,
    xcb_xf86dri_get_device_info_device_private_length:
        LazySymbol<unsafe fn(r: *const xcb_xf86dri_get_device_info_reply_t) -> c_int>,
    xcb_xf86dri_get_device_info_device_private_end: LazySymbol<
        unsafe fn(r: *const xcb_xf86dri_get_device_info_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_xf86dri_get_device_info_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xf86dri_get_device_info_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_xf86dri_get_device_info_reply_t,
    >,
    xcb_xf86dri_auth_connection: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            magic: u32,
        ) -> xcb_xf86dri_auth_connection_cookie_t,
    >,
    xcb_xf86dri_auth_connection_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            magic: u32,
        ) -> xcb_xf86dri_auth_connection_cookie_t,
    >,
    xcb_xf86dri_auth_connection_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xf86dri_auth_connection_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_xf86dri_auth_connection_reply_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self
            .xf86dri
            .$f
            .get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .xf86dri
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

impl XcbXf86dri {
    pub fn xcb_xf86dri_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_xf86dri_id) }
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_id(&self) -> bool {
        has_sym!(self, xcb_xf86dri_id)
    }

    pub unsafe fn xcb_xf86dri_drm_clip_rect_next(
        &self,
        i: *mut xcb_xf86dri_drm_clip_rect_iterator_t,
    ) {
        sym!(self, xcb_xf86dri_drm_clip_rect_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_drm_clip_rect_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_drm_clip_rect_next(&self) -> bool {
        has_sym!(self, xcb_xf86dri_drm_clip_rect_next)
    }

    pub unsafe fn xcb_xf86dri_drm_clip_rect_end(
        &self,
        i: xcb_xf86dri_drm_clip_rect_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xf86dri_drm_clip_rect_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_drm_clip_rect_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_drm_clip_rect_end(&self) -> bool {
        has_sym!(self, xcb_xf86dri_drm_clip_rect_end)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xf86dri_query_version(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xf86dri_query_version_cookie_t {
        sym!(self, xcb_xf86dri_query_version)(c)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_query_version(&self) -> bool {
        has_sym!(self, xcb_xf86dri_query_version)
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
    pub unsafe fn xcb_xf86dri_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xf86dri_query_version_cookie_t {
        sym!(self, xcb_xf86dri_query_version_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_query_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_xf86dri_query_version_unchecked)
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
     * xcb_xf86dri_query_version_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xf86dri_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xf86dri_query_version_reply_t {
        sym!(self, xcb_xf86dri_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_xf86dri_query_version_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xf86dri_query_direct_rendering_capable(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_xf86dri_query_direct_rendering_capable_cookie_t {
        sym!(self, xcb_xf86dri_query_direct_rendering_capable)(c, screen)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_query_direct_rendering_capable` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_query_direct_rendering_capable(&self) -> bool {
        has_sym!(self, xcb_xf86dri_query_direct_rendering_capable)
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
    pub unsafe fn xcb_xf86dri_query_direct_rendering_capable_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_xf86dri_query_direct_rendering_capable_cookie_t {
        sym!(self, xcb_xf86dri_query_direct_rendering_capable_unchecked)(c, screen)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_query_direct_rendering_capable_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_query_direct_rendering_capable_unchecked(&self) -> bool {
        has_sym!(self, xcb_xf86dri_query_direct_rendering_capable_unchecked)
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
     * xcb_xf86dri_query_direct_rendering_capable_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xf86dri_query_direct_rendering_capable_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_query_direct_rendering_capable_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xf86dri_query_direct_rendering_capable_reply_t {
        sym!(self, xcb_xf86dri_query_direct_rendering_capable_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_query_direct_rendering_capable_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_query_direct_rendering_capable_reply(&self) -> bool {
        has_sym!(self, xcb_xf86dri_query_direct_rendering_capable_reply)
    }

    pub unsafe fn xcb_xf86dri_open_connection_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xf86dri_open_connection_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_open_connection_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_open_connection_sizeof(&self) -> bool {
        has_sym!(self, xcb_xf86dri_open_connection_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xf86dri_open_connection(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_xf86dri_open_connection_cookie_t {
        sym!(self, xcb_xf86dri_open_connection)(c, screen)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_open_connection` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_open_connection(&self) -> bool {
        has_sym!(self, xcb_xf86dri_open_connection)
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
    pub unsafe fn xcb_xf86dri_open_connection_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_xf86dri_open_connection_cookie_t {
        sym!(self, xcb_xf86dri_open_connection_unchecked)(c, screen)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_open_connection_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_open_connection_unchecked(&self) -> bool {
        has_sym!(self, xcb_xf86dri_open_connection_unchecked)
    }

    pub unsafe fn xcb_xf86dri_open_connection_bus_id(
        &self,
        r: *const xcb_xf86dri_open_connection_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_xf86dri_open_connection_bus_id)(r)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_open_connection_bus_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_open_connection_bus_id(&self) -> bool {
        has_sym!(self, xcb_xf86dri_open_connection_bus_id)
    }

    pub unsafe fn xcb_xf86dri_open_connection_bus_id_length(
        &self,
        r: *const xcb_xf86dri_open_connection_reply_t,
    ) -> c_int {
        sym!(self, xcb_xf86dri_open_connection_bus_id_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_open_connection_bus_id_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_open_connection_bus_id_length(&self) -> bool {
        has_sym!(self, xcb_xf86dri_open_connection_bus_id_length)
    }

    pub unsafe fn xcb_xf86dri_open_connection_bus_id_end(
        &self,
        r: *const xcb_xf86dri_open_connection_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xf86dri_open_connection_bus_id_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_open_connection_bus_id_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_open_connection_bus_id_end(&self) -> bool {
        has_sym!(self, xcb_xf86dri_open_connection_bus_id_end)
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
     * xcb_xf86dri_open_connection_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xf86dri_open_connection_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_open_connection_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xf86dri_open_connection_reply_t {
        sym!(self, xcb_xf86dri_open_connection_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_open_connection_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_open_connection_reply(&self) -> bool {
        has_sym!(self, xcb_xf86dri_open_connection_reply)
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
    pub unsafe fn xcb_xf86dri_close_connection_checked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xf86dri_close_connection_checked)(c, screen)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_close_connection_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_close_connection_checked(&self) -> bool {
        has_sym!(self, xcb_xf86dri_close_connection_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xf86dri_close_connection(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xf86dri_close_connection)(c, screen)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_close_connection` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_close_connection(&self) -> bool {
        has_sym!(self, xcb_xf86dri_close_connection)
    }

    pub unsafe fn xcb_xf86dri_get_client_driver_name_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_xf86dri_get_client_driver_name_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_client_driver_name_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_client_driver_name_sizeof(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_client_driver_name_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xf86dri_get_client_driver_name(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_xf86dri_get_client_driver_name_cookie_t {
        sym!(self, xcb_xf86dri_get_client_driver_name)(c, screen)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_client_driver_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_client_driver_name(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_client_driver_name)
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
    pub unsafe fn xcb_xf86dri_get_client_driver_name_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_xf86dri_get_client_driver_name_cookie_t {
        sym!(self, xcb_xf86dri_get_client_driver_name_unchecked)(c, screen)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_client_driver_name_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_client_driver_name_unchecked(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_client_driver_name_unchecked)
    }

    pub unsafe fn xcb_xf86dri_get_client_driver_name_client_driver_name(
        &self,
        r: *const xcb_xf86dri_get_client_driver_name_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_xf86dri_get_client_driver_name_client_driver_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_client_driver_name_client_driver_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_client_driver_name_client_driver_name(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_client_driver_name_client_driver_name)
    }

    pub unsafe fn xcb_xf86dri_get_client_driver_name_client_driver_name_length(
        &self,
        r: *const xcb_xf86dri_get_client_driver_name_reply_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xf86dri_get_client_driver_name_client_driver_name_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_client_driver_name_client_driver_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_client_driver_name_client_driver_name_length(&self) -> bool {
        has_sym!(
            self,
            xcb_xf86dri_get_client_driver_name_client_driver_name_length
        )
    }

    pub unsafe fn xcb_xf86dri_get_client_driver_name_client_driver_name_end(
        &self,
        r: *const xcb_xf86dri_get_client_driver_name_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_xf86dri_get_client_driver_name_client_driver_name_end
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_client_driver_name_client_driver_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_client_driver_name_client_driver_name_end(&self) -> bool {
        has_sym!(
            self,
            xcb_xf86dri_get_client_driver_name_client_driver_name_end
        )
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
     * xcb_xf86dri_get_client_driver_name_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xf86dri_get_client_driver_name_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_get_client_driver_name_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xf86dri_get_client_driver_name_reply_t {
        sym!(self, xcb_xf86dri_get_client_driver_name_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_client_driver_name_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_client_driver_name_reply(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_client_driver_name_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xf86dri_create_context(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        visual: u32,
        context: u32,
    ) -> xcb_xf86dri_create_context_cookie_t {
        sym!(self, xcb_xf86dri_create_context)(c, screen, visual, context)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_create_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_create_context(&self) -> bool {
        has_sym!(self, xcb_xf86dri_create_context)
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
    pub unsafe fn xcb_xf86dri_create_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        visual: u32,
        context: u32,
    ) -> xcb_xf86dri_create_context_cookie_t {
        sym!(self, xcb_xf86dri_create_context_unchecked)(c, screen, visual, context)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_create_context_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_create_context_unchecked(&self) -> bool {
        has_sym!(self, xcb_xf86dri_create_context_unchecked)
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
     * xcb_xf86dri_create_context_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xf86dri_create_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_create_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xf86dri_create_context_reply_t {
        sym!(self, xcb_xf86dri_create_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_create_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_create_context_reply(&self) -> bool {
        has_sym!(self, xcb_xf86dri_create_context_reply)
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
    pub unsafe fn xcb_xf86dri_destroy_context_checked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        context: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xf86dri_destroy_context_checked)(c, screen, context)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_destroy_context_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_destroy_context_checked(&self) -> bool {
        has_sym!(self, xcb_xf86dri_destroy_context_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xf86dri_destroy_context(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        context: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xf86dri_destroy_context)(c, screen, context)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_destroy_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_destroy_context(&self) -> bool {
        has_sym!(self, xcb_xf86dri_destroy_context)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xf86dri_create_drawable(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        drawable: u32,
    ) -> xcb_xf86dri_create_drawable_cookie_t {
        sym!(self, xcb_xf86dri_create_drawable)(c, screen, drawable)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_create_drawable` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_create_drawable(&self) -> bool {
        has_sym!(self, xcb_xf86dri_create_drawable)
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
    pub unsafe fn xcb_xf86dri_create_drawable_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        drawable: u32,
    ) -> xcb_xf86dri_create_drawable_cookie_t {
        sym!(self, xcb_xf86dri_create_drawable_unchecked)(c, screen, drawable)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_create_drawable_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_create_drawable_unchecked(&self) -> bool {
        has_sym!(self, xcb_xf86dri_create_drawable_unchecked)
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
     * xcb_xf86dri_create_drawable_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xf86dri_create_drawable_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_create_drawable_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xf86dri_create_drawable_reply_t {
        sym!(self, xcb_xf86dri_create_drawable_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_create_drawable_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_create_drawable_reply(&self) -> bool {
        has_sym!(self, xcb_xf86dri_create_drawable_reply)
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
    pub unsafe fn xcb_xf86dri_destroy_drawable_checked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        drawable: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xf86dri_destroy_drawable_checked)(c, screen, drawable)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_destroy_drawable_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_destroy_drawable_checked(&self) -> bool {
        has_sym!(self, xcb_xf86dri_destroy_drawable_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xf86dri_destroy_drawable(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        drawable: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xf86dri_destroy_drawable)(c, screen, drawable)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_destroy_drawable` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_destroy_drawable(&self) -> bool {
        has_sym!(self, xcb_xf86dri_destroy_drawable)
    }

    pub unsafe fn xcb_xf86dri_get_drawable_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xf86dri_get_drawable_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_drawable_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_drawable_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_drawable_info_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xf86dri_get_drawable_info(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        drawable: u32,
    ) -> xcb_xf86dri_get_drawable_info_cookie_t {
        sym!(self, xcb_xf86dri_get_drawable_info)(c, screen, drawable)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_drawable_info` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_drawable_info(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_drawable_info)
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
    pub unsafe fn xcb_xf86dri_get_drawable_info_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        drawable: u32,
    ) -> xcb_xf86dri_get_drawable_info_cookie_t {
        sym!(self, xcb_xf86dri_get_drawable_info_unchecked)(c, screen, drawable)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_drawable_info_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_drawable_info_unchecked(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_drawable_info_unchecked)
    }

    pub unsafe fn xcb_xf86dri_get_drawable_info_clip_rects(
        &self,
        r: *const xcb_xf86dri_get_drawable_info_reply_t,
    ) -> *mut xcb_xf86dri_drm_clip_rect_t {
        sym!(self, xcb_xf86dri_get_drawable_info_clip_rects)(r)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_drawable_info_clip_rects` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_drawable_info_clip_rects(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_drawable_info_clip_rects)
    }

    pub unsafe fn xcb_xf86dri_get_drawable_info_clip_rects_length(
        &self,
        r: *const xcb_xf86dri_get_drawable_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_xf86dri_get_drawable_info_clip_rects_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_drawable_info_clip_rects_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_drawable_info_clip_rects_length(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_drawable_info_clip_rects_length)
    }

    pub unsafe fn xcb_xf86dri_get_drawable_info_clip_rects_iterator(
        &self,
        r: *const xcb_xf86dri_get_drawable_info_reply_t,
    ) -> xcb_xf86dri_drm_clip_rect_iterator_t {
        sym!(self, xcb_xf86dri_get_drawable_info_clip_rects_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_drawable_info_clip_rects_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_drawable_info_clip_rects_iterator(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_drawable_info_clip_rects_iterator)
    }

    pub unsafe fn xcb_xf86dri_get_drawable_info_back_clip_rects(
        &self,
        r: *const xcb_xf86dri_get_drawable_info_reply_t,
    ) -> *mut xcb_xf86dri_drm_clip_rect_t {
        sym!(self, xcb_xf86dri_get_drawable_info_back_clip_rects)(r)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_drawable_info_back_clip_rects` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_drawable_info_back_clip_rects(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_drawable_info_back_clip_rects)
    }

    pub unsafe fn xcb_xf86dri_get_drawable_info_back_clip_rects_length(
        &self,
        r: *const xcb_xf86dri_get_drawable_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_xf86dri_get_drawable_info_back_clip_rects_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_drawable_info_back_clip_rects_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_drawable_info_back_clip_rects_length(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_drawable_info_back_clip_rects_length)
    }

    pub unsafe fn xcb_xf86dri_get_drawable_info_back_clip_rects_iterator(
        &self,
        r: *const xcb_xf86dri_get_drawable_info_reply_t,
    ) -> xcb_xf86dri_drm_clip_rect_iterator_t {
        sym!(self, xcb_xf86dri_get_drawable_info_back_clip_rects_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_drawable_info_back_clip_rects_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_drawable_info_back_clip_rects_iterator(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_drawable_info_back_clip_rects_iterator)
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
     * xcb_xf86dri_get_drawable_info_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xf86dri_get_drawable_info_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_get_drawable_info_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xf86dri_get_drawable_info_reply_t {
        sym!(self, xcb_xf86dri_get_drawable_info_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_drawable_info_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_drawable_info_reply(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_drawable_info_reply)
    }

    pub unsafe fn xcb_xf86dri_get_device_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xf86dri_get_device_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_device_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_device_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_device_info_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xf86dri_get_device_info(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_xf86dri_get_device_info_cookie_t {
        sym!(self, xcb_xf86dri_get_device_info)(c, screen)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_device_info` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_device_info(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_device_info)
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
    pub unsafe fn xcb_xf86dri_get_device_info_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_xf86dri_get_device_info_cookie_t {
        sym!(self, xcb_xf86dri_get_device_info_unchecked)(c, screen)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_device_info_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_device_info_unchecked(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_device_info_unchecked)
    }

    pub unsafe fn xcb_xf86dri_get_device_info_device_private(
        &self,
        r: *const xcb_xf86dri_get_device_info_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_xf86dri_get_device_info_device_private)(r)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_device_info_device_private` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_device_info_device_private(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_device_info_device_private)
    }

    pub unsafe fn xcb_xf86dri_get_device_info_device_private_length(
        &self,
        r: *const xcb_xf86dri_get_device_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_xf86dri_get_device_info_device_private_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_device_info_device_private_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_device_info_device_private_length(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_device_info_device_private_length)
    }

    pub unsafe fn xcb_xf86dri_get_device_info_device_private_end(
        &self,
        r: *const xcb_xf86dri_get_device_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xf86dri_get_device_info_device_private_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_device_info_device_private_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_device_info_device_private_end(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_device_info_device_private_end)
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
     * xcb_xf86dri_get_device_info_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xf86dri_get_device_info_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_get_device_info_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xf86dri_get_device_info_reply_t {
        sym!(self, xcb_xf86dri_get_device_info_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_device_info_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_device_info_reply(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_device_info_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xf86dri_auth_connection(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        magic: u32,
    ) -> xcb_xf86dri_auth_connection_cookie_t {
        sym!(self, xcb_xf86dri_auth_connection)(c, screen, magic)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_auth_connection` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_auth_connection(&self) -> bool {
        has_sym!(self, xcb_xf86dri_auth_connection)
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
    pub unsafe fn xcb_xf86dri_auth_connection_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        magic: u32,
    ) -> xcb_xf86dri_auth_connection_cookie_t {
        sym!(self, xcb_xf86dri_auth_connection_unchecked)(c, screen, magic)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_auth_connection_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_auth_connection_unchecked(&self) -> bool {
        has_sym!(self, xcb_xf86dri_auth_connection_unchecked)
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
     * xcb_xf86dri_auth_connection_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xf86dri_auth_connection_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_auth_connection_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xf86dri_auth_connection_reply_t {
        sym!(self, xcb_xf86dri_auth_connection_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_auth_connection_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_auth_connection_reply(&self) -> bool {
        has_sym!(self, xcb_xf86dri_auth_connection_reply)
    }
}

#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbXf86dri::load().unwrap() };
        assert!(lib.has_xcb_xf86dri_id());
        assert!(lib.has_xcb_xf86dri_drm_clip_rect_next());
        assert!(lib.has_xcb_xf86dri_drm_clip_rect_end());
        assert!(lib.has_xcb_xf86dri_query_version());
        assert!(lib.has_xcb_xf86dri_query_version_unchecked());
        assert!(lib.has_xcb_xf86dri_query_version_reply());
        assert!(lib.has_xcb_xf86dri_query_direct_rendering_capable());
        assert!(lib.has_xcb_xf86dri_query_direct_rendering_capable_unchecked());
        assert!(lib.has_xcb_xf86dri_query_direct_rendering_capable_reply());
        assert!(lib.has_xcb_xf86dri_open_connection_sizeof());
        assert!(lib.has_xcb_xf86dri_open_connection());
        assert!(lib.has_xcb_xf86dri_open_connection_unchecked());
        assert!(lib.has_xcb_xf86dri_open_connection_bus_id());
        assert!(lib.has_xcb_xf86dri_open_connection_bus_id_length());
        assert!(lib.has_xcb_xf86dri_open_connection_bus_id_end());
        assert!(lib.has_xcb_xf86dri_open_connection_reply());
        assert!(lib.has_xcb_xf86dri_close_connection_checked());
        assert!(lib.has_xcb_xf86dri_close_connection());
        assert!(lib.has_xcb_xf86dri_get_client_driver_name_sizeof());
        assert!(lib.has_xcb_xf86dri_get_client_driver_name());
        assert!(lib.has_xcb_xf86dri_get_client_driver_name_unchecked());
        assert!(lib.has_xcb_xf86dri_get_client_driver_name_client_driver_name());
        assert!(lib.has_xcb_xf86dri_get_client_driver_name_client_driver_name_length());
        assert!(lib.has_xcb_xf86dri_get_client_driver_name_client_driver_name_end());
        assert!(lib.has_xcb_xf86dri_get_client_driver_name_reply());
        assert!(lib.has_xcb_xf86dri_create_context());
        assert!(lib.has_xcb_xf86dri_create_context_unchecked());
        assert!(lib.has_xcb_xf86dri_create_context_reply());
        assert!(lib.has_xcb_xf86dri_destroy_context_checked());
        assert!(lib.has_xcb_xf86dri_destroy_context());
        assert!(lib.has_xcb_xf86dri_create_drawable());
        assert!(lib.has_xcb_xf86dri_create_drawable_unchecked());
        assert!(lib.has_xcb_xf86dri_create_drawable_reply());
        assert!(lib.has_xcb_xf86dri_destroy_drawable_checked());
        assert!(lib.has_xcb_xf86dri_destroy_drawable());
        assert!(lib.has_xcb_xf86dri_get_drawable_info_sizeof());
        assert!(lib.has_xcb_xf86dri_get_drawable_info());
        assert!(lib.has_xcb_xf86dri_get_drawable_info_unchecked());
        assert!(lib.has_xcb_xf86dri_get_drawable_info_clip_rects());
        assert!(lib.has_xcb_xf86dri_get_drawable_info_clip_rects_length());
        assert!(lib.has_xcb_xf86dri_get_drawable_info_clip_rects_iterator());
        assert!(lib.has_xcb_xf86dri_get_drawable_info_back_clip_rects());
        assert!(lib.has_xcb_xf86dri_get_drawable_info_back_clip_rects_length());
        assert!(lib.has_xcb_xf86dri_get_drawable_info_back_clip_rects_iterator());
        assert!(lib.has_xcb_xf86dri_get_drawable_info_reply());
        assert!(lib.has_xcb_xf86dri_get_device_info_sizeof());
        assert!(lib.has_xcb_xf86dri_get_device_info());
        assert!(lib.has_xcb_xf86dri_get_device_info_unchecked());
        assert!(lib.has_xcb_xf86dri_get_device_info_device_private());
        assert!(lib.has_xcb_xf86dri_get_device_info_device_private_length());
        assert!(lib.has_xcb_xf86dri_get_device_info_device_private_end());
        assert!(lib.has_xcb_xf86dri_get_device_info_reply());
        assert!(lib.has_xcb_xf86dri_auth_connection());
        assert!(lib.has_xcb_xf86dri_auth_connection_unchecked());
        assert!(lib.has_xcb_xf86dri_auth_connection_reply());
    }
}
