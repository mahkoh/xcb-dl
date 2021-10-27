// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The name of the `DRI2` extension.
pub const XCB_DRI2_NAME: &[u8] = b"DRI2";

/// The name of the `DRI2` extension.
pub const XCB_DRI2_NAME_STR: &str = "DRI2";

/// The `DRI2::Attachment` enum.
///
/// This enum has the following variants:
///
/// - [`DRI2::Attachment::BufferFrontLeft`](XCB_DRI2_ATTACHMENT_BUFFER_FRONT_LEFT)
/// - [`DRI2::Attachment::BufferBackLeft`](XCB_DRI2_ATTACHMENT_BUFFER_BACK_LEFT)
/// - [`DRI2::Attachment::BufferFrontRight`](XCB_DRI2_ATTACHMENT_BUFFER_FRONT_RIGHT)
/// - [`DRI2::Attachment::BufferBackRight`](XCB_DRI2_ATTACHMENT_BUFFER_BACK_RIGHT)
/// - [`DRI2::Attachment::BufferDepth`](XCB_DRI2_ATTACHMENT_BUFFER_DEPTH)
/// - [`DRI2::Attachment::BufferStencil`](XCB_DRI2_ATTACHMENT_BUFFER_STENCIL)
/// - [`DRI2::Attachment::BufferAccum`](XCB_DRI2_ATTACHMENT_BUFFER_ACCUM)
/// - [`DRI2::Attachment::BufferFakeFrontLeft`](XCB_DRI2_ATTACHMENT_BUFFER_FAKE_FRONT_LEFT)
/// - [`DRI2::Attachment::BufferFakeFrontRight`](XCB_DRI2_ATTACHMENT_BUFFER_FAKE_FRONT_RIGHT)
/// - [`DRI2::Attachment::BufferDepthStencil`](XCB_DRI2_ATTACHMENT_BUFFER_DEPTH_STENCIL)
/// - [`DRI2::Attachment::BufferHiz`](XCB_DRI2_ATTACHMENT_BUFFER_HIZ)
pub type xcb_dri2_attachment_t = u32;
/// The `DRI2::Attachment::BufferFrontLeft` enum variant.
///
/// This is a variant of [`xcb_dri2_attachment_t`].
pub const XCB_DRI2_ATTACHMENT_BUFFER_FRONT_LEFT: xcb_dri2_attachment_t = 0;
/// The `DRI2::Attachment::BufferBackLeft` enum variant.
///
/// This is a variant of [`xcb_dri2_attachment_t`].
pub const XCB_DRI2_ATTACHMENT_BUFFER_BACK_LEFT: xcb_dri2_attachment_t = 1;
/// The `DRI2::Attachment::BufferFrontRight` enum variant.
///
/// This is a variant of [`xcb_dri2_attachment_t`].
pub const XCB_DRI2_ATTACHMENT_BUFFER_FRONT_RIGHT: xcb_dri2_attachment_t = 2;
/// The `DRI2::Attachment::BufferBackRight` enum variant.
///
/// This is a variant of [`xcb_dri2_attachment_t`].
pub const XCB_DRI2_ATTACHMENT_BUFFER_BACK_RIGHT: xcb_dri2_attachment_t = 3;
/// The `DRI2::Attachment::BufferDepth` enum variant.
///
/// This is a variant of [`xcb_dri2_attachment_t`].
pub const XCB_DRI2_ATTACHMENT_BUFFER_DEPTH: xcb_dri2_attachment_t = 4;
/// The `DRI2::Attachment::BufferStencil` enum variant.
///
/// This is a variant of [`xcb_dri2_attachment_t`].
pub const XCB_DRI2_ATTACHMENT_BUFFER_STENCIL: xcb_dri2_attachment_t = 5;
/// The `DRI2::Attachment::BufferAccum` enum variant.
///
/// This is a variant of [`xcb_dri2_attachment_t`].
pub const XCB_DRI2_ATTACHMENT_BUFFER_ACCUM: xcb_dri2_attachment_t = 6;
/// The `DRI2::Attachment::BufferFakeFrontLeft` enum variant.
///
/// This is a variant of [`xcb_dri2_attachment_t`].
pub const XCB_DRI2_ATTACHMENT_BUFFER_FAKE_FRONT_LEFT: xcb_dri2_attachment_t = 7;
/// The `DRI2::Attachment::BufferFakeFrontRight` enum variant.
///
/// This is a variant of [`xcb_dri2_attachment_t`].
pub const XCB_DRI2_ATTACHMENT_BUFFER_FAKE_FRONT_RIGHT: xcb_dri2_attachment_t = 8;
/// The `DRI2::Attachment::BufferDepthStencil` enum variant.
///
/// This is a variant of [`xcb_dri2_attachment_t`].
pub const XCB_DRI2_ATTACHMENT_BUFFER_DEPTH_STENCIL: xcb_dri2_attachment_t = 9;
/// The `DRI2::Attachment::BufferHiz` enum variant.
///
/// This is a variant of [`xcb_dri2_attachment_t`].
pub const XCB_DRI2_ATTACHMENT_BUFFER_HIZ: xcb_dri2_attachment_t = 10;

/// The `DRI2::DriverType` enum.
///
/// This enum has the following variants:
///
/// - [`DRI2::DriverType::DRI`](XCB_DRI2_DRIVER_TYPE_DRI)
/// - [`DRI2::DriverType::VDPAU`](XCB_DRI2_DRIVER_TYPE_VDPAU)
pub type xcb_dri2_driver_type_t = u32;
/// The `DRI2::DriverType::DRI` enum variant.
///
/// This is a variant of [`xcb_dri2_driver_type_t`].
pub const XCB_DRI2_DRIVER_TYPE_DRI: xcb_dri2_driver_type_t = 0;
/// The `DRI2::DriverType::VDPAU` enum variant.
///
/// This is a variant of [`xcb_dri2_driver_type_t`].
pub const XCB_DRI2_DRIVER_TYPE_VDPAU: xcb_dri2_driver_type_t = 1;

/// The `DRI2::EventType` enum.
///
/// This enum has the following variants:
///
/// - [`DRI2::EventType::ExchangeComplete`](XCB_DRI2_EVENT_TYPE_EXCHANGE_COMPLETE)
/// - [`DRI2::EventType::BlitComplete`](XCB_DRI2_EVENT_TYPE_BLIT_COMPLETE)
/// - [`DRI2::EventType::FlipComplete`](XCB_DRI2_EVENT_TYPE_FLIP_COMPLETE)
pub type xcb_dri2_event_type_t = u32;
/// The `DRI2::EventType::ExchangeComplete` enum variant.
///
/// This is a variant of [`xcb_dri2_event_type_t`].
pub const XCB_DRI2_EVENT_TYPE_EXCHANGE_COMPLETE: xcb_dri2_event_type_t = 1;
/// The `DRI2::EventType::BlitComplete` enum variant.
///
/// This is a variant of [`xcb_dri2_event_type_t`].
pub const XCB_DRI2_EVENT_TYPE_BLIT_COMPLETE: xcb_dri2_event_type_t = 2;
/// The `DRI2::EventType::FlipComplete` enum variant.
///
/// This is a variant of [`xcb_dri2_event_type_t`].
pub const XCB_DRI2_EVENT_TYPE_FLIP_COMPLETE: xcb_dri2_event_type_t = 3;

/// The `DRI2::DRI2Buffer` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_dri2_buffer_t {
    pub attachment: u32,
    pub name: u32,
    pub pitch: u32,
    pub cpp: u32,
    pub flags: u32,
}

impl Default for xcb_dri2_dri2_buffer_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `DRI2::DRI2Buffer` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_dri2_buffer_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_dri2_dri2_buffer_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_dri2_dri2_buffer_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `DRI2::AttachFormat` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_attach_format_t {
    pub attachment: u32,
    pub format: u32,
}

impl Default for xcb_dri2_attach_format_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `DRI2::AttachFormat` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_attach_format_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_dri2_attach_format_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_dri2_attach_format_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `DRI2::QueryVersion` request.
///
/// Pass this cookie to [`xcb_dri2_query_version_reply`] to retrieve the reply.
///
/// [`xcb_dri2_query_version_reply`]: XcbDri2::xcb_dri2_query_version_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_query_version_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_dri2_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DRI2::QueryVersion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDri2::xcb_dri2_id()`], then the type of the request is
/// [`xcb_dri2_query_version_request_t`].
pub const XCB_DRI2_QUERY_VERSION: u8 = 0i32 as u8;

/// The `DRI2::QueryVersion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub major_version: u32,
    pub minor_version: u32,
}

impl Default for xcb_dri2_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `DRI2::QueryVersion` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
}

impl Default for xcb_dri2_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `DRI2::Connect` request.
///
/// Pass this cookie to [`xcb_dri2_connect_reply`] to retrieve the reply.
///
/// [`xcb_dri2_connect_reply`]: XcbDri2::xcb_dri2_connect_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_connect_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_dri2_connect_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DRI2::Connect` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDri2::xcb_dri2_id()`], then the type of the request is
/// [`xcb_dri2_connect_request_t`].
pub const XCB_DRI2_CONNECT: u8 = 1i32 as u8;

/// The `DRI2::Connect` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_connect_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub driver_type: u32,
}

impl Default for xcb_dri2_connect_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `DRI2::Connect` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `driver_name`
/// - `alignment_pad`
/// - `device_name`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_connect_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub driver_name_length: u32,
    pub device_name_length: u32,
    pub pad1: [u8; 16],
}

impl Default for xcb_dri2_connect_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `DRI2::Authenticate` request.
///
/// Pass this cookie to [`xcb_dri2_authenticate_reply`] to retrieve the reply.
///
/// [`xcb_dri2_authenticate_reply`]: XcbDri2::xcb_dri2_authenticate_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_authenticate_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_dri2_authenticate_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DRI2::Authenticate` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDri2::xcb_dri2_id()`], then the type of the request is
/// [`xcb_dri2_authenticate_request_t`].
pub const XCB_DRI2_AUTHENTICATE: u8 = 2i32 as u8;

/// The `DRI2::Authenticate` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_authenticate_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub magic: u32,
}

impl Default for xcb_dri2_authenticate_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `DRI2::Authenticate` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_authenticate_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub authenticated: u32,
}

impl Default for xcb_dri2_authenticate_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DRI2::CreateDrawable` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDri2::xcb_dri2_id()`], then the type of the request is
/// [`xcb_dri2_create_drawable_request_t`].
pub const XCB_DRI2_CREATE_DRAWABLE: u8 = 3i32 as u8;

/// The `DRI2::CreateDrawable` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_create_drawable_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
}

impl Default for xcb_dri2_create_drawable_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DRI2::DestroyDrawable` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDri2::xcb_dri2_id()`], then the type of the request is
/// [`xcb_dri2_destroy_drawable_request_t`].
pub const XCB_DRI2_DESTROY_DRAWABLE: u8 = 4i32 as u8;

/// The `DRI2::DestroyDrawable` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_destroy_drawable_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
}

impl Default for xcb_dri2_destroy_drawable_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `DRI2::GetBuffers` request.
///
/// Pass this cookie to [`xcb_dri2_get_buffers_reply`] to retrieve the reply.
///
/// [`xcb_dri2_get_buffers_reply`]: XcbDri2::xcb_dri2_get_buffers_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_get_buffers_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_dri2_get_buffers_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DRI2::GetBuffers` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDri2::xcb_dri2_id()`], then the type of the request is
/// [`xcb_dri2_get_buffers_request_t`].
pub const XCB_DRI2_GET_BUFFERS: u8 = 5i32 as u8;

/// The `DRI2::GetBuffers` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `attachments`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_get_buffers_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub count: u32,
}

impl Default for xcb_dri2_get_buffers_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `DRI2::GetBuffers` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `buffers`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_get_buffers_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: u32,
    pub height: u32,
    pub count: u32,
    pub pad1: [u8; 12],
}

impl Default for xcb_dri2_get_buffers_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `DRI2::CopyRegion` request.
///
/// Pass this cookie to [`xcb_dri2_copy_region_reply`] to retrieve the reply.
///
/// [`xcb_dri2_copy_region_reply`]: XcbDri2::xcb_dri2_copy_region_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_copy_region_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_dri2_copy_region_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DRI2::CopyRegion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDri2::xcb_dri2_id()`], then the type of the request is
/// [`xcb_dri2_copy_region_request_t`].
pub const XCB_DRI2_COPY_REGION: u8 = 6i32 as u8;

/// The `DRI2::CopyRegion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_copy_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub region: u32,
    pub dest: u32,
    pub src: u32,
}

impl Default for xcb_dri2_copy_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `DRI2::CopyRegion` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_copy_region_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
}

impl Default for xcb_dri2_copy_region_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `DRI2::GetBuffersWithFormat` request.
///
/// Pass this cookie to [`xcb_dri2_get_buffers_with_format_reply`] to retrieve the reply.
///
/// [`xcb_dri2_get_buffers_with_format_reply`]: XcbDri2::xcb_dri2_get_buffers_with_format_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_get_buffers_with_format_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_dri2_get_buffers_with_format_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DRI2::GetBuffersWithFormat` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDri2::xcb_dri2_id()`], then the type of the request is
/// [`xcb_dri2_get_buffers_with_format_request_t`].
pub const XCB_DRI2_GET_BUFFERS_WITH_FORMAT: u8 = 7i32 as u8;

/// The `DRI2::GetBuffersWithFormat` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `attachments`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_get_buffers_with_format_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub count: u32,
}

impl Default for xcb_dri2_get_buffers_with_format_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `DRI2::GetBuffersWithFormat` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `buffers`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_get_buffers_with_format_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: u32,
    pub height: u32,
    pub count: u32,
    pub pad1: [u8; 12],
}

impl Default for xcb_dri2_get_buffers_with_format_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `DRI2::SwapBuffers` request.
///
/// Pass this cookie to [`xcb_dri2_swap_buffers_reply`] to retrieve the reply.
///
/// [`xcb_dri2_swap_buffers_reply`]: XcbDri2::xcb_dri2_swap_buffers_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_swap_buffers_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_dri2_swap_buffers_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DRI2::SwapBuffers` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDri2::xcb_dri2_id()`], then the type of the request is
/// [`xcb_dri2_swap_buffers_request_t`].
pub const XCB_DRI2_SWAP_BUFFERS: u8 = 8i32 as u8;

/// The `DRI2::SwapBuffers` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_swap_buffers_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub target_msc_hi: u32,
    pub target_msc_lo: u32,
    pub divisor_hi: u32,
    pub divisor_lo: u32,
    pub remainder_hi: u32,
    pub remainder_lo: u32,
}

impl Default for xcb_dri2_swap_buffers_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `DRI2::SwapBuffers` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_swap_buffers_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub swap_hi: u32,
    pub swap_lo: u32,
}

impl Default for xcb_dri2_swap_buffers_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `DRI2::GetMSC` request.
///
/// Pass this cookie to [`xcb_dri2_get_msc_reply`] to retrieve the reply.
///
/// [`xcb_dri2_get_msc_reply`]: XcbDri2::xcb_dri2_get_msc_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_get_msc_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_dri2_get_msc_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DRI2::GetMSC` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDri2::xcb_dri2_id()`], then the type of the request is
/// [`xcb_dri2_get_msc_request_t`].
pub const XCB_DRI2_GET_MSC: u8 = 9i32 as u8;

/// The `DRI2::GetMSC` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_get_msc_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
}

impl Default for xcb_dri2_get_msc_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `DRI2::GetMSC` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_get_msc_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub ust_hi: u32,
    pub ust_lo: u32,
    pub msc_hi: u32,
    pub msc_lo: u32,
    pub sbc_hi: u32,
    pub sbc_lo: u32,
}

impl Default for xcb_dri2_get_msc_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `DRI2::WaitMSC` request.
///
/// Pass this cookie to [`xcb_dri2_wait_msc_reply`] to retrieve the reply.
///
/// [`xcb_dri2_wait_msc_reply`]: XcbDri2::xcb_dri2_wait_msc_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_wait_msc_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_dri2_wait_msc_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DRI2::WaitMSC` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDri2::xcb_dri2_id()`], then the type of the request is
/// [`xcb_dri2_wait_msc_request_t`].
pub const XCB_DRI2_WAIT_MSC: u8 = 10i32 as u8;

/// The `DRI2::WaitMSC` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_wait_msc_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub target_msc_hi: u32,
    pub target_msc_lo: u32,
    pub divisor_hi: u32,
    pub divisor_lo: u32,
    pub remainder_hi: u32,
    pub remainder_lo: u32,
}

impl Default for xcb_dri2_wait_msc_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `DRI2::WaitMSC` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_wait_msc_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub ust_hi: u32,
    pub ust_lo: u32,
    pub msc_hi: u32,
    pub msc_lo: u32,
    pub sbc_hi: u32,
    pub sbc_lo: u32,
}

impl Default for xcb_dri2_wait_msc_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `DRI2::WaitSBC` request.
///
/// Pass this cookie to [`xcb_dri2_wait_sbc_reply`] to retrieve the reply.
///
/// [`xcb_dri2_wait_sbc_reply`]: XcbDri2::xcb_dri2_wait_sbc_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_wait_sbc_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_dri2_wait_sbc_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DRI2::WaitSBC` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDri2::xcb_dri2_id()`], then the type of the request is
/// [`xcb_dri2_wait_sbc_request_t`].
pub const XCB_DRI2_WAIT_SBC: u8 = 11i32 as u8;

/// The `DRI2::WaitSBC` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_wait_sbc_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub target_sbc_hi: u32,
    pub target_sbc_lo: u32,
}

impl Default for xcb_dri2_wait_sbc_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `DRI2::WaitSBC` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_wait_sbc_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub ust_hi: u32,
    pub ust_lo: u32,
    pub msc_hi: u32,
    pub msc_lo: u32,
    pub sbc_hi: u32,
    pub sbc_lo: u32,
}

impl Default for xcb_dri2_wait_sbc_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DRI2::SwapInterval` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDri2::xcb_dri2_id()`], then the type of the request is
/// [`xcb_dri2_swap_interval_request_t`].
pub const XCB_DRI2_SWAP_INTERVAL: u8 = 12i32 as u8;

/// The `DRI2::SwapInterval` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_swap_interval_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub interval: u32,
}

impl Default for xcb_dri2_swap_interval_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `DRI2::GetParam` request.
///
/// Pass this cookie to [`xcb_dri2_get_param_reply`] to retrieve the reply.
///
/// [`xcb_dri2_get_param_reply`]: XcbDri2::xcb_dri2_get_param_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_get_param_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_dri2_get_param_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DRI2::GetParam` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDri2::xcb_dri2_id()`], then the type of the request is
/// [`xcb_dri2_get_param_request_t`].
pub const XCB_DRI2_GET_PARAM: u8 = 13i32 as u8;

/// The `DRI2::GetParam` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_get_param_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub param: u32,
}

impl Default for xcb_dri2_get_param_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `DRI2::GetParam` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_get_param_reply_t {
    pub response_type: u8,
    pub is_param_recognized: u8,
    pub sequence: u16,
    pub length: u32,
    pub value_hi: u32,
    pub value_lo: u32,
}

impl Default for xcb_dri2_get_param_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DRI2::BufferSwapComplete` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_dri2_buffer_swap_complete_event_t`].
pub const XCB_DRI2_BUFFER_SWAP_COMPLETE: u8 = 0i32 as u8;

/// The `DRI2::BufferSwapComplete` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_buffer_swap_complete_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event_type: u16,
    pub pad1: [u8; 2],
    pub drawable: xcb_drawable_t,
    pub ust_hi: u32,
    pub ust_lo: u32,
    pub msc_hi: u32,
    pub msc_lo: u32,
    pub sbc: u32,
}

impl Default for xcb_dri2_buffer_swap_complete_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DRI2::InvalidateBuffers` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_dri2_invalidate_buffers_event_t`].
pub const XCB_DRI2_INVALIDATE_BUFFERS: u8 = 1i32 as u8;

/// The `DRI2::InvalidateBuffers` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_invalidate_buffers_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub drawable: xcb_drawable_t,
}

impl Default for xcb_dri2_invalidate_buffers_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_dri2")]
pub(crate) struct XcbDri2Dri2 {
    xcb_dri2_id: LazySymbol<*mut xcb_extension_t>,
    xcb_dri2_dri2_buffer_next: LazySymbol<unsafe fn(i: *mut xcb_dri2_dri2_buffer_iterator_t)>,
    xcb_dri2_dri2_buffer_end:
        LazySymbol<unsafe fn(i: xcb_dri2_dri2_buffer_iterator_t) -> xcb_generic_iterator_t>,
    xcb_dri2_attach_format_next: LazySymbol<unsafe fn(i: *mut xcb_dri2_attach_format_iterator_t)>,
    xcb_dri2_attach_format_end:
        LazySymbol<unsafe fn(i: xcb_dri2_attach_format_iterator_t) -> xcb_generic_iterator_t>,
    xcb_dri2_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u32,
            minor_version: u32,
        ) -> xcb_dri2_query_version_cookie_t,
    >,
    xcb_dri2_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u32,
            minor_version: u32,
        ) -> xcb_dri2_query_version_cookie_t,
    >,
    xcb_dri2_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dri2_query_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_dri2_query_version_reply_t,
    >,
    xcb_dri2_connect_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_dri2_connect: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            driver_type: u32,
        ) -> xcb_dri2_connect_cookie_t,
    >,
    xcb_dri2_connect_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            driver_type: u32,
        ) -> xcb_dri2_connect_cookie_t,
    >,
    xcb_dri2_connect_driver_name:
        LazySymbol<unsafe fn(r: *const xcb_dri2_connect_reply_t) -> *mut c_char>,
    xcb_dri2_connect_driver_name_length:
        LazySymbol<unsafe fn(r: *const xcb_dri2_connect_reply_t) -> c_int>,
    xcb_dri2_connect_driver_name_end:
        LazySymbol<unsafe fn(r: *const xcb_dri2_connect_reply_t) -> xcb_generic_iterator_t>,
    xcb_dri2_connect_alignment_pad:
        LazySymbol<unsafe fn(r: *const xcb_dri2_connect_reply_t) -> *mut c_void>,
    xcb_dri2_connect_alignment_pad_length:
        LazySymbol<unsafe fn(r: *const xcb_dri2_connect_reply_t) -> c_int>,
    xcb_dri2_connect_alignment_pad_end:
        LazySymbol<unsafe fn(r: *const xcb_dri2_connect_reply_t) -> xcb_generic_iterator_t>,
    xcb_dri2_connect_device_name:
        LazySymbol<unsafe fn(r: *const xcb_dri2_connect_reply_t) -> *mut c_char>,
    xcb_dri2_connect_device_name_length:
        LazySymbol<unsafe fn(r: *const xcb_dri2_connect_reply_t) -> c_int>,
    xcb_dri2_connect_device_name_end:
        LazySymbol<unsafe fn(r: *const xcb_dri2_connect_reply_t) -> xcb_generic_iterator_t>,
    xcb_dri2_connect_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dri2_connect_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_dri2_connect_reply_t,
    >,
    xcb_dri2_authenticate: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            magic: u32,
        ) -> xcb_dri2_authenticate_cookie_t,
    >,
    xcb_dri2_authenticate_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            magic: u32,
        ) -> xcb_dri2_authenticate_cookie_t,
    >,
    xcb_dri2_authenticate_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dri2_authenticate_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_dri2_authenticate_reply_t,
    >,
    xcb_dri2_create_drawable_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t) -> xcb_void_cookie_t,
    >,
    xcb_dri2_create_drawable: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t) -> xcb_void_cookie_t,
    >,
    xcb_dri2_destroy_drawable_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t) -> xcb_void_cookie_t,
    >,
    xcb_dri2_destroy_drawable: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t) -> xcb_void_cookie_t,
    >,
    xcb_dri2_get_buffers_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, attachments_len: u32) -> c_int>,
    xcb_dri2_get_buffers: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            count: u32,
            attachments_len: u32,
            attachments: *const u32,
        ) -> xcb_dri2_get_buffers_cookie_t,
    >,
    xcb_dri2_get_buffers_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            count: u32,
            attachments_len: u32,
            attachments: *const u32,
        ) -> xcb_dri2_get_buffers_cookie_t,
    >,
    xcb_dri2_get_buffers_buffers: LazySymbol<
        unsafe fn(r: *const xcb_dri2_get_buffers_reply_t) -> *mut xcb_dri2_dri2_buffer_t,
    >,
    xcb_dri2_get_buffers_buffers_length:
        LazySymbol<unsafe fn(r: *const xcb_dri2_get_buffers_reply_t) -> c_int>,
    xcb_dri2_get_buffers_buffers_iterator: LazySymbol<
        unsafe fn(r: *const xcb_dri2_get_buffers_reply_t) -> xcb_dri2_dri2_buffer_iterator_t,
    >,
    xcb_dri2_get_buffers_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dri2_get_buffers_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_dri2_get_buffers_reply_t,
    >,
    xcb_dri2_copy_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            region: u32,
            dest: u32,
            src: u32,
        ) -> xcb_dri2_copy_region_cookie_t,
    >,
    xcb_dri2_copy_region_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            region: u32,
            dest: u32,
            src: u32,
        ) -> xcb_dri2_copy_region_cookie_t,
    >,
    xcb_dri2_copy_region_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dri2_copy_region_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_dri2_copy_region_reply_t,
    >,
    xcb_dri2_get_buffers_with_format_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, attachments_len: u32) -> c_int>,
    xcb_dri2_get_buffers_with_format: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            count: u32,
            attachments_len: u32,
            attachments: *const xcb_dri2_attach_format_t,
        ) -> xcb_dri2_get_buffers_with_format_cookie_t,
    >,
    xcb_dri2_get_buffers_with_format_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            count: u32,
            attachments_len: u32,
            attachments: *const xcb_dri2_attach_format_t,
        ) -> xcb_dri2_get_buffers_with_format_cookie_t,
    >,
    xcb_dri2_get_buffers_with_format_buffers: LazySymbol<
        unsafe fn(
            r: *const xcb_dri2_get_buffers_with_format_reply_t,
        ) -> *mut xcb_dri2_dri2_buffer_t,
    >,
    xcb_dri2_get_buffers_with_format_buffers_length:
        LazySymbol<unsafe fn(r: *const xcb_dri2_get_buffers_with_format_reply_t) -> c_int>,
    xcb_dri2_get_buffers_with_format_buffers_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_dri2_get_buffers_with_format_reply_t,
        ) -> xcb_dri2_dri2_buffer_iterator_t,
    >,
    xcb_dri2_get_buffers_with_format_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dri2_get_buffers_with_format_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_dri2_get_buffers_with_format_reply_t,
    >,
    xcb_dri2_swap_buffers: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            target_msc_hi: u32,
            target_msc_lo: u32,
            divisor_hi: u32,
            divisor_lo: u32,
            remainder_hi: u32,
            remainder_lo: u32,
        ) -> xcb_dri2_swap_buffers_cookie_t,
    >,
    xcb_dri2_swap_buffers_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            target_msc_hi: u32,
            target_msc_lo: u32,
            divisor_hi: u32,
            divisor_lo: u32,
            remainder_hi: u32,
            remainder_lo: u32,
        ) -> xcb_dri2_swap_buffers_cookie_t,
    >,
    xcb_dri2_swap_buffers_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dri2_swap_buffers_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_dri2_swap_buffers_reply_t,
    >,
    xcb_dri2_get_msc: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t) -> xcb_dri2_get_msc_cookie_t,
    >,
    xcb_dri2_get_msc_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t) -> xcb_dri2_get_msc_cookie_t,
    >,
    xcb_dri2_get_msc_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dri2_get_msc_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_dri2_get_msc_reply_t,
    >,
    xcb_dri2_wait_msc: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            target_msc_hi: u32,
            target_msc_lo: u32,
            divisor_hi: u32,
            divisor_lo: u32,
            remainder_hi: u32,
            remainder_lo: u32,
        ) -> xcb_dri2_wait_msc_cookie_t,
    >,
    xcb_dri2_wait_msc_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            target_msc_hi: u32,
            target_msc_lo: u32,
            divisor_hi: u32,
            divisor_lo: u32,
            remainder_hi: u32,
            remainder_lo: u32,
        ) -> xcb_dri2_wait_msc_cookie_t,
    >,
    xcb_dri2_wait_msc_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dri2_wait_msc_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_dri2_wait_msc_reply_t,
    >,
    xcb_dri2_wait_sbc: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            target_sbc_hi: u32,
            target_sbc_lo: u32,
        ) -> xcb_dri2_wait_sbc_cookie_t,
    >,
    xcb_dri2_wait_sbc_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            target_sbc_hi: u32,
            target_sbc_lo: u32,
        ) -> xcb_dri2_wait_sbc_cookie_t,
    >,
    xcb_dri2_wait_sbc_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dri2_wait_sbc_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_dri2_wait_sbc_reply_t,
    >,
    xcb_dri2_swap_interval_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            interval: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_dri2_swap_interval: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            interval: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_dri2_get_param: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            param: u32,
        ) -> xcb_dri2_get_param_cookie_t,
    >,
    xcb_dri2_get_param_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            param: u32,
        ) -> xcb_dri2_get_param_cookie_t,
    >,
    xcb_dri2_get_param_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dri2_get_param_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_dri2_get_param_reply_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self.dri2.$f.get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .dri2
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_dri2")]
impl XcbDri2 {
    /// The libxcb identifier of the `DRI2` extension.
    #[inline]
    pub fn xcb_dri2_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_dri2_id) }
    }

    /// Returns `true` iff the symbol `xcb_dri2_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_id(&self) -> bool {
        has_sym!(self, xcb_dri2_id)
    }

    /// Advances a `xcb_dri2_dri2_buffer_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_dri2_dri2_buffer_next(&self, i: *mut xcb_dri2_dri2_buffer_iterator_t) {
        sym!(self, xcb_dri2_dri2_buffer_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_dri2_dri2_buffer_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_dri2_buffer_next(&self) -> bool {
        has_sym!(self, xcb_dri2_dri2_buffer_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_dri2_dri2_buffer_iterator_t`.
    #[inline]
    pub unsafe fn xcb_dri2_dri2_buffer_end(
        &self,
        i: xcb_dri2_dri2_buffer_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_dri2_dri2_buffer_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_dri2_dri2_buffer_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_dri2_buffer_end(&self) -> bool {
        has_sym!(self, xcb_dri2_dri2_buffer_end)
    }

    /// Advances a `xcb_dri2_attach_format_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_dri2_attach_format_next(&self, i: *mut xcb_dri2_attach_format_iterator_t) {
        sym!(self, xcb_dri2_attach_format_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_dri2_attach_format_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_attach_format_next(&self) -> bool {
        has_sym!(self, xcb_dri2_attach_format_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_dri2_attach_format_iterator_t`.
    #[inline]
    pub unsafe fn xcb_dri2_attach_format_end(
        &self,
        i: xcb_dri2_attach_format_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_dri2_attach_format_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_dri2_attach_format_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_attach_format_end(&self) -> bool {
        has_sym!(self, xcb_dri2_attach_format_end)
    }

    /// Sends a `DRI2::QueryVersion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri2_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri2_query_version_reply`]: Self::xcb_dri2_query_version_reply
    #[inline]
    pub unsafe fn xcb_dri2_query_version(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
    ) -> xcb_dri2_query_version_cookie_t {
        sym!(self, xcb_dri2_query_version)(c, major_version, minor_version)
    }

    /// Returns `true` iff the symbol `xcb_dri2_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_query_version(&self) -> bool {
        has_sym!(self, xcb_dri2_query_version)
    }

    /// Sends a `DRI2::QueryVersion` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri2_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri2_query_version_reply`]: Self::xcb_dri2_query_version_reply
    #[inline]
    pub unsafe fn xcb_dri2_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
    ) -> xcb_dri2_query_version_cookie_t {
        sym!(self, xcb_dri2_query_version_unchecked)(c, major_version, minor_version)
    }

    /// Returns `true` iff the symbol `xcb_dri2_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_query_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_dri2_query_version_unchecked)
    }

    /// Waits for the reply to a `DRI2::QueryVersion` request.
    #[inline]
    pub unsafe fn xcb_dri2_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri2_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri2_query_version_reply_t {
        sym!(self, xcb_dri2_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dri2_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_dri2_query_version_reply)
    }

    /// Computes the size of a `xcb_dri2_connect_reply_t` object.
    #[inline]
    pub unsafe fn xcb_dri2_connect_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_dri2_connect_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_dri2_connect_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_connect_sizeof(&self) -> bool {
        has_sym!(self, xcb_dri2_connect_sizeof)
    }

    /// Sends a `DRI2::Connect` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri2_connect_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri2_connect_reply`]: Self::xcb_dri2_connect_reply
    #[inline]
    pub unsafe fn xcb_dri2_connect(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        driver_type: u32,
    ) -> xcb_dri2_connect_cookie_t {
        sym!(self, xcb_dri2_connect)(c, window, driver_type)
    }

    /// Returns `true` iff the symbol `xcb_dri2_connect` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_connect(&self) -> bool {
        has_sym!(self, xcb_dri2_connect)
    }

    /// Sends a `DRI2::Connect` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri2_connect_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri2_connect_reply`]: Self::xcb_dri2_connect_reply
    #[inline]
    pub unsafe fn xcb_dri2_connect_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        driver_type: u32,
    ) -> xcb_dri2_connect_cookie_t {
        sym!(self, xcb_dri2_connect_unchecked)(c, window, driver_type)
    }

    /// Returns `true` iff the symbol `xcb_dri2_connect_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_connect_unchecked(&self) -> bool {
        has_sym!(self, xcb_dri2_connect_unchecked)
    }

    /// Returns a pointer to the `driver_name` field of a `xcb_dri2_connect_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_dri2_connect_driver_name(
        &self,
        r: *const xcb_dri2_connect_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_dri2_connect_driver_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri2_connect_driver_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_connect_driver_name(&self) -> bool {
        has_sym!(self, xcb_dri2_connect_driver_name)
    }

    /// Returns the number of elements of the `driver_name` field of a `xcb_dri2_connect_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_dri2_connect_driver_name_length(
        &self,
        r: *const xcb_dri2_connect_reply_t,
    ) -> c_int {
        sym!(self, xcb_dri2_connect_driver_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri2_connect_driver_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_connect_driver_name_length(&self) -> bool {
        has_sym!(self, xcb_dri2_connect_driver_name_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `driver_name` field of a `xcb_dri2_connect_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_dri2_connect_driver_name_end(
        &self,
        r: *const xcb_dri2_connect_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_dri2_connect_driver_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri2_connect_driver_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_connect_driver_name_end(&self) -> bool {
        has_sym!(self, xcb_dri2_connect_driver_name_end)
    }

    /// Returns a pointer to the `alignment_pad` field of a `xcb_dri2_connect_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_dri2_connect_alignment_pad(
        &self,
        r: *const xcb_dri2_connect_reply_t,
    ) -> *mut c_void {
        sym!(self, xcb_dri2_connect_alignment_pad)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri2_connect_alignment_pad` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_connect_alignment_pad(&self) -> bool {
        has_sym!(self, xcb_dri2_connect_alignment_pad)
    }

    /// Returns the number of elements of the `alignment_pad` field of a `xcb_dri2_connect_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_dri2_connect_alignment_pad_length(
        &self,
        r: *const xcb_dri2_connect_reply_t,
    ) -> c_int {
        sym!(self, xcb_dri2_connect_alignment_pad_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri2_connect_alignment_pad_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_connect_alignment_pad_length(&self) -> bool {
        has_sym!(self, xcb_dri2_connect_alignment_pad_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `alignment_pad` field of a `xcb_dri2_connect_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_dri2_connect_alignment_pad_end(
        &self,
        r: *const xcb_dri2_connect_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_dri2_connect_alignment_pad_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri2_connect_alignment_pad_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_connect_alignment_pad_end(&self) -> bool {
        has_sym!(self, xcb_dri2_connect_alignment_pad_end)
    }

    /// Returns a pointer to the `device_name` field of a `xcb_dri2_connect_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_dri2_connect_device_name(
        &self,
        r: *const xcb_dri2_connect_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_dri2_connect_device_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri2_connect_device_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_connect_device_name(&self) -> bool {
        has_sym!(self, xcb_dri2_connect_device_name)
    }

    /// Returns the number of elements of the `device_name` field of a `xcb_dri2_connect_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_dri2_connect_device_name_length(
        &self,
        r: *const xcb_dri2_connect_reply_t,
    ) -> c_int {
        sym!(self, xcb_dri2_connect_device_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri2_connect_device_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_connect_device_name_length(&self) -> bool {
        has_sym!(self, xcb_dri2_connect_device_name_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `device_name` field of a `xcb_dri2_connect_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_dri2_connect_device_name_end(
        &self,
        r: *const xcb_dri2_connect_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_dri2_connect_device_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri2_connect_device_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_connect_device_name_end(&self) -> bool {
        has_sym!(self, xcb_dri2_connect_device_name_end)
    }

    /// Waits for the reply to a `DRI2::Connect` request.
    #[inline]
    pub unsafe fn xcb_dri2_connect_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri2_connect_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri2_connect_reply_t {
        sym!(self, xcb_dri2_connect_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dri2_connect_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_connect_reply(&self) -> bool {
        has_sym!(self, xcb_dri2_connect_reply)
    }

    /// Sends a `DRI2::Authenticate` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri2_authenticate_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri2_authenticate_reply`]: Self::xcb_dri2_authenticate_reply
    #[inline]
    pub unsafe fn xcb_dri2_authenticate(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        magic: u32,
    ) -> xcb_dri2_authenticate_cookie_t {
        sym!(self, xcb_dri2_authenticate)(c, window, magic)
    }

    /// Returns `true` iff the symbol `xcb_dri2_authenticate` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_authenticate(&self) -> bool {
        has_sym!(self, xcb_dri2_authenticate)
    }

    /// Sends a `DRI2::Authenticate` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri2_authenticate_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri2_authenticate_reply`]: Self::xcb_dri2_authenticate_reply
    #[inline]
    pub unsafe fn xcb_dri2_authenticate_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        magic: u32,
    ) -> xcb_dri2_authenticate_cookie_t {
        sym!(self, xcb_dri2_authenticate_unchecked)(c, window, magic)
    }

    /// Returns `true` iff the symbol `xcb_dri2_authenticate_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_authenticate_unchecked(&self) -> bool {
        has_sym!(self, xcb_dri2_authenticate_unchecked)
    }

    /// Waits for the reply to a `DRI2::Authenticate` request.
    #[inline]
    pub unsafe fn xcb_dri2_authenticate_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri2_authenticate_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri2_authenticate_reply_t {
        sym!(self, xcb_dri2_authenticate_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dri2_authenticate_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_authenticate_reply(&self) -> bool {
        has_sym!(self, xcb_dri2_authenticate_reply)
    }

    /// Sends a `DRI2::CreateDrawable` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_dri2_create_drawable_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dri2_create_drawable_checked)(c, drawable)
    }

    /// Returns `true` iff the symbol `xcb_dri2_create_drawable_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_create_drawable_checked(&self) -> bool {
        has_sym!(self, xcb_dri2_create_drawable_checked)
    }

    /// Sends a `DRI2::CreateDrawable` request (unchecked).
    #[inline]
    pub unsafe fn xcb_dri2_create_drawable(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dri2_create_drawable)(c, drawable)
    }

    /// Returns `true` iff the symbol `xcb_dri2_create_drawable` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_create_drawable(&self) -> bool {
        has_sym!(self, xcb_dri2_create_drawable)
    }

    /// Sends a `DRI2::DestroyDrawable` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_dri2_destroy_drawable_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dri2_destroy_drawable_checked)(c, drawable)
    }

    /// Returns `true` iff the symbol `xcb_dri2_destroy_drawable_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_destroy_drawable_checked(&self) -> bool {
        has_sym!(self, xcb_dri2_destroy_drawable_checked)
    }

    /// Sends a `DRI2::DestroyDrawable` request (unchecked).
    #[inline]
    pub unsafe fn xcb_dri2_destroy_drawable(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dri2_destroy_drawable)(c, drawable)
    }

    /// Returns `true` iff the symbol `xcb_dri2_destroy_drawable` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_destroy_drawable(&self) -> bool {
        has_sym!(self, xcb_dri2_destroy_drawable)
    }

    /// Computes the size of a `xcb_dri2_get_buffers_request_t` object.
    #[inline]
    pub unsafe fn xcb_dri2_get_buffers_sizeof(
        &self,
        _buffer: *const c_void,
        attachments_len: u32,
    ) -> c_int {
        sym!(self, xcb_dri2_get_buffers_sizeof)(_buffer, attachments_len)
    }

    /// Returns `true` iff the symbol `xcb_dri2_get_buffers_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_get_buffers_sizeof(&self) -> bool {
        has_sym!(self, xcb_dri2_get_buffers_sizeof)
    }

    /// Sends a `DRI2::GetBuffers` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri2_get_buffers_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri2_get_buffers_reply`]: Self::xcb_dri2_get_buffers_reply
    #[inline]
    pub unsafe fn xcb_dri2_get_buffers(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        count: u32,
        attachments_len: u32,
        attachments: *const u32,
    ) -> xcb_dri2_get_buffers_cookie_t {
        sym!(self, xcb_dri2_get_buffers)(c, drawable, count, attachments_len, attachments)
    }

    /// Returns `true` iff the symbol `xcb_dri2_get_buffers` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_get_buffers(&self) -> bool {
        has_sym!(self, xcb_dri2_get_buffers)
    }

    /// Sends a `DRI2::GetBuffers` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri2_get_buffers_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri2_get_buffers_reply`]: Self::xcb_dri2_get_buffers_reply
    #[inline]
    pub unsafe fn xcb_dri2_get_buffers_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        count: u32,
        attachments_len: u32,
        attachments: *const u32,
    ) -> xcb_dri2_get_buffers_cookie_t {
        sym!(self, xcb_dri2_get_buffers_unchecked)(c, drawable, count, attachments_len, attachments)
    }

    /// Returns `true` iff the symbol `xcb_dri2_get_buffers_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_get_buffers_unchecked(&self) -> bool {
        has_sym!(self, xcb_dri2_get_buffers_unchecked)
    }

    /// Returns a pointer to the `buffers` field of a `xcb_dri2_get_buffers_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_dri2_get_buffers_buffers(
        &self,
        r: *const xcb_dri2_get_buffers_reply_t,
    ) -> *mut xcb_dri2_dri2_buffer_t {
        sym!(self, xcb_dri2_get_buffers_buffers)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri2_get_buffers_buffers` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_get_buffers_buffers(&self) -> bool {
        has_sym!(self, xcb_dri2_get_buffers_buffers)
    }

    /// Returns the number of elements of the `buffers` field of a `xcb_dri2_get_buffers_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_dri2_get_buffers_buffers_length(
        &self,
        r: *const xcb_dri2_get_buffers_reply_t,
    ) -> c_int {
        sym!(self, xcb_dri2_get_buffers_buffers_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri2_get_buffers_buffers_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_get_buffers_buffers_length(&self) -> bool {
        has_sym!(self, xcb_dri2_get_buffers_buffers_length)
    }

    /// Returns an iterator over the elements of the
    /// `buffers` field of a `xcb_dri2_get_buffers_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_dri2_get_buffers_buffers_iterator(
        &self,
        r: *const xcb_dri2_get_buffers_reply_t,
    ) -> xcb_dri2_dri2_buffer_iterator_t {
        sym!(self, xcb_dri2_get_buffers_buffers_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri2_get_buffers_buffers_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_get_buffers_buffers_iterator(&self) -> bool {
        has_sym!(self, xcb_dri2_get_buffers_buffers_iterator)
    }

    /// Waits for the reply to a `DRI2::GetBuffers` request.
    #[inline]
    pub unsafe fn xcb_dri2_get_buffers_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri2_get_buffers_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri2_get_buffers_reply_t {
        sym!(self, xcb_dri2_get_buffers_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dri2_get_buffers_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_get_buffers_reply(&self) -> bool {
        has_sym!(self, xcb_dri2_get_buffers_reply)
    }

    /// Sends a `DRI2::CopyRegion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri2_copy_region_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri2_copy_region_reply`]: Self::xcb_dri2_copy_region_reply
    #[inline]
    pub unsafe fn xcb_dri2_copy_region(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        region: u32,
        dest: u32,
        src: u32,
    ) -> xcb_dri2_copy_region_cookie_t {
        sym!(self, xcb_dri2_copy_region)(c, drawable, region, dest, src)
    }

    /// Returns `true` iff the symbol `xcb_dri2_copy_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_copy_region(&self) -> bool {
        has_sym!(self, xcb_dri2_copy_region)
    }

    /// Sends a `DRI2::CopyRegion` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri2_copy_region_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri2_copy_region_reply`]: Self::xcb_dri2_copy_region_reply
    #[inline]
    pub unsafe fn xcb_dri2_copy_region_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        region: u32,
        dest: u32,
        src: u32,
    ) -> xcb_dri2_copy_region_cookie_t {
        sym!(self, xcb_dri2_copy_region_unchecked)(c, drawable, region, dest, src)
    }

    /// Returns `true` iff the symbol `xcb_dri2_copy_region_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_copy_region_unchecked(&self) -> bool {
        has_sym!(self, xcb_dri2_copy_region_unchecked)
    }

    /// Waits for the reply to a `DRI2::CopyRegion` request.
    #[inline]
    pub unsafe fn xcb_dri2_copy_region_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri2_copy_region_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri2_copy_region_reply_t {
        sym!(self, xcb_dri2_copy_region_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dri2_copy_region_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_copy_region_reply(&self) -> bool {
        has_sym!(self, xcb_dri2_copy_region_reply)
    }

    /// Computes the size of a `xcb_dri2_get_buffers_with_format_request_t` object.
    #[inline]
    pub unsafe fn xcb_dri2_get_buffers_with_format_sizeof(
        &self,
        _buffer: *const c_void,
        attachments_len: u32,
    ) -> c_int {
        sym!(self, xcb_dri2_get_buffers_with_format_sizeof)(_buffer, attachments_len)
    }

    /// Returns `true` iff the symbol `xcb_dri2_get_buffers_with_format_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_get_buffers_with_format_sizeof(&self) -> bool {
        has_sym!(self, xcb_dri2_get_buffers_with_format_sizeof)
    }

    /// Sends a `DRI2::GetBuffersWithFormat` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri2_get_buffers_with_format_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri2_get_buffers_with_format_reply`]: Self::xcb_dri2_get_buffers_with_format_reply
    #[inline]
    pub unsafe fn xcb_dri2_get_buffers_with_format(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        count: u32,
        attachments_len: u32,
        attachments: *const xcb_dri2_attach_format_t,
    ) -> xcb_dri2_get_buffers_with_format_cookie_t {
        sym!(self, xcb_dri2_get_buffers_with_format)(
            c,
            drawable,
            count,
            attachments_len,
            attachments,
        )
    }

    /// Returns `true` iff the symbol `xcb_dri2_get_buffers_with_format` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_get_buffers_with_format(&self) -> bool {
        has_sym!(self, xcb_dri2_get_buffers_with_format)
    }

    /// Sends a `DRI2::GetBuffersWithFormat` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri2_get_buffers_with_format_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri2_get_buffers_with_format_reply`]: Self::xcb_dri2_get_buffers_with_format_reply
    #[inline]
    pub unsafe fn xcb_dri2_get_buffers_with_format_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        count: u32,
        attachments_len: u32,
        attachments: *const xcb_dri2_attach_format_t,
    ) -> xcb_dri2_get_buffers_with_format_cookie_t {
        sym!(self, xcb_dri2_get_buffers_with_format_unchecked)(
            c,
            drawable,
            count,
            attachments_len,
            attachments,
        )
    }

    /// Returns `true` iff the symbol `xcb_dri2_get_buffers_with_format_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_get_buffers_with_format_unchecked(&self) -> bool {
        has_sym!(self, xcb_dri2_get_buffers_with_format_unchecked)
    }

    /// Returns a pointer to the `buffers` field of a `xcb_dri2_get_buffers_with_format_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_dri2_get_buffers_with_format_buffers(
        &self,
        r: *const xcb_dri2_get_buffers_with_format_reply_t,
    ) -> *mut xcb_dri2_dri2_buffer_t {
        sym!(self, xcb_dri2_get_buffers_with_format_buffers)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri2_get_buffers_with_format_buffers` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_get_buffers_with_format_buffers(&self) -> bool {
        has_sym!(self, xcb_dri2_get_buffers_with_format_buffers)
    }

    /// Returns the number of elements of the `buffers` field of a `xcb_dri2_get_buffers_with_format_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_dri2_get_buffers_with_format_buffers_length(
        &self,
        r: *const xcb_dri2_get_buffers_with_format_reply_t,
    ) -> c_int {
        sym!(self, xcb_dri2_get_buffers_with_format_buffers_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri2_get_buffers_with_format_buffers_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_get_buffers_with_format_buffers_length(&self) -> bool {
        has_sym!(self, xcb_dri2_get_buffers_with_format_buffers_length)
    }

    /// Returns an iterator over the elements of the
    /// `buffers` field of a `xcb_dri2_get_buffers_with_format_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_dri2_get_buffers_with_format_buffers_iterator(
        &self,
        r: *const xcb_dri2_get_buffers_with_format_reply_t,
    ) -> xcb_dri2_dri2_buffer_iterator_t {
        sym!(self, xcb_dri2_get_buffers_with_format_buffers_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_dri2_get_buffers_with_format_buffers_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_get_buffers_with_format_buffers_iterator(&self) -> bool {
        has_sym!(self, xcb_dri2_get_buffers_with_format_buffers_iterator)
    }

    /// Waits for the reply to a `DRI2::GetBuffersWithFormat` request.
    #[inline]
    pub unsafe fn xcb_dri2_get_buffers_with_format_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri2_get_buffers_with_format_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri2_get_buffers_with_format_reply_t {
        sym!(self, xcb_dri2_get_buffers_with_format_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dri2_get_buffers_with_format_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_get_buffers_with_format_reply(&self) -> bool {
        has_sym!(self, xcb_dri2_get_buffers_with_format_reply)
    }

    /// Sends a `DRI2::SwapBuffers` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri2_swap_buffers_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri2_swap_buffers_reply`]: Self::xcb_dri2_swap_buffers_reply
    #[inline]
    pub unsafe fn xcb_dri2_swap_buffers(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        target_msc_hi: u32,
        target_msc_lo: u32,
        divisor_hi: u32,
        divisor_lo: u32,
        remainder_hi: u32,
        remainder_lo: u32,
    ) -> xcb_dri2_swap_buffers_cookie_t {
        sym!(self, xcb_dri2_swap_buffers)(
            c,
            drawable,
            target_msc_hi,
            target_msc_lo,
            divisor_hi,
            divisor_lo,
            remainder_hi,
            remainder_lo,
        )
    }

    /// Returns `true` iff the symbol `xcb_dri2_swap_buffers` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_swap_buffers(&self) -> bool {
        has_sym!(self, xcb_dri2_swap_buffers)
    }

    /// Sends a `DRI2::SwapBuffers` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri2_swap_buffers_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri2_swap_buffers_reply`]: Self::xcb_dri2_swap_buffers_reply
    #[inline]
    pub unsafe fn xcb_dri2_swap_buffers_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        target_msc_hi: u32,
        target_msc_lo: u32,
        divisor_hi: u32,
        divisor_lo: u32,
        remainder_hi: u32,
        remainder_lo: u32,
    ) -> xcb_dri2_swap_buffers_cookie_t {
        sym!(self, xcb_dri2_swap_buffers_unchecked)(
            c,
            drawable,
            target_msc_hi,
            target_msc_lo,
            divisor_hi,
            divisor_lo,
            remainder_hi,
            remainder_lo,
        )
    }

    /// Returns `true` iff the symbol `xcb_dri2_swap_buffers_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_swap_buffers_unchecked(&self) -> bool {
        has_sym!(self, xcb_dri2_swap_buffers_unchecked)
    }

    /// Waits for the reply to a `DRI2::SwapBuffers` request.
    #[inline]
    pub unsafe fn xcb_dri2_swap_buffers_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri2_swap_buffers_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri2_swap_buffers_reply_t {
        sym!(self, xcb_dri2_swap_buffers_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dri2_swap_buffers_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_swap_buffers_reply(&self) -> bool {
        has_sym!(self, xcb_dri2_swap_buffers_reply)
    }

    /// Sends a `DRI2::GetMSC` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri2_get_msc_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri2_get_msc_reply`]: Self::xcb_dri2_get_msc_reply
    #[inline]
    pub unsafe fn xcb_dri2_get_msc(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_dri2_get_msc_cookie_t {
        sym!(self, xcb_dri2_get_msc)(c, drawable)
    }

    /// Returns `true` iff the symbol `xcb_dri2_get_msc` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_get_msc(&self) -> bool {
        has_sym!(self, xcb_dri2_get_msc)
    }

    /// Sends a `DRI2::GetMSC` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri2_get_msc_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri2_get_msc_reply`]: Self::xcb_dri2_get_msc_reply
    #[inline]
    pub unsafe fn xcb_dri2_get_msc_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_dri2_get_msc_cookie_t {
        sym!(self, xcb_dri2_get_msc_unchecked)(c, drawable)
    }

    /// Returns `true` iff the symbol `xcb_dri2_get_msc_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_get_msc_unchecked(&self) -> bool {
        has_sym!(self, xcb_dri2_get_msc_unchecked)
    }

    /// Waits for the reply to a `DRI2::GetMSC` request.
    #[inline]
    pub unsafe fn xcb_dri2_get_msc_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri2_get_msc_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri2_get_msc_reply_t {
        sym!(self, xcb_dri2_get_msc_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dri2_get_msc_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_get_msc_reply(&self) -> bool {
        has_sym!(self, xcb_dri2_get_msc_reply)
    }

    /// Sends a `DRI2::WaitMSC` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri2_wait_msc_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri2_wait_msc_reply`]: Self::xcb_dri2_wait_msc_reply
    #[inline]
    pub unsafe fn xcb_dri2_wait_msc(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        target_msc_hi: u32,
        target_msc_lo: u32,
        divisor_hi: u32,
        divisor_lo: u32,
        remainder_hi: u32,
        remainder_lo: u32,
    ) -> xcb_dri2_wait_msc_cookie_t {
        sym!(self, xcb_dri2_wait_msc)(
            c,
            drawable,
            target_msc_hi,
            target_msc_lo,
            divisor_hi,
            divisor_lo,
            remainder_hi,
            remainder_lo,
        )
    }

    /// Returns `true` iff the symbol `xcb_dri2_wait_msc` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_wait_msc(&self) -> bool {
        has_sym!(self, xcb_dri2_wait_msc)
    }

    /// Sends a `DRI2::WaitMSC` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri2_wait_msc_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri2_wait_msc_reply`]: Self::xcb_dri2_wait_msc_reply
    #[inline]
    pub unsafe fn xcb_dri2_wait_msc_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        target_msc_hi: u32,
        target_msc_lo: u32,
        divisor_hi: u32,
        divisor_lo: u32,
        remainder_hi: u32,
        remainder_lo: u32,
    ) -> xcb_dri2_wait_msc_cookie_t {
        sym!(self, xcb_dri2_wait_msc_unchecked)(
            c,
            drawable,
            target_msc_hi,
            target_msc_lo,
            divisor_hi,
            divisor_lo,
            remainder_hi,
            remainder_lo,
        )
    }

    /// Returns `true` iff the symbol `xcb_dri2_wait_msc_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_wait_msc_unchecked(&self) -> bool {
        has_sym!(self, xcb_dri2_wait_msc_unchecked)
    }

    /// Waits for the reply to a `DRI2::WaitMSC` request.
    #[inline]
    pub unsafe fn xcb_dri2_wait_msc_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri2_wait_msc_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri2_wait_msc_reply_t {
        sym!(self, xcb_dri2_wait_msc_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dri2_wait_msc_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_wait_msc_reply(&self) -> bool {
        has_sym!(self, xcb_dri2_wait_msc_reply)
    }

    /// Sends a `DRI2::WaitSBC` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri2_wait_sbc_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri2_wait_sbc_reply`]: Self::xcb_dri2_wait_sbc_reply
    #[inline]
    pub unsafe fn xcb_dri2_wait_sbc(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        target_sbc_hi: u32,
        target_sbc_lo: u32,
    ) -> xcb_dri2_wait_sbc_cookie_t {
        sym!(self, xcb_dri2_wait_sbc)(c, drawable, target_sbc_hi, target_sbc_lo)
    }

    /// Returns `true` iff the symbol `xcb_dri2_wait_sbc` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_wait_sbc(&self) -> bool {
        has_sym!(self, xcb_dri2_wait_sbc)
    }

    /// Sends a `DRI2::WaitSBC` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri2_wait_sbc_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri2_wait_sbc_reply`]: Self::xcb_dri2_wait_sbc_reply
    #[inline]
    pub unsafe fn xcb_dri2_wait_sbc_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        target_sbc_hi: u32,
        target_sbc_lo: u32,
    ) -> xcb_dri2_wait_sbc_cookie_t {
        sym!(self, xcb_dri2_wait_sbc_unchecked)(c, drawable, target_sbc_hi, target_sbc_lo)
    }

    /// Returns `true` iff the symbol `xcb_dri2_wait_sbc_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_wait_sbc_unchecked(&self) -> bool {
        has_sym!(self, xcb_dri2_wait_sbc_unchecked)
    }

    /// Waits for the reply to a `DRI2::WaitSBC` request.
    #[inline]
    pub unsafe fn xcb_dri2_wait_sbc_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri2_wait_sbc_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri2_wait_sbc_reply_t {
        sym!(self, xcb_dri2_wait_sbc_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dri2_wait_sbc_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_wait_sbc_reply(&self) -> bool {
        has_sym!(self, xcb_dri2_wait_sbc_reply)
    }

    /// Sends a `DRI2::SwapInterval` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_dri2_swap_interval_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        interval: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dri2_swap_interval_checked)(c, drawable, interval)
    }

    /// Returns `true` iff the symbol `xcb_dri2_swap_interval_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_swap_interval_checked(&self) -> bool {
        has_sym!(self, xcb_dri2_swap_interval_checked)
    }

    /// Sends a `DRI2::SwapInterval` request (unchecked).
    #[inline]
    pub unsafe fn xcb_dri2_swap_interval(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        interval: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dri2_swap_interval)(c, drawable, interval)
    }

    /// Returns `true` iff the symbol `xcb_dri2_swap_interval` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_swap_interval(&self) -> bool {
        has_sym!(self, xcb_dri2_swap_interval)
    }

    /// Sends a `DRI2::GetParam` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri2_get_param_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri2_get_param_reply`]: Self::xcb_dri2_get_param_reply
    #[inline]
    pub unsafe fn xcb_dri2_get_param(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        param: u32,
    ) -> xcb_dri2_get_param_cookie_t {
        sym!(self, xcb_dri2_get_param)(c, drawable, param)
    }

    /// Returns `true` iff the symbol `xcb_dri2_get_param` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_get_param(&self) -> bool {
        has_sym!(self, xcb_dri2_get_param)
    }

    /// Sends a `DRI2::GetParam` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_dri2_get_param_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_dri2_get_param_reply`]: Self::xcb_dri2_get_param_reply
    #[inline]
    pub unsafe fn xcb_dri2_get_param_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        param: u32,
    ) -> xcb_dri2_get_param_cookie_t {
        sym!(self, xcb_dri2_get_param_unchecked)(c, drawable, param)
    }

    /// Returns `true` iff the symbol `xcb_dri2_get_param_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_get_param_unchecked(&self) -> bool {
        has_sym!(self, xcb_dri2_get_param_unchecked)
    }

    /// Waits for the reply to a `DRI2::GetParam` request.
    #[inline]
    pub unsafe fn xcb_dri2_get_param_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri2_get_param_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri2_get_param_reply_t {
        sym!(self, xcb_dri2_get_param_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dri2_get_param_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dri2_get_param_reply(&self) -> bool {
        has_sym!(self, xcb_dri2_get_param_reply)
    }
}

#[cfg(feature = "xcb_dri2")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbDri2::load().unwrap() };
        assert!(lib.has_xcb_dri2_id());
        assert!(lib.has_xcb_dri2_dri2_buffer_next());
        assert!(lib.has_xcb_dri2_dri2_buffer_end());
        assert!(lib.has_xcb_dri2_attach_format_next());
        assert!(lib.has_xcb_dri2_attach_format_end());
        assert!(lib.has_xcb_dri2_query_version());
        assert!(lib.has_xcb_dri2_query_version_unchecked());
        assert!(lib.has_xcb_dri2_query_version_reply());
        assert!(lib.has_xcb_dri2_connect_sizeof());
        assert!(lib.has_xcb_dri2_connect());
        assert!(lib.has_xcb_dri2_connect_unchecked());
        assert!(lib.has_xcb_dri2_connect_driver_name());
        assert!(lib.has_xcb_dri2_connect_driver_name_length());
        assert!(lib.has_xcb_dri2_connect_driver_name_end());
        assert!(lib.has_xcb_dri2_connect_alignment_pad());
        assert!(lib.has_xcb_dri2_connect_alignment_pad_length());
        assert!(lib.has_xcb_dri2_connect_alignment_pad_end());
        assert!(lib.has_xcb_dri2_connect_device_name());
        assert!(lib.has_xcb_dri2_connect_device_name_length());
        assert!(lib.has_xcb_dri2_connect_device_name_end());
        assert!(lib.has_xcb_dri2_connect_reply());
        assert!(lib.has_xcb_dri2_authenticate());
        assert!(lib.has_xcb_dri2_authenticate_unchecked());
        assert!(lib.has_xcb_dri2_authenticate_reply());
        assert!(lib.has_xcb_dri2_create_drawable_checked());
        assert!(lib.has_xcb_dri2_create_drawable());
        assert!(lib.has_xcb_dri2_destroy_drawable_checked());
        assert!(lib.has_xcb_dri2_destroy_drawable());
        assert!(lib.has_xcb_dri2_get_buffers_sizeof());
        assert!(lib.has_xcb_dri2_get_buffers());
        assert!(lib.has_xcb_dri2_get_buffers_unchecked());
        assert!(lib.has_xcb_dri2_get_buffers_buffers());
        assert!(lib.has_xcb_dri2_get_buffers_buffers_length());
        assert!(lib.has_xcb_dri2_get_buffers_buffers_iterator());
        assert!(lib.has_xcb_dri2_get_buffers_reply());
        assert!(lib.has_xcb_dri2_copy_region());
        assert!(lib.has_xcb_dri2_copy_region_unchecked());
        assert!(lib.has_xcb_dri2_copy_region_reply());
        assert!(lib.has_xcb_dri2_get_buffers_with_format_sizeof());
        assert!(lib.has_xcb_dri2_get_buffers_with_format());
        assert!(lib.has_xcb_dri2_get_buffers_with_format_unchecked());
        assert!(lib.has_xcb_dri2_get_buffers_with_format_buffers());
        assert!(lib.has_xcb_dri2_get_buffers_with_format_buffers_length());
        assert!(lib.has_xcb_dri2_get_buffers_with_format_buffers_iterator());
        assert!(lib.has_xcb_dri2_get_buffers_with_format_reply());
        assert!(lib.has_xcb_dri2_swap_buffers());
        assert!(lib.has_xcb_dri2_swap_buffers_unchecked());
        assert!(lib.has_xcb_dri2_swap_buffers_reply());
        assert!(lib.has_xcb_dri2_get_msc());
        assert!(lib.has_xcb_dri2_get_msc_unchecked());
        assert!(lib.has_xcb_dri2_get_msc_reply());
        assert!(lib.has_xcb_dri2_wait_msc());
        assert!(lib.has_xcb_dri2_wait_msc_unchecked());
        assert!(lib.has_xcb_dri2_wait_msc_reply());
        assert!(lib.has_xcb_dri2_wait_sbc());
        assert!(lib.has_xcb_dri2_wait_sbc_unchecked());
        assert!(lib.has_xcb_dri2_wait_sbc_reply());
        assert!(lib.has_xcb_dri2_swap_interval_checked());
        assert!(lib.has_xcb_dri2_swap_interval());
        assert!(lib.has_xcb_dri2_get_param());
        assert!(lib.has_xcb_dri2_get_param_unchecked());
        assert!(lib.has_xcb_dri2_get_param_reply());
    }
}
