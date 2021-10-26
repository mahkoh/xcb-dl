// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The `RandR::MODE` type.
pub type xcb_randr_mode_t = u32;

/// An iterator over `RandR::MODE` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_mode_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_randr_mode_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_randr_mode_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::CRTC` type.
pub type xcb_randr_crtc_t = u32;

/// An iterator over `RandR::CRTC` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_crtc_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_randr_crtc_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_randr_crtc_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::OUTPUT` type.
pub type xcb_randr_output_t = u32;

/// An iterator over `RandR::OUTPUT` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_output_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_randr_output_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_randr_output_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::PROVIDER` type.
pub type xcb_randr_provider_t = u32;

/// An iterator over `RandR::PROVIDER` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_provider_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_randr_provider_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_randr_provider_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::LEASE` type.
pub type xcb_randr_lease_t = u32;

/// An iterator over `RandR::LEASE` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_lease_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_randr_lease_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_randr_lease_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::BadOutput` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_randr_bad_output_error_t`].
pub const XCB_RANDR_BAD_OUTPUT: u8 = 0i32 as u8;

/// The `RandR::BadOutput` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_bad_output_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_randr_bad_output_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::BadCrtc` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_randr_bad_crtc_error_t`].
pub const XCB_RANDR_BAD_CRTC: u8 = 1i32 as u8;

/// The `RandR::BadCrtc` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_bad_crtc_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_randr_bad_crtc_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::BadMode` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_randr_bad_mode_error_t`].
pub const XCB_RANDR_BAD_MODE: u8 = 2i32 as u8;

/// The `RandR::BadMode` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_bad_mode_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_randr_bad_mode_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::BadProvider` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_randr_bad_provider_error_t`].
pub const XCB_RANDR_BAD_PROVIDER: u8 = 3i32 as u8;

/// The `RandR::BadProvider` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_bad_provider_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_randr_bad_provider_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::Rotation` enum.
///
/// This enum has the following variants:
///
/// - [`RandR::Rotation::Rotate_0`](XCB_RANDR_ROTATION_ROTATE_0)
/// - [`RandR::Rotation::Rotate_90`](XCB_RANDR_ROTATION_ROTATE_90)
/// - [`RandR::Rotation::Rotate_180`](XCB_RANDR_ROTATION_ROTATE_180)
/// - [`RandR::Rotation::Rotate_270`](XCB_RANDR_ROTATION_ROTATE_270)
/// - [`RandR::Rotation::Reflect_X`](XCB_RANDR_ROTATION_REFLECT_X)
/// - [`RandR::Rotation::Reflect_Y`](XCB_RANDR_ROTATION_REFLECT_Y)
pub type xcb_randr_rotation_t = u32;
/// The `RandR::Rotation::Rotate_0` enum variant.
///
/// This is a variant of [`xcb_randr_rotation_t`].
pub const XCB_RANDR_ROTATION_ROTATE_0: xcb_randr_rotation_t = 1;
/// The `RandR::Rotation::Rotate_90` enum variant.
///
/// This is a variant of [`xcb_randr_rotation_t`].
pub const XCB_RANDR_ROTATION_ROTATE_90: xcb_randr_rotation_t = 2;
/// The `RandR::Rotation::Rotate_180` enum variant.
///
/// This is a variant of [`xcb_randr_rotation_t`].
pub const XCB_RANDR_ROTATION_ROTATE_180: xcb_randr_rotation_t = 4;
/// The `RandR::Rotation::Rotate_270` enum variant.
///
/// This is a variant of [`xcb_randr_rotation_t`].
pub const XCB_RANDR_ROTATION_ROTATE_270: xcb_randr_rotation_t = 8;
/// The `RandR::Rotation::Reflect_X` enum variant.
///
/// This is a variant of [`xcb_randr_rotation_t`].
pub const XCB_RANDR_ROTATION_REFLECT_X: xcb_randr_rotation_t = 16;
/// The `RandR::Rotation::Reflect_Y` enum variant.
///
/// This is a variant of [`xcb_randr_rotation_t`].
pub const XCB_RANDR_ROTATION_REFLECT_Y: xcb_randr_rotation_t = 32;

/// The `RandR::ScreenSize` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_screen_size_t {
    pub width: u16,
    pub height: u16,
    pub mwidth: u16,
    pub mheight: u16,
}

impl Default for xcb_randr_screen_size_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `RandR::ScreenSize` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_screen_size_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_randr_screen_size_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_randr_screen_size_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::RefreshRates` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `rates`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_refresh_rates_t {
    pub n_rates: u16,
}

impl Default for xcb_randr_refresh_rates_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `RandR::RefreshRates` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_refresh_rates_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_randr_refresh_rates_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_randr_refresh_rates_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `RandR::QueryVersion` request.
///
/// Pass this cookie to [`xcb_randr_query_version_reply`] to retrieve the reply.
///
/// [`xcb_randr_query_version_reply`]: XcbRandr::xcb_randr_query_version_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_query_version_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::QueryVersion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_query_version_request_t`].
pub const XCB_RANDR_QUERY_VERSION: u8 = 0i32 as u8;

/// The `RandR::QueryVersion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub major_version: u32,
    pub minor_version: u32,
}

impl Default for xcb_randr_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::QueryVersion` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
    pub pad1: [u8; 16],
}

impl Default for xcb_randr_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::SetConfig` enum.
///
/// This enum has the following variants:
///
/// - [`RandR::SetConfig::Success`](XCB_RANDR_SET_CONFIG_SUCCESS)
/// - [`RandR::SetConfig::InvalidConfigTime`](XCB_RANDR_SET_CONFIG_INVALID_CONFIG_TIME)
/// - [`RandR::SetConfig::InvalidTime`](XCB_RANDR_SET_CONFIG_INVALID_TIME)
/// - [`RandR::SetConfig::Failed`](XCB_RANDR_SET_CONFIG_FAILED)
pub type xcb_randr_set_config_t = u32;
/// The `RandR::SetConfig::Success` enum variant.
///
/// This is a variant of [`xcb_randr_set_config_t`].
pub const XCB_RANDR_SET_CONFIG_SUCCESS: xcb_randr_set_config_t = 0;
/// The `RandR::SetConfig::InvalidConfigTime` enum variant.
///
/// This is a variant of [`xcb_randr_set_config_t`].
pub const XCB_RANDR_SET_CONFIG_INVALID_CONFIG_TIME: xcb_randr_set_config_t = 1;
/// The `RandR::SetConfig::InvalidTime` enum variant.
///
/// This is a variant of [`xcb_randr_set_config_t`].
pub const XCB_RANDR_SET_CONFIG_INVALID_TIME: xcb_randr_set_config_t = 2;
/// The `RandR::SetConfig::Failed` enum variant.
///
/// This is a variant of [`xcb_randr_set_config_t`].
pub const XCB_RANDR_SET_CONFIG_FAILED: xcb_randr_set_config_t = 3;

/// The cookie for the reply to a `RandR::SetScreenConfig` request.
///
/// Pass this cookie to [`xcb_randr_set_screen_config_reply`] to retrieve the reply.
///
/// [`xcb_randr_set_screen_config_reply`]: XcbRandr::xcb_randr_set_screen_config_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_screen_config_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_set_screen_config_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::SetScreenConfig` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_set_screen_config_request_t`].
pub const XCB_RANDR_SET_SCREEN_CONFIG: u8 = 2i32 as u8;

/// The `RandR::SetScreenConfig` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_screen_config_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub timestamp: xcb_timestamp_t,
    pub config_timestamp: xcb_timestamp_t,
    pub size_i_d: u16,
    pub rotation: u16,
    pub rate: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_randr_set_screen_config_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::SetScreenConfig` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_screen_config_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
    pub new_timestamp: xcb_timestamp_t,
    pub config_timestamp: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub subpixel_order: u16,
    pub pad0: [u8; 10],
}

impl Default for xcb_randr_set_screen_config_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::NotifyMask` enum.
///
/// This enum has the following variants:
///
/// - [`RandR::NotifyMask::ScreenChange`](XCB_RANDR_NOTIFY_MASK_SCREEN_CHANGE)
/// - [`RandR::NotifyMask::CrtcChange`](XCB_RANDR_NOTIFY_MASK_CRTC_CHANGE)
/// - [`RandR::NotifyMask::OutputChange`](XCB_RANDR_NOTIFY_MASK_OUTPUT_CHANGE)
/// - [`RandR::NotifyMask::OutputProperty`](XCB_RANDR_NOTIFY_MASK_OUTPUT_PROPERTY)
/// - [`RandR::NotifyMask::ProviderChange`](XCB_RANDR_NOTIFY_MASK_PROVIDER_CHANGE)
/// - [`RandR::NotifyMask::ProviderProperty`](XCB_RANDR_NOTIFY_MASK_PROVIDER_PROPERTY)
/// - [`RandR::NotifyMask::ResourceChange`](XCB_RANDR_NOTIFY_MASK_RESOURCE_CHANGE)
/// - [`RandR::NotifyMask::Lease`](XCB_RANDR_NOTIFY_MASK_LEASE)
pub type xcb_randr_notify_mask_t = u32;
/// The `RandR::NotifyMask::ScreenChange` enum variant.
///
/// This is a variant of [`xcb_randr_notify_mask_t`].
pub const XCB_RANDR_NOTIFY_MASK_SCREEN_CHANGE: xcb_randr_notify_mask_t = 1;
/// The `RandR::NotifyMask::CrtcChange` enum variant.
///
/// This is a variant of [`xcb_randr_notify_mask_t`].
pub const XCB_RANDR_NOTIFY_MASK_CRTC_CHANGE: xcb_randr_notify_mask_t = 2;
/// The `RandR::NotifyMask::OutputChange` enum variant.
///
/// This is a variant of [`xcb_randr_notify_mask_t`].
pub const XCB_RANDR_NOTIFY_MASK_OUTPUT_CHANGE: xcb_randr_notify_mask_t = 4;
/// The `RandR::NotifyMask::OutputProperty` enum variant.
///
/// This is a variant of [`xcb_randr_notify_mask_t`].
pub const XCB_RANDR_NOTIFY_MASK_OUTPUT_PROPERTY: xcb_randr_notify_mask_t = 8;
/// The `RandR::NotifyMask::ProviderChange` enum variant.
///
/// This is a variant of [`xcb_randr_notify_mask_t`].
pub const XCB_RANDR_NOTIFY_MASK_PROVIDER_CHANGE: xcb_randr_notify_mask_t = 16;
/// The `RandR::NotifyMask::ProviderProperty` enum variant.
///
/// This is a variant of [`xcb_randr_notify_mask_t`].
pub const XCB_RANDR_NOTIFY_MASK_PROVIDER_PROPERTY: xcb_randr_notify_mask_t = 32;
/// The `RandR::NotifyMask::ResourceChange` enum variant.
///
/// This is a variant of [`xcb_randr_notify_mask_t`].
pub const XCB_RANDR_NOTIFY_MASK_RESOURCE_CHANGE: xcb_randr_notify_mask_t = 64;
/// The `RandR::NotifyMask::Lease` enum variant.
///
/// This is a variant of [`xcb_randr_notify_mask_t`].
pub const XCB_RANDR_NOTIFY_MASK_LEASE: xcb_randr_notify_mask_t = 128;

/// The opcode for `RandR::SelectInput` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_select_input_request_t`].
pub const XCB_RANDR_SELECT_INPUT: u8 = 4i32 as u8;

/// The `RandR::SelectInput` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_select_input_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub enable: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_randr_select_input_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `RandR::GetScreenInfo` request.
///
/// Pass this cookie to [`xcb_randr_get_screen_info_reply`] to retrieve the reply.
///
/// [`xcb_randr_get_screen_info_reply`]: XcbRandr::xcb_randr_get_screen_info_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_screen_info_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_get_screen_info_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::GetScreenInfo` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_get_screen_info_request_t`].
pub const XCB_RANDR_GET_SCREEN_INFO: u8 = 5i32 as u8;

/// The `RandR::GetScreenInfo` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_screen_info_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_randr_get_screen_info_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::GetScreenInfo` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `sizes`
/// - `rates`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_screen_info_reply_t {
    pub response_type: u8,
    pub rotations: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: xcb_window_t,
    pub timestamp: xcb_timestamp_t,
    pub config_timestamp: xcb_timestamp_t,
    pub n_sizes: u16,
    pub size_i_d: u16,
    pub rotation: u16,
    pub rate: u16,
    pub n_info: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_randr_get_screen_info_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `RandR::GetScreenSizeRange` request.
///
/// Pass this cookie to [`xcb_randr_get_screen_size_range_reply`] to retrieve the reply.
///
/// [`xcb_randr_get_screen_size_range_reply`]: XcbRandr::xcb_randr_get_screen_size_range_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_screen_size_range_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_get_screen_size_range_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::GetScreenSizeRange` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_get_screen_size_range_request_t`].
pub const XCB_RANDR_GET_SCREEN_SIZE_RANGE: u8 = 6i32 as u8;

/// The `RandR::GetScreenSizeRange` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_screen_size_range_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_randr_get_screen_size_range_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::GetScreenSizeRange` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_screen_size_range_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub min_width: u16,
    pub min_height: u16,
    pub max_width: u16,
    pub max_height: u16,
    pub pad1: [u8; 16],
}

impl Default for xcb_randr_get_screen_size_range_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::SetScreenSize` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_set_screen_size_request_t`].
pub const XCB_RANDR_SET_SCREEN_SIZE: u8 = 7i32 as u8;

/// The `RandR::SetScreenSize` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_screen_size_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub width: u16,
    pub height: u16,
    pub mm_width: u32,
    pub mm_height: u32,
}

impl Default for xcb_randr_set_screen_size_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::ModeFlag` enum.
///
/// This enum has the following variants:
///
/// - [`RandR::ModeFlag::HsyncPositive`](XCB_RANDR_MODE_FLAG_HSYNC_POSITIVE)
/// - [`RandR::ModeFlag::HsyncNegative`](XCB_RANDR_MODE_FLAG_HSYNC_NEGATIVE)
/// - [`RandR::ModeFlag::VsyncPositive`](XCB_RANDR_MODE_FLAG_VSYNC_POSITIVE)
/// - [`RandR::ModeFlag::VsyncNegative`](XCB_RANDR_MODE_FLAG_VSYNC_NEGATIVE)
/// - [`RandR::ModeFlag::Interlace`](XCB_RANDR_MODE_FLAG_INTERLACE)
/// - [`RandR::ModeFlag::DoubleScan`](XCB_RANDR_MODE_FLAG_DOUBLE_SCAN)
/// - [`RandR::ModeFlag::Csync`](XCB_RANDR_MODE_FLAG_CSYNC)
/// - [`RandR::ModeFlag::CsyncPositive`](XCB_RANDR_MODE_FLAG_CSYNC_POSITIVE)
/// - [`RandR::ModeFlag::CsyncNegative`](XCB_RANDR_MODE_FLAG_CSYNC_NEGATIVE)
/// - [`RandR::ModeFlag::HskewPresent`](XCB_RANDR_MODE_FLAG_HSKEW_PRESENT)
/// - [`RandR::ModeFlag::Bcast`](XCB_RANDR_MODE_FLAG_BCAST)
/// - [`RandR::ModeFlag::PixelMultiplex`](XCB_RANDR_MODE_FLAG_PIXEL_MULTIPLEX)
/// - [`RandR::ModeFlag::DoubleClock`](XCB_RANDR_MODE_FLAG_DOUBLE_CLOCK)
/// - [`RandR::ModeFlag::HalveClock`](XCB_RANDR_MODE_FLAG_HALVE_CLOCK)
pub type xcb_randr_mode_flag_t = u32;
/// The `RandR::ModeFlag::HsyncPositive` enum variant.
///
/// This is a variant of [`xcb_randr_mode_flag_t`].
pub const XCB_RANDR_MODE_FLAG_HSYNC_POSITIVE: xcb_randr_mode_flag_t = 1;
/// The `RandR::ModeFlag::HsyncNegative` enum variant.
///
/// This is a variant of [`xcb_randr_mode_flag_t`].
pub const XCB_RANDR_MODE_FLAG_HSYNC_NEGATIVE: xcb_randr_mode_flag_t = 2;
/// The `RandR::ModeFlag::VsyncPositive` enum variant.
///
/// This is a variant of [`xcb_randr_mode_flag_t`].
pub const XCB_RANDR_MODE_FLAG_VSYNC_POSITIVE: xcb_randr_mode_flag_t = 4;
/// The `RandR::ModeFlag::VsyncNegative` enum variant.
///
/// This is a variant of [`xcb_randr_mode_flag_t`].
pub const XCB_RANDR_MODE_FLAG_VSYNC_NEGATIVE: xcb_randr_mode_flag_t = 8;
/// The `RandR::ModeFlag::Interlace` enum variant.
///
/// This is a variant of [`xcb_randr_mode_flag_t`].
pub const XCB_RANDR_MODE_FLAG_INTERLACE: xcb_randr_mode_flag_t = 16;
/// The `RandR::ModeFlag::DoubleScan` enum variant.
///
/// This is a variant of [`xcb_randr_mode_flag_t`].
pub const XCB_RANDR_MODE_FLAG_DOUBLE_SCAN: xcb_randr_mode_flag_t = 32;
/// The `RandR::ModeFlag::Csync` enum variant.
///
/// This is a variant of [`xcb_randr_mode_flag_t`].
pub const XCB_RANDR_MODE_FLAG_CSYNC: xcb_randr_mode_flag_t = 64;
/// The `RandR::ModeFlag::CsyncPositive` enum variant.
///
/// This is a variant of [`xcb_randr_mode_flag_t`].
pub const XCB_RANDR_MODE_FLAG_CSYNC_POSITIVE: xcb_randr_mode_flag_t = 128;
/// The `RandR::ModeFlag::CsyncNegative` enum variant.
///
/// This is a variant of [`xcb_randr_mode_flag_t`].
pub const XCB_RANDR_MODE_FLAG_CSYNC_NEGATIVE: xcb_randr_mode_flag_t = 256;
/// The `RandR::ModeFlag::HskewPresent` enum variant.
///
/// This is a variant of [`xcb_randr_mode_flag_t`].
pub const XCB_RANDR_MODE_FLAG_HSKEW_PRESENT: xcb_randr_mode_flag_t = 512;
/// The `RandR::ModeFlag::Bcast` enum variant.
///
/// This is a variant of [`xcb_randr_mode_flag_t`].
pub const XCB_RANDR_MODE_FLAG_BCAST: xcb_randr_mode_flag_t = 1024;
/// The `RandR::ModeFlag::PixelMultiplex` enum variant.
///
/// This is a variant of [`xcb_randr_mode_flag_t`].
pub const XCB_RANDR_MODE_FLAG_PIXEL_MULTIPLEX: xcb_randr_mode_flag_t = 2048;
/// The `RandR::ModeFlag::DoubleClock` enum variant.
///
/// This is a variant of [`xcb_randr_mode_flag_t`].
pub const XCB_RANDR_MODE_FLAG_DOUBLE_CLOCK: xcb_randr_mode_flag_t = 4096;
/// The `RandR::ModeFlag::HalveClock` enum variant.
///
/// This is a variant of [`xcb_randr_mode_flag_t`].
pub const XCB_RANDR_MODE_FLAG_HALVE_CLOCK: xcb_randr_mode_flag_t = 8192;

/// The `RandR::ModeInfo` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_mode_info_t {
    pub id: u32,
    pub width: u16,
    pub height: u16,
    pub dot_clock: u32,
    pub hsync_start: u16,
    pub hsync_end: u16,
    pub htotal: u16,
    pub hskew: u16,
    pub vsync_start: u16,
    pub vsync_end: u16,
    pub vtotal: u16,
    pub name_len: u16,
    pub mode_flags: u32,
}

impl Default for xcb_randr_mode_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `RandR::ModeInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_mode_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_randr_mode_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_randr_mode_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `RandR::GetScreenResources` request.
///
/// Pass this cookie to [`xcb_randr_get_screen_resources_reply`] to retrieve the reply.
///
/// [`xcb_randr_get_screen_resources_reply`]: XcbRandr::xcb_randr_get_screen_resources_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_screen_resources_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_get_screen_resources_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::GetScreenResources` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_get_screen_resources_request_t`].
pub const XCB_RANDR_GET_SCREEN_RESOURCES: u8 = 8i32 as u8;

/// The `RandR::GetScreenResources` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_screen_resources_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_randr_get_screen_resources_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::GetScreenResources` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `crtcs`
/// - `outputs`
/// - `modes`
/// - `names`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_screen_resources_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xcb_timestamp_t,
    pub config_timestamp: xcb_timestamp_t,
    pub num_crtcs: u16,
    pub num_outputs: u16,
    pub num_modes: u16,
    pub names_len: u16,
    pub pad1: [u8; 8],
}

impl Default for xcb_randr_get_screen_resources_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::Connection` enum.
///
/// This enum has the following variants:
///
/// - [`RandR::Connection::Connected`](XCB_RANDR_CONNECTION_CONNECTED)
/// - [`RandR::Connection::Disconnected`](XCB_RANDR_CONNECTION_DISCONNECTED)
/// - [`RandR::Connection::Unknown`](XCB_RANDR_CONNECTION_UNKNOWN)
pub type xcb_randr_connection_t = u32;
/// The `RandR::Connection::Connected` enum variant.
///
/// This is a variant of [`xcb_randr_connection_t`].
pub const XCB_RANDR_CONNECTION_CONNECTED: xcb_randr_connection_t = 0;
/// The `RandR::Connection::Disconnected` enum variant.
///
/// This is a variant of [`xcb_randr_connection_t`].
pub const XCB_RANDR_CONNECTION_DISCONNECTED: xcb_randr_connection_t = 1;
/// The `RandR::Connection::Unknown` enum variant.
///
/// This is a variant of [`xcb_randr_connection_t`].
pub const XCB_RANDR_CONNECTION_UNKNOWN: xcb_randr_connection_t = 2;

/// The cookie for the reply to a `RandR::GetOutputInfo` request.
///
/// Pass this cookie to [`xcb_randr_get_output_info_reply`] to retrieve the reply.
///
/// [`xcb_randr_get_output_info_reply`]: XcbRandr::xcb_randr_get_output_info_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_output_info_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_get_output_info_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::GetOutputInfo` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_get_output_info_request_t`].
pub const XCB_RANDR_GET_OUTPUT_INFO: u8 = 9i32 as u8;

/// The `RandR::GetOutputInfo` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_output_info_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub output: xcb_randr_output_t,
    pub config_timestamp: xcb_timestamp_t,
}

impl Default for xcb_randr_get_output_info_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::GetOutputInfo` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `crtcs`
/// - `modes`
/// - `clones`
/// - `name`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_output_info_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xcb_timestamp_t,
    pub crtc: xcb_randr_crtc_t,
    pub mm_width: u32,
    pub mm_height: u32,
    pub connection: u8,
    pub subpixel_order: u8,
    pub num_crtcs: u16,
    pub num_modes: u16,
    pub num_preferred: u16,
    pub num_clones: u16,
    pub name_len: u16,
}

impl Default for xcb_randr_get_output_info_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `RandR::ListOutputProperties` request.
///
/// Pass this cookie to [`xcb_randr_list_output_properties_reply`] to retrieve the reply.
///
/// [`xcb_randr_list_output_properties_reply`]: XcbRandr::xcb_randr_list_output_properties_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_list_output_properties_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_list_output_properties_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::ListOutputProperties` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_list_output_properties_request_t`].
pub const XCB_RANDR_LIST_OUTPUT_PROPERTIES: u8 = 10i32 as u8;

/// The `RandR::ListOutputProperties` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_list_output_properties_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub output: xcb_randr_output_t,
}

impl Default for xcb_randr_list_output_properties_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::ListOutputProperties` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `atoms`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_list_output_properties_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_atoms: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_randr_list_output_properties_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `RandR::QueryOutputProperty` request.
///
/// Pass this cookie to [`xcb_randr_query_output_property_reply`] to retrieve the reply.
///
/// [`xcb_randr_query_output_property_reply`]: XcbRandr::xcb_randr_query_output_property_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_query_output_property_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_query_output_property_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::QueryOutputProperty` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_query_output_property_request_t`].
pub const XCB_RANDR_QUERY_OUTPUT_PROPERTY: u8 = 11i32 as u8;

/// The `RandR::QueryOutputProperty` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_query_output_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub output: xcb_randr_output_t,
    pub property: xcb_atom_t,
}

impl Default for xcb_randr_query_output_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::QueryOutputProperty` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `valid_values`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_query_output_property_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pending: u8,
    pub range: u8,
    pub immutable: u8,
    pub pad1: [u8; 21],
}

impl Default for xcb_randr_query_output_property_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::ConfigureOutputProperty` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_configure_output_property_request_t`].
pub const XCB_RANDR_CONFIGURE_OUTPUT_PROPERTY: u8 = 12i32 as u8;

/// The `RandR::ConfigureOutputProperty` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `values`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_configure_output_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub output: xcb_randr_output_t,
    pub property: xcb_atom_t,
    pub pending: u8,
    pub range: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_randr_configure_output_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::ChangeOutputProperty` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_change_output_property_request_t`].
pub const XCB_RANDR_CHANGE_OUTPUT_PROPERTY: u8 = 13i32 as u8;

/// The `RandR::ChangeOutputProperty` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `data`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_change_output_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub output: xcb_randr_output_t,
    pub property: xcb_atom_t,
    pub type_: xcb_atom_t,
    pub format: u8,
    pub mode: u8,
    pub pad0: [u8; 2],
    pub num_units: u32,
}

impl Default for xcb_randr_change_output_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::DeleteOutputProperty` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_delete_output_property_request_t`].
pub const XCB_RANDR_DELETE_OUTPUT_PROPERTY: u8 = 14i32 as u8;

/// The `RandR::DeleteOutputProperty` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_delete_output_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub output: xcb_randr_output_t,
    pub property: xcb_atom_t,
}

impl Default for xcb_randr_delete_output_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `RandR::GetOutputProperty` request.
///
/// Pass this cookie to [`xcb_randr_get_output_property_reply`] to retrieve the reply.
///
/// [`xcb_randr_get_output_property_reply`]: XcbRandr::xcb_randr_get_output_property_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_output_property_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_get_output_property_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::GetOutputProperty` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_get_output_property_request_t`].
pub const XCB_RANDR_GET_OUTPUT_PROPERTY: u8 = 15i32 as u8;

/// The `RandR::GetOutputProperty` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_output_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub output: xcb_randr_output_t,
    pub property: xcb_atom_t,
    pub type_: xcb_atom_t,
    pub long_offset: u32,
    pub long_length: u32,
    pub delete: u8,
    pub pending: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_randr_get_output_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::GetOutputProperty` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `data`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_output_property_reply_t {
    pub response_type: u8,
    pub format: u8,
    pub sequence: u16,
    pub length: u32,
    pub type_: xcb_atom_t,
    pub bytes_after: u32,
    pub num_items: u32,
    pub pad0: [u8; 12],
}

impl Default for xcb_randr_get_output_property_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `RandR::CreateMode` request.
///
/// Pass this cookie to [`xcb_randr_create_mode_reply`] to retrieve the reply.
///
/// [`xcb_randr_create_mode_reply`]: XcbRandr::xcb_randr_create_mode_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_create_mode_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_create_mode_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::CreateMode` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_create_mode_request_t`].
pub const XCB_RANDR_CREATE_MODE: u8 = 16i32 as u8;

/// The `RandR::CreateMode` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `name`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_create_mode_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub mode_info: xcb_randr_mode_info_t,
}

impl Default for xcb_randr_create_mode_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::CreateMode` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_create_mode_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub mode: xcb_randr_mode_t,
    pub pad1: [u8; 20],
}

impl Default for xcb_randr_create_mode_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::DestroyMode` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_destroy_mode_request_t`].
pub const XCB_RANDR_DESTROY_MODE: u8 = 17i32 as u8;

/// The `RandR::DestroyMode` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_destroy_mode_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub mode: xcb_randr_mode_t,
}

impl Default for xcb_randr_destroy_mode_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::AddOutputMode` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_add_output_mode_request_t`].
pub const XCB_RANDR_ADD_OUTPUT_MODE: u8 = 18i32 as u8;

/// The `RandR::AddOutputMode` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_add_output_mode_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub output: xcb_randr_output_t,
    pub mode: xcb_randr_mode_t,
}

impl Default for xcb_randr_add_output_mode_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::DeleteOutputMode` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_delete_output_mode_request_t`].
pub const XCB_RANDR_DELETE_OUTPUT_MODE: u8 = 19i32 as u8;

/// The `RandR::DeleteOutputMode` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_delete_output_mode_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub output: xcb_randr_output_t,
    pub mode: xcb_randr_mode_t,
}

impl Default for xcb_randr_delete_output_mode_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `RandR::GetCrtcInfo` request.
///
/// Pass this cookie to [`xcb_randr_get_crtc_info_reply`] to retrieve the reply.
///
/// [`xcb_randr_get_crtc_info_reply`]: XcbRandr::xcb_randr_get_crtc_info_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_crtc_info_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_get_crtc_info_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::GetCrtcInfo` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_get_crtc_info_request_t`].
pub const XCB_RANDR_GET_CRTC_INFO: u8 = 20i32 as u8;

/// The `RandR::GetCrtcInfo` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_crtc_info_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub crtc: xcb_randr_crtc_t,
    pub config_timestamp: xcb_timestamp_t,
}

impl Default for xcb_randr_get_crtc_info_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::GetCrtcInfo` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `outputs`
/// - `possible`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_crtc_info_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xcb_timestamp_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub mode: xcb_randr_mode_t,
    pub rotation: u16,
    pub rotations: u16,
    pub num_outputs: u16,
    pub num_possible_outputs: u16,
}

impl Default for xcb_randr_get_crtc_info_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `RandR::SetCrtcConfig` request.
///
/// Pass this cookie to [`xcb_randr_set_crtc_config_reply`] to retrieve the reply.
///
/// [`xcb_randr_set_crtc_config_reply`]: XcbRandr::xcb_randr_set_crtc_config_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_crtc_config_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_set_crtc_config_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::SetCrtcConfig` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_set_crtc_config_request_t`].
pub const XCB_RANDR_SET_CRTC_CONFIG: u8 = 21i32 as u8;

/// The `RandR::SetCrtcConfig` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `outputs`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_crtc_config_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub crtc: xcb_randr_crtc_t,
    pub timestamp: xcb_timestamp_t,
    pub config_timestamp: xcb_timestamp_t,
    pub x: i16,
    pub y: i16,
    pub mode: xcb_randr_mode_t,
    pub rotation: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_randr_set_crtc_config_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::SetCrtcConfig` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_crtc_config_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xcb_timestamp_t,
    pub pad0: [u8; 20],
}

impl Default for xcb_randr_set_crtc_config_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `RandR::GetCrtcGammaSize` request.
///
/// Pass this cookie to [`xcb_randr_get_crtc_gamma_size_reply`] to retrieve the reply.
///
/// [`xcb_randr_get_crtc_gamma_size_reply`]: XcbRandr::xcb_randr_get_crtc_gamma_size_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_crtc_gamma_size_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_get_crtc_gamma_size_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::GetCrtcGammaSize` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_get_crtc_gamma_size_request_t`].
pub const XCB_RANDR_GET_CRTC_GAMMA_SIZE: u8 = 22i32 as u8;

/// The `RandR::GetCrtcGammaSize` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_crtc_gamma_size_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub crtc: xcb_randr_crtc_t,
}

impl Default for xcb_randr_get_crtc_gamma_size_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::GetCrtcGammaSize` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_crtc_gamma_size_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub size: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_randr_get_crtc_gamma_size_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `RandR::GetCrtcGamma` request.
///
/// Pass this cookie to [`xcb_randr_get_crtc_gamma_reply`] to retrieve the reply.
///
/// [`xcb_randr_get_crtc_gamma_reply`]: XcbRandr::xcb_randr_get_crtc_gamma_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_crtc_gamma_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_get_crtc_gamma_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::GetCrtcGamma` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_get_crtc_gamma_request_t`].
pub const XCB_RANDR_GET_CRTC_GAMMA: u8 = 23i32 as u8;

/// The `RandR::GetCrtcGamma` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_crtc_gamma_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub crtc: xcb_randr_crtc_t,
}

impl Default for xcb_randr_get_crtc_gamma_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::GetCrtcGamma` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `red`
/// - `green`
/// - `blue`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_crtc_gamma_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub size: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_randr_get_crtc_gamma_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::SetCrtcGamma` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_set_crtc_gamma_request_t`].
pub const XCB_RANDR_SET_CRTC_GAMMA: u8 = 24i32 as u8;

/// The `RandR::SetCrtcGamma` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `red`
/// - `green`
/// - `blue`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_crtc_gamma_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub crtc: xcb_randr_crtc_t,
    pub size: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_randr_set_crtc_gamma_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `RandR::GetScreenResourcesCurrent` request.
///
/// Pass this cookie to [`xcb_randr_get_screen_resources_current_reply`] to retrieve the reply.
///
/// [`xcb_randr_get_screen_resources_current_reply`]: XcbRandr::xcb_randr_get_screen_resources_current_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_screen_resources_current_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_get_screen_resources_current_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::GetScreenResourcesCurrent` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_get_screen_resources_current_request_t`].
pub const XCB_RANDR_GET_SCREEN_RESOURCES_CURRENT: u8 = 25i32 as u8;

/// The `RandR::GetScreenResourcesCurrent` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_screen_resources_current_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_randr_get_screen_resources_current_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::GetScreenResourcesCurrent` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `crtcs`
/// - `outputs`
/// - `modes`
/// - `names`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_screen_resources_current_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xcb_timestamp_t,
    pub config_timestamp: xcb_timestamp_t,
    pub num_crtcs: u16,
    pub num_outputs: u16,
    pub num_modes: u16,
    pub names_len: u16,
    pub pad1: [u8; 8],
}

impl Default for xcb_randr_get_screen_resources_current_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::Transform` enum.
///
/// This enum has the following variants:
///
/// - [`RandR::Transform::Unit`](XCB_RANDR_TRANSFORM_UNIT)
/// - [`RandR::Transform::ScaleUp`](XCB_RANDR_TRANSFORM_SCALE_UP)
/// - [`RandR::Transform::ScaleDown`](XCB_RANDR_TRANSFORM_SCALE_DOWN)
/// - [`RandR::Transform::Projective`](XCB_RANDR_TRANSFORM_PROJECTIVE)
pub type xcb_randr_transform_t = u32;
/// The `RandR::Transform::Unit` enum variant.
///
/// This is a variant of [`xcb_randr_transform_t`].
pub const XCB_RANDR_TRANSFORM_UNIT: xcb_randr_transform_t = 1;
/// The `RandR::Transform::ScaleUp` enum variant.
///
/// This is a variant of [`xcb_randr_transform_t`].
pub const XCB_RANDR_TRANSFORM_SCALE_UP: xcb_randr_transform_t = 2;
/// The `RandR::Transform::ScaleDown` enum variant.
///
/// This is a variant of [`xcb_randr_transform_t`].
pub const XCB_RANDR_TRANSFORM_SCALE_DOWN: xcb_randr_transform_t = 4;
/// The `RandR::Transform::Projective` enum variant.
///
/// This is a variant of [`xcb_randr_transform_t`].
pub const XCB_RANDR_TRANSFORM_PROJECTIVE: xcb_randr_transform_t = 8;

/// The opcode for `RandR::SetCrtcTransform` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_set_crtc_transform_request_t`].
pub const XCB_RANDR_SET_CRTC_TRANSFORM: u8 = 26i32 as u8;

/// The `RandR::SetCrtcTransform` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `filter_name`
/// - `filter_params_len`
/// - `filter_params`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_crtc_transform_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub crtc: xcb_randr_crtc_t,
    pub transform: xcb_render_transform_t,
    pub filter_len: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_randr_set_crtc_transform_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `RandR::GetCrtcTransform` request.
///
/// Pass this cookie to [`xcb_randr_get_crtc_transform_reply`] to retrieve the reply.
///
/// [`xcb_randr_get_crtc_transform_reply`]: XcbRandr::xcb_randr_get_crtc_transform_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_crtc_transform_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_get_crtc_transform_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::GetCrtcTransform` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_get_crtc_transform_request_t`].
pub const XCB_RANDR_GET_CRTC_TRANSFORM: u8 = 27i32 as u8;

/// The `RandR::GetCrtcTransform` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_crtc_transform_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub crtc: xcb_randr_crtc_t,
}

impl Default for xcb_randr_get_crtc_transform_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::GetCrtcTransform` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `pending_filter_name`
/// - `pending_params`
/// - `current_filter_name`
/// - `current_params`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_crtc_transform_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pending_transform: xcb_render_transform_t,
    pub has_transforms: u8,
    pub pad1: [u8; 3],
    pub current_transform: xcb_render_transform_t,
    pub pad2: [u8; 4],
    pub pending_len: u16,
    pub pending_nparams: u16,
    pub current_len: u16,
    pub current_nparams: u16,
}

impl Default for xcb_randr_get_crtc_transform_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `RandR::GetPanning` request.
///
/// Pass this cookie to [`xcb_randr_get_panning_reply`] to retrieve the reply.
///
/// [`xcb_randr_get_panning_reply`]: XcbRandr::xcb_randr_get_panning_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_panning_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_get_panning_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::GetPanning` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_get_panning_request_t`].
pub const XCB_RANDR_GET_PANNING: u8 = 28i32 as u8;

/// The `RandR::GetPanning` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_panning_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub crtc: xcb_randr_crtc_t,
}

impl Default for xcb_randr_get_panning_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::GetPanning` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_panning_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xcb_timestamp_t,
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16,
    pub track_left: u16,
    pub track_top: u16,
    pub track_width: u16,
    pub track_height: u16,
    pub border_left: i16,
    pub border_top: i16,
    pub border_right: i16,
    pub border_bottom: i16,
}

impl Default for xcb_randr_get_panning_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `RandR::SetPanning` request.
///
/// Pass this cookie to [`xcb_randr_set_panning_reply`] to retrieve the reply.
///
/// [`xcb_randr_set_panning_reply`]: XcbRandr::xcb_randr_set_panning_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_panning_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_set_panning_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::SetPanning` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_set_panning_request_t`].
pub const XCB_RANDR_SET_PANNING: u8 = 29i32 as u8;

/// The `RandR::SetPanning` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_panning_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub crtc: xcb_randr_crtc_t,
    pub timestamp: xcb_timestamp_t,
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16,
    pub track_left: u16,
    pub track_top: u16,
    pub track_width: u16,
    pub track_height: u16,
    pub border_left: i16,
    pub border_top: i16,
    pub border_right: i16,
    pub border_bottom: i16,
}

impl Default for xcb_randr_set_panning_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::SetPanning` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_panning_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xcb_timestamp_t,
}

impl Default for xcb_randr_set_panning_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::SetOutputPrimary` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_set_output_primary_request_t`].
pub const XCB_RANDR_SET_OUTPUT_PRIMARY: u8 = 30i32 as u8;

/// The `RandR::SetOutputPrimary` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_output_primary_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub output: xcb_randr_output_t,
}

impl Default for xcb_randr_set_output_primary_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `RandR::GetOutputPrimary` request.
///
/// Pass this cookie to [`xcb_randr_get_output_primary_reply`] to retrieve the reply.
///
/// [`xcb_randr_get_output_primary_reply`]: XcbRandr::xcb_randr_get_output_primary_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_output_primary_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_get_output_primary_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::GetOutputPrimary` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_get_output_primary_request_t`].
pub const XCB_RANDR_GET_OUTPUT_PRIMARY: u8 = 31i32 as u8;

/// The `RandR::GetOutputPrimary` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_output_primary_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_randr_get_output_primary_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::GetOutputPrimary` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_output_primary_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub output: xcb_randr_output_t,
}

impl Default for xcb_randr_get_output_primary_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `RandR::GetProviders` request.
///
/// Pass this cookie to [`xcb_randr_get_providers_reply`] to retrieve the reply.
///
/// [`xcb_randr_get_providers_reply`]: XcbRandr::xcb_randr_get_providers_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_providers_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_get_providers_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::GetProviders` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_get_providers_request_t`].
pub const XCB_RANDR_GET_PROVIDERS: u8 = 32i32 as u8;

/// The `RandR::GetProviders` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_providers_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_randr_get_providers_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::GetProviders` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `providers`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_providers_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xcb_timestamp_t,
    pub num_providers: u16,
    pub pad1: [u8; 18],
}

impl Default for xcb_randr_get_providers_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::ProviderCapability` enum.
///
/// This enum has the following variants:
///
/// - [`RandR::ProviderCapability::SourceOutput`](XCB_RANDR_PROVIDER_CAPABILITY_SOURCE_OUTPUT)
/// - [`RandR::ProviderCapability::SinkOutput`](XCB_RANDR_PROVIDER_CAPABILITY_SINK_OUTPUT)
/// - [`RandR::ProviderCapability::SourceOffload`](XCB_RANDR_PROVIDER_CAPABILITY_SOURCE_OFFLOAD)
/// - [`RandR::ProviderCapability::SinkOffload`](XCB_RANDR_PROVIDER_CAPABILITY_SINK_OFFLOAD)
pub type xcb_randr_provider_capability_t = u32;
/// The `RandR::ProviderCapability::SourceOutput` enum variant.
///
/// This is a variant of [`xcb_randr_provider_capability_t`].
pub const XCB_RANDR_PROVIDER_CAPABILITY_SOURCE_OUTPUT: xcb_randr_provider_capability_t = 1;
/// The `RandR::ProviderCapability::SinkOutput` enum variant.
///
/// This is a variant of [`xcb_randr_provider_capability_t`].
pub const XCB_RANDR_PROVIDER_CAPABILITY_SINK_OUTPUT: xcb_randr_provider_capability_t = 2;
/// The `RandR::ProviderCapability::SourceOffload` enum variant.
///
/// This is a variant of [`xcb_randr_provider_capability_t`].
pub const XCB_RANDR_PROVIDER_CAPABILITY_SOURCE_OFFLOAD: xcb_randr_provider_capability_t = 4;
/// The `RandR::ProviderCapability::SinkOffload` enum variant.
///
/// This is a variant of [`xcb_randr_provider_capability_t`].
pub const XCB_RANDR_PROVIDER_CAPABILITY_SINK_OFFLOAD: xcb_randr_provider_capability_t = 8;

/// The cookie for the reply to a `RandR::GetProviderInfo` request.
///
/// Pass this cookie to [`xcb_randr_get_provider_info_reply`] to retrieve the reply.
///
/// [`xcb_randr_get_provider_info_reply`]: XcbRandr::xcb_randr_get_provider_info_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_provider_info_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_get_provider_info_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::GetProviderInfo` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_get_provider_info_request_t`].
pub const XCB_RANDR_GET_PROVIDER_INFO: u8 = 33i32 as u8;

/// The `RandR::GetProviderInfo` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_provider_info_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub provider: xcb_randr_provider_t,
    pub config_timestamp: xcb_timestamp_t,
}

impl Default for xcb_randr_get_provider_info_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::GetProviderInfo` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `crtcs`
/// - `outputs`
/// - `associated_providers`
/// - `associated_capability`
/// - `name`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_provider_info_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xcb_timestamp_t,
    pub capabilities: u32,
    pub num_crtcs: u16,
    pub num_outputs: u16,
    pub num_associated_providers: u16,
    pub name_len: u16,
    pub pad0: [u8; 8],
}

impl Default for xcb_randr_get_provider_info_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::SetProviderOffloadSink` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_set_provider_offload_sink_request_t`].
pub const XCB_RANDR_SET_PROVIDER_OFFLOAD_SINK: u8 = 34i32 as u8;

/// The `RandR::SetProviderOffloadSink` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_provider_offload_sink_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub provider: xcb_randr_provider_t,
    pub sink_provider: xcb_randr_provider_t,
    pub config_timestamp: xcb_timestamp_t,
}

impl Default for xcb_randr_set_provider_offload_sink_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::SetProviderOutputSource` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_set_provider_output_source_request_t`].
pub const XCB_RANDR_SET_PROVIDER_OUTPUT_SOURCE: u8 = 35i32 as u8;

/// The `RandR::SetProviderOutputSource` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_provider_output_source_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub provider: xcb_randr_provider_t,
    pub source_provider: xcb_randr_provider_t,
    pub config_timestamp: xcb_timestamp_t,
}

impl Default for xcb_randr_set_provider_output_source_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `RandR::ListProviderProperties` request.
///
/// Pass this cookie to [`xcb_randr_list_provider_properties_reply`] to retrieve the reply.
///
/// [`xcb_randr_list_provider_properties_reply`]: XcbRandr::xcb_randr_list_provider_properties_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_list_provider_properties_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_list_provider_properties_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::ListProviderProperties` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_list_provider_properties_request_t`].
pub const XCB_RANDR_LIST_PROVIDER_PROPERTIES: u8 = 36i32 as u8;

/// The `RandR::ListProviderProperties` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_list_provider_properties_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub provider: xcb_randr_provider_t,
}

impl Default for xcb_randr_list_provider_properties_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::ListProviderProperties` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `atoms`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_list_provider_properties_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_atoms: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_randr_list_provider_properties_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `RandR::QueryProviderProperty` request.
///
/// Pass this cookie to [`xcb_randr_query_provider_property_reply`] to retrieve the reply.
///
/// [`xcb_randr_query_provider_property_reply`]: XcbRandr::xcb_randr_query_provider_property_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_query_provider_property_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_query_provider_property_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::QueryProviderProperty` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_query_provider_property_request_t`].
pub const XCB_RANDR_QUERY_PROVIDER_PROPERTY: u8 = 37i32 as u8;

/// The `RandR::QueryProviderProperty` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_query_provider_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub provider: xcb_randr_provider_t,
    pub property: xcb_atom_t,
}

impl Default for xcb_randr_query_provider_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::QueryProviderProperty` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `valid_values`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_query_provider_property_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pending: u8,
    pub range: u8,
    pub immutable: u8,
    pub pad1: [u8; 21],
}

impl Default for xcb_randr_query_provider_property_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::ConfigureProviderProperty` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_configure_provider_property_request_t`].
pub const XCB_RANDR_CONFIGURE_PROVIDER_PROPERTY: u8 = 38i32 as u8;

/// The `RandR::ConfigureProviderProperty` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `values`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_configure_provider_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub provider: xcb_randr_provider_t,
    pub property: xcb_atom_t,
    pub pending: u8,
    pub range: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_randr_configure_provider_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::ChangeProviderProperty` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_change_provider_property_request_t`].
pub const XCB_RANDR_CHANGE_PROVIDER_PROPERTY: u8 = 39i32 as u8;

/// The `RandR::ChangeProviderProperty` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `data`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_change_provider_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub provider: xcb_randr_provider_t,
    pub property: xcb_atom_t,
    pub type_: xcb_atom_t,
    pub format: u8,
    pub mode: u8,
    pub pad0: [u8; 2],
    pub num_items: u32,
}

impl Default for xcb_randr_change_provider_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::DeleteProviderProperty` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_delete_provider_property_request_t`].
pub const XCB_RANDR_DELETE_PROVIDER_PROPERTY: u8 = 40i32 as u8;

/// The `RandR::DeleteProviderProperty` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_delete_provider_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub provider: xcb_randr_provider_t,
    pub property: xcb_atom_t,
}

impl Default for xcb_randr_delete_provider_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `RandR::GetProviderProperty` request.
///
/// Pass this cookie to [`xcb_randr_get_provider_property_reply`] to retrieve the reply.
///
/// [`xcb_randr_get_provider_property_reply`]: XcbRandr::xcb_randr_get_provider_property_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_provider_property_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_get_provider_property_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::GetProviderProperty` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_get_provider_property_request_t`].
pub const XCB_RANDR_GET_PROVIDER_PROPERTY: u8 = 41i32 as u8;

/// The `RandR::GetProviderProperty` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_provider_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub provider: xcb_randr_provider_t,
    pub property: xcb_atom_t,
    pub type_: xcb_atom_t,
    pub long_offset: u32,
    pub long_length: u32,
    pub delete: u8,
    pub pending: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_randr_get_provider_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::GetProviderProperty` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `data`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_provider_property_reply_t {
    pub response_type: u8,
    pub format: u8,
    pub sequence: u16,
    pub length: u32,
    pub type_: xcb_atom_t,
    pub bytes_after: u32,
    pub num_items: u32,
    pub pad0: [u8; 12],
}

impl Default for xcb_randr_get_provider_property_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::ScreenChangeNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_randr_screen_change_notify_event_t`].
pub const XCB_RANDR_SCREEN_CHANGE_NOTIFY: u8 = 0i32 as u8;

/// The `RandR::ScreenChangeNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_screen_change_notify_event_t {
    pub response_type: u8,
    pub rotation: u8,
    pub sequence: u16,
    pub timestamp: xcb_timestamp_t,
    pub config_timestamp: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub request_window: xcb_window_t,
    pub size_i_d: u16,
    pub subpixel_order: u16,
    pub width: u16,
    pub height: u16,
    pub mwidth: u16,
    pub mheight: u16,
}

impl Default for xcb_randr_screen_change_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::Notify` enum.
///
/// This enum has the following variants:
///
/// - [`RandR::Notify::CrtcChange`](XCB_RANDR_NOTIFY_CRTC_CHANGE)
/// - [`RandR::Notify::OutputChange`](XCB_RANDR_NOTIFY_OUTPUT_CHANGE)
/// - [`RandR::Notify::OutputProperty`](XCB_RANDR_NOTIFY_OUTPUT_PROPERTY)
/// - [`RandR::Notify::ProviderChange`](XCB_RANDR_NOTIFY_PROVIDER_CHANGE)
/// - [`RandR::Notify::ProviderProperty`](XCB_RANDR_NOTIFY_PROVIDER_PROPERTY)
/// - [`RandR::Notify::ResourceChange`](XCB_RANDR_NOTIFY_RESOURCE_CHANGE)
/// - [`RandR::Notify::Lease`](XCB_RANDR_NOTIFY_LEASE)
pub type xcb_randr_notify_t = u32;
/// The `RandR::Notify::CrtcChange` enum variant.
///
/// This is a variant of [`xcb_randr_notify_t`].
pub const XCB_RANDR_NOTIFY_CRTC_CHANGE: xcb_randr_notify_t = 0;
/// The `RandR::Notify::OutputChange` enum variant.
///
/// This is a variant of [`xcb_randr_notify_t`].
pub const XCB_RANDR_NOTIFY_OUTPUT_CHANGE: xcb_randr_notify_t = 1;
/// The `RandR::Notify::OutputProperty` enum variant.
///
/// This is a variant of [`xcb_randr_notify_t`].
pub const XCB_RANDR_NOTIFY_OUTPUT_PROPERTY: xcb_randr_notify_t = 2;
/// The `RandR::Notify::ProviderChange` enum variant.
///
/// This is a variant of [`xcb_randr_notify_t`].
pub const XCB_RANDR_NOTIFY_PROVIDER_CHANGE: xcb_randr_notify_t = 3;
/// The `RandR::Notify::ProviderProperty` enum variant.
///
/// This is a variant of [`xcb_randr_notify_t`].
pub const XCB_RANDR_NOTIFY_PROVIDER_PROPERTY: xcb_randr_notify_t = 4;
/// The `RandR::Notify::ResourceChange` enum variant.
///
/// This is a variant of [`xcb_randr_notify_t`].
pub const XCB_RANDR_NOTIFY_RESOURCE_CHANGE: xcb_randr_notify_t = 5;
/// The `RandR::Notify::Lease` enum variant.
///
/// This is a variant of [`xcb_randr_notify_t`].
pub const XCB_RANDR_NOTIFY_LEASE: xcb_randr_notify_t = 6;

/// The `RandR::CrtcChange` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_crtc_change_t {
    pub timestamp: xcb_timestamp_t,
    pub window: xcb_window_t,
    pub crtc: xcb_randr_crtc_t,
    pub mode: xcb_randr_mode_t,
    pub rotation: u16,
    pub pad0: [u8; 2],
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

impl Default for xcb_randr_crtc_change_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `RandR::CrtcChange` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_crtc_change_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_randr_crtc_change_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_randr_crtc_change_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::OutputChange` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_output_change_t {
    pub timestamp: xcb_timestamp_t,
    pub config_timestamp: xcb_timestamp_t,
    pub window: xcb_window_t,
    pub output: xcb_randr_output_t,
    pub crtc: xcb_randr_crtc_t,
    pub mode: xcb_randr_mode_t,
    pub rotation: u16,
    pub connection: u8,
    pub subpixel_order: u8,
}

impl Default for xcb_randr_output_change_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `RandR::OutputChange` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_output_change_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_randr_output_change_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_randr_output_change_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::OutputProperty` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_output_property_t {
    pub window: xcb_window_t,
    pub output: xcb_randr_output_t,
    pub atom: xcb_atom_t,
    pub timestamp: xcb_timestamp_t,
    pub status: u8,
    pub pad0: [u8; 11],
}

impl Default for xcb_randr_output_property_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `RandR::OutputProperty` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_output_property_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_randr_output_property_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_randr_output_property_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::ProviderChange` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_provider_change_t {
    pub timestamp: xcb_timestamp_t,
    pub window: xcb_window_t,
    pub provider: xcb_randr_provider_t,
    pub pad0: [u8; 16],
}

impl Default for xcb_randr_provider_change_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `RandR::ProviderChange` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_provider_change_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_randr_provider_change_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_randr_provider_change_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::ProviderProperty` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_provider_property_t {
    pub window: xcb_window_t,
    pub provider: xcb_randr_provider_t,
    pub atom: xcb_atom_t,
    pub timestamp: xcb_timestamp_t,
    pub state: u8,
    pub pad0: [u8; 11],
}

impl Default for xcb_randr_provider_property_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `RandR::ProviderProperty` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_provider_property_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_randr_provider_property_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_randr_provider_property_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::ResourceChange` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_resource_change_t {
    pub timestamp: xcb_timestamp_t,
    pub window: xcb_window_t,
    pub pad0: [u8; 20],
}

impl Default for xcb_randr_resource_change_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `RandR::ResourceChange` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_resource_change_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_randr_resource_change_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_randr_resource_change_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::MonitorInfo` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `outputs`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_monitor_info_t {
    pub name: xcb_atom_t,
    pub primary: u8,
    pub automatic: u8,
    pub n_output: u16,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub width_in_millimeters: u32,
    pub height_in_millimeters: u32,
}

impl Default for xcb_randr_monitor_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `RandR::MonitorInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_monitor_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_randr_monitor_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_randr_monitor_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `RandR::GetMonitors` request.
///
/// Pass this cookie to [`xcb_randr_get_monitors_reply`] to retrieve the reply.
///
/// [`xcb_randr_get_monitors_reply`]: XcbRandr::xcb_randr_get_monitors_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_monitors_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_get_monitors_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::GetMonitors` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_get_monitors_request_t`].
pub const XCB_RANDR_GET_MONITORS: u8 = 42i32 as u8;

/// The `RandR::GetMonitors` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_monitors_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub get_active: u8,
}

impl Default for xcb_randr_get_monitors_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::GetMonitors` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `monitors`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_monitors_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xcb_timestamp_t,
    pub n_monitors: u32,
    pub n_outputs: u32,
    pub pad1: [u8; 12],
}

impl Default for xcb_randr_get_monitors_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::SetMonitor` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_set_monitor_request_t`].
pub const XCB_RANDR_SET_MONITOR: u8 = 43i32 as u8;

/// The `RandR::SetMonitor` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `monitorinfo`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_monitor_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_randr_set_monitor_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::DeleteMonitor` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_delete_monitor_request_t`].
pub const XCB_RANDR_DELETE_MONITOR: u8 = 44i32 as u8;

/// The `RandR::DeleteMonitor` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_delete_monitor_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub name: xcb_atom_t,
}

impl Default for xcb_randr_delete_monitor_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `RandR::CreateLease` request.
///
/// Pass this cookie to [`xcb_randr_create_lease_reply`] to retrieve the reply.
///
/// [`xcb_randr_create_lease_reply`]: XcbRandr::xcb_randr_create_lease_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_create_lease_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_randr_create_lease_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::CreateLease` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_create_lease_request_t`].
pub const XCB_RANDR_CREATE_LEASE: u8 = 45i32 as u8;

/// The `RandR::CreateLease` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `crtcs`
/// - `outputs`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_create_lease_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub lid: xcb_randr_lease_t,
    pub num_crtcs: u16,
    pub num_outputs: u16,
}

impl Default for xcb_randr_create_lease_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::CreateLease` reply.
///
/// This reply contains file descriptors that can be accessed with [`xcb_randr_create_lease_reply_fds`].
///
/// [`xcb_randr_create_lease_reply_fds`]: XcbRandr::xcb_randr_create_lease_reply_fds
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_create_lease_reply_t {
    pub response_type: u8,
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad0: [u8; 24],
}

impl Default for xcb_randr_create_lease_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::FreeLease` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRandr::xcb_randr_id()`], then the type of the request is
/// [`xcb_randr_free_lease_request_t`].
pub const XCB_RANDR_FREE_LEASE: u8 = 46i32 as u8;

/// The `RandR::FreeLease` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_free_lease_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub lid: xcb_randr_lease_t,
    pub terminate: u8,
}

impl Default for xcb_randr_free_lease_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::LeaseNotify` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_lease_notify_t {
    pub timestamp: xcb_timestamp_t,
    pub window: xcb_window_t,
    pub lease: xcb_randr_lease_t,
    pub created: u8,
    pub pad0: [u8; 15],
}

impl Default for xcb_randr_lease_notify_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `RandR::LeaseNotify` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_lease_notify_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_randr_lease_notify_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_randr_lease_notify_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RandR::NotifyData` union.
#[derive(Copy, Clone)]
#[repr(C)]
pub union xcb_randr_notify_data_t {
    pub cc: xcb_randr_crtc_change_t,
    pub oc: xcb_randr_output_change_t,
    pub op: xcb_randr_output_property_t,
    pub pc: xcb_randr_provider_change_t,
    pub pp: xcb_randr_provider_property_t,
    pub rc: xcb_randr_resource_change_t,
    pub lc: xcb_randr_lease_notify_t,
}

impl Default for xcb_randr_notify_data_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `RandR::NotifyData` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_notify_data_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_randr_notify_data_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_randr_notify_data_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RandR::Notify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_randr_notify_event_t`].
pub const XCB_RANDR_NOTIFY: u8 = 1i32 as u8;

/// The `RandR::Notify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_notify_event_t {
    pub response_type: u8,
    pub sub_code: u8,
    pub sequence: u16,
    pub u: xcb_randr_notify_data_t,
}

impl Default for xcb_randr_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_randr")]
pub(crate) struct XcbRandrRandr {
    xcb_randr_id: LazySymbol<*mut xcb_extension_t>,
    xcb_randr_mode_next: LazySymbol<unsafe fn(i: *mut xcb_randr_mode_iterator_t)>,
    xcb_randr_mode_end:
        LazySymbol<unsafe fn(i: xcb_randr_mode_iterator_t) -> xcb_generic_iterator_t>,
    xcb_randr_crtc_next: LazySymbol<unsafe fn(i: *mut xcb_randr_crtc_iterator_t)>,
    xcb_randr_crtc_end:
        LazySymbol<unsafe fn(i: xcb_randr_crtc_iterator_t) -> xcb_generic_iterator_t>,
    xcb_randr_output_next: LazySymbol<unsafe fn(i: *mut xcb_randr_output_iterator_t)>,
    xcb_randr_output_end:
        LazySymbol<unsafe fn(i: xcb_randr_output_iterator_t) -> xcb_generic_iterator_t>,
    xcb_randr_provider_next: LazySymbol<unsafe fn(i: *mut xcb_randr_provider_iterator_t)>,
    xcb_randr_provider_end:
        LazySymbol<unsafe fn(i: xcb_randr_provider_iterator_t) -> xcb_generic_iterator_t>,
    xcb_randr_lease_next: LazySymbol<unsafe fn(i: *mut xcb_randr_lease_iterator_t)>,
    xcb_randr_lease_end:
        LazySymbol<unsafe fn(i: xcb_randr_lease_iterator_t) -> xcb_generic_iterator_t>,
    xcb_randr_screen_size_next: LazySymbol<unsafe fn(i: *mut xcb_randr_screen_size_iterator_t)>,
    xcb_randr_screen_size_end:
        LazySymbol<unsafe fn(i: xcb_randr_screen_size_iterator_t) -> xcb_generic_iterator_t>,
    xcb_randr_refresh_rates_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_randr_refresh_rates_rates:
        LazySymbol<unsafe fn(r: *const xcb_randr_refresh_rates_t) -> *mut u16>,
    xcb_randr_refresh_rates_rates_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_refresh_rates_t) -> c_int>,
    xcb_randr_refresh_rates_rates_end:
        LazySymbol<unsafe fn(r: *const xcb_randr_refresh_rates_t) -> xcb_generic_iterator_t>,
    xcb_randr_refresh_rates_next: LazySymbol<unsafe fn(i: *mut xcb_randr_refresh_rates_iterator_t)>,
    xcb_randr_refresh_rates_end:
        LazySymbol<unsafe fn(i: xcb_randr_refresh_rates_iterator_t) -> xcb_generic_iterator_t>,
    xcb_randr_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u32,
            minor_version: u32,
        ) -> xcb_randr_query_version_cookie_t,
    >,
    xcb_randr_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u32,
            minor_version: u32,
        ) -> xcb_randr_query_version_cookie_t,
    >,
    xcb_randr_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_query_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_query_version_reply_t,
    >,
    xcb_randr_set_screen_config: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            timestamp: xcb_timestamp_t,
            config_timestamp: xcb_timestamp_t,
            size_i_d: u16,
            rotation: u16,
            rate: u16,
        ) -> xcb_randr_set_screen_config_cookie_t,
    >,
    xcb_randr_set_screen_config_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            timestamp: xcb_timestamp_t,
            config_timestamp: xcb_timestamp_t,
            size_i_d: u16,
            rotation: u16,
            rate: u16,
        ) -> xcb_randr_set_screen_config_cookie_t,
    >,
    xcb_randr_set_screen_config_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_set_screen_config_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_set_screen_config_reply_t,
    >,
    xcb_randr_select_input_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, enable: u16) -> xcb_void_cookie_t,
    >,
    xcb_randr_select_input: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, enable: u16) -> xcb_void_cookie_t,
    >,
    xcb_randr_get_screen_info_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_randr_get_screen_info: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_randr_get_screen_info_cookie_t,
    >,
    xcb_randr_get_screen_info_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_randr_get_screen_info_cookie_t,
    >,
    xcb_randr_get_screen_info_sizes: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_screen_info_reply_t) -> *mut xcb_randr_screen_size_t,
    >,
    xcb_randr_get_screen_info_sizes_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_screen_info_reply_t) -> c_int>,
    xcb_randr_get_screen_info_sizes_iterator: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_screen_info_reply_t) -> xcb_randr_screen_size_iterator_t,
    >,
    xcb_randr_get_screen_info_rates_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_screen_info_reply_t) -> c_int>,
    xcb_randr_get_screen_info_rates_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_randr_get_screen_info_reply_t,
        ) -> xcb_randr_refresh_rates_iterator_t,
    >,
    xcb_randr_get_screen_info_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_get_screen_info_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_get_screen_info_reply_t,
    >,
    xcb_randr_get_screen_size_range: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_randr_get_screen_size_range_cookie_t,
    >,
    xcb_randr_get_screen_size_range_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_randr_get_screen_size_range_cookie_t,
    >,
    xcb_randr_get_screen_size_range_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_get_screen_size_range_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_get_screen_size_range_reply_t,
    >,
    xcb_randr_set_screen_size_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            width: u16,
            height: u16,
            mm_width: u32,
            mm_height: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_set_screen_size: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            width: u16,
            height: u16,
            mm_width: u32,
            mm_height: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_mode_info_next: LazySymbol<unsafe fn(i: *mut xcb_randr_mode_info_iterator_t)>,
    xcb_randr_mode_info_end:
        LazySymbol<unsafe fn(i: xcb_randr_mode_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_randr_get_screen_resources_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_randr_get_screen_resources: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_randr_get_screen_resources_cookie_t,
    >,
    xcb_randr_get_screen_resources_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_randr_get_screen_resources_cookie_t,
    >,
    xcb_randr_get_screen_resources_crtcs: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_screen_resources_reply_t) -> *mut xcb_randr_crtc_t,
    >,
    xcb_randr_get_screen_resources_crtcs_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_screen_resources_reply_t) -> c_int>,
    xcb_randr_get_screen_resources_crtcs_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_screen_resources_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_get_screen_resources_outputs: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_screen_resources_reply_t) -> *mut xcb_randr_output_t,
    >,
    xcb_randr_get_screen_resources_outputs_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_screen_resources_reply_t) -> c_int>,
    xcb_randr_get_screen_resources_outputs_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_screen_resources_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_get_screen_resources_modes: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_screen_resources_reply_t) -> *mut xcb_randr_mode_info_t,
    >,
    xcb_randr_get_screen_resources_modes_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_screen_resources_reply_t) -> c_int>,
    xcb_randr_get_screen_resources_modes_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_randr_get_screen_resources_reply_t,
        ) -> xcb_randr_mode_info_iterator_t,
    >,
    xcb_randr_get_screen_resources_names:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_screen_resources_reply_t) -> *mut u8>,
    xcb_randr_get_screen_resources_names_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_screen_resources_reply_t) -> c_int>,
    xcb_randr_get_screen_resources_names_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_screen_resources_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_get_screen_resources_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_get_screen_resources_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_get_screen_resources_reply_t,
    >,
    xcb_randr_get_output_info_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_randr_get_output_info: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            output: xcb_randr_output_t,
            config_timestamp: xcb_timestamp_t,
        ) -> xcb_randr_get_output_info_cookie_t,
    >,
    xcb_randr_get_output_info_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            output: xcb_randr_output_t,
            config_timestamp: xcb_timestamp_t,
        ) -> xcb_randr_get_output_info_cookie_t,
    >,
    xcb_randr_get_output_info_crtcs:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_output_info_reply_t) -> *mut xcb_randr_crtc_t>,
    xcb_randr_get_output_info_crtcs_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_output_info_reply_t) -> c_int>,
    xcb_randr_get_output_info_crtcs_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_output_info_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_get_output_info_modes:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_output_info_reply_t) -> *mut xcb_randr_mode_t>,
    xcb_randr_get_output_info_modes_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_output_info_reply_t) -> c_int>,
    xcb_randr_get_output_info_modes_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_output_info_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_get_output_info_clones: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_output_info_reply_t) -> *mut xcb_randr_output_t,
    >,
    xcb_randr_get_output_info_clones_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_output_info_reply_t) -> c_int>,
    xcb_randr_get_output_info_clones_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_output_info_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_get_output_info_name:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_output_info_reply_t) -> *mut u8>,
    xcb_randr_get_output_info_name_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_output_info_reply_t) -> c_int>,
    xcb_randr_get_output_info_name_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_output_info_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_get_output_info_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_get_output_info_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_get_output_info_reply_t,
    >,
    xcb_randr_list_output_properties_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_randr_list_output_properties: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            output: xcb_randr_output_t,
        ) -> xcb_randr_list_output_properties_cookie_t,
    >,
    xcb_randr_list_output_properties_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            output: xcb_randr_output_t,
        ) -> xcb_randr_list_output_properties_cookie_t,
    >,
    xcb_randr_list_output_properties_atoms: LazySymbol<
        unsafe fn(r: *const xcb_randr_list_output_properties_reply_t) -> *mut xcb_atom_t,
    >,
    xcb_randr_list_output_properties_atoms_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_list_output_properties_reply_t) -> c_int>,
    xcb_randr_list_output_properties_atoms_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_list_output_properties_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_list_output_properties_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_list_output_properties_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_list_output_properties_reply_t,
    >,
    xcb_randr_query_output_property_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_randr_query_output_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            output: xcb_randr_output_t,
            property: xcb_atom_t,
        ) -> xcb_randr_query_output_property_cookie_t,
    >,
    xcb_randr_query_output_property_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            output: xcb_randr_output_t,
            property: xcb_atom_t,
        ) -> xcb_randr_query_output_property_cookie_t,
    >,
    xcb_randr_query_output_property_valid_values:
        LazySymbol<unsafe fn(r: *const xcb_randr_query_output_property_reply_t) -> *mut i32>,
    xcb_randr_query_output_property_valid_values_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_query_output_property_reply_t) -> c_int>,
    xcb_randr_query_output_property_valid_values_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_query_output_property_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_query_output_property_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_query_output_property_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_query_output_property_reply_t,
    >,
    xcb_randr_configure_output_property_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, values_len: u32) -> c_int>,
    xcb_randr_configure_output_property_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            output: xcb_randr_output_t,
            property: xcb_atom_t,
            pending: u8,
            range: u8,
            values_len: u32,
            values: *const i32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_configure_output_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            output: xcb_randr_output_t,
            property: xcb_atom_t,
            pending: u8,
            range: u8,
            values_len: u32,
            values: *const i32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_configure_output_property_values:
        LazySymbol<unsafe fn(r: *const xcb_randr_configure_output_property_request_t) -> *mut i32>,
    xcb_randr_configure_output_property_values_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_configure_output_property_request_t) -> c_int>,
    xcb_randr_configure_output_property_values_end: LazySymbol<
        unsafe fn(
            r: *const xcb_randr_configure_output_property_request_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_randr_change_output_property_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_randr_change_output_property_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            output: xcb_randr_output_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            format: u8,
            mode: u8,
            num_units: u32,
            data: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_change_output_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            output: xcb_randr_output_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            format: u8,
            mode: u8,
            num_units: u32,
            data: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_change_output_property_data:
        LazySymbol<unsafe fn(r: *const xcb_randr_change_output_property_request_t) -> *mut c_void>,
    xcb_randr_change_output_property_data_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_change_output_property_request_t) -> c_int>,
    xcb_randr_change_output_property_data_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_change_output_property_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_delete_output_property_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            output: xcb_randr_output_t,
            property: xcb_atom_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_delete_output_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            output: xcb_randr_output_t,
            property: xcb_atom_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_get_output_property_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_randr_get_output_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            output: xcb_randr_output_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            long_offset: u32,
            long_length: u32,
            delete: u8,
            pending: u8,
        ) -> xcb_randr_get_output_property_cookie_t,
    >,
    xcb_randr_get_output_property_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            output: xcb_randr_output_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            long_offset: u32,
            long_length: u32,
            delete: u8,
            pending: u8,
        ) -> xcb_randr_get_output_property_cookie_t,
    >,
    xcb_randr_get_output_property_data:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_output_property_reply_t) -> *mut u8>,
    xcb_randr_get_output_property_data_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_output_property_reply_t) -> c_int>,
    xcb_randr_get_output_property_data_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_output_property_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_get_output_property_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_get_output_property_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_get_output_property_reply_t,
    >,
    xcb_randr_create_mode_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, name_len: u32) -> c_int>,
    xcb_randr_create_mode: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            mode_info: xcb_randr_mode_info_t,
            name_len: u32,
            name: *const c_char,
        ) -> xcb_randr_create_mode_cookie_t,
    >,
    xcb_randr_create_mode_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            mode_info: xcb_randr_mode_info_t,
            name_len: u32,
            name: *const c_char,
        ) -> xcb_randr_create_mode_cookie_t,
    >,
    xcb_randr_create_mode_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_create_mode_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_create_mode_reply_t,
    >,
    xcb_randr_destroy_mode_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, mode: xcb_randr_mode_t) -> xcb_void_cookie_t,
    >,
    xcb_randr_destroy_mode: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, mode: xcb_randr_mode_t) -> xcb_void_cookie_t,
    >,
    xcb_randr_add_output_mode_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            output: xcb_randr_output_t,
            mode: xcb_randr_mode_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_add_output_mode: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            output: xcb_randr_output_t,
            mode: xcb_randr_mode_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_delete_output_mode_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            output: xcb_randr_output_t,
            mode: xcb_randr_mode_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_delete_output_mode: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            output: xcb_randr_output_t,
            mode: xcb_randr_mode_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_get_crtc_info_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_randr_get_crtc_info: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            crtc: xcb_randr_crtc_t,
            config_timestamp: xcb_timestamp_t,
        ) -> xcb_randr_get_crtc_info_cookie_t,
    >,
    xcb_randr_get_crtc_info_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            crtc: xcb_randr_crtc_t,
            config_timestamp: xcb_timestamp_t,
        ) -> xcb_randr_get_crtc_info_cookie_t,
    >,
    xcb_randr_get_crtc_info_outputs:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_crtc_info_reply_t) -> *mut xcb_randr_output_t>,
    xcb_randr_get_crtc_info_outputs_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_crtc_info_reply_t) -> c_int>,
    xcb_randr_get_crtc_info_outputs_end:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_crtc_info_reply_t) -> xcb_generic_iterator_t>,
    xcb_randr_get_crtc_info_possible:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_crtc_info_reply_t) -> *mut xcb_randr_output_t>,
    xcb_randr_get_crtc_info_possible_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_crtc_info_reply_t) -> c_int>,
    xcb_randr_get_crtc_info_possible_end:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_crtc_info_reply_t) -> xcb_generic_iterator_t>,
    xcb_randr_get_crtc_info_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_get_crtc_info_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_get_crtc_info_reply_t,
    >,
    xcb_randr_set_crtc_config_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, outputs_len: u32) -> c_int>,
    xcb_randr_set_crtc_config: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            crtc: xcb_randr_crtc_t,
            timestamp: xcb_timestamp_t,
            config_timestamp: xcb_timestamp_t,
            x: i16,
            y: i16,
            mode: xcb_randr_mode_t,
            rotation: u16,
            outputs_len: u32,
            outputs: *const xcb_randr_output_t,
        ) -> xcb_randr_set_crtc_config_cookie_t,
    >,
    xcb_randr_set_crtc_config_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            crtc: xcb_randr_crtc_t,
            timestamp: xcb_timestamp_t,
            config_timestamp: xcb_timestamp_t,
            x: i16,
            y: i16,
            mode: xcb_randr_mode_t,
            rotation: u16,
            outputs_len: u32,
            outputs: *const xcb_randr_output_t,
        ) -> xcb_randr_set_crtc_config_cookie_t,
    >,
    xcb_randr_set_crtc_config_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_set_crtc_config_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_set_crtc_config_reply_t,
    >,
    xcb_randr_get_crtc_gamma_size: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            crtc: xcb_randr_crtc_t,
        ) -> xcb_randr_get_crtc_gamma_size_cookie_t,
    >,
    xcb_randr_get_crtc_gamma_size_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            crtc: xcb_randr_crtc_t,
        ) -> xcb_randr_get_crtc_gamma_size_cookie_t,
    >,
    xcb_randr_get_crtc_gamma_size_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_get_crtc_gamma_size_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_get_crtc_gamma_size_reply_t,
    >,
    xcb_randr_get_crtc_gamma_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_randr_get_crtc_gamma: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            crtc: xcb_randr_crtc_t,
        ) -> xcb_randr_get_crtc_gamma_cookie_t,
    >,
    xcb_randr_get_crtc_gamma_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            crtc: xcb_randr_crtc_t,
        ) -> xcb_randr_get_crtc_gamma_cookie_t,
    >,
    xcb_randr_get_crtc_gamma_red:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_crtc_gamma_reply_t) -> *mut u16>,
    xcb_randr_get_crtc_gamma_red_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_crtc_gamma_reply_t) -> c_int>,
    xcb_randr_get_crtc_gamma_red_end:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_crtc_gamma_reply_t) -> xcb_generic_iterator_t>,
    xcb_randr_get_crtc_gamma_green:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_crtc_gamma_reply_t) -> *mut u16>,
    xcb_randr_get_crtc_gamma_green_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_crtc_gamma_reply_t) -> c_int>,
    xcb_randr_get_crtc_gamma_green_end:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_crtc_gamma_reply_t) -> xcb_generic_iterator_t>,
    xcb_randr_get_crtc_gamma_blue:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_crtc_gamma_reply_t) -> *mut u16>,
    xcb_randr_get_crtc_gamma_blue_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_crtc_gamma_reply_t) -> c_int>,
    xcb_randr_get_crtc_gamma_blue_end:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_crtc_gamma_reply_t) -> xcb_generic_iterator_t>,
    xcb_randr_get_crtc_gamma_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_get_crtc_gamma_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_get_crtc_gamma_reply_t,
    >,
    xcb_randr_set_crtc_gamma_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_randr_set_crtc_gamma_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            crtc: xcb_randr_crtc_t,
            size: u16,
            red: *const u16,
            green: *const u16,
            blue: *const u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_set_crtc_gamma: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            crtc: xcb_randr_crtc_t,
            size: u16,
            red: *const u16,
            green: *const u16,
            blue: *const u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_set_crtc_gamma_red:
        LazySymbol<unsafe fn(r: *const xcb_randr_set_crtc_gamma_request_t) -> *mut u16>,
    xcb_randr_set_crtc_gamma_red_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_set_crtc_gamma_request_t) -> c_int>,
    xcb_randr_set_crtc_gamma_red_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_set_crtc_gamma_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_set_crtc_gamma_green:
        LazySymbol<unsafe fn(r: *const xcb_randr_set_crtc_gamma_request_t) -> *mut u16>,
    xcb_randr_set_crtc_gamma_green_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_set_crtc_gamma_request_t) -> c_int>,
    xcb_randr_set_crtc_gamma_green_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_set_crtc_gamma_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_set_crtc_gamma_blue:
        LazySymbol<unsafe fn(r: *const xcb_randr_set_crtc_gamma_request_t) -> *mut u16>,
    xcb_randr_set_crtc_gamma_blue_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_set_crtc_gamma_request_t) -> c_int>,
    xcb_randr_set_crtc_gamma_blue_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_set_crtc_gamma_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_get_screen_resources_current_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_randr_get_screen_resources_current: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_randr_get_screen_resources_current_cookie_t,
    >,
    xcb_randr_get_screen_resources_current_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_randr_get_screen_resources_current_cookie_t,
    >,
    xcb_randr_get_screen_resources_current_crtcs: LazySymbol<
        unsafe fn(
            r: *const xcb_randr_get_screen_resources_current_reply_t,
        ) -> *mut xcb_randr_crtc_t,
    >,
    xcb_randr_get_screen_resources_current_crtcs_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_screen_resources_current_reply_t) -> c_int>,
    xcb_randr_get_screen_resources_current_crtcs_end: LazySymbol<
        unsafe fn(
            r: *const xcb_randr_get_screen_resources_current_reply_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_randr_get_screen_resources_current_outputs: LazySymbol<
        unsafe fn(
            r: *const xcb_randr_get_screen_resources_current_reply_t,
        ) -> *mut xcb_randr_output_t,
    >,
    xcb_randr_get_screen_resources_current_outputs_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_screen_resources_current_reply_t) -> c_int>,
    xcb_randr_get_screen_resources_current_outputs_end: LazySymbol<
        unsafe fn(
            r: *const xcb_randr_get_screen_resources_current_reply_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_randr_get_screen_resources_current_modes: LazySymbol<
        unsafe fn(
            r: *const xcb_randr_get_screen_resources_current_reply_t,
        ) -> *mut xcb_randr_mode_info_t,
    >,
    xcb_randr_get_screen_resources_current_modes_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_screen_resources_current_reply_t) -> c_int>,
    xcb_randr_get_screen_resources_current_modes_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_randr_get_screen_resources_current_reply_t,
        ) -> xcb_randr_mode_info_iterator_t,
    >,
    xcb_randr_get_screen_resources_current_names:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_screen_resources_current_reply_t) -> *mut u8>,
    xcb_randr_get_screen_resources_current_names_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_screen_resources_current_reply_t) -> c_int>,
    xcb_randr_get_screen_resources_current_names_end: LazySymbol<
        unsafe fn(
            r: *const xcb_randr_get_screen_resources_current_reply_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_randr_get_screen_resources_current_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_get_screen_resources_current_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_get_screen_resources_current_reply_t,
    >,
    xcb_randr_set_crtc_transform_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, filter_params_len: u32) -> c_int>,
    xcb_randr_set_crtc_transform_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            crtc: xcb_randr_crtc_t,
            transform: xcb_render_transform_t,
            filter_len: u16,
            filter_name: *const c_char,
            filter_params_len: u32,
            filter_params: *const xcb_render_fixed_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_set_crtc_transform: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            crtc: xcb_randr_crtc_t,
            transform: xcb_render_transform_t,
            filter_len: u16,
            filter_name: *const c_char,
            filter_params_len: u32,
            filter_params: *const xcb_render_fixed_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_set_crtc_transform_filter_name:
        LazySymbol<unsafe fn(r: *const xcb_randr_set_crtc_transform_request_t) -> *mut c_char>,
    xcb_randr_set_crtc_transform_filter_name_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_set_crtc_transform_request_t) -> c_int>,
    xcb_randr_set_crtc_transform_filter_name_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_set_crtc_transform_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_set_crtc_transform_filter_params: LazySymbol<
        unsafe fn(r: *const xcb_randr_set_crtc_transform_request_t) -> *mut xcb_render_fixed_t,
    >,
    xcb_randr_set_crtc_transform_filter_params_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_set_crtc_transform_request_t) -> c_int>,
    xcb_randr_set_crtc_transform_filter_params_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_set_crtc_transform_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_get_crtc_transform_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_randr_get_crtc_transform: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            crtc: xcb_randr_crtc_t,
        ) -> xcb_randr_get_crtc_transform_cookie_t,
    >,
    xcb_randr_get_crtc_transform_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            crtc: xcb_randr_crtc_t,
        ) -> xcb_randr_get_crtc_transform_cookie_t,
    >,
    xcb_randr_get_crtc_transform_pending_filter_name:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_crtc_transform_reply_t) -> *mut c_char>,
    xcb_randr_get_crtc_transform_pending_filter_name_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_crtc_transform_reply_t) -> c_int>,
    xcb_randr_get_crtc_transform_pending_filter_name_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_crtc_transform_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_get_crtc_transform_pending_params: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_crtc_transform_reply_t) -> *mut xcb_render_fixed_t,
    >,
    xcb_randr_get_crtc_transform_pending_params_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_crtc_transform_reply_t) -> c_int>,
    xcb_randr_get_crtc_transform_pending_params_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_crtc_transform_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_get_crtc_transform_current_filter_name:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_crtc_transform_reply_t) -> *mut c_char>,
    xcb_randr_get_crtc_transform_current_filter_name_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_crtc_transform_reply_t) -> c_int>,
    xcb_randr_get_crtc_transform_current_filter_name_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_crtc_transform_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_get_crtc_transform_current_params: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_crtc_transform_reply_t) -> *mut xcb_render_fixed_t,
    >,
    xcb_randr_get_crtc_transform_current_params_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_crtc_transform_reply_t) -> c_int>,
    xcb_randr_get_crtc_transform_current_params_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_crtc_transform_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_get_crtc_transform_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_get_crtc_transform_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_get_crtc_transform_reply_t,
    >,
    xcb_randr_get_panning: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            crtc: xcb_randr_crtc_t,
        ) -> xcb_randr_get_panning_cookie_t,
    >,
    xcb_randr_get_panning_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            crtc: xcb_randr_crtc_t,
        ) -> xcb_randr_get_panning_cookie_t,
    >,
    xcb_randr_get_panning_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_get_panning_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_get_panning_reply_t,
    >,
    xcb_randr_set_panning: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            crtc: xcb_randr_crtc_t,
            timestamp: xcb_timestamp_t,
            left: u16,
            top: u16,
            width: u16,
            height: u16,
            track_left: u16,
            track_top: u16,
            track_width: u16,
            track_height: u16,
            border_left: i16,
            border_top: i16,
            border_right: i16,
            border_bottom: i16,
        ) -> xcb_randr_set_panning_cookie_t,
    >,
    xcb_randr_set_panning_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            crtc: xcb_randr_crtc_t,
            timestamp: xcb_timestamp_t,
            left: u16,
            top: u16,
            width: u16,
            height: u16,
            track_left: u16,
            track_top: u16,
            track_width: u16,
            track_height: u16,
            border_left: i16,
            border_top: i16,
            border_right: i16,
            border_bottom: i16,
        ) -> xcb_randr_set_panning_cookie_t,
    >,
    xcb_randr_set_panning_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_set_panning_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_set_panning_reply_t,
    >,
    xcb_randr_set_output_primary_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            output: xcb_randr_output_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_set_output_primary: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            output: xcb_randr_output_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_get_output_primary: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_randr_get_output_primary_cookie_t,
    >,
    xcb_randr_get_output_primary_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_randr_get_output_primary_cookie_t,
    >,
    xcb_randr_get_output_primary_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_get_output_primary_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_get_output_primary_reply_t,
    >,
    xcb_randr_get_providers_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_randr_get_providers: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_randr_get_providers_cookie_t,
    >,
    xcb_randr_get_providers_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_randr_get_providers_cookie_t,
    >,
    xcb_randr_get_providers_providers: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_providers_reply_t) -> *mut xcb_randr_provider_t,
    >,
    xcb_randr_get_providers_providers_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_providers_reply_t) -> c_int>,
    xcb_randr_get_providers_providers_end:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_providers_reply_t) -> xcb_generic_iterator_t>,
    xcb_randr_get_providers_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_get_providers_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_get_providers_reply_t,
    >,
    xcb_randr_get_provider_info_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_randr_get_provider_info: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            provider: xcb_randr_provider_t,
            config_timestamp: xcb_timestamp_t,
        ) -> xcb_randr_get_provider_info_cookie_t,
    >,
    xcb_randr_get_provider_info_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            provider: xcb_randr_provider_t,
            config_timestamp: xcb_timestamp_t,
        ) -> xcb_randr_get_provider_info_cookie_t,
    >,
    xcb_randr_get_provider_info_crtcs: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_provider_info_reply_t) -> *mut xcb_randr_crtc_t,
    >,
    xcb_randr_get_provider_info_crtcs_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_provider_info_reply_t) -> c_int>,
    xcb_randr_get_provider_info_crtcs_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_provider_info_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_get_provider_info_outputs: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_provider_info_reply_t) -> *mut xcb_randr_output_t,
    >,
    xcb_randr_get_provider_info_outputs_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_provider_info_reply_t) -> c_int>,
    xcb_randr_get_provider_info_outputs_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_provider_info_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_get_provider_info_associated_providers: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_provider_info_reply_t) -> *mut xcb_randr_provider_t,
    >,
    xcb_randr_get_provider_info_associated_providers_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_provider_info_reply_t) -> c_int>,
    xcb_randr_get_provider_info_associated_providers_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_provider_info_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_get_provider_info_associated_capability:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_provider_info_reply_t) -> *mut u32>,
    xcb_randr_get_provider_info_associated_capability_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_provider_info_reply_t) -> c_int>,
    xcb_randr_get_provider_info_associated_capability_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_provider_info_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_get_provider_info_name:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_provider_info_reply_t) -> *mut c_char>,
    xcb_randr_get_provider_info_name_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_provider_info_reply_t) -> c_int>,
    xcb_randr_get_provider_info_name_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_provider_info_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_get_provider_info_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_get_provider_info_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_get_provider_info_reply_t,
    >,
    xcb_randr_set_provider_offload_sink_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            provider: xcb_randr_provider_t,
            sink_provider: xcb_randr_provider_t,
            config_timestamp: xcb_timestamp_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_set_provider_offload_sink: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            provider: xcb_randr_provider_t,
            sink_provider: xcb_randr_provider_t,
            config_timestamp: xcb_timestamp_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_set_provider_output_source_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            provider: xcb_randr_provider_t,
            source_provider: xcb_randr_provider_t,
            config_timestamp: xcb_timestamp_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_set_provider_output_source: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            provider: xcb_randr_provider_t,
            source_provider: xcb_randr_provider_t,
            config_timestamp: xcb_timestamp_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_list_provider_properties_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_randr_list_provider_properties: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            provider: xcb_randr_provider_t,
        ) -> xcb_randr_list_provider_properties_cookie_t,
    >,
    xcb_randr_list_provider_properties_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            provider: xcb_randr_provider_t,
        ) -> xcb_randr_list_provider_properties_cookie_t,
    >,
    xcb_randr_list_provider_properties_atoms: LazySymbol<
        unsafe fn(r: *const xcb_randr_list_provider_properties_reply_t) -> *mut xcb_atom_t,
    >,
    xcb_randr_list_provider_properties_atoms_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_list_provider_properties_reply_t) -> c_int>,
    xcb_randr_list_provider_properties_atoms_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_list_provider_properties_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_list_provider_properties_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_list_provider_properties_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_list_provider_properties_reply_t,
    >,
    xcb_randr_query_provider_property_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_randr_query_provider_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            provider: xcb_randr_provider_t,
            property: xcb_atom_t,
        ) -> xcb_randr_query_provider_property_cookie_t,
    >,
    xcb_randr_query_provider_property_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            provider: xcb_randr_provider_t,
            property: xcb_atom_t,
        ) -> xcb_randr_query_provider_property_cookie_t,
    >,
    xcb_randr_query_provider_property_valid_values:
        LazySymbol<unsafe fn(r: *const xcb_randr_query_provider_property_reply_t) -> *mut i32>,
    xcb_randr_query_provider_property_valid_values_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_query_provider_property_reply_t) -> c_int>,
    xcb_randr_query_provider_property_valid_values_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_query_provider_property_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_query_provider_property_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_query_provider_property_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_query_provider_property_reply_t,
    >,
    xcb_randr_configure_provider_property_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, values_len: u32) -> c_int>,
    xcb_randr_configure_provider_property_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            provider: xcb_randr_provider_t,
            property: xcb_atom_t,
            pending: u8,
            range: u8,
            values_len: u32,
            values: *const i32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_configure_provider_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            provider: xcb_randr_provider_t,
            property: xcb_atom_t,
            pending: u8,
            range: u8,
            values_len: u32,
            values: *const i32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_configure_provider_property_values: LazySymbol<
        unsafe fn(r: *const xcb_randr_configure_provider_property_request_t) -> *mut i32,
    >,
    xcb_randr_configure_provider_property_values_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_configure_provider_property_request_t) -> c_int>,
    xcb_randr_configure_provider_property_values_end: LazySymbol<
        unsafe fn(
            r: *const xcb_randr_configure_provider_property_request_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_randr_change_provider_property_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_randr_change_provider_property_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            provider: xcb_randr_provider_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            format: u8,
            mode: u8,
            num_items: u32,
            data: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_change_provider_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            provider: xcb_randr_provider_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            format: u8,
            mode: u8,
            num_items: u32,
            data: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_change_provider_property_data: LazySymbol<
        unsafe fn(r: *const xcb_randr_change_provider_property_request_t) -> *mut c_void,
    >,
    xcb_randr_change_provider_property_data_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_change_provider_property_request_t) -> c_int>,
    xcb_randr_change_provider_property_data_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_change_provider_property_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_delete_provider_property_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            provider: xcb_randr_provider_t,
            property: xcb_atom_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_delete_provider_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            provider: xcb_randr_provider_t,
            property: xcb_atom_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_get_provider_property_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_randr_get_provider_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            provider: xcb_randr_provider_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            long_offset: u32,
            long_length: u32,
            delete: u8,
            pending: u8,
        ) -> xcb_randr_get_provider_property_cookie_t,
    >,
    xcb_randr_get_provider_property_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            provider: xcb_randr_provider_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            long_offset: u32,
            long_length: u32,
            delete: u8,
            pending: u8,
        ) -> xcb_randr_get_provider_property_cookie_t,
    >,
    xcb_randr_get_provider_property_data:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_provider_property_reply_t) -> *mut c_void>,
    xcb_randr_get_provider_property_data_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_provider_property_reply_t) -> c_int>,
    xcb_randr_get_provider_property_data_end: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_provider_property_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_randr_get_provider_property_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_get_provider_property_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_get_provider_property_reply_t,
    >,
    xcb_randr_crtc_change_next: LazySymbol<unsafe fn(i: *mut xcb_randr_crtc_change_iterator_t)>,
    xcb_randr_crtc_change_end:
        LazySymbol<unsafe fn(i: xcb_randr_crtc_change_iterator_t) -> xcb_generic_iterator_t>,
    xcb_randr_output_change_next: LazySymbol<unsafe fn(i: *mut xcb_randr_output_change_iterator_t)>,
    xcb_randr_output_change_end:
        LazySymbol<unsafe fn(i: xcb_randr_output_change_iterator_t) -> xcb_generic_iterator_t>,
    xcb_randr_output_property_next:
        LazySymbol<unsafe fn(i: *mut xcb_randr_output_property_iterator_t)>,
    xcb_randr_output_property_end:
        LazySymbol<unsafe fn(i: xcb_randr_output_property_iterator_t) -> xcb_generic_iterator_t>,
    xcb_randr_provider_change_next:
        LazySymbol<unsafe fn(i: *mut xcb_randr_provider_change_iterator_t)>,
    xcb_randr_provider_change_end:
        LazySymbol<unsafe fn(i: xcb_randr_provider_change_iterator_t) -> xcb_generic_iterator_t>,
    xcb_randr_provider_property_next:
        LazySymbol<unsafe fn(i: *mut xcb_randr_provider_property_iterator_t)>,
    xcb_randr_provider_property_end:
        LazySymbol<unsafe fn(i: xcb_randr_provider_property_iterator_t) -> xcb_generic_iterator_t>,
    xcb_randr_resource_change_next:
        LazySymbol<unsafe fn(i: *mut xcb_randr_resource_change_iterator_t)>,
    xcb_randr_resource_change_end:
        LazySymbol<unsafe fn(i: xcb_randr_resource_change_iterator_t) -> xcb_generic_iterator_t>,
    xcb_randr_monitor_info_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_randr_monitor_info_outputs:
        LazySymbol<unsafe fn(r: *const xcb_randr_monitor_info_t) -> *mut xcb_randr_output_t>,
    xcb_randr_monitor_info_outputs_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_monitor_info_t) -> c_int>,
    xcb_randr_monitor_info_outputs_end:
        LazySymbol<unsafe fn(r: *const xcb_randr_monitor_info_t) -> xcb_generic_iterator_t>,
    xcb_randr_monitor_info_next: LazySymbol<unsafe fn(i: *mut xcb_randr_monitor_info_iterator_t)>,
    xcb_randr_monitor_info_end:
        LazySymbol<unsafe fn(i: xcb_randr_monitor_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_randr_get_monitors_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_randr_get_monitors: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            get_active: u8,
        ) -> xcb_randr_get_monitors_cookie_t,
    >,
    xcb_randr_get_monitors_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            get_active: u8,
        ) -> xcb_randr_get_monitors_cookie_t,
    >,
    xcb_randr_get_monitors_monitors_length:
        LazySymbol<unsafe fn(r: *const xcb_randr_get_monitors_reply_t) -> c_int>,
    xcb_randr_get_monitors_monitors_iterator: LazySymbol<
        unsafe fn(r: *const xcb_randr_get_monitors_reply_t) -> xcb_randr_monitor_info_iterator_t,
    >,
    xcb_randr_get_monitors_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_get_monitors_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_get_monitors_reply_t,
    >,
    xcb_randr_set_monitor_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_randr_set_monitor_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            monitorinfo: *mut xcb_randr_monitor_info_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_set_monitor: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            monitorinfo: *mut xcb_randr_monitor_info_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_set_monitor_monitorinfo: LazySymbol<
        unsafe fn(r: *const xcb_randr_set_monitor_request_t) -> *mut xcb_randr_monitor_info_t,
    >,
    xcb_randr_delete_monitor_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            name: xcb_atom_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_delete_monitor: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            name: xcb_atom_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_create_lease_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_randr_create_lease: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            lid: xcb_randr_lease_t,
            num_crtcs: u16,
            num_outputs: u16,
            crtcs: *const xcb_randr_crtc_t,
            outputs: *const xcb_randr_output_t,
        ) -> xcb_randr_create_lease_cookie_t,
    >,
    xcb_randr_create_lease_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            lid: xcb_randr_lease_t,
            num_crtcs: u16,
            num_outputs: u16,
            crtcs: *const xcb_randr_crtc_t,
            outputs: *const xcb_randr_output_t,
        ) -> xcb_randr_create_lease_cookie_t,
    >,
    xcb_randr_create_lease_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_randr_create_lease_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_randr_create_lease_reply_t,
    >,
    xcb_randr_create_lease_reply_fds: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            reply: *mut xcb_randr_create_lease_reply_t,
        ) -> *mut c_int,
    >,
    xcb_randr_free_lease_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            lid: xcb_randr_lease_t,
            terminate: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_free_lease: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            lid: xcb_randr_lease_t,
            terminate: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_randr_lease_notify_next: LazySymbol<unsafe fn(i: *mut xcb_randr_lease_notify_iterator_t)>,
    xcb_randr_lease_notify_end:
        LazySymbol<unsafe fn(i: xcb_randr_lease_notify_iterator_t) -> xcb_generic_iterator_t>,
    xcb_randr_notify_data_next: LazySymbol<unsafe fn(i: *mut xcb_randr_notify_data_iterator_t)>,
    xcb_randr_notify_data_end:
        LazySymbol<unsafe fn(i: xcb_randr_notify_data_iterator_t) -> xcb_generic_iterator_t>,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self
            .randr
            .$f
            .get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .randr
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_randr")]
impl XcbRandr {
    /// The libxcb identifier of the `RandR` extension.
    #[inline]
    pub fn xcb_randr_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_randr_id) }
    }

    /// Returns `true` iff the symbol `xcb_randr_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_id(&self) -> bool {
        has_sym!(self, xcb_randr_id)
    }

    /// Advances a `xcb_randr_mode_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_randr_mode_next(&self, i: *mut xcb_randr_mode_iterator_t) {
        sym!(self, xcb_randr_mode_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_mode_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_mode_next(&self) -> bool {
        has_sym!(self, xcb_randr_mode_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_randr_mode_iterator_t`.
    #[inline]
    pub unsafe fn xcb_randr_mode_end(
        &self,
        i: xcb_randr_mode_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_mode_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_mode_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_mode_end(&self) -> bool {
        has_sym!(self, xcb_randr_mode_end)
    }

    /// Advances a `xcb_randr_crtc_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_randr_crtc_next(&self, i: *mut xcb_randr_crtc_iterator_t) {
        sym!(self, xcb_randr_crtc_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_crtc_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_crtc_next(&self) -> bool {
        has_sym!(self, xcb_randr_crtc_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_randr_crtc_iterator_t`.
    #[inline]
    pub unsafe fn xcb_randr_crtc_end(
        &self,
        i: xcb_randr_crtc_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_crtc_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_crtc_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_crtc_end(&self) -> bool {
        has_sym!(self, xcb_randr_crtc_end)
    }

    /// Advances a `xcb_randr_output_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_randr_output_next(&self, i: *mut xcb_randr_output_iterator_t) {
        sym!(self, xcb_randr_output_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_output_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_output_next(&self) -> bool {
        has_sym!(self, xcb_randr_output_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_randr_output_iterator_t`.
    #[inline]
    pub unsafe fn xcb_randr_output_end(
        &self,
        i: xcb_randr_output_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_output_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_output_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_output_end(&self) -> bool {
        has_sym!(self, xcb_randr_output_end)
    }

    /// Advances a `xcb_randr_provider_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_randr_provider_next(&self, i: *mut xcb_randr_provider_iterator_t) {
        sym!(self, xcb_randr_provider_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_provider_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_provider_next(&self) -> bool {
        has_sym!(self, xcb_randr_provider_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_randr_provider_iterator_t`.
    #[inline]
    pub unsafe fn xcb_randr_provider_end(
        &self,
        i: xcb_randr_provider_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_provider_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_provider_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_provider_end(&self) -> bool {
        has_sym!(self, xcb_randr_provider_end)
    }

    /// Advances a `xcb_randr_lease_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_randr_lease_next(&self, i: *mut xcb_randr_lease_iterator_t) {
        sym!(self, xcb_randr_lease_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_lease_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_lease_next(&self) -> bool {
        has_sym!(self, xcb_randr_lease_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_randr_lease_iterator_t`.
    #[inline]
    pub unsafe fn xcb_randr_lease_end(
        &self,
        i: xcb_randr_lease_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_lease_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_lease_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_lease_end(&self) -> bool {
        has_sym!(self, xcb_randr_lease_end)
    }

    /// Advances a `xcb_randr_screen_size_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_randr_screen_size_next(&self, i: *mut xcb_randr_screen_size_iterator_t) {
        sym!(self, xcb_randr_screen_size_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_screen_size_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_screen_size_next(&self) -> bool {
        has_sym!(self, xcb_randr_screen_size_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_randr_screen_size_iterator_t`.
    #[inline]
    pub unsafe fn xcb_randr_screen_size_end(
        &self,
        i: xcb_randr_screen_size_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_screen_size_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_screen_size_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_screen_size_end(&self) -> bool {
        has_sym!(self, xcb_randr_screen_size_end)
    }

    /// Computes the size of a `xcb_randr_refresh_rates_t` object.
    #[inline]
    pub unsafe fn xcb_randr_refresh_rates_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_randr_refresh_rates_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_randr_refresh_rates_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_refresh_rates_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_refresh_rates_sizeof)
    }

    /// Returns a pointer to the `rates` field of a `xcb_randr_refresh_rates_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_refresh_rates_rates(
        &self,
        r: *const xcb_randr_refresh_rates_t,
    ) -> *mut u16 {
        sym!(self, xcb_randr_refresh_rates_rates)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_refresh_rates_rates` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_refresh_rates_rates(&self) -> bool {
        has_sym!(self, xcb_randr_refresh_rates_rates)
    }

    /// Returns the number of elements of the `rates` field of a `xcb_randr_refresh_rates_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_refresh_rates_rates_length(
        &self,
        r: *const xcb_randr_refresh_rates_t,
    ) -> c_int {
        sym!(self, xcb_randr_refresh_rates_rates_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_refresh_rates_rates_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_refresh_rates_rates_length(&self) -> bool {
        has_sym!(self, xcb_randr_refresh_rates_rates_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `rates` field of a `xcb_randr_refresh_rates_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_refresh_rates_rates_end(
        &self,
        r: *const xcb_randr_refresh_rates_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_refresh_rates_rates_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_refresh_rates_rates_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_refresh_rates_rates_end(&self) -> bool {
        has_sym!(self, xcb_randr_refresh_rates_rates_end)
    }

    /// Advances a `xcb_randr_refresh_rates_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_randr_refresh_rates_next(&self, i: *mut xcb_randr_refresh_rates_iterator_t) {
        sym!(self, xcb_randr_refresh_rates_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_refresh_rates_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_refresh_rates_next(&self) -> bool {
        has_sym!(self, xcb_randr_refresh_rates_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_randr_refresh_rates_iterator_t`.
    #[inline]
    pub unsafe fn xcb_randr_refresh_rates_end(
        &self,
        i: xcb_randr_refresh_rates_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_refresh_rates_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_refresh_rates_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_refresh_rates_end(&self) -> bool {
        has_sym!(self, xcb_randr_refresh_rates_end)
    }

    /// Sends a `RandR::QueryVersion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_query_version_reply`]: Self::xcb_randr_query_version_reply
    #[inline]
    pub unsafe fn xcb_randr_query_version(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
    ) -> xcb_randr_query_version_cookie_t {
        sym!(self, xcb_randr_query_version)(c, major_version, minor_version)
    }

    /// Returns `true` iff the symbol `xcb_randr_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_query_version(&self) -> bool {
        has_sym!(self, xcb_randr_query_version)
    }

    /// Sends a `RandR::QueryVersion` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_query_version_reply`]: Self::xcb_randr_query_version_reply
    #[inline]
    pub unsafe fn xcb_randr_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
    ) -> xcb_randr_query_version_cookie_t {
        sym!(self, xcb_randr_query_version_unchecked)(c, major_version, minor_version)
    }

    /// Returns `true` iff the symbol `xcb_randr_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_query_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_query_version_unchecked)
    }

    /// Waits for the reply to a `RandR::QueryVersion` request.
    #[inline]
    pub unsafe fn xcb_randr_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_query_version_reply_t {
        sym!(self, xcb_randr_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_randr_query_version_reply)
    }

    /// Sends a `RandR::SetScreenConfig` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_set_screen_config_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_set_screen_config_reply`]: Self::xcb_randr_set_screen_config_reply
    #[inline]
    pub unsafe fn xcb_randr_set_screen_config(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        timestamp: xcb_timestamp_t,
        config_timestamp: xcb_timestamp_t,
        size_i_d: u16,
        rotation: u16,
        rate: u16,
    ) -> xcb_randr_set_screen_config_cookie_t {
        sym!(self, xcb_randr_set_screen_config)(
            c,
            window,
            timestamp,
            config_timestamp,
            size_i_d,
            rotation,
            rate,
        )
    }

    /// Returns `true` iff the symbol `xcb_randr_set_screen_config` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_screen_config(&self) -> bool {
        has_sym!(self, xcb_randr_set_screen_config)
    }

    /// Sends a `RandR::SetScreenConfig` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_set_screen_config_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_set_screen_config_reply`]: Self::xcb_randr_set_screen_config_reply
    #[inline]
    pub unsafe fn xcb_randr_set_screen_config_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        timestamp: xcb_timestamp_t,
        config_timestamp: xcb_timestamp_t,
        size_i_d: u16,
        rotation: u16,
        rate: u16,
    ) -> xcb_randr_set_screen_config_cookie_t {
        sym!(self, xcb_randr_set_screen_config_unchecked)(
            c,
            window,
            timestamp,
            config_timestamp,
            size_i_d,
            rotation,
            rate,
        )
    }

    /// Returns `true` iff the symbol `xcb_randr_set_screen_config_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_screen_config_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_set_screen_config_unchecked)
    }

    /// Waits for the reply to a `RandR::SetScreenConfig` request.
    #[inline]
    pub unsafe fn xcb_randr_set_screen_config_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_set_screen_config_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_set_screen_config_reply_t {
        sym!(self, xcb_randr_set_screen_config_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_screen_config_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_screen_config_reply(&self) -> bool {
        has_sym!(self, xcb_randr_set_screen_config_reply)
    }

    /// Sends a `RandR::SelectInput` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_randr_select_input_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        enable: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_select_input_checked)(c, window, enable)
    }

    /// Returns `true` iff the symbol `xcb_randr_select_input_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_select_input_checked(&self) -> bool {
        has_sym!(self, xcb_randr_select_input_checked)
    }

    /// Sends a `RandR::SelectInput` request (unchecked).
    #[inline]
    pub unsafe fn xcb_randr_select_input(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        enable: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_select_input)(c, window, enable)
    }

    /// Returns `true` iff the symbol `xcb_randr_select_input` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_select_input(&self) -> bool {
        has_sym!(self, xcb_randr_select_input)
    }

    /// Computes the size of a `xcb_randr_get_screen_info_reply_t` object.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_randr_get_screen_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_info_sizeof)
    }

    /// Sends a `RandR::GetScreenInfo` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_screen_info_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_screen_info_reply`]: Self::xcb_randr_get_screen_info_reply
    #[inline]
    pub unsafe fn xcb_randr_get_screen_info(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_screen_info_cookie_t {
        sym!(self, xcb_randr_get_screen_info)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_info` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_info(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_info)
    }

    /// Sends a `RandR::GetScreenInfo` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_screen_info_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_screen_info_reply`]: Self::xcb_randr_get_screen_info_reply
    #[inline]
    pub unsafe fn xcb_randr_get_screen_info_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_screen_info_cookie_t {
        sym!(self, xcb_randr_get_screen_info_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_info_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_info_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_info_unchecked)
    }

    /// Returns a pointer to the `sizes` field of a `xcb_randr_get_screen_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_info_sizes(
        &self,
        r: *const xcb_randr_get_screen_info_reply_t,
    ) -> *mut xcb_randr_screen_size_t {
        sym!(self, xcb_randr_get_screen_info_sizes)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_info_sizes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_info_sizes(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_info_sizes)
    }

    /// Returns the number of elements of the `sizes` field of a `xcb_randr_get_screen_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_info_sizes_length(
        &self,
        r: *const xcb_randr_get_screen_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_screen_info_sizes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_info_sizes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_info_sizes_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_info_sizes_length)
    }

    /// Returns an iterator over the elements of the
    /// `sizes` field of a `xcb_randr_get_screen_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_info_sizes_iterator(
        &self,
        r: *const xcb_randr_get_screen_info_reply_t,
    ) -> xcb_randr_screen_size_iterator_t {
        sym!(self, xcb_randr_get_screen_info_sizes_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_info_sizes_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_info_sizes_iterator(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_info_sizes_iterator)
    }

    /// Returns the number of elements of the `rates` field of a `xcb_randr_get_screen_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_info_rates_length(
        &self,
        r: *const xcb_randr_get_screen_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_screen_info_rates_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_info_rates_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_info_rates_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_info_rates_length)
    }

    /// Returns an iterator over the elements of the
    /// `rates` field of a `xcb_randr_get_screen_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_info_rates_iterator(
        &self,
        r: *const xcb_randr_get_screen_info_reply_t,
    ) -> xcb_randr_refresh_rates_iterator_t {
        sym!(self, xcb_randr_get_screen_info_rates_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_info_rates_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_info_rates_iterator(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_info_rates_iterator)
    }

    /// Waits for the reply to a `RandR::GetScreenInfo` request.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_info_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_screen_info_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_screen_info_reply_t {
        sym!(self, xcb_randr_get_screen_info_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_info_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_info_reply(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_info_reply)
    }

    /// Sends a `RandR::GetScreenSizeRange` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_screen_size_range_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_screen_size_range_reply`]: Self::xcb_randr_get_screen_size_range_reply
    #[inline]
    pub unsafe fn xcb_randr_get_screen_size_range(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_screen_size_range_cookie_t {
        sym!(self, xcb_randr_get_screen_size_range)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_size_range` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_size_range(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_size_range)
    }

    /// Sends a `RandR::GetScreenSizeRange` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_screen_size_range_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_screen_size_range_reply`]: Self::xcb_randr_get_screen_size_range_reply
    #[inline]
    pub unsafe fn xcb_randr_get_screen_size_range_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_screen_size_range_cookie_t {
        sym!(self, xcb_randr_get_screen_size_range_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_size_range_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_size_range_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_size_range_unchecked)
    }

    /// Waits for the reply to a `RandR::GetScreenSizeRange` request.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_size_range_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_screen_size_range_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_screen_size_range_reply_t {
        sym!(self, xcb_randr_get_screen_size_range_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_size_range_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_size_range_reply(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_size_range_reply)
    }

    /// Sends a `RandR::SetScreenSize` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_randr_set_screen_size_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        width: u16,
        height: u16,
        mm_width: u32,
        mm_height: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_screen_size_checked)(c, window, width, height, mm_width, mm_height)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_screen_size_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_screen_size_checked(&self) -> bool {
        has_sym!(self, xcb_randr_set_screen_size_checked)
    }

    /// Sends a `RandR::SetScreenSize` request (unchecked).
    #[inline]
    pub unsafe fn xcb_randr_set_screen_size(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        width: u16,
        height: u16,
        mm_width: u32,
        mm_height: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_screen_size)(c, window, width, height, mm_width, mm_height)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_screen_size` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_screen_size(&self) -> bool {
        has_sym!(self, xcb_randr_set_screen_size)
    }

    /// Advances a `xcb_randr_mode_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_randr_mode_info_next(&self, i: *mut xcb_randr_mode_info_iterator_t) {
        sym!(self, xcb_randr_mode_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_mode_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_mode_info_next(&self) -> bool {
        has_sym!(self, xcb_randr_mode_info_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_randr_mode_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_randr_mode_info_end(
        &self,
        i: xcb_randr_mode_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_mode_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_mode_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_mode_info_end(&self) -> bool {
        has_sym!(self, xcb_randr_mode_info_end)
    }

    /// Computes the size of a `xcb_randr_get_screen_resources_reply_t` object.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_randr_get_screen_resources_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_sizeof)
    }

    /// Sends a `RandR::GetScreenResources` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_screen_resources_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_screen_resources_reply`]: Self::xcb_randr_get_screen_resources_reply
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_screen_resources_cookie_t {
        sym!(self, xcb_randr_get_screen_resources)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources)
    }

    /// Sends a `RandR::GetScreenResources` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_screen_resources_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_screen_resources_reply`]: Self::xcb_randr_get_screen_resources_reply
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_screen_resources_cookie_t {
        sym!(self, xcb_randr_get_screen_resources_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_unchecked)
    }

    /// Returns a pointer to the `crtcs` field of a `xcb_randr_get_screen_resources_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_crtcs(
        &self,
        r: *const xcb_randr_get_screen_resources_reply_t,
    ) -> *mut xcb_randr_crtc_t {
        sym!(self, xcb_randr_get_screen_resources_crtcs)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_crtcs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_crtcs(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_crtcs)
    }

    /// Returns the number of elements of the `crtcs` field of a `xcb_randr_get_screen_resources_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_crtcs_length(
        &self,
        r: *const xcb_randr_get_screen_resources_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_screen_resources_crtcs_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_crtcs_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_crtcs_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_crtcs_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `crtcs` field of a `xcb_randr_get_screen_resources_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_crtcs_end(
        &self,
        r: *const xcb_randr_get_screen_resources_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_screen_resources_crtcs_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_crtcs_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_crtcs_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_crtcs_end)
    }

    /// Returns a pointer to the `outputs` field of a `xcb_randr_get_screen_resources_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_outputs(
        &self,
        r: *const xcb_randr_get_screen_resources_reply_t,
    ) -> *mut xcb_randr_output_t {
        sym!(self, xcb_randr_get_screen_resources_outputs)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_outputs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_outputs(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_outputs)
    }

    /// Returns the number of elements of the `outputs` field of a `xcb_randr_get_screen_resources_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_outputs_length(
        &self,
        r: *const xcb_randr_get_screen_resources_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_screen_resources_outputs_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_outputs_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_outputs_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_outputs_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `outputs` field of a `xcb_randr_get_screen_resources_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_outputs_end(
        &self,
        r: *const xcb_randr_get_screen_resources_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_screen_resources_outputs_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_outputs_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_outputs_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_outputs_end)
    }

    /// Returns a pointer to the `modes` field of a `xcb_randr_get_screen_resources_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_modes(
        &self,
        r: *const xcb_randr_get_screen_resources_reply_t,
    ) -> *mut xcb_randr_mode_info_t {
        sym!(self, xcb_randr_get_screen_resources_modes)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_modes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_modes(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_modes)
    }

    /// Returns the number of elements of the `modes` field of a `xcb_randr_get_screen_resources_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_modes_length(
        &self,
        r: *const xcb_randr_get_screen_resources_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_screen_resources_modes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_modes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_modes_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_modes_length)
    }

    /// Returns an iterator over the elements of the
    /// `modes` field of a `xcb_randr_get_screen_resources_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_modes_iterator(
        &self,
        r: *const xcb_randr_get_screen_resources_reply_t,
    ) -> xcb_randr_mode_info_iterator_t {
        sym!(self, xcb_randr_get_screen_resources_modes_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_modes_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_modes_iterator(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_modes_iterator)
    }

    /// Returns a pointer to the `names` field of a `xcb_randr_get_screen_resources_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_names(
        &self,
        r: *const xcb_randr_get_screen_resources_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_randr_get_screen_resources_names)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_names` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_names(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_names)
    }

    /// Returns the number of elements of the `names` field of a `xcb_randr_get_screen_resources_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_names_length(
        &self,
        r: *const xcb_randr_get_screen_resources_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_screen_resources_names_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_names_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_names_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `names` field of a `xcb_randr_get_screen_resources_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_names_end(
        &self,
        r: *const xcb_randr_get_screen_resources_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_screen_resources_names_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_names_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_names_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_names_end)
    }

    /// Waits for the reply to a `RandR::GetScreenResources` request.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_screen_resources_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_screen_resources_reply_t {
        sym!(self, xcb_randr_get_screen_resources_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_reply(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_reply)
    }

    /// Computes the size of a `xcb_randr_get_output_info_reply_t` object.
    #[inline]
    pub unsafe fn xcb_randr_get_output_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_randr_get_output_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_info_sizeof)
    }

    /// Sends a `RandR::GetOutputInfo` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_output_info_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_output_info_reply`]: Self::xcb_randr_get_output_info_reply
    #[inline]
    pub unsafe fn xcb_randr_get_output_info(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_randr_get_output_info_cookie_t {
        sym!(self, xcb_randr_get_output_info)(c, output, config_timestamp)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_info` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_info(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_info)
    }

    /// Sends a `RandR::GetOutputInfo` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_output_info_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_output_info_reply`]: Self::xcb_randr_get_output_info_reply
    #[inline]
    pub unsafe fn xcb_randr_get_output_info_unchecked(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_randr_get_output_info_cookie_t {
        sym!(self, xcb_randr_get_output_info_unchecked)(c, output, config_timestamp)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_info_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_info_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_info_unchecked)
    }

    /// Returns a pointer to the `crtcs` field of a `xcb_randr_get_output_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_output_info_crtcs(
        &self,
        r: *const xcb_randr_get_output_info_reply_t,
    ) -> *mut xcb_randr_crtc_t {
        sym!(self, xcb_randr_get_output_info_crtcs)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_info_crtcs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_info_crtcs(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_info_crtcs)
    }

    /// Returns the number of elements of the `crtcs` field of a `xcb_randr_get_output_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_output_info_crtcs_length(
        &self,
        r: *const xcb_randr_get_output_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_output_info_crtcs_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_info_crtcs_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_info_crtcs_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_info_crtcs_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `crtcs` field of a `xcb_randr_get_output_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_output_info_crtcs_end(
        &self,
        r: *const xcb_randr_get_output_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_output_info_crtcs_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_info_crtcs_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_info_crtcs_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_info_crtcs_end)
    }

    /// Returns a pointer to the `modes` field of a `xcb_randr_get_output_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_output_info_modes(
        &self,
        r: *const xcb_randr_get_output_info_reply_t,
    ) -> *mut xcb_randr_mode_t {
        sym!(self, xcb_randr_get_output_info_modes)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_info_modes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_info_modes(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_info_modes)
    }

    /// Returns the number of elements of the `modes` field of a `xcb_randr_get_output_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_output_info_modes_length(
        &self,
        r: *const xcb_randr_get_output_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_output_info_modes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_info_modes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_info_modes_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_info_modes_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `modes` field of a `xcb_randr_get_output_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_output_info_modes_end(
        &self,
        r: *const xcb_randr_get_output_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_output_info_modes_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_info_modes_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_info_modes_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_info_modes_end)
    }

    /// Returns a pointer to the `clones` field of a `xcb_randr_get_output_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_output_info_clones(
        &self,
        r: *const xcb_randr_get_output_info_reply_t,
    ) -> *mut xcb_randr_output_t {
        sym!(self, xcb_randr_get_output_info_clones)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_info_clones` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_info_clones(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_info_clones)
    }

    /// Returns the number of elements of the `clones` field of a `xcb_randr_get_output_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_output_info_clones_length(
        &self,
        r: *const xcb_randr_get_output_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_output_info_clones_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_info_clones_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_info_clones_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_info_clones_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `clones` field of a `xcb_randr_get_output_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_output_info_clones_end(
        &self,
        r: *const xcb_randr_get_output_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_output_info_clones_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_info_clones_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_info_clones_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_info_clones_end)
    }

    /// Returns a pointer to the `name` field of a `xcb_randr_get_output_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_output_info_name(
        &self,
        r: *const xcb_randr_get_output_info_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_randr_get_output_info_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_info_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_info_name(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_info_name)
    }

    /// Returns the number of elements of the `name` field of a `xcb_randr_get_output_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_output_info_name_length(
        &self,
        r: *const xcb_randr_get_output_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_output_info_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_info_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_info_name_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_info_name_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `name` field of a `xcb_randr_get_output_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_output_info_name_end(
        &self,
        r: *const xcb_randr_get_output_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_output_info_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_info_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_info_name_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_info_name_end)
    }

    /// Waits for the reply to a `RandR::GetOutputInfo` request.
    #[inline]
    pub unsafe fn xcb_randr_get_output_info_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_output_info_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_output_info_reply_t {
        sym!(self, xcb_randr_get_output_info_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_info_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_info_reply(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_info_reply)
    }

    /// Computes the size of a `xcb_randr_list_output_properties_reply_t` object.
    #[inline]
    pub unsafe fn xcb_randr_list_output_properties_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_randr_list_output_properties_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_randr_list_output_properties_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_list_output_properties_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_list_output_properties_sizeof)
    }

    /// Sends a `RandR::ListOutputProperties` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_list_output_properties_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_list_output_properties_reply`]: Self::xcb_randr_list_output_properties_reply
    #[inline]
    pub unsafe fn xcb_randr_list_output_properties(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
    ) -> xcb_randr_list_output_properties_cookie_t {
        sym!(self, xcb_randr_list_output_properties)(c, output)
    }

    /// Returns `true` iff the symbol `xcb_randr_list_output_properties` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_list_output_properties(&self) -> bool {
        has_sym!(self, xcb_randr_list_output_properties)
    }

    /// Sends a `RandR::ListOutputProperties` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_list_output_properties_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_list_output_properties_reply`]: Self::xcb_randr_list_output_properties_reply
    #[inline]
    pub unsafe fn xcb_randr_list_output_properties_unchecked(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
    ) -> xcb_randr_list_output_properties_cookie_t {
        sym!(self, xcb_randr_list_output_properties_unchecked)(c, output)
    }

    /// Returns `true` iff the symbol `xcb_randr_list_output_properties_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_list_output_properties_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_list_output_properties_unchecked)
    }

    /// Returns a pointer to the `atoms` field of a `xcb_randr_list_output_properties_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_list_output_properties_atoms(
        &self,
        r: *const xcb_randr_list_output_properties_reply_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_randr_list_output_properties_atoms)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_list_output_properties_atoms` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_list_output_properties_atoms(&self) -> bool {
        has_sym!(self, xcb_randr_list_output_properties_atoms)
    }

    /// Returns the number of elements of the `atoms` field of a `xcb_randr_list_output_properties_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_list_output_properties_atoms_length(
        &self,
        r: *const xcb_randr_list_output_properties_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_list_output_properties_atoms_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_list_output_properties_atoms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_list_output_properties_atoms_length(&self) -> bool {
        has_sym!(self, xcb_randr_list_output_properties_atoms_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `atoms` field of a `xcb_randr_list_output_properties_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_list_output_properties_atoms_end(
        &self,
        r: *const xcb_randr_list_output_properties_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_list_output_properties_atoms_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_list_output_properties_atoms_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_list_output_properties_atoms_end(&self) -> bool {
        has_sym!(self, xcb_randr_list_output_properties_atoms_end)
    }

    /// Waits for the reply to a `RandR::ListOutputProperties` request.
    #[inline]
    pub unsafe fn xcb_randr_list_output_properties_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_list_output_properties_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_list_output_properties_reply_t {
        sym!(self, xcb_randr_list_output_properties_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_list_output_properties_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_list_output_properties_reply(&self) -> bool {
        has_sym!(self, xcb_randr_list_output_properties_reply)
    }

    /// Computes the size of a `xcb_randr_query_output_property_reply_t` object.
    #[inline]
    pub unsafe fn xcb_randr_query_output_property_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_randr_query_output_property_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_randr_query_output_property_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_query_output_property_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_query_output_property_sizeof)
    }

    /// Sends a `RandR::QueryOutputProperty` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_query_output_property_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_query_output_property_reply`]: Self::xcb_randr_query_output_property_reply
    #[inline]
    pub unsafe fn xcb_randr_query_output_property(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        property: xcb_atom_t,
    ) -> xcb_randr_query_output_property_cookie_t {
        sym!(self, xcb_randr_query_output_property)(c, output, property)
    }

    /// Returns `true` iff the symbol `xcb_randr_query_output_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_query_output_property(&self) -> bool {
        has_sym!(self, xcb_randr_query_output_property)
    }

    /// Sends a `RandR::QueryOutputProperty` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_query_output_property_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_query_output_property_reply`]: Self::xcb_randr_query_output_property_reply
    #[inline]
    pub unsafe fn xcb_randr_query_output_property_unchecked(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        property: xcb_atom_t,
    ) -> xcb_randr_query_output_property_cookie_t {
        sym!(self, xcb_randr_query_output_property_unchecked)(c, output, property)
    }

    /// Returns `true` iff the symbol `xcb_randr_query_output_property_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_query_output_property_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_query_output_property_unchecked)
    }

    /// Returns a pointer to the `validValues` field of a `xcb_randr_query_output_property_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_query_output_property_valid_values(
        &self,
        r: *const xcb_randr_query_output_property_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_randr_query_output_property_valid_values)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_query_output_property_valid_values` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_query_output_property_valid_values(&self) -> bool {
        has_sym!(self, xcb_randr_query_output_property_valid_values)
    }

    /// Returns the number of elements of the `validValues` field of a `xcb_randr_query_output_property_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_query_output_property_valid_values_length(
        &self,
        r: *const xcb_randr_query_output_property_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_query_output_property_valid_values_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_query_output_property_valid_values_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_query_output_property_valid_values_length(&self) -> bool {
        has_sym!(self, xcb_randr_query_output_property_valid_values_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `validValues` field of a `xcb_randr_query_output_property_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_query_output_property_valid_values_end(
        &self,
        r: *const xcb_randr_query_output_property_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_query_output_property_valid_values_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_query_output_property_valid_values_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_query_output_property_valid_values_end(&self) -> bool {
        has_sym!(self, xcb_randr_query_output_property_valid_values_end)
    }

    /// Waits for the reply to a `RandR::QueryOutputProperty` request.
    #[inline]
    pub unsafe fn xcb_randr_query_output_property_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_query_output_property_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_query_output_property_reply_t {
        sym!(self, xcb_randr_query_output_property_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_query_output_property_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_query_output_property_reply(&self) -> bool {
        has_sym!(self, xcb_randr_query_output_property_reply)
    }

    /// Computes the size of a `xcb_randr_configure_output_property_request_t` object.
    #[inline]
    pub unsafe fn xcb_randr_configure_output_property_sizeof(
        &self,
        _buffer: *const c_void,
        values_len: u32,
    ) -> c_int {
        sym!(self, xcb_randr_configure_output_property_sizeof)(_buffer, values_len)
    }

    /// Returns `true` iff the symbol `xcb_randr_configure_output_property_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_configure_output_property_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_configure_output_property_sizeof)
    }

    /// Sends a `RandR::ConfigureOutputProperty` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_randr_configure_output_property_checked(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        property: xcb_atom_t,
        pending: u8,
        range: u8,
        values_len: u32,
        values: *const i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_configure_output_property_checked)(
            c, output, property, pending, range, values_len, values,
        )
    }

    /// Returns `true` iff the symbol `xcb_randr_configure_output_property_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_configure_output_property_checked(&self) -> bool {
        has_sym!(self, xcb_randr_configure_output_property_checked)
    }

    /// Sends a `RandR::ConfigureOutputProperty` request (unchecked).
    #[inline]
    pub unsafe fn xcb_randr_configure_output_property(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        property: xcb_atom_t,
        pending: u8,
        range: u8,
        values_len: u32,
        values: *const i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_configure_output_property)(
            c, output, property, pending, range, values_len, values,
        )
    }

    /// Returns `true` iff the symbol `xcb_randr_configure_output_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_configure_output_property(&self) -> bool {
        has_sym!(self, xcb_randr_configure_output_property)
    }

    /// Returns a pointer to the `values` field of a `xcb_randr_configure_output_property_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_configure_output_property_values(
        &self,
        r: *const xcb_randr_configure_output_property_request_t,
    ) -> *mut i32 {
        sym!(self, xcb_randr_configure_output_property_values)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_configure_output_property_values` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_configure_output_property_values(&self) -> bool {
        has_sym!(self, xcb_randr_configure_output_property_values)
    }

    /// Returns the number of elements of the `values` field of a `xcb_randr_configure_output_property_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_configure_output_property_values_length(
        &self,
        r: *const xcb_randr_configure_output_property_request_t,
    ) -> c_int {
        sym!(self, xcb_randr_configure_output_property_values_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_configure_output_property_values_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_configure_output_property_values_length(&self) -> bool {
        has_sym!(self, xcb_randr_configure_output_property_values_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `values` field of a `xcb_randr_configure_output_property_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_configure_output_property_values_end(
        &self,
        r: *const xcb_randr_configure_output_property_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_configure_output_property_values_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_configure_output_property_values_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_configure_output_property_values_end(&self) -> bool {
        has_sym!(self, xcb_randr_configure_output_property_values_end)
    }

    /// Computes the size of a `xcb_randr_change_output_property_request_t` object.
    #[inline]
    pub unsafe fn xcb_randr_change_output_property_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_randr_change_output_property_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_randr_change_output_property_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_change_output_property_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_change_output_property_sizeof)
    }

    /// Sends a `RandR::ChangeOutputProperty` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_randr_change_output_property_checked(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        format: u8,
        mode: u8,
        num_units: u32,
        data: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_change_output_property_checked)(
            c, output, property, type_, format, mode, num_units, data,
        )
    }

    /// Returns `true` iff the symbol `xcb_randr_change_output_property_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_change_output_property_checked(&self) -> bool {
        has_sym!(self, xcb_randr_change_output_property_checked)
    }

    /// Sends a `RandR::ChangeOutputProperty` request (unchecked).
    #[inline]
    pub unsafe fn xcb_randr_change_output_property(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        format: u8,
        mode: u8,
        num_units: u32,
        data: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_change_output_property)(
            c, output, property, type_, format, mode, num_units, data,
        )
    }

    /// Returns `true` iff the symbol `xcb_randr_change_output_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_change_output_property(&self) -> bool {
        has_sym!(self, xcb_randr_change_output_property)
    }

    /// Returns a pointer to the `data` field of a `xcb_randr_change_output_property_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_change_output_property_data(
        &self,
        r: *const xcb_randr_change_output_property_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_randr_change_output_property_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_change_output_property_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_change_output_property_data(&self) -> bool {
        has_sym!(self, xcb_randr_change_output_property_data)
    }

    /// Returns the number of elements of the `data` field of a `xcb_randr_change_output_property_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_change_output_property_data_length(
        &self,
        r: *const xcb_randr_change_output_property_request_t,
    ) -> c_int {
        sym!(self, xcb_randr_change_output_property_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_change_output_property_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_change_output_property_data_length(&self) -> bool {
        has_sym!(self, xcb_randr_change_output_property_data_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `data` field of a `xcb_randr_change_output_property_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_change_output_property_data_end(
        &self,
        r: *const xcb_randr_change_output_property_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_change_output_property_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_change_output_property_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_change_output_property_data_end(&self) -> bool {
        has_sym!(self, xcb_randr_change_output_property_data_end)
    }

    /// Sends a `RandR::DeleteOutputProperty` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_randr_delete_output_property_checked(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        property: xcb_atom_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_delete_output_property_checked)(c, output, property)
    }

    /// Returns `true` iff the symbol `xcb_randr_delete_output_property_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_delete_output_property_checked(&self) -> bool {
        has_sym!(self, xcb_randr_delete_output_property_checked)
    }

    /// Sends a `RandR::DeleteOutputProperty` request (unchecked).
    #[inline]
    pub unsafe fn xcb_randr_delete_output_property(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        property: xcb_atom_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_delete_output_property)(c, output, property)
    }

    /// Returns `true` iff the symbol `xcb_randr_delete_output_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_delete_output_property(&self) -> bool {
        has_sym!(self, xcb_randr_delete_output_property)
    }

    /// Computes the size of a `xcb_randr_get_output_property_reply_t` object.
    #[inline]
    pub unsafe fn xcb_randr_get_output_property_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_randr_get_output_property_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_property_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_property_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_property_sizeof)
    }

    /// Sends a `RandR::GetOutputProperty` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_output_property_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_output_property_reply`]: Self::xcb_randr_get_output_property_reply
    #[inline]
    pub unsafe fn xcb_randr_get_output_property(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        long_offset: u32,
        long_length: u32,
        delete: u8,
        pending: u8,
    ) -> xcb_randr_get_output_property_cookie_t {
        sym!(self, xcb_randr_get_output_property)(
            c,
            output,
            property,
            type_,
            long_offset,
            long_length,
            delete,
            pending,
        )
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_property(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_property)
    }

    /// Sends a `RandR::GetOutputProperty` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_output_property_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_output_property_reply`]: Self::xcb_randr_get_output_property_reply
    #[inline]
    pub unsafe fn xcb_randr_get_output_property_unchecked(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        long_offset: u32,
        long_length: u32,
        delete: u8,
        pending: u8,
    ) -> xcb_randr_get_output_property_cookie_t {
        sym!(self, xcb_randr_get_output_property_unchecked)(
            c,
            output,
            property,
            type_,
            long_offset,
            long_length,
            delete,
            pending,
        )
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_property_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_property_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_property_unchecked)
    }

    /// Returns a pointer to the `data` field of a `xcb_randr_get_output_property_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_output_property_data(
        &self,
        r: *const xcb_randr_get_output_property_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_randr_get_output_property_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_property_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_property_data(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_property_data)
    }

    /// Returns the number of elements of the `data` field of a `xcb_randr_get_output_property_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_output_property_data_length(
        &self,
        r: *const xcb_randr_get_output_property_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_output_property_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_property_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_property_data_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_property_data_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `data` field of a `xcb_randr_get_output_property_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_output_property_data_end(
        &self,
        r: *const xcb_randr_get_output_property_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_output_property_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_property_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_property_data_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_property_data_end)
    }

    /// Waits for the reply to a `RandR::GetOutputProperty` request.
    #[inline]
    pub unsafe fn xcb_randr_get_output_property_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_output_property_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_output_property_reply_t {
        sym!(self, xcb_randr_get_output_property_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_property_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_property_reply(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_property_reply)
    }

    /// Computes the size of a `xcb_randr_create_mode_request_t` object.
    #[inline]
    pub unsafe fn xcb_randr_create_mode_sizeof(
        &self,
        _buffer: *const c_void,
        name_len: u32,
    ) -> c_int {
        sym!(self, xcb_randr_create_mode_sizeof)(_buffer, name_len)
    }

    /// Returns `true` iff the symbol `xcb_randr_create_mode_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_create_mode_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_create_mode_sizeof)
    }

    /// Sends a `RandR::CreateMode` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_create_mode_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_create_mode_reply`]: Self::xcb_randr_create_mode_reply
    #[inline]
    pub unsafe fn xcb_randr_create_mode(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        mode_info: xcb_randr_mode_info_t,
        name_len: u32,
        name: *const c_char,
    ) -> xcb_randr_create_mode_cookie_t {
        sym!(self, xcb_randr_create_mode)(c, window, mode_info, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_randr_create_mode` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_create_mode(&self) -> bool {
        has_sym!(self, xcb_randr_create_mode)
    }

    /// Sends a `RandR::CreateMode` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_create_mode_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_create_mode_reply`]: Self::xcb_randr_create_mode_reply
    #[inline]
    pub unsafe fn xcb_randr_create_mode_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        mode_info: xcb_randr_mode_info_t,
        name_len: u32,
        name: *const c_char,
    ) -> xcb_randr_create_mode_cookie_t {
        sym!(self, xcb_randr_create_mode_unchecked)(c, window, mode_info, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_randr_create_mode_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_create_mode_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_create_mode_unchecked)
    }

    /// Waits for the reply to a `RandR::CreateMode` request.
    #[inline]
    pub unsafe fn xcb_randr_create_mode_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_create_mode_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_create_mode_reply_t {
        sym!(self, xcb_randr_create_mode_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_create_mode_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_create_mode_reply(&self) -> bool {
        has_sym!(self, xcb_randr_create_mode_reply)
    }

    /// Sends a `RandR::DestroyMode` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_randr_destroy_mode_checked(
        &self,
        c: *mut xcb_connection_t,
        mode: xcb_randr_mode_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_destroy_mode_checked)(c, mode)
    }

    /// Returns `true` iff the symbol `xcb_randr_destroy_mode_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_destroy_mode_checked(&self) -> bool {
        has_sym!(self, xcb_randr_destroy_mode_checked)
    }

    /// Sends a `RandR::DestroyMode` request (unchecked).
    #[inline]
    pub unsafe fn xcb_randr_destroy_mode(
        &self,
        c: *mut xcb_connection_t,
        mode: xcb_randr_mode_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_destroy_mode)(c, mode)
    }

    /// Returns `true` iff the symbol `xcb_randr_destroy_mode` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_destroy_mode(&self) -> bool {
        has_sym!(self, xcb_randr_destroy_mode)
    }

    /// Sends a `RandR::AddOutputMode` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_randr_add_output_mode_checked(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        mode: xcb_randr_mode_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_add_output_mode_checked)(c, output, mode)
    }

    /// Returns `true` iff the symbol `xcb_randr_add_output_mode_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_add_output_mode_checked(&self) -> bool {
        has_sym!(self, xcb_randr_add_output_mode_checked)
    }

    /// Sends a `RandR::AddOutputMode` request (unchecked).
    #[inline]
    pub unsafe fn xcb_randr_add_output_mode(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        mode: xcb_randr_mode_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_add_output_mode)(c, output, mode)
    }

    /// Returns `true` iff the symbol `xcb_randr_add_output_mode` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_add_output_mode(&self) -> bool {
        has_sym!(self, xcb_randr_add_output_mode)
    }

    /// Sends a `RandR::DeleteOutputMode` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_randr_delete_output_mode_checked(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        mode: xcb_randr_mode_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_delete_output_mode_checked)(c, output, mode)
    }

    /// Returns `true` iff the symbol `xcb_randr_delete_output_mode_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_delete_output_mode_checked(&self) -> bool {
        has_sym!(self, xcb_randr_delete_output_mode_checked)
    }

    /// Sends a `RandR::DeleteOutputMode` request (unchecked).
    #[inline]
    pub unsafe fn xcb_randr_delete_output_mode(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        mode: xcb_randr_mode_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_delete_output_mode)(c, output, mode)
    }

    /// Returns `true` iff the symbol `xcb_randr_delete_output_mode` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_delete_output_mode(&self) -> bool {
        has_sym!(self, xcb_randr_delete_output_mode)
    }

    /// Computes the size of a `xcb_randr_get_crtc_info_reply_t` object.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_randr_get_crtc_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_info_sizeof)
    }

    /// Sends a `RandR::GetCrtcInfo` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_crtc_info_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_crtc_info_reply`]: Self::xcb_randr_get_crtc_info_reply
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_info(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_randr_get_crtc_info_cookie_t {
        sym!(self, xcb_randr_get_crtc_info)(c, crtc, config_timestamp)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_info` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_info(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_info)
    }

    /// Sends a `RandR::GetCrtcInfo` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_crtc_info_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_crtc_info_reply`]: Self::xcb_randr_get_crtc_info_reply
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_info_unchecked(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_randr_get_crtc_info_cookie_t {
        sym!(self, xcb_randr_get_crtc_info_unchecked)(c, crtc, config_timestamp)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_info_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_info_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_info_unchecked)
    }

    /// Returns a pointer to the `outputs` field of a `xcb_randr_get_crtc_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_info_outputs(
        &self,
        r: *const xcb_randr_get_crtc_info_reply_t,
    ) -> *mut xcb_randr_output_t {
        sym!(self, xcb_randr_get_crtc_info_outputs)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_info_outputs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_info_outputs(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_info_outputs)
    }

    /// Returns the number of elements of the `outputs` field of a `xcb_randr_get_crtc_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_info_outputs_length(
        &self,
        r: *const xcb_randr_get_crtc_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_crtc_info_outputs_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_info_outputs_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_info_outputs_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_info_outputs_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `outputs` field of a `xcb_randr_get_crtc_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_info_outputs_end(
        &self,
        r: *const xcb_randr_get_crtc_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_crtc_info_outputs_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_info_outputs_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_info_outputs_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_info_outputs_end)
    }

    /// Returns a pointer to the `possible` field of a `xcb_randr_get_crtc_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_info_possible(
        &self,
        r: *const xcb_randr_get_crtc_info_reply_t,
    ) -> *mut xcb_randr_output_t {
        sym!(self, xcb_randr_get_crtc_info_possible)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_info_possible` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_info_possible(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_info_possible)
    }

    /// Returns the number of elements of the `possible` field of a `xcb_randr_get_crtc_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_info_possible_length(
        &self,
        r: *const xcb_randr_get_crtc_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_crtc_info_possible_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_info_possible_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_info_possible_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_info_possible_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `possible` field of a `xcb_randr_get_crtc_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_info_possible_end(
        &self,
        r: *const xcb_randr_get_crtc_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_crtc_info_possible_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_info_possible_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_info_possible_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_info_possible_end)
    }

    /// Waits for the reply to a `RandR::GetCrtcInfo` request.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_info_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_crtc_info_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_crtc_info_reply_t {
        sym!(self, xcb_randr_get_crtc_info_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_info_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_info_reply(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_info_reply)
    }

    /// Computes the size of a `xcb_randr_set_crtc_config_request_t` object.
    #[inline]
    pub unsafe fn xcb_randr_set_crtc_config_sizeof(
        &self,
        _buffer: *const c_void,
        outputs_len: u32,
    ) -> c_int {
        sym!(self, xcb_randr_set_crtc_config_sizeof)(_buffer, outputs_len)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_crtc_config_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_crtc_config_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_set_crtc_config_sizeof)
    }

    /// Sends a `RandR::SetCrtcConfig` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_set_crtc_config_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_set_crtc_config_reply`]: Self::xcb_randr_set_crtc_config_reply
    #[inline]
    pub unsafe fn xcb_randr_set_crtc_config(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
        timestamp: xcb_timestamp_t,
        config_timestamp: xcb_timestamp_t,
        x: i16,
        y: i16,
        mode: xcb_randr_mode_t,
        rotation: u16,
        outputs_len: u32,
        outputs: *const xcb_randr_output_t,
    ) -> xcb_randr_set_crtc_config_cookie_t {
        sym!(self, xcb_randr_set_crtc_config)(
            c,
            crtc,
            timestamp,
            config_timestamp,
            x,
            y,
            mode,
            rotation,
            outputs_len,
            outputs,
        )
    }

    /// Returns `true` iff the symbol `xcb_randr_set_crtc_config` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_crtc_config(&self) -> bool {
        has_sym!(self, xcb_randr_set_crtc_config)
    }

    /// Sends a `RandR::SetCrtcConfig` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_set_crtc_config_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_set_crtc_config_reply`]: Self::xcb_randr_set_crtc_config_reply
    #[inline]
    pub unsafe fn xcb_randr_set_crtc_config_unchecked(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
        timestamp: xcb_timestamp_t,
        config_timestamp: xcb_timestamp_t,
        x: i16,
        y: i16,
        mode: xcb_randr_mode_t,
        rotation: u16,
        outputs_len: u32,
        outputs: *const xcb_randr_output_t,
    ) -> xcb_randr_set_crtc_config_cookie_t {
        sym!(self, xcb_randr_set_crtc_config_unchecked)(
            c,
            crtc,
            timestamp,
            config_timestamp,
            x,
            y,
            mode,
            rotation,
            outputs_len,
            outputs,
        )
    }

    /// Returns `true` iff the symbol `xcb_randr_set_crtc_config_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_crtc_config_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_set_crtc_config_unchecked)
    }

    /// Waits for the reply to a `RandR::SetCrtcConfig` request.
    #[inline]
    pub unsafe fn xcb_randr_set_crtc_config_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_set_crtc_config_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_set_crtc_config_reply_t {
        sym!(self, xcb_randr_set_crtc_config_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_crtc_config_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_crtc_config_reply(&self) -> bool {
        has_sym!(self, xcb_randr_set_crtc_config_reply)
    }

    /// Sends a `RandR::GetCrtcGammaSize` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_crtc_gamma_size_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_crtc_gamma_size_reply`]: Self::xcb_randr_get_crtc_gamma_size_reply
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_size(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
    ) -> xcb_randr_get_crtc_gamma_size_cookie_t {
        sym!(self, xcb_randr_get_crtc_gamma_size)(c, crtc)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_gamma_size` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_gamma_size(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_gamma_size)
    }

    /// Sends a `RandR::GetCrtcGammaSize` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_crtc_gamma_size_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_crtc_gamma_size_reply`]: Self::xcb_randr_get_crtc_gamma_size_reply
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_size_unchecked(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
    ) -> xcb_randr_get_crtc_gamma_size_cookie_t {
        sym!(self, xcb_randr_get_crtc_gamma_size_unchecked)(c, crtc)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_gamma_size_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_gamma_size_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_gamma_size_unchecked)
    }

    /// Waits for the reply to a `RandR::GetCrtcGammaSize` request.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_size_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_crtc_gamma_size_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_crtc_gamma_size_reply_t {
        sym!(self, xcb_randr_get_crtc_gamma_size_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_gamma_size_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_gamma_size_reply(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_gamma_size_reply)
    }

    /// Computes the size of a `xcb_randr_get_crtc_gamma_reply_t` object.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_randr_get_crtc_gamma_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_gamma_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_gamma_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_gamma_sizeof)
    }

    /// Sends a `RandR::GetCrtcGamma` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_crtc_gamma_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_crtc_gamma_reply`]: Self::xcb_randr_get_crtc_gamma_reply
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
    ) -> xcb_randr_get_crtc_gamma_cookie_t {
        sym!(self, xcb_randr_get_crtc_gamma)(c, crtc)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_gamma` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_gamma(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_gamma)
    }

    /// Sends a `RandR::GetCrtcGamma` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_crtc_gamma_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_crtc_gamma_reply`]: Self::xcb_randr_get_crtc_gamma_reply
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_unchecked(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
    ) -> xcb_randr_get_crtc_gamma_cookie_t {
        sym!(self, xcb_randr_get_crtc_gamma_unchecked)(c, crtc)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_gamma_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_gamma_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_gamma_unchecked)
    }

    /// Returns a pointer to the `red` field of a `xcb_randr_get_crtc_gamma_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_red(
        &self,
        r: *const xcb_randr_get_crtc_gamma_reply_t,
    ) -> *mut u16 {
        sym!(self, xcb_randr_get_crtc_gamma_red)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_gamma_red` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_gamma_red(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_gamma_red)
    }

    /// Returns the number of elements of the `red` field of a `xcb_randr_get_crtc_gamma_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_red_length(
        &self,
        r: *const xcb_randr_get_crtc_gamma_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_crtc_gamma_red_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_gamma_red_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_gamma_red_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_gamma_red_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `red` field of a `xcb_randr_get_crtc_gamma_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_red_end(
        &self,
        r: *const xcb_randr_get_crtc_gamma_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_crtc_gamma_red_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_gamma_red_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_gamma_red_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_gamma_red_end)
    }

    /// Returns a pointer to the `green` field of a `xcb_randr_get_crtc_gamma_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_green(
        &self,
        r: *const xcb_randr_get_crtc_gamma_reply_t,
    ) -> *mut u16 {
        sym!(self, xcb_randr_get_crtc_gamma_green)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_gamma_green` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_gamma_green(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_gamma_green)
    }

    /// Returns the number of elements of the `green` field of a `xcb_randr_get_crtc_gamma_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_green_length(
        &self,
        r: *const xcb_randr_get_crtc_gamma_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_crtc_gamma_green_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_gamma_green_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_gamma_green_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_gamma_green_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `green` field of a `xcb_randr_get_crtc_gamma_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_green_end(
        &self,
        r: *const xcb_randr_get_crtc_gamma_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_crtc_gamma_green_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_gamma_green_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_gamma_green_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_gamma_green_end)
    }

    /// Returns a pointer to the `blue` field of a `xcb_randr_get_crtc_gamma_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_blue(
        &self,
        r: *const xcb_randr_get_crtc_gamma_reply_t,
    ) -> *mut u16 {
        sym!(self, xcb_randr_get_crtc_gamma_blue)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_gamma_blue` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_gamma_blue(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_gamma_blue)
    }

    /// Returns the number of elements of the `blue` field of a `xcb_randr_get_crtc_gamma_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_blue_length(
        &self,
        r: *const xcb_randr_get_crtc_gamma_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_crtc_gamma_blue_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_gamma_blue_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_gamma_blue_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_gamma_blue_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `blue` field of a `xcb_randr_get_crtc_gamma_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_blue_end(
        &self,
        r: *const xcb_randr_get_crtc_gamma_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_crtc_gamma_blue_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_gamma_blue_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_gamma_blue_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_gamma_blue_end)
    }

    /// Waits for the reply to a `RandR::GetCrtcGamma` request.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_crtc_gamma_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_crtc_gamma_reply_t {
        sym!(self, xcb_randr_get_crtc_gamma_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_gamma_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_gamma_reply(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_gamma_reply)
    }

    /// Computes the size of a `xcb_randr_set_crtc_gamma_request_t` object.
    #[inline]
    pub unsafe fn xcb_randr_set_crtc_gamma_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_randr_set_crtc_gamma_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_crtc_gamma_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_crtc_gamma_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_set_crtc_gamma_sizeof)
    }

    /// Sends a `RandR::SetCrtcGamma` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_randr_set_crtc_gamma_checked(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
        size: u16,
        red: *const u16,
        green: *const u16,
        blue: *const u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_crtc_gamma_checked)(c, crtc, size, red, green, blue)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_crtc_gamma_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_crtc_gamma_checked(&self) -> bool {
        has_sym!(self, xcb_randr_set_crtc_gamma_checked)
    }

    /// Sends a `RandR::SetCrtcGamma` request (unchecked).
    #[inline]
    pub unsafe fn xcb_randr_set_crtc_gamma(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
        size: u16,
        red: *const u16,
        green: *const u16,
        blue: *const u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_crtc_gamma)(c, crtc, size, red, green, blue)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_crtc_gamma` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_crtc_gamma(&self) -> bool {
        has_sym!(self, xcb_randr_set_crtc_gamma)
    }

    /// Returns a pointer to the `red` field of a `xcb_randr_set_crtc_gamma_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_set_crtc_gamma_red(
        &self,
        r: *const xcb_randr_set_crtc_gamma_request_t,
    ) -> *mut u16 {
        sym!(self, xcb_randr_set_crtc_gamma_red)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_crtc_gamma_red` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_crtc_gamma_red(&self) -> bool {
        has_sym!(self, xcb_randr_set_crtc_gamma_red)
    }

    /// Returns the number of elements of the `red` field of a `xcb_randr_set_crtc_gamma_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_set_crtc_gamma_red_length(
        &self,
        r: *const xcb_randr_set_crtc_gamma_request_t,
    ) -> c_int {
        sym!(self, xcb_randr_set_crtc_gamma_red_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_crtc_gamma_red_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_crtc_gamma_red_length(&self) -> bool {
        has_sym!(self, xcb_randr_set_crtc_gamma_red_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `red` field of a `xcb_randr_set_crtc_gamma_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_set_crtc_gamma_red_end(
        &self,
        r: *const xcb_randr_set_crtc_gamma_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_set_crtc_gamma_red_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_crtc_gamma_red_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_crtc_gamma_red_end(&self) -> bool {
        has_sym!(self, xcb_randr_set_crtc_gamma_red_end)
    }

    /// Returns a pointer to the `green` field of a `xcb_randr_set_crtc_gamma_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_set_crtc_gamma_green(
        &self,
        r: *const xcb_randr_set_crtc_gamma_request_t,
    ) -> *mut u16 {
        sym!(self, xcb_randr_set_crtc_gamma_green)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_crtc_gamma_green` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_crtc_gamma_green(&self) -> bool {
        has_sym!(self, xcb_randr_set_crtc_gamma_green)
    }

    /// Returns the number of elements of the `green` field of a `xcb_randr_set_crtc_gamma_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_set_crtc_gamma_green_length(
        &self,
        r: *const xcb_randr_set_crtc_gamma_request_t,
    ) -> c_int {
        sym!(self, xcb_randr_set_crtc_gamma_green_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_crtc_gamma_green_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_crtc_gamma_green_length(&self) -> bool {
        has_sym!(self, xcb_randr_set_crtc_gamma_green_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `green` field of a `xcb_randr_set_crtc_gamma_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_set_crtc_gamma_green_end(
        &self,
        r: *const xcb_randr_set_crtc_gamma_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_set_crtc_gamma_green_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_crtc_gamma_green_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_crtc_gamma_green_end(&self) -> bool {
        has_sym!(self, xcb_randr_set_crtc_gamma_green_end)
    }

    /// Returns a pointer to the `blue` field of a `xcb_randr_set_crtc_gamma_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_set_crtc_gamma_blue(
        &self,
        r: *const xcb_randr_set_crtc_gamma_request_t,
    ) -> *mut u16 {
        sym!(self, xcb_randr_set_crtc_gamma_blue)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_crtc_gamma_blue` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_crtc_gamma_blue(&self) -> bool {
        has_sym!(self, xcb_randr_set_crtc_gamma_blue)
    }

    /// Returns the number of elements of the `blue` field of a `xcb_randr_set_crtc_gamma_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_set_crtc_gamma_blue_length(
        &self,
        r: *const xcb_randr_set_crtc_gamma_request_t,
    ) -> c_int {
        sym!(self, xcb_randr_set_crtc_gamma_blue_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_crtc_gamma_blue_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_crtc_gamma_blue_length(&self) -> bool {
        has_sym!(self, xcb_randr_set_crtc_gamma_blue_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `blue` field of a `xcb_randr_set_crtc_gamma_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_set_crtc_gamma_blue_end(
        &self,
        r: *const xcb_randr_set_crtc_gamma_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_set_crtc_gamma_blue_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_crtc_gamma_blue_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_crtc_gamma_blue_end(&self) -> bool {
        has_sym!(self, xcb_randr_set_crtc_gamma_blue_end)
    }

    /// Computes the size of a `xcb_randr_get_screen_resources_current_reply_t` object.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_randr_get_screen_resources_current_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_current_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_current_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_current_sizeof)
    }

    /// Sends a `RandR::GetScreenResourcesCurrent` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_screen_resources_current_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_screen_resources_current_reply`]: Self::xcb_randr_get_screen_resources_current_reply
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_screen_resources_current_cookie_t {
        sym!(self, xcb_randr_get_screen_resources_current)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_current` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_current(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_current)
    }

    /// Sends a `RandR::GetScreenResourcesCurrent` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_screen_resources_current_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_screen_resources_current_reply`]: Self::xcb_randr_get_screen_resources_current_reply
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_screen_resources_current_cookie_t {
        sym!(self, xcb_randr_get_screen_resources_current_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_current_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_current_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_current_unchecked)
    }

    /// Returns a pointer to the `crtcs` field of a `xcb_randr_get_screen_resources_current_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_crtcs(
        &self,
        r: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> *mut xcb_randr_crtc_t {
        sym!(self, xcb_randr_get_screen_resources_current_crtcs)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_current_crtcs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_current_crtcs(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_current_crtcs)
    }

    /// Returns the number of elements of the `crtcs` field of a `xcb_randr_get_screen_resources_current_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_crtcs_length(
        &self,
        r: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_screen_resources_current_crtcs_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_current_crtcs_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_current_crtcs_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_current_crtcs_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `crtcs` field of a `xcb_randr_get_screen_resources_current_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_crtcs_end(
        &self,
        r: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_screen_resources_current_crtcs_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_current_crtcs_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_current_crtcs_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_current_crtcs_end)
    }

    /// Returns a pointer to the `outputs` field of a `xcb_randr_get_screen_resources_current_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_outputs(
        &self,
        r: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> *mut xcb_randr_output_t {
        sym!(self, xcb_randr_get_screen_resources_current_outputs)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_current_outputs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_current_outputs(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_current_outputs)
    }

    /// Returns the number of elements of the `outputs` field of a `xcb_randr_get_screen_resources_current_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_outputs_length(
        &self,
        r: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_screen_resources_current_outputs_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_current_outputs_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_current_outputs_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_current_outputs_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `outputs` field of a `xcb_randr_get_screen_resources_current_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_outputs_end(
        &self,
        r: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_screen_resources_current_outputs_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_current_outputs_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_current_outputs_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_current_outputs_end)
    }

    /// Returns a pointer to the `modes` field of a `xcb_randr_get_screen_resources_current_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_modes(
        &self,
        r: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> *mut xcb_randr_mode_info_t {
        sym!(self, xcb_randr_get_screen_resources_current_modes)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_current_modes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_current_modes(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_current_modes)
    }

    /// Returns the number of elements of the `modes` field of a `xcb_randr_get_screen_resources_current_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_modes_length(
        &self,
        r: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_screen_resources_current_modes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_current_modes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_current_modes_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_current_modes_length)
    }

    /// Returns an iterator over the elements of the
    /// `modes` field of a `xcb_randr_get_screen_resources_current_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_modes_iterator(
        &self,
        r: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> xcb_randr_mode_info_iterator_t {
        sym!(self, xcb_randr_get_screen_resources_current_modes_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_current_modes_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_current_modes_iterator(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_current_modes_iterator)
    }

    /// Returns a pointer to the `names` field of a `xcb_randr_get_screen_resources_current_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_names(
        &self,
        r: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_randr_get_screen_resources_current_names)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_current_names` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_current_names(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_current_names)
    }

    /// Returns the number of elements of the `names` field of a `xcb_randr_get_screen_resources_current_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_names_length(
        &self,
        r: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_screen_resources_current_names_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_current_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_current_names_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_current_names_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `names` field of a `xcb_randr_get_screen_resources_current_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_names_end(
        &self,
        r: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_screen_resources_current_names_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_current_names_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_current_names_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_current_names_end)
    }

    /// Waits for the reply to a `RandR::GetScreenResourcesCurrent` request.
    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_screen_resources_current_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_screen_resources_current_reply_t {
        sym!(self, xcb_randr_get_screen_resources_current_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_screen_resources_current_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_screen_resources_current_reply(&self) -> bool {
        has_sym!(self, xcb_randr_get_screen_resources_current_reply)
    }

    /// Computes the size of a `xcb_randr_set_crtc_transform_request_t` object.
    #[inline]
    pub unsafe fn xcb_randr_set_crtc_transform_sizeof(
        &self,
        _buffer: *const c_void,
        filter_params_len: u32,
    ) -> c_int {
        sym!(self, xcb_randr_set_crtc_transform_sizeof)(_buffer, filter_params_len)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_crtc_transform_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_crtc_transform_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_set_crtc_transform_sizeof)
    }

    /// Sends a `RandR::SetCrtcTransform` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_randr_set_crtc_transform_checked(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
        transform: xcb_render_transform_t,
        filter_len: u16,
        filter_name: *const c_char,
        filter_params_len: u32,
        filter_params: *const xcb_render_fixed_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_crtc_transform_checked)(
            c,
            crtc,
            transform,
            filter_len,
            filter_name,
            filter_params_len,
            filter_params,
        )
    }

    /// Returns `true` iff the symbol `xcb_randr_set_crtc_transform_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_crtc_transform_checked(&self) -> bool {
        has_sym!(self, xcb_randr_set_crtc_transform_checked)
    }

    /// Sends a `RandR::SetCrtcTransform` request (unchecked).
    #[inline]
    pub unsafe fn xcb_randr_set_crtc_transform(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
        transform: xcb_render_transform_t,
        filter_len: u16,
        filter_name: *const c_char,
        filter_params_len: u32,
        filter_params: *const xcb_render_fixed_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_crtc_transform)(
            c,
            crtc,
            transform,
            filter_len,
            filter_name,
            filter_params_len,
            filter_params,
        )
    }

    /// Returns `true` iff the symbol `xcb_randr_set_crtc_transform` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_crtc_transform(&self) -> bool {
        has_sym!(self, xcb_randr_set_crtc_transform)
    }

    /// Returns a pointer to the `filter_name` field of a `xcb_randr_set_crtc_transform_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_set_crtc_transform_filter_name(
        &self,
        r: *const xcb_randr_set_crtc_transform_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_randr_set_crtc_transform_filter_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_crtc_transform_filter_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_crtc_transform_filter_name(&self) -> bool {
        has_sym!(self, xcb_randr_set_crtc_transform_filter_name)
    }

    /// Returns the number of elements of the `filter_name` field of a `xcb_randr_set_crtc_transform_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_set_crtc_transform_filter_name_length(
        &self,
        r: *const xcb_randr_set_crtc_transform_request_t,
    ) -> c_int {
        sym!(self, xcb_randr_set_crtc_transform_filter_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_crtc_transform_filter_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_crtc_transform_filter_name_length(&self) -> bool {
        has_sym!(self, xcb_randr_set_crtc_transform_filter_name_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `filter_name` field of a `xcb_randr_set_crtc_transform_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_set_crtc_transform_filter_name_end(
        &self,
        r: *const xcb_randr_set_crtc_transform_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_set_crtc_transform_filter_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_crtc_transform_filter_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_crtc_transform_filter_name_end(&self) -> bool {
        has_sym!(self, xcb_randr_set_crtc_transform_filter_name_end)
    }

    /// Returns a pointer to the `filter_params` field of a `xcb_randr_set_crtc_transform_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_set_crtc_transform_filter_params(
        &self,
        r: *const xcb_randr_set_crtc_transform_request_t,
    ) -> *mut xcb_render_fixed_t {
        sym!(self, xcb_randr_set_crtc_transform_filter_params)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_crtc_transform_filter_params` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_crtc_transform_filter_params(&self) -> bool {
        has_sym!(self, xcb_randr_set_crtc_transform_filter_params)
    }

    /// Returns the number of elements of the `filter_params` field of a `xcb_randr_set_crtc_transform_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_set_crtc_transform_filter_params_length(
        &self,
        r: *const xcb_randr_set_crtc_transform_request_t,
    ) -> c_int {
        sym!(self, xcb_randr_set_crtc_transform_filter_params_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_crtc_transform_filter_params_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_crtc_transform_filter_params_length(&self) -> bool {
        has_sym!(self, xcb_randr_set_crtc_transform_filter_params_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `filter_params` field of a `xcb_randr_set_crtc_transform_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_set_crtc_transform_filter_params_end(
        &self,
        r: *const xcb_randr_set_crtc_transform_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_set_crtc_transform_filter_params_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_crtc_transform_filter_params_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_crtc_transform_filter_params_end(&self) -> bool {
        has_sym!(self, xcb_randr_set_crtc_transform_filter_params_end)
    }

    /// Computes the size of a `xcb_randr_get_crtc_transform_reply_t` object.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_randr_get_crtc_transform_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_transform_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_transform_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_transform_sizeof)
    }

    /// Sends a `RandR::GetCrtcTransform` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_crtc_transform_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_crtc_transform_reply`]: Self::xcb_randr_get_crtc_transform_reply
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
    ) -> xcb_randr_get_crtc_transform_cookie_t {
        sym!(self, xcb_randr_get_crtc_transform)(c, crtc)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_transform` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_transform(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_transform)
    }

    /// Sends a `RandR::GetCrtcTransform` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_crtc_transform_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_crtc_transform_reply`]: Self::xcb_randr_get_crtc_transform_reply
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_unchecked(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
    ) -> xcb_randr_get_crtc_transform_cookie_t {
        sym!(self, xcb_randr_get_crtc_transform_unchecked)(c, crtc)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_transform_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_transform_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_transform_unchecked)
    }

    /// Returns a pointer to the `pending_filter_name` field of a `xcb_randr_get_crtc_transform_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_pending_filter_name(
        &self,
        r: *const xcb_randr_get_crtc_transform_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_randr_get_crtc_transform_pending_filter_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_transform_pending_filter_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_transform_pending_filter_name(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_transform_pending_filter_name)
    }

    /// Returns the number of elements of the `pending_filter_name` field of a `xcb_randr_get_crtc_transform_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_pending_filter_name_length(
        &self,
        r: *const xcb_randr_get_crtc_transform_reply_t,
    ) -> c_int {
        sym!(
            self,
            xcb_randr_get_crtc_transform_pending_filter_name_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_transform_pending_filter_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_transform_pending_filter_name_length(&self) -> bool {
        has_sym!(
            self,
            xcb_randr_get_crtc_transform_pending_filter_name_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `pending_filter_name` field of a `xcb_randr_get_crtc_transform_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_pending_filter_name_end(
        &self,
        r: *const xcb_randr_get_crtc_transform_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_crtc_transform_pending_filter_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_transform_pending_filter_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_transform_pending_filter_name_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_transform_pending_filter_name_end)
    }

    /// Returns a pointer to the `pending_params` field of a `xcb_randr_get_crtc_transform_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_pending_params(
        &self,
        r: *const xcb_randr_get_crtc_transform_reply_t,
    ) -> *mut xcb_render_fixed_t {
        sym!(self, xcb_randr_get_crtc_transform_pending_params)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_transform_pending_params` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_transform_pending_params(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_transform_pending_params)
    }

    /// Returns the number of elements of the `pending_params` field of a `xcb_randr_get_crtc_transform_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_pending_params_length(
        &self,
        r: *const xcb_randr_get_crtc_transform_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_crtc_transform_pending_params_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_transform_pending_params_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_transform_pending_params_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_transform_pending_params_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `pending_params` field of a `xcb_randr_get_crtc_transform_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_pending_params_end(
        &self,
        r: *const xcb_randr_get_crtc_transform_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_crtc_transform_pending_params_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_transform_pending_params_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_transform_pending_params_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_transform_pending_params_end)
    }

    /// Returns a pointer to the `current_filter_name` field of a `xcb_randr_get_crtc_transform_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_current_filter_name(
        &self,
        r: *const xcb_randr_get_crtc_transform_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_randr_get_crtc_transform_current_filter_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_transform_current_filter_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_transform_current_filter_name(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_transform_current_filter_name)
    }

    /// Returns the number of elements of the `current_filter_name` field of a `xcb_randr_get_crtc_transform_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_current_filter_name_length(
        &self,
        r: *const xcb_randr_get_crtc_transform_reply_t,
    ) -> c_int {
        sym!(
            self,
            xcb_randr_get_crtc_transform_current_filter_name_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_transform_current_filter_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_transform_current_filter_name_length(&self) -> bool {
        has_sym!(
            self,
            xcb_randr_get_crtc_transform_current_filter_name_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `current_filter_name` field of a `xcb_randr_get_crtc_transform_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_current_filter_name_end(
        &self,
        r: *const xcb_randr_get_crtc_transform_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_crtc_transform_current_filter_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_transform_current_filter_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_transform_current_filter_name_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_transform_current_filter_name_end)
    }

    /// Returns a pointer to the `current_params` field of a `xcb_randr_get_crtc_transform_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_current_params(
        &self,
        r: *const xcb_randr_get_crtc_transform_reply_t,
    ) -> *mut xcb_render_fixed_t {
        sym!(self, xcb_randr_get_crtc_transform_current_params)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_transform_current_params` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_transform_current_params(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_transform_current_params)
    }

    /// Returns the number of elements of the `current_params` field of a `xcb_randr_get_crtc_transform_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_current_params_length(
        &self,
        r: *const xcb_randr_get_crtc_transform_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_crtc_transform_current_params_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_transform_current_params_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_transform_current_params_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_transform_current_params_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `current_params` field of a `xcb_randr_get_crtc_transform_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_current_params_end(
        &self,
        r: *const xcb_randr_get_crtc_transform_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_crtc_transform_current_params_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_transform_current_params_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_transform_current_params_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_transform_current_params_end)
    }

    /// Waits for the reply to a `RandR::GetCrtcTransform` request.
    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_crtc_transform_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_crtc_transform_reply_t {
        sym!(self, xcb_randr_get_crtc_transform_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_crtc_transform_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_crtc_transform_reply(&self) -> bool {
        has_sym!(self, xcb_randr_get_crtc_transform_reply)
    }

    /// Sends a `RandR::GetPanning` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_panning_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_panning_reply`]: Self::xcb_randr_get_panning_reply
    #[inline]
    pub unsafe fn xcb_randr_get_panning(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
    ) -> xcb_randr_get_panning_cookie_t {
        sym!(self, xcb_randr_get_panning)(c, crtc)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_panning` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_panning(&self) -> bool {
        has_sym!(self, xcb_randr_get_panning)
    }

    /// Sends a `RandR::GetPanning` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_panning_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_panning_reply`]: Self::xcb_randr_get_panning_reply
    #[inline]
    pub unsafe fn xcb_randr_get_panning_unchecked(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
    ) -> xcb_randr_get_panning_cookie_t {
        sym!(self, xcb_randr_get_panning_unchecked)(c, crtc)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_panning_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_panning_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_get_panning_unchecked)
    }

    /// Waits for the reply to a `RandR::GetPanning` request.
    #[inline]
    pub unsafe fn xcb_randr_get_panning_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_panning_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_panning_reply_t {
        sym!(self, xcb_randr_get_panning_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_panning_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_panning_reply(&self) -> bool {
        has_sym!(self, xcb_randr_get_panning_reply)
    }

    /// Sends a `RandR::SetPanning` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_set_panning_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_set_panning_reply`]: Self::xcb_randr_set_panning_reply
    #[inline]
    pub unsafe fn xcb_randr_set_panning(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
        timestamp: xcb_timestamp_t,
        left: u16,
        top: u16,
        width: u16,
        height: u16,
        track_left: u16,
        track_top: u16,
        track_width: u16,
        track_height: u16,
        border_left: i16,
        border_top: i16,
        border_right: i16,
        border_bottom: i16,
    ) -> xcb_randr_set_panning_cookie_t {
        sym!(self, xcb_randr_set_panning)(
            c,
            crtc,
            timestamp,
            left,
            top,
            width,
            height,
            track_left,
            track_top,
            track_width,
            track_height,
            border_left,
            border_top,
            border_right,
            border_bottom,
        )
    }

    /// Returns `true` iff the symbol `xcb_randr_set_panning` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_panning(&self) -> bool {
        has_sym!(self, xcb_randr_set_panning)
    }

    /// Sends a `RandR::SetPanning` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_set_panning_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_set_panning_reply`]: Self::xcb_randr_set_panning_reply
    #[inline]
    pub unsafe fn xcb_randr_set_panning_unchecked(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
        timestamp: xcb_timestamp_t,
        left: u16,
        top: u16,
        width: u16,
        height: u16,
        track_left: u16,
        track_top: u16,
        track_width: u16,
        track_height: u16,
        border_left: i16,
        border_top: i16,
        border_right: i16,
        border_bottom: i16,
    ) -> xcb_randr_set_panning_cookie_t {
        sym!(self, xcb_randr_set_panning_unchecked)(
            c,
            crtc,
            timestamp,
            left,
            top,
            width,
            height,
            track_left,
            track_top,
            track_width,
            track_height,
            border_left,
            border_top,
            border_right,
            border_bottom,
        )
    }

    /// Returns `true` iff the symbol `xcb_randr_set_panning_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_panning_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_set_panning_unchecked)
    }

    /// Waits for the reply to a `RandR::SetPanning` request.
    #[inline]
    pub unsafe fn xcb_randr_set_panning_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_set_panning_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_set_panning_reply_t {
        sym!(self, xcb_randr_set_panning_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_panning_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_panning_reply(&self) -> bool {
        has_sym!(self, xcb_randr_set_panning_reply)
    }

    /// Sends a `RandR::SetOutputPrimary` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_randr_set_output_primary_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        output: xcb_randr_output_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_output_primary_checked)(c, window, output)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_output_primary_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_output_primary_checked(&self) -> bool {
        has_sym!(self, xcb_randr_set_output_primary_checked)
    }

    /// Sends a `RandR::SetOutputPrimary` request (unchecked).
    #[inline]
    pub unsafe fn xcb_randr_set_output_primary(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        output: xcb_randr_output_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_output_primary)(c, window, output)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_output_primary` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_output_primary(&self) -> bool {
        has_sym!(self, xcb_randr_set_output_primary)
    }

    /// Sends a `RandR::GetOutputPrimary` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_output_primary_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_output_primary_reply`]: Self::xcb_randr_get_output_primary_reply
    #[inline]
    pub unsafe fn xcb_randr_get_output_primary(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_output_primary_cookie_t {
        sym!(self, xcb_randr_get_output_primary)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_primary` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_primary(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_primary)
    }

    /// Sends a `RandR::GetOutputPrimary` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_output_primary_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_output_primary_reply`]: Self::xcb_randr_get_output_primary_reply
    #[inline]
    pub unsafe fn xcb_randr_get_output_primary_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_output_primary_cookie_t {
        sym!(self, xcb_randr_get_output_primary_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_primary_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_primary_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_primary_unchecked)
    }

    /// Waits for the reply to a `RandR::GetOutputPrimary` request.
    #[inline]
    pub unsafe fn xcb_randr_get_output_primary_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_output_primary_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_output_primary_reply_t {
        sym!(self, xcb_randr_get_output_primary_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_output_primary_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_output_primary_reply(&self) -> bool {
        has_sym!(self, xcb_randr_get_output_primary_reply)
    }

    /// Computes the size of a `xcb_randr_get_providers_reply_t` object.
    #[inline]
    pub unsafe fn xcb_randr_get_providers_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_randr_get_providers_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_providers_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_providers_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_get_providers_sizeof)
    }

    /// Sends a `RandR::GetProviders` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_providers_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_providers_reply`]: Self::xcb_randr_get_providers_reply
    #[inline]
    pub unsafe fn xcb_randr_get_providers(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_providers_cookie_t {
        sym!(self, xcb_randr_get_providers)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_providers` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_providers(&self) -> bool {
        has_sym!(self, xcb_randr_get_providers)
    }

    /// Sends a `RandR::GetProviders` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_providers_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_providers_reply`]: Self::xcb_randr_get_providers_reply
    #[inline]
    pub unsafe fn xcb_randr_get_providers_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_providers_cookie_t {
        sym!(self, xcb_randr_get_providers_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_providers_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_providers_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_get_providers_unchecked)
    }

    /// Returns a pointer to the `providers` field of a `xcb_randr_get_providers_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_providers_providers(
        &self,
        r: *const xcb_randr_get_providers_reply_t,
    ) -> *mut xcb_randr_provider_t {
        sym!(self, xcb_randr_get_providers_providers)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_providers_providers` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_providers_providers(&self) -> bool {
        has_sym!(self, xcb_randr_get_providers_providers)
    }

    /// Returns the number of elements of the `providers` field of a `xcb_randr_get_providers_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_providers_providers_length(
        &self,
        r: *const xcb_randr_get_providers_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_providers_providers_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_providers_providers_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_providers_providers_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_providers_providers_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `providers` field of a `xcb_randr_get_providers_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_providers_providers_end(
        &self,
        r: *const xcb_randr_get_providers_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_providers_providers_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_providers_providers_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_providers_providers_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_providers_providers_end)
    }

    /// Waits for the reply to a `RandR::GetProviders` request.
    #[inline]
    pub unsafe fn xcb_randr_get_providers_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_providers_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_providers_reply_t {
        sym!(self, xcb_randr_get_providers_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_providers_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_providers_reply(&self) -> bool {
        has_sym!(self, xcb_randr_get_providers_reply)
    }

    /// Computes the size of a `xcb_randr_get_provider_info_reply_t` object.
    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_randr_get_provider_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_get_provider_info_sizeof)
    }

    /// Sends a `RandR::GetProviderInfo` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_provider_info_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_provider_info_reply`]: Self::xcb_randr_get_provider_info_reply
    #[inline]
    pub unsafe fn xcb_randr_get_provider_info(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_randr_get_provider_info_cookie_t {
        sym!(self, xcb_randr_get_provider_info)(c, provider, config_timestamp)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_info` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_info(&self) -> bool {
        has_sym!(self, xcb_randr_get_provider_info)
    }

    /// Sends a `RandR::GetProviderInfo` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_provider_info_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_provider_info_reply`]: Self::xcb_randr_get_provider_info_reply
    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_unchecked(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_randr_get_provider_info_cookie_t {
        sym!(self, xcb_randr_get_provider_info_unchecked)(c, provider, config_timestamp)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_info_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_info_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_get_provider_info_unchecked)
    }

    /// Returns a pointer to the `crtcs` field of a `xcb_randr_get_provider_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_crtcs(
        &self,
        r: *const xcb_randr_get_provider_info_reply_t,
    ) -> *mut xcb_randr_crtc_t {
        sym!(self, xcb_randr_get_provider_info_crtcs)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_info_crtcs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_info_crtcs(&self) -> bool {
        has_sym!(self, xcb_randr_get_provider_info_crtcs)
    }

    /// Returns the number of elements of the `crtcs` field of a `xcb_randr_get_provider_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_crtcs_length(
        &self,
        r: *const xcb_randr_get_provider_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_provider_info_crtcs_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_info_crtcs_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_info_crtcs_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_provider_info_crtcs_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `crtcs` field of a `xcb_randr_get_provider_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_crtcs_end(
        &self,
        r: *const xcb_randr_get_provider_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_provider_info_crtcs_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_info_crtcs_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_info_crtcs_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_provider_info_crtcs_end)
    }

    /// Returns a pointer to the `outputs` field of a `xcb_randr_get_provider_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_outputs(
        &self,
        r: *const xcb_randr_get_provider_info_reply_t,
    ) -> *mut xcb_randr_output_t {
        sym!(self, xcb_randr_get_provider_info_outputs)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_info_outputs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_info_outputs(&self) -> bool {
        has_sym!(self, xcb_randr_get_provider_info_outputs)
    }

    /// Returns the number of elements of the `outputs` field of a `xcb_randr_get_provider_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_outputs_length(
        &self,
        r: *const xcb_randr_get_provider_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_provider_info_outputs_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_info_outputs_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_info_outputs_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_provider_info_outputs_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `outputs` field of a `xcb_randr_get_provider_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_outputs_end(
        &self,
        r: *const xcb_randr_get_provider_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_provider_info_outputs_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_info_outputs_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_info_outputs_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_provider_info_outputs_end)
    }

    /// Returns a pointer to the `associated_providers` field of a `xcb_randr_get_provider_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_associated_providers(
        &self,
        r: *const xcb_randr_get_provider_info_reply_t,
    ) -> *mut xcb_randr_provider_t {
        sym!(self, xcb_randr_get_provider_info_associated_providers)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_info_associated_providers` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_info_associated_providers(&self) -> bool {
        has_sym!(self, xcb_randr_get_provider_info_associated_providers)
    }

    /// Returns the number of elements of the `associated_providers` field of a `xcb_randr_get_provider_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_associated_providers_length(
        &self,
        r: *const xcb_randr_get_provider_info_reply_t,
    ) -> c_int {
        sym!(
            self,
            xcb_randr_get_provider_info_associated_providers_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_info_associated_providers_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_info_associated_providers_length(&self) -> bool {
        has_sym!(
            self,
            xcb_randr_get_provider_info_associated_providers_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `associated_providers` field of a `xcb_randr_get_provider_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_associated_providers_end(
        &self,
        r: *const xcb_randr_get_provider_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_provider_info_associated_providers_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_info_associated_providers_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_info_associated_providers_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_provider_info_associated_providers_end)
    }

    /// Returns a pointer to the `associated_capability` field of a `xcb_randr_get_provider_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_associated_capability(
        &self,
        r: *const xcb_randr_get_provider_info_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_randr_get_provider_info_associated_capability)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_info_associated_capability` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_info_associated_capability(&self) -> bool {
        has_sym!(self, xcb_randr_get_provider_info_associated_capability)
    }

    /// Returns the number of elements of the `associated_capability` field of a `xcb_randr_get_provider_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_associated_capability_length(
        &self,
        r: *const xcb_randr_get_provider_info_reply_t,
    ) -> c_int {
        sym!(
            self,
            xcb_randr_get_provider_info_associated_capability_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_info_associated_capability_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_info_associated_capability_length(&self) -> bool {
        has_sym!(
            self,
            xcb_randr_get_provider_info_associated_capability_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `associated_capability` field of a `xcb_randr_get_provider_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_associated_capability_end(
        &self,
        r: *const xcb_randr_get_provider_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_provider_info_associated_capability_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_info_associated_capability_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_info_associated_capability_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_provider_info_associated_capability_end)
    }

    /// Returns a pointer to the `name` field of a `xcb_randr_get_provider_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_name(
        &self,
        r: *const xcb_randr_get_provider_info_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_randr_get_provider_info_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_info_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_info_name(&self) -> bool {
        has_sym!(self, xcb_randr_get_provider_info_name)
    }

    /// Returns the number of elements of the `name` field of a `xcb_randr_get_provider_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_name_length(
        &self,
        r: *const xcb_randr_get_provider_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_provider_info_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_info_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_info_name_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_provider_info_name_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `name` field of a `xcb_randr_get_provider_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_name_end(
        &self,
        r: *const xcb_randr_get_provider_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_provider_info_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_info_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_info_name_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_provider_info_name_end)
    }

    /// Waits for the reply to a `RandR::GetProviderInfo` request.
    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_provider_info_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_provider_info_reply_t {
        sym!(self, xcb_randr_get_provider_info_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_info_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_info_reply(&self) -> bool {
        has_sym!(self, xcb_randr_get_provider_info_reply)
    }

    /// Sends a `RandR::SetProviderOffloadSink` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_randr_set_provider_offload_sink_checked(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        sink_provider: xcb_randr_provider_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_provider_offload_sink_checked)(
            c,
            provider,
            sink_provider,
            config_timestamp,
        )
    }

    /// Returns `true` iff the symbol `xcb_randr_set_provider_offload_sink_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_provider_offload_sink_checked(&self) -> bool {
        has_sym!(self, xcb_randr_set_provider_offload_sink_checked)
    }

    /// Sends a `RandR::SetProviderOffloadSink` request (unchecked).
    #[inline]
    pub unsafe fn xcb_randr_set_provider_offload_sink(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        sink_provider: xcb_randr_provider_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_provider_offload_sink)(
            c,
            provider,
            sink_provider,
            config_timestamp,
        )
    }

    /// Returns `true` iff the symbol `xcb_randr_set_provider_offload_sink` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_provider_offload_sink(&self) -> bool {
        has_sym!(self, xcb_randr_set_provider_offload_sink)
    }

    /// Sends a `RandR::SetProviderOutputSource` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_randr_set_provider_output_source_checked(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        source_provider: xcb_randr_provider_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_provider_output_source_checked)(
            c,
            provider,
            source_provider,
            config_timestamp,
        )
    }

    /// Returns `true` iff the symbol `xcb_randr_set_provider_output_source_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_provider_output_source_checked(&self) -> bool {
        has_sym!(self, xcb_randr_set_provider_output_source_checked)
    }

    /// Sends a `RandR::SetProviderOutputSource` request (unchecked).
    #[inline]
    pub unsafe fn xcb_randr_set_provider_output_source(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        source_provider: xcb_randr_provider_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_provider_output_source)(
            c,
            provider,
            source_provider,
            config_timestamp,
        )
    }

    /// Returns `true` iff the symbol `xcb_randr_set_provider_output_source` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_provider_output_source(&self) -> bool {
        has_sym!(self, xcb_randr_set_provider_output_source)
    }

    /// Computes the size of a `xcb_randr_list_provider_properties_reply_t` object.
    #[inline]
    pub unsafe fn xcb_randr_list_provider_properties_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_randr_list_provider_properties_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_randr_list_provider_properties_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_list_provider_properties_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_list_provider_properties_sizeof)
    }

    /// Sends a `RandR::ListProviderProperties` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_list_provider_properties_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_list_provider_properties_reply`]: Self::xcb_randr_list_provider_properties_reply
    #[inline]
    pub unsafe fn xcb_randr_list_provider_properties(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
    ) -> xcb_randr_list_provider_properties_cookie_t {
        sym!(self, xcb_randr_list_provider_properties)(c, provider)
    }

    /// Returns `true` iff the symbol `xcb_randr_list_provider_properties` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_list_provider_properties(&self) -> bool {
        has_sym!(self, xcb_randr_list_provider_properties)
    }

    /// Sends a `RandR::ListProviderProperties` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_list_provider_properties_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_list_provider_properties_reply`]: Self::xcb_randr_list_provider_properties_reply
    #[inline]
    pub unsafe fn xcb_randr_list_provider_properties_unchecked(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
    ) -> xcb_randr_list_provider_properties_cookie_t {
        sym!(self, xcb_randr_list_provider_properties_unchecked)(c, provider)
    }

    /// Returns `true` iff the symbol `xcb_randr_list_provider_properties_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_list_provider_properties_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_list_provider_properties_unchecked)
    }

    /// Returns a pointer to the `atoms` field of a `xcb_randr_list_provider_properties_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_list_provider_properties_atoms(
        &self,
        r: *const xcb_randr_list_provider_properties_reply_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_randr_list_provider_properties_atoms)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_list_provider_properties_atoms` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_list_provider_properties_atoms(&self) -> bool {
        has_sym!(self, xcb_randr_list_provider_properties_atoms)
    }

    /// Returns the number of elements of the `atoms` field of a `xcb_randr_list_provider_properties_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_list_provider_properties_atoms_length(
        &self,
        r: *const xcb_randr_list_provider_properties_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_list_provider_properties_atoms_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_list_provider_properties_atoms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_list_provider_properties_atoms_length(&self) -> bool {
        has_sym!(self, xcb_randr_list_provider_properties_atoms_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `atoms` field of a `xcb_randr_list_provider_properties_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_list_provider_properties_atoms_end(
        &self,
        r: *const xcb_randr_list_provider_properties_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_list_provider_properties_atoms_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_list_provider_properties_atoms_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_list_provider_properties_atoms_end(&self) -> bool {
        has_sym!(self, xcb_randr_list_provider_properties_atoms_end)
    }

    /// Waits for the reply to a `RandR::ListProviderProperties` request.
    #[inline]
    pub unsafe fn xcb_randr_list_provider_properties_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_list_provider_properties_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_list_provider_properties_reply_t {
        sym!(self, xcb_randr_list_provider_properties_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_list_provider_properties_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_list_provider_properties_reply(&self) -> bool {
        has_sym!(self, xcb_randr_list_provider_properties_reply)
    }

    /// Computes the size of a `xcb_randr_query_provider_property_reply_t` object.
    #[inline]
    pub unsafe fn xcb_randr_query_provider_property_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_randr_query_provider_property_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_randr_query_provider_property_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_query_provider_property_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_query_provider_property_sizeof)
    }

    /// Sends a `RandR::QueryProviderProperty` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_query_provider_property_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_query_provider_property_reply`]: Self::xcb_randr_query_provider_property_reply
    #[inline]
    pub unsafe fn xcb_randr_query_provider_property(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        property: xcb_atom_t,
    ) -> xcb_randr_query_provider_property_cookie_t {
        sym!(self, xcb_randr_query_provider_property)(c, provider, property)
    }

    /// Returns `true` iff the symbol `xcb_randr_query_provider_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_query_provider_property(&self) -> bool {
        has_sym!(self, xcb_randr_query_provider_property)
    }

    /// Sends a `RandR::QueryProviderProperty` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_query_provider_property_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_query_provider_property_reply`]: Self::xcb_randr_query_provider_property_reply
    #[inline]
    pub unsafe fn xcb_randr_query_provider_property_unchecked(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        property: xcb_atom_t,
    ) -> xcb_randr_query_provider_property_cookie_t {
        sym!(self, xcb_randr_query_provider_property_unchecked)(c, provider, property)
    }

    /// Returns `true` iff the symbol `xcb_randr_query_provider_property_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_query_provider_property_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_query_provider_property_unchecked)
    }

    /// Returns a pointer to the `valid_values` field of a `xcb_randr_query_provider_property_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_query_provider_property_valid_values(
        &self,
        r: *const xcb_randr_query_provider_property_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_randr_query_provider_property_valid_values)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_query_provider_property_valid_values` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_query_provider_property_valid_values(&self) -> bool {
        has_sym!(self, xcb_randr_query_provider_property_valid_values)
    }

    /// Returns the number of elements of the `valid_values` field of a `xcb_randr_query_provider_property_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_query_provider_property_valid_values_length(
        &self,
        r: *const xcb_randr_query_provider_property_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_query_provider_property_valid_values_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_query_provider_property_valid_values_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_query_provider_property_valid_values_length(&self) -> bool {
        has_sym!(self, xcb_randr_query_provider_property_valid_values_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `valid_values` field of a `xcb_randr_query_provider_property_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_query_provider_property_valid_values_end(
        &self,
        r: *const xcb_randr_query_provider_property_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_query_provider_property_valid_values_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_query_provider_property_valid_values_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_query_provider_property_valid_values_end(&self) -> bool {
        has_sym!(self, xcb_randr_query_provider_property_valid_values_end)
    }

    /// Waits for the reply to a `RandR::QueryProviderProperty` request.
    #[inline]
    pub unsafe fn xcb_randr_query_provider_property_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_query_provider_property_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_query_provider_property_reply_t {
        sym!(self, xcb_randr_query_provider_property_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_query_provider_property_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_query_provider_property_reply(&self) -> bool {
        has_sym!(self, xcb_randr_query_provider_property_reply)
    }

    /// Computes the size of a `xcb_randr_configure_provider_property_request_t` object.
    #[inline]
    pub unsafe fn xcb_randr_configure_provider_property_sizeof(
        &self,
        _buffer: *const c_void,
        values_len: u32,
    ) -> c_int {
        sym!(self, xcb_randr_configure_provider_property_sizeof)(_buffer, values_len)
    }

    /// Returns `true` iff the symbol `xcb_randr_configure_provider_property_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_configure_provider_property_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_configure_provider_property_sizeof)
    }

    /// Sends a `RandR::ConfigureProviderProperty` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_randr_configure_provider_property_checked(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        property: xcb_atom_t,
        pending: u8,
        range: u8,
        values_len: u32,
        values: *const i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_configure_provider_property_checked)(
            c, provider, property, pending, range, values_len, values,
        )
    }

    /// Returns `true` iff the symbol `xcb_randr_configure_provider_property_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_configure_provider_property_checked(&self) -> bool {
        has_sym!(self, xcb_randr_configure_provider_property_checked)
    }

    /// Sends a `RandR::ConfigureProviderProperty` request (unchecked).
    #[inline]
    pub unsafe fn xcb_randr_configure_provider_property(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        property: xcb_atom_t,
        pending: u8,
        range: u8,
        values_len: u32,
        values: *const i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_configure_provider_property)(
            c, provider, property, pending, range, values_len, values,
        )
    }

    /// Returns `true` iff the symbol `xcb_randr_configure_provider_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_configure_provider_property(&self) -> bool {
        has_sym!(self, xcb_randr_configure_provider_property)
    }

    /// Returns a pointer to the `values` field of a `xcb_randr_configure_provider_property_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_configure_provider_property_values(
        &self,
        r: *const xcb_randr_configure_provider_property_request_t,
    ) -> *mut i32 {
        sym!(self, xcb_randr_configure_provider_property_values)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_configure_provider_property_values` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_configure_provider_property_values(&self) -> bool {
        has_sym!(self, xcb_randr_configure_provider_property_values)
    }

    /// Returns the number of elements of the `values` field of a `xcb_randr_configure_provider_property_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_configure_provider_property_values_length(
        &self,
        r: *const xcb_randr_configure_provider_property_request_t,
    ) -> c_int {
        sym!(self, xcb_randr_configure_provider_property_values_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_configure_provider_property_values_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_configure_provider_property_values_length(&self) -> bool {
        has_sym!(self, xcb_randr_configure_provider_property_values_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `values` field of a `xcb_randr_configure_provider_property_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_configure_provider_property_values_end(
        &self,
        r: *const xcb_randr_configure_provider_property_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_configure_provider_property_values_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_configure_provider_property_values_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_configure_provider_property_values_end(&self) -> bool {
        has_sym!(self, xcb_randr_configure_provider_property_values_end)
    }

    /// Computes the size of a `xcb_randr_change_provider_property_request_t` object.
    #[inline]
    pub unsafe fn xcb_randr_change_provider_property_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_randr_change_provider_property_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_randr_change_provider_property_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_change_provider_property_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_change_provider_property_sizeof)
    }

    /// Sends a `RandR::ChangeProviderProperty` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_randr_change_provider_property_checked(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        format: u8,
        mode: u8,
        num_items: u32,
        data: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_change_provider_property_checked)(
            c, provider, property, type_, format, mode, num_items, data,
        )
    }

    /// Returns `true` iff the symbol `xcb_randr_change_provider_property_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_change_provider_property_checked(&self) -> bool {
        has_sym!(self, xcb_randr_change_provider_property_checked)
    }

    /// Sends a `RandR::ChangeProviderProperty` request (unchecked).
    #[inline]
    pub unsafe fn xcb_randr_change_provider_property(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        format: u8,
        mode: u8,
        num_items: u32,
        data: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_change_provider_property)(
            c, provider, property, type_, format, mode, num_items, data,
        )
    }

    /// Returns `true` iff the symbol `xcb_randr_change_provider_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_change_provider_property(&self) -> bool {
        has_sym!(self, xcb_randr_change_provider_property)
    }

    /// Returns a pointer to the `data` field of a `xcb_randr_change_provider_property_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_change_provider_property_data(
        &self,
        r: *const xcb_randr_change_provider_property_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_randr_change_provider_property_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_change_provider_property_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_change_provider_property_data(&self) -> bool {
        has_sym!(self, xcb_randr_change_provider_property_data)
    }

    /// Returns the number of elements of the `data` field of a `xcb_randr_change_provider_property_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_change_provider_property_data_length(
        &self,
        r: *const xcb_randr_change_provider_property_request_t,
    ) -> c_int {
        sym!(self, xcb_randr_change_provider_property_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_change_provider_property_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_change_provider_property_data_length(&self) -> bool {
        has_sym!(self, xcb_randr_change_provider_property_data_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `data` field of a `xcb_randr_change_provider_property_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_change_provider_property_data_end(
        &self,
        r: *const xcb_randr_change_provider_property_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_change_provider_property_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_change_provider_property_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_change_provider_property_data_end(&self) -> bool {
        has_sym!(self, xcb_randr_change_provider_property_data_end)
    }

    /// Sends a `RandR::DeleteProviderProperty` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_randr_delete_provider_property_checked(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        property: xcb_atom_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_delete_provider_property_checked)(c, provider, property)
    }

    /// Returns `true` iff the symbol `xcb_randr_delete_provider_property_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_delete_provider_property_checked(&self) -> bool {
        has_sym!(self, xcb_randr_delete_provider_property_checked)
    }

    /// Sends a `RandR::DeleteProviderProperty` request (unchecked).
    #[inline]
    pub unsafe fn xcb_randr_delete_provider_property(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        property: xcb_atom_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_delete_provider_property)(c, provider, property)
    }

    /// Returns `true` iff the symbol `xcb_randr_delete_provider_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_delete_provider_property(&self) -> bool {
        has_sym!(self, xcb_randr_delete_provider_property)
    }

    /// Computes the size of a `xcb_randr_get_provider_property_reply_t` object.
    #[inline]
    pub unsafe fn xcb_randr_get_provider_property_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_randr_get_provider_property_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_property_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_property_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_get_provider_property_sizeof)
    }

    /// Sends a `RandR::GetProviderProperty` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_provider_property_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_provider_property_reply`]: Self::xcb_randr_get_provider_property_reply
    #[inline]
    pub unsafe fn xcb_randr_get_provider_property(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        long_offset: u32,
        long_length: u32,
        delete: u8,
        pending: u8,
    ) -> xcb_randr_get_provider_property_cookie_t {
        sym!(self, xcb_randr_get_provider_property)(
            c,
            provider,
            property,
            type_,
            long_offset,
            long_length,
            delete,
            pending,
        )
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_property(&self) -> bool {
        has_sym!(self, xcb_randr_get_provider_property)
    }

    /// Sends a `RandR::GetProviderProperty` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_provider_property_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_provider_property_reply`]: Self::xcb_randr_get_provider_property_reply
    #[inline]
    pub unsafe fn xcb_randr_get_provider_property_unchecked(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        long_offset: u32,
        long_length: u32,
        delete: u8,
        pending: u8,
    ) -> xcb_randr_get_provider_property_cookie_t {
        sym!(self, xcb_randr_get_provider_property_unchecked)(
            c,
            provider,
            property,
            type_,
            long_offset,
            long_length,
            delete,
            pending,
        )
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_property_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_property_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_get_provider_property_unchecked)
    }

    /// Returns a pointer to the `data` field of a `xcb_randr_get_provider_property_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_provider_property_data(
        &self,
        r: *const xcb_randr_get_provider_property_reply_t,
    ) -> *mut c_void {
        sym!(self, xcb_randr_get_provider_property_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_property_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_property_data(&self) -> bool {
        has_sym!(self, xcb_randr_get_provider_property_data)
    }

    /// Returns the number of elements of the `data` field of a `xcb_randr_get_provider_property_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_provider_property_data_length(
        &self,
        r: *const xcb_randr_get_provider_property_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_provider_property_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_property_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_property_data_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_provider_property_data_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `data` field of a `xcb_randr_get_provider_property_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_provider_property_data_end(
        &self,
        r: *const xcb_randr_get_provider_property_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_provider_property_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_property_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_property_data_end(&self) -> bool {
        has_sym!(self, xcb_randr_get_provider_property_data_end)
    }

    /// Waits for the reply to a `RandR::GetProviderProperty` request.
    #[inline]
    pub unsafe fn xcb_randr_get_provider_property_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_provider_property_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_provider_property_reply_t {
        sym!(self, xcb_randr_get_provider_property_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_provider_property_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_provider_property_reply(&self) -> bool {
        has_sym!(self, xcb_randr_get_provider_property_reply)
    }

    /// Advances a `xcb_randr_crtc_change_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_randr_crtc_change_next(&self, i: *mut xcb_randr_crtc_change_iterator_t) {
        sym!(self, xcb_randr_crtc_change_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_crtc_change_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_crtc_change_next(&self) -> bool {
        has_sym!(self, xcb_randr_crtc_change_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_randr_crtc_change_iterator_t`.
    #[inline]
    pub unsafe fn xcb_randr_crtc_change_end(
        &self,
        i: xcb_randr_crtc_change_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_crtc_change_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_crtc_change_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_crtc_change_end(&self) -> bool {
        has_sym!(self, xcb_randr_crtc_change_end)
    }

    /// Advances a `xcb_randr_output_change_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_randr_output_change_next(&self, i: *mut xcb_randr_output_change_iterator_t) {
        sym!(self, xcb_randr_output_change_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_output_change_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_output_change_next(&self) -> bool {
        has_sym!(self, xcb_randr_output_change_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_randr_output_change_iterator_t`.
    #[inline]
    pub unsafe fn xcb_randr_output_change_end(
        &self,
        i: xcb_randr_output_change_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_output_change_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_output_change_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_output_change_end(&self) -> bool {
        has_sym!(self, xcb_randr_output_change_end)
    }

    /// Advances a `xcb_randr_output_property_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_randr_output_property_next(
        &self,
        i: *mut xcb_randr_output_property_iterator_t,
    ) {
        sym!(self, xcb_randr_output_property_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_output_property_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_output_property_next(&self) -> bool {
        has_sym!(self, xcb_randr_output_property_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_randr_output_property_iterator_t`.
    #[inline]
    pub unsafe fn xcb_randr_output_property_end(
        &self,
        i: xcb_randr_output_property_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_output_property_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_output_property_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_output_property_end(&self) -> bool {
        has_sym!(self, xcb_randr_output_property_end)
    }

    /// Advances a `xcb_randr_provider_change_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_randr_provider_change_next(
        &self,
        i: *mut xcb_randr_provider_change_iterator_t,
    ) {
        sym!(self, xcb_randr_provider_change_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_provider_change_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_provider_change_next(&self) -> bool {
        has_sym!(self, xcb_randr_provider_change_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_randr_provider_change_iterator_t`.
    #[inline]
    pub unsafe fn xcb_randr_provider_change_end(
        &self,
        i: xcb_randr_provider_change_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_provider_change_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_provider_change_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_provider_change_end(&self) -> bool {
        has_sym!(self, xcb_randr_provider_change_end)
    }

    /// Advances a `xcb_randr_provider_property_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_randr_provider_property_next(
        &self,
        i: *mut xcb_randr_provider_property_iterator_t,
    ) {
        sym!(self, xcb_randr_provider_property_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_provider_property_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_provider_property_next(&self) -> bool {
        has_sym!(self, xcb_randr_provider_property_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_randr_provider_property_iterator_t`.
    #[inline]
    pub unsafe fn xcb_randr_provider_property_end(
        &self,
        i: xcb_randr_provider_property_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_provider_property_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_provider_property_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_provider_property_end(&self) -> bool {
        has_sym!(self, xcb_randr_provider_property_end)
    }

    /// Advances a `xcb_randr_resource_change_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_randr_resource_change_next(
        &self,
        i: *mut xcb_randr_resource_change_iterator_t,
    ) {
        sym!(self, xcb_randr_resource_change_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_resource_change_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_resource_change_next(&self) -> bool {
        has_sym!(self, xcb_randr_resource_change_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_randr_resource_change_iterator_t`.
    #[inline]
    pub unsafe fn xcb_randr_resource_change_end(
        &self,
        i: xcb_randr_resource_change_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_resource_change_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_resource_change_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_resource_change_end(&self) -> bool {
        has_sym!(self, xcb_randr_resource_change_end)
    }

    /// Computes the size of a `xcb_randr_monitor_info_t` object.
    #[inline]
    pub unsafe fn xcb_randr_monitor_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_randr_monitor_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_randr_monitor_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_monitor_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_monitor_info_sizeof)
    }

    /// Returns a pointer to the `outputs` field of a `xcb_randr_monitor_info_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_monitor_info_outputs(
        &self,
        r: *const xcb_randr_monitor_info_t,
    ) -> *mut xcb_randr_output_t {
        sym!(self, xcb_randr_monitor_info_outputs)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_monitor_info_outputs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_monitor_info_outputs(&self) -> bool {
        has_sym!(self, xcb_randr_monitor_info_outputs)
    }

    /// Returns the number of elements of the `outputs` field of a `xcb_randr_monitor_info_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_monitor_info_outputs_length(
        &self,
        r: *const xcb_randr_monitor_info_t,
    ) -> c_int {
        sym!(self, xcb_randr_monitor_info_outputs_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_monitor_info_outputs_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_monitor_info_outputs_length(&self) -> bool {
        has_sym!(self, xcb_randr_monitor_info_outputs_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `outputs` field of a `xcb_randr_monitor_info_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_monitor_info_outputs_end(
        &self,
        r: *const xcb_randr_monitor_info_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_monitor_info_outputs_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_monitor_info_outputs_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_monitor_info_outputs_end(&self) -> bool {
        has_sym!(self, xcb_randr_monitor_info_outputs_end)
    }

    /// Advances a `xcb_randr_monitor_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_randr_monitor_info_next(&self, i: *mut xcb_randr_monitor_info_iterator_t) {
        sym!(self, xcb_randr_monitor_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_monitor_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_monitor_info_next(&self) -> bool {
        has_sym!(self, xcb_randr_monitor_info_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_randr_monitor_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_randr_monitor_info_end(
        &self,
        i: xcb_randr_monitor_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_monitor_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_monitor_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_monitor_info_end(&self) -> bool {
        has_sym!(self, xcb_randr_monitor_info_end)
    }

    /// Computes the size of a `xcb_randr_get_monitors_reply_t` object.
    #[inline]
    pub unsafe fn xcb_randr_get_monitors_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_randr_get_monitors_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_monitors_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_monitors_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_get_monitors_sizeof)
    }

    /// Sends a `RandR::GetMonitors` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_monitors_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_monitors_reply`]: Self::xcb_randr_get_monitors_reply
    #[inline]
    pub unsafe fn xcb_randr_get_monitors(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        get_active: u8,
    ) -> xcb_randr_get_monitors_cookie_t {
        sym!(self, xcb_randr_get_monitors)(c, window, get_active)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_monitors` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_monitors(&self) -> bool {
        has_sym!(self, xcb_randr_get_monitors)
    }

    /// Sends a `RandR::GetMonitors` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_get_monitors_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_get_monitors_reply`]: Self::xcb_randr_get_monitors_reply
    #[inline]
    pub unsafe fn xcb_randr_get_monitors_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        get_active: u8,
    ) -> xcb_randr_get_monitors_cookie_t {
        sym!(self, xcb_randr_get_monitors_unchecked)(c, window, get_active)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_monitors_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_monitors_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_get_monitors_unchecked)
    }

    /// Returns the number of elements of the `monitors` field of a `xcb_randr_get_monitors_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_monitors_monitors_length(
        &self,
        r: *const xcb_randr_get_monitors_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_monitors_monitors_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_monitors_monitors_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_monitors_monitors_length(&self) -> bool {
        has_sym!(self, xcb_randr_get_monitors_monitors_length)
    }

    /// Returns an iterator over the elements of the
    /// `monitors` field of a `xcb_randr_get_monitors_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_get_monitors_monitors_iterator(
        &self,
        r: *const xcb_randr_get_monitors_reply_t,
    ) -> xcb_randr_monitor_info_iterator_t {
        sym!(self, xcb_randr_get_monitors_monitors_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_monitors_monitors_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_monitors_monitors_iterator(&self) -> bool {
        has_sym!(self, xcb_randr_get_monitors_monitors_iterator)
    }

    /// Waits for the reply to a `RandR::GetMonitors` request.
    #[inline]
    pub unsafe fn xcb_randr_get_monitors_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_monitors_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_monitors_reply_t {
        sym!(self, xcb_randr_get_monitors_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_get_monitors_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_get_monitors_reply(&self) -> bool {
        has_sym!(self, xcb_randr_get_monitors_reply)
    }

    /// Computes the size of a `xcb_randr_set_monitor_request_t` object.
    #[inline]
    pub unsafe fn xcb_randr_set_monitor_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_randr_set_monitor_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_monitor_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_monitor_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_set_monitor_sizeof)
    }

    /// Sends a `RandR::SetMonitor` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_randr_set_monitor_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        monitorinfo: *mut xcb_randr_monitor_info_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_monitor_checked)(c, window, monitorinfo)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_monitor_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_monitor_checked(&self) -> bool {
        has_sym!(self, xcb_randr_set_monitor_checked)
    }

    /// Sends a `RandR::SetMonitor` request (unchecked).
    #[inline]
    pub unsafe fn xcb_randr_set_monitor(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        monitorinfo: *mut xcb_randr_monitor_info_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_monitor)(c, window, monitorinfo)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_monitor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_monitor(&self) -> bool {
        has_sym!(self, xcb_randr_set_monitor)
    }

    /// Returns a pointer to the `monitorinfo` field of a `xcb_randr_set_monitor_request_t` struct.
    #[inline]
    pub unsafe fn xcb_randr_set_monitor_monitorinfo(
        &self,
        r: *const xcb_randr_set_monitor_request_t,
    ) -> *mut xcb_randr_monitor_info_t {
        sym!(self, xcb_randr_set_monitor_monitorinfo)(r)
    }

    /// Returns `true` iff the symbol `xcb_randr_set_monitor_monitorinfo` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_set_monitor_monitorinfo(&self) -> bool {
        has_sym!(self, xcb_randr_set_monitor_monitorinfo)
    }

    /// Sends a `RandR::DeleteMonitor` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_randr_delete_monitor_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        name: xcb_atom_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_delete_monitor_checked)(c, window, name)
    }

    /// Returns `true` iff the symbol `xcb_randr_delete_monitor_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_delete_monitor_checked(&self) -> bool {
        has_sym!(self, xcb_randr_delete_monitor_checked)
    }

    /// Sends a `RandR::DeleteMonitor` request (unchecked).
    #[inline]
    pub unsafe fn xcb_randr_delete_monitor(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        name: xcb_atom_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_delete_monitor)(c, window, name)
    }

    /// Returns `true` iff the symbol `xcb_randr_delete_monitor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_delete_monitor(&self) -> bool {
        has_sym!(self, xcb_randr_delete_monitor)
    }

    /// Computes the size of a `xcb_randr_create_lease_request_t` object.
    #[inline]
    pub unsafe fn xcb_randr_create_lease_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_randr_create_lease_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_randr_create_lease_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_create_lease_sizeof(&self) -> bool {
        has_sym!(self, xcb_randr_create_lease_sizeof)
    }

    /// Sends a `RandR::CreateLease` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_create_lease_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_create_lease_reply`]: Self::xcb_randr_create_lease_reply
    #[inline]
    pub unsafe fn xcb_randr_create_lease(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        lid: xcb_randr_lease_t,
        num_crtcs: u16,
        num_outputs: u16,
        crtcs: *const xcb_randr_crtc_t,
        outputs: *const xcb_randr_output_t,
    ) -> xcb_randr_create_lease_cookie_t {
        sym!(self, xcb_randr_create_lease)(c, window, lid, num_crtcs, num_outputs, crtcs, outputs)
    }

    /// Returns `true` iff the symbol `xcb_randr_create_lease` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_create_lease(&self) -> bool {
        has_sym!(self, xcb_randr_create_lease)
    }

    /// Sends a `RandR::CreateLease` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_randr_create_lease_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_randr_create_lease_reply`]: Self::xcb_randr_create_lease_reply
    #[inline]
    pub unsafe fn xcb_randr_create_lease_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        lid: xcb_randr_lease_t,
        num_crtcs: u16,
        num_outputs: u16,
        crtcs: *const xcb_randr_crtc_t,
        outputs: *const xcb_randr_output_t,
    ) -> xcb_randr_create_lease_cookie_t {
        sym!(self, xcb_randr_create_lease_unchecked)(
            c,
            window,
            lid,
            num_crtcs,
            num_outputs,
            crtcs,
            outputs,
        )
    }

    /// Returns `true` iff the symbol `xcb_randr_create_lease_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_create_lease_unchecked(&self) -> bool {
        has_sym!(self, xcb_randr_create_lease_unchecked)
    }

    /// Waits for the reply to a `RandR::CreateLease` request.
    #[inline]
    pub unsafe fn xcb_randr_create_lease_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_create_lease_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_create_lease_reply_t {
        sym!(self, xcb_randr_create_lease_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_randr_create_lease_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_create_lease_reply(&self) -> bool {
        has_sym!(self, xcb_randr_create_lease_reply)
    }

    /// Retrieves the file descriptors from the reply to a `RandR::CreateLease` request.
    ///
    /// The returned pointer must be freed with `libc::free`.
    #[inline]
    pub unsafe fn xcb_randr_create_lease_reply_fds(
        &self,
        c: *mut xcb_connection_t,
        reply: *mut xcb_randr_create_lease_reply_t,
    ) -> *mut c_int {
        sym!(self, xcb_randr_create_lease_reply_fds)(c, reply)
    }

    /// Returns `true` iff the symbol `xcb_randr_create_lease_reply_fds` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_create_lease_reply_fds(&self) -> bool {
        has_sym!(self, xcb_randr_create_lease_reply_fds)
    }

    /// Sends a `RandR::FreeLease` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_randr_free_lease_checked(
        &self,
        c: *mut xcb_connection_t,
        lid: xcb_randr_lease_t,
        terminate: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_free_lease_checked)(c, lid, terminate)
    }

    /// Returns `true` iff the symbol `xcb_randr_free_lease_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_free_lease_checked(&self) -> bool {
        has_sym!(self, xcb_randr_free_lease_checked)
    }

    /// Sends a `RandR::FreeLease` request (unchecked).
    #[inline]
    pub unsafe fn xcb_randr_free_lease(
        &self,
        c: *mut xcb_connection_t,
        lid: xcb_randr_lease_t,
        terminate: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_free_lease)(c, lid, terminate)
    }

    /// Returns `true` iff the symbol `xcb_randr_free_lease` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_free_lease(&self) -> bool {
        has_sym!(self, xcb_randr_free_lease)
    }

    /// Advances a `xcb_randr_lease_notify_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_randr_lease_notify_next(&self, i: *mut xcb_randr_lease_notify_iterator_t) {
        sym!(self, xcb_randr_lease_notify_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_lease_notify_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_lease_notify_next(&self) -> bool {
        has_sym!(self, xcb_randr_lease_notify_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_randr_lease_notify_iterator_t`.
    #[inline]
    pub unsafe fn xcb_randr_lease_notify_end(
        &self,
        i: xcb_randr_lease_notify_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_lease_notify_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_lease_notify_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_lease_notify_end(&self) -> bool {
        has_sym!(self, xcb_randr_lease_notify_end)
    }

    /// Advances a `xcb_randr_notify_data_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_randr_notify_data_next(&self, i: *mut xcb_randr_notify_data_iterator_t) {
        sym!(self, xcb_randr_notify_data_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_notify_data_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_notify_data_next(&self) -> bool {
        has_sym!(self, xcb_randr_notify_data_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_randr_notify_data_iterator_t`.
    #[inline]
    pub unsafe fn xcb_randr_notify_data_end(
        &self,
        i: xcb_randr_notify_data_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_notify_data_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_randr_notify_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_randr_notify_data_end(&self) -> bool {
        has_sym!(self, xcb_randr_notify_data_end)
    }
}

#[cfg(feature = "xcb_randr")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbRandr::load().unwrap() };
        assert!(lib.has_xcb_randr_id());
        assert!(lib.has_xcb_randr_mode_next());
        assert!(lib.has_xcb_randr_mode_end());
        assert!(lib.has_xcb_randr_crtc_next());
        assert!(lib.has_xcb_randr_crtc_end());
        assert!(lib.has_xcb_randr_output_next());
        assert!(lib.has_xcb_randr_output_end());
        assert!(lib.has_xcb_randr_provider_next());
        assert!(lib.has_xcb_randr_provider_end());
        assert!(lib.has_xcb_randr_lease_next());
        assert!(lib.has_xcb_randr_lease_end());
        assert!(lib.has_xcb_randr_screen_size_next());
        assert!(lib.has_xcb_randr_screen_size_end());
        assert!(lib.has_xcb_randr_refresh_rates_sizeof());
        assert!(lib.has_xcb_randr_refresh_rates_rates());
        assert!(lib.has_xcb_randr_refresh_rates_rates_length());
        assert!(lib.has_xcb_randr_refresh_rates_rates_end());
        assert!(lib.has_xcb_randr_refresh_rates_next());
        assert!(lib.has_xcb_randr_refresh_rates_end());
        assert!(lib.has_xcb_randr_query_version());
        assert!(lib.has_xcb_randr_query_version_unchecked());
        assert!(lib.has_xcb_randr_query_version_reply());
        assert!(lib.has_xcb_randr_set_screen_config());
        assert!(lib.has_xcb_randr_set_screen_config_unchecked());
        assert!(lib.has_xcb_randr_set_screen_config_reply());
        assert!(lib.has_xcb_randr_select_input_checked());
        assert!(lib.has_xcb_randr_select_input());
        assert!(lib.has_xcb_randr_get_screen_info_sizeof());
        assert!(lib.has_xcb_randr_get_screen_info());
        assert!(lib.has_xcb_randr_get_screen_info_unchecked());
        assert!(lib.has_xcb_randr_get_screen_info_sizes());
        assert!(lib.has_xcb_randr_get_screen_info_sizes_length());
        assert!(lib.has_xcb_randr_get_screen_info_sizes_iterator());
        assert!(lib.has_xcb_randr_get_screen_info_rates_length());
        assert!(lib.has_xcb_randr_get_screen_info_rates_iterator());
        assert!(lib.has_xcb_randr_get_screen_info_reply());
        assert!(lib.has_xcb_randr_get_screen_size_range());
        assert!(lib.has_xcb_randr_get_screen_size_range_unchecked());
        assert!(lib.has_xcb_randr_get_screen_size_range_reply());
        assert!(lib.has_xcb_randr_set_screen_size_checked());
        assert!(lib.has_xcb_randr_set_screen_size());
        assert!(lib.has_xcb_randr_mode_info_next());
        assert!(lib.has_xcb_randr_mode_info_end());
        assert!(lib.has_xcb_randr_get_screen_resources_sizeof());
        assert!(lib.has_xcb_randr_get_screen_resources());
        assert!(lib.has_xcb_randr_get_screen_resources_unchecked());
        assert!(lib.has_xcb_randr_get_screen_resources_crtcs());
        assert!(lib.has_xcb_randr_get_screen_resources_crtcs_length());
        assert!(lib.has_xcb_randr_get_screen_resources_crtcs_end());
        assert!(lib.has_xcb_randr_get_screen_resources_outputs());
        assert!(lib.has_xcb_randr_get_screen_resources_outputs_length());
        assert!(lib.has_xcb_randr_get_screen_resources_outputs_end());
        assert!(lib.has_xcb_randr_get_screen_resources_modes());
        assert!(lib.has_xcb_randr_get_screen_resources_modes_length());
        assert!(lib.has_xcb_randr_get_screen_resources_modes_iterator());
        assert!(lib.has_xcb_randr_get_screen_resources_names());
        assert!(lib.has_xcb_randr_get_screen_resources_names_length());
        assert!(lib.has_xcb_randr_get_screen_resources_names_end());
        assert!(lib.has_xcb_randr_get_screen_resources_reply());
        assert!(lib.has_xcb_randr_get_output_info_sizeof());
        assert!(lib.has_xcb_randr_get_output_info());
        assert!(lib.has_xcb_randr_get_output_info_unchecked());
        assert!(lib.has_xcb_randr_get_output_info_crtcs());
        assert!(lib.has_xcb_randr_get_output_info_crtcs_length());
        assert!(lib.has_xcb_randr_get_output_info_crtcs_end());
        assert!(lib.has_xcb_randr_get_output_info_modes());
        assert!(lib.has_xcb_randr_get_output_info_modes_length());
        assert!(lib.has_xcb_randr_get_output_info_modes_end());
        assert!(lib.has_xcb_randr_get_output_info_clones());
        assert!(lib.has_xcb_randr_get_output_info_clones_length());
        assert!(lib.has_xcb_randr_get_output_info_clones_end());
        assert!(lib.has_xcb_randr_get_output_info_name());
        assert!(lib.has_xcb_randr_get_output_info_name_length());
        assert!(lib.has_xcb_randr_get_output_info_name_end());
        assert!(lib.has_xcb_randr_get_output_info_reply());
        assert!(lib.has_xcb_randr_list_output_properties_sizeof());
        assert!(lib.has_xcb_randr_list_output_properties());
        assert!(lib.has_xcb_randr_list_output_properties_unchecked());
        assert!(lib.has_xcb_randr_list_output_properties_atoms());
        assert!(lib.has_xcb_randr_list_output_properties_atoms_length());
        assert!(lib.has_xcb_randr_list_output_properties_atoms_end());
        assert!(lib.has_xcb_randr_list_output_properties_reply());
        assert!(lib.has_xcb_randr_query_output_property_sizeof());
        assert!(lib.has_xcb_randr_query_output_property());
        assert!(lib.has_xcb_randr_query_output_property_unchecked());
        assert!(lib.has_xcb_randr_query_output_property_valid_values());
        assert!(lib.has_xcb_randr_query_output_property_valid_values_length());
        assert!(lib.has_xcb_randr_query_output_property_valid_values_end());
        assert!(lib.has_xcb_randr_query_output_property_reply());
        assert!(lib.has_xcb_randr_configure_output_property_sizeof());
        assert!(lib.has_xcb_randr_configure_output_property_checked());
        assert!(lib.has_xcb_randr_configure_output_property());
        assert!(lib.has_xcb_randr_configure_output_property_values());
        assert!(lib.has_xcb_randr_configure_output_property_values_length());
        assert!(lib.has_xcb_randr_configure_output_property_values_end());
        assert!(lib.has_xcb_randr_change_output_property_sizeof());
        assert!(lib.has_xcb_randr_change_output_property_checked());
        assert!(lib.has_xcb_randr_change_output_property());
        assert!(lib.has_xcb_randr_change_output_property_data());
        assert!(lib.has_xcb_randr_change_output_property_data_length());
        assert!(lib.has_xcb_randr_change_output_property_data_end());
        assert!(lib.has_xcb_randr_delete_output_property_checked());
        assert!(lib.has_xcb_randr_delete_output_property());
        assert!(lib.has_xcb_randr_get_output_property_sizeof());
        assert!(lib.has_xcb_randr_get_output_property());
        assert!(lib.has_xcb_randr_get_output_property_unchecked());
        assert!(lib.has_xcb_randr_get_output_property_data());
        assert!(lib.has_xcb_randr_get_output_property_data_length());
        assert!(lib.has_xcb_randr_get_output_property_data_end());
        assert!(lib.has_xcb_randr_get_output_property_reply());
        assert!(lib.has_xcb_randr_create_mode_sizeof());
        assert!(lib.has_xcb_randr_create_mode());
        assert!(lib.has_xcb_randr_create_mode_unchecked());
        assert!(lib.has_xcb_randr_create_mode_reply());
        assert!(lib.has_xcb_randr_destroy_mode_checked());
        assert!(lib.has_xcb_randr_destroy_mode());
        assert!(lib.has_xcb_randr_add_output_mode_checked());
        assert!(lib.has_xcb_randr_add_output_mode());
        assert!(lib.has_xcb_randr_delete_output_mode_checked());
        assert!(lib.has_xcb_randr_delete_output_mode());
        assert!(lib.has_xcb_randr_get_crtc_info_sizeof());
        assert!(lib.has_xcb_randr_get_crtc_info());
        assert!(lib.has_xcb_randr_get_crtc_info_unchecked());
        assert!(lib.has_xcb_randr_get_crtc_info_outputs());
        assert!(lib.has_xcb_randr_get_crtc_info_outputs_length());
        assert!(lib.has_xcb_randr_get_crtc_info_outputs_end());
        assert!(lib.has_xcb_randr_get_crtc_info_possible());
        assert!(lib.has_xcb_randr_get_crtc_info_possible_length());
        assert!(lib.has_xcb_randr_get_crtc_info_possible_end());
        assert!(lib.has_xcb_randr_get_crtc_info_reply());
        assert!(lib.has_xcb_randr_set_crtc_config_sizeof());
        assert!(lib.has_xcb_randr_set_crtc_config());
        assert!(lib.has_xcb_randr_set_crtc_config_unchecked());
        assert!(lib.has_xcb_randr_set_crtc_config_reply());
        assert!(lib.has_xcb_randr_get_crtc_gamma_size());
        assert!(lib.has_xcb_randr_get_crtc_gamma_size_unchecked());
        assert!(lib.has_xcb_randr_get_crtc_gamma_size_reply());
        assert!(lib.has_xcb_randr_get_crtc_gamma_sizeof());
        assert!(lib.has_xcb_randr_get_crtc_gamma());
        assert!(lib.has_xcb_randr_get_crtc_gamma_unchecked());
        assert!(lib.has_xcb_randr_get_crtc_gamma_red());
        assert!(lib.has_xcb_randr_get_crtc_gamma_red_length());
        assert!(lib.has_xcb_randr_get_crtc_gamma_red_end());
        assert!(lib.has_xcb_randr_get_crtc_gamma_green());
        assert!(lib.has_xcb_randr_get_crtc_gamma_green_length());
        assert!(lib.has_xcb_randr_get_crtc_gamma_green_end());
        assert!(lib.has_xcb_randr_get_crtc_gamma_blue());
        assert!(lib.has_xcb_randr_get_crtc_gamma_blue_length());
        assert!(lib.has_xcb_randr_get_crtc_gamma_blue_end());
        assert!(lib.has_xcb_randr_get_crtc_gamma_reply());
        assert!(lib.has_xcb_randr_set_crtc_gamma_sizeof());
        assert!(lib.has_xcb_randr_set_crtc_gamma_checked());
        assert!(lib.has_xcb_randr_set_crtc_gamma());
        assert!(lib.has_xcb_randr_set_crtc_gamma_red());
        assert!(lib.has_xcb_randr_set_crtc_gamma_red_length());
        assert!(lib.has_xcb_randr_set_crtc_gamma_red_end());
        assert!(lib.has_xcb_randr_set_crtc_gamma_green());
        assert!(lib.has_xcb_randr_set_crtc_gamma_green_length());
        assert!(lib.has_xcb_randr_set_crtc_gamma_green_end());
        assert!(lib.has_xcb_randr_set_crtc_gamma_blue());
        assert!(lib.has_xcb_randr_set_crtc_gamma_blue_length());
        assert!(lib.has_xcb_randr_set_crtc_gamma_blue_end());
        assert!(lib.has_xcb_randr_get_screen_resources_current_sizeof());
        assert!(lib.has_xcb_randr_get_screen_resources_current());
        assert!(lib.has_xcb_randr_get_screen_resources_current_unchecked());
        assert!(lib.has_xcb_randr_get_screen_resources_current_crtcs());
        assert!(lib.has_xcb_randr_get_screen_resources_current_crtcs_length());
        assert!(lib.has_xcb_randr_get_screen_resources_current_crtcs_end());
        assert!(lib.has_xcb_randr_get_screen_resources_current_outputs());
        assert!(lib.has_xcb_randr_get_screen_resources_current_outputs_length());
        assert!(lib.has_xcb_randr_get_screen_resources_current_outputs_end());
        assert!(lib.has_xcb_randr_get_screen_resources_current_modes());
        assert!(lib.has_xcb_randr_get_screen_resources_current_modes_length());
        assert!(lib.has_xcb_randr_get_screen_resources_current_modes_iterator());
        assert!(lib.has_xcb_randr_get_screen_resources_current_names());
        assert!(lib.has_xcb_randr_get_screen_resources_current_names_length());
        assert!(lib.has_xcb_randr_get_screen_resources_current_names_end());
        assert!(lib.has_xcb_randr_get_screen_resources_current_reply());
        assert!(lib.has_xcb_randr_set_crtc_transform_sizeof());
        assert!(lib.has_xcb_randr_set_crtc_transform_checked());
        assert!(lib.has_xcb_randr_set_crtc_transform());
        assert!(lib.has_xcb_randr_set_crtc_transform_filter_name());
        assert!(lib.has_xcb_randr_set_crtc_transform_filter_name_length());
        assert!(lib.has_xcb_randr_set_crtc_transform_filter_name_end());
        assert!(lib.has_xcb_randr_set_crtc_transform_filter_params());
        assert!(lib.has_xcb_randr_set_crtc_transform_filter_params_length());
        assert!(lib.has_xcb_randr_set_crtc_transform_filter_params_end());
        assert!(lib.has_xcb_randr_get_crtc_transform_sizeof());
        assert!(lib.has_xcb_randr_get_crtc_transform());
        assert!(lib.has_xcb_randr_get_crtc_transform_unchecked());
        assert!(lib.has_xcb_randr_get_crtc_transform_pending_filter_name());
        assert!(lib.has_xcb_randr_get_crtc_transform_pending_filter_name_length());
        assert!(lib.has_xcb_randr_get_crtc_transform_pending_filter_name_end());
        assert!(lib.has_xcb_randr_get_crtc_transform_pending_params());
        assert!(lib.has_xcb_randr_get_crtc_transform_pending_params_length());
        assert!(lib.has_xcb_randr_get_crtc_transform_pending_params_end());
        assert!(lib.has_xcb_randr_get_crtc_transform_current_filter_name());
        assert!(lib.has_xcb_randr_get_crtc_transform_current_filter_name_length());
        assert!(lib.has_xcb_randr_get_crtc_transform_current_filter_name_end());
        assert!(lib.has_xcb_randr_get_crtc_transform_current_params());
        assert!(lib.has_xcb_randr_get_crtc_transform_current_params_length());
        assert!(lib.has_xcb_randr_get_crtc_transform_current_params_end());
        assert!(lib.has_xcb_randr_get_crtc_transform_reply());
        assert!(lib.has_xcb_randr_get_panning());
        assert!(lib.has_xcb_randr_get_panning_unchecked());
        assert!(lib.has_xcb_randr_get_panning_reply());
        assert!(lib.has_xcb_randr_set_panning());
        assert!(lib.has_xcb_randr_set_panning_unchecked());
        assert!(lib.has_xcb_randr_set_panning_reply());
        assert!(lib.has_xcb_randr_set_output_primary_checked());
        assert!(lib.has_xcb_randr_set_output_primary());
        assert!(lib.has_xcb_randr_get_output_primary());
        assert!(lib.has_xcb_randr_get_output_primary_unchecked());
        assert!(lib.has_xcb_randr_get_output_primary_reply());
        assert!(lib.has_xcb_randr_get_providers_sizeof());
        assert!(lib.has_xcb_randr_get_providers());
        assert!(lib.has_xcb_randr_get_providers_unchecked());
        assert!(lib.has_xcb_randr_get_providers_providers());
        assert!(lib.has_xcb_randr_get_providers_providers_length());
        assert!(lib.has_xcb_randr_get_providers_providers_end());
        assert!(lib.has_xcb_randr_get_providers_reply());
        assert!(lib.has_xcb_randr_get_provider_info_sizeof());
        assert!(lib.has_xcb_randr_get_provider_info());
        assert!(lib.has_xcb_randr_get_provider_info_unchecked());
        assert!(lib.has_xcb_randr_get_provider_info_crtcs());
        assert!(lib.has_xcb_randr_get_provider_info_crtcs_length());
        assert!(lib.has_xcb_randr_get_provider_info_crtcs_end());
        assert!(lib.has_xcb_randr_get_provider_info_outputs());
        assert!(lib.has_xcb_randr_get_provider_info_outputs_length());
        assert!(lib.has_xcb_randr_get_provider_info_outputs_end());
        assert!(lib.has_xcb_randr_get_provider_info_associated_providers());
        assert!(lib.has_xcb_randr_get_provider_info_associated_providers_length());
        assert!(lib.has_xcb_randr_get_provider_info_associated_providers_end());
        assert!(lib.has_xcb_randr_get_provider_info_associated_capability());
        assert!(lib.has_xcb_randr_get_provider_info_associated_capability_length());
        assert!(lib.has_xcb_randr_get_provider_info_associated_capability_end());
        assert!(lib.has_xcb_randr_get_provider_info_name());
        assert!(lib.has_xcb_randr_get_provider_info_name_length());
        assert!(lib.has_xcb_randr_get_provider_info_name_end());
        assert!(lib.has_xcb_randr_get_provider_info_reply());
        assert!(lib.has_xcb_randr_set_provider_offload_sink_checked());
        assert!(lib.has_xcb_randr_set_provider_offload_sink());
        assert!(lib.has_xcb_randr_set_provider_output_source_checked());
        assert!(lib.has_xcb_randr_set_provider_output_source());
        assert!(lib.has_xcb_randr_list_provider_properties_sizeof());
        assert!(lib.has_xcb_randr_list_provider_properties());
        assert!(lib.has_xcb_randr_list_provider_properties_unchecked());
        assert!(lib.has_xcb_randr_list_provider_properties_atoms());
        assert!(lib.has_xcb_randr_list_provider_properties_atoms_length());
        assert!(lib.has_xcb_randr_list_provider_properties_atoms_end());
        assert!(lib.has_xcb_randr_list_provider_properties_reply());
        assert!(lib.has_xcb_randr_query_provider_property_sizeof());
        assert!(lib.has_xcb_randr_query_provider_property());
        assert!(lib.has_xcb_randr_query_provider_property_unchecked());
        assert!(lib.has_xcb_randr_query_provider_property_valid_values());
        assert!(lib.has_xcb_randr_query_provider_property_valid_values_length());
        assert!(lib.has_xcb_randr_query_provider_property_valid_values_end());
        assert!(lib.has_xcb_randr_query_provider_property_reply());
        assert!(lib.has_xcb_randr_configure_provider_property_sizeof());
        assert!(lib.has_xcb_randr_configure_provider_property_checked());
        assert!(lib.has_xcb_randr_configure_provider_property());
        assert!(lib.has_xcb_randr_configure_provider_property_values());
        assert!(lib.has_xcb_randr_configure_provider_property_values_length());
        assert!(lib.has_xcb_randr_configure_provider_property_values_end());
        assert!(lib.has_xcb_randr_change_provider_property_sizeof());
        assert!(lib.has_xcb_randr_change_provider_property_checked());
        assert!(lib.has_xcb_randr_change_provider_property());
        assert!(lib.has_xcb_randr_change_provider_property_data());
        assert!(lib.has_xcb_randr_change_provider_property_data_length());
        assert!(lib.has_xcb_randr_change_provider_property_data_end());
        assert!(lib.has_xcb_randr_delete_provider_property_checked());
        assert!(lib.has_xcb_randr_delete_provider_property());
        assert!(lib.has_xcb_randr_get_provider_property_sizeof());
        assert!(lib.has_xcb_randr_get_provider_property());
        assert!(lib.has_xcb_randr_get_provider_property_unchecked());
        assert!(lib.has_xcb_randr_get_provider_property_data());
        assert!(lib.has_xcb_randr_get_provider_property_data_length());
        assert!(lib.has_xcb_randr_get_provider_property_data_end());
        assert!(lib.has_xcb_randr_get_provider_property_reply());
        assert!(lib.has_xcb_randr_crtc_change_next());
        assert!(lib.has_xcb_randr_crtc_change_end());
        assert!(lib.has_xcb_randr_output_change_next());
        assert!(lib.has_xcb_randr_output_change_end());
        assert!(lib.has_xcb_randr_output_property_next());
        assert!(lib.has_xcb_randr_output_property_end());
        assert!(lib.has_xcb_randr_provider_change_next());
        assert!(lib.has_xcb_randr_provider_change_end());
        assert!(lib.has_xcb_randr_provider_property_next());
        assert!(lib.has_xcb_randr_provider_property_end());
        assert!(lib.has_xcb_randr_resource_change_next());
        assert!(lib.has_xcb_randr_resource_change_end());
        assert!(lib.has_xcb_randr_monitor_info_sizeof());
        assert!(lib.has_xcb_randr_monitor_info_outputs());
        assert!(lib.has_xcb_randr_monitor_info_outputs_length());
        assert!(lib.has_xcb_randr_monitor_info_outputs_end());
        assert!(lib.has_xcb_randr_monitor_info_next());
        assert!(lib.has_xcb_randr_monitor_info_end());
        assert!(lib.has_xcb_randr_get_monitors_sizeof());
        assert!(lib.has_xcb_randr_get_monitors());
        assert!(lib.has_xcb_randr_get_monitors_unchecked());
        assert!(lib.has_xcb_randr_get_monitors_monitors_length());
        assert!(lib.has_xcb_randr_get_monitors_monitors_iterator());
        assert!(lib.has_xcb_randr_get_monitors_reply());
        assert!(lib.has_xcb_randr_set_monitor_sizeof());
        assert!(lib.has_xcb_randr_set_monitor_checked());
        assert!(lib.has_xcb_randr_set_monitor());
        assert!(lib.has_xcb_randr_set_monitor_monitorinfo());
        assert!(lib.has_xcb_randr_delete_monitor_checked());
        assert!(lib.has_xcb_randr_delete_monitor());
        assert!(lib.has_xcb_randr_create_lease_sizeof());
        assert!(lib.has_xcb_randr_create_lease());
        assert!(lib.has_xcb_randr_create_lease_unchecked());
        assert!(lib.has_xcb_randr_create_lease_reply());
        assert!(lib.has_xcb_randr_create_lease_reply_fds());
        assert!(lib.has_xcb_randr_free_lease_checked());
        assert!(lib.has_xcb_randr_free_lease());
        assert!(lib.has_xcb_randr_lease_notify_next());
        assert!(lib.has_xcb_randr_lease_notify_end());
        assert!(lib.has_xcb_randr_notify_data_next());
        assert!(lib.has_xcb_randr_notify_data_end());
    }
}
