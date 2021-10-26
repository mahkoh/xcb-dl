// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The cookie for the reply to a `SELinux::QueryVersion` request.
///
/// Pass this cookie to [`xcb_selinux_query_version_reply`] to retrieve the reply.
///
/// [`xcb_selinux_query_version_reply`]: XcbXselinux::xcb_selinux_query_version_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_query_version_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_selinux_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SELinux::QueryVersion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXselinux::xcb_selinux_id()`], then the type of the request is
/// [`xcb_selinux_query_version_request_t`].
pub const XCB_SELINUX_QUERY_VERSION: u8 = 0i32 as u8;

/// The `SELinux::QueryVersion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client_major: u8,
    pub client_minor: u8,
}

impl Default for xcb_selinux_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `SELinux::QueryVersion` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major: u16,
    pub server_minor: u16,
}

impl Default for xcb_selinux_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SELinux::SetDeviceCreateContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXselinux::xcb_selinux_id()`], then the type of the request is
/// [`xcb_selinux_set_device_create_context_request_t`].
pub const XCB_SELINUX_SET_DEVICE_CREATE_CONTEXT: u8 = 1i32 as u8;

/// The `SELinux::SetDeviceCreateContext` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `context`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_set_device_create_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_len: u32,
}

impl Default for xcb_selinux_set_device_create_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `SELinux::GetDeviceCreateContext` request.
///
/// Pass this cookie to [`xcb_selinux_get_device_create_context_reply`] to retrieve the reply.
///
/// [`xcb_selinux_get_device_create_context_reply`]: XcbXselinux::xcb_selinux_get_device_create_context_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_device_create_context_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_device_create_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SELinux::GetDeviceCreateContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXselinux::xcb_selinux_id()`], then the type of the request is
/// [`xcb_selinux_get_device_create_context_request_t`].
pub const XCB_SELINUX_GET_DEVICE_CREATE_CONTEXT: u8 = 2i32 as u8;

/// The `SELinux::GetDeviceCreateContext` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_device_create_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_selinux_get_device_create_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `SELinux::GetDeviceCreateContext` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `context`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_device_create_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_selinux_get_device_create_context_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SELinux::SetDeviceContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXselinux::xcb_selinux_id()`], then the type of the request is
/// [`xcb_selinux_set_device_context_request_t`].
pub const XCB_SELINUX_SET_DEVICE_CONTEXT: u8 = 3i32 as u8;

/// The `SELinux::SetDeviceContext` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `context`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_set_device_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device: u32,
    pub context_len: u32,
}

impl Default for xcb_selinux_set_device_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `SELinux::GetDeviceContext` request.
///
/// Pass this cookie to [`xcb_selinux_get_device_context_reply`] to retrieve the reply.
///
/// [`xcb_selinux_get_device_context_reply`]: XcbXselinux::xcb_selinux_get_device_context_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_device_context_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_device_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SELinux::GetDeviceContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXselinux::xcb_selinux_id()`], then the type of the request is
/// [`xcb_selinux_get_device_context_request_t`].
pub const XCB_SELINUX_GET_DEVICE_CONTEXT: u8 = 4i32 as u8;

/// The `SELinux::GetDeviceContext` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_device_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device: u32,
}

impl Default for xcb_selinux_get_device_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `SELinux::GetDeviceContext` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `context`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_device_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_selinux_get_device_context_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SELinux::SetWindowCreateContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXselinux::xcb_selinux_id()`], then the type of the request is
/// [`xcb_selinux_set_window_create_context_request_t`].
pub const XCB_SELINUX_SET_WINDOW_CREATE_CONTEXT: u8 = 5i32 as u8;

/// The `SELinux::SetWindowCreateContext` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `context`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_set_window_create_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_len: u32,
}

impl Default for xcb_selinux_set_window_create_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `SELinux::GetWindowCreateContext` request.
///
/// Pass this cookie to [`xcb_selinux_get_window_create_context_reply`] to retrieve the reply.
///
/// [`xcb_selinux_get_window_create_context_reply`]: XcbXselinux::xcb_selinux_get_window_create_context_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_window_create_context_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_window_create_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SELinux::GetWindowCreateContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXselinux::xcb_selinux_id()`], then the type of the request is
/// [`xcb_selinux_get_window_create_context_request_t`].
pub const XCB_SELINUX_GET_WINDOW_CREATE_CONTEXT: u8 = 6i32 as u8;

/// The `SELinux::GetWindowCreateContext` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_window_create_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_selinux_get_window_create_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `SELinux::GetWindowCreateContext` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `context`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_window_create_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_selinux_get_window_create_context_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `SELinux::GetWindowContext` request.
///
/// Pass this cookie to [`xcb_selinux_get_window_context_reply`] to retrieve the reply.
///
/// [`xcb_selinux_get_window_context_reply`]: XcbXselinux::xcb_selinux_get_window_context_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_window_context_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_window_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SELinux::GetWindowContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXselinux::xcb_selinux_id()`], then the type of the request is
/// [`xcb_selinux_get_window_context_request_t`].
pub const XCB_SELINUX_GET_WINDOW_CONTEXT: u8 = 7i32 as u8;

/// The `SELinux::GetWindowContext` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_window_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_selinux_get_window_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `SELinux::GetWindowContext` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `context`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_window_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_selinux_get_window_context_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `SELinux::ListItem` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `object_context`
/// - `data_context`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_list_item_t {
    pub name: xcb_atom_t,
    pub object_context_len: u32,
    pub data_context_len: u32,
}

impl Default for xcb_selinux_list_item_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `SELinux::ListItem` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_list_item_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_selinux_list_item_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_selinux_list_item_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SELinux::SetPropertyCreateContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXselinux::xcb_selinux_id()`], then the type of the request is
/// [`xcb_selinux_set_property_create_context_request_t`].
pub const XCB_SELINUX_SET_PROPERTY_CREATE_CONTEXT: u8 = 8i32 as u8;

/// The `SELinux::SetPropertyCreateContext` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `context`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_set_property_create_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_len: u32,
}

impl Default for xcb_selinux_set_property_create_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `SELinux::GetPropertyCreateContext` request.
///
/// Pass this cookie to [`xcb_selinux_get_property_create_context_reply`] to retrieve the reply.
///
/// [`xcb_selinux_get_property_create_context_reply`]: XcbXselinux::xcb_selinux_get_property_create_context_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_create_context_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_property_create_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SELinux::GetPropertyCreateContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXselinux::xcb_selinux_id()`], then the type of the request is
/// [`xcb_selinux_get_property_create_context_request_t`].
pub const XCB_SELINUX_GET_PROPERTY_CREATE_CONTEXT: u8 = 9i32 as u8;

/// The `SELinux::GetPropertyCreateContext` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_create_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_selinux_get_property_create_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `SELinux::GetPropertyCreateContext` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `context`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_create_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_selinux_get_property_create_context_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SELinux::SetPropertyUseContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXselinux::xcb_selinux_id()`], then the type of the request is
/// [`xcb_selinux_set_property_use_context_request_t`].
pub const XCB_SELINUX_SET_PROPERTY_USE_CONTEXT: u8 = 10i32 as u8;

/// The `SELinux::SetPropertyUseContext` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `context`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_set_property_use_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_len: u32,
}

impl Default for xcb_selinux_set_property_use_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `SELinux::GetPropertyUseContext` request.
///
/// Pass this cookie to [`xcb_selinux_get_property_use_context_reply`] to retrieve the reply.
///
/// [`xcb_selinux_get_property_use_context_reply`]: XcbXselinux::xcb_selinux_get_property_use_context_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_use_context_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_property_use_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SELinux::GetPropertyUseContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXselinux::xcb_selinux_id()`], then the type of the request is
/// [`xcb_selinux_get_property_use_context_request_t`].
pub const XCB_SELINUX_GET_PROPERTY_USE_CONTEXT: u8 = 11i32 as u8;

/// The `SELinux::GetPropertyUseContext` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_use_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_selinux_get_property_use_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `SELinux::GetPropertyUseContext` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `context`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_use_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_selinux_get_property_use_context_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `SELinux::GetPropertyContext` request.
///
/// Pass this cookie to [`xcb_selinux_get_property_context_reply`] to retrieve the reply.
///
/// [`xcb_selinux_get_property_context_reply`]: XcbXselinux::xcb_selinux_get_property_context_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_context_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_property_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SELinux::GetPropertyContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXselinux::xcb_selinux_id()`], then the type of the request is
/// [`xcb_selinux_get_property_context_request_t`].
pub const XCB_SELINUX_GET_PROPERTY_CONTEXT: u8 = 12i32 as u8;

/// The `SELinux::GetPropertyContext` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub property: xcb_atom_t,
}

impl Default for xcb_selinux_get_property_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `SELinux::GetPropertyContext` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `context`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_selinux_get_property_context_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `SELinux::GetPropertyDataContext` request.
///
/// Pass this cookie to [`xcb_selinux_get_property_data_context_reply`] to retrieve the reply.
///
/// [`xcb_selinux_get_property_data_context_reply`]: XcbXselinux::xcb_selinux_get_property_data_context_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_data_context_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_property_data_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SELinux::GetPropertyDataContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXselinux::xcb_selinux_id()`], then the type of the request is
/// [`xcb_selinux_get_property_data_context_request_t`].
pub const XCB_SELINUX_GET_PROPERTY_DATA_CONTEXT: u8 = 13i32 as u8;

/// The `SELinux::GetPropertyDataContext` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_data_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub property: xcb_atom_t,
}

impl Default for xcb_selinux_get_property_data_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `SELinux::GetPropertyDataContext` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `context`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_data_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_selinux_get_property_data_context_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `SELinux::ListProperties` request.
///
/// Pass this cookie to [`xcb_selinux_list_properties_reply`] to retrieve the reply.
///
/// [`xcb_selinux_list_properties_reply`]: XcbXselinux::xcb_selinux_list_properties_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_list_properties_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_selinux_list_properties_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SELinux::ListProperties` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXselinux::xcb_selinux_id()`], then the type of the request is
/// [`xcb_selinux_list_properties_request_t`].
pub const XCB_SELINUX_LIST_PROPERTIES: u8 = 14i32 as u8;

/// The `SELinux::ListProperties` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_list_properties_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_selinux_list_properties_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `SELinux::ListProperties` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `properties`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_list_properties_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub properties_len: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_selinux_list_properties_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SELinux::SetSelectionCreateContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXselinux::xcb_selinux_id()`], then the type of the request is
/// [`xcb_selinux_set_selection_create_context_request_t`].
pub const XCB_SELINUX_SET_SELECTION_CREATE_CONTEXT: u8 = 15i32 as u8;

/// The `SELinux::SetSelectionCreateContext` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `context`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_set_selection_create_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_len: u32,
}

impl Default for xcb_selinux_set_selection_create_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `SELinux::GetSelectionCreateContext` request.
///
/// Pass this cookie to [`xcb_selinux_get_selection_create_context_reply`] to retrieve the reply.
///
/// [`xcb_selinux_get_selection_create_context_reply`]: XcbXselinux::xcb_selinux_get_selection_create_context_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_create_context_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_selection_create_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SELinux::GetSelectionCreateContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXselinux::xcb_selinux_id()`], then the type of the request is
/// [`xcb_selinux_get_selection_create_context_request_t`].
pub const XCB_SELINUX_GET_SELECTION_CREATE_CONTEXT: u8 = 16i32 as u8;

/// The `SELinux::GetSelectionCreateContext` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_create_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_selinux_get_selection_create_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `SELinux::GetSelectionCreateContext` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `context`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_create_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_selinux_get_selection_create_context_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SELinux::SetSelectionUseContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXselinux::xcb_selinux_id()`], then the type of the request is
/// [`xcb_selinux_set_selection_use_context_request_t`].
pub const XCB_SELINUX_SET_SELECTION_USE_CONTEXT: u8 = 17i32 as u8;

/// The `SELinux::SetSelectionUseContext` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `context`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_set_selection_use_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_len: u32,
}

impl Default for xcb_selinux_set_selection_use_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `SELinux::GetSelectionUseContext` request.
///
/// Pass this cookie to [`xcb_selinux_get_selection_use_context_reply`] to retrieve the reply.
///
/// [`xcb_selinux_get_selection_use_context_reply`]: XcbXselinux::xcb_selinux_get_selection_use_context_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_use_context_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_selection_use_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SELinux::GetSelectionUseContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXselinux::xcb_selinux_id()`], then the type of the request is
/// [`xcb_selinux_get_selection_use_context_request_t`].
pub const XCB_SELINUX_GET_SELECTION_USE_CONTEXT: u8 = 18i32 as u8;

/// The `SELinux::GetSelectionUseContext` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_use_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_selinux_get_selection_use_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `SELinux::GetSelectionUseContext` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `context`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_use_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_selinux_get_selection_use_context_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `SELinux::GetSelectionContext` request.
///
/// Pass this cookie to [`xcb_selinux_get_selection_context_reply`] to retrieve the reply.
///
/// [`xcb_selinux_get_selection_context_reply`]: XcbXselinux::xcb_selinux_get_selection_context_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_context_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_selection_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SELinux::GetSelectionContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXselinux::xcb_selinux_id()`], then the type of the request is
/// [`xcb_selinux_get_selection_context_request_t`].
pub const XCB_SELINUX_GET_SELECTION_CONTEXT: u8 = 19i32 as u8;

/// The `SELinux::GetSelectionContext` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub selection: xcb_atom_t,
}

impl Default for xcb_selinux_get_selection_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `SELinux::GetSelectionContext` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `context`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_selinux_get_selection_context_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `SELinux::GetSelectionDataContext` request.
///
/// Pass this cookie to [`xcb_selinux_get_selection_data_context_reply`] to retrieve the reply.
///
/// [`xcb_selinux_get_selection_data_context_reply`]: XcbXselinux::xcb_selinux_get_selection_data_context_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_data_context_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_selection_data_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SELinux::GetSelectionDataContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXselinux::xcb_selinux_id()`], then the type of the request is
/// [`xcb_selinux_get_selection_data_context_request_t`].
pub const XCB_SELINUX_GET_SELECTION_DATA_CONTEXT: u8 = 20i32 as u8;

/// The `SELinux::GetSelectionDataContext` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_data_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub selection: xcb_atom_t,
}

impl Default for xcb_selinux_get_selection_data_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `SELinux::GetSelectionDataContext` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `context`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_data_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_selinux_get_selection_data_context_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `SELinux::ListSelections` request.
///
/// Pass this cookie to [`xcb_selinux_list_selections_reply`] to retrieve the reply.
///
/// [`xcb_selinux_list_selections_reply`]: XcbXselinux::xcb_selinux_list_selections_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_list_selections_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_selinux_list_selections_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SELinux::ListSelections` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXselinux::xcb_selinux_id()`], then the type of the request is
/// [`xcb_selinux_list_selections_request_t`].
pub const XCB_SELINUX_LIST_SELECTIONS: u8 = 21i32 as u8;

/// The `SELinux::ListSelections` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_list_selections_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_selinux_list_selections_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `SELinux::ListSelections` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `selections`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_list_selections_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub selections_len: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_selinux_list_selections_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `SELinux::GetClientContext` request.
///
/// Pass this cookie to [`xcb_selinux_get_client_context_reply`] to retrieve the reply.
///
/// [`xcb_selinux_get_client_context_reply`]: XcbXselinux::xcb_selinux_get_client_context_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_client_context_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_client_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SELinux::GetClientContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXselinux::xcb_selinux_id()`], then the type of the request is
/// [`xcb_selinux_get_client_context_request_t`].
pub const XCB_SELINUX_GET_CLIENT_CONTEXT: u8 = 22i32 as u8;

/// The `SELinux::GetClientContext` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_client_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub resource: u32,
}

impl Default for xcb_selinux_get_client_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `SELinux::GetClientContext` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `context`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_client_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_selinux_get_client_context_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_xselinux")]
pub(crate) struct XcbXselinuxXselinux {
    xcb_selinux_id: LazySymbol<*mut xcb_extension_t>,
    xcb_selinux_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major: u8,
            client_minor: u8,
        ) -> xcb_selinux_query_version_cookie_t,
    >,
    xcb_selinux_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major: u8,
            client_minor: u8,
        ) -> xcb_selinux_query_version_cookie_t,
    >,
    xcb_selinux_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_query_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_query_version_reply_t,
    >,
    xcb_selinux_set_device_create_context_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_selinux_set_device_create_context_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_selinux_set_device_create_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_selinux_set_device_create_context_context: LazySymbol<
        unsafe fn(r: *const xcb_selinux_set_device_create_context_request_t) -> *mut c_char,
    >,
    xcb_selinux_set_device_create_context_context_length:
        LazySymbol<unsafe fn(r: *const xcb_selinux_set_device_create_context_request_t) -> c_int>,
    xcb_selinux_set_device_create_context_context_end: LazySymbol<
        unsafe fn(
            r: *const xcb_selinux_set_device_create_context_request_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_selinux_get_device_create_context_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_selinux_get_device_create_context: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_get_device_create_context_cookie_t,
    >,
    xcb_selinux_get_device_create_context_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_get_device_create_context_cookie_t,
    >,
    xcb_selinux_get_device_create_context_context: LazySymbol<
        unsafe fn(r: *const xcb_selinux_get_device_create_context_reply_t) -> *mut c_char,
    >,
    xcb_selinux_get_device_create_context_context_length:
        LazySymbol<unsafe fn(r: *const xcb_selinux_get_device_create_context_reply_t) -> c_int>,
    xcb_selinux_get_device_create_context_context_end: LazySymbol<
        unsafe fn(
            r: *const xcb_selinux_get_device_create_context_reply_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_selinux_get_device_create_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_device_create_context_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_device_create_context_reply_t,
    >,
    xcb_selinux_set_device_context_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_selinux_set_device_context_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device: u32,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_selinux_set_device_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device: u32,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_selinux_set_device_context_context:
        LazySymbol<unsafe fn(r: *const xcb_selinux_set_device_context_request_t) -> *mut c_char>,
    xcb_selinux_set_device_context_context_length:
        LazySymbol<unsafe fn(r: *const xcb_selinux_set_device_context_request_t) -> c_int>,
    xcb_selinux_set_device_context_context_end: LazySymbol<
        unsafe fn(r: *const xcb_selinux_set_device_context_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_selinux_get_device_context_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_selinux_get_device_context: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, device: u32) -> xcb_selinux_get_device_context_cookie_t,
    >,
    xcb_selinux_get_device_context_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, device: u32) -> xcb_selinux_get_device_context_cookie_t,
    >,
    xcb_selinux_get_device_context_context:
        LazySymbol<unsafe fn(r: *const xcb_selinux_get_device_context_reply_t) -> *mut c_char>,
    xcb_selinux_get_device_context_context_length:
        LazySymbol<unsafe fn(r: *const xcb_selinux_get_device_context_reply_t) -> c_int>,
    xcb_selinux_get_device_context_context_end: LazySymbol<
        unsafe fn(r: *const xcb_selinux_get_device_context_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_selinux_get_device_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_device_context_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_device_context_reply_t,
    >,
    xcb_selinux_set_window_create_context_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_selinux_set_window_create_context_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_selinux_set_window_create_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_selinux_set_window_create_context_context: LazySymbol<
        unsafe fn(r: *const xcb_selinux_set_window_create_context_request_t) -> *mut c_char,
    >,
    xcb_selinux_set_window_create_context_context_length:
        LazySymbol<unsafe fn(r: *const xcb_selinux_set_window_create_context_request_t) -> c_int>,
    xcb_selinux_set_window_create_context_context_end: LazySymbol<
        unsafe fn(
            r: *const xcb_selinux_set_window_create_context_request_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_selinux_get_window_create_context_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_selinux_get_window_create_context: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_get_window_create_context_cookie_t,
    >,
    xcb_selinux_get_window_create_context_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_get_window_create_context_cookie_t,
    >,
    xcb_selinux_get_window_create_context_context: LazySymbol<
        unsafe fn(r: *const xcb_selinux_get_window_create_context_reply_t) -> *mut c_char,
    >,
    xcb_selinux_get_window_create_context_context_length:
        LazySymbol<unsafe fn(r: *const xcb_selinux_get_window_create_context_reply_t) -> c_int>,
    xcb_selinux_get_window_create_context_context_end: LazySymbol<
        unsafe fn(
            r: *const xcb_selinux_get_window_create_context_reply_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_selinux_get_window_create_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_window_create_context_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_window_create_context_reply_t,
    >,
    xcb_selinux_get_window_context_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_selinux_get_window_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_selinux_get_window_context_cookie_t,
    >,
    xcb_selinux_get_window_context_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_selinux_get_window_context_cookie_t,
    >,
    xcb_selinux_get_window_context_context:
        LazySymbol<unsafe fn(r: *const xcb_selinux_get_window_context_reply_t) -> *mut c_char>,
    xcb_selinux_get_window_context_context_length:
        LazySymbol<unsafe fn(r: *const xcb_selinux_get_window_context_reply_t) -> c_int>,
    xcb_selinux_get_window_context_context_end: LazySymbol<
        unsafe fn(r: *const xcb_selinux_get_window_context_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_selinux_get_window_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_window_context_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_window_context_reply_t,
    >,
    xcb_selinux_list_item_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_selinux_list_item_object_context:
        LazySymbol<unsafe fn(r: *const xcb_selinux_list_item_t) -> *mut c_char>,
    xcb_selinux_list_item_object_context_length:
        LazySymbol<unsafe fn(r: *const xcb_selinux_list_item_t) -> c_int>,
    xcb_selinux_list_item_object_context_end:
        LazySymbol<unsafe fn(r: *const xcb_selinux_list_item_t) -> xcb_generic_iterator_t>,
    xcb_selinux_list_item_data_context:
        LazySymbol<unsafe fn(r: *const xcb_selinux_list_item_t) -> *mut c_char>,
    xcb_selinux_list_item_data_context_length:
        LazySymbol<unsafe fn(r: *const xcb_selinux_list_item_t) -> c_int>,
    xcb_selinux_list_item_data_context_end:
        LazySymbol<unsafe fn(r: *const xcb_selinux_list_item_t) -> xcb_generic_iterator_t>,
    xcb_selinux_list_item_next: LazySymbol<unsafe fn(i: *mut xcb_selinux_list_item_iterator_t)>,
    xcb_selinux_list_item_end:
        LazySymbol<unsafe fn(i: xcb_selinux_list_item_iterator_t) -> xcb_generic_iterator_t>,
    xcb_selinux_set_property_create_context_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_selinux_set_property_create_context_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_selinux_set_property_create_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_selinux_set_property_create_context_context: LazySymbol<
        unsafe fn(r: *const xcb_selinux_set_property_create_context_request_t) -> *mut c_char,
    >,
    xcb_selinux_set_property_create_context_context_length:
        LazySymbol<unsafe fn(r: *const xcb_selinux_set_property_create_context_request_t) -> c_int>,
    xcb_selinux_set_property_create_context_context_end: LazySymbol<
        unsafe fn(
            r: *const xcb_selinux_set_property_create_context_request_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_selinux_get_property_create_context_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_selinux_get_property_create_context: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_get_property_create_context_cookie_t,
    >,
    xcb_selinux_get_property_create_context_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_get_property_create_context_cookie_t,
    >,
    xcb_selinux_get_property_create_context_context: LazySymbol<
        unsafe fn(r: *const xcb_selinux_get_property_create_context_reply_t) -> *mut c_char,
    >,
    xcb_selinux_get_property_create_context_context_length:
        LazySymbol<unsafe fn(r: *const xcb_selinux_get_property_create_context_reply_t) -> c_int>,
    xcb_selinux_get_property_create_context_context_end: LazySymbol<
        unsafe fn(
            r: *const xcb_selinux_get_property_create_context_reply_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_selinux_get_property_create_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_property_create_context_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_property_create_context_reply_t,
    >,
    xcb_selinux_set_property_use_context_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_selinux_set_property_use_context_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_selinux_set_property_use_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_selinux_set_property_use_context_context: LazySymbol<
        unsafe fn(r: *const xcb_selinux_set_property_use_context_request_t) -> *mut c_char,
    >,
    xcb_selinux_set_property_use_context_context_length:
        LazySymbol<unsafe fn(r: *const xcb_selinux_set_property_use_context_request_t) -> c_int>,
    xcb_selinux_set_property_use_context_context_end: LazySymbol<
        unsafe fn(
            r: *const xcb_selinux_set_property_use_context_request_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_selinux_get_property_use_context_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_selinux_get_property_use_context: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_get_property_use_context_cookie_t,
    >,
    xcb_selinux_get_property_use_context_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_get_property_use_context_cookie_t,
    >,
    xcb_selinux_get_property_use_context_context: LazySymbol<
        unsafe fn(r: *const xcb_selinux_get_property_use_context_reply_t) -> *mut c_char,
    >,
    xcb_selinux_get_property_use_context_context_length:
        LazySymbol<unsafe fn(r: *const xcb_selinux_get_property_use_context_reply_t) -> c_int>,
    xcb_selinux_get_property_use_context_context_end: LazySymbol<
        unsafe fn(r: *const xcb_selinux_get_property_use_context_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_selinux_get_property_use_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_property_use_context_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_property_use_context_reply_t,
    >,
    xcb_selinux_get_property_context_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_selinux_get_property_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            property: xcb_atom_t,
        ) -> xcb_selinux_get_property_context_cookie_t,
    >,
    xcb_selinux_get_property_context_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            property: xcb_atom_t,
        ) -> xcb_selinux_get_property_context_cookie_t,
    >,
    xcb_selinux_get_property_context_context:
        LazySymbol<unsafe fn(r: *const xcb_selinux_get_property_context_reply_t) -> *mut c_char>,
    xcb_selinux_get_property_context_context_length:
        LazySymbol<unsafe fn(r: *const xcb_selinux_get_property_context_reply_t) -> c_int>,
    xcb_selinux_get_property_context_context_end: LazySymbol<
        unsafe fn(r: *const xcb_selinux_get_property_context_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_selinux_get_property_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_property_context_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_property_context_reply_t,
    >,
    xcb_selinux_get_property_data_context_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_selinux_get_property_data_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            property: xcb_atom_t,
        ) -> xcb_selinux_get_property_data_context_cookie_t,
    >,
    xcb_selinux_get_property_data_context_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            property: xcb_atom_t,
        ) -> xcb_selinux_get_property_data_context_cookie_t,
    >,
    xcb_selinux_get_property_data_context_context: LazySymbol<
        unsafe fn(r: *const xcb_selinux_get_property_data_context_reply_t) -> *mut c_char,
    >,
    xcb_selinux_get_property_data_context_context_length:
        LazySymbol<unsafe fn(r: *const xcb_selinux_get_property_data_context_reply_t) -> c_int>,
    xcb_selinux_get_property_data_context_context_end: LazySymbol<
        unsafe fn(
            r: *const xcb_selinux_get_property_data_context_reply_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_selinux_get_property_data_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_property_data_context_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_property_data_context_reply_t,
    >,
    xcb_selinux_list_properties_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_selinux_list_properties: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_selinux_list_properties_cookie_t,
    >,
    xcb_selinux_list_properties_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_selinux_list_properties_cookie_t,
    >,
    xcb_selinux_list_properties_properties_length:
        LazySymbol<unsafe fn(r: *const xcb_selinux_list_properties_reply_t) -> c_int>,
    xcb_selinux_list_properties_properties_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_selinux_list_properties_reply_t,
        ) -> xcb_selinux_list_item_iterator_t,
    >,
    xcb_selinux_list_properties_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_list_properties_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_list_properties_reply_t,
    >,
    xcb_selinux_set_selection_create_context_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_selinux_set_selection_create_context_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_selinux_set_selection_create_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_selinux_set_selection_create_context_context: LazySymbol<
        unsafe fn(r: *const xcb_selinux_set_selection_create_context_request_t) -> *mut c_char,
    >,
    xcb_selinux_set_selection_create_context_context_length: LazySymbol<
        unsafe fn(r: *const xcb_selinux_set_selection_create_context_request_t) -> c_int,
    >,
    xcb_selinux_set_selection_create_context_context_end: LazySymbol<
        unsafe fn(
            r: *const xcb_selinux_set_selection_create_context_request_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_selinux_get_selection_create_context_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_selinux_get_selection_create_context: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_get_selection_create_context_cookie_t,
    >,
    xcb_selinux_get_selection_create_context_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_get_selection_create_context_cookie_t,
    >,
    xcb_selinux_get_selection_create_context_context: LazySymbol<
        unsafe fn(r: *const xcb_selinux_get_selection_create_context_reply_t) -> *mut c_char,
    >,
    xcb_selinux_get_selection_create_context_context_length:
        LazySymbol<unsafe fn(r: *const xcb_selinux_get_selection_create_context_reply_t) -> c_int>,
    xcb_selinux_get_selection_create_context_context_end: LazySymbol<
        unsafe fn(
            r: *const xcb_selinux_get_selection_create_context_reply_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_selinux_get_selection_create_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_selection_create_context_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_selection_create_context_reply_t,
    >,
    xcb_selinux_set_selection_use_context_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_selinux_set_selection_use_context_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_selinux_set_selection_use_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_selinux_set_selection_use_context_context: LazySymbol<
        unsafe fn(r: *const xcb_selinux_set_selection_use_context_request_t) -> *mut c_char,
    >,
    xcb_selinux_set_selection_use_context_context_length:
        LazySymbol<unsafe fn(r: *const xcb_selinux_set_selection_use_context_request_t) -> c_int>,
    xcb_selinux_set_selection_use_context_context_end: LazySymbol<
        unsafe fn(
            r: *const xcb_selinux_set_selection_use_context_request_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_selinux_get_selection_use_context_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_selinux_get_selection_use_context: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_get_selection_use_context_cookie_t,
    >,
    xcb_selinux_get_selection_use_context_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_get_selection_use_context_cookie_t,
    >,
    xcb_selinux_get_selection_use_context_context: LazySymbol<
        unsafe fn(r: *const xcb_selinux_get_selection_use_context_reply_t) -> *mut c_char,
    >,
    xcb_selinux_get_selection_use_context_context_length:
        LazySymbol<unsafe fn(r: *const xcb_selinux_get_selection_use_context_reply_t) -> c_int>,
    xcb_selinux_get_selection_use_context_context_end: LazySymbol<
        unsafe fn(
            r: *const xcb_selinux_get_selection_use_context_reply_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_selinux_get_selection_use_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_selection_use_context_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_selection_use_context_reply_t,
    >,
    xcb_selinux_get_selection_context_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_selinux_get_selection_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            selection: xcb_atom_t,
        ) -> xcb_selinux_get_selection_context_cookie_t,
    >,
    xcb_selinux_get_selection_context_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            selection: xcb_atom_t,
        ) -> xcb_selinux_get_selection_context_cookie_t,
    >,
    xcb_selinux_get_selection_context_context:
        LazySymbol<unsafe fn(r: *const xcb_selinux_get_selection_context_reply_t) -> *mut c_char>,
    xcb_selinux_get_selection_context_context_length:
        LazySymbol<unsafe fn(r: *const xcb_selinux_get_selection_context_reply_t) -> c_int>,
    xcb_selinux_get_selection_context_context_end: LazySymbol<
        unsafe fn(r: *const xcb_selinux_get_selection_context_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_selinux_get_selection_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_selection_context_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_selection_context_reply_t,
    >,
    xcb_selinux_get_selection_data_context_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_selinux_get_selection_data_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            selection: xcb_atom_t,
        ) -> xcb_selinux_get_selection_data_context_cookie_t,
    >,
    xcb_selinux_get_selection_data_context_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            selection: xcb_atom_t,
        ) -> xcb_selinux_get_selection_data_context_cookie_t,
    >,
    xcb_selinux_get_selection_data_context_context: LazySymbol<
        unsafe fn(r: *const xcb_selinux_get_selection_data_context_reply_t) -> *mut c_char,
    >,
    xcb_selinux_get_selection_data_context_context_length:
        LazySymbol<unsafe fn(r: *const xcb_selinux_get_selection_data_context_reply_t) -> c_int>,
    xcb_selinux_get_selection_data_context_context_end: LazySymbol<
        unsafe fn(
            r: *const xcb_selinux_get_selection_data_context_reply_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_selinux_get_selection_data_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_selection_data_context_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_selection_data_context_reply_t,
    >,
    xcb_selinux_list_selections_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_selinux_list_selections:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_list_selections_cookie_t>,
    xcb_selinux_list_selections_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_list_selections_cookie_t>,
    xcb_selinux_list_selections_selections_length:
        LazySymbol<unsafe fn(r: *const xcb_selinux_list_selections_reply_t) -> c_int>,
    xcb_selinux_list_selections_selections_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_selinux_list_selections_reply_t,
        ) -> xcb_selinux_list_item_iterator_t,
    >,
    xcb_selinux_list_selections_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_list_selections_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_list_selections_reply_t,
    >,
    xcb_selinux_get_client_context_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_selinux_get_client_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            resource: u32,
        ) -> xcb_selinux_get_client_context_cookie_t,
    >,
    xcb_selinux_get_client_context_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            resource: u32,
        ) -> xcb_selinux_get_client_context_cookie_t,
    >,
    xcb_selinux_get_client_context_context:
        LazySymbol<unsafe fn(r: *const xcb_selinux_get_client_context_reply_t) -> *mut c_char>,
    xcb_selinux_get_client_context_context_length:
        LazySymbol<unsafe fn(r: *const xcb_selinux_get_client_context_reply_t) -> c_int>,
    xcb_selinux_get_client_context_context_end: LazySymbol<
        unsafe fn(r: *const xcb_selinux_get_client_context_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_selinux_get_client_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_client_context_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_client_context_reply_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self
            .xselinux
            .$f
            .get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .xselinux
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_xselinux")]
impl XcbXselinux {
    /// The libxcb identifier of the `SELinux` extension.
    #[inline]
    pub fn xcb_selinux_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_selinux_id) }
    }

    /// Returns `true` iff the symbol `xcb_selinux_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_id(&self) -> bool {
        has_sym!(self, xcb_selinux_id)
    }

    /// Sends a `SELinux::QueryVersion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_query_version_reply`]: Self::xcb_selinux_query_version_reply
    #[inline]
    pub unsafe fn xcb_selinux_query_version(
        &self,
        c: *mut xcb_connection_t,
        client_major: u8,
        client_minor: u8,
    ) -> xcb_selinux_query_version_cookie_t {
        sym!(self, xcb_selinux_query_version)(c, client_major, client_minor)
    }

    /// Returns `true` iff the symbol `xcb_selinux_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_query_version(&self) -> bool {
        has_sym!(self, xcb_selinux_query_version)
    }

    /// Sends a `SELinux::QueryVersion` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_query_version_reply`]: Self::xcb_selinux_query_version_reply
    #[inline]
    pub unsafe fn xcb_selinux_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        client_major: u8,
        client_minor: u8,
    ) -> xcb_selinux_query_version_cookie_t {
        sym!(self, xcb_selinux_query_version_unchecked)(c, client_major, client_minor)
    }

    /// Returns `true` iff the symbol `xcb_selinux_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_query_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_selinux_query_version_unchecked)
    }

    /// Waits for the reply to a `SELinux::QueryVersion` request.
    #[inline]
    pub unsafe fn xcb_selinux_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_query_version_reply_t {
        sym!(self, xcb_selinux_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_query_version_reply)
    }

    /// Computes the size of a `xcb_selinux_set_device_create_context_request_t` object.
    #[inline]
    pub unsafe fn xcb_selinux_set_device_create_context_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_selinux_set_device_create_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_device_create_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_device_create_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_set_device_create_context_sizeof)
    }

    /// Sends a `SELinux::SetDeviceCreateContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_selinux_set_device_create_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_selinux_set_device_create_context_checked)(c, context_len, context)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_device_create_context_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_device_create_context_checked(&self) -> bool {
        has_sym!(self, xcb_selinux_set_device_create_context_checked)
    }

    /// Sends a `SELinux::SetDeviceCreateContext` request (unchecked).
    #[inline]
    pub unsafe fn xcb_selinux_set_device_create_context(
        &self,
        c: *mut xcb_connection_t,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_selinux_set_device_create_context)(c, context_len, context)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_device_create_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_device_create_context(&self) -> bool {
        has_sym!(self, xcb_selinux_set_device_create_context)
    }

    /// Returns a pointer to the `context` field of a `xcb_selinux_set_device_create_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_set_device_create_context_context(
        &self,
        r: *const xcb_selinux_set_device_create_context_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_selinux_set_device_create_context_context)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_device_create_context_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_device_create_context_context(&self) -> bool {
        has_sym!(self, xcb_selinux_set_device_create_context_context)
    }

    /// Returns the number of elements of the `context` field of a `xcb_selinux_set_device_create_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_set_device_create_context_context_length(
        &self,
        r: *const xcb_selinux_set_device_create_context_request_t,
    ) -> c_int {
        sym!(self, xcb_selinux_set_device_create_context_context_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_device_create_context_context_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_device_create_context_context_length(&self) -> bool {
        has_sym!(self, xcb_selinux_set_device_create_context_context_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `context` field of a `xcb_selinux_set_device_create_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_set_device_create_context_context_end(
        &self,
        r: *const xcb_selinux_set_device_create_context_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_selinux_set_device_create_context_context_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_device_create_context_context_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_device_create_context_context_end(&self) -> bool {
        has_sym!(self, xcb_selinux_set_device_create_context_context_end)
    }

    /// Computes the size of a `xcb_selinux_get_device_create_context_reply_t` object.
    #[inline]
    pub unsafe fn xcb_selinux_get_device_create_context_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_selinux_get_device_create_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_device_create_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_device_create_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_get_device_create_context_sizeof)
    }

    /// Sends a `SELinux::GetDeviceCreateContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_device_create_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_device_create_context_reply`]: Self::xcb_selinux_get_device_create_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_device_create_context(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_get_device_create_context_cookie_t {
        sym!(self, xcb_selinux_get_device_create_context)(c)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_device_create_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_device_create_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_device_create_context)
    }

    /// Sends a `SELinux::GetDeviceCreateContext` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_device_create_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_device_create_context_reply`]: Self::xcb_selinux_get_device_create_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_device_create_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_get_device_create_context_cookie_t {
        sym!(self, xcb_selinux_get_device_create_context_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_device_create_context_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_device_create_context_unchecked(&self) -> bool {
        has_sym!(self, xcb_selinux_get_device_create_context_unchecked)
    }

    /// Returns a pointer to the `context` field of a `xcb_selinux_get_device_create_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_device_create_context_context(
        &self,
        r: *const xcb_selinux_get_device_create_context_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_selinux_get_device_create_context_context)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_device_create_context_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_device_create_context_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_device_create_context_context)
    }

    /// Returns the number of elements of the `context` field of a `xcb_selinux_get_device_create_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_device_create_context_context_length(
        &self,
        r: *const xcb_selinux_get_device_create_context_reply_t,
    ) -> c_int {
        sym!(self, xcb_selinux_get_device_create_context_context_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_device_create_context_context_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_device_create_context_context_length(&self) -> bool {
        has_sym!(self, xcb_selinux_get_device_create_context_context_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `context` field of a `xcb_selinux_get_device_create_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_device_create_context_context_end(
        &self,
        r: *const xcb_selinux_get_device_create_context_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_selinux_get_device_create_context_context_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_device_create_context_context_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_device_create_context_context_end(&self) -> bool {
        has_sym!(self, xcb_selinux_get_device_create_context_context_end)
    }

    /// Waits for the reply to a `SELinux::GetDeviceCreateContext` request.
    #[inline]
    pub unsafe fn xcb_selinux_get_device_create_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_device_create_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_device_create_context_reply_t {
        sym!(self, xcb_selinux_get_device_create_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_device_create_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_device_create_context_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_get_device_create_context_reply)
    }

    /// Computes the size of a `xcb_selinux_set_device_context_request_t` object.
    #[inline]
    pub unsafe fn xcb_selinux_set_device_context_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_selinux_set_device_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_device_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_device_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_set_device_context_sizeof)
    }

    /// Sends a `SELinux::SetDeviceContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_selinux_set_device_context_checked(
        &self,
        c: *mut xcb_connection_t,
        device: u32,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_selinux_set_device_context_checked)(c, device, context_len, context)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_device_context_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_device_context_checked(&self) -> bool {
        has_sym!(self, xcb_selinux_set_device_context_checked)
    }

    /// Sends a `SELinux::SetDeviceContext` request (unchecked).
    #[inline]
    pub unsafe fn xcb_selinux_set_device_context(
        &self,
        c: *mut xcb_connection_t,
        device: u32,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_selinux_set_device_context)(c, device, context_len, context)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_device_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_device_context(&self) -> bool {
        has_sym!(self, xcb_selinux_set_device_context)
    }

    /// Returns a pointer to the `context` field of a `xcb_selinux_set_device_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_set_device_context_context(
        &self,
        r: *const xcb_selinux_set_device_context_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_selinux_set_device_context_context)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_device_context_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_device_context_context(&self) -> bool {
        has_sym!(self, xcb_selinux_set_device_context_context)
    }

    /// Returns the number of elements of the `context` field of a `xcb_selinux_set_device_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_set_device_context_context_length(
        &self,
        r: *const xcb_selinux_set_device_context_request_t,
    ) -> c_int {
        sym!(self, xcb_selinux_set_device_context_context_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_device_context_context_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_device_context_context_length(&self) -> bool {
        has_sym!(self, xcb_selinux_set_device_context_context_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `context` field of a `xcb_selinux_set_device_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_set_device_context_context_end(
        &self,
        r: *const xcb_selinux_set_device_context_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_selinux_set_device_context_context_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_device_context_context_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_device_context_context_end(&self) -> bool {
        has_sym!(self, xcb_selinux_set_device_context_context_end)
    }

    /// Computes the size of a `xcb_selinux_get_device_context_reply_t` object.
    #[inline]
    pub unsafe fn xcb_selinux_get_device_context_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_selinux_get_device_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_device_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_device_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_get_device_context_sizeof)
    }

    /// Sends a `SELinux::GetDeviceContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_device_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_device_context_reply`]: Self::xcb_selinux_get_device_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_device_context(
        &self,
        c: *mut xcb_connection_t,
        device: u32,
    ) -> xcb_selinux_get_device_context_cookie_t {
        sym!(self, xcb_selinux_get_device_context)(c, device)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_device_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_device_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_device_context)
    }

    /// Sends a `SELinux::GetDeviceContext` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_device_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_device_context_reply`]: Self::xcb_selinux_get_device_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_device_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device: u32,
    ) -> xcb_selinux_get_device_context_cookie_t {
        sym!(self, xcb_selinux_get_device_context_unchecked)(c, device)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_device_context_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_device_context_unchecked(&self) -> bool {
        has_sym!(self, xcb_selinux_get_device_context_unchecked)
    }

    /// Returns a pointer to the `context` field of a `xcb_selinux_get_device_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_device_context_context(
        &self,
        r: *const xcb_selinux_get_device_context_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_selinux_get_device_context_context)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_device_context_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_device_context_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_device_context_context)
    }

    /// Returns the number of elements of the `context` field of a `xcb_selinux_get_device_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_device_context_context_length(
        &self,
        r: *const xcb_selinux_get_device_context_reply_t,
    ) -> c_int {
        sym!(self, xcb_selinux_get_device_context_context_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_device_context_context_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_device_context_context_length(&self) -> bool {
        has_sym!(self, xcb_selinux_get_device_context_context_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `context` field of a `xcb_selinux_get_device_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_device_context_context_end(
        &self,
        r: *const xcb_selinux_get_device_context_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_selinux_get_device_context_context_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_device_context_context_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_device_context_context_end(&self) -> bool {
        has_sym!(self, xcb_selinux_get_device_context_context_end)
    }

    /// Waits for the reply to a `SELinux::GetDeviceContext` request.
    #[inline]
    pub unsafe fn xcb_selinux_get_device_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_device_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_device_context_reply_t {
        sym!(self, xcb_selinux_get_device_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_device_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_device_context_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_get_device_context_reply)
    }

    /// Computes the size of a `xcb_selinux_set_window_create_context_request_t` object.
    #[inline]
    pub unsafe fn xcb_selinux_set_window_create_context_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_selinux_set_window_create_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_window_create_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_window_create_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_set_window_create_context_sizeof)
    }

    /// Sends a `SELinux::SetWindowCreateContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_selinux_set_window_create_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_selinux_set_window_create_context_checked)(c, context_len, context)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_window_create_context_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_window_create_context_checked(&self) -> bool {
        has_sym!(self, xcb_selinux_set_window_create_context_checked)
    }

    /// Sends a `SELinux::SetWindowCreateContext` request (unchecked).
    #[inline]
    pub unsafe fn xcb_selinux_set_window_create_context(
        &self,
        c: *mut xcb_connection_t,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_selinux_set_window_create_context)(c, context_len, context)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_window_create_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_window_create_context(&self) -> bool {
        has_sym!(self, xcb_selinux_set_window_create_context)
    }

    /// Returns a pointer to the `context` field of a `xcb_selinux_set_window_create_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_set_window_create_context_context(
        &self,
        r: *const xcb_selinux_set_window_create_context_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_selinux_set_window_create_context_context)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_window_create_context_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_window_create_context_context(&self) -> bool {
        has_sym!(self, xcb_selinux_set_window_create_context_context)
    }

    /// Returns the number of elements of the `context` field of a `xcb_selinux_set_window_create_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_set_window_create_context_context_length(
        &self,
        r: *const xcb_selinux_set_window_create_context_request_t,
    ) -> c_int {
        sym!(self, xcb_selinux_set_window_create_context_context_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_window_create_context_context_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_window_create_context_context_length(&self) -> bool {
        has_sym!(self, xcb_selinux_set_window_create_context_context_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `context` field of a `xcb_selinux_set_window_create_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_set_window_create_context_context_end(
        &self,
        r: *const xcb_selinux_set_window_create_context_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_selinux_set_window_create_context_context_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_window_create_context_context_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_window_create_context_context_end(&self) -> bool {
        has_sym!(self, xcb_selinux_set_window_create_context_context_end)
    }

    /// Computes the size of a `xcb_selinux_get_window_create_context_reply_t` object.
    #[inline]
    pub unsafe fn xcb_selinux_get_window_create_context_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_selinux_get_window_create_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_window_create_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_window_create_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_get_window_create_context_sizeof)
    }

    /// Sends a `SELinux::GetWindowCreateContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_window_create_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_window_create_context_reply`]: Self::xcb_selinux_get_window_create_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_window_create_context(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_get_window_create_context_cookie_t {
        sym!(self, xcb_selinux_get_window_create_context)(c)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_window_create_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_window_create_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_window_create_context)
    }

    /// Sends a `SELinux::GetWindowCreateContext` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_window_create_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_window_create_context_reply`]: Self::xcb_selinux_get_window_create_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_window_create_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_get_window_create_context_cookie_t {
        sym!(self, xcb_selinux_get_window_create_context_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_window_create_context_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_window_create_context_unchecked(&self) -> bool {
        has_sym!(self, xcb_selinux_get_window_create_context_unchecked)
    }

    /// Returns a pointer to the `context` field of a `xcb_selinux_get_window_create_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_window_create_context_context(
        &self,
        r: *const xcb_selinux_get_window_create_context_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_selinux_get_window_create_context_context)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_window_create_context_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_window_create_context_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_window_create_context_context)
    }

    /// Returns the number of elements of the `context` field of a `xcb_selinux_get_window_create_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_window_create_context_context_length(
        &self,
        r: *const xcb_selinux_get_window_create_context_reply_t,
    ) -> c_int {
        sym!(self, xcb_selinux_get_window_create_context_context_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_window_create_context_context_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_window_create_context_context_length(&self) -> bool {
        has_sym!(self, xcb_selinux_get_window_create_context_context_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `context` field of a `xcb_selinux_get_window_create_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_window_create_context_context_end(
        &self,
        r: *const xcb_selinux_get_window_create_context_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_selinux_get_window_create_context_context_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_window_create_context_context_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_window_create_context_context_end(&self) -> bool {
        has_sym!(self, xcb_selinux_get_window_create_context_context_end)
    }

    /// Waits for the reply to a `SELinux::GetWindowCreateContext` request.
    #[inline]
    pub unsafe fn xcb_selinux_get_window_create_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_window_create_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_window_create_context_reply_t {
        sym!(self, xcb_selinux_get_window_create_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_window_create_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_window_create_context_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_get_window_create_context_reply)
    }

    /// Computes the size of a `xcb_selinux_get_window_context_reply_t` object.
    #[inline]
    pub unsafe fn xcb_selinux_get_window_context_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_selinux_get_window_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_window_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_window_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_get_window_context_sizeof)
    }

    /// Sends a `SELinux::GetWindowContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_window_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_window_context_reply`]: Self::xcb_selinux_get_window_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_window_context(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_selinux_get_window_context_cookie_t {
        sym!(self, xcb_selinux_get_window_context)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_window_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_window_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_window_context)
    }

    /// Sends a `SELinux::GetWindowContext` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_window_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_window_context_reply`]: Self::xcb_selinux_get_window_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_window_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_selinux_get_window_context_cookie_t {
        sym!(self, xcb_selinux_get_window_context_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_window_context_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_window_context_unchecked(&self) -> bool {
        has_sym!(self, xcb_selinux_get_window_context_unchecked)
    }

    /// Returns a pointer to the `context` field of a `xcb_selinux_get_window_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_window_context_context(
        &self,
        r: *const xcb_selinux_get_window_context_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_selinux_get_window_context_context)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_window_context_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_window_context_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_window_context_context)
    }

    /// Returns the number of elements of the `context` field of a `xcb_selinux_get_window_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_window_context_context_length(
        &self,
        r: *const xcb_selinux_get_window_context_reply_t,
    ) -> c_int {
        sym!(self, xcb_selinux_get_window_context_context_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_window_context_context_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_window_context_context_length(&self) -> bool {
        has_sym!(self, xcb_selinux_get_window_context_context_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `context` field of a `xcb_selinux_get_window_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_window_context_context_end(
        &self,
        r: *const xcb_selinux_get_window_context_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_selinux_get_window_context_context_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_window_context_context_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_window_context_context_end(&self) -> bool {
        has_sym!(self, xcb_selinux_get_window_context_context_end)
    }

    /// Waits for the reply to a `SELinux::GetWindowContext` request.
    #[inline]
    pub unsafe fn xcb_selinux_get_window_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_window_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_window_context_reply_t {
        sym!(self, xcb_selinux_get_window_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_window_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_window_context_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_get_window_context_reply)
    }

    /// Computes the size of a `xcb_selinux_list_item_t` object.
    #[inline]
    pub unsafe fn xcb_selinux_list_item_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_selinux_list_item_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_item_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_item_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_list_item_sizeof)
    }

    /// Returns a pointer to the `object_context` field of a `xcb_selinux_list_item_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_list_item_object_context(
        &self,
        r: *const xcb_selinux_list_item_t,
    ) -> *mut c_char {
        sym!(self, xcb_selinux_list_item_object_context)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_item_object_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_item_object_context(&self) -> bool {
        has_sym!(self, xcb_selinux_list_item_object_context)
    }

    /// Returns the number of elements of the `object_context` field of a `xcb_selinux_list_item_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_list_item_object_context_length(
        &self,
        r: *const xcb_selinux_list_item_t,
    ) -> c_int {
        sym!(self, xcb_selinux_list_item_object_context_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_item_object_context_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_item_object_context_length(&self) -> bool {
        has_sym!(self, xcb_selinux_list_item_object_context_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `object_context` field of a `xcb_selinux_list_item_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_list_item_object_context_end(
        &self,
        r: *const xcb_selinux_list_item_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_selinux_list_item_object_context_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_item_object_context_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_item_object_context_end(&self) -> bool {
        has_sym!(self, xcb_selinux_list_item_object_context_end)
    }

    /// Returns a pointer to the `data_context` field of a `xcb_selinux_list_item_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_list_item_data_context(
        &self,
        r: *const xcb_selinux_list_item_t,
    ) -> *mut c_char {
        sym!(self, xcb_selinux_list_item_data_context)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_item_data_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_item_data_context(&self) -> bool {
        has_sym!(self, xcb_selinux_list_item_data_context)
    }

    /// Returns the number of elements of the `data_context` field of a `xcb_selinux_list_item_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_list_item_data_context_length(
        &self,
        r: *const xcb_selinux_list_item_t,
    ) -> c_int {
        sym!(self, xcb_selinux_list_item_data_context_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_item_data_context_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_item_data_context_length(&self) -> bool {
        has_sym!(self, xcb_selinux_list_item_data_context_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `data_context` field of a `xcb_selinux_list_item_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_list_item_data_context_end(
        &self,
        r: *const xcb_selinux_list_item_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_selinux_list_item_data_context_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_item_data_context_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_item_data_context_end(&self) -> bool {
        has_sym!(self, xcb_selinux_list_item_data_context_end)
    }

    /// Advances a `xcb_selinux_list_item_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_selinux_list_item_next(&self, i: *mut xcb_selinux_list_item_iterator_t) {
        sym!(self, xcb_selinux_list_item_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_item_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_item_next(&self) -> bool {
        has_sym!(self, xcb_selinux_list_item_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_selinux_list_item_iterator_t`.
    #[inline]
    pub unsafe fn xcb_selinux_list_item_end(
        &self,
        i: xcb_selinux_list_item_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_selinux_list_item_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_item_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_item_end(&self) -> bool {
        has_sym!(self, xcb_selinux_list_item_end)
    }

    /// Computes the size of a `xcb_selinux_set_property_create_context_request_t` object.
    #[inline]
    pub unsafe fn xcb_selinux_set_property_create_context_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_selinux_set_property_create_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_property_create_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_property_create_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_set_property_create_context_sizeof)
    }

    /// Sends a `SELinux::SetPropertyCreateContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_selinux_set_property_create_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_selinux_set_property_create_context_checked)(c, context_len, context)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_property_create_context_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_property_create_context_checked(&self) -> bool {
        has_sym!(self, xcb_selinux_set_property_create_context_checked)
    }

    /// Sends a `SELinux::SetPropertyCreateContext` request (unchecked).
    #[inline]
    pub unsafe fn xcb_selinux_set_property_create_context(
        &self,
        c: *mut xcb_connection_t,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_selinux_set_property_create_context)(c, context_len, context)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_property_create_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_property_create_context(&self) -> bool {
        has_sym!(self, xcb_selinux_set_property_create_context)
    }

    /// Returns a pointer to the `context` field of a `xcb_selinux_set_property_create_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_set_property_create_context_context(
        &self,
        r: *const xcb_selinux_set_property_create_context_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_selinux_set_property_create_context_context)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_property_create_context_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_property_create_context_context(&self) -> bool {
        has_sym!(self, xcb_selinux_set_property_create_context_context)
    }

    /// Returns the number of elements of the `context` field of a `xcb_selinux_set_property_create_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_set_property_create_context_context_length(
        &self,
        r: *const xcb_selinux_set_property_create_context_request_t,
    ) -> c_int {
        sym!(self, xcb_selinux_set_property_create_context_context_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_property_create_context_context_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_property_create_context_context_length(&self) -> bool {
        has_sym!(self, xcb_selinux_set_property_create_context_context_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `context` field of a `xcb_selinux_set_property_create_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_set_property_create_context_context_end(
        &self,
        r: *const xcb_selinux_set_property_create_context_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_selinux_set_property_create_context_context_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_property_create_context_context_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_property_create_context_context_end(&self) -> bool {
        has_sym!(self, xcb_selinux_set_property_create_context_context_end)
    }

    /// Computes the size of a `xcb_selinux_get_property_create_context_reply_t` object.
    #[inline]
    pub unsafe fn xcb_selinux_get_property_create_context_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_selinux_get_property_create_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_create_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_create_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_create_context_sizeof)
    }

    /// Sends a `SELinux::GetPropertyCreateContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_property_create_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_property_create_context_reply`]: Self::xcb_selinux_get_property_create_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_property_create_context(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_get_property_create_context_cookie_t {
        sym!(self, xcb_selinux_get_property_create_context)(c)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_create_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_create_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_create_context)
    }

    /// Sends a `SELinux::GetPropertyCreateContext` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_property_create_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_property_create_context_reply`]: Self::xcb_selinux_get_property_create_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_property_create_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_get_property_create_context_cookie_t {
        sym!(self, xcb_selinux_get_property_create_context_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_create_context_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_create_context_unchecked(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_create_context_unchecked)
    }

    /// Returns a pointer to the `context` field of a `xcb_selinux_get_property_create_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_property_create_context_context(
        &self,
        r: *const xcb_selinux_get_property_create_context_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_selinux_get_property_create_context_context)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_create_context_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_create_context_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_create_context_context)
    }

    /// Returns the number of elements of the `context` field of a `xcb_selinux_get_property_create_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_property_create_context_context_length(
        &self,
        r: *const xcb_selinux_get_property_create_context_reply_t,
    ) -> c_int {
        sym!(self, xcb_selinux_get_property_create_context_context_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_create_context_context_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_create_context_context_length(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_create_context_context_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `context` field of a `xcb_selinux_get_property_create_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_property_create_context_context_end(
        &self,
        r: *const xcb_selinux_get_property_create_context_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_selinux_get_property_create_context_context_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_create_context_context_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_create_context_context_end(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_create_context_context_end)
    }

    /// Waits for the reply to a `SELinux::GetPropertyCreateContext` request.
    #[inline]
    pub unsafe fn xcb_selinux_get_property_create_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_property_create_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_property_create_context_reply_t {
        sym!(self, xcb_selinux_get_property_create_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_create_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_create_context_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_create_context_reply)
    }

    /// Computes the size of a `xcb_selinux_set_property_use_context_request_t` object.
    #[inline]
    pub unsafe fn xcb_selinux_set_property_use_context_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_selinux_set_property_use_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_property_use_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_property_use_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_set_property_use_context_sizeof)
    }

    /// Sends a `SELinux::SetPropertyUseContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_selinux_set_property_use_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_selinux_set_property_use_context_checked)(c, context_len, context)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_property_use_context_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_property_use_context_checked(&self) -> bool {
        has_sym!(self, xcb_selinux_set_property_use_context_checked)
    }

    /// Sends a `SELinux::SetPropertyUseContext` request (unchecked).
    #[inline]
    pub unsafe fn xcb_selinux_set_property_use_context(
        &self,
        c: *mut xcb_connection_t,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_selinux_set_property_use_context)(c, context_len, context)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_property_use_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_property_use_context(&self) -> bool {
        has_sym!(self, xcb_selinux_set_property_use_context)
    }

    /// Returns a pointer to the `context` field of a `xcb_selinux_set_property_use_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_set_property_use_context_context(
        &self,
        r: *const xcb_selinux_set_property_use_context_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_selinux_set_property_use_context_context)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_property_use_context_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_property_use_context_context(&self) -> bool {
        has_sym!(self, xcb_selinux_set_property_use_context_context)
    }

    /// Returns the number of elements of the `context` field of a `xcb_selinux_set_property_use_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_set_property_use_context_context_length(
        &self,
        r: *const xcb_selinux_set_property_use_context_request_t,
    ) -> c_int {
        sym!(self, xcb_selinux_set_property_use_context_context_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_property_use_context_context_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_property_use_context_context_length(&self) -> bool {
        has_sym!(self, xcb_selinux_set_property_use_context_context_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `context` field of a `xcb_selinux_set_property_use_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_set_property_use_context_context_end(
        &self,
        r: *const xcb_selinux_set_property_use_context_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_selinux_set_property_use_context_context_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_property_use_context_context_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_property_use_context_context_end(&self) -> bool {
        has_sym!(self, xcb_selinux_set_property_use_context_context_end)
    }

    /// Computes the size of a `xcb_selinux_get_property_use_context_reply_t` object.
    #[inline]
    pub unsafe fn xcb_selinux_get_property_use_context_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_selinux_get_property_use_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_use_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_use_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_use_context_sizeof)
    }

    /// Sends a `SELinux::GetPropertyUseContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_property_use_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_property_use_context_reply`]: Self::xcb_selinux_get_property_use_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_property_use_context(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_get_property_use_context_cookie_t {
        sym!(self, xcb_selinux_get_property_use_context)(c)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_use_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_use_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_use_context)
    }

    /// Sends a `SELinux::GetPropertyUseContext` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_property_use_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_property_use_context_reply`]: Self::xcb_selinux_get_property_use_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_property_use_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_get_property_use_context_cookie_t {
        sym!(self, xcb_selinux_get_property_use_context_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_use_context_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_use_context_unchecked(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_use_context_unchecked)
    }

    /// Returns a pointer to the `context` field of a `xcb_selinux_get_property_use_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_property_use_context_context(
        &self,
        r: *const xcb_selinux_get_property_use_context_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_selinux_get_property_use_context_context)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_use_context_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_use_context_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_use_context_context)
    }

    /// Returns the number of elements of the `context` field of a `xcb_selinux_get_property_use_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_property_use_context_context_length(
        &self,
        r: *const xcb_selinux_get_property_use_context_reply_t,
    ) -> c_int {
        sym!(self, xcb_selinux_get_property_use_context_context_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_use_context_context_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_use_context_context_length(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_use_context_context_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `context` field of a `xcb_selinux_get_property_use_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_property_use_context_context_end(
        &self,
        r: *const xcb_selinux_get_property_use_context_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_selinux_get_property_use_context_context_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_use_context_context_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_use_context_context_end(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_use_context_context_end)
    }

    /// Waits for the reply to a `SELinux::GetPropertyUseContext` request.
    #[inline]
    pub unsafe fn xcb_selinux_get_property_use_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_property_use_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_property_use_context_reply_t {
        sym!(self, xcb_selinux_get_property_use_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_use_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_use_context_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_use_context_reply)
    }

    /// Computes the size of a `xcb_selinux_get_property_context_reply_t` object.
    #[inline]
    pub unsafe fn xcb_selinux_get_property_context_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_selinux_get_property_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_context_sizeof)
    }

    /// Sends a `SELinux::GetPropertyContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_property_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_property_context_reply`]: Self::xcb_selinux_get_property_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_property_context(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        property: xcb_atom_t,
    ) -> xcb_selinux_get_property_context_cookie_t {
        sym!(self, xcb_selinux_get_property_context)(c, window, property)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_context)
    }

    /// Sends a `SELinux::GetPropertyContext` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_property_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_property_context_reply`]: Self::xcb_selinux_get_property_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_property_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        property: xcb_atom_t,
    ) -> xcb_selinux_get_property_context_cookie_t {
        sym!(self, xcb_selinux_get_property_context_unchecked)(c, window, property)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_context_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_context_unchecked(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_context_unchecked)
    }

    /// Returns a pointer to the `context` field of a `xcb_selinux_get_property_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_property_context_context(
        &self,
        r: *const xcb_selinux_get_property_context_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_selinux_get_property_context_context)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_context_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_context_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_context_context)
    }

    /// Returns the number of elements of the `context` field of a `xcb_selinux_get_property_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_property_context_context_length(
        &self,
        r: *const xcb_selinux_get_property_context_reply_t,
    ) -> c_int {
        sym!(self, xcb_selinux_get_property_context_context_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_context_context_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_context_context_length(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_context_context_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `context` field of a `xcb_selinux_get_property_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_property_context_context_end(
        &self,
        r: *const xcb_selinux_get_property_context_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_selinux_get_property_context_context_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_context_context_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_context_context_end(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_context_context_end)
    }

    /// Waits for the reply to a `SELinux::GetPropertyContext` request.
    #[inline]
    pub unsafe fn xcb_selinux_get_property_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_property_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_property_context_reply_t {
        sym!(self, xcb_selinux_get_property_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_context_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_context_reply)
    }

    /// Computes the size of a `xcb_selinux_get_property_data_context_reply_t` object.
    #[inline]
    pub unsafe fn xcb_selinux_get_property_data_context_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_selinux_get_property_data_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_data_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_data_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_data_context_sizeof)
    }

    /// Sends a `SELinux::GetPropertyDataContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_property_data_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_property_data_context_reply`]: Self::xcb_selinux_get_property_data_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_property_data_context(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        property: xcb_atom_t,
    ) -> xcb_selinux_get_property_data_context_cookie_t {
        sym!(self, xcb_selinux_get_property_data_context)(c, window, property)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_data_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_data_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_data_context)
    }

    /// Sends a `SELinux::GetPropertyDataContext` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_property_data_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_property_data_context_reply`]: Self::xcb_selinux_get_property_data_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_property_data_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        property: xcb_atom_t,
    ) -> xcb_selinux_get_property_data_context_cookie_t {
        sym!(self, xcb_selinux_get_property_data_context_unchecked)(c, window, property)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_data_context_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_data_context_unchecked(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_data_context_unchecked)
    }

    /// Returns a pointer to the `context` field of a `xcb_selinux_get_property_data_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_property_data_context_context(
        &self,
        r: *const xcb_selinux_get_property_data_context_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_selinux_get_property_data_context_context)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_data_context_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_data_context_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_data_context_context)
    }

    /// Returns the number of elements of the `context` field of a `xcb_selinux_get_property_data_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_property_data_context_context_length(
        &self,
        r: *const xcb_selinux_get_property_data_context_reply_t,
    ) -> c_int {
        sym!(self, xcb_selinux_get_property_data_context_context_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_data_context_context_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_data_context_context_length(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_data_context_context_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `context` field of a `xcb_selinux_get_property_data_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_property_data_context_context_end(
        &self,
        r: *const xcb_selinux_get_property_data_context_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_selinux_get_property_data_context_context_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_data_context_context_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_data_context_context_end(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_data_context_context_end)
    }

    /// Waits for the reply to a `SELinux::GetPropertyDataContext` request.
    #[inline]
    pub unsafe fn xcb_selinux_get_property_data_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_property_data_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_property_data_context_reply_t {
        sym!(self, xcb_selinux_get_property_data_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_data_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_data_context_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_data_context_reply)
    }

    /// Computes the size of a `xcb_selinux_list_properties_reply_t` object.
    #[inline]
    pub unsafe fn xcb_selinux_list_properties_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_selinux_list_properties_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_properties_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_properties_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_list_properties_sizeof)
    }

    /// Sends a `SELinux::ListProperties` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_list_properties_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_list_properties_reply`]: Self::xcb_selinux_list_properties_reply
    #[inline]
    pub unsafe fn xcb_selinux_list_properties(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_selinux_list_properties_cookie_t {
        sym!(self, xcb_selinux_list_properties)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_properties` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_properties(&self) -> bool {
        has_sym!(self, xcb_selinux_list_properties)
    }

    /// Sends a `SELinux::ListProperties` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_list_properties_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_list_properties_reply`]: Self::xcb_selinux_list_properties_reply
    #[inline]
    pub unsafe fn xcb_selinux_list_properties_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_selinux_list_properties_cookie_t {
        sym!(self, xcb_selinux_list_properties_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_properties_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_properties_unchecked(&self) -> bool {
        has_sym!(self, xcb_selinux_list_properties_unchecked)
    }

    /// Returns the number of elements of the `properties` field of a `xcb_selinux_list_properties_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_list_properties_properties_length(
        &self,
        r: *const xcb_selinux_list_properties_reply_t,
    ) -> c_int {
        sym!(self, xcb_selinux_list_properties_properties_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_properties_properties_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_properties_properties_length(&self) -> bool {
        has_sym!(self, xcb_selinux_list_properties_properties_length)
    }

    /// Returns an iterator over the elements of the
    /// `properties` field of a `xcb_selinux_list_properties_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_list_properties_properties_iterator(
        &self,
        r: *const xcb_selinux_list_properties_reply_t,
    ) -> xcb_selinux_list_item_iterator_t {
        sym!(self, xcb_selinux_list_properties_properties_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_properties_properties_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_properties_properties_iterator(&self) -> bool {
        has_sym!(self, xcb_selinux_list_properties_properties_iterator)
    }

    /// Waits for the reply to a `SELinux::ListProperties` request.
    #[inline]
    pub unsafe fn xcb_selinux_list_properties_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_list_properties_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_list_properties_reply_t {
        sym!(self, xcb_selinux_list_properties_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_properties_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_properties_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_list_properties_reply)
    }

    /// Computes the size of a `xcb_selinux_set_selection_create_context_request_t` object.
    #[inline]
    pub unsafe fn xcb_selinux_set_selection_create_context_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_selinux_set_selection_create_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_selection_create_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_selection_create_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_set_selection_create_context_sizeof)
    }

    /// Sends a `SELinux::SetSelectionCreateContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_selinux_set_selection_create_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_selinux_set_selection_create_context_checked)(c, context_len, context)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_selection_create_context_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_selection_create_context_checked(&self) -> bool {
        has_sym!(self, xcb_selinux_set_selection_create_context_checked)
    }

    /// Sends a `SELinux::SetSelectionCreateContext` request (unchecked).
    #[inline]
    pub unsafe fn xcb_selinux_set_selection_create_context(
        &self,
        c: *mut xcb_connection_t,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_selinux_set_selection_create_context)(c, context_len, context)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_selection_create_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_selection_create_context(&self) -> bool {
        has_sym!(self, xcb_selinux_set_selection_create_context)
    }

    /// Returns a pointer to the `context` field of a `xcb_selinux_set_selection_create_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_set_selection_create_context_context(
        &self,
        r: *const xcb_selinux_set_selection_create_context_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_selinux_set_selection_create_context_context)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_selection_create_context_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_selection_create_context_context(&self) -> bool {
        has_sym!(self, xcb_selinux_set_selection_create_context_context)
    }

    /// Returns the number of elements of the `context` field of a `xcb_selinux_set_selection_create_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_set_selection_create_context_context_length(
        &self,
        r: *const xcb_selinux_set_selection_create_context_request_t,
    ) -> c_int {
        sym!(
            self,
            xcb_selinux_set_selection_create_context_context_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_selection_create_context_context_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_selection_create_context_context_length(&self) -> bool {
        has_sym!(
            self,
            xcb_selinux_set_selection_create_context_context_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `context` field of a `xcb_selinux_set_selection_create_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_set_selection_create_context_context_end(
        &self,
        r: *const xcb_selinux_set_selection_create_context_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_selinux_set_selection_create_context_context_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_selection_create_context_context_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_selection_create_context_context_end(&self) -> bool {
        has_sym!(self, xcb_selinux_set_selection_create_context_context_end)
    }

    /// Computes the size of a `xcb_selinux_get_selection_create_context_reply_t` object.
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_create_context_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_selinux_get_selection_create_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_create_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_create_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_create_context_sizeof)
    }

    /// Sends a `SELinux::GetSelectionCreateContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_selection_create_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_selection_create_context_reply`]: Self::xcb_selinux_get_selection_create_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_create_context(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_get_selection_create_context_cookie_t {
        sym!(self, xcb_selinux_get_selection_create_context)(c)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_create_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_create_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_create_context)
    }

    /// Sends a `SELinux::GetSelectionCreateContext` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_selection_create_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_selection_create_context_reply`]: Self::xcb_selinux_get_selection_create_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_create_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_get_selection_create_context_cookie_t {
        sym!(self, xcb_selinux_get_selection_create_context_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_create_context_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_create_context_unchecked(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_create_context_unchecked)
    }

    /// Returns a pointer to the `context` field of a `xcb_selinux_get_selection_create_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_create_context_context(
        &self,
        r: *const xcb_selinux_get_selection_create_context_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_selinux_get_selection_create_context_context)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_create_context_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_create_context_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_create_context_context)
    }

    /// Returns the number of elements of the `context` field of a `xcb_selinux_get_selection_create_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_create_context_context_length(
        &self,
        r: *const xcb_selinux_get_selection_create_context_reply_t,
    ) -> c_int {
        sym!(
            self,
            xcb_selinux_get_selection_create_context_context_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_create_context_context_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_create_context_context_length(&self) -> bool {
        has_sym!(
            self,
            xcb_selinux_get_selection_create_context_context_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `context` field of a `xcb_selinux_get_selection_create_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_create_context_context_end(
        &self,
        r: *const xcb_selinux_get_selection_create_context_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_selinux_get_selection_create_context_context_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_create_context_context_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_create_context_context_end(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_create_context_context_end)
    }

    /// Waits for the reply to a `SELinux::GetSelectionCreateContext` request.
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_create_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_selection_create_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_selection_create_context_reply_t {
        sym!(self, xcb_selinux_get_selection_create_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_create_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_create_context_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_create_context_reply)
    }

    /// Computes the size of a `xcb_selinux_set_selection_use_context_request_t` object.
    #[inline]
    pub unsafe fn xcb_selinux_set_selection_use_context_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_selinux_set_selection_use_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_selection_use_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_selection_use_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_set_selection_use_context_sizeof)
    }

    /// Sends a `SELinux::SetSelectionUseContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_selinux_set_selection_use_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_selinux_set_selection_use_context_checked)(c, context_len, context)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_selection_use_context_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_selection_use_context_checked(&self) -> bool {
        has_sym!(self, xcb_selinux_set_selection_use_context_checked)
    }

    /// Sends a `SELinux::SetSelectionUseContext` request (unchecked).
    #[inline]
    pub unsafe fn xcb_selinux_set_selection_use_context(
        &self,
        c: *mut xcb_connection_t,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_selinux_set_selection_use_context)(c, context_len, context)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_selection_use_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_selection_use_context(&self) -> bool {
        has_sym!(self, xcb_selinux_set_selection_use_context)
    }

    /// Returns a pointer to the `context` field of a `xcb_selinux_set_selection_use_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_set_selection_use_context_context(
        &self,
        r: *const xcb_selinux_set_selection_use_context_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_selinux_set_selection_use_context_context)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_selection_use_context_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_selection_use_context_context(&self) -> bool {
        has_sym!(self, xcb_selinux_set_selection_use_context_context)
    }

    /// Returns the number of elements of the `context` field of a `xcb_selinux_set_selection_use_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_set_selection_use_context_context_length(
        &self,
        r: *const xcb_selinux_set_selection_use_context_request_t,
    ) -> c_int {
        sym!(self, xcb_selinux_set_selection_use_context_context_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_selection_use_context_context_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_selection_use_context_context_length(&self) -> bool {
        has_sym!(self, xcb_selinux_set_selection_use_context_context_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `context` field of a `xcb_selinux_set_selection_use_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_set_selection_use_context_context_end(
        &self,
        r: *const xcb_selinux_set_selection_use_context_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_selinux_set_selection_use_context_context_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_selection_use_context_context_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_selection_use_context_context_end(&self) -> bool {
        has_sym!(self, xcb_selinux_set_selection_use_context_context_end)
    }

    /// Computes the size of a `xcb_selinux_get_selection_use_context_reply_t` object.
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_use_context_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_selinux_get_selection_use_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_use_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_use_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_use_context_sizeof)
    }

    /// Sends a `SELinux::GetSelectionUseContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_selection_use_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_selection_use_context_reply`]: Self::xcb_selinux_get_selection_use_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_use_context(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_get_selection_use_context_cookie_t {
        sym!(self, xcb_selinux_get_selection_use_context)(c)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_use_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_use_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_use_context)
    }

    /// Sends a `SELinux::GetSelectionUseContext` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_selection_use_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_selection_use_context_reply`]: Self::xcb_selinux_get_selection_use_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_use_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_get_selection_use_context_cookie_t {
        sym!(self, xcb_selinux_get_selection_use_context_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_use_context_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_use_context_unchecked(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_use_context_unchecked)
    }

    /// Returns a pointer to the `context` field of a `xcb_selinux_get_selection_use_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_use_context_context(
        &self,
        r: *const xcb_selinux_get_selection_use_context_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_selinux_get_selection_use_context_context)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_use_context_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_use_context_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_use_context_context)
    }

    /// Returns the number of elements of the `context` field of a `xcb_selinux_get_selection_use_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_use_context_context_length(
        &self,
        r: *const xcb_selinux_get_selection_use_context_reply_t,
    ) -> c_int {
        sym!(self, xcb_selinux_get_selection_use_context_context_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_use_context_context_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_use_context_context_length(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_use_context_context_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `context` field of a `xcb_selinux_get_selection_use_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_use_context_context_end(
        &self,
        r: *const xcb_selinux_get_selection_use_context_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_selinux_get_selection_use_context_context_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_use_context_context_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_use_context_context_end(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_use_context_context_end)
    }

    /// Waits for the reply to a `SELinux::GetSelectionUseContext` request.
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_use_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_selection_use_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_selection_use_context_reply_t {
        sym!(self, xcb_selinux_get_selection_use_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_use_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_use_context_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_use_context_reply)
    }

    /// Computes the size of a `xcb_selinux_get_selection_context_reply_t` object.
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_context_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_selinux_get_selection_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_context_sizeof)
    }

    /// Sends a `SELinux::GetSelectionContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_selection_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_selection_context_reply`]: Self::xcb_selinux_get_selection_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_context(
        &self,
        c: *mut xcb_connection_t,
        selection: xcb_atom_t,
    ) -> xcb_selinux_get_selection_context_cookie_t {
        sym!(self, xcb_selinux_get_selection_context)(c, selection)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_context)
    }

    /// Sends a `SELinux::GetSelectionContext` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_selection_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_selection_context_reply`]: Self::xcb_selinux_get_selection_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
        selection: xcb_atom_t,
    ) -> xcb_selinux_get_selection_context_cookie_t {
        sym!(self, xcb_selinux_get_selection_context_unchecked)(c, selection)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_context_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_context_unchecked(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_context_unchecked)
    }

    /// Returns a pointer to the `context` field of a `xcb_selinux_get_selection_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_context_context(
        &self,
        r: *const xcb_selinux_get_selection_context_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_selinux_get_selection_context_context)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_context_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_context_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_context_context)
    }

    /// Returns the number of elements of the `context` field of a `xcb_selinux_get_selection_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_context_context_length(
        &self,
        r: *const xcb_selinux_get_selection_context_reply_t,
    ) -> c_int {
        sym!(self, xcb_selinux_get_selection_context_context_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_context_context_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_context_context_length(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_context_context_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `context` field of a `xcb_selinux_get_selection_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_context_context_end(
        &self,
        r: *const xcb_selinux_get_selection_context_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_selinux_get_selection_context_context_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_context_context_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_context_context_end(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_context_context_end)
    }

    /// Waits for the reply to a `SELinux::GetSelectionContext` request.
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_selection_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_selection_context_reply_t {
        sym!(self, xcb_selinux_get_selection_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_context_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_context_reply)
    }

    /// Computes the size of a `xcb_selinux_get_selection_data_context_reply_t` object.
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_data_context_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_selinux_get_selection_data_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_data_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_data_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_data_context_sizeof)
    }

    /// Sends a `SELinux::GetSelectionDataContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_selection_data_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_selection_data_context_reply`]: Self::xcb_selinux_get_selection_data_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_data_context(
        &self,
        c: *mut xcb_connection_t,
        selection: xcb_atom_t,
    ) -> xcb_selinux_get_selection_data_context_cookie_t {
        sym!(self, xcb_selinux_get_selection_data_context)(c, selection)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_data_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_data_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_data_context)
    }

    /// Sends a `SELinux::GetSelectionDataContext` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_selection_data_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_selection_data_context_reply`]: Self::xcb_selinux_get_selection_data_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_data_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
        selection: xcb_atom_t,
    ) -> xcb_selinux_get_selection_data_context_cookie_t {
        sym!(self, xcb_selinux_get_selection_data_context_unchecked)(c, selection)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_data_context_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_data_context_unchecked(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_data_context_unchecked)
    }

    /// Returns a pointer to the `context` field of a `xcb_selinux_get_selection_data_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_data_context_context(
        &self,
        r: *const xcb_selinux_get_selection_data_context_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_selinux_get_selection_data_context_context)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_data_context_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_data_context_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_data_context_context)
    }

    /// Returns the number of elements of the `context` field of a `xcb_selinux_get_selection_data_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_data_context_context_length(
        &self,
        r: *const xcb_selinux_get_selection_data_context_reply_t,
    ) -> c_int {
        sym!(self, xcb_selinux_get_selection_data_context_context_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_data_context_context_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_data_context_context_length(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_data_context_context_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `context` field of a `xcb_selinux_get_selection_data_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_data_context_context_end(
        &self,
        r: *const xcb_selinux_get_selection_data_context_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_selinux_get_selection_data_context_context_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_data_context_context_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_data_context_context_end(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_data_context_context_end)
    }

    /// Waits for the reply to a `SELinux::GetSelectionDataContext` request.
    #[inline]
    pub unsafe fn xcb_selinux_get_selection_data_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_selection_data_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_selection_data_context_reply_t {
        sym!(self, xcb_selinux_get_selection_data_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_data_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_data_context_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_data_context_reply)
    }

    /// Computes the size of a `xcb_selinux_list_selections_reply_t` object.
    #[inline]
    pub unsafe fn xcb_selinux_list_selections_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_selinux_list_selections_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_selections_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_selections_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_list_selections_sizeof)
    }

    /// Sends a `SELinux::ListSelections` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_list_selections_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_list_selections_reply`]: Self::xcb_selinux_list_selections_reply
    #[inline]
    pub unsafe fn xcb_selinux_list_selections(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_list_selections_cookie_t {
        sym!(self, xcb_selinux_list_selections)(c)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_selections` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_selections(&self) -> bool {
        has_sym!(self, xcb_selinux_list_selections)
    }

    /// Sends a `SELinux::ListSelections` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_list_selections_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_list_selections_reply`]: Self::xcb_selinux_list_selections_reply
    #[inline]
    pub unsafe fn xcb_selinux_list_selections_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_list_selections_cookie_t {
        sym!(self, xcb_selinux_list_selections_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_selections_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_selections_unchecked(&self) -> bool {
        has_sym!(self, xcb_selinux_list_selections_unchecked)
    }

    /// Returns the number of elements of the `selections` field of a `xcb_selinux_list_selections_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_list_selections_selections_length(
        &self,
        r: *const xcb_selinux_list_selections_reply_t,
    ) -> c_int {
        sym!(self, xcb_selinux_list_selections_selections_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_selections_selections_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_selections_selections_length(&self) -> bool {
        has_sym!(self, xcb_selinux_list_selections_selections_length)
    }

    /// Returns an iterator over the elements of the
    /// `selections` field of a `xcb_selinux_list_selections_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_list_selections_selections_iterator(
        &self,
        r: *const xcb_selinux_list_selections_reply_t,
    ) -> xcb_selinux_list_item_iterator_t {
        sym!(self, xcb_selinux_list_selections_selections_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_selections_selections_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_selections_selections_iterator(&self) -> bool {
        has_sym!(self, xcb_selinux_list_selections_selections_iterator)
    }

    /// Waits for the reply to a `SELinux::ListSelections` request.
    #[inline]
    pub unsafe fn xcb_selinux_list_selections_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_list_selections_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_list_selections_reply_t {
        sym!(self, xcb_selinux_list_selections_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_selections_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_selections_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_list_selections_reply)
    }

    /// Computes the size of a `xcb_selinux_get_client_context_reply_t` object.
    #[inline]
    pub unsafe fn xcb_selinux_get_client_context_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_selinux_get_client_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_client_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_client_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_get_client_context_sizeof)
    }

    /// Sends a `SELinux::GetClientContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_client_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_client_context_reply`]: Self::xcb_selinux_get_client_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_client_context(
        &self,
        c: *mut xcb_connection_t,
        resource: u32,
    ) -> xcb_selinux_get_client_context_cookie_t {
        sym!(self, xcb_selinux_get_client_context)(c, resource)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_client_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_client_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_client_context)
    }

    /// Sends a `SELinux::GetClientContext` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_selinux_get_client_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_selinux_get_client_context_reply`]: Self::xcb_selinux_get_client_context_reply
    #[inline]
    pub unsafe fn xcb_selinux_get_client_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
        resource: u32,
    ) -> xcb_selinux_get_client_context_cookie_t {
        sym!(self, xcb_selinux_get_client_context_unchecked)(c, resource)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_client_context_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_client_context_unchecked(&self) -> bool {
        has_sym!(self, xcb_selinux_get_client_context_unchecked)
    }

    /// Returns a pointer to the `context` field of a `xcb_selinux_get_client_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_client_context_context(
        &self,
        r: *const xcb_selinux_get_client_context_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_selinux_get_client_context_context)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_client_context_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_client_context_context(&self) -> bool {
        has_sym!(self, xcb_selinux_get_client_context_context)
    }

    /// Returns the number of elements of the `context` field of a `xcb_selinux_get_client_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_client_context_context_length(
        &self,
        r: *const xcb_selinux_get_client_context_reply_t,
    ) -> c_int {
        sym!(self, xcb_selinux_get_client_context_context_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_client_context_context_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_client_context_context_length(&self) -> bool {
        has_sym!(self, xcb_selinux_get_client_context_context_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `context` field of a `xcb_selinux_get_client_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_selinux_get_client_context_context_end(
        &self,
        r: *const xcb_selinux_get_client_context_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_selinux_get_client_context_context_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_client_context_context_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_client_context_context_end(&self) -> bool {
        has_sym!(self, xcb_selinux_get_client_context_context_end)
    }

    /// Waits for the reply to a `SELinux::GetClientContext` request.
    #[inline]
    pub unsafe fn xcb_selinux_get_client_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_client_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_client_context_reply_t {
        sym!(self, xcb_selinux_get_client_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_client_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_client_context_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_get_client_context_reply)
    }
}

#[cfg(feature = "xcb_xselinux")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbXselinux::load().unwrap() };
        assert!(lib.has_xcb_selinux_id());
        assert!(lib.has_xcb_selinux_query_version());
        assert!(lib.has_xcb_selinux_query_version_unchecked());
        assert!(lib.has_xcb_selinux_query_version_reply());
        assert!(lib.has_xcb_selinux_set_device_create_context_sizeof());
        assert!(lib.has_xcb_selinux_set_device_create_context_checked());
        assert!(lib.has_xcb_selinux_set_device_create_context());
        assert!(lib.has_xcb_selinux_set_device_create_context_context());
        assert!(lib.has_xcb_selinux_set_device_create_context_context_length());
        assert!(lib.has_xcb_selinux_set_device_create_context_context_end());
        assert!(lib.has_xcb_selinux_get_device_create_context_sizeof());
        assert!(lib.has_xcb_selinux_get_device_create_context());
        assert!(lib.has_xcb_selinux_get_device_create_context_unchecked());
        assert!(lib.has_xcb_selinux_get_device_create_context_context());
        assert!(lib.has_xcb_selinux_get_device_create_context_context_length());
        assert!(lib.has_xcb_selinux_get_device_create_context_context_end());
        assert!(lib.has_xcb_selinux_get_device_create_context_reply());
        assert!(lib.has_xcb_selinux_set_device_context_sizeof());
        assert!(lib.has_xcb_selinux_set_device_context_checked());
        assert!(lib.has_xcb_selinux_set_device_context());
        assert!(lib.has_xcb_selinux_set_device_context_context());
        assert!(lib.has_xcb_selinux_set_device_context_context_length());
        assert!(lib.has_xcb_selinux_set_device_context_context_end());
        assert!(lib.has_xcb_selinux_get_device_context_sizeof());
        assert!(lib.has_xcb_selinux_get_device_context());
        assert!(lib.has_xcb_selinux_get_device_context_unchecked());
        assert!(lib.has_xcb_selinux_get_device_context_context());
        assert!(lib.has_xcb_selinux_get_device_context_context_length());
        assert!(lib.has_xcb_selinux_get_device_context_context_end());
        assert!(lib.has_xcb_selinux_get_device_context_reply());
        assert!(lib.has_xcb_selinux_set_window_create_context_sizeof());
        assert!(lib.has_xcb_selinux_set_window_create_context_checked());
        assert!(lib.has_xcb_selinux_set_window_create_context());
        assert!(lib.has_xcb_selinux_set_window_create_context_context());
        assert!(lib.has_xcb_selinux_set_window_create_context_context_length());
        assert!(lib.has_xcb_selinux_set_window_create_context_context_end());
        assert!(lib.has_xcb_selinux_get_window_create_context_sizeof());
        assert!(lib.has_xcb_selinux_get_window_create_context());
        assert!(lib.has_xcb_selinux_get_window_create_context_unchecked());
        assert!(lib.has_xcb_selinux_get_window_create_context_context());
        assert!(lib.has_xcb_selinux_get_window_create_context_context_length());
        assert!(lib.has_xcb_selinux_get_window_create_context_context_end());
        assert!(lib.has_xcb_selinux_get_window_create_context_reply());
        assert!(lib.has_xcb_selinux_get_window_context_sizeof());
        assert!(lib.has_xcb_selinux_get_window_context());
        assert!(lib.has_xcb_selinux_get_window_context_unchecked());
        assert!(lib.has_xcb_selinux_get_window_context_context());
        assert!(lib.has_xcb_selinux_get_window_context_context_length());
        assert!(lib.has_xcb_selinux_get_window_context_context_end());
        assert!(lib.has_xcb_selinux_get_window_context_reply());
        assert!(lib.has_xcb_selinux_list_item_sizeof());
        assert!(lib.has_xcb_selinux_list_item_object_context());
        assert!(lib.has_xcb_selinux_list_item_object_context_length());
        assert!(lib.has_xcb_selinux_list_item_object_context_end());
        assert!(lib.has_xcb_selinux_list_item_data_context());
        assert!(lib.has_xcb_selinux_list_item_data_context_length());
        assert!(lib.has_xcb_selinux_list_item_data_context_end());
        assert!(lib.has_xcb_selinux_list_item_next());
        assert!(lib.has_xcb_selinux_list_item_end());
        assert!(lib.has_xcb_selinux_set_property_create_context_sizeof());
        assert!(lib.has_xcb_selinux_set_property_create_context_checked());
        assert!(lib.has_xcb_selinux_set_property_create_context());
        assert!(lib.has_xcb_selinux_set_property_create_context_context());
        assert!(lib.has_xcb_selinux_set_property_create_context_context_length());
        assert!(lib.has_xcb_selinux_set_property_create_context_context_end());
        assert!(lib.has_xcb_selinux_get_property_create_context_sizeof());
        assert!(lib.has_xcb_selinux_get_property_create_context());
        assert!(lib.has_xcb_selinux_get_property_create_context_unchecked());
        assert!(lib.has_xcb_selinux_get_property_create_context_context());
        assert!(lib.has_xcb_selinux_get_property_create_context_context_length());
        assert!(lib.has_xcb_selinux_get_property_create_context_context_end());
        assert!(lib.has_xcb_selinux_get_property_create_context_reply());
        assert!(lib.has_xcb_selinux_set_property_use_context_sizeof());
        assert!(lib.has_xcb_selinux_set_property_use_context_checked());
        assert!(lib.has_xcb_selinux_set_property_use_context());
        assert!(lib.has_xcb_selinux_set_property_use_context_context());
        assert!(lib.has_xcb_selinux_set_property_use_context_context_length());
        assert!(lib.has_xcb_selinux_set_property_use_context_context_end());
        assert!(lib.has_xcb_selinux_get_property_use_context_sizeof());
        assert!(lib.has_xcb_selinux_get_property_use_context());
        assert!(lib.has_xcb_selinux_get_property_use_context_unchecked());
        assert!(lib.has_xcb_selinux_get_property_use_context_context());
        assert!(lib.has_xcb_selinux_get_property_use_context_context_length());
        assert!(lib.has_xcb_selinux_get_property_use_context_context_end());
        assert!(lib.has_xcb_selinux_get_property_use_context_reply());
        assert!(lib.has_xcb_selinux_get_property_context_sizeof());
        assert!(lib.has_xcb_selinux_get_property_context());
        assert!(lib.has_xcb_selinux_get_property_context_unchecked());
        assert!(lib.has_xcb_selinux_get_property_context_context());
        assert!(lib.has_xcb_selinux_get_property_context_context_length());
        assert!(lib.has_xcb_selinux_get_property_context_context_end());
        assert!(lib.has_xcb_selinux_get_property_context_reply());
        assert!(lib.has_xcb_selinux_get_property_data_context_sizeof());
        assert!(lib.has_xcb_selinux_get_property_data_context());
        assert!(lib.has_xcb_selinux_get_property_data_context_unchecked());
        assert!(lib.has_xcb_selinux_get_property_data_context_context());
        assert!(lib.has_xcb_selinux_get_property_data_context_context_length());
        assert!(lib.has_xcb_selinux_get_property_data_context_context_end());
        assert!(lib.has_xcb_selinux_get_property_data_context_reply());
        assert!(lib.has_xcb_selinux_list_properties_sizeof());
        assert!(lib.has_xcb_selinux_list_properties());
        assert!(lib.has_xcb_selinux_list_properties_unchecked());
        assert!(lib.has_xcb_selinux_list_properties_properties_length());
        assert!(lib.has_xcb_selinux_list_properties_properties_iterator());
        assert!(lib.has_xcb_selinux_list_properties_reply());
        assert!(lib.has_xcb_selinux_set_selection_create_context_sizeof());
        assert!(lib.has_xcb_selinux_set_selection_create_context_checked());
        assert!(lib.has_xcb_selinux_set_selection_create_context());
        assert!(lib.has_xcb_selinux_set_selection_create_context_context());
        assert!(lib.has_xcb_selinux_set_selection_create_context_context_length());
        assert!(lib.has_xcb_selinux_set_selection_create_context_context_end());
        assert!(lib.has_xcb_selinux_get_selection_create_context_sizeof());
        assert!(lib.has_xcb_selinux_get_selection_create_context());
        assert!(lib.has_xcb_selinux_get_selection_create_context_unchecked());
        assert!(lib.has_xcb_selinux_get_selection_create_context_context());
        assert!(lib.has_xcb_selinux_get_selection_create_context_context_length());
        assert!(lib.has_xcb_selinux_get_selection_create_context_context_end());
        assert!(lib.has_xcb_selinux_get_selection_create_context_reply());
        assert!(lib.has_xcb_selinux_set_selection_use_context_sizeof());
        assert!(lib.has_xcb_selinux_set_selection_use_context_checked());
        assert!(lib.has_xcb_selinux_set_selection_use_context());
        assert!(lib.has_xcb_selinux_set_selection_use_context_context());
        assert!(lib.has_xcb_selinux_set_selection_use_context_context_length());
        assert!(lib.has_xcb_selinux_set_selection_use_context_context_end());
        assert!(lib.has_xcb_selinux_get_selection_use_context_sizeof());
        assert!(lib.has_xcb_selinux_get_selection_use_context());
        assert!(lib.has_xcb_selinux_get_selection_use_context_unchecked());
        assert!(lib.has_xcb_selinux_get_selection_use_context_context());
        assert!(lib.has_xcb_selinux_get_selection_use_context_context_length());
        assert!(lib.has_xcb_selinux_get_selection_use_context_context_end());
        assert!(lib.has_xcb_selinux_get_selection_use_context_reply());
        assert!(lib.has_xcb_selinux_get_selection_context_sizeof());
        assert!(lib.has_xcb_selinux_get_selection_context());
        assert!(lib.has_xcb_selinux_get_selection_context_unchecked());
        assert!(lib.has_xcb_selinux_get_selection_context_context());
        assert!(lib.has_xcb_selinux_get_selection_context_context_length());
        assert!(lib.has_xcb_selinux_get_selection_context_context_end());
        assert!(lib.has_xcb_selinux_get_selection_context_reply());
        assert!(lib.has_xcb_selinux_get_selection_data_context_sizeof());
        assert!(lib.has_xcb_selinux_get_selection_data_context());
        assert!(lib.has_xcb_selinux_get_selection_data_context_unchecked());
        assert!(lib.has_xcb_selinux_get_selection_data_context_context());
        assert!(lib.has_xcb_selinux_get_selection_data_context_context_length());
        assert!(lib.has_xcb_selinux_get_selection_data_context_context_end());
        assert!(lib.has_xcb_selinux_get_selection_data_context_reply());
        assert!(lib.has_xcb_selinux_list_selections_sizeof());
        assert!(lib.has_xcb_selinux_list_selections());
        assert!(lib.has_xcb_selinux_list_selections_unchecked());
        assert!(lib.has_xcb_selinux_list_selections_selections_length());
        assert!(lib.has_xcb_selinux_list_selections_selections_iterator());
        assert!(lib.has_xcb_selinux_list_selections_reply());
        assert!(lib.has_xcb_selinux_get_client_context_sizeof());
        assert!(lib.has_xcb_selinux_get_client_context());
        assert!(lib.has_xcb_selinux_get_client_context_unchecked());
        assert!(lib.has_xcb_selinux_get_client_context_context());
        assert!(lib.has_xcb_selinux_get_client_context_context_length());
        assert!(lib.has_xcb_selinux_get_client_context_context_end());
        assert!(lib.has_xcb_selinux_get_client_context_reply());
    }
}
