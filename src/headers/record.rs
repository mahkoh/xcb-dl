// This file was generated using generate.py. Do not edit.

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

pub type xcb_record_context_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_context_iterator_t {
    pub data: *mut xcb_record_context_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_record_context_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_range_8_iterator_t {
    pub data: *mut xcb_record_range_8_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_record_range_8_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_range_16_iterator_t {
    pub data: *mut xcb_record_range_16_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_record_range_16_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_ext_range_iterator_t {
    pub data: *mut xcb_record_ext_range_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_record_ext_range_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_range_iterator_t {
    pub data: *mut xcb_record_range_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_record_range_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_record_element_header_t = u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_element_header_iterator_t {
    pub data: *mut xcb_record_element_header_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_record_element_header_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_record_h_type_t = u32;
pub const XCB_RECORD_H_TYPE_FROM_SERVER_TIME: xcb_record_h_type_t = 1;
pub const XCB_RECORD_H_TYPE_FROM_CLIENT_TIME: xcb_record_h_type_t = 2;
pub const XCB_RECORD_H_TYPE_FROM_CLIENT_SEQUENCE: xcb_record_h_type_t = 4;

pub type xcb_record_client_spec_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_client_spec_iterator_t {
    pub data: *mut xcb_record_client_spec_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_record_client_spec_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_record_cs_t = u32;
pub const XCB_RECORD_CS_CURRENT_CLIENTS: xcb_record_cs_t = 1;
pub const XCB_RECORD_CS_FUTURE_CLIENTS: xcb_record_cs_t = 2;
pub const XCB_RECORD_CS_ALL_CLIENTS: xcb_record_cs_t = 3;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_client_info_iterator_t {
    pub data: *mut xcb_record_client_info_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_record_client_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_record_bad_context.
pub const XCB_RECORD_BAD_CONTEXT: u8 = 0i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_query_version_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_record_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_record_query_version.
pub const XCB_RECORD_QUERY_VERSION: u8 = 0i32 as u8;

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

/// Opcode for xcb_record_create_context.
pub const XCB_RECORD_CREATE_CONTEXT: u8 = 1i32 as u8;

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

/// Opcode for xcb_record_register_clients.
pub const XCB_RECORD_REGISTER_CLIENTS: u8 = 2i32 as u8;

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

/// Opcode for xcb_record_unregister_clients.
pub const XCB_RECORD_UNREGISTER_CLIENTS: u8 = 3i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_get_context_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_record_get_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_record_get_context.
pub const XCB_RECORD_GET_CONTEXT: u8 = 4i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_enable_context_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_record_enable_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_record_enable_context.
pub const XCB_RECORD_ENABLE_CONTEXT: u8 = 5i32 as u8;

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

/// Opcode for xcb_record_disable_context.
pub const XCB_RECORD_DISABLE_CONTEXT: u8 = 6i32 as u8;

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

/// Opcode for xcb_record_free_context.
pub const XCB_RECORD_FREE_CONTEXT: u8 = 7i32 as u8;

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
        ) -> xcb_record_query_version_reply_t,
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
        ) -> xcb_record_get_context_reply_t,
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
        ) -> xcb_record_enable_context_reply_t,
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
    pub fn xcb_record_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_record_id) }
    }

    /// Returns `true` iff the symbol `xcb_record_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_id(&self) -> bool {
        has_sym!(self, xcb_record_id)
    }

    pub unsafe fn xcb_record_context_next(&self, i: *mut xcb_record_context_iterator_t) {
        sym!(self, xcb_record_context_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_record_context_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_context_next(&self) -> bool {
        has_sym!(self, xcb_record_context_next)
    }

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

    pub unsafe fn xcb_record_range_8_next(&self, i: *mut xcb_record_range_8_iterator_t) {
        sym!(self, xcb_record_range_8_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_record_range_8_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_range_8_next(&self) -> bool {
        has_sym!(self, xcb_record_range_8_next)
    }

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

    pub unsafe fn xcb_record_range_16_next(&self, i: *mut xcb_record_range_16_iterator_t) {
        sym!(self, xcb_record_range_16_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_record_range_16_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_range_16_next(&self) -> bool {
        has_sym!(self, xcb_record_range_16_next)
    }

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

    pub unsafe fn xcb_record_ext_range_next(&self, i: *mut xcb_record_ext_range_iterator_t) {
        sym!(self, xcb_record_ext_range_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_record_ext_range_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_ext_range_next(&self) -> bool {
        has_sym!(self, xcb_record_ext_range_next)
    }

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

    pub unsafe fn xcb_record_range_next(&self, i: *mut xcb_record_range_iterator_t) {
        sym!(self, xcb_record_range_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_record_range_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_range_next(&self) -> bool {
        has_sym!(self, xcb_record_range_next)
    }

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

    pub unsafe fn xcb_record_client_spec_next(&self, i: *mut xcb_record_client_spec_iterator_t) {
        sym!(self, xcb_record_client_spec_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_record_client_spec_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_client_spec_next(&self) -> bool {
        has_sym!(self, xcb_record_client_spec_next)
    }

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

    pub unsafe fn xcb_record_client_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_record_client_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_record_client_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_client_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_record_client_info_sizeof)
    }

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

    pub unsafe fn xcb_record_client_info_next(&self, i: *mut xcb_record_client_info_iterator_t) {
        sym!(self, xcb_record_client_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_record_client_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_client_info_next(&self) -> bool {
        has_sym!(self, xcb_record_client_info_next)
    }

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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_record_query_version_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_record_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_record_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_record_query_version_reply_t {
        sym!(self, xcb_record_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_record_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_record_query_version_reply)
    }

    pub unsafe fn xcb_record_create_context_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_record_create_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_record_create_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_create_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_record_create_context_sizeof)
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    pub unsafe fn xcb_record_register_clients_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_record_register_clients_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_record_register_clients_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_register_clients_sizeof(&self) -> bool {
        has_sym!(self, xcb_record_register_clients_sizeof)
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    pub unsafe fn xcb_record_unregister_clients_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_record_unregister_clients_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_record_unregister_clients_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_unregister_clients_sizeof(&self) -> bool {
        has_sym!(self, xcb_record_unregister_clients_sizeof)
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    pub unsafe fn xcb_record_get_context_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_record_get_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_record_get_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_get_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_record_get_context_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_record_get_context_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_record_get_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_record_get_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_record_get_context_reply_t {
        sym!(self, xcb_record_get_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_record_get_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_get_context_reply(&self) -> bool {
        has_sym!(self, xcb_record_get_context_reply)
    }

    pub unsafe fn xcb_record_enable_context_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_record_enable_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_record_enable_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_enable_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_record_enable_context_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_record_enable_context_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_record_enable_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_record_enable_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_record_enable_context_reply_t {
        sym!(self, xcb_record_enable_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_record_enable_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_record_enable_context_reply(&self) -> bool {
        has_sym!(self, xcb_record_enable_context_reply)
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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
