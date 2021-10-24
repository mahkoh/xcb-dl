use crate::*;
use std::os::raw::*;

pub const XCB_RES_MAJOR_VERSION: u32 = 1;
pub const XCB_RES_MINOR_VERSION: u32 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_client_t {
    pub resource_base: u32,
    pub resource_mask: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_client_iterator_t {
    pub data: *mut xcb_res_client_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_type_t {
    pub resource_type: xcb_atom_t,
    pub count: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_type_iterator_t {
    pub data: *mut xcb_res_type_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_res_client_id_mask_t = u32;
pub const XCB_RES_CLIENT_ID_MASK_CLIENT_XID: xcb_res_client_id_mask_t = 0x01;
pub const XCB_RES_CLIENT_ID_MASK_LOCAL_CLIENT_PID: xcb_res_client_id_mask_t = 0x02;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_client_id_spec_t {
    pub client: u32,
    pub mask: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_client_id_spec_iterator_t {
    pub data: *mut xcb_res_client_id_spec_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_client_id_value_t {
    pub spec: xcb_res_client_id_spec_t,
    pub length: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_client_id_value_iterator_t {
    pub data: *mut xcb_res_client_id_value_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_resource_id_spec_t {
    pub resource: u32,
    pub type_: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_resource_id_spec_iterator_t {
    pub data: *mut xcb_res_resource_id_spec_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_resource_size_spec_t {
    pub spec: xcb_res_resource_id_spec_t,
    pub bytes: u32,
    pub ref_count: u32,
    pub use_count: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_resource_size_spec_iterator_t {
    pub data: *mut xcb_res_resource_size_spec_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_resource_size_value_t {
    pub size: xcb_res_resource_size_spec_t,
    pub num_cross_references: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_resource_size_value_iterator_t {
    pub data: *mut xcb_res_resource_size_value_t,
    pub rem: c_int,
    pub index: c_int,
}

pub const XCB_RES_QUERY_VERSION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client_major: u8,
    pub client_minor: u8,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_version_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major: u16,
    pub server_minor: u16,
}

pub const XCB_RES_QUERY_CLIENTS: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_clients_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_clients_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_clients_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_clients: u32,
    pub pad1: [u8; 20],
}

pub const XCB_RES_QUERY_CLIENT_RESOURCES: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_client_resources_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub xid: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_client_resources_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_client_resources_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_types: u32,
    pub pad1: [u8; 20],
}

pub const XCB_RES_QUERY_CLIENT_PIXMAP_BYTES: u8 = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_client_pixmap_bytes_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub xid: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_client_pixmap_bytes_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_client_pixmap_bytes_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub bytes: u32,
    pub bytes_overflow: u32,
}

pub const XCB_RES_QUERY_CLIENT_IDS: u8 = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_client_ids_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub num_specs: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_client_ids_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_client_ids_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_ids: u32,
    pub pad1: [u8; 20],
}

pub const XCB_RES_QUERY_RESOURCE_BYTES: u8 = 5;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_resource_bytes_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client: u32,
    pub num_specs: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_resource_bytes_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_resource_bytes_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_sizes: u32,
    pub pad1: [u8; 20],
}

impl XcbRes {
    #[inline]
    pub fn xcb_res_id(&self) -> *mut xcb_extension_t {
        call!(self, xcb_res_id)
    }

    #[inline]
    pub unsafe fn xcb_res_client_next(&self, i: *mut xcb_res_client_iterator_t) {
        call!(self, xcb_res_client_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_res_client_end(
        &self,
        i: *mut xcb_res_client_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_res_client_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_res_type_next(&self, i: *mut xcb_res_type_iterator_t) {
        call!(self, xcb_res_type_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_res_type_end(
        &self,
        i: *mut xcb_res_type_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_res_type_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_res_client_id_spec_next(&self, i: *mut xcb_res_client_id_spec_iterator_t) {
        call!(self, xcb_res_client_id_spec_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_res_client_id_spec_end(
        &self,
        i: *mut xcb_res_client_id_spec_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_res_client_id_spec_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_res_client_id_value_value(
        &self,
        R: *const xcb_res_client_id_value_t,
    ) -> *mut u32 {
        call!(self, xcb_res_client_id_value_value)(R)
    }

    #[inline]
    pub unsafe fn xcb_res_client_id_value_value_length(
        &self,
        R: *const xcb_res_client_id_value_t,
    ) -> c_int {
        call!(self, xcb_res_client_id_value_value_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_res_client_id_value_value_end(
        &self,
        R: *const xcb_res_client_id_value_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_res_client_id_value_value_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_res_client_id_value_next(&self, i: *mut xcb_res_client_id_value_iterator_t) {
        call!(self, xcb_res_client_id_value_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_res_client_id_value_end(
        &self,
        i: *mut xcb_res_client_id_value_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_res_client_id_value_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_res_resource_id_spec_next(
        &self,
        i: *mut xcb_res_resource_id_spec_iterator_t,
    ) {
        call!(self, xcb_res_resource_id_spec_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_res_resource_id_spec_end(
        &self,
        i: *mut xcb_res_resource_id_spec_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_res_resource_id_spec_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_res_resource_size_spec_next(
        &self,
        i: *mut xcb_res_resource_size_spec_iterator_t,
    ) {
        call!(self, xcb_res_resource_size_spec_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_res_resource_size_spec_end(
        &self,
        i: *mut xcb_res_resource_size_spec_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_res_resource_size_spec_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_res_resource_size_value_cross_references(
        &self,
        R: *const xcb_res_resource_size_value_t,
    ) -> *mut xcb_res_resource_size_spec_t {
        call!(self, xcb_res_resource_size_value_cross_references)(R)
    }

    #[inline]
    pub unsafe fn xcb_res_resource_size_value_cross_references_length(
        &self,
        R: *const xcb_res_resource_size_value_t,
    ) -> c_int {
        call!(self, xcb_res_resource_size_value_cross_references_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_res_resource_size_value_cross_references_iterator(
        &self,
        R: *const xcb_res_resource_size_value_t,
    ) -> xcb_res_resource_size_spec_iterator_t {
        call!(self, xcb_res_resource_size_value_cross_references_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_res_resource_size_value_next(
        &self,
        i: *mut xcb_res_resource_size_value_iterator_t,
    ) {
        call!(self, xcb_res_resource_size_value_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_res_resource_size_value_end(
        &self,
        i: *mut xcb_res_resource_size_value_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_res_resource_size_value_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_res_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_res_query_version_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_res_query_version_reply_t {
        call!(self, xcb_res_query_version_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_res_query_version(
        &self,
        c: *mut xcb_connection_t,
        client_major: u8,
        client_minor: u8,
    ) -> xcb_res_query_version_cookie_t {
        call!(self, xcb_res_query_version)(c, client_major, client_minor)
    }

    #[inline]
    pub unsafe fn xcb_res_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        client_major: u8,
        client_minor: u8,
    ) -> xcb_res_query_version_cookie_t {
        call!(self, xcb_res_query_version_unchecked)(c, client_major, client_minor)
    }

    #[inline]
    pub unsafe fn xcb_res_query_clients_clients(
        &self,
        R: *const xcb_res_query_clients_reply_t,
    ) -> *mut xcb_res_client_t {
        call!(self, xcb_res_query_clients_clients)(R)
    }

    #[inline]
    pub unsafe fn xcb_res_query_clients_clients_length(
        &self,
        R: *const xcb_res_query_clients_reply_t,
    ) -> c_int {
        call!(self, xcb_res_query_clients_clients_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_res_query_clients_clients_iterator(
        &self,
        R: *const xcb_res_query_clients_reply_t,
    ) -> xcb_res_client_iterator_t {
        call!(self, xcb_res_query_clients_clients_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_res_query_clients_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_res_query_clients_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_res_query_clients_reply_t {
        call!(self, xcb_res_query_clients_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_res_query_clients(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_res_query_clients_cookie_t {
        call!(self, xcb_res_query_clients)(c)
    }

    #[inline]
    pub unsafe fn xcb_res_query_clients_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_res_query_clients_cookie_t {
        call!(self, xcb_res_query_clients_unchecked)(c)
    }

    #[inline]
    pub unsafe fn xcb_res_query_client_resources_types(
        &self,
        R: *const xcb_res_query_client_resources_reply_t,
    ) -> *mut xcb_res_type_t {
        call!(self, xcb_res_query_client_resources_types)(R)
    }

    #[inline]
    pub unsafe fn xcb_res_query_client_resources_types_length(
        &self,
        R: *const xcb_res_query_client_resources_reply_t,
    ) -> c_int {
        call!(self, xcb_res_query_client_resources_types_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_res_query_client_resources_types_iterator(
        &self,
        R: *const xcb_res_query_client_resources_reply_t,
    ) -> xcb_res_type_iterator_t {
        call!(self, xcb_res_query_client_resources_types_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_res_query_client_resources_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_res_query_client_resources_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_res_query_client_resources_reply_t {
        call!(self, xcb_res_query_client_resources_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_res_query_client_resources(
        &self,
        c: *mut xcb_connection_t,
        xid: u32,
    ) -> xcb_res_query_client_resources_cookie_t {
        call!(self, xcb_res_query_client_resources)(c, xid)
    }

    #[inline]
    pub unsafe fn xcb_res_query_client_resources_unchecked(
        &self,
        c: *mut xcb_connection_t,
        xid: u32,
    ) -> xcb_res_query_client_resources_cookie_t {
        call!(self, xcb_res_query_client_resources_unchecked)(c, xid)
    }

    #[inline]
    pub unsafe fn xcb_res_query_client_pixmap_bytes_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_res_query_client_pixmap_bytes_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_res_query_client_pixmap_bytes_reply_t {
        call!(self, xcb_res_query_client_pixmap_bytes_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_res_query_client_pixmap_bytes(
        &self,
        c: *mut xcb_connection_t,
        xid: u32,
    ) -> xcb_res_query_client_pixmap_bytes_cookie_t {
        call!(self, xcb_res_query_client_pixmap_bytes)(c, xid)
    }

    #[inline]
    pub unsafe fn xcb_res_query_client_pixmap_bytes_unchecked(
        &self,
        c: *mut xcb_connection_t,
        xid: u32,
    ) -> xcb_res_query_client_pixmap_bytes_cookie_t {
        call!(self, xcb_res_query_client_pixmap_bytes_unchecked)(c, xid)
    }

    #[inline]
    pub unsafe fn xcb_res_query_client_ids_ids_length(
        &self,
        R: *const xcb_res_query_client_ids_reply_t,
    ) -> c_int {
        call!(self, xcb_res_query_client_ids_ids_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_res_query_client_ids_ids_iterator(
        &self,
        R: *const xcb_res_query_client_ids_reply_t,
    ) -> xcb_res_client_id_value_iterator_t {
        call!(self, xcb_res_query_client_ids_ids_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_res_query_client_ids_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_res_query_client_ids_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_res_query_client_ids_reply_t {
        call!(self, xcb_res_query_client_ids_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_res_query_client_ids(
        &self,
        c: *mut xcb_connection_t,
        num_specs: u32,
        specs: *const xcb_res_client_id_spec_t,
    ) -> xcb_res_query_client_ids_cookie_t {
        call!(self, xcb_res_query_client_ids)(c, num_specs, specs)
    }

    #[inline]
    pub unsafe fn xcb_res_query_client_ids_unchecked(
        &self,
        c: *mut xcb_connection_t,
        num_specs: u32,
        specs: *const xcb_res_client_id_spec_t,
    ) -> xcb_res_query_client_ids_cookie_t {
        call!(self, xcb_res_query_client_ids_unchecked)(c, num_specs, specs)
    }

    #[inline]
    pub unsafe fn xcb_res_query_resource_bytes_sizes_length(
        &self,
        R: *const xcb_res_query_resource_bytes_reply_t,
    ) -> c_int {
        call!(self, xcb_res_query_resource_bytes_sizes_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_res_query_resource_bytes_sizes_iterator(
        &self,
        R: *const xcb_res_query_resource_bytes_reply_t,
    ) -> xcb_res_resource_size_value_iterator_t {
        call!(self, xcb_res_query_resource_bytes_sizes_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_res_query_resource_bytes_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_res_query_resource_bytes_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_res_query_resource_bytes_reply_t {
        call!(self, xcb_res_query_resource_bytes_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_res_query_resource_bytes(
        &self,
        c: *mut xcb_connection_t,
        client: u32,
        num_specs: u32,
        specs: *const xcb_res_resource_id_spec_t,
    ) -> xcb_res_query_resource_bytes_cookie_t {
        call!(self, xcb_res_query_resource_bytes)(c, client, num_specs, specs)
    }

    #[inline]
    pub unsafe fn xcb_res_query_resource_bytes_unchecked(
        &self,
        c: *mut xcb_connection_t,
        client: u32,
        num_specs: u32,
        specs: *const xcb_res_resource_id_spec_t,
    ) -> xcb_res_query_resource_bytes_cookie_t {
        call!(self, xcb_res_query_resource_bytes_unchecked)(c, client, num_specs, specs)
    }
}

pub struct XcbRes {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_res_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_res_client_next: LazySymbol<unsafe fn(i: *mut xcb_res_client_iterator_t)>,
    pub(crate) xcb_res_client_end:
        LazySymbol<unsafe fn(i: *mut xcb_res_client_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_res_type_next: LazySymbol<unsafe fn(i: *mut xcb_res_type_iterator_t)>,
    pub(crate) xcb_res_type_end:
        LazySymbol<unsafe fn(i: *mut xcb_res_type_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_res_client_id_spec_next:
        LazySymbol<unsafe fn(i: *mut xcb_res_client_id_spec_iterator_t)>,
    pub(crate) xcb_res_client_id_spec_end:
        LazySymbol<unsafe fn(i: *mut xcb_res_client_id_spec_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_res_client_id_value_value:
        LazySymbol<unsafe fn(R: *const xcb_res_client_id_value_t) -> *mut u32>,
    pub(crate) xcb_res_client_id_value_value_length:
        LazySymbol<unsafe fn(R: *const xcb_res_client_id_value_t) -> c_int>,
    pub(crate) xcb_res_client_id_value_value_end:
        LazySymbol<unsafe fn(R: *const xcb_res_client_id_value_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_res_client_id_value_next:
        LazySymbol<unsafe fn(i: *mut xcb_res_client_id_value_iterator_t)>,
    pub(crate) xcb_res_client_id_value_end:
        LazySymbol<unsafe fn(i: *mut xcb_res_client_id_value_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_res_resource_id_spec_next:
        LazySymbol<unsafe fn(i: *mut xcb_res_resource_id_spec_iterator_t)>,
    pub(crate) xcb_res_resource_id_spec_end: LazySymbol<
        unsafe fn(i: *mut xcb_res_resource_id_spec_iterator_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_res_resource_size_spec_next:
        LazySymbol<unsafe fn(i: *mut xcb_res_resource_size_spec_iterator_t)>,
    pub(crate) xcb_res_resource_size_spec_end: LazySymbol<
        unsafe fn(i: *mut xcb_res_resource_size_spec_iterator_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_res_resource_size_value_cross_references: LazySymbol<
        unsafe fn(R: *const xcb_res_resource_size_value_t) -> *mut xcb_res_resource_size_spec_t,
    >,
    pub(crate) xcb_res_resource_size_value_cross_references_length:
        LazySymbol<unsafe fn(R: *const xcb_res_resource_size_value_t) -> c_int>,
    pub(crate) xcb_res_resource_size_value_cross_references_iterator: LazySymbol<
        unsafe fn(R: *const xcb_res_resource_size_value_t) -> xcb_res_resource_size_spec_iterator_t,
    >,
    pub(crate) xcb_res_resource_size_value_next:
        LazySymbol<unsafe fn(i: *mut xcb_res_resource_size_value_iterator_t)>,
    pub(crate) xcb_res_resource_size_value_end: LazySymbol<
        unsafe fn(i: *mut xcb_res_resource_size_value_iterator_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_res_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_res_query_version_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_res_query_version_reply_t,
    >,
    pub(crate) xcb_res_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major: u8,
            client_minor: u8,
        ) -> xcb_res_query_version_cookie_t,
    >,
    pub(crate) xcb_res_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major: u8,
            client_minor: u8,
        ) -> xcb_res_query_version_cookie_t,
    >,
    pub(crate) xcb_res_query_clients_clients:
        LazySymbol<unsafe fn(R: *const xcb_res_query_clients_reply_t) -> *mut xcb_res_client_t>,
    pub(crate) xcb_res_query_clients_clients_length:
        LazySymbol<unsafe fn(R: *const xcb_res_query_clients_reply_t) -> c_int>,
    pub(crate) xcb_res_query_clients_clients_iterator:
        LazySymbol<unsafe fn(R: *const xcb_res_query_clients_reply_t) -> xcb_res_client_iterator_t>,
    pub(crate) xcb_res_query_clients_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_res_query_clients_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_res_query_clients_reply_t,
    >,
    pub(crate) xcb_res_query_clients:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_res_query_clients_cookie_t>,
    pub(crate) xcb_res_query_clients_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_res_query_clients_cookie_t>,
    pub(crate) xcb_res_query_client_resources_types: LazySymbol<
        unsafe fn(R: *const xcb_res_query_client_resources_reply_t) -> *mut xcb_res_type_t,
    >,
    pub(crate) xcb_res_query_client_resources_types_length:
        LazySymbol<unsafe fn(R: *const xcb_res_query_client_resources_reply_t) -> c_int>,
    pub(crate) xcb_res_query_client_resources_types_iterator: LazySymbol<
        unsafe fn(R: *const xcb_res_query_client_resources_reply_t) -> xcb_res_type_iterator_t,
    >,
    pub(crate) xcb_res_query_client_resources_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_res_query_client_resources_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_res_query_client_resources_reply_t,
    >,
    pub(crate) xcb_res_query_client_resources: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, xid: u32) -> xcb_res_query_client_resources_cookie_t,
    >,
    pub(crate) xcb_res_query_client_resources_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, xid: u32) -> xcb_res_query_client_resources_cookie_t,
    >,
    pub(crate) xcb_res_query_client_pixmap_bytes_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_res_query_client_pixmap_bytes_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_res_query_client_pixmap_bytes_reply_t,
    >,
    pub(crate) xcb_res_query_client_pixmap_bytes: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, xid: u32) -> xcb_res_query_client_pixmap_bytes_cookie_t,
    >,
    pub(crate) xcb_res_query_client_pixmap_bytes_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, xid: u32) -> xcb_res_query_client_pixmap_bytes_cookie_t,
    >,
    pub(crate) xcb_res_query_client_ids_ids_length:
        LazySymbol<unsafe fn(R: *const xcb_res_query_client_ids_reply_t) -> c_int>,
    pub(crate) xcb_res_query_client_ids_ids_iterator: LazySymbol<
        unsafe fn(R: *const xcb_res_query_client_ids_reply_t) -> xcb_res_client_id_value_iterator_t,
    >,
    pub(crate) xcb_res_query_client_ids_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_res_query_client_ids_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_res_query_client_ids_reply_t,
    >,
    pub(crate) xcb_res_query_client_ids: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            num_specs: u32,
            specs: *const xcb_res_client_id_spec_t,
        ) -> xcb_res_query_client_ids_cookie_t,
    >,
    pub(crate) xcb_res_query_client_ids_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            num_specs: u32,
            specs: *const xcb_res_client_id_spec_t,
        ) -> xcb_res_query_client_ids_cookie_t,
    >,
    pub(crate) xcb_res_query_resource_bytes_sizes_length:
        LazySymbol<unsafe fn(R: *const xcb_res_query_resource_bytes_reply_t) -> c_int>,
    pub(crate) xcb_res_query_resource_bytes_sizes_iterator: LazySymbol<
        unsafe fn(
            R: *const xcb_res_query_resource_bytes_reply_t,
        ) -> xcb_res_resource_size_value_iterator_t,
    >,
    pub(crate) xcb_res_query_resource_bytes_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_res_query_resource_bytes_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_res_query_resource_bytes_reply_t,
    >,
    pub(crate) xcb_res_query_resource_bytes: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client: u32,
            num_specs: u32,
            specs: *const xcb_res_resource_id_spec_t,
        ) -> xcb_res_query_resource_bytes_cookie_t,
    >,
    pub(crate) xcb_res_query_resource_bytes_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client: u32,
            num_specs: u32,
            specs: *const xcb_res_resource_id_spec_t,
        ) -> xcb_res_query_resource_bytes_cookie_t,
    >,
}
