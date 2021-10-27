// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The name of the `Record` extension.
pub const XCB_RECORD_NAME: &[u8] = b"RECORD";

/// The name of the `Record` extension.
pub const XCB_RECORD_NAME_STR: &str = "RECORD";

/// The `Record::CONTEXT` type.
pub type xcb_record_context_t = u32;

/// An iterator over `Record::CONTEXT` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_context_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_record_context_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_record_context_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Record::Range8` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_range_8_t {
    pub first: u8,
    pub last: u8,
}

impl Default for xcb_record_range_8_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Record::Range8` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_range_8_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_record_range_8_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_record_range_8_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Record::Range16` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_range_16_t {
    pub first: u16,
    pub last: u16,
}

impl Default for xcb_record_range_16_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Record::Range16` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_range_16_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_record_range_16_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_record_range_16_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Record::ExtRange` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_ext_range_t {
    pub major: xcb_record_range_8_t,
    pub minor: xcb_record_range_16_t,
}

impl Default for xcb_record_ext_range_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Record::ExtRange` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_ext_range_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_record_ext_range_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_record_ext_range_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Record::Range` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_range_t {
    pub core_requests: xcb_record_range_8_t,
    pub core_replies: xcb_record_range_8_t,
    pub ext_requests: xcb_record_ext_range_t,
    pub ext_replies: xcb_record_ext_range_t,
    pub delivered_events: xcb_record_range_8_t,
    pub device_events: xcb_record_range_8_t,
    pub errors: xcb_record_range_8_t,
    pub client_started: u8,
    pub client_died: u8,
}

impl Default for xcb_record_range_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Record::Range` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_range_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_record_range_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_record_range_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Record::ElementHeader` type.
pub type xcb_record_element_header_t = u8;

/// An iterator over `Record::ElementHeader` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_element_header_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_record_element_header_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_record_element_header_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Record::HType` enum.
///
/// This enum has the following variants:
///
/// - [`Record::HType::FromServerTime`](XCB_RECORD_H_TYPE_FROM_SERVER_TIME)
/// - [`Record::HType::FromClientTime`](XCB_RECORD_H_TYPE_FROM_CLIENT_TIME)
/// - [`Record::HType::FromClientSequence`](XCB_RECORD_H_TYPE_FROM_CLIENT_SEQUENCE)
pub type xcb_record_h_type_t = u32;
/// The `Record::HType::FromServerTime` enum variant.
///
/// This is a variant of [`xcb_record_h_type_t`].
pub const XCB_RECORD_H_TYPE_FROM_SERVER_TIME: xcb_record_h_type_t = 1;
/// The `Record::HType::FromClientTime` enum variant.
///
/// This is a variant of [`xcb_record_h_type_t`].
pub const XCB_RECORD_H_TYPE_FROM_CLIENT_TIME: xcb_record_h_type_t = 2;
/// The `Record::HType::FromClientSequence` enum variant.
///
/// This is a variant of [`xcb_record_h_type_t`].
pub const XCB_RECORD_H_TYPE_FROM_CLIENT_SEQUENCE: xcb_record_h_type_t = 4;

/// The `Record::ClientSpec` type.
pub type xcb_record_client_spec_t = u32;

/// An iterator over `Record::ClientSpec` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_client_spec_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_record_client_spec_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_record_client_spec_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Record::CS` enum.
///
/// This enum has the following variants:
///
/// - [`Record::CS::CurrentClients`](XCB_RECORD_CS_CURRENT_CLIENTS)
/// - [`Record::CS::FutureClients`](XCB_RECORD_CS_FUTURE_CLIENTS)
/// - [`Record::CS::AllClients`](XCB_RECORD_CS_ALL_CLIENTS)
pub type xcb_record_cs_t = u32;
/// The `Record::CS::CurrentClients` enum variant.
///
/// This is a variant of [`xcb_record_cs_t`].
pub const XCB_RECORD_CS_CURRENT_CLIENTS: xcb_record_cs_t = 1;
/// The `Record::CS::FutureClients` enum variant.
///
/// This is a variant of [`xcb_record_cs_t`].
pub const XCB_RECORD_CS_FUTURE_CLIENTS: xcb_record_cs_t = 2;
/// The `Record::CS::AllClients` enum variant.
///
/// This is a variant of [`xcb_record_cs_t`].
pub const XCB_RECORD_CS_ALL_CLIENTS: xcb_record_cs_t = 3;

/// The `Record::ClientInfo` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `ranges`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_client_info_t {
    pub client_resource: xcb_record_client_spec_t,
    pub num_ranges: u32,
}

impl Default for xcb_record_client_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Record::ClientInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_client_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_record_client_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_record_client_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Record::BadContext` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_record_bad_context_error_t`].
pub const XCB_RECORD_BAD_CONTEXT: u8 = 0i32 as u8;

/// The `Record::BadContext` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_bad_context_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub invalid_record: u32,
}

impl Default for xcb_record_bad_context_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Record::QueryVersion` request.
///
/// Pass this cookie to [`xcb_record_query_version_reply`] to retrieve the reply.
///
/// [`xcb_record_query_version_reply`]: XcbRecord::xcb_record_query_version_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_query_version_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_record_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Record::QueryVersion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRecord::xcb_record_id()`], then the type of the request is
/// [`xcb_record_query_version_request_t`].
pub const XCB_RECORD_QUERY_VERSION: u8 = 0i32 as u8;

/// The `Record::QueryVersion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub major_version: u16,
    pub minor_version: u16,
}

impl Default for xcb_record_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Record::QueryVersion` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u16,
    pub minor_version: u16,
}

impl Default for xcb_record_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Record::CreateContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRecord::xcb_record_id()`], then the type of the request is
/// [`xcb_record_create_context_request_t`].
pub const XCB_RECORD_CREATE_CONTEXT: u8 = 1i32 as u8;

/// The `Record::CreateContext` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `client_specs`
/// - `ranges`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_create_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_record_context_t,
    pub element_header: xcb_record_element_header_t,
    pub pad0: [u8; 3],
    pub num_client_specs: u32,
    pub num_ranges: u32,
}

impl Default for xcb_record_create_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Record::RegisterClients` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRecord::xcb_record_id()`], then the type of the request is
/// [`xcb_record_register_clients_request_t`].
pub const XCB_RECORD_REGISTER_CLIENTS: u8 = 2i32 as u8;

/// The `Record::RegisterClients` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `client_specs`
/// - `ranges`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_register_clients_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_record_context_t,
    pub element_header: xcb_record_element_header_t,
    pub pad0: [u8; 3],
    pub num_client_specs: u32,
    pub num_ranges: u32,
}

impl Default for xcb_record_register_clients_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Record::UnregisterClients` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRecord::xcb_record_id()`], then the type of the request is
/// [`xcb_record_unregister_clients_request_t`].
pub const XCB_RECORD_UNREGISTER_CLIENTS: u8 = 3i32 as u8;

/// The `Record::UnregisterClients` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `client_specs`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_unregister_clients_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_record_context_t,
    pub num_client_specs: u32,
}

impl Default for xcb_record_unregister_clients_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Record::GetContext` request.
///
/// Pass this cookie to [`xcb_record_get_context_reply`] to retrieve the reply.
///
/// [`xcb_record_get_context_reply`]: XcbRecord::xcb_record_get_context_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_get_context_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_record_get_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Record::GetContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRecord::xcb_record_id()`], then the type of the request is
/// [`xcb_record_get_context_request_t`].
pub const XCB_RECORD_GET_CONTEXT: u8 = 4i32 as u8;

/// The `Record::GetContext` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_get_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_record_context_t,
}

impl Default for xcb_record_get_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Record::GetContext` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `intercepted_clients`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_get_context_reply_t {
    pub response_type: u8,
    pub enabled: u8,
    pub sequence: u16,
    pub length: u32,
    pub element_header: xcb_record_element_header_t,
    pub pad0: [u8; 3],
    pub num_intercepted_clients: u32,
    pub pad1: [u8; 16],
}

impl Default for xcb_record_get_context_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Record::EnableContext` request.
///
/// Pass this cookie to [`xcb_record_enable_context_reply`] to retrieve the reply.
///
/// [`xcb_record_enable_context_reply`]: XcbRecord::xcb_record_enable_context_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_enable_context_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_record_enable_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Record::EnableContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRecord::xcb_record_id()`], then the type of the request is
/// [`xcb_record_enable_context_request_t`].
pub const XCB_RECORD_ENABLE_CONTEXT: u8 = 5i32 as u8;

/// The `Record::EnableContext` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_enable_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_record_context_t,
}

impl Default for xcb_record_enable_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Record::EnableContext` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `data`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_enable_context_reply_t {
    pub response_type: u8,
    pub category: u8,
    pub sequence: u16,
    pub length: u32,
    pub element_header: xcb_record_element_header_t,
    pub client_swapped: u8,
    pub pad0: [u8; 2],
    pub xid_base: u32,
    pub server_time: u32,
    pub rec_sequence_num: u32,
    pub pad1: [u8; 8],
}

impl Default for xcb_record_enable_context_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Record::DisableContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRecord::xcb_record_id()`], then the type of the request is
/// [`xcb_record_disable_context_request_t`].
pub const XCB_RECORD_DISABLE_CONTEXT: u8 = 6i32 as u8;

/// The `Record::DisableContext` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_disable_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_record_context_t,
}

impl Default for xcb_record_disable_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Record::FreeContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRecord::xcb_record_id()`], then the type of the request is
/// [`xcb_record_free_context_request_t`].
pub const XCB_RECORD_FREE_CONTEXT: u8 = 7i32 as u8;

/// The `Record::FreeContext` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_free_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_record_context_t,
}

impl Default for xcb_record_free_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_record")]
pub(crate) struct XcbRecordRecord {
    xcb_record_id: LazySymbol<*mut xcb_extension_t>,
    xcb_record_context_next: LazySymbol<unsafe fn(i: *mut xcb_record_context_iterator_t)>,
    xcb_record_context_end:
        LazySymbol<unsafe fn(i: xcb_record_context_iterator_t) -> xcb_generic_iterator_t>,
    xcb_record_range_8_next: LazySymbol<unsafe fn(i: *mut xcb_record_range_8_iterator_t)>,
    xcb_record_range_8_end:
        LazySymbol<unsafe fn(i: xcb_record_range_8_iterator_t) -> xcb_generic_iterator_t>,
    xcb_record_range_16_next: LazySymbol<unsafe fn(i: *mut xcb_record_range_16_iterator_t)>,
    xcb_record_range_16_end:
        LazySymbol<unsafe fn(i: xcb_record_range_16_iterator_t) -> xcb_generic_iterator_t>,
    xcb_record_ext_range_next: LazySymbol<unsafe fn(i: *mut xcb_record_ext_range_iterator_t)>,
    xcb_record_ext_range_end:
        LazySymbol<unsafe fn(i: xcb_record_ext_range_iterator_t) -> xcb_generic_iterator_t>,
    xcb_record_range_next: LazySymbol<unsafe fn(i: *mut xcb_record_range_iterator_t)>,
    xcb_record_range_end:
        LazySymbol<unsafe fn(i: xcb_record_range_iterator_t) -> xcb_generic_iterator_t>,
    xcb_record_element_header_next:
        LazySymbol<unsafe fn(i: *mut xcb_record_element_header_iterator_t)>,
    xcb_record_element_header_end:
        LazySymbol<unsafe fn(i: xcb_record_element_header_iterator_t) -> xcb_generic_iterator_t>,
    xcb_record_client_spec_next: LazySymbol<unsafe fn(i: *mut xcb_record_client_spec_iterator_t)>,
    xcb_record_client_spec_end:
        LazySymbol<unsafe fn(i: xcb_record_client_spec_iterator_t) -> xcb_generic_iterator_t>,
    xcb_record_client_info_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_record_client_info_ranges:
        LazySymbol<unsafe fn(r: *const xcb_record_client_info_t) -> *mut xcb_record_range_t>,
    xcb_record_client_info_ranges_length:
        LazySymbol<unsafe fn(r: *const xcb_record_client_info_t) -> c_int>,
    xcb_record_client_info_ranges_iterator:
        LazySymbol<unsafe fn(r: *const xcb_record_client_info_t) -> xcb_record_range_iterator_t>,
    xcb_record_client_info_next: LazySymbol<unsafe fn(i: *mut xcb_record_client_info_iterator_t)>,
    xcb_record_client_info_end:
        LazySymbol<unsafe fn(i: xcb_record_client_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_record_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u16,
            minor_version: u16,
        ) -> xcb_record_query_version_cookie_t,
    >,
    xcb_record_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u16,
            minor_version: u16,
        ) -> xcb_record_query_version_cookie_t,
    >,
    xcb_record_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_record_query_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_record_query_version_reply_t,
    >,
    xcb_record_create_context_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_record_create_context_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_record_context_t,
            element_header: xcb_record_element_header_t,
            num_client_specs: u32,
            num_ranges: u32,
            client_specs: *const xcb_record_client_spec_t,
            ranges: *const xcb_record_range_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_record_create_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_record_context_t,
            element_header: xcb_record_element_header_t,
            num_client_specs: u32,
            num_ranges: u32,
            client_specs: *const xcb_record_client_spec_t,
            ranges: *const xcb_record_range_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_record_create_context_client_specs: LazySymbol<
        unsafe fn(r: *const xcb_record_create_context_request_t) -> *mut xcb_record_client_spec_t,
    >,
    xcb_record_create_context_client_specs_length:
        LazySymbol<unsafe fn(r: *const xcb_record_create_context_request_t) -> c_int>,
    xcb_record_create_context_client_specs_end: LazySymbol<
        unsafe fn(r: *const xcb_record_create_context_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_record_create_context_ranges: LazySymbol<
        unsafe fn(r: *const xcb_record_create_context_request_t) -> *mut xcb_record_range_t,
    >,
    xcb_record_create_context_ranges_length:
        LazySymbol<unsafe fn(r: *const xcb_record_create_context_request_t) -> c_int>,
    xcb_record_create_context_ranges_iterator: LazySymbol<
        unsafe fn(r: *const xcb_record_create_context_request_t) -> xcb_record_range_iterator_t,
    >,
    xcb_record_register_clients_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_record_register_clients_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_record_context_t,
            element_header: xcb_record_element_header_t,
            num_client_specs: u32,
            num_ranges: u32,
            client_specs: *const xcb_record_client_spec_t,
            ranges: *const xcb_record_range_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_record_register_clients: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_record_context_t,
            element_header: xcb_record_element_header_t,
            num_client_specs: u32,
            num_ranges: u32,
            client_specs: *const xcb_record_client_spec_t,
            ranges: *const xcb_record_range_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_record_register_clients_client_specs: LazySymbol<
        unsafe fn(r: *const xcb_record_register_clients_request_t) -> *mut xcb_record_client_spec_t,
    >,
    xcb_record_register_clients_client_specs_length:
        LazySymbol<unsafe fn(r: *const xcb_record_register_clients_request_t) -> c_int>,
    xcb_record_register_clients_client_specs_end: LazySymbol<
        unsafe fn(r: *const xcb_record_register_clients_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_record_register_clients_ranges: LazySymbol<
        unsafe fn(r: *const xcb_record_register_clients_request_t) -> *mut xcb_record_range_t,
    >,
    xcb_record_register_clients_ranges_length:
        LazySymbol<unsafe fn(r: *const xcb_record_register_clients_request_t) -> c_int>,
    xcb_record_register_clients_ranges_iterator: LazySymbol<
        unsafe fn(r: *const xcb_record_register_clients_request_t) -> xcb_record_range_iterator_t,
    >,
    xcb_record_unregister_clients_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_record_unregister_clients_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_record_context_t,
            num_client_specs: u32,
            client_specs: *const xcb_record_client_spec_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_record_unregister_clients: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_record_context_t,
            num_client_specs: u32,
            client_specs: *const xcb_record_client_spec_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_record_unregister_clients_client_specs: LazySymbol<
        unsafe fn(
            r: *const xcb_record_unregister_clients_request_t,
        ) -> *mut xcb_record_client_spec_t,
    >,
    xcb_record_unregister_clients_client_specs_length:
        LazySymbol<unsafe fn(r: *const xcb_record_unregister_clients_request_t) -> c_int>,
    xcb_record_unregister_clients_client_specs_end: LazySymbol<
        unsafe fn(r: *const xcb_record_unregister_clients_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_record_get_context_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_record_get_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_record_context_t,
        ) -> xcb_record_get_context_cookie_t,
    >,
    xcb_record_get_context_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_record_context_t,
        ) -> xcb_record_get_context_cookie_t,
    >,
    xcb_record_get_context_intercepted_clients_length:
        LazySymbol<unsafe fn(r: *const xcb_record_get_context_reply_t) -> c_int>,
    xcb_record_get_context_intercepted_clients_iterator: LazySymbol<
        unsafe fn(r: *const xcb_record_get_context_reply_t) -> xcb_record_client_info_iterator_t,
    >,
    xcb_record_get_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_record_get_context_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_record_get_context_reply_t,
    >,
    xcb_record_enable_context_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_record_enable_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_record_context_t,
        ) -> xcb_record_enable_context_cookie_t,
    >,
    xcb_record_enable_context_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_record_context_t,
        ) -> xcb_record_enable_context_cookie_t,
    >,
    xcb_record_enable_context_data:
        LazySymbol<unsafe fn(r: *const xcb_record_enable_context_reply_t) -> *mut u8>,
    xcb_record_enable_context_data_length:
        LazySymbol<unsafe fn(r: *const xcb_record_enable_context_reply_t) -> c_int>,
    xcb_record_enable_context_data_end: LazySymbol<
        unsafe fn(r: *const xcb_record_enable_context_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_record_enable_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_record_enable_context_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_record_enable_context_reply_t,
    >,
    xcb_record_disable_context_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, context: xcb_record_context_t) -> xcb_void_cookie_t,
    >,
    xcb_record_disable_context: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, context: xcb_record_context_t) -> xcb_void_cookie_t,
    >,
    xcb_record_free_context_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, context: xcb_record_context_t) -> xcb_void_cookie_t,
    >,
    xcb_record_free_context: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, context: xcb_record_context_t) -> xcb_void_cookie_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self
            .record
            .$f
            .get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .record
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_record")]
impl XcbRecord {
    /// The libxcb identifier of the `Record` extension.
    #[inline]
    pub fn xcb_record_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_record_id) }
    }

    /// Returns `true` iff the symbol `xcb_record_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_id(&self) -> bool {
        has_sym!(self, xcb_record_id)
    }

    /// Advances a `xcb_record_context_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_record_context_next(&self, i: *mut xcb_record_context_iterator_t) {
        sym!(self, xcb_record_context_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_record_context_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_context_next(&self) -> bool {
        has_sym!(self, xcb_record_context_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_record_context_iterator_t`.
    #[inline]
    pub unsafe fn xcb_record_context_end(
        &self,
        i: xcb_record_context_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_record_context_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_record_context_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_context_end(&self) -> bool {
        has_sym!(self, xcb_record_context_end)
    }

    /// Advances a `xcb_record_range_8_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_record_range_8_next(&self, i: *mut xcb_record_range_8_iterator_t) {
        sym!(self, xcb_record_range_8_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_record_range_8_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_range_8_next(&self) -> bool {
        has_sym!(self, xcb_record_range_8_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_record_range_8_iterator_t`.
    #[inline]
    pub unsafe fn xcb_record_range_8_end(
        &self,
        i: xcb_record_range_8_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_record_range_8_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_record_range_8_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_range_8_end(&self) -> bool {
        has_sym!(self, xcb_record_range_8_end)
    }

    /// Advances a `xcb_record_range_16_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_record_range_16_next(&self, i: *mut xcb_record_range_16_iterator_t) {
        sym!(self, xcb_record_range_16_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_record_range_16_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_range_16_next(&self) -> bool {
        has_sym!(self, xcb_record_range_16_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_record_range_16_iterator_t`.
    #[inline]
    pub unsafe fn xcb_record_range_16_end(
        &self,
        i: xcb_record_range_16_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_record_range_16_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_record_range_16_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_range_16_end(&self) -> bool {
        has_sym!(self, xcb_record_range_16_end)
    }

    /// Advances a `xcb_record_ext_range_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_record_ext_range_next(&self, i: *mut xcb_record_ext_range_iterator_t) {
        sym!(self, xcb_record_ext_range_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_record_ext_range_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_ext_range_next(&self) -> bool {
        has_sym!(self, xcb_record_ext_range_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_record_ext_range_iterator_t`.
    #[inline]
    pub unsafe fn xcb_record_ext_range_end(
        &self,
        i: xcb_record_ext_range_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_record_ext_range_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_record_ext_range_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_ext_range_end(&self) -> bool {
        has_sym!(self, xcb_record_ext_range_end)
    }

    /// Advances a `xcb_record_range_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_record_range_next(&self, i: *mut xcb_record_range_iterator_t) {
        sym!(self, xcb_record_range_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_record_range_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_range_next(&self) -> bool {
        has_sym!(self, xcb_record_range_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_record_range_iterator_t`.
    #[inline]
    pub unsafe fn xcb_record_range_end(
        &self,
        i: xcb_record_range_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_record_range_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_record_range_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_range_end(&self) -> bool {
        has_sym!(self, xcb_record_range_end)
    }

    /// Advances a `xcb_record_element_header_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_record_element_header_next(
        &self,
        i: *mut xcb_record_element_header_iterator_t,
    ) {
        sym!(self, xcb_record_element_header_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_record_element_header_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_element_header_next(&self) -> bool {
        has_sym!(self, xcb_record_element_header_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_record_element_header_iterator_t`.
    #[inline]
    pub unsafe fn xcb_record_element_header_end(
        &self,
        i: xcb_record_element_header_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_record_element_header_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_record_element_header_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_element_header_end(&self) -> bool {
        has_sym!(self, xcb_record_element_header_end)
    }

    /// Advances a `xcb_record_client_spec_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_record_client_spec_next(&self, i: *mut xcb_record_client_spec_iterator_t) {
        sym!(self, xcb_record_client_spec_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_record_client_spec_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_client_spec_next(&self) -> bool {
        has_sym!(self, xcb_record_client_spec_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_record_client_spec_iterator_t`.
    #[inline]
    pub unsafe fn xcb_record_client_spec_end(
        &self,
        i: xcb_record_client_spec_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_record_client_spec_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_record_client_spec_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_client_spec_end(&self) -> bool {
        has_sym!(self, xcb_record_client_spec_end)
    }

    /// Computes the size of a `xcb_record_client_info_t` object.
    #[inline]
    pub unsafe fn xcb_record_client_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_record_client_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_record_client_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_client_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_record_client_info_sizeof)
    }

    /// Returns a pointer to the `ranges` field of a `xcb_record_client_info_t` struct.
    #[inline]
    pub unsafe fn xcb_record_client_info_ranges(
        &self,
        r: *const xcb_record_client_info_t,
    ) -> *mut xcb_record_range_t {
        sym!(self, xcb_record_client_info_ranges)(r)
    }

    /// Returns `true` iff the symbol `xcb_record_client_info_ranges` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_client_info_ranges(&self) -> bool {
        has_sym!(self, xcb_record_client_info_ranges)
    }

    /// Returns the number of elements of the `ranges` field of a `xcb_record_client_info_t` struct.
    #[inline]
    pub unsafe fn xcb_record_client_info_ranges_length(
        &self,
        r: *const xcb_record_client_info_t,
    ) -> c_int {
        sym!(self, xcb_record_client_info_ranges_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_record_client_info_ranges_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_client_info_ranges_length(&self) -> bool {
        has_sym!(self, xcb_record_client_info_ranges_length)
    }

    /// Returns an iterator over the elements of the
    /// `ranges` field of a `xcb_record_client_info_t` struct.
    #[inline]
    pub unsafe fn xcb_record_client_info_ranges_iterator(
        &self,
        r: *const xcb_record_client_info_t,
    ) -> xcb_record_range_iterator_t {
        sym!(self, xcb_record_client_info_ranges_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_record_client_info_ranges_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_client_info_ranges_iterator(&self) -> bool {
        has_sym!(self, xcb_record_client_info_ranges_iterator)
    }

    /// Advances a `xcb_record_client_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_record_client_info_next(&self, i: *mut xcb_record_client_info_iterator_t) {
        sym!(self, xcb_record_client_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_record_client_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_client_info_next(&self) -> bool {
        has_sym!(self, xcb_record_client_info_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_record_client_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_record_client_info_end(
        &self,
        i: xcb_record_client_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_record_client_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_record_client_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_client_info_end(&self) -> bool {
        has_sym!(self, xcb_record_client_info_end)
    }

    /// Sends a `Record::QueryVersion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_record_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_record_query_version_reply`]: Self::xcb_record_query_version_reply
    #[inline]
    pub unsafe fn xcb_record_query_version(
        &self,
        c: *mut xcb_connection_t,
        major_version: u16,
        minor_version: u16,
    ) -> xcb_record_query_version_cookie_t {
        sym!(self, xcb_record_query_version)(c, major_version, minor_version)
    }

    /// Returns `true` iff the symbol `xcb_record_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_query_version(&self) -> bool {
        has_sym!(self, xcb_record_query_version)
    }

    /// Sends a `Record::QueryVersion` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_record_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_record_query_version_reply`]: Self::xcb_record_query_version_reply
    #[inline]
    pub unsafe fn xcb_record_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        major_version: u16,
        minor_version: u16,
    ) -> xcb_record_query_version_cookie_t {
        sym!(self, xcb_record_query_version_unchecked)(c, major_version, minor_version)
    }

    /// Returns `true` iff the symbol `xcb_record_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_query_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_record_query_version_unchecked)
    }

    /// Waits for the reply to a `Record::QueryVersion` request.
    #[inline]
    pub unsafe fn xcb_record_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_record_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_record_query_version_reply_t {
        sym!(self, xcb_record_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_record_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_record_query_version_reply)
    }

    /// Computes the size of a `xcb_record_create_context_request_t` object.
    #[inline]
    pub unsafe fn xcb_record_create_context_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_record_create_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_record_create_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_create_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_record_create_context_sizeof)
    }

    /// Sends a `Record::CreateContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_record_create_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_record_context_t,
        element_header: xcb_record_element_header_t,
        num_client_specs: u32,
        num_ranges: u32,
        client_specs: *const xcb_record_client_spec_t,
        ranges: *const xcb_record_range_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_record_create_context_checked)(
            c,
            context,
            element_header,
            num_client_specs,
            num_ranges,
            client_specs,
            ranges,
        )
    }

    /// Returns `true` iff the symbol `xcb_record_create_context_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_create_context_checked(&self) -> bool {
        has_sym!(self, xcb_record_create_context_checked)
    }

    /// Sends a `Record::CreateContext` request (unchecked).
    #[inline]
    pub unsafe fn xcb_record_create_context(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_record_context_t,
        element_header: xcb_record_element_header_t,
        num_client_specs: u32,
        num_ranges: u32,
        client_specs: *const xcb_record_client_spec_t,
        ranges: *const xcb_record_range_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_record_create_context)(
            c,
            context,
            element_header,
            num_client_specs,
            num_ranges,
            client_specs,
            ranges,
        )
    }

    /// Returns `true` iff the symbol `xcb_record_create_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_create_context(&self) -> bool {
        has_sym!(self, xcb_record_create_context)
    }

    /// Returns a pointer to the `client_specs` field of a `xcb_record_create_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_record_create_context_client_specs(
        &self,
        r: *const xcb_record_create_context_request_t,
    ) -> *mut xcb_record_client_spec_t {
        sym!(self, xcb_record_create_context_client_specs)(r)
    }

    /// Returns `true` iff the symbol `xcb_record_create_context_client_specs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_create_context_client_specs(&self) -> bool {
        has_sym!(self, xcb_record_create_context_client_specs)
    }

    /// Returns the number of elements of the `client_specs` field of a `xcb_record_create_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_record_create_context_client_specs_length(
        &self,
        r: *const xcb_record_create_context_request_t,
    ) -> c_int {
        sym!(self, xcb_record_create_context_client_specs_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_record_create_context_client_specs_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_create_context_client_specs_length(&self) -> bool {
        has_sym!(self, xcb_record_create_context_client_specs_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `client_specs` field of a `xcb_record_create_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_record_create_context_client_specs_end(
        &self,
        r: *const xcb_record_create_context_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_record_create_context_client_specs_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_record_create_context_client_specs_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_create_context_client_specs_end(&self) -> bool {
        has_sym!(self, xcb_record_create_context_client_specs_end)
    }

    /// Returns a pointer to the `ranges` field of a `xcb_record_create_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_record_create_context_ranges(
        &self,
        r: *const xcb_record_create_context_request_t,
    ) -> *mut xcb_record_range_t {
        sym!(self, xcb_record_create_context_ranges)(r)
    }

    /// Returns `true` iff the symbol `xcb_record_create_context_ranges` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_create_context_ranges(&self) -> bool {
        has_sym!(self, xcb_record_create_context_ranges)
    }

    /// Returns the number of elements of the `ranges` field of a `xcb_record_create_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_record_create_context_ranges_length(
        &self,
        r: *const xcb_record_create_context_request_t,
    ) -> c_int {
        sym!(self, xcb_record_create_context_ranges_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_record_create_context_ranges_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_create_context_ranges_length(&self) -> bool {
        has_sym!(self, xcb_record_create_context_ranges_length)
    }

    /// Returns an iterator over the elements of the
    /// `ranges` field of a `xcb_record_create_context_request_t` struct.
    #[inline]
    pub unsafe fn xcb_record_create_context_ranges_iterator(
        &self,
        r: *const xcb_record_create_context_request_t,
    ) -> xcb_record_range_iterator_t {
        sym!(self, xcb_record_create_context_ranges_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_record_create_context_ranges_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_create_context_ranges_iterator(&self) -> bool {
        has_sym!(self, xcb_record_create_context_ranges_iterator)
    }

    /// Computes the size of a `xcb_record_register_clients_request_t` object.
    #[inline]
    pub unsafe fn xcb_record_register_clients_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_record_register_clients_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_record_register_clients_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_register_clients_sizeof(&self) -> bool {
        has_sym!(self, xcb_record_register_clients_sizeof)
    }

    /// Sends a `Record::RegisterClients` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_record_register_clients_checked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_record_context_t,
        element_header: xcb_record_element_header_t,
        num_client_specs: u32,
        num_ranges: u32,
        client_specs: *const xcb_record_client_spec_t,
        ranges: *const xcb_record_range_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_record_register_clients_checked)(
            c,
            context,
            element_header,
            num_client_specs,
            num_ranges,
            client_specs,
            ranges,
        )
    }

    /// Returns `true` iff the symbol `xcb_record_register_clients_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_register_clients_checked(&self) -> bool {
        has_sym!(self, xcb_record_register_clients_checked)
    }

    /// Sends a `Record::RegisterClients` request (unchecked).
    #[inline]
    pub unsafe fn xcb_record_register_clients(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_record_context_t,
        element_header: xcb_record_element_header_t,
        num_client_specs: u32,
        num_ranges: u32,
        client_specs: *const xcb_record_client_spec_t,
        ranges: *const xcb_record_range_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_record_register_clients)(
            c,
            context,
            element_header,
            num_client_specs,
            num_ranges,
            client_specs,
            ranges,
        )
    }

    /// Returns `true` iff the symbol `xcb_record_register_clients` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_register_clients(&self) -> bool {
        has_sym!(self, xcb_record_register_clients)
    }

    /// Returns a pointer to the `client_specs` field of a `xcb_record_register_clients_request_t` struct.
    #[inline]
    pub unsafe fn xcb_record_register_clients_client_specs(
        &self,
        r: *const xcb_record_register_clients_request_t,
    ) -> *mut xcb_record_client_spec_t {
        sym!(self, xcb_record_register_clients_client_specs)(r)
    }

    /// Returns `true` iff the symbol `xcb_record_register_clients_client_specs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_register_clients_client_specs(&self) -> bool {
        has_sym!(self, xcb_record_register_clients_client_specs)
    }

    /// Returns the number of elements of the `client_specs` field of a `xcb_record_register_clients_request_t` struct.
    #[inline]
    pub unsafe fn xcb_record_register_clients_client_specs_length(
        &self,
        r: *const xcb_record_register_clients_request_t,
    ) -> c_int {
        sym!(self, xcb_record_register_clients_client_specs_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_record_register_clients_client_specs_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_register_clients_client_specs_length(&self) -> bool {
        has_sym!(self, xcb_record_register_clients_client_specs_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `client_specs` field of a `xcb_record_register_clients_request_t` struct.
    #[inline]
    pub unsafe fn xcb_record_register_clients_client_specs_end(
        &self,
        r: *const xcb_record_register_clients_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_record_register_clients_client_specs_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_record_register_clients_client_specs_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_register_clients_client_specs_end(&self) -> bool {
        has_sym!(self, xcb_record_register_clients_client_specs_end)
    }

    /// Returns a pointer to the `ranges` field of a `xcb_record_register_clients_request_t` struct.
    #[inline]
    pub unsafe fn xcb_record_register_clients_ranges(
        &self,
        r: *const xcb_record_register_clients_request_t,
    ) -> *mut xcb_record_range_t {
        sym!(self, xcb_record_register_clients_ranges)(r)
    }

    /// Returns `true` iff the symbol `xcb_record_register_clients_ranges` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_register_clients_ranges(&self) -> bool {
        has_sym!(self, xcb_record_register_clients_ranges)
    }

    /// Returns the number of elements of the `ranges` field of a `xcb_record_register_clients_request_t` struct.
    #[inline]
    pub unsafe fn xcb_record_register_clients_ranges_length(
        &self,
        r: *const xcb_record_register_clients_request_t,
    ) -> c_int {
        sym!(self, xcb_record_register_clients_ranges_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_record_register_clients_ranges_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_register_clients_ranges_length(&self) -> bool {
        has_sym!(self, xcb_record_register_clients_ranges_length)
    }

    /// Returns an iterator over the elements of the
    /// `ranges` field of a `xcb_record_register_clients_request_t` struct.
    #[inline]
    pub unsafe fn xcb_record_register_clients_ranges_iterator(
        &self,
        r: *const xcb_record_register_clients_request_t,
    ) -> xcb_record_range_iterator_t {
        sym!(self, xcb_record_register_clients_ranges_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_record_register_clients_ranges_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_register_clients_ranges_iterator(&self) -> bool {
        has_sym!(self, xcb_record_register_clients_ranges_iterator)
    }

    /// Computes the size of a `xcb_record_unregister_clients_request_t` object.
    #[inline]
    pub unsafe fn xcb_record_unregister_clients_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_record_unregister_clients_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_record_unregister_clients_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_unregister_clients_sizeof(&self) -> bool {
        has_sym!(self, xcb_record_unregister_clients_sizeof)
    }

    /// Sends a `Record::UnregisterClients` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_record_unregister_clients_checked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_record_context_t,
        num_client_specs: u32,
        client_specs: *const xcb_record_client_spec_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_record_unregister_clients_checked)(
            c,
            context,
            num_client_specs,
            client_specs,
        )
    }

    /// Returns `true` iff the symbol `xcb_record_unregister_clients_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_unregister_clients_checked(&self) -> bool {
        has_sym!(self, xcb_record_unregister_clients_checked)
    }

    /// Sends a `Record::UnregisterClients` request (unchecked).
    #[inline]
    pub unsafe fn xcb_record_unregister_clients(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_record_context_t,
        num_client_specs: u32,
        client_specs: *const xcb_record_client_spec_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_record_unregister_clients)(c, context, num_client_specs, client_specs)
    }

    /// Returns `true` iff the symbol `xcb_record_unregister_clients` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_unregister_clients(&self) -> bool {
        has_sym!(self, xcb_record_unregister_clients)
    }

    /// Returns a pointer to the `client_specs` field of a `xcb_record_unregister_clients_request_t` struct.
    #[inline]
    pub unsafe fn xcb_record_unregister_clients_client_specs(
        &self,
        r: *const xcb_record_unregister_clients_request_t,
    ) -> *mut xcb_record_client_spec_t {
        sym!(self, xcb_record_unregister_clients_client_specs)(r)
    }

    /// Returns `true` iff the symbol `xcb_record_unregister_clients_client_specs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_unregister_clients_client_specs(&self) -> bool {
        has_sym!(self, xcb_record_unregister_clients_client_specs)
    }

    /// Returns the number of elements of the `client_specs` field of a `xcb_record_unregister_clients_request_t` struct.
    #[inline]
    pub unsafe fn xcb_record_unregister_clients_client_specs_length(
        &self,
        r: *const xcb_record_unregister_clients_request_t,
    ) -> c_int {
        sym!(self, xcb_record_unregister_clients_client_specs_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_record_unregister_clients_client_specs_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_unregister_clients_client_specs_length(&self) -> bool {
        has_sym!(self, xcb_record_unregister_clients_client_specs_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `client_specs` field of a `xcb_record_unregister_clients_request_t` struct.
    #[inline]
    pub unsafe fn xcb_record_unregister_clients_client_specs_end(
        &self,
        r: *const xcb_record_unregister_clients_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_record_unregister_clients_client_specs_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_record_unregister_clients_client_specs_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_unregister_clients_client_specs_end(&self) -> bool {
        has_sym!(self, xcb_record_unregister_clients_client_specs_end)
    }

    /// Computes the size of a `xcb_record_get_context_reply_t` object.
    #[inline]
    pub unsafe fn xcb_record_get_context_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_record_get_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_record_get_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_get_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_record_get_context_sizeof)
    }

    /// Sends a `Record::GetContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_record_get_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_record_get_context_reply`]: Self::xcb_record_get_context_reply
    #[inline]
    pub unsafe fn xcb_record_get_context(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_record_context_t,
    ) -> xcb_record_get_context_cookie_t {
        sym!(self, xcb_record_get_context)(c, context)
    }

    /// Returns `true` iff the symbol `xcb_record_get_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_get_context(&self) -> bool {
        has_sym!(self, xcb_record_get_context)
    }

    /// Sends a `Record::GetContext` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_record_get_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_record_get_context_reply`]: Self::xcb_record_get_context_reply
    #[inline]
    pub unsafe fn xcb_record_get_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_record_context_t,
    ) -> xcb_record_get_context_cookie_t {
        sym!(self, xcb_record_get_context_unchecked)(c, context)
    }

    /// Returns `true` iff the symbol `xcb_record_get_context_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_get_context_unchecked(&self) -> bool {
        has_sym!(self, xcb_record_get_context_unchecked)
    }

    /// Returns the number of elements of the `intercepted_clients` field of a `xcb_record_get_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_record_get_context_intercepted_clients_length(
        &self,
        r: *const xcb_record_get_context_reply_t,
    ) -> c_int {
        sym!(self, xcb_record_get_context_intercepted_clients_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_record_get_context_intercepted_clients_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_get_context_intercepted_clients_length(&self) -> bool {
        has_sym!(self, xcb_record_get_context_intercepted_clients_length)
    }

    /// Returns an iterator over the elements of the
    /// `intercepted_clients` field of a `xcb_record_get_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_record_get_context_intercepted_clients_iterator(
        &self,
        r: *const xcb_record_get_context_reply_t,
    ) -> xcb_record_client_info_iterator_t {
        sym!(self, xcb_record_get_context_intercepted_clients_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_record_get_context_intercepted_clients_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_get_context_intercepted_clients_iterator(&self) -> bool {
        has_sym!(self, xcb_record_get_context_intercepted_clients_iterator)
    }

    /// Waits for the reply to a `Record::GetContext` request.
    #[inline]
    pub unsafe fn xcb_record_get_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_record_get_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_record_get_context_reply_t {
        sym!(self, xcb_record_get_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_record_get_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_get_context_reply(&self) -> bool {
        has_sym!(self, xcb_record_get_context_reply)
    }

    /// Computes the size of a `xcb_record_enable_context_reply_t` object.
    #[inline]
    pub unsafe fn xcb_record_enable_context_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_record_enable_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_record_enable_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_enable_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_record_enable_context_sizeof)
    }

    /// Sends a `Record::EnableContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_record_enable_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_record_enable_context_reply`]: Self::xcb_record_enable_context_reply
    #[inline]
    pub unsafe fn xcb_record_enable_context(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_record_context_t,
    ) -> xcb_record_enable_context_cookie_t {
        sym!(self, xcb_record_enable_context)(c, context)
    }

    /// Returns `true` iff the symbol `xcb_record_enable_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_enable_context(&self) -> bool {
        has_sym!(self, xcb_record_enable_context)
    }

    /// Sends a `Record::EnableContext` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_record_enable_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_record_enable_context_reply`]: Self::xcb_record_enable_context_reply
    #[inline]
    pub unsafe fn xcb_record_enable_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_record_context_t,
    ) -> xcb_record_enable_context_cookie_t {
        sym!(self, xcb_record_enable_context_unchecked)(c, context)
    }

    /// Returns `true` iff the symbol `xcb_record_enable_context_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_enable_context_unchecked(&self) -> bool {
        has_sym!(self, xcb_record_enable_context_unchecked)
    }

    /// Returns a pointer to the `data` field of a `xcb_record_enable_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_record_enable_context_data(
        &self,
        r: *const xcb_record_enable_context_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_record_enable_context_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_record_enable_context_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_enable_context_data(&self) -> bool {
        has_sym!(self, xcb_record_enable_context_data)
    }

    /// Returns the number of elements of the `data` field of a `xcb_record_enable_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_record_enable_context_data_length(
        &self,
        r: *const xcb_record_enable_context_reply_t,
    ) -> c_int {
        sym!(self, xcb_record_enable_context_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_record_enable_context_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_enable_context_data_length(&self) -> bool {
        has_sym!(self, xcb_record_enable_context_data_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `data` field of a `xcb_record_enable_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_record_enable_context_data_end(
        &self,
        r: *const xcb_record_enable_context_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_record_enable_context_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_record_enable_context_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_enable_context_data_end(&self) -> bool {
        has_sym!(self, xcb_record_enable_context_data_end)
    }

    /// Waits for the reply to a `Record::EnableContext` request.
    #[inline]
    pub unsafe fn xcb_record_enable_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_record_enable_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_record_enable_context_reply_t {
        sym!(self, xcb_record_enable_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_record_enable_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_enable_context_reply(&self) -> bool {
        has_sym!(self, xcb_record_enable_context_reply)
    }

    /// Sends a `Record::DisableContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_record_disable_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_record_context_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_record_disable_context_checked)(c, context)
    }

    /// Returns `true` iff the symbol `xcb_record_disable_context_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_disable_context_checked(&self) -> bool {
        has_sym!(self, xcb_record_disable_context_checked)
    }

    /// Sends a `Record::DisableContext` request (unchecked).
    #[inline]
    pub unsafe fn xcb_record_disable_context(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_record_context_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_record_disable_context)(c, context)
    }

    /// Returns `true` iff the symbol `xcb_record_disable_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_disable_context(&self) -> bool {
        has_sym!(self, xcb_record_disable_context)
    }

    /// Sends a `Record::FreeContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_record_free_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_record_context_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_record_free_context_checked)(c, context)
    }

    /// Returns `true` iff the symbol `xcb_record_free_context_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_free_context_checked(&self) -> bool {
        has_sym!(self, xcb_record_free_context_checked)
    }

    /// Sends a `Record::FreeContext` request (unchecked).
    #[inline]
    pub unsafe fn xcb_record_free_context(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_record_context_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_record_free_context)(c, context)
    }

    /// Returns `true` iff the symbol `xcb_record_free_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_free_context(&self) -> bool {
        has_sym!(self, xcb_record_free_context)
    }
}

#[cfg(feature = "xcb_record")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbRecord::load().unwrap() };
        assert!(lib.has_xcb_record_id());
        assert!(lib.has_xcb_record_context_next());
        assert!(lib.has_xcb_record_context_end());
        assert!(lib.has_xcb_record_range_8_next());
        assert!(lib.has_xcb_record_range_8_end());
        assert!(lib.has_xcb_record_range_16_next());
        assert!(lib.has_xcb_record_range_16_end());
        assert!(lib.has_xcb_record_ext_range_next());
        assert!(lib.has_xcb_record_ext_range_end());
        assert!(lib.has_xcb_record_range_next());
        assert!(lib.has_xcb_record_range_end());
        assert!(lib.has_xcb_record_element_header_next());
        assert!(lib.has_xcb_record_element_header_end());
        assert!(lib.has_xcb_record_client_spec_next());
        assert!(lib.has_xcb_record_client_spec_end());
        assert!(lib.has_xcb_record_client_info_sizeof());
        assert!(lib.has_xcb_record_client_info_ranges());
        assert!(lib.has_xcb_record_client_info_ranges_length());
        assert!(lib.has_xcb_record_client_info_ranges_iterator());
        assert!(lib.has_xcb_record_client_info_next());
        assert!(lib.has_xcb_record_client_info_end());
        assert!(lib.has_xcb_record_query_version());
        assert!(lib.has_xcb_record_query_version_unchecked());
        assert!(lib.has_xcb_record_query_version_reply());
        assert!(lib.has_xcb_record_create_context_sizeof());
        assert!(lib.has_xcb_record_create_context_checked());
        assert!(lib.has_xcb_record_create_context());
        assert!(lib.has_xcb_record_create_context_client_specs());
        assert!(lib.has_xcb_record_create_context_client_specs_length());
        assert!(lib.has_xcb_record_create_context_client_specs_end());
        assert!(lib.has_xcb_record_create_context_ranges());
        assert!(lib.has_xcb_record_create_context_ranges_length());
        assert!(lib.has_xcb_record_create_context_ranges_iterator());
        assert!(lib.has_xcb_record_register_clients_sizeof());
        assert!(lib.has_xcb_record_register_clients_checked());
        assert!(lib.has_xcb_record_register_clients());
        assert!(lib.has_xcb_record_register_clients_client_specs());
        assert!(lib.has_xcb_record_register_clients_client_specs_length());
        assert!(lib.has_xcb_record_register_clients_client_specs_end());
        assert!(lib.has_xcb_record_register_clients_ranges());
        assert!(lib.has_xcb_record_register_clients_ranges_length());
        assert!(lib.has_xcb_record_register_clients_ranges_iterator());
        assert!(lib.has_xcb_record_unregister_clients_sizeof());
        assert!(lib.has_xcb_record_unregister_clients_checked());
        assert!(lib.has_xcb_record_unregister_clients());
        assert!(lib.has_xcb_record_unregister_clients_client_specs());
        assert!(lib.has_xcb_record_unregister_clients_client_specs_length());
        assert!(lib.has_xcb_record_unregister_clients_client_specs_end());
        assert!(lib.has_xcb_record_get_context_sizeof());
        assert!(lib.has_xcb_record_get_context());
        assert!(lib.has_xcb_record_get_context_unchecked());
        assert!(lib.has_xcb_record_get_context_intercepted_clients_length());
        assert!(lib.has_xcb_record_get_context_intercepted_clients_iterator());
        assert!(lib.has_xcb_record_get_context_reply());
        assert!(lib.has_xcb_record_enable_context_sizeof());
        assert!(lib.has_xcb_record_enable_context());
        assert!(lib.has_xcb_record_enable_context_unchecked());
        assert!(lib.has_xcb_record_enable_context_data());
        assert!(lib.has_xcb_record_enable_context_data_length());
        assert!(lib.has_xcb_record_enable_context_data_end());
        assert!(lib.has_xcb_record_enable_context_reply());
        assert!(lib.has_xcb_record_disable_context_checked());
        assert!(lib.has_xcb_record_disable_context());
        assert!(lib.has_xcb_record_free_context_checked());
        assert!(lib.has_xcb_record_free_context());
    }
}
