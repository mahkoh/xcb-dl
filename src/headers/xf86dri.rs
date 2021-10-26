// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The `XF86Dri::DrmClipRect` struct.
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

/// An iterator over `XF86Dri::DrmClipRect` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_drm_clip_rect_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xf86dri_drm_clip_rect_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xf86dri_drm_clip_rect_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `XF86Dri::QueryVersion` request.
///
/// Pass this cookie to [`xcb_xf86dri_query_version_reply`] to retrieve the reply.
///
/// [`xcb_xf86dri_query_version_reply`]: XcbXf86dri::xcb_xf86dri_query_version_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_query_version_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xf86dri_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86Dri::QueryVersion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86dri::xcb_xf86dri_id()`], then the type of the request is
/// [`xcb_xf86dri_query_version_request_t`].
pub const XCB_XF86DRI_QUERY_VERSION: u8 = 0i32 as u8;

/// The `XF86Dri::QueryVersion` request.
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

/// The `XF86Dri::QueryVersion` reply.
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

/// The cookie for the reply to a `XF86Dri::QueryDirectRenderingCapable` request.
///
/// Pass this cookie to [`xcb_xf86dri_query_direct_rendering_capable_reply`] to retrieve the reply.
///
/// [`xcb_xf86dri_query_direct_rendering_capable_reply`]: XcbXf86dri::xcb_xf86dri_query_direct_rendering_capable_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_query_direct_rendering_capable_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xf86dri_query_direct_rendering_capable_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86Dri::QueryDirectRenderingCapable` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86dri::xcb_xf86dri_id()`], then the type of the request is
/// [`xcb_xf86dri_query_direct_rendering_capable_request_t`].
pub const XCB_XF86DRI_QUERY_DIRECT_RENDERING_CAPABLE: u8 = 1i32 as u8;

/// The `XF86Dri::QueryDirectRenderingCapable` request.
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

/// The `XF86Dri::QueryDirectRenderingCapable` reply.
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

/// The cookie for the reply to a `XF86Dri::OpenConnection` request.
///
/// Pass this cookie to [`xcb_xf86dri_open_connection_reply`] to retrieve the reply.
///
/// [`xcb_xf86dri_open_connection_reply`]: XcbXf86dri::xcb_xf86dri_open_connection_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_open_connection_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xf86dri_open_connection_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86Dri::OpenConnection` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86dri::xcb_xf86dri_id()`], then the type of the request is
/// [`xcb_xf86dri_open_connection_request_t`].
pub const XCB_XF86DRI_OPEN_CONNECTION: u8 = 2i32 as u8;

/// The `XF86Dri::OpenConnection` request.
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

/// The `XF86Dri::OpenConnection` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `bus_id`
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

/// The opcode for `XF86Dri::CloseConnection` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86dri::xcb_xf86dri_id()`], then the type of the request is
/// [`xcb_xf86dri_close_connection_request_t`].
pub const XCB_XF86DRI_CLOSE_CONNECTION: u8 = 3i32 as u8;

/// The `XF86Dri::CloseConnection` request.
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

/// The cookie for the reply to a `XF86Dri::GetClientDriverName` request.
///
/// Pass this cookie to [`xcb_xf86dri_get_client_driver_name_reply`] to retrieve the reply.
///
/// [`xcb_xf86dri_get_client_driver_name_reply`]: XcbXf86dri::xcb_xf86dri_get_client_driver_name_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_get_client_driver_name_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xf86dri_get_client_driver_name_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86Dri::GetClientDriverName` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86dri::xcb_xf86dri_id()`], then the type of the request is
/// [`xcb_xf86dri_get_client_driver_name_request_t`].
pub const XCB_XF86DRI_GET_CLIENT_DRIVER_NAME: u8 = 4i32 as u8;

/// The `XF86Dri::GetClientDriverName` request.
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

/// The `XF86Dri::GetClientDriverName` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `client_driver_name`
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

/// The cookie for the reply to a `XF86Dri::CreateContext` request.
///
/// Pass this cookie to [`xcb_xf86dri_create_context_reply`] to retrieve the reply.
///
/// [`xcb_xf86dri_create_context_reply`]: XcbXf86dri::xcb_xf86dri_create_context_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_create_context_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xf86dri_create_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86Dri::CreateContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86dri::xcb_xf86dri_id()`], then the type of the request is
/// [`xcb_xf86dri_create_context_request_t`].
pub const XCB_XF86DRI_CREATE_CONTEXT: u8 = 5i32 as u8;

/// The `XF86Dri::CreateContext` request.
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

/// The `XF86Dri::CreateContext` reply.
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

/// The opcode for `XF86Dri::DestroyContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86dri::xcb_xf86dri_id()`], then the type of the request is
/// [`xcb_xf86dri_destroy_context_request_t`].
pub const XCB_XF86DRI_DESTROY_CONTEXT: u8 = 6i32 as u8;

/// The `XF86Dri::DestroyContext` request.
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

/// The cookie for the reply to a `XF86Dri::CreateDrawable` request.
///
/// Pass this cookie to [`xcb_xf86dri_create_drawable_reply`] to retrieve the reply.
///
/// [`xcb_xf86dri_create_drawable_reply`]: XcbXf86dri::xcb_xf86dri_create_drawable_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_create_drawable_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xf86dri_create_drawable_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86Dri::CreateDrawable` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86dri::xcb_xf86dri_id()`], then the type of the request is
/// [`xcb_xf86dri_create_drawable_request_t`].
pub const XCB_XF86DRI_CREATE_DRAWABLE: u8 = 7i32 as u8;

/// The `XF86Dri::CreateDrawable` request.
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

/// The `XF86Dri::CreateDrawable` reply.
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

/// The opcode for `XF86Dri::DestroyDrawable` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86dri::xcb_xf86dri_id()`], then the type of the request is
/// [`xcb_xf86dri_destroy_drawable_request_t`].
pub const XCB_XF86DRI_DESTROY_DRAWABLE: u8 = 8i32 as u8;

/// The `XF86Dri::DestroyDrawable` request.
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

/// The cookie for the reply to a `XF86Dri::GetDrawableInfo` request.
///
/// Pass this cookie to [`xcb_xf86dri_get_drawable_info_reply`] to retrieve the reply.
///
/// [`xcb_xf86dri_get_drawable_info_reply`]: XcbXf86dri::xcb_xf86dri_get_drawable_info_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_get_drawable_info_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xf86dri_get_drawable_info_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86Dri::GetDrawableInfo` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86dri::xcb_xf86dri_id()`], then the type of the request is
/// [`xcb_xf86dri_get_drawable_info_request_t`].
pub const XCB_XF86DRI_GET_DRAWABLE_INFO: u8 = 9i32 as u8;

/// The `XF86Dri::GetDrawableInfo` request.
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

/// The `XF86Dri::GetDrawableInfo` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `clip_rects`
/// - `back_clip_rects`
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

/// The cookie for the reply to a `XF86Dri::GetDeviceInfo` request.
///
/// Pass this cookie to [`xcb_xf86dri_get_device_info_reply`] to retrieve the reply.
///
/// [`xcb_xf86dri_get_device_info_reply`]: XcbXf86dri::xcb_xf86dri_get_device_info_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_get_device_info_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xf86dri_get_device_info_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86Dri::GetDeviceInfo` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86dri::xcb_xf86dri_id()`], then the type of the request is
/// [`xcb_xf86dri_get_device_info_request_t`].
pub const XCB_XF86DRI_GET_DEVICE_INFO: u8 = 10i32 as u8;

/// The `XF86Dri::GetDeviceInfo` request.
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

/// The `XF86Dri::GetDeviceInfo` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `device_private`
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

/// The cookie for the reply to a `XF86Dri::AuthConnection` request.
///
/// Pass this cookie to [`xcb_xf86dri_auth_connection_reply`] to retrieve the reply.
///
/// [`xcb_xf86dri_auth_connection_reply`]: XcbXf86dri::xcb_xf86dri_auth_connection_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_auth_connection_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xf86dri_auth_connection_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XF86Dri::AuthConnection` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXf86dri::xcb_xf86dri_id()`], then the type of the request is
/// [`xcb_xf86dri_auth_connection_request_t`].
pub const XCB_XF86DRI_AUTH_CONNECTION: u8 = 11i32 as u8;

/// The `XF86Dri::AuthConnection` request.
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

/// The `XF86Dri::AuthConnection` reply.
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

#[cfg(feature = "xcb_xf86dri")]
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
        ) -> *mut xcb_xf86dri_query_version_reply_t,
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
        ) -> *mut xcb_xf86dri_query_direct_rendering_capable_reply_t,
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
        ) -> *mut xcb_xf86dri_open_connection_reply_t,
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
        ) -> *mut xcb_xf86dri_get_client_driver_name_reply_t,
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
        ) -> *mut xcb_xf86dri_create_context_reply_t,
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
        ) -> *mut xcb_xf86dri_create_drawable_reply_t,
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
        ) -> *mut xcb_xf86dri_get_drawable_info_reply_t,
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
        ) -> *mut xcb_xf86dri_get_device_info_reply_t,
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
        ) -> *mut xcb_xf86dri_auth_connection_reply_t,
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

#[cfg(feature = "xcb_xf86dri")]
impl XcbXf86dri {
    /// The libxcb identifier of the `XF86Dri` extension.
    #[inline]
    pub fn xcb_xf86dri_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_xf86dri_id) }
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_id(&self) -> bool {
        has_sym!(self, xcb_xf86dri_id)
    }

    /// Advances a `xcb_xf86dri_drm_clip_rect_iterator_t` iterator by 1 element.
    #[inline]
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

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xf86dri_drm_clip_rect_iterator_t`.
    #[inline]
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

    /// Sends a `XF86Dri::QueryVersion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xf86dri_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xf86dri_query_version_reply`]: Self::xcb_xf86dri_query_version_reply
    #[inline]
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

    /// Sends a `XF86Dri::QueryVersion` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xf86dri_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xf86dri_query_version_reply`]: Self::xcb_xf86dri_query_version_reply
    #[inline]
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

    /// Waits for the reply to a `XF86Dri::QueryVersion` request.
    #[inline]
    pub unsafe fn xcb_xf86dri_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xf86dri_query_version_reply_t {
        sym!(self, xcb_xf86dri_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_xf86dri_query_version_reply)
    }

    /// Sends a `XF86Dri::QueryDirectRenderingCapable` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xf86dri_query_direct_rendering_capable_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xf86dri_query_direct_rendering_capable_reply`]: Self::xcb_xf86dri_query_direct_rendering_capable_reply
    #[inline]
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

    /// Sends a `XF86Dri::QueryDirectRenderingCapable` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xf86dri_query_direct_rendering_capable_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xf86dri_query_direct_rendering_capable_reply`]: Self::xcb_xf86dri_query_direct_rendering_capable_reply
    #[inline]
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

    /// Waits for the reply to a `XF86Dri::QueryDirectRenderingCapable` request.
    #[inline]
    pub unsafe fn xcb_xf86dri_query_direct_rendering_capable_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_query_direct_rendering_capable_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xf86dri_query_direct_rendering_capable_reply_t {
        sym!(self, xcb_xf86dri_query_direct_rendering_capable_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_query_direct_rendering_capable_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_query_direct_rendering_capable_reply(&self) -> bool {
        has_sym!(self, xcb_xf86dri_query_direct_rendering_capable_reply)
    }

    /// Computes the size of a `xcb_xf86dri_open_connection_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xf86dri_open_connection_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xf86dri_open_connection_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_open_connection_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_open_connection_sizeof(&self) -> bool {
        has_sym!(self, xcb_xf86dri_open_connection_sizeof)
    }

    /// Sends a `XF86Dri::OpenConnection` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xf86dri_open_connection_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xf86dri_open_connection_reply`]: Self::xcb_xf86dri_open_connection_reply
    #[inline]
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

    /// Sends a `XF86Dri::OpenConnection` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xf86dri_open_connection_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xf86dri_open_connection_reply`]: Self::xcb_xf86dri_open_connection_reply
    #[inline]
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

    /// Returns a pointer to the `bus_id` field of a `xcb_xf86dri_open_connection_reply_t` struct.
    #[inline]
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

    /// Returns the number of elements of the `bus_id` field of a `xcb_xf86dri_open_connection_reply_t` struct.
    #[inline]
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

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `bus_id` field of a `xcb_xf86dri_open_connection_reply_t` struct.
    #[inline]
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

    /// Waits for the reply to a `XF86Dri::OpenConnection` request.
    #[inline]
    pub unsafe fn xcb_xf86dri_open_connection_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_open_connection_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xf86dri_open_connection_reply_t {
        sym!(self, xcb_xf86dri_open_connection_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_open_connection_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_open_connection_reply(&self) -> bool {
        has_sym!(self, xcb_xf86dri_open_connection_reply)
    }

    /// Sends a `XF86Dri::CloseConnection` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
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

    /// Sends a `XF86Dri::CloseConnection` request (unchecked).
    #[inline]
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

    /// Computes the size of a `xcb_xf86dri_get_client_driver_name_reply_t` object.
    #[inline]
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

    /// Sends a `XF86Dri::GetClientDriverName` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xf86dri_get_client_driver_name_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xf86dri_get_client_driver_name_reply`]: Self::xcb_xf86dri_get_client_driver_name_reply
    #[inline]
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

    /// Sends a `XF86Dri::GetClientDriverName` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xf86dri_get_client_driver_name_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xf86dri_get_client_driver_name_reply`]: Self::xcb_xf86dri_get_client_driver_name_reply
    #[inline]
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

    /// Returns a pointer to the `client_driver_name` field of a `xcb_xf86dri_get_client_driver_name_reply_t` struct.
    #[inline]
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

    /// Returns the number of elements of the `client_driver_name` field of a `xcb_xf86dri_get_client_driver_name_reply_t` struct.
    #[inline]
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

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `client_driver_name` field of a `xcb_xf86dri_get_client_driver_name_reply_t` struct.
    #[inline]
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

    /// Waits for the reply to a `XF86Dri::GetClientDriverName` request.
    #[inline]
    pub unsafe fn xcb_xf86dri_get_client_driver_name_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_get_client_driver_name_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xf86dri_get_client_driver_name_reply_t {
        sym!(self, xcb_xf86dri_get_client_driver_name_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_client_driver_name_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_client_driver_name_reply(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_client_driver_name_reply)
    }

    /// Sends a `XF86Dri::CreateContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xf86dri_create_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xf86dri_create_context_reply`]: Self::xcb_xf86dri_create_context_reply
    #[inline]
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

    /// Sends a `XF86Dri::CreateContext` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xf86dri_create_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xf86dri_create_context_reply`]: Self::xcb_xf86dri_create_context_reply
    #[inline]
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

    /// Waits for the reply to a `XF86Dri::CreateContext` request.
    #[inline]
    pub unsafe fn xcb_xf86dri_create_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_create_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xf86dri_create_context_reply_t {
        sym!(self, xcb_xf86dri_create_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_create_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_create_context_reply(&self) -> bool {
        has_sym!(self, xcb_xf86dri_create_context_reply)
    }

    /// Sends a `XF86Dri::DestroyContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
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

    /// Sends a `XF86Dri::DestroyContext` request (unchecked).
    #[inline]
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

    /// Sends a `XF86Dri::CreateDrawable` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xf86dri_create_drawable_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xf86dri_create_drawable_reply`]: Self::xcb_xf86dri_create_drawable_reply
    #[inline]
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

    /// Sends a `XF86Dri::CreateDrawable` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xf86dri_create_drawable_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xf86dri_create_drawable_reply`]: Self::xcb_xf86dri_create_drawable_reply
    #[inline]
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

    /// Waits for the reply to a `XF86Dri::CreateDrawable` request.
    #[inline]
    pub unsafe fn xcb_xf86dri_create_drawable_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_create_drawable_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xf86dri_create_drawable_reply_t {
        sym!(self, xcb_xf86dri_create_drawable_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_create_drawable_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_create_drawable_reply(&self) -> bool {
        has_sym!(self, xcb_xf86dri_create_drawable_reply)
    }

    /// Sends a `XF86Dri::DestroyDrawable` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
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

    /// Sends a `XF86Dri::DestroyDrawable` request (unchecked).
    #[inline]
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

    /// Computes the size of a `xcb_xf86dri_get_drawable_info_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xf86dri_get_drawable_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xf86dri_get_drawable_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_drawable_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_drawable_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_drawable_info_sizeof)
    }

    /// Sends a `XF86Dri::GetDrawableInfo` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xf86dri_get_drawable_info_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xf86dri_get_drawable_info_reply`]: Self::xcb_xf86dri_get_drawable_info_reply
    #[inline]
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

    /// Sends a `XF86Dri::GetDrawableInfo` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xf86dri_get_drawable_info_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xf86dri_get_drawable_info_reply`]: Self::xcb_xf86dri_get_drawable_info_reply
    #[inline]
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

    /// Returns a pointer to the `clip_rects` field of a `xcb_xf86dri_get_drawable_info_reply_t` struct.
    #[inline]
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

    /// Returns the number of elements of the `clip_rects` field of a `xcb_xf86dri_get_drawable_info_reply_t` struct.
    #[inline]
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

    /// Returns an iterator over the elements of the
    /// `clip_rects` field of a `xcb_xf86dri_get_drawable_info_reply_t` struct.
    #[inline]
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

    /// Returns a pointer to the `back_clip_rects` field of a `xcb_xf86dri_get_drawable_info_reply_t` struct.
    #[inline]
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

    /// Returns the number of elements of the `back_clip_rects` field of a `xcb_xf86dri_get_drawable_info_reply_t` struct.
    #[inline]
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

    /// Returns an iterator over the elements of the
    /// `back_clip_rects` field of a `xcb_xf86dri_get_drawable_info_reply_t` struct.
    #[inline]
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

    /// Waits for the reply to a `XF86Dri::GetDrawableInfo` request.
    #[inline]
    pub unsafe fn xcb_xf86dri_get_drawable_info_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_get_drawable_info_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xf86dri_get_drawable_info_reply_t {
        sym!(self, xcb_xf86dri_get_drawable_info_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_drawable_info_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_drawable_info_reply(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_drawable_info_reply)
    }

    /// Computes the size of a `xcb_xf86dri_get_device_info_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xf86dri_get_device_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xf86dri_get_device_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_device_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_device_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_device_info_sizeof)
    }

    /// Sends a `XF86Dri::GetDeviceInfo` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xf86dri_get_device_info_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xf86dri_get_device_info_reply`]: Self::xcb_xf86dri_get_device_info_reply
    #[inline]
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

    /// Sends a `XF86Dri::GetDeviceInfo` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xf86dri_get_device_info_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xf86dri_get_device_info_reply`]: Self::xcb_xf86dri_get_device_info_reply
    #[inline]
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

    /// Returns a pointer to the `device_private` field of a `xcb_xf86dri_get_device_info_reply_t` struct.
    #[inline]
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

    /// Returns the number of elements of the `device_private` field of a `xcb_xf86dri_get_device_info_reply_t` struct.
    #[inline]
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

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `device_private` field of a `xcb_xf86dri_get_device_info_reply_t` struct.
    #[inline]
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

    /// Waits for the reply to a `XF86Dri::GetDeviceInfo` request.
    #[inline]
    pub unsafe fn xcb_xf86dri_get_device_info_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_get_device_info_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xf86dri_get_device_info_reply_t {
        sym!(self, xcb_xf86dri_get_device_info_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_get_device_info_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_get_device_info_reply(&self) -> bool {
        has_sym!(self, xcb_xf86dri_get_device_info_reply)
    }

    /// Sends a `XF86Dri::AuthConnection` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xf86dri_auth_connection_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xf86dri_auth_connection_reply`]: Self::xcb_xf86dri_auth_connection_reply
    #[inline]
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

    /// Sends a `XF86Dri::AuthConnection` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xf86dri_auth_connection_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xf86dri_auth_connection_reply`]: Self::xcb_xf86dri_auth_connection_reply
    #[inline]
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

    /// Waits for the reply to a `XF86Dri::AuthConnection` request.
    #[inline]
    pub unsafe fn xcb_xf86dri_auth_connection_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_auth_connection_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xf86dri_auth_connection_reply_t {
        sym!(self, xcb_xf86dri_auth_connection_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xf86dri_auth_connection_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xf86dri_auth_connection_reply(&self) -> bool {
        has_sym!(self, xcb_xf86dri_auth_connection_reply)
    }
}

#[cfg(feature = "xcb_xf86dri")]
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
