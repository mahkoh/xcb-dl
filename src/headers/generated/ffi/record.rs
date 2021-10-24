use crate::*;
use std::os::raw::*;

pub const XCB_RECORD_MAJOR_VERSION: u32 = 1;
pub const XCB_RECORD_MINOR_VERSION: u32 = 13;

pub type xcb_record_context_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_context_iterator_t {
    pub data: *mut xcb_record_context_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_range_8_t {
    pub first: u8,
    pub last: u8,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_range_8_iterator_t {
    pub data: *mut xcb_record_range_8_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_range_16_t {
    pub first: u16,
    pub last: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_range_16_iterator_t {
    pub data: *mut xcb_record_range_16_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_ext_range_t {
    pub major: xcb_record_range_8_t,
    pub minor: xcb_record_range_16_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_ext_range_iterator_t {
    pub data: *mut xcb_record_ext_range_t,
    pub rem: c_int,
    pub index: c_int,
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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_range_iterator_t {
    pub data: *mut xcb_record_range_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_record_element_header_t = u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_element_header_iterator_t {
    pub data: *mut xcb_record_element_header_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_record_h_type_t = u32;
pub const XCB_RECORD_H_TYPE_FROM_SERVER_TIME: xcb_record_h_type_t = 0x01;
pub const XCB_RECORD_H_TYPE_FROM_CLIENT_TIME: xcb_record_h_type_t = 0x02;
pub const XCB_RECORD_H_TYPE_FROM_CLIENT_SEQUENCE: xcb_record_h_type_t = 0x04;

pub type xcb_record_client_spec_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_client_spec_iterator_t {
    pub data: *mut xcb_record_client_spec_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_record_cs_t = u32;
pub const XCB_RECORD_CS_CURRENT_CLIENTS: xcb_record_cs_t = 0x01;
pub const XCB_RECORD_CS_FUTURE_CLIENTS: xcb_record_cs_t = 0x02;
pub const XCB_RECORD_CS_ALL_CLIENTS: xcb_record_cs_t = 0x03;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_client_info_t {
    pub client_resource: xcb_record_client_spec_t,
    pub num_ranges: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_client_info_iterator_t {
    pub data: *mut xcb_record_client_info_t,
    pub rem: c_int,
    pub index: c_int,
}

pub const XCB_RECORD_BAD_CONTEXT: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_bad_context_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub invalid_record: u32,
}

pub const XCB_RECORD_QUERY_VERSION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub major_version: u16,
    pub minor_version: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_query_version_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_RECORD_CREATE_CONTEXT: u8 = 1;

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

pub const XCB_RECORD_REGISTER_CLIENTS: u8 = 2;

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

pub const XCB_RECORD_UNREGISTER_CLIENTS: u8 = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_unregister_clients_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_record_context_t,
    pub num_client_specs: u32,
}

pub const XCB_RECORD_GET_CONTEXT: u8 = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_get_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_record_context_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_get_context_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_RECORD_ENABLE_CONTEXT: u8 = 5;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_enable_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_record_context_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_enable_context_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_RECORD_DISABLE_CONTEXT: u8 = 6;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_disable_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_record_context_t,
}

pub const XCB_RECORD_FREE_CONTEXT: u8 = 7;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_record_free_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_record_context_t,
}

impl XcbRecord {
    #[inline]
    pub fn xcb_record_id(&self) -> *mut xcb_extension_t {
        call!(self, xcb_record_id)
    }

    #[inline]
    pub unsafe fn xcb_record_context_next(&self, i: *mut xcb_record_context_iterator_t) {
        call!(self, xcb_record_context_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_record_context_end(
        &self,
        i: *mut xcb_record_context_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_record_context_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_record_range_8_next(&self, i: *mut xcb_record_range_8_iterator_t) {
        call!(self, xcb_record_range_8_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_record_range_8_end(
        &self,
        i: *mut xcb_record_range_8_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_record_range_8_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_record_range_16_next(&self, i: *mut xcb_record_range_16_iterator_t) {
        call!(self, xcb_record_range_16_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_record_range_16_end(
        &self,
        i: *mut xcb_record_range_16_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_record_range_16_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_record_ext_range_next(&self, i: *mut xcb_record_ext_range_iterator_t) {
        call!(self, xcb_record_ext_range_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_record_ext_range_end(
        &self,
        i: *mut xcb_record_ext_range_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_record_ext_range_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_record_range_next(&self, i: *mut xcb_record_range_iterator_t) {
        call!(self, xcb_record_range_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_record_range_end(
        &self,
        i: *mut xcb_record_range_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_record_range_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_record_element_header_next(
        &self,
        i: *mut xcb_record_element_header_iterator_t,
    ) {
        call!(self, xcb_record_element_header_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_record_element_header_end(
        &self,
        i: *mut xcb_record_element_header_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_record_element_header_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_record_client_spec_next(&self, i: *mut xcb_record_client_spec_iterator_t) {
        call!(self, xcb_record_client_spec_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_record_client_spec_end(
        &self,
        i: *mut xcb_record_client_spec_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_record_client_spec_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_record_client_info_ranges(
        &self,
        R: *const xcb_record_client_info_t,
    ) -> *mut xcb_record_range_t {
        call!(self, xcb_record_client_info_ranges)(R)
    }

    #[inline]
    pub unsafe fn xcb_record_client_info_ranges_length(
        &self,
        R: *const xcb_record_client_info_t,
    ) -> c_int {
        call!(self, xcb_record_client_info_ranges_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_record_client_info_ranges_iterator(
        &self,
        R: *const xcb_record_client_info_t,
    ) -> xcb_record_range_iterator_t {
        call!(self, xcb_record_client_info_ranges_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_record_client_info_next(&self, i: *mut xcb_record_client_info_iterator_t) {
        call!(self, xcb_record_client_info_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_record_client_info_end(
        &self,
        i: *mut xcb_record_client_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_record_client_info_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_record_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_record_query_version_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_record_query_version_reply_t {
        call!(self, xcb_record_query_version_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_record_query_version(
        &self,
        c: *mut xcb_connection_t,
        major_version: u16,
        minor_version: u16,
    ) -> xcb_record_query_version_cookie_t {
        call!(self, xcb_record_query_version)(c, major_version, minor_version)
    }

    #[inline]
    pub unsafe fn xcb_record_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        major_version: u16,
        minor_version: u16,
    ) -> xcb_record_query_version_cookie_t {
        call!(self, xcb_record_query_version_unchecked)(c, major_version, minor_version)
    }

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
        call!(self, xcb_record_create_context)(
            c,
            context,
            element_header,
            num_client_specs,
            num_ranges,
            client_specs,
            ranges,
        )
    }

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
        call!(self, xcb_record_create_context_checked)(
            c,
            context,
            element_header,
            num_client_specs,
            num_ranges,
            client_specs,
            ranges,
        )
    }

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
        call!(self, xcb_record_register_clients)(
            c,
            context,
            element_header,
            num_client_specs,
            num_ranges,
            client_specs,
            ranges,
        )
    }

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
        call!(self, xcb_record_register_clients_checked)(
            c,
            context,
            element_header,
            num_client_specs,
            num_ranges,
            client_specs,
            ranges,
        )
    }

    #[inline]
    pub unsafe fn xcb_record_unregister_clients(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_record_context_t,
        num_client_specs: u32,
        client_specs: *const xcb_record_client_spec_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_record_unregister_clients)(c, context, num_client_specs, client_specs)
    }

    #[inline]
    pub unsafe fn xcb_record_unregister_clients_checked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_record_context_t,
        num_client_specs: u32,
        client_specs: *const xcb_record_client_spec_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_record_unregister_clients_checked)(
            c,
            context,
            num_client_specs,
            client_specs,
        )
    }

    #[inline]
    pub unsafe fn xcb_record_get_context_intercepted_clients_length(
        &self,
        R: *const xcb_record_get_context_reply_t,
    ) -> c_int {
        call!(self, xcb_record_get_context_intercepted_clients_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_record_get_context_intercepted_clients_iterator(
        &self,
        R: *const xcb_record_get_context_reply_t,
    ) -> xcb_record_client_info_iterator_t {
        call!(self, xcb_record_get_context_intercepted_clients_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_record_get_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_record_get_context_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_record_get_context_reply_t {
        call!(self, xcb_record_get_context_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_record_get_context(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_record_context_t,
    ) -> xcb_record_get_context_cookie_t {
        call!(self, xcb_record_get_context)(c, context)
    }

    #[inline]
    pub unsafe fn xcb_record_get_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_record_context_t,
    ) -> xcb_record_get_context_cookie_t {
        call!(self, xcb_record_get_context_unchecked)(c, context)
    }

    #[inline]
    pub unsafe fn xcb_record_enable_context_data(
        &self,
        R: *const xcb_record_enable_context_reply_t,
    ) -> *mut u8 {
        call!(self, xcb_record_enable_context_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_record_enable_context_data_length(
        &self,
        R: *const xcb_record_enable_context_reply_t,
    ) -> c_int {
        call!(self, xcb_record_enable_context_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_record_enable_context_data_end(
        &self,
        R: *const xcb_record_enable_context_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_record_enable_context_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_record_enable_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_record_enable_context_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_record_enable_context_reply_t {
        call!(self, xcb_record_enable_context_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_record_enable_context(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_record_context_t,
    ) -> xcb_record_enable_context_cookie_t {
        call!(self, xcb_record_enable_context)(c, context)
    }

    #[inline]
    pub unsafe fn xcb_record_enable_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_record_context_t,
    ) -> xcb_record_enable_context_cookie_t {
        call!(self, xcb_record_enable_context_unchecked)(c, context)
    }

    #[inline]
    pub unsafe fn xcb_record_disable_context(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_record_context_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_record_disable_context)(c, context)
    }

    #[inline]
    pub unsafe fn xcb_record_disable_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_record_context_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_record_disable_context_checked)(c, context)
    }

    #[inline]
    pub unsafe fn xcb_record_free_context(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_record_context_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_record_free_context)(c, context)
    }

    #[inline]
    pub unsafe fn xcb_record_free_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_record_context_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_record_free_context_checked)(c, context)
    }
}

pub struct XcbRecord {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_record_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_record_context_next:
        LazySymbol<unsafe fn(i: *mut xcb_record_context_iterator_t)>,
    pub(crate) xcb_record_context_end:
        LazySymbol<unsafe fn(i: *mut xcb_record_context_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_record_range_8_next:
        LazySymbol<unsafe fn(i: *mut xcb_record_range_8_iterator_t)>,
    pub(crate) xcb_record_range_8_end:
        LazySymbol<unsafe fn(i: *mut xcb_record_range_8_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_record_range_16_next:
        LazySymbol<unsafe fn(i: *mut xcb_record_range_16_iterator_t)>,
    pub(crate) xcb_record_range_16_end:
        LazySymbol<unsafe fn(i: *mut xcb_record_range_16_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_record_ext_range_next:
        LazySymbol<unsafe fn(i: *mut xcb_record_ext_range_iterator_t)>,
    pub(crate) xcb_record_ext_range_end:
        LazySymbol<unsafe fn(i: *mut xcb_record_ext_range_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_record_range_next: LazySymbol<unsafe fn(i: *mut xcb_record_range_iterator_t)>,
    pub(crate) xcb_record_range_end:
        LazySymbol<unsafe fn(i: *mut xcb_record_range_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_record_element_header_next:
        LazySymbol<unsafe fn(i: *mut xcb_record_element_header_iterator_t)>,
    pub(crate) xcb_record_element_header_end: LazySymbol<
        unsafe fn(i: *mut xcb_record_element_header_iterator_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_record_client_spec_next:
        LazySymbol<unsafe fn(i: *mut xcb_record_client_spec_iterator_t)>,
    pub(crate) xcb_record_client_spec_end:
        LazySymbol<unsafe fn(i: *mut xcb_record_client_spec_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_record_client_info_ranges:
        LazySymbol<unsafe fn(R: *const xcb_record_client_info_t) -> *mut xcb_record_range_t>,
    pub(crate) xcb_record_client_info_ranges_length:
        LazySymbol<unsafe fn(R: *const xcb_record_client_info_t) -> c_int>,
    pub(crate) xcb_record_client_info_ranges_iterator:
        LazySymbol<unsafe fn(R: *const xcb_record_client_info_t) -> xcb_record_range_iterator_t>,
    pub(crate) xcb_record_client_info_next:
        LazySymbol<unsafe fn(i: *mut xcb_record_client_info_iterator_t)>,
    pub(crate) xcb_record_client_info_end:
        LazySymbol<unsafe fn(i: *mut xcb_record_client_info_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_record_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_record_query_version_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_record_query_version_reply_t,
    >,
    pub(crate) xcb_record_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u16,
            minor_version: u16,
        ) -> xcb_record_query_version_cookie_t,
    >,
    pub(crate) xcb_record_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u16,
            minor_version: u16,
        ) -> xcb_record_query_version_cookie_t,
    >,
    pub(crate) xcb_record_create_context: LazySymbol<
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
    pub(crate) xcb_record_create_context_checked: LazySymbol<
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
    pub(crate) xcb_record_register_clients: LazySymbol<
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
    pub(crate) xcb_record_register_clients_checked: LazySymbol<
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
    pub(crate) xcb_record_unregister_clients: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_record_context_t,
            num_client_specs: u32,
            client_specs: *const xcb_record_client_spec_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_record_unregister_clients_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_record_context_t,
            num_client_specs: u32,
            client_specs: *const xcb_record_client_spec_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_record_get_context_intercepted_clients_length:
        LazySymbol<unsafe fn(R: *const xcb_record_get_context_reply_t) -> c_int>,
    pub(crate) xcb_record_get_context_intercepted_clients_iterator: LazySymbol<
        unsafe fn(R: *const xcb_record_get_context_reply_t) -> xcb_record_client_info_iterator_t,
    >,
    pub(crate) xcb_record_get_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_record_get_context_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_record_get_context_reply_t,
    >,
    pub(crate) xcb_record_get_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_record_context_t,
        ) -> xcb_record_get_context_cookie_t,
    >,
    pub(crate) xcb_record_get_context_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_record_context_t,
        ) -> xcb_record_get_context_cookie_t,
    >,
    pub(crate) xcb_record_enable_context_data:
        LazySymbol<unsafe fn(R: *const xcb_record_enable_context_reply_t) -> *mut u8>,
    pub(crate) xcb_record_enable_context_data_length:
        LazySymbol<unsafe fn(R: *const xcb_record_enable_context_reply_t) -> c_int>,
    pub(crate) xcb_record_enable_context_data_end: LazySymbol<
        unsafe fn(R: *const xcb_record_enable_context_reply_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_record_enable_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_record_enable_context_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_record_enable_context_reply_t,
    >,
    pub(crate) xcb_record_enable_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_record_context_t,
        ) -> xcb_record_enable_context_cookie_t,
    >,
    pub(crate) xcb_record_enable_context_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_record_context_t,
        ) -> xcb_record_enable_context_cookie_t,
    >,
    pub(crate) xcb_record_disable_context: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, context: xcb_record_context_t) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_record_disable_context_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, context: xcb_record_context_t) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_record_free_context: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, context: xcb_record_context_t) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_record_free_context_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, context: xcb_record_context_t) -> xcb_void_cookie_t,
    >,
}
