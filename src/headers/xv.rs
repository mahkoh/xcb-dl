// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The name of the `Xv` extension.
pub const XCB_XV_NAME: &[u8] = b"XVideo";

/// The name of the `Xv` extension.
pub const XCB_XV_NAME_STR: &str = "XVideo";

/// The `Xv::PORT` type.
pub type xcb_xv_port_t = u32;

/// An iterator over `Xv::PORT` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_port_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xv_port_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xv_port_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xv::ENCODING` type.
pub type xcb_xv_encoding_t = u32;

/// An iterator over `Xv::ENCODING` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_encoding_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xv_encoding_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xv_encoding_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xv::Type` enum.
///
/// This enum has the following variants:
///
/// - [`Xv::Type::InputMask`](XCB_XV_TYPE_INPUT_MASK)
/// - [`Xv::Type::OutputMask`](XCB_XV_TYPE_OUTPUT_MASK)
/// - [`Xv::Type::VideoMask`](XCB_XV_TYPE_VIDEO_MASK)
/// - [`Xv::Type::StillMask`](XCB_XV_TYPE_STILL_MASK)
/// - [`Xv::Type::ImageMask`](XCB_XV_TYPE_IMAGE_MASK)
pub type xcb_xv_type_t = u32;
/// The `Xv::Type::InputMask` enum variant.
///
/// This is a variant of [`xcb_xv_type_t`].
pub const XCB_XV_TYPE_INPUT_MASK: xcb_xv_type_t = 1;
/// The `Xv::Type::OutputMask` enum variant.
///
/// This is a variant of [`xcb_xv_type_t`].
pub const XCB_XV_TYPE_OUTPUT_MASK: xcb_xv_type_t = 2;
/// The `Xv::Type::VideoMask` enum variant.
///
/// This is a variant of [`xcb_xv_type_t`].
pub const XCB_XV_TYPE_VIDEO_MASK: xcb_xv_type_t = 4;
/// The `Xv::Type::StillMask` enum variant.
///
/// This is a variant of [`xcb_xv_type_t`].
pub const XCB_XV_TYPE_STILL_MASK: xcb_xv_type_t = 8;
/// The `Xv::Type::ImageMask` enum variant.
///
/// This is a variant of [`xcb_xv_type_t`].
pub const XCB_XV_TYPE_IMAGE_MASK: xcb_xv_type_t = 16;

/// The `Xv::ImageFormatInfoType` enum.
///
/// This enum has the following variants:
///
/// - [`Xv::ImageFormatInfoType::RGB`](XCB_XV_IMAGE_FORMAT_INFO_TYPE_RGB)
/// - [`Xv::ImageFormatInfoType::YUV`](XCB_XV_IMAGE_FORMAT_INFO_TYPE_YUV)
pub type xcb_xv_image_format_info_type_t = u32;
/// The `Xv::ImageFormatInfoType::RGB` enum variant.
///
/// This is a variant of [`xcb_xv_image_format_info_type_t`].
pub const XCB_XV_IMAGE_FORMAT_INFO_TYPE_RGB: xcb_xv_image_format_info_type_t = 0;
/// The `Xv::ImageFormatInfoType::YUV` enum variant.
///
/// This is a variant of [`xcb_xv_image_format_info_type_t`].
pub const XCB_XV_IMAGE_FORMAT_INFO_TYPE_YUV: xcb_xv_image_format_info_type_t = 1;

/// The `Xv::ImageFormatInfoFormat` enum.
///
/// This enum has the following variants:
///
/// - [`Xv::ImageFormatInfoFormat::Packed`](XCB_XV_IMAGE_FORMAT_INFO_FORMAT_PACKED)
/// - [`Xv::ImageFormatInfoFormat::Planar`](XCB_XV_IMAGE_FORMAT_INFO_FORMAT_PLANAR)
pub type xcb_xv_image_format_info_format_t = u32;
/// The `Xv::ImageFormatInfoFormat::Packed` enum variant.
///
/// This is a variant of [`xcb_xv_image_format_info_format_t`].
pub const XCB_XV_IMAGE_FORMAT_INFO_FORMAT_PACKED: xcb_xv_image_format_info_format_t = 0;
/// The `Xv::ImageFormatInfoFormat::Planar` enum variant.
///
/// This is a variant of [`xcb_xv_image_format_info_format_t`].
pub const XCB_XV_IMAGE_FORMAT_INFO_FORMAT_PLANAR: xcb_xv_image_format_info_format_t = 1;

/// The `Xv::AttributeFlag` enum.
///
/// This enum has the following variants:
///
/// - [`Xv::AttributeFlag::Gettable`](XCB_XV_ATTRIBUTE_FLAG_GETTABLE)
/// - [`Xv::AttributeFlag::Settable`](XCB_XV_ATTRIBUTE_FLAG_SETTABLE)
pub type xcb_xv_attribute_flag_t = u32;
/// The `Xv::AttributeFlag::Gettable` enum variant.
///
/// This is a variant of [`xcb_xv_attribute_flag_t`].
pub const XCB_XV_ATTRIBUTE_FLAG_GETTABLE: xcb_xv_attribute_flag_t = 1;
/// The `Xv::AttributeFlag::Settable` enum variant.
///
/// This is a variant of [`xcb_xv_attribute_flag_t`].
pub const XCB_XV_ATTRIBUTE_FLAG_SETTABLE: xcb_xv_attribute_flag_t = 2;

/// The `Xv::VideoNotifyReason` enum.
///
/// This enum has the following variants:
///
/// - [`Xv::VideoNotifyReason::Started`](XCB_XV_VIDEO_NOTIFY_REASON_STARTED)
/// - [`Xv::VideoNotifyReason::Stopped`](XCB_XV_VIDEO_NOTIFY_REASON_STOPPED)
/// - [`Xv::VideoNotifyReason::Busy`](XCB_XV_VIDEO_NOTIFY_REASON_BUSY)
/// - [`Xv::VideoNotifyReason::Preempted`](XCB_XV_VIDEO_NOTIFY_REASON_PREEMPTED)
/// - [`Xv::VideoNotifyReason::HardError`](XCB_XV_VIDEO_NOTIFY_REASON_HARD_ERROR)
pub type xcb_xv_video_notify_reason_t = u32;
/// The `Xv::VideoNotifyReason::Started` enum variant.
///
/// This is a variant of [`xcb_xv_video_notify_reason_t`].
pub const XCB_XV_VIDEO_NOTIFY_REASON_STARTED: xcb_xv_video_notify_reason_t = 0;
/// The `Xv::VideoNotifyReason::Stopped` enum variant.
///
/// This is a variant of [`xcb_xv_video_notify_reason_t`].
pub const XCB_XV_VIDEO_NOTIFY_REASON_STOPPED: xcb_xv_video_notify_reason_t = 1;
/// The `Xv::VideoNotifyReason::Busy` enum variant.
///
/// This is a variant of [`xcb_xv_video_notify_reason_t`].
pub const XCB_XV_VIDEO_NOTIFY_REASON_BUSY: xcb_xv_video_notify_reason_t = 2;
/// The `Xv::VideoNotifyReason::Preempted` enum variant.
///
/// This is a variant of [`xcb_xv_video_notify_reason_t`].
pub const XCB_XV_VIDEO_NOTIFY_REASON_PREEMPTED: xcb_xv_video_notify_reason_t = 3;
/// The `Xv::VideoNotifyReason::HardError` enum variant.
///
/// This is a variant of [`xcb_xv_video_notify_reason_t`].
pub const XCB_XV_VIDEO_NOTIFY_REASON_HARD_ERROR: xcb_xv_video_notify_reason_t = 4;

/// The `Xv::ScanlineOrder` enum.
///
/// This enum has the following variants:
///
/// - [`Xv::ScanlineOrder::TopToBottom`](XCB_XV_SCANLINE_ORDER_TOP_TO_BOTTOM)
/// - [`Xv::ScanlineOrder::BottomToTop`](XCB_XV_SCANLINE_ORDER_BOTTOM_TO_TOP)
pub type xcb_xv_scanline_order_t = u32;
/// The `Xv::ScanlineOrder::TopToBottom` enum variant.
///
/// This is a variant of [`xcb_xv_scanline_order_t`].
pub const XCB_XV_SCANLINE_ORDER_TOP_TO_BOTTOM: xcb_xv_scanline_order_t = 0;
/// The `Xv::ScanlineOrder::BottomToTop` enum variant.
///
/// This is a variant of [`xcb_xv_scanline_order_t`].
pub const XCB_XV_SCANLINE_ORDER_BOTTOM_TO_TOP: xcb_xv_scanline_order_t = 1;

/// The `Xv::GrabPortStatus` enum.
///
/// This enum has the following variants:
///
/// - [`Xv::GrabPortStatus::Success`](XCB_XV_GRAB_PORT_STATUS_SUCCESS)
/// - [`Xv::GrabPortStatus::BadExtension`](XCB_XV_GRAB_PORT_STATUS_BAD_EXTENSION)
/// - [`Xv::GrabPortStatus::AlreadyGrabbed`](XCB_XV_GRAB_PORT_STATUS_ALREADY_GRABBED)
/// - [`Xv::GrabPortStatus::InvalidTime`](XCB_XV_GRAB_PORT_STATUS_INVALID_TIME)
/// - [`Xv::GrabPortStatus::BadReply`](XCB_XV_GRAB_PORT_STATUS_BAD_REPLY)
/// - [`Xv::GrabPortStatus::BadAlloc`](XCB_XV_GRAB_PORT_STATUS_BAD_ALLOC)
pub type xcb_xv_grab_port_status_t = u32;
/// The `Xv::GrabPortStatus::Success` enum variant.
///
/// This is a variant of [`xcb_xv_grab_port_status_t`].
pub const XCB_XV_GRAB_PORT_STATUS_SUCCESS: xcb_xv_grab_port_status_t = 0;
/// The `Xv::GrabPortStatus::BadExtension` enum variant.
///
/// This is a variant of [`xcb_xv_grab_port_status_t`].
pub const XCB_XV_GRAB_PORT_STATUS_BAD_EXTENSION: xcb_xv_grab_port_status_t = 1;
/// The `Xv::GrabPortStatus::AlreadyGrabbed` enum variant.
///
/// This is a variant of [`xcb_xv_grab_port_status_t`].
pub const XCB_XV_GRAB_PORT_STATUS_ALREADY_GRABBED: xcb_xv_grab_port_status_t = 2;
/// The `Xv::GrabPortStatus::InvalidTime` enum variant.
///
/// This is a variant of [`xcb_xv_grab_port_status_t`].
pub const XCB_XV_GRAB_PORT_STATUS_INVALID_TIME: xcb_xv_grab_port_status_t = 3;
/// The `Xv::GrabPortStatus::BadReply` enum variant.
///
/// This is a variant of [`xcb_xv_grab_port_status_t`].
pub const XCB_XV_GRAB_PORT_STATUS_BAD_REPLY: xcb_xv_grab_port_status_t = 4;
/// The `Xv::GrabPortStatus::BadAlloc` enum variant.
///
/// This is a variant of [`xcb_xv_grab_port_status_t`].
pub const XCB_XV_GRAB_PORT_STATUS_BAD_ALLOC: xcb_xv_grab_port_status_t = 5;

/// The `Xv::Rational` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_rational_t {
    pub numerator: i32,
    pub denominator: i32,
}

impl Default for xcb_xv_rational_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Xv::Rational` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_rational_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xv_rational_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xv_rational_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xv::Format` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_format_t {
    pub visual: xcb_visualid_t,
    pub depth: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_xv_format_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Xv::Format` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_format_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xv_format_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xv_format_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xv::AdaptorInfo` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `name`
/// - `formats`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_adaptor_info_t {
    pub base_id: xcb_xv_port_t,
    pub name_size: u16,
    pub num_ports: u16,
    pub num_formats: u16,
    pub type_: u8,
    pub pad0: u8,
}

impl Default for xcb_xv_adaptor_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Xv::AdaptorInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_adaptor_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xv_adaptor_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xv_adaptor_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xv::EncodingInfo` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `name`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_encoding_info_t {
    pub encoding: xcb_xv_encoding_t,
    pub name_size: u16,
    pub width: u16,
    pub height: u16,
    pub pad0: [u8; 2],
    pub rate: xcb_xv_rational_t,
}

impl Default for xcb_xv_encoding_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Xv::EncodingInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_encoding_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xv_encoding_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xv_encoding_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xv::Image` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `pitches`
/// - `offsets`
/// - `data`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_image_t {
    pub id: u32,
    pub width: u16,
    pub height: u16,
    pub data_size: u32,
    pub num_planes: u32,
}

impl Default for xcb_xv_image_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Xv::Image` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_image_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xv_image_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xv_image_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xv::AttributeInfo` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `name`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_attribute_info_t {
    pub flags: u32,
    pub min: i32,
    pub max: i32,
    pub size: u32,
}

impl Default for xcb_xv_attribute_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Xv::AttributeInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_attribute_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xv_attribute_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xv_attribute_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xv::ImageFormatInfo` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_image_format_info_t {
    pub id: u32,
    pub type_: u8,
    pub byte_order: u8,
    pub pad0: [u8; 2],
    pub guid: [u8; 16],
    pub bpp: u8,
    pub num_planes: u8,
    pub pad1: [u8; 2],
    pub depth: u8,
    pub pad2: [u8; 3],
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub format: u8,
    pub pad3: [u8; 3],
    pub y_sample_bits: u32,
    pub u_sample_bits: u32,
    pub v_sample_bits: u32,
    pub vhorz_y_period: u32,
    pub vhorz_u_period: u32,
    pub vhorz_v_period: u32,
    pub vvert_y_period: u32,
    pub vvert_u_period: u32,
    pub vvert_v_period: u32,
    pub vcomp_order: [u8; 32],
    pub vscanline_order: u8,
    pub pad4: [u8; 11],
}

impl Default for xcb_xv_image_format_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Xv::ImageFormatInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_image_format_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xv_image_format_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xv_image_format_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xv::BadPort` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_xv_bad_port_error_t`].
pub const XCB_XV_BAD_PORT: u8 = 0i32 as u8;

/// The `Xv::BadPort` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_bad_port_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_xv_bad_port_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xv::BadEncoding` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_xv_bad_encoding_error_t`].
pub const XCB_XV_BAD_ENCODING: u8 = 1i32 as u8;

/// The `Xv::BadEncoding` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_bad_encoding_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_xv_bad_encoding_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xv::BadControl` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_xv_bad_control_error_t`].
pub const XCB_XV_BAD_CONTROL: u8 = 2i32 as u8;

/// The `Xv::BadControl` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_bad_control_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_xv_bad_control_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xv::VideoNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_xv_video_notify_event_t`].
pub const XCB_XV_VIDEO_NOTIFY: u8 = 0i32 as u8;

/// The `Xv::VideoNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_video_notify_event_t {
    pub response_type: u8,
    pub reason: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub drawable: xcb_drawable_t,
    pub port: xcb_xv_port_t,
}

impl Default for xcb_xv_video_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xv::PortNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_xv_port_notify_event_t`].
pub const XCB_XV_PORT_NOTIFY: u8 = 1i32 as u8;

/// The `Xv::PortNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_port_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub port: xcb_xv_port_t,
    pub attribute: xcb_atom_t,
    pub value: i32,
}

impl Default for xcb_xv_port_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Xv::QueryExtension` request.
///
/// Pass this cookie to [`xcb_xv_query_extension_reply`] to retrieve the reply.
///
/// [`xcb_xv_query_extension_reply`]: XcbXv::xcb_xv_query_extension_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_extension_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xv_query_extension_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xv::QueryExtension` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXv::xcb_xv_id()`], then the type of the request is
/// [`xcb_xv_query_extension_request_t`].
pub const XCB_XV_QUERY_EXTENSION: u8 = 0i32 as u8;

/// The `Xv::QueryExtension` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_extension_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_xv_query_extension_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xv::QueryExtension` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_extension_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major: u16,
    pub minor: u16,
}

impl Default for xcb_xv_query_extension_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Xv::QueryAdaptors` request.
///
/// Pass this cookie to [`xcb_xv_query_adaptors_reply`] to retrieve the reply.
///
/// [`xcb_xv_query_adaptors_reply`]: XcbXv::xcb_xv_query_adaptors_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_adaptors_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xv_query_adaptors_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xv::QueryAdaptors` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXv::xcb_xv_id()`], then the type of the request is
/// [`xcb_xv_query_adaptors_request_t`].
pub const XCB_XV_QUERY_ADAPTORS: u8 = 1i32 as u8;

/// The `Xv::QueryAdaptors` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_adaptors_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_xv_query_adaptors_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xv::QueryAdaptors` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `info`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_adaptors_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_adaptors: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_xv_query_adaptors_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Xv::QueryEncodings` request.
///
/// Pass this cookie to [`xcb_xv_query_encodings_reply`] to retrieve the reply.
///
/// [`xcb_xv_query_encodings_reply`]: XcbXv::xcb_xv_query_encodings_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_encodings_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xv_query_encodings_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xv::QueryEncodings` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXv::xcb_xv_id()`], then the type of the request is
/// [`xcb_xv_query_encodings_request_t`].
pub const XCB_XV_QUERY_ENCODINGS: u8 = 2i32 as u8;

/// The `Xv::QueryEncodings` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_encodings_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
}

impl Default for xcb_xv_query_encodings_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xv::QueryEncodings` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `info`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_encodings_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_encodings: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_xv_query_encodings_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Xv::GrabPort` request.
///
/// Pass this cookie to [`xcb_xv_grab_port_reply`] to retrieve the reply.
///
/// [`xcb_xv_grab_port_reply`]: XcbXv::xcb_xv_grab_port_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_grab_port_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xv_grab_port_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xv::GrabPort` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXv::xcb_xv_id()`], then the type of the request is
/// [`xcb_xv_grab_port_request_t`].
pub const XCB_XV_GRAB_PORT: u8 = 3i32 as u8;

/// The `Xv::GrabPort` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_grab_port_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub time: xcb_timestamp_t,
}

impl Default for xcb_xv_grab_port_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xv::GrabPort` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_grab_port_reply_t {
    pub response_type: u8,
    pub result: u8,
    pub sequence: u16,
    pub length: u32,
}

impl Default for xcb_xv_grab_port_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xv::UngrabPort` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXv::xcb_xv_id()`], then the type of the request is
/// [`xcb_xv_ungrab_port_request_t`].
pub const XCB_XV_UNGRAB_PORT: u8 = 4i32 as u8;

/// The `Xv::UngrabPort` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_ungrab_port_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub time: xcb_timestamp_t,
}

impl Default for xcb_xv_ungrab_port_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xv::PutVideo` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXv::xcb_xv_id()`], then the type of the request is
/// [`xcb_xv_put_video_request_t`].
pub const XCB_XV_PUT_VIDEO: u8 = 5i32 as u8;

/// The `Xv::PutVideo` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_put_video_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub vid_x: i16,
    pub vid_y: i16,
    pub vid_w: u16,
    pub vid_h: u16,
    pub drw_x: i16,
    pub drw_y: i16,
    pub drw_w: u16,
    pub drw_h: u16,
}

impl Default for xcb_xv_put_video_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xv::PutStill` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXv::xcb_xv_id()`], then the type of the request is
/// [`xcb_xv_put_still_request_t`].
pub const XCB_XV_PUT_STILL: u8 = 6i32 as u8;

/// The `Xv::PutStill` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_put_still_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub vid_x: i16,
    pub vid_y: i16,
    pub vid_w: u16,
    pub vid_h: u16,
    pub drw_x: i16,
    pub drw_y: i16,
    pub drw_w: u16,
    pub drw_h: u16,
}

impl Default for xcb_xv_put_still_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xv::GetVideo` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXv::xcb_xv_id()`], then the type of the request is
/// [`xcb_xv_get_video_request_t`].
pub const XCB_XV_GET_VIDEO: u8 = 7i32 as u8;

/// The `Xv::GetVideo` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_get_video_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub vid_x: i16,
    pub vid_y: i16,
    pub vid_w: u16,
    pub vid_h: u16,
    pub drw_x: i16,
    pub drw_y: i16,
    pub drw_w: u16,
    pub drw_h: u16,
}

impl Default for xcb_xv_get_video_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xv::GetStill` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXv::xcb_xv_id()`], then the type of the request is
/// [`xcb_xv_get_still_request_t`].
pub const XCB_XV_GET_STILL: u8 = 8i32 as u8;

/// The `Xv::GetStill` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_get_still_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub vid_x: i16,
    pub vid_y: i16,
    pub vid_w: u16,
    pub vid_h: u16,
    pub drw_x: i16,
    pub drw_y: i16,
    pub drw_w: u16,
    pub drw_h: u16,
}

impl Default for xcb_xv_get_still_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xv::StopVideo` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXv::xcb_xv_id()`], then the type of the request is
/// [`xcb_xv_stop_video_request_t`].
pub const XCB_XV_STOP_VIDEO: u8 = 9i32 as u8;

/// The `Xv::StopVideo` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_stop_video_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub drawable: xcb_drawable_t,
}

impl Default for xcb_xv_stop_video_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xv::SelectVideoNotify` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXv::xcb_xv_id()`], then the type of the request is
/// [`xcb_xv_select_video_notify_request_t`].
pub const XCB_XV_SELECT_VIDEO_NOTIFY: u8 = 10i32 as u8;

/// The `Xv::SelectVideoNotify` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_select_video_notify_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub onoff: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_xv_select_video_notify_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xv::SelectPortNotify` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXv::xcb_xv_id()`], then the type of the request is
/// [`xcb_xv_select_port_notify_request_t`].
pub const XCB_XV_SELECT_PORT_NOTIFY: u8 = 11i32 as u8;

/// The `Xv::SelectPortNotify` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_select_port_notify_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub onoff: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_xv_select_port_notify_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Xv::QueryBestSize` request.
///
/// Pass this cookie to [`xcb_xv_query_best_size_reply`] to retrieve the reply.
///
/// [`xcb_xv_query_best_size_reply`]: XcbXv::xcb_xv_query_best_size_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_best_size_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xv_query_best_size_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xv::QueryBestSize` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXv::xcb_xv_id()`], then the type of the request is
/// [`xcb_xv_query_best_size_request_t`].
pub const XCB_XV_QUERY_BEST_SIZE: u8 = 12i32 as u8;

/// The `Xv::QueryBestSize` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_best_size_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub vid_w: u16,
    pub vid_h: u16,
    pub drw_w: u16,
    pub drw_h: u16,
    pub motion: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_xv_query_best_size_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xv::QueryBestSize` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_best_size_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub actual_width: u16,
    pub actual_height: u16,
}

impl Default for xcb_xv_query_best_size_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xv::SetPortAttribute` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXv::xcb_xv_id()`], then the type of the request is
/// [`xcb_xv_set_port_attribute_request_t`].
pub const XCB_XV_SET_PORT_ATTRIBUTE: u8 = 13i32 as u8;

/// The `Xv::SetPortAttribute` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_set_port_attribute_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub attribute: xcb_atom_t,
    pub value: i32,
}

impl Default for xcb_xv_set_port_attribute_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Xv::GetPortAttribute` request.
///
/// Pass this cookie to [`xcb_xv_get_port_attribute_reply`] to retrieve the reply.
///
/// [`xcb_xv_get_port_attribute_reply`]: XcbXv::xcb_xv_get_port_attribute_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_get_port_attribute_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xv_get_port_attribute_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xv::GetPortAttribute` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXv::xcb_xv_id()`], then the type of the request is
/// [`xcb_xv_get_port_attribute_request_t`].
pub const XCB_XV_GET_PORT_ATTRIBUTE: u8 = 14i32 as u8;

/// The `Xv::GetPortAttribute` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_get_port_attribute_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub attribute: xcb_atom_t,
}

impl Default for xcb_xv_get_port_attribute_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xv::GetPortAttribute` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_get_port_attribute_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub value: i32,
}

impl Default for xcb_xv_get_port_attribute_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Xv::QueryPortAttributes` request.
///
/// Pass this cookie to [`xcb_xv_query_port_attributes_reply`] to retrieve the reply.
///
/// [`xcb_xv_query_port_attributes_reply`]: XcbXv::xcb_xv_query_port_attributes_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_port_attributes_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xv_query_port_attributes_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xv::QueryPortAttributes` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXv::xcb_xv_id()`], then the type of the request is
/// [`xcb_xv_query_port_attributes_request_t`].
pub const XCB_XV_QUERY_PORT_ATTRIBUTES: u8 = 15i32 as u8;

/// The `Xv::QueryPortAttributes` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_port_attributes_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
}

impl Default for xcb_xv_query_port_attributes_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xv::QueryPortAttributes` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `attributes`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_port_attributes_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_attributes: u32,
    pub text_size: u32,
    pub pad1: [u8; 16],
}

impl Default for xcb_xv_query_port_attributes_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Xv::ListImageFormats` request.
///
/// Pass this cookie to [`xcb_xv_list_image_formats_reply`] to retrieve the reply.
///
/// [`xcb_xv_list_image_formats_reply`]: XcbXv::xcb_xv_list_image_formats_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_list_image_formats_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xv_list_image_formats_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xv::ListImageFormats` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXv::xcb_xv_id()`], then the type of the request is
/// [`xcb_xv_list_image_formats_request_t`].
pub const XCB_XV_LIST_IMAGE_FORMATS: u8 = 16i32 as u8;

/// The `Xv::ListImageFormats` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_list_image_formats_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
}

impl Default for xcb_xv_list_image_formats_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xv::ListImageFormats` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `format`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_list_image_formats_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_formats: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_xv_list_image_formats_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Xv::QueryImageAttributes` request.
///
/// Pass this cookie to [`xcb_xv_query_image_attributes_reply`] to retrieve the reply.
///
/// [`xcb_xv_query_image_attributes_reply`]: XcbXv::xcb_xv_query_image_attributes_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_image_attributes_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xv_query_image_attributes_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xv::QueryImageAttributes` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXv::xcb_xv_id()`], then the type of the request is
/// [`xcb_xv_query_image_attributes_request_t`].
pub const XCB_XV_QUERY_IMAGE_ATTRIBUTES: u8 = 17i32 as u8;

/// The `Xv::QueryImageAttributes` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_image_attributes_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub id: u32,
    pub width: u16,
    pub height: u16,
}

impl Default for xcb_xv_query_image_attributes_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Xv::QueryImageAttributes` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `pitches`
/// - `offsets`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_image_attributes_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_planes: u32,
    pub data_size: u32,
    pub width: u16,
    pub height: u16,
    pub pad1: [u8; 12],
}

impl Default for xcb_xv_query_image_attributes_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xv::PutImage` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXv::xcb_xv_id()`], then the type of the request is
/// [`xcb_xv_put_image_request_t`].
pub const XCB_XV_PUT_IMAGE: u8 = 18i32 as u8;

/// The `Xv::PutImage` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `data`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_put_image_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub id: u32,
    pub src_x: i16,
    pub src_y: i16,
    pub src_w: u16,
    pub src_h: u16,
    pub drw_x: i16,
    pub drw_y: i16,
    pub drw_w: u16,
    pub drw_h: u16,
    pub width: u16,
    pub height: u16,
}

impl Default for xcb_xv_put_image_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Xv::ShmPutImage` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXv::xcb_xv_id()`], then the type of the request is
/// [`xcb_xv_shm_put_image_request_t`].
pub const XCB_XV_SHM_PUT_IMAGE: u8 = 19i32 as u8;

/// The `Xv::ShmPutImage` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_shm_put_image_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub shmseg: xcb_shm_seg_t,
    pub id: u32,
    pub offset: u32,
    pub src_x: i16,
    pub src_y: i16,
    pub src_w: u16,
    pub src_h: u16,
    pub drw_x: i16,
    pub drw_y: i16,
    pub drw_w: u16,
    pub drw_h: u16,
    pub width: u16,
    pub height: u16,
    pub send_event: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_xv_shm_put_image_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_xv")]
pub(crate) struct XcbXvXv {
    xcb_xv_id: LazySymbol<*mut xcb_extension_t>,
    xcb_xv_port_next: LazySymbol<unsafe fn(i: *mut xcb_xv_port_iterator_t)>,
    xcb_xv_port_end: LazySymbol<unsafe fn(i: xcb_xv_port_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xv_encoding_next: LazySymbol<unsafe fn(i: *mut xcb_xv_encoding_iterator_t)>,
    xcb_xv_encoding_end:
        LazySymbol<unsafe fn(i: xcb_xv_encoding_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xv_rational_next: LazySymbol<unsafe fn(i: *mut xcb_xv_rational_iterator_t)>,
    xcb_xv_rational_end:
        LazySymbol<unsafe fn(i: xcb_xv_rational_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xv_format_next: LazySymbol<unsafe fn(i: *mut xcb_xv_format_iterator_t)>,
    xcb_xv_format_end: LazySymbol<unsafe fn(i: xcb_xv_format_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xv_adaptor_info_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xv_adaptor_info_name: LazySymbol<unsafe fn(r: *const xcb_xv_adaptor_info_t) -> *mut c_char>,
    xcb_xv_adaptor_info_name_length:
        LazySymbol<unsafe fn(r: *const xcb_xv_adaptor_info_t) -> c_int>,
    xcb_xv_adaptor_info_name_end:
        LazySymbol<unsafe fn(r: *const xcb_xv_adaptor_info_t) -> xcb_generic_iterator_t>,
    xcb_xv_adaptor_info_formats:
        LazySymbol<unsafe fn(r: *const xcb_xv_adaptor_info_t) -> *mut xcb_xv_format_t>,
    xcb_xv_adaptor_info_formats_length:
        LazySymbol<unsafe fn(r: *const xcb_xv_adaptor_info_t) -> c_int>,
    xcb_xv_adaptor_info_formats_iterator:
        LazySymbol<unsafe fn(r: *const xcb_xv_adaptor_info_t) -> xcb_xv_format_iterator_t>,
    xcb_xv_adaptor_info_next: LazySymbol<unsafe fn(i: *mut xcb_xv_adaptor_info_iterator_t)>,
    xcb_xv_adaptor_info_end:
        LazySymbol<unsafe fn(i: xcb_xv_adaptor_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xv_encoding_info_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xv_encoding_info_name:
        LazySymbol<unsafe fn(r: *const xcb_xv_encoding_info_t) -> *mut c_char>,
    xcb_xv_encoding_info_name_length:
        LazySymbol<unsafe fn(r: *const xcb_xv_encoding_info_t) -> c_int>,
    xcb_xv_encoding_info_name_end:
        LazySymbol<unsafe fn(r: *const xcb_xv_encoding_info_t) -> xcb_generic_iterator_t>,
    xcb_xv_encoding_info_next: LazySymbol<unsafe fn(i: *mut xcb_xv_encoding_info_iterator_t)>,
    xcb_xv_encoding_info_end:
        LazySymbol<unsafe fn(i: xcb_xv_encoding_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xv_image_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xv_image_pitches: LazySymbol<unsafe fn(r: *const xcb_xv_image_t) -> *mut u32>,
    xcb_xv_image_pitches_length: LazySymbol<unsafe fn(r: *const xcb_xv_image_t) -> c_int>,
    xcb_xv_image_pitches_end:
        LazySymbol<unsafe fn(r: *const xcb_xv_image_t) -> xcb_generic_iterator_t>,
    xcb_xv_image_offsets: LazySymbol<unsafe fn(r: *const xcb_xv_image_t) -> *mut u32>,
    xcb_xv_image_offsets_length: LazySymbol<unsafe fn(r: *const xcb_xv_image_t) -> c_int>,
    xcb_xv_image_offsets_end:
        LazySymbol<unsafe fn(r: *const xcb_xv_image_t) -> xcb_generic_iterator_t>,
    xcb_xv_image_data: LazySymbol<unsafe fn(r: *const xcb_xv_image_t) -> *mut u8>,
    xcb_xv_image_data_length: LazySymbol<unsafe fn(r: *const xcb_xv_image_t) -> c_int>,
    xcb_xv_image_data_end:
        LazySymbol<unsafe fn(r: *const xcb_xv_image_t) -> xcb_generic_iterator_t>,
    xcb_xv_image_next: LazySymbol<unsafe fn(i: *mut xcb_xv_image_iterator_t)>,
    xcb_xv_image_end: LazySymbol<unsafe fn(i: xcb_xv_image_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xv_attribute_info_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xv_attribute_info_name:
        LazySymbol<unsafe fn(r: *const xcb_xv_attribute_info_t) -> *mut c_char>,
    xcb_xv_attribute_info_name_length:
        LazySymbol<unsafe fn(r: *const xcb_xv_attribute_info_t) -> c_int>,
    xcb_xv_attribute_info_name_end:
        LazySymbol<unsafe fn(r: *const xcb_xv_attribute_info_t) -> xcb_generic_iterator_t>,
    xcb_xv_attribute_info_next: LazySymbol<unsafe fn(i: *mut xcb_xv_attribute_info_iterator_t)>,
    xcb_xv_attribute_info_end:
        LazySymbol<unsafe fn(i: xcb_xv_attribute_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xv_image_format_info_next:
        LazySymbol<unsafe fn(i: *mut xcb_xv_image_format_info_iterator_t)>,
    xcb_xv_image_format_info_end:
        LazySymbol<unsafe fn(i: xcb_xv_image_format_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xv_query_extension:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xv_query_extension_cookie_t>,
    xcb_xv_query_extension_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xv_query_extension_cookie_t>,
    xcb_xv_query_extension_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xv_query_extension_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xv_query_extension_reply_t,
    >,
    xcb_xv_query_adaptors_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xv_query_adaptors: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_xv_query_adaptors_cookie_t,
    >,
    xcb_xv_query_adaptors_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_xv_query_adaptors_cookie_t,
    >,
    xcb_xv_query_adaptors_info_length:
        LazySymbol<unsafe fn(r: *const xcb_xv_query_adaptors_reply_t) -> c_int>,
    xcb_xv_query_adaptors_info_iterator: LazySymbol<
        unsafe fn(r: *const xcb_xv_query_adaptors_reply_t) -> xcb_xv_adaptor_info_iterator_t,
    >,
    xcb_xv_query_adaptors_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xv_query_adaptors_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xv_query_adaptors_reply_t,
    >,
    xcb_xv_query_encodings_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xv_query_encodings: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, port: xcb_xv_port_t) -> xcb_xv_query_encodings_cookie_t,
    >,
    xcb_xv_query_encodings_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, port: xcb_xv_port_t) -> xcb_xv_query_encodings_cookie_t,
    >,
    xcb_xv_query_encodings_info_length:
        LazySymbol<unsafe fn(r: *const xcb_xv_query_encodings_reply_t) -> c_int>,
    xcb_xv_query_encodings_info_iterator: LazySymbol<
        unsafe fn(r: *const xcb_xv_query_encodings_reply_t) -> xcb_xv_encoding_info_iterator_t,
    >,
    xcb_xv_query_encodings_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xv_query_encodings_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xv_query_encodings_reply_t,
    >,
    xcb_xv_grab_port: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            time: xcb_timestamp_t,
        ) -> xcb_xv_grab_port_cookie_t,
    >,
    xcb_xv_grab_port_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            time: xcb_timestamp_t,
        ) -> xcb_xv_grab_port_cookie_t,
    >,
    xcb_xv_grab_port_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xv_grab_port_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xv_grab_port_reply_t,
    >,
    xcb_xv_ungrab_port_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            time: xcb_timestamp_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xv_ungrab_port: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            time: xcb_timestamp_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xv_put_video_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            vid_x: i16,
            vid_y: i16,
            vid_w: u16,
            vid_h: u16,
            drw_x: i16,
            drw_y: i16,
            drw_w: u16,
            drw_h: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xv_put_video: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            vid_x: i16,
            vid_y: i16,
            vid_w: u16,
            vid_h: u16,
            drw_x: i16,
            drw_y: i16,
            drw_w: u16,
            drw_h: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xv_put_still_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            vid_x: i16,
            vid_y: i16,
            vid_w: u16,
            vid_h: u16,
            drw_x: i16,
            drw_y: i16,
            drw_w: u16,
            drw_h: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xv_put_still: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            vid_x: i16,
            vid_y: i16,
            vid_w: u16,
            vid_h: u16,
            drw_x: i16,
            drw_y: i16,
            drw_w: u16,
            drw_h: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xv_get_video_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            vid_x: i16,
            vid_y: i16,
            vid_w: u16,
            vid_h: u16,
            drw_x: i16,
            drw_y: i16,
            drw_w: u16,
            drw_h: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xv_get_video: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            vid_x: i16,
            vid_y: i16,
            vid_w: u16,
            vid_h: u16,
            drw_x: i16,
            drw_y: i16,
            drw_w: u16,
            drw_h: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xv_get_still_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            vid_x: i16,
            vid_y: i16,
            vid_w: u16,
            vid_h: u16,
            drw_x: i16,
            drw_y: i16,
            drw_w: u16,
            drw_h: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xv_get_still: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            vid_x: i16,
            vid_y: i16,
            vid_w: u16,
            vid_h: u16,
            drw_x: i16,
            drw_y: i16,
            drw_w: u16,
            drw_h: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xv_stop_video_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            drawable: xcb_drawable_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xv_stop_video: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            drawable: xcb_drawable_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xv_select_video_notify_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            onoff: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xv_select_video_notify: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            onoff: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xv_select_port_notify_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, port: xcb_xv_port_t, onoff: u8) -> xcb_void_cookie_t,
    >,
    xcb_xv_select_port_notify: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, port: xcb_xv_port_t, onoff: u8) -> xcb_void_cookie_t,
    >,
    xcb_xv_query_best_size: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            vid_w: u16,
            vid_h: u16,
            drw_w: u16,
            drw_h: u16,
            motion: u8,
        ) -> xcb_xv_query_best_size_cookie_t,
    >,
    xcb_xv_query_best_size_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            vid_w: u16,
            vid_h: u16,
            drw_w: u16,
            drw_h: u16,
            motion: u8,
        ) -> xcb_xv_query_best_size_cookie_t,
    >,
    xcb_xv_query_best_size_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xv_query_best_size_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xv_query_best_size_reply_t,
    >,
    xcb_xv_set_port_attribute_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            attribute: xcb_atom_t,
            value: i32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xv_set_port_attribute: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            attribute: xcb_atom_t,
            value: i32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xv_get_port_attribute: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            attribute: xcb_atom_t,
        ) -> xcb_xv_get_port_attribute_cookie_t,
    >,
    xcb_xv_get_port_attribute_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            attribute: xcb_atom_t,
        ) -> xcb_xv_get_port_attribute_cookie_t,
    >,
    xcb_xv_get_port_attribute_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xv_get_port_attribute_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xv_get_port_attribute_reply_t,
    >,
    xcb_xv_query_port_attributes_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xv_query_port_attributes: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
        ) -> xcb_xv_query_port_attributes_cookie_t,
    >,
    xcb_xv_query_port_attributes_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
        ) -> xcb_xv_query_port_attributes_cookie_t,
    >,
    xcb_xv_query_port_attributes_attributes_length:
        LazySymbol<unsafe fn(r: *const xcb_xv_query_port_attributes_reply_t) -> c_int>,
    xcb_xv_query_port_attributes_attributes_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xv_query_port_attributes_reply_t,
        ) -> xcb_xv_attribute_info_iterator_t,
    >,
    xcb_xv_query_port_attributes_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xv_query_port_attributes_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xv_query_port_attributes_reply_t,
    >,
    xcb_xv_list_image_formats_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xv_list_image_formats: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
        ) -> xcb_xv_list_image_formats_cookie_t,
    >,
    xcb_xv_list_image_formats_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
        ) -> xcb_xv_list_image_formats_cookie_t,
    >,
    xcb_xv_list_image_formats_format: LazySymbol<
        unsafe fn(r: *const xcb_xv_list_image_formats_reply_t) -> *mut xcb_xv_image_format_info_t,
    >,
    xcb_xv_list_image_formats_format_length:
        LazySymbol<unsafe fn(r: *const xcb_xv_list_image_formats_reply_t) -> c_int>,
    xcb_xv_list_image_formats_format_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xv_list_image_formats_reply_t,
        ) -> xcb_xv_image_format_info_iterator_t,
    >,
    xcb_xv_list_image_formats_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xv_list_image_formats_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xv_list_image_formats_reply_t,
    >,
    xcb_xv_query_image_attributes_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xv_query_image_attributes: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            id: u32,
            width: u16,
            height: u16,
        ) -> xcb_xv_query_image_attributes_cookie_t,
    >,
    xcb_xv_query_image_attributes_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            id: u32,
            width: u16,
            height: u16,
        ) -> xcb_xv_query_image_attributes_cookie_t,
    >,
    xcb_xv_query_image_attributes_pitches:
        LazySymbol<unsafe fn(r: *const xcb_xv_query_image_attributes_reply_t) -> *mut u32>,
    xcb_xv_query_image_attributes_pitches_length:
        LazySymbol<unsafe fn(r: *const xcb_xv_query_image_attributes_reply_t) -> c_int>,
    xcb_xv_query_image_attributes_pitches_end: LazySymbol<
        unsafe fn(r: *const xcb_xv_query_image_attributes_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_xv_query_image_attributes_offsets:
        LazySymbol<unsafe fn(r: *const xcb_xv_query_image_attributes_reply_t) -> *mut u32>,
    xcb_xv_query_image_attributes_offsets_length:
        LazySymbol<unsafe fn(r: *const xcb_xv_query_image_attributes_reply_t) -> c_int>,
    xcb_xv_query_image_attributes_offsets_end: LazySymbol<
        unsafe fn(r: *const xcb_xv_query_image_attributes_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_xv_query_image_attributes_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xv_query_image_attributes_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xv_query_image_attributes_reply_t,
    >,
    xcb_xv_put_image_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void, data_len: u32) -> c_int>,
    xcb_xv_put_image_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            id: u32,
            src_x: i16,
            src_y: i16,
            src_w: u16,
            src_h: u16,
            drw_x: i16,
            drw_y: i16,
            drw_w: u16,
            drw_h: u16,
            width: u16,
            height: u16,
            data_len: u32,
            data: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xv_put_image: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            id: u32,
            src_x: i16,
            src_y: i16,
            src_w: u16,
            src_h: u16,
            drw_x: i16,
            drw_y: i16,
            drw_w: u16,
            drw_h: u16,
            width: u16,
            height: u16,
            data_len: u32,
            data: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xv_put_image_data: LazySymbol<unsafe fn(r: *const xcb_xv_put_image_request_t) -> *mut u8>,
    xcb_xv_put_image_data_length:
        LazySymbol<unsafe fn(r: *const xcb_xv_put_image_request_t) -> c_int>,
    xcb_xv_put_image_data_end:
        LazySymbol<unsafe fn(r: *const xcb_xv_put_image_request_t) -> xcb_generic_iterator_t>,
    xcb_xv_shm_put_image_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            shmseg: xcb_shm_seg_t,
            id: u32,
            offset: u32,
            src_x: i16,
            src_y: i16,
            src_w: u16,
            src_h: u16,
            drw_x: i16,
            drw_y: i16,
            drw_w: u16,
            drw_h: u16,
            width: u16,
            height: u16,
            send_event: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xv_shm_put_image: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port: xcb_xv_port_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            shmseg: xcb_shm_seg_t,
            id: u32,
            offset: u32,
            src_x: i16,
            src_y: i16,
            src_w: u16,
            src_h: u16,
            drw_x: i16,
            drw_y: i16,
            drw_w: u16,
            drw_h: u16,
            width: u16,
            height: u16,
            send_event: u8,
        ) -> xcb_void_cookie_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self.xv.$f.get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .xv
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_xv")]
impl XcbXv {
    /// The libxcb identifier of the `Xv` extension.
    #[inline]
    pub fn xcb_xv_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_xv_id) }
    }

    /// Returns `true` iff the symbol `xcb_xv_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_id(&self) -> bool {
        has_sym!(self, xcb_xv_id)
    }

    /// Advances a `xcb_xv_port_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xv_port_next(&self, i: *mut xcb_xv_port_iterator_t) {
        sym!(self, xcb_xv_port_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xv_port_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_port_next(&self) -> bool {
        has_sym!(self, xcb_xv_port_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xv_port_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xv_port_end(&self, i: xcb_xv_port_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_port_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xv_port_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_port_end(&self) -> bool {
        has_sym!(self, xcb_xv_port_end)
    }

    /// Advances a `xcb_xv_encoding_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xv_encoding_next(&self, i: *mut xcb_xv_encoding_iterator_t) {
        sym!(self, xcb_xv_encoding_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xv_encoding_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_encoding_next(&self) -> bool {
        has_sym!(self, xcb_xv_encoding_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xv_encoding_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xv_encoding_end(
        &self,
        i: xcb_xv_encoding_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_encoding_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xv_encoding_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_encoding_end(&self) -> bool {
        has_sym!(self, xcb_xv_encoding_end)
    }

    /// Advances a `xcb_xv_rational_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xv_rational_next(&self, i: *mut xcb_xv_rational_iterator_t) {
        sym!(self, xcb_xv_rational_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xv_rational_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_rational_next(&self) -> bool {
        has_sym!(self, xcb_xv_rational_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xv_rational_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xv_rational_end(
        &self,
        i: xcb_xv_rational_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_rational_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xv_rational_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_rational_end(&self) -> bool {
        has_sym!(self, xcb_xv_rational_end)
    }

    /// Advances a `xcb_xv_format_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xv_format_next(&self, i: *mut xcb_xv_format_iterator_t) {
        sym!(self, xcb_xv_format_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xv_format_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_format_next(&self) -> bool {
        has_sym!(self, xcb_xv_format_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xv_format_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xv_format_end(&self, i: xcb_xv_format_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_format_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xv_format_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_format_end(&self) -> bool {
        has_sym!(self, xcb_xv_format_end)
    }

    /// Computes the size of a `xcb_xv_adaptor_info_t` object.
    #[inline]
    pub unsafe fn xcb_xv_adaptor_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xv_adaptor_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xv_adaptor_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_adaptor_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_xv_adaptor_info_sizeof)
    }

    /// Returns a pointer to the `name` field of a `xcb_xv_adaptor_info_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_adaptor_info_name(&self, r: *const xcb_xv_adaptor_info_t) -> *mut c_char {
        sym!(self, xcb_xv_adaptor_info_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_adaptor_info_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_adaptor_info_name(&self) -> bool {
        has_sym!(self, xcb_xv_adaptor_info_name)
    }

    /// Returns the number of elements of the `name` field of a `xcb_xv_adaptor_info_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_adaptor_info_name_length(&self, r: *const xcb_xv_adaptor_info_t) -> c_int {
        sym!(self, xcb_xv_adaptor_info_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_adaptor_info_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_adaptor_info_name_length(&self) -> bool {
        has_sym!(self, xcb_xv_adaptor_info_name_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `name` field of a `xcb_xv_adaptor_info_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_adaptor_info_name_end(
        &self,
        r: *const xcb_xv_adaptor_info_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_adaptor_info_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_adaptor_info_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_adaptor_info_name_end(&self) -> bool {
        has_sym!(self, xcb_xv_adaptor_info_name_end)
    }

    /// Returns a pointer to the `formats` field of a `xcb_xv_adaptor_info_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_adaptor_info_formats(
        &self,
        r: *const xcb_xv_adaptor_info_t,
    ) -> *mut xcb_xv_format_t {
        sym!(self, xcb_xv_adaptor_info_formats)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_adaptor_info_formats` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_adaptor_info_formats(&self) -> bool {
        has_sym!(self, xcb_xv_adaptor_info_formats)
    }

    /// Returns the number of elements of the `formats` field of a `xcb_xv_adaptor_info_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_adaptor_info_formats_length(
        &self,
        r: *const xcb_xv_adaptor_info_t,
    ) -> c_int {
        sym!(self, xcb_xv_adaptor_info_formats_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_adaptor_info_formats_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_adaptor_info_formats_length(&self) -> bool {
        has_sym!(self, xcb_xv_adaptor_info_formats_length)
    }

    /// Returns an iterator over the elements of the
    /// `formats` field of a `xcb_xv_adaptor_info_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_adaptor_info_formats_iterator(
        &self,
        r: *const xcb_xv_adaptor_info_t,
    ) -> xcb_xv_format_iterator_t {
        sym!(self, xcb_xv_adaptor_info_formats_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_adaptor_info_formats_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_adaptor_info_formats_iterator(&self) -> bool {
        has_sym!(self, xcb_xv_adaptor_info_formats_iterator)
    }

    /// Advances a `xcb_xv_adaptor_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xv_adaptor_info_next(&self, i: *mut xcb_xv_adaptor_info_iterator_t) {
        sym!(self, xcb_xv_adaptor_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xv_adaptor_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_adaptor_info_next(&self) -> bool {
        has_sym!(self, xcb_xv_adaptor_info_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xv_adaptor_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xv_adaptor_info_end(
        &self,
        i: xcb_xv_adaptor_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_adaptor_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xv_adaptor_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_adaptor_info_end(&self) -> bool {
        has_sym!(self, xcb_xv_adaptor_info_end)
    }

    /// Computes the size of a `xcb_xv_encoding_info_t` object.
    #[inline]
    pub unsafe fn xcb_xv_encoding_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xv_encoding_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xv_encoding_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_encoding_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_xv_encoding_info_sizeof)
    }

    /// Returns a pointer to the `name` field of a `xcb_xv_encoding_info_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_encoding_info_name(
        &self,
        r: *const xcb_xv_encoding_info_t,
    ) -> *mut c_char {
        sym!(self, xcb_xv_encoding_info_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_encoding_info_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_encoding_info_name(&self) -> bool {
        has_sym!(self, xcb_xv_encoding_info_name)
    }

    /// Returns the number of elements of the `name` field of a `xcb_xv_encoding_info_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_encoding_info_name_length(
        &self,
        r: *const xcb_xv_encoding_info_t,
    ) -> c_int {
        sym!(self, xcb_xv_encoding_info_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_encoding_info_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_encoding_info_name_length(&self) -> bool {
        has_sym!(self, xcb_xv_encoding_info_name_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `name` field of a `xcb_xv_encoding_info_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_encoding_info_name_end(
        &self,
        r: *const xcb_xv_encoding_info_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_encoding_info_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_encoding_info_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_encoding_info_name_end(&self) -> bool {
        has_sym!(self, xcb_xv_encoding_info_name_end)
    }

    /// Advances a `xcb_xv_encoding_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xv_encoding_info_next(&self, i: *mut xcb_xv_encoding_info_iterator_t) {
        sym!(self, xcb_xv_encoding_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xv_encoding_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_encoding_info_next(&self) -> bool {
        has_sym!(self, xcb_xv_encoding_info_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xv_encoding_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xv_encoding_info_end(
        &self,
        i: xcb_xv_encoding_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_encoding_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xv_encoding_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_encoding_info_end(&self) -> bool {
        has_sym!(self, xcb_xv_encoding_info_end)
    }

    /// Computes the size of a `xcb_xv_image_t` object.
    #[inline]
    pub unsafe fn xcb_xv_image_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xv_image_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xv_image_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_image_sizeof(&self) -> bool {
        has_sym!(self, xcb_xv_image_sizeof)
    }

    /// Returns a pointer to the `pitches` field of a `xcb_xv_image_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_image_pitches(&self, r: *const xcb_xv_image_t) -> *mut u32 {
        sym!(self, xcb_xv_image_pitches)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_image_pitches` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_image_pitches(&self) -> bool {
        has_sym!(self, xcb_xv_image_pitches)
    }

    /// Returns the number of elements of the `pitches` field of a `xcb_xv_image_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_image_pitches_length(&self, r: *const xcb_xv_image_t) -> c_int {
        sym!(self, xcb_xv_image_pitches_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_image_pitches_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_image_pitches_length(&self) -> bool {
        has_sym!(self, xcb_xv_image_pitches_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `pitches` field of a `xcb_xv_image_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_image_pitches_end(
        &self,
        r: *const xcb_xv_image_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_image_pitches_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_image_pitches_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_image_pitches_end(&self) -> bool {
        has_sym!(self, xcb_xv_image_pitches_end)
    }

    /// Returns a pointer to the `offsets` field of a `xcb_xv_image_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_image_offsets(&self, r: *const xcb_xv_image_t) -> *mut u32 {
        sym!(self, xcb_xv_image_offsets)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_image_offsets` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_image_offsets(&self) -> bool {
        has_sym!(self, xcb_xv_image_offsets)
    }

    /// Returns the number of elements of the `offsets` field of a `xcb_xv_image_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_image_offsets_length(&self, r: *const xcb_xv_image_t) -> c_int {
        sym!(self, xcb_xv_image_offsets_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_image_offsets_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_image_offsets_length(&self) -> bool {
        has_sym!(self, xcb_xv_image_offsets_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `offsets` field of a `xcb_xv_image_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_image_offsets_end(
        &self,
        r: *const xcb_xv_image_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_image_offsets_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_image_offsets_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_image_offsets_end(&self) -> bool {
        has_sym!(self, xcb_xv_image_offsets_end)
    }

    /// Returns a pointer to the `data` field of a `xcb_xv_image_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_image_data(&self, r: *const xcb_xv_image_t) -> *mut u8 {
        sym!(self, xcb_xv_image_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_image_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_image_data(&self) -> bool {
        has_sym!(self, xcb_xv_image_data)
    }

    /// Returns the number of elements of the `data` field of a `xcb_xv_image_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_image_data_length(&self, r: *const xcb_xv_image_t) -> c_int {
        sym!(self, xcb_xv_image_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_image_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_image_data_length(&self) -> bool {
        has_sym!(self, xcb_xv_image_data_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `data` field of a `xcb_xv_image_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_image_data_end(&self, r: *const xcb_xv_image_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_image_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_image_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_image_data_end(&self) -> bool {
        has_sym!(self, xcb_xv_image_data_end)
    }

    /// Advances a `xcb_xv_image_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xv_image_next(&self, i: *mut xcb_xv_image_iterator_t) {
        sym!(self, xcb_xv_image_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xv_image_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_image_next(&self) -> bool {
        has_sym!(self, xcb_xv_image_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xv_image_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xv_image_end(&self, i: xcb_xv_image_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_image_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xv_image_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_image_end(&self) -> bool {
        has_sym!(self, xcb_xv_image_end)
    }

    /// Computes the size of a `xcb_xv_attribute_info_t` object.
    #[inline]
    pub unsafe fn xcb_xv_attribute_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xv_attribute_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xv_attribute_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_attribute_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_xv_attribute_info_sizeof)
    }

    /// Returns a pointer to the `name` field of a `xcb_xv_attribute_info_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_attribute_info_name(
        &self,
        r: *const xcb_xv_attribute_info_t,
    ) -> *mut c_char {
        sym!(self, xcb_xv_attribute_info_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_attribute_info_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_attribute_info_name(&self) -> bool {
        has_sym!(self, xcb_xv_attribute_info_name)
    }

    /// Returns the number of elements of the `name` field of a `xcb_xv_attribute_info_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_attribute_info_name_length(
        &self,
        r: *const xcb_xv_attribute_info_t,
    ) -> c_int {
        sym!(self, xcb_xv_attribute_info_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_attribute_info_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_attribute_info_name_length(&self) -> bool {
        has_sym!(self, xcb_xv_attribute_info_name_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `name` field of a `xcb_xv_attribute_info_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_attribute_info_name_end(
        &self,
        r: *const xcb_xv_attribute_info_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_attribute_info_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_attribute_info_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_attribute_info_name_end(&self) -> bool {
        has_sym!(self, xcb_xv_attribute_info_name_end)
    }

    /// Advances a `xcb_xv_attribute_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xv_attribute_info_next(&self, i: *mut xcb_xv_attribute_info_iterator_t) {
        sym!(self, xcb_xv_attribute_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xv_attribute_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_attribute_info_next(&self) -> bool {
        has_sym!(self, xcb_xv_attribute_info_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xv_attribute_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xv_attribute_info_end(
        &self,
        i: xcb_xv_attribute_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_attribute_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xv_attribute_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_attribute_info_end(&self) -> bool {
        has_sym!(self, xcb_xv_attribute_info_end)
    }

    /// Advances a `xcb_xv_image_format_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xv_image_format_info_next(
        &self,
        i: *mut xcb_xv_image_format_info_iterator_t,
    ) {
        sym!(self, xcb_xv_image_format_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xv_image_format_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_image_format_info_next(&self) -> bool {
        has_sym!(self, xcb_xv_image_format_info_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xv_image_format_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xv_image_format_info_end(
        &self,
        i: xcb_xv_image_format_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_image_format_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xv_image_format_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_image_format_info_end(&self) -> bool {
        has_sym!(self, xcb_xv_image_format_info_end)
    }

    /// Sends a `Xv::QueryExtension` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xv_query_extension_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xv_query_extension_reply`]: Self::xcb_xv_query_extension_reply
    #[inline]
    pub unsafe fn xcb_xv_query_extension(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xv_query_extension_cookie_t {
        sym!(self, xcb_xv_query_extension)(c)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_extension` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_extension(&self) -> bool {
        has_sym!(self, xcb_xv_query_extension)
    }

    /// Sends a `Xv::QueryExtension` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xv_query_extension_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xv_query_extension_reply`]: Self::xcb_xv_query_extension_reply
    #[inline]
    pub unsafe fn xcb_xv_query_extension_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xv_query_extension_cookie_t {
        sym!(self, xcb_xv_query_extension_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_extension_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_extension_unchecked(&self) -> bool {
        has_sym!(self, xcb_xv_query_extension_unchecked)
    }

    /// Waits for the reply to a `Xv::QueryExtension` request.
    #[inline]
    pub unsafe fn xcb_xv_query_extension_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xv_query_extension_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xv_query_extension_reply_t {
        sym!(self, xcb_xv_query_extension_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_extension_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_extension_reply(&self) -> bool {
        has_sym!(self, xcb_xv_query_extension_reply)
    }

    /// Computes the size of a `xcb_xv_query_adaptors_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xv_query_adaptors_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xv_query_adaptors_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_adaptors_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_adaptors_sizeof(&self) -> bool {
        has_sym!(self, xcb_xv_query_adaptors_sizeof)
    }

    /// Sends a `Xv::QueryAdaptors` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xv_query_adaptors_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xv_query_adaptors_reply`]: Self::xcb_xv_query_adaptors_reply
    #[inline]
    pub unsafe fn xcb_xv_query_adaptors(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_xv_query_adaptors_cookie_t {
        sym!(self, xcb_xv_query_adaptors)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_adaptors` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_adaptors(&self) -> bool {
        has_sym!(self, xcb_xv_query_adaptors)
    }

    /// Sends a `Xv::QueryAdaptors` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xv_query_adaptors_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xv_query_adaptors_reply`]: Self::xcb_xv_query_adaptors_reply
    #[inline]
    pub unsafe fn xcb_xv_query_adaptors_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_xv_query_adaptors_cookie_t {
        sym!(self, xcb_xv_query_adaptors_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_adaptors_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_adaptors_unchecked(&self) -> bool {
        has_sym!(self, xcb_xv_query_adaptors_unchecked)
    }

    /// Returns the number of elements of the `info` field of a `xcb_xv_query_adaptors_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_query_adaptors_info_length(
        &self,
        r: *const xcb_xv_query_adaptors_reply_t,
    ) -> c_int {
        sym!(self, xcb_xv_query_adaptors_info_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_adaptors_info_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_adaptors_info_length(&self) -> bool {
        has_sym!(self, xcb_xv_query_adaptors_info_length)
    }

    /// Returns an iterator over the elements of the
    /// `info` field of a `xcb_xv_query_adaptors_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_query_adaptors_info_iterator(
        &self,
        r: *const xcb_xv_query_adaptors_reply_t,
    ) -> xcb_xv_adaptor_info_iterator_t {
        sym!(self, xcb_xv_query_adaptors_info_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_adaptors_info_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_adaptors_info_iterator(&self) -> bool {
        has_sym!(self, xcb_xv_query_adaptors_info_iterator)
    }

    /// Waits for the reply to a `Xv::QueryAdaptors` request.
    #[inline]
    pub unsafe fn xcb_xv_query_adaptors_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xv_query_adaptors_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xv_query_adaptors_reply_t {
        sym!(self, xcb_xv_query_adaptors_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_adaptors_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_adaptors_reply(&self) -> bool {
        has_sym!(self, xcb_xv_query_adaptors_reply)
    }

    /// Computes the size of a `xcb_xv_query_encodings_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xv_query_encodings_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xv_query_encodings_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_encodings_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_encodings_sizeof(&self) -> bool {
        has_sym!(self, xcb_xv_query_encodings_sizeof)
    }

    /// Sends a `Xv::QueryEncodings` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xv_query_encodings_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xv_query_encodings_reply`]: Self::xcb_xv_query_encodings_reply
    #[inline]
    pub unsafe fn xcb_xv_query_encodings(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
    ) -> xcb_xv_query_encodings_cookie_t {
        sym!(self, xcb_xv_query_encodings)(c, port)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_encodings` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_encodings(&self) -> bool {
        has_sym!(self, xcb_xv_query_encodings)
    }

    /// Sends a `Xv::QueryEncodings` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xv_query_encodings_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xv_query_encodings_reply`]: Self::xcb_xv_query_encodings_reply
    #[inline]
    pub unsafe fn xcb_xv_query_encodings_unchecked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
    ) -> xcb_xv_query_encodings_cookie_t {
        sym!(self, xcb_xv_query_encodings_unchecked)(c, port)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_encodings_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_encodings_unchecked(&self) -> bool {
        has_sym!(self, xcb_xv_query_encodings_unchecked)
    }

    /// Returns the number of elements of the `info` field of a `xcb_xv_query_encodings_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_query_encodings_info_length(
        &self,
        r: *const xcb_xv_query_encodings_reply_t,
    ) -> c_int {
        sym!(self, xcb_xv_query_encodings_info_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_encodings_info_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_encodings_info_length(&self) -> bool {
        has_sym!(self, xcb_xv_query_encodings_info_length)
    }

    /// Returns an iterator over the elements of the
    /// `info` field of a `xcb_xv_query_encodings_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_query_encodings_info_iterator(
        &self,
        r: *const xcb_xv_query_encodings_reply_t,
    ) -> xcb_xv_encoding_info_iterator_t {
        sym!(self, xcb_xv_query_encodings_info_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_encodings_info_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_encodings_info_iterator(&self) -> bool {
        has_sym!(self, xcb_xv_query_encodings_info_iterator)
    }

    /// Waits for the reply to a `Xv::QueryEncodings` request.
    #[inline]
    pub unsafe fn xcb_xv_query_encodings_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xv_query_encodings_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xv_query_encodings_reply_t {
        sym!(self, xcb_xv_query_encodings_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_encodings_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_encodings_reply(&self) -> bool {
        has_sym!(self, xcb_xv_query_encodings_reply)
    }

    /// Sends a `Xv::GrabPort` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xv_grab_port_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xv_grab_port_reply`]: Self::xcb_xv_grab_port_reply
    #[inline]
    pub unsafe fn xcb_xv_grab_port(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        time: xcb_timestamp_t,
    ) -> xcb_xv_grab_port_cookie_t {
        sym!(self, xcb_xv_grab_port)(c, port, time)
    }

    /// Returns `true` iff the symbol `xcb_xv_grab_port` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_grab_port(&self) -> bool {
        has_sym!(self, xcb_xv_grab_port)
    }

    /// Sends a `Xv::GrabPort` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xv_grab_port_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xv_grab_port_reply`]: Self::xcb_xv_grab_port_reply
    #[inline]
    pub unsafe fn xcb_xv_grab_port_unchecked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        time: xcb_timestamp_t,
    ) -> xcb_xv_grab_port_cookie_t {
        sym!(self, xcb_xv_grab_port_unchecked)(c, port, time)
    }

    /// Returns `true` iff the symbol `xcb_xv_grab_port_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_grab_port_unchecked(&self) -> bool {
        has_sym!(self, xcb_xv_grab_port_unchecked)
    }

    /// Waits for the reply to a `Xv::GrabPort` request.
    #[inline]
    pub unsafe fn xcb_xv_grab_port_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xv_grab_port_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xv_grab_port_reply_t {
        sym!(self, xcb_xv_grab_port_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xv_grab_port_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_grab_port_reply(&self) -> bool {
        has_sym!(self, xcb_xv_grab_port_reply)
    }

    /// Sends a `Xv::UngrabPort` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xv_ungrab_port_checked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_ungrab_port_checked)(c, port, time)
    }

    /// Returns `true` iff the symbol `xcb_xv_ungrab_port_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_ungrab_port_checked(&self) -> bool {
        has_sym!(self, xcb_xv_ungrab_port_checked)
    }

    /// Sends a `Xv::UngrabPort` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xv_ungrab_port(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_ungrab_port)(c, port, time)
    }

    /// Returns `true` iff the symbol `xcb_xv_ungrab_port` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_ungrab_port(&self) -> bool {
        has_sym!(self, xcb_xv_ungrab_port)
    }

    /// Sends a `Xv::PutVideo` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xv_put_video_checked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        vid_x: i16,
        vid_y: i16,
        vid_w: u16,
        vid_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_put_video_checked)(
            c, port, drawable, gc, vid_x, vid_y, vid_w, vid_h, drw_x, drw_y, drw_w, drw_h,
        )
    }

    /// Returns `true` iff the symbol `xcb_xv_put_video_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_put_video_checked(&self) -> bool {
        has_sym!(self, xcb_xv_put_video_checked)
    }

    /// Sends a `Xv::PutVideo` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xv_put_video(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        vid_x: i16,
        vid_y: i16,
        vid_w: u16,
        vid_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_put_video)(
            c, port, drawable, gc, vid_x, vid_y, vid_w, vid_h, drw_x, drw_y, drw_w, drw_h,
        )
    }

    /// Returns `true` iff the symbol `xcb_xv_put_video` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_put_video(&self) -> bool {
        has_sym!(self, xcb_xv_put_video)
    }

    /// Sends a `Xv::PutStill` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xv_put_still_checked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        vid_x: i16,
        vid_y: i16,
        vid_w: u16,
        vid_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_put_still_checked)(
            c, port, drawable, gc, vid_x, vid_y, vid_w, vid_h, drw_x, drw_y, drw_w, drw_h,
        )
    }

    /// Returns `true` iff the symbol `xcb_xv_put_still_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_put_still_checked(&self) -> bool {
        has_sym!(self, xcb_xv_put_still_checked)
    }

    /// Sends a `Xv::PutStill` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xv_put_still(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        vid_x: i16,
        vid_y: i16,
        vid_w: u16,
        vid_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_put_still)(
            c, port, drawable, gc, vid_x, vid_y, vid_w, vid_h, drw_x, drw_y, drw_w, drw_h,
        )
    }

    /// Returns `true` iff the symbol `xcb_xv_put_still` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_put_still(&self) -> bool {
        has_sym!(self, xcb_xv_put_still)
    }

    /// Sends a `Xv::GetVideo` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xv_get_video_checked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        vid_x: i16,
        vid_y: i16,
        vid_w: u16,
        vid_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_get_video_checked)(
            c, port, drawable, gc, vid_x, vid_y, vid_w, vid_h, drw_x, drw_y, drw_w, drw_h,
        )
    }

    /// Returns `true` iff the symbol `xcb_xv_get_video_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_get_video_checked(&self) -> bool {
        has_sym!(self, xcb_xv_get_video_checked)
    }

    /// Sends a `Xv::GetVideo` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xv_get_video(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        vid_x: i16,
        vid_y: i16,
        vid_w: u16,
        vid_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_get_video)(
            c, port, drawable, gc, vid_x, vid_y, vid_w, vid_h, drw_x, drw_y, drw_w, drw_h,
        )
    }

    /// Returns `true` iff the symbol `xcb_xv_get_video` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_get_video(&self) -> bool {
        has_sym!(self, xcb_xv_get_video)
    }

    /// Sends a `Xv::GetStill` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xv_get_still_checked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        vid_x: i16,
        vid_y: i16,
        vid_w: u16,
        vid_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_get_still_checked)(
            c, port, drawable, gc, vid_x, vid_y, vid_w, vid_h, drw_x, drw_y, drw_w, drw_h,
        )
    }

    /// Returns `true` iff the symbol `xcb_xv_get_still_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_get_still_checked(&self) -> bool {
        has_sym!(self, xcb_xv_get_still_checked)
    }

    /// Sends a `Xv::GetStill` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xv_get_still(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        vid_x: i16,
        vid_y: i16,
        vid_w: u16,
        vid_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_get_still)(
            c, port, drawable, gc, vid_x, vid_y, vid_w, vid_h, drw_x, drw_y, drw_w, drw_h,
        )
    }

    /// Returns `true` iff the symbol `xcb_xv_get_still` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_get_still(&self) -> bool {
        has_sym!(self, xcb_xv_get_still)
    }

    /// Sends a `Xv::StopVideo` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xv_stop_video_checked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_stop_video_checked)(c, port, drawable)
    }

    /// Returns `true` iff the symbol `xcb_xv_stop_video_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_stop_video_checked(&self) -> bool {
        has_sym!(self, xcb_xv_stop_video_checked)
    }

    /// Sends a `Xv::StopVideo` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xv_stop_video(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_stop_video)(c, port, drawable)
    }

    /// Returns `true` iff the symbol `xcb_xv_stop_video` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_stop_video(&self) -> bool {
        has_sym!(self, xcb_xv_stop_video)
    }

    /// Sends a `Xv::SelectVideoNotify` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xv_select_video_notify_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        onoff: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_select_video_notify_checked)(c, drawable, onoff)
    }

    /// Returns `true` iff the symbol `xcb_xv_select_video_notify_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_select_video_notify_checked(&self) -> bool {
        has_sym!(self, xcb_xv_select_video_notify_checked)
    }

    /// Sends a `Xv::SelectVideoNotify` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xv_select_video_notify(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        onoff: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_select_video_notify)(c, drawable, onoff)
    }

    /// Returns `true` iff the symbol `xcb_xv_select_video_notify` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_select_video_notify(&self) -> bool {
        has_sym!(self, xcb_xv_select_video_notify)
    }

    /// Sends a `Xv::SelectPortNotify` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xv_select_port_notify_checked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        onoff: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_select_port_notify_checked)(c, port, onoff)
    }

    /// Returns `true` iff the symbol `xcb_xv_select_port_notify_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_select_port_notify_checked(&self) -> bool {
        has_sym!(self, xcb_xv_select_port_notify_checked)
    }

    /// Sends a `Xv::SelectPortNotify` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xv_select_port_notify(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        onoff: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_select_port_notify)(c, port, onoff)
    }

    /// Returns `true` iff the symbol `xcb_xv_select_port_notify` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_select_port_notify(&self) -> bool {
        has_sym!(self, xcb_xv_select_port_notify)
    }

    /// Sends a `Xv::QueryBestSize` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xv_query_best_size_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xv_query_best_size_reply`]: Self::xcb_xv_query_best_size_reply
    #[inline]
    pub unsafe fn xcb_xv_query_best_size(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        vid_w: u16,
        vid_h: u16,
        drw_w: u16,
        drw_h: u16,
        motion: u8,
    ) -> xcb_xv_query_best_size_cookie_t {
        sym!(self, xcb_xv_query_best_size)(c, port, vid_w, vid_h, drw_w, drw_h, motion)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_best_size` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_best_size(&self) -> bool {
        has_sym!(self, xcb_xv_query_best_size)
    }

    /// Sends a `Xv::QueryBestSize` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xv_query_best_size_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xv_query_best_size_reply`]: Self::xcb_xv_query_best_size_reply
    #[inline]
    pub unsafe fn xcb_xv_query_best_size_unchecked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        vid_w: u16,
        vid_h: u16,
        drw_w: u16,
        drw_h: u16,
        motion: u8,
    ) -> xcb_xv_query_best_size_cookie_t {
        sym!(self, xcb_xv_query_best_size_unchecked)(c, port, vid_w, vid_h, drw_w, drw_h, motion)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_best_size_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_best_size_unchecked(&self) -> bool {
        has_sym!(self, xcb_xv_query_best_size_unchecked)
    }

    /// Waits for the reply to a `Xv::QueryBestSize` request.
    #[inline]
    pub unsafe fn xcb_xv_query_best_size_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xv_query_best_size_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xv_query_best_size_reply_t {
        sym!(self, xcb_xv_query_best_size_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_best_size_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_best_size_reply(&self) -> bool {
        has_sym!(self, xcb_xv_query_best_size_reply)
    }

    /// Sends a `Xv::SetPortAttribute` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xv_set_port_attribute_checked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        attribute: xcb_atom_t,
        value: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_set_port_attribute_checked)(c, port, attribute, value)
    }

    /// Returns `true` iff the symbol `xcb_xv_set_port_attribute_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_set_port_attribute_checked(&self) -> bool {
        has_sym!(self, xcb_xv_set_port_attribute_checked)
    }

    /// Sends a `Xv::SetPortAttribute` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xv_set_port_attribute(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        attribute: xcb_atom_t,
        value: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_set_port_attribute)(c, port, attribute, value)
    }

    /// Returns `true` iff the symbol `xcb_xv_set_port_attribute` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_set_port_attribute(&self) -> bool {
        has_sym!(self, xcb_xv_set_port_attribute)
    }

    /// Sends a `Xv::GetPortAttribute` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xv_get_port_attribute_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xv_get_port_attribute_reply`]: Self::xcb_xv_get_port_attribute_reply
    #[inline]
    pub unsafe fn xcb_xv_get_port_attribute(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        attribute: xcb_atom_t,
    ) -> xcb_xv_get_port_attribute_cookie_t {
        sym!(self, xcb_xv_get_port_attribute)(c, port, attribute)
    }

    /// Returns `true` iff the symbol `xcb_xv_get_port_attribute` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_get_port_attribute(&self) -> bool {
        has_sym!(self, xcb_xv_get_port_attribute)
    }

    /// Sends a `Xv::GetPortAttribute` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xv_get_port_attribute_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xv_get_port_attribute_reply`]: Self::xcb_xv_get_port_attribute_reply
    #[inline]
    pub unsafe fn xcb_xv_get_port_attribute_unchecked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        attribute: xcb_atom_t,
    ) -> xcb_xv_get_port_attribute_cookie_t {
        sym!(self, xcb_xv_get_port_attribute_unchecked)(c, port, attribute)
    }

    /// Returns `true` iff the symbol `xcb_xv_get_port_attribute_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_get_port_attribute_unchecked(&self) -> bool {
        has_sym!(self, xcb_xv_get_port_attribute_unchecked)
    }

    /// Waits for the reply to a `Xv::GetPortAttribute` request.
    #[inline]
    pub unsafe fn xcb_xv_get_port_attribute_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xv_get_port_attribute_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xv_get_port_attribute_reply_t {
        sym!(self, xcb_xv_get_port_attribute_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xv_get_port_attribute_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_get_port_attribute_reply(&self) -> bool {
        has_sym!(self, xcb_xv_get_port_attribute_reply)
    }

    /// Computes the size of a `xcb_xv_query_port_attributes_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xv_query_port_attributes_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xv_query_port_attributes_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_port_attributes_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_port_attributes_sizeof(&self) -> bool {
        has_sym!(self, xcb_xv_query_port_attributes_sizeof)
    }

    /// Sends a `Xv::QueryPortAttributes` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xv_query_port_attributes_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xv_query_port_attributes_reply`]: Self::xcb_xv_query_port_attributes_reply
    #[inline]
    pub unsafe fn xcb_xv_query_port_attributes(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
    ) -> xcb_xv_query_port_attributes_cookie_t {
        sym!(self, xcb_xv_query_port_attributes)(c, port)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_port_attributes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_port_attributes(&self) -> bool {
        has_sym!(self, xcb_xv_query_port_attributes)
    }

    /// Sends a `Xv::QueryPortAttributes` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xv_query_port_attributes_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xv_query_port_attributes_reply`]: Self::xcb_xv_query_port_attributes_reply
    #[inline]
    pub unsafe fn xcb_xv_query_port_attributes_unchecked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
    ) -> xcb_xv_query_port_attributes_cookie_t {
        sym!(self, xcb_xv_query_port_attributes_unchecked)(c, port)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_port_attributes_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_port_attributes_unchecked(&self) -> bool {
        has_sym!(self, xcb_xv_query_port_attributes_unchecked)
    }

    /// Returns the number of elements of the `attributes` field of a `xcb_xv_query_port_attributes_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_query_port_attributes_attributes_length(
        &self,
        r: *const xcb_xv_query_port_attributes_reply_t,
    ) -> c_int {
        sym!(self, xcb_xv_query_port_attributes_attributes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_port_attributes_attributes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_port_attributes_attributes_length(&self) -> bool {
        has_sym!(self, xcb_xv_query_port_attributes_attributes_length)
    }

    /// Returns an iterator over the elements of the
    /// `attributes` field of a `xcb_xv_query_port_attributes_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_query_port_attributes_attributes_iterator(
        &self,
        r: *const xcb_xv_query_port_attributes_reply_t,
    ) -> xcb_xv_attribute_info_iterator_t {
        sym!(self, xcb_xv_query_port_attributes_attributes_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_port_attributes_attributes_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_port_attributes_attributes_iterator(&self) -> bool {
        has_sym!(self, xcb_xv_query_port_attributes_attributes_iterator)
    }

    /// Waits for the reply to a `Xv::QueryPortAttributes` request.
    #[inline]
    pub unsafe fn xcb_xv_query_port_attributes_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xv_query_port_attributes_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xv_query_port_attributes_reply_t {
        sym!(self, xcb_xv_query_port_attributes_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_port_attributes_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_port_attributes_reply(&self) -> bool {
        has_sym!(self, xcb_xv_query_port_attributes_reply)
    }

    /// Computes the size of a `xcb_xv_list_image_formats_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xv_list_image_formats_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xv_list_image_formats_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xv_list_image_formats_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_list_image_formats_sizeof(&self) -> bool {
        has_sym!(self, xcb_xv_list_image_formats_sizeof)
    }

    /// Sends a `Xv::ListImageFormats` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xv_list_image_formats_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xv_list_image_formats_reply`]: Self::xcb_xv_list_image_formats_reply
    #[inline]
    pub unsafe fn xcb_xv_list_image_formats(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
    ) -> xcb_xv_list_image_formats_cookie_t {
        sym!(self, xcb_xv_list_image_formats)(c, port)
    }

    /// Returns `true` iff the symbol `xcb_xv_list_image_formats` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_list_image_formats(&self) -> bool {
        has_sym!(self, xcb_xv_list_image_formats)
    }

    /// Sends a `Xv::ListImageFormats` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xv_list_image_formats_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xv_list_image_formats_reply`]: Self::xcb_xv_list_image_formats_reply
    #[inline]
    pub unsafe fn xcb_xv_list_image_formats_unchecked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
    ) -> xcb_xv_list_image_formats_cookie_t {
        sym!(self, xcb_xv_list_image_formats_unchecked)(c, port)
    }

    /// Returns `true` iff the symbol `xcb_xv_list_image_formats_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_list_image_formats_unchecked(&self) -> bool {
        has_sym!(self, xcb_xv_list_image_formats_unchecked)
    }

    /// Returns a pointer to the `format` field of a `xcb_xv_list_image_formats_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_list_image_formats_format(
        &self,
        r: *const xcb_xv_list_image_formats_reply_t,
    ) -> *mut xcb_xv_image_format_info_t {
        sym!(self, xcb_xv_list_image_formats_format)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_list_image_formats_format` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_list_image_formats_format(&self) -> bool {
        has_sym!(self, xcb_xv_list_image_formats_format)
    }

    /// Returns the number of elements of the `format` field of a `xcb_xv_list_image_formats_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_list_image_formats_format_length(
        &self,
        r: *const xcb_xv_list_image_formats_reply_t,
    ) -> c_int {
        sym!(self, xcb_xv_list_image_formats_format_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_list_image_formats_format_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_list_image_formats_format_length(&self) -> bool {
        has_sym!(self, xcb_xv_list_image_formats_format_length)
    }

    /// Returns an iterator over the elements of the
    /// `format` field of a `xcb_xv_list_image_formats_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_list_image_formats_format_iterator(
        &self,
        r: *const xcb_xv_list_image_formats_reply_t,
    ) -> xcb_xv_image_format_info_iterator_t {
        sym!(self, xcb_xv_list_image_formats_format_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_list_image_formats_format_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_list_image_formats_format_iterator(&self) -> bool {
        has_sym!(self, xcb_xv_list_image_formats_format_iterator)
    }

    /// Waits for the reply to a `Xv::ListImageFormats` request.
    #[inline]
    pub unsafe fn xcb_xv_list_image_formats_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xv_list_image_formats_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xv_list_image_formats_reply_t {
        sym!(self, xcb_xv_list_image_formats_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xv_list_image_formats_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_list_image_formats_reply(&self) -> bool {
        has_sym!(self, xcb_xv_list_image_formats_reply)
    }

    /// Computes the size of a `xcb_xv_query_image_attributes_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xv_query_image_attributes_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xv_query_image_attributes_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_image_attributes_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_image_attributes_sizeof(&self) -> bool {
        has_sym!(self, xcb_xv_query_image_attributes_sizeof)
    }

    /// Sends a `Xv::QueryImageAttributes` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xv_query_image_attributes_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xv_query_image_attributes_reply`]: Self::xcb_xv_query_image_attributes_reply
    #[inline]
    pub unsafe fn xcb_xv_query_image_attributes(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        id: u32,
        width: u16,
        height: u16,
    ) -> xcb_xv_query_image_attributes_cookie_t {
        sym!(self, xcb_xv_query_image_attributes)(c, port, id, width, height)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_image_attributes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_image_attributes(&self) -> bool {
        has_sym!(self, xcb_xv_query_image_attributes)
    }

    /// Sends a `Xv::QueryImageAttributes` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xv_query_image_attributes_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xv_query_image_attributes_reply`]: Self::xcb_xv_query_image_attributes_reply
    #[inline]
    pub unsafe fn xcb_xv_query_image_attributes_unchecked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        id: u32,
        width: u16,
        height: u16,
    ) -> xcb_xv_query_image_attributes_cookie_t {
        sym!(self, xcb_xv_query_image_attributes_unchecked)(c, port, id, width, height)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_image_attributes_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_image_attributes_unchecked(&self) -> bool {
        has_sym!(self, xcb_xv_query_image_attributes_unchecked)
    }

    /// Returns a pointer to the `pitches` field of a `xcb_xv_query_image_attributes_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_query_image_attributes_pitches(
        &self,
        r: *const xcb_xv_query_image_attributes_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_xv_query_image_attributes_pitches)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_image_attributes_pitches` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_image_attributes_pitches(&self) -> bool {
        has_sym!(self, xcb_xv_query_image_attributes_pitches)
    }

    /// Returns the number of elements of the `pitches` field of a `xcb_xv_query_image_attributes_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_query_image_attributes_pitches_length(
        &self,
        r: *const xcb_xv_query_image_attributes_reply_t,
    ) -> c_int {
        sym!(self, xcb_xv_query_image_attributes_pitches_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_image_attributes_pitches_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_image_attributes_pitches_length(&self) -> bool {
        has_sym!(self, xcb_xv_query_image_attributes_pitches_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `pitches` field of a `xcb_xv_query_image_attributes_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_query_image_attributes_pitches_end(
        &self,
        r: *const xcb_xv_query_image_attributes_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_query_image_attributes_pitches_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_image_attributes_pitches_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_image_attributes_pitches_end(&self) -> bool {
        has_sym!(self, xcb_xv_query_image_attributes_pitches_end)
    }

    /// Returns a pointer to the `offsets` field of a `xcb_xv_query_image_attributes_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_query_image_attributes_offsets(
        &self,
        r: *const xcb_xv_query_image_attributes_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_xv_query_image_attributes_offsets)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_image_attributes_offsets` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_image_attributes_offsets(&self) -> bool {
        has_sym!(self, xcb_xv_query_image_attributes_offsets)
    }

    /// Returns the number of elements of the `offsets` field of a `xcb_xv_query_image_attributes_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_query_image_attributes_offsets_length(
        &self,
        r: *const xcb_xv_query_image_attributes_reply_t,
    ) -> c_int {
        sym!(self, xcb_xv_query_image_attributes_offsets_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_image_attributes_offsets_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_image_attributes_offsets_length(&self) -> bool {
        has_sym!(self, xcb_xv_query_image_attributes_offsets_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `offsets` field of a `xcb_xv_query_image_attributes_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_query_image_attributes_offsets_end(
        &self,
        r: *const xcb_xv_query_image_attributes_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_query_image_attributes_offsets_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_image_attributes_offsets_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_image_attributes_offsets_end(&self) -> bool {
        has_sym!(self, xcb_xv_query_image_attributes_offsets_end)
    }

    /// Waits for the reply to a `Xv::QueryImageAttributes` request.
    #[inline]
    pub unsafe fn xcb_xv_query_image_attributes_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xv_query_image_attributes_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xv_query_image_attributes_reply_t {
        sym!(self, xcb_xv_query_image_attributes_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xv_query_image_attributes_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_query_image_attributes_reply(&self) -> bool {
        has_sym!(self, xcb_xv_query_image_attributes_reply)
    }

    /// Computes the size of a `xcb_xv_put_image_request_t` object.
    #[inline]
    pub unsafe fn xcb_xv_put_image_sizeof(&self, _buffer: *const c_void, data_len: u32) -> c_int {
        sym!(self, xcb_xv_put_image_sizeof)(_buffer, data_len)
    }

    /// Returns `true` iff the symbol `xcb_xv_put_image_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_put_image_sizeof(&self) -> bool {
        has_sym!(self, xcb_xv_put_image_sizeof)
    }

    /// Sends a `Xv::PutImage` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xv_put_image_checked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        id: u32,
        src_x: i16,
        src_y: i16,
        src_w: u16,
        src_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
        width: u16,
        height: u16,
        data_len: u32,
        data: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_put_image_checked)(
            c, port, drawable, gc, id, src_x, src_y, src_w, src_h, drw_x, drw_y, drw_w, drw_h,
            width, height, data_len, data,
        )
    }

    /// Returns `true` iff the symbol `xcb_xv_put_image_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_put_image_checked(&self) -> bool {
        has_sym!(self, xcb_xv_put_image_checked)
    }

    /// Sends a `Xv::PutImage` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xv_put_image(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        id: u32,
        src_x: i16,
        src_y: i16,
        src_w: u16,
        src_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
        width: u16,
        height: u16,
        data_len: u32,
        data: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_put_image)(
            c, port, drawable, gc, id, src_x, src_y, src_w, src_h, drw_x, drw_y, drw_w, drw_h,
            width, height, data_len, data,
        )
    }

    /// Returns `true` iff the symbol `xcb_xv_put_image` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_put_image(&self) -> bool {
        has_sym!(self, xcb_xv_put_image)
    }

    /// Returns a pointer to the `data` field of a `xcb_xv_put_image_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_put_image_data(&self, r: *const xcb_xv_put_image_request_t) -> *mut u8 {
        sym!(self, xcb_xv_put_image_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_put_image_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_put_image_data(&self) -> bool {
        has_sym!(self, xcb_xv_put_image_data)
    }

    /// Returns the number of elements of the `data` field of a `xcb_xv_put_image_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_put_image_data_length(
        &self,
        r: *const xcb_xv_put_image_request_t,
    ) -> c_int {
        sym!(self, xcb_xv_put_image_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_put_image_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_put_image_data_length(&self) -> bool {
        has_sym!(self, xcb_xv_put_image_data_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `data` field of a `xcb_xv_put_image_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xv_put_image_data_end(
        &self,
        r: *const xcb_xv_put_image_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_put_image_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xv_put_image_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_put_image_data_end(&self) -> bool {
        has_sym!(self, xcb_xv_put_image_data_end)
    }

    /// Sends a `Xv::ShmPutImage` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xv_shm_put_image_checked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        shmseg: xcb_shm_seg_t,
        id: u32,
        offset: u32,
        src_x: i16,
        src_y: i16,
        src_w: u16,
        src_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
        width: u16,
        height: u16,
        send_event: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_shm_put_image_checked)(
            c, port, drawable, gc, shmseg, id, offset, src_x, src_y, src_w, src_h, drw_x, drw_y,
            drw_w, drw_h, width, height, send_event,
        )
    }

    /// Returns `true` iff the symbol `xcb_xv_shm_put_image_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_shm_put_image_checked(&self) -> bool {
        has_sym!(self, xcb_xv_shm_put_image_checked)
    }

    /// Sends a `Xv::ShmPutImage` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xv_shm_put_image(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        shmseg: xcb_shm_seg_t,
        id: u32,
        offset: u32,
        src_x: i16,
        src_y: i16,
        src_w: u16,
        src_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
        width: u16,
        height: u16,
        send_event: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_shm_put_image)(
            c, port, drawable, gc, shmseg, id, offset, src_x, src_y, src_w, src_h, drw_x, drw_y,
            drw_w, drw_h, width, height, send_event,
        )
    }

    /// Returns `true` iff the symbol `xcb_xv_shm_put_image` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xv_shm_put_image(&self) -> bool {
        has_sym!(self, xcb_xv_shm_put_image)
    }
}

#[cfg(feature = "xcb_xv")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbXv::load().unwrap() };
        assert!(lib.has_xcb_xv_id());
        assert!(lib.has_xcb_xv_port_next());
        assert!(lib.has_xcb_xv_port_end());
        assert!(lib.has_xcb_xv_encoding_next());
        assert!(lib.has_xcb_xv_encoding_end());
        assert!(lib.has_xcb_xv_rational_next());
        assert!(lib.has_xcb_xv_rational_end());
        assert!(lib.has_xcb_xv_format_next());
        assert!(lib.has_xcb_xv_format_end());
        assert!(lib.has_xcb_xv_adaptor_info_sizeof());
        assert!(lib.has_xcb_xv_adaptor_info_name());
        assert!(lib.has_xcb_xv_adaptor_info_name_length());
        assert!(lib.has_xcb_xv_adaptor_info_name_end());
        assert!(lib.has_xcb_xv_adaptor_info_formats());
        assert!(lib.has_xcb_xv_adaptor_info_formats_length());
        assert!(lib.has_xcb_xv_adaptor_info_formats_iterator());
        assert!(lib.has_xcb_xv_adaptor_info_next());
        assert!(lib.has_xcb_xv_adaptor_info_end());
        assert!(lib.has_xcb_xv_encoding_info_sizeof());
        assert!(lib.has_xcb_xv_encoding_info_name());
        assert!(lib.has_xcb_xv_encoding_info_name_length());
        assert!(lib.has_xcb_xv_encoding_info_name_end());
        assert!(lib.has_xcb_xv_encoding_info_next());
        assert!(lib.has_xcb_xv_encoding_info_end());
        assert!(lib.has_xcb_xv_image_sizeof());
        assert!(lib.has_xcb_xv_image_pitches());
        assert!(lib.has_xcb_xv_image_pitches_length());
        assert!(lib.has_xcb_xv_image_pitches_end());
        assert!(lib.has_xcb_xv_image_offsets());
        assert!(lib.has_xcb_xv_image_offsets_length());
        assert!(lib.has_xcb_xv_image_offsets_end());
        assert!(lib.has_xcb_xv_image_data());
        assert!(lib.has_xcb_xv_image_data_length());
        assert!(lib.has_xcb_xv_image_data_end());
        assert!(lib.has_xcb_xv_image_next());
        assert!(lib.has_xcb_xv_image_end());
        assert!(lib.has_xcb_xv_attribute_info_sizeof());
        assert!(lib.has_xcb_xv_attribute_info_name());
        assert!(lib.has_xcb_xv_attribute_info_name_length());
        assert!(lib.has_xcb_xv_attribute_info_name_end());
        assert!(lib.has_xcb_xv_attribute_info_next());
        assert!(lib.has_xcb_xv_attribute_info_end());
        assert!(lib.has_xcb_xv_image_format_info_next());
        assert!(lib.has_xcb_xv_image_format_info_end());
        assert!(lib.has_xcb_xv_query_extension());
        assert!(lib.has_xcb_xv_query_extension_unchecked());
        assert!(lib.has_xcb_xv_query_extension_reply());
        assert!(lib.has_xcb_xv_query_adaptors_sizeof());
        assert!(lib.has_xcb_xv_query_adaptors());
        assert!(lib.has_xcb_xv_query_adaptors_unchecked());
        assert!(lib.has_xcb_xv_query_adaptors_info_length());
        assert!(lib.has_xcb_xv_query_adaptors_info_iterator());
        assert!(lib.has_xcb_xv_query_adaptors_reply());
        assert!(lib.has_xcb_xv_query_encodings_sizeof());
        assert!(lib.has_xcb_xv_query_encodings());
        assert!(lib.has_xcb_xv_query_encodings_unchecked());
        assert!(lib.has_xcb_xv_query_encodings_info_length());
        assert!(lib.has_xcb_xv_query_encodings_info_iterator());
        assert!(lib.has_xcb_xv_query_encodings_reply());
        assert!(lib.has_xcb_xv_grab_port());
        assert!(lib.has_xcb_xv_grab_port_unchecked());
        assert!(lib.has_xcb_xv_grab_port_reply());
        assert!(lib.has_xcb_xv_ungrab_port_checked());
        assert!(lib.has_xcb_xv_ungrab_port());
        assert!(lib.has_xcb_xv_put_video_checked());
        assert!(lib.has_xcb_xv_put_video());
        assert!(lib.has_xcb_xv_put_still_checked());
        assert!(lib.has_xcb_xv_put_still());
        assert!(lib.has_xcb_xv_get_video_checked());
        assert!(lib.has_xcb_xv_get_video());
        assert!(lib.has_xcb_xv_get_still_checked());
        assert!(lib.has_xcb_xv_get_still());
        assert!(lib.has_xcb_xv_stop_video_checked());
        assert!(lib.has_xcb_xv_stop_video());
        assert!(lib.has_xcb_xv_select_video_notify_checked());
        assert!(lib.has_xcb_xv_select_video_notify());
        assert!(lib.has_xcb_xv_select_port_notify_checked());
        assert!(lib.has_xcb_xv_select_port_notify());
        assert!(lib.has_xcb_xv_query_best_size());
        assert!(lib.has_xcb_xv_query_best_size_unchecked());
        assert!(lib.has_xcb_xv_query_best_size_reply());
        assert!(lib.has_xcb_xv_set_port_attribute_checked());
        assert!(lib.has_xcb_xv_set_port_attribute());
        assert!(lib.has_xcb_xv_get_port_attribute());
        assert!(lib.has_xcb_xv_get_port_attribute_unchecked());
        assert!(lib.has_xcb_xv_get_port_attribute_reply());
        assert!(lib.has_xcb_xv_query_port_attributes_sizeof());
        assert!(lib.has_xcb_xv_query_port_attributes());
        assert!(lib.has_xcb_xv_query_port_attributes_unchecked());
        assert!(lib.has_xcb_xv_query_port_attributes_attributes_length());
        assert!(lib.has_xcb_xv_query_port_attributes_attributes_iterator());
        assert!(lib.has_xcb_xv_query_port_attributes_reply());
        assert!(lib.has_xcb_xv_list_image_formats_sizeof());
        assert!(lib.has_xcb_xv_list_image_formats());
        assert!(lib.has_xcb_xv_list_image_formats_unchecked());
        assert!(lib.has_xcb_xv_list_image_formats_format());
        assert!(lib.has_xcb_xv_list_image_formats_format_length());
        assert!(lib.has_xcb_xv_list_image_formats_format_iterator());
        assert!(lib.has_xcb_xv_list_image_formats_reply());
        assert!(lib.has_xcb_xv_query_image_attributes_sizeof());
        assert!(lib.has_xcb_xv_query_image_attributes());
        assert!(lib.has_xcb_xv_query_image_attributes_unchecked());
        assert!(lib.has_xcb_xv_query_image_attributes_pitches());
        assert!(lib.has_xcb_xv_query_image_attributes_pitches_length());
        assert!(lib.has_xcb_xv_query_image_attributes_pitches_end());
        assert!(lib.has_xcb_xv_query_image_attributes_offsets());
        assert!(lib.has_xcb_xv_query_image_attributes_offsets_length());
        assert!(lib.has_xcb_xv_query_image_attributes_offsets_end());
        assert!(lib.has_xcb_xv_query_image_attributes_reply());
        assert!(lib.has_xcb_xv_put_image_sizeof());
        assert!(lib.has_xcb_xv_put_image_checked());
        assert!(lib.has_xcb_xv_put_image());
        assert!(lib.has_xcb_xv_put_image_data());
        assert!(lib.has_xcb_xv_put_image_data_length());
        assert!(lib.has_xcb_xv_put_image_data_end());
        assert!(lib.has_xcb_xv_shm_put_image_checked());
        assert!(lib.has_xcb_xv_shm_put_image());
    }
}
