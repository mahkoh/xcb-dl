// This file was generated using generate.py. Do not edit.

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_client_t {
    pub resource_base: u32,
    pub resource_mask: u32,
}

impl Default for xcb_res_client_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_client_iterator_t {
    pub data: *mut xcb_res_client_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_res_client_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_type_t {
    pub resource_type: xcb_atom_t,
    pub count: u32,
}

impl Default for xcb_res_type_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_type_iterator_t {
    pub data: *mut xcb_res_type_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_res_type_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_res_client_id_mask_t = u32;
pub const XCB_RES_CLIENT_ID_MASK_CLIENT_XID: xcb_res_client_id_mask_t = 1;
pub const XCB_RES_CLIENT_ID_MASK_LOCAL_CLIENT_PID: xcb_res_client_id_mask_t = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_client_id_spec_t {
    pub client: u32,
    pub mask: u32,
}

impl Default for xcb_res_client_id_spec_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_client_id_spec_iterator_t {
    pub data: *mut xcb_res_client_id_spec_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_res_client_id_spec_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_client_id_value_t {
    pub spec: xcb_res_client_id_spec_t,
    pub length: u32,
}

impl Default for xcb_res_client_id_value_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_client_id_value_iterator_t {
    pub data: *mut xcb_res_client_id_value_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_res_client_id_value_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_resource_id_spec_t {
    pub resource: u32,
    pub type_: u32,
}

impl Default for xcb_res_resource_id_spec_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_resource_id_spec_iterator_t {
    pub data: *mut xcb_res_resource_id_spec_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_res_resource_id_spec_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_resource_size_spec_t {
    pub spec: xcb_res_resource_id_spec_t,
    pub bytes: u32,
    pub ref_count: u32,
    pub use_count: u32,
}

impl Default for xcb_res_resource_size_spec_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_resource_size_spec_iterator_t {
    pub data: *mut xcb_res_resource_size_spec_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_res_resource_size_spec_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_resource_size_value_t {
    pub size: xcb_res_resource_size_spec_t,
    pub num_cross_references: u32,
}

impl Default for xcb_res_resource_size_value_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_resource_size_value_iterator_t {
    pub data: *mut xcb_res_resource_size_value_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_res_resource_size_value_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_version_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_res_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_res_query_version.
pub const XCB_RES_QUERY_VERSION: u8 = 0i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client_major: u8,
    pub client_minor: u8,
}

impl Default for xcb_res_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_res_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_clients_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_res_query_clients_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_res_query_clients.
pub const XCB_RES_QUERY_CLIENTS: u8 = 1i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_clients_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_res_query_clients_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_res_query_clients_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_client_resources_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_res_query_client_resources_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_res_query_client_resources.
pub const XCB_RES_QUERY_CLIENT_RESOURCES: u8 = 2i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_client_resources_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub xid: u32,
}

impl Default for xcb_res_query_client_resources_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_res_query_client_resources_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_client_pixmap_bytes_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_res_query_client_pixmap_bytes_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_res_query_client_pixmap_bytes.
pub const XCB_RES_QUERY_CLIENT_PIXMAP_BYTES: u8 = 3i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_client_pixmap_bytes_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub xid: u32,
}

impl Default for xcb_res_query_client_pixmap_bytes_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_res_query_client_pixmap_bytes_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_client_ids_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_res_query_client_ids_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_res_query_client_ids.
pub const XCB_RES_QUERY_CLIENT_IDS: u8 = 4i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_client_ids_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub num_specs: u32,
}

impl Default for xcb_res_query_client_ids_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_res_query_client_ids_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_resource_bytes_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_res_query_resource_bytes_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_res_query_resource_bytes.
pub const XCB_RES_QUERY_RESOURCE_BYTES: u8 = 5i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_resource_bytes_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client: u32,
    pub num_specs: u32,
}

impl Default for xcb_res_query_resource_bytes_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_res_query_resource_bytes_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_res")]
pub(crate) struct XcbResRes {
    xcb_res_id: LazySymbol<*mut xcb_extension_t>,
    xcb_res_client_next: LazySymbol<unsafe fn(i: *mut xcb_res_client_iterator_t)>,
    xcb_res_client_end:
        LazySymbol<unsafe fn(i: xcb_res_client_iterator_t) -> xcb_generic_iterator_t>,
    xcb_res_type_next: LazySymbol<unsafe fn(i: *mut xcb_res_type_iterator_t)>,
    xcb_res_type_end: LazySymbol<unsafe fn(i: xcb_res_type_iterator_t) -> xcb_generic_iterator_t>,
    xcb_res_client_id_spec_next: LazySymbol<unsafe fn(i: *mut xcb_res_client_id_spec_iterator_t)>,
    xcb_res_client_id_spec_end:
        LazySymbol<unsafe fn(i: xcb_res_client_id_spec_iterator_t) -> xcb_generic_iterator_t>,
    xcb_res_client_id_value_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_res_client_id_value_value:
        LazySymbol<unsafe fn(r: *const xcb_res_client_id_value_t) -> *mut u32>,
    xcb_res_client_id_value_value_length:
        LazySymbol<unsafe fn(r: *const xcb_res_client_id_value_t) -> c_int>,
    xcb_res_client_id_value_value_end:
        LazySymbol<unsafe fn(r: *const xcb_res_client_id_value_t) -> xcb_generic_iterator_t>,
    xcb_res_client_id_value_next: LazySymbol<unsafe fn(i: *mut xcb_res_client_id_value_iterator_t)>,
    xcb_res_client_id_value_end:
        LazySymbol<unsafe fn(i: xcb_res_client_id_value_iterator_t) -> xcb_generic_iterator_t>,
    xcb_res_resource_id_spec_next:
        LazySymbol<unsafe fn(i: *mut xcb_res_resource_id_spec_iterator_t)>,
    xcb_res_resource_id_spec_end:
        LazySymbol<unsafe fn(i: xcb_res_resource_id_spec_iterator_t) -> xcb_generic_iterator_t>,
    xcb_res_resource_size_spec_next:
        LazySymbol<unsafe fn(i: *mut xcb_res_resource_size_spec_iterator_t)>,
    xcb_res_resource_size_spec_end:
        LazySymbol<unsafe fn(i: xcb_res_resource_size_spec_iterator_t) -> xcb_generic_iterator_t>,
    xcb_res_resource_size_value_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_res_resource_size_value_cross_references: LazySymbol<
        unsafe fn(r: *const xcb_res_resource_size_value_t) -> *mut xcb_res_resource_size_spec_t,
    >,
    xcb_res_resource_size_value_cross_references_length:
        LazySymbol<unsafe fn(r: *const xcb_res_resource_size_value_t) -> c_int>,
    xcb_res_resource_size_value_cross_references_iterator: LazySymbol<
        unsafe fn(r: *const xcb_res_resource_size_value_t) -> xcb_res_resource_size_spec_iterator_t,
    >,
    xcb_res_resource_size_value_next:
        LazySymbol<unsafe fn(i: *mut xcb_res_resource_size_value_iterator_t)>,
    xcb_res_resource_size_value_end:
        LazySymbol<unsafe fn(i: xcb_res_resource_size_value_iterator_t) -> xcb_generic_iterator_t>,
    xcb_res_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major: u8,
            client_minor: u8,
        ) -> xcb_res_query_version_cookie_t,
    >,
    xcb_res_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major: u8,
            client_minor: u8,
        ) -> xcb_res_query_version_cookie_t,
    >,
    xcb_res_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_res_query_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_res_query_version_reply_t,
    >,
    xcb_res_query_clients_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_res_query_clients:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_res_query_clients_cookie_t>,
    xcb_res_query_clients_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_res_query_clients_cookie_t>,
    xcb_res_query_clients_clients:
        LazySymbol<unsafe fn(r: *const xcb_res_query_clients_reply_t) -> *mut xcb_res_client_t>,
    xcb_res_query_clients_clients_length:
        LazySymbol<unsafe fn(r: *const xcb_res_query_clients_reply_t) -> c_int>,
    xcb_res_query_clients_clients_iterator:
        LazySymbol<unsafe fn(r: *const xcb_res_query_clients_reply_t) -> xcb_res_client_iterator_t>,
    xcb_res_query_clients_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_res_query_clients_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_res_query_clients_reply_t,
    >,
    xcb_res_query_client_resources_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_res_query_client_resources: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, xid: u32) -> xcb_res_query_client_resources_cookie_t,
    >,
    xcb_res_query_client_resources_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, xid: u32) -> xcb_res_query_client_resources_cookie_t,
    >,
    xcb_res_query_client_resources_types: LazySymbol<
        unsafe fn(r: *const xcb_res_query_client_resources_reply_t) -> *mut xcb_res_type_t,
    >,
    xcb_res_query_client_resources_types_length:
        LazySymbol<unsafe fn(r: *const xcb_res_query_client_resources_reply_t) -> c_int>,
    xcb_res_query_client_resources_types_iterator: LazySymbol<
        unsafe fn(r: *const xcb_res_query_client_resources_reply_t) -> xcb_res_type_iterator_t,
    >,
    xcb_res_query_client_resources_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_res_query_client_resources_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_res_query_client_resources_reply_t,
    >,
    xcb_res_query_client_pixmap_bytes: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, xid: u32) -> xcb_res_query_client_pixmap_bytes_cookie_t,
    >,
    xcb_res_query_client_pixmap_bytes_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, xid: u32) -> xcb_res_query_client_pixmap_bytes_cookie_t,
    >,
    xcb_res_query_client_pixmap_bytes_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_res_query_client_pixmap_bytes_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_res_query_client_pixmap_bytes_reply_t,
    >,
    xcb_res_query_client_ids_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_res_query_client_ids: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            num_specs: u32,
            specs: *const xcb_res_client_id_spec_t,
        ) -> xcb_res_query_client_ids_cookie_t,
    >,
    xcb_res_query_client_ids_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            num_specs: u32,
            specs: *const xcb_res_client_id_spec_t,
        ) -> xcb_res_query_client_ids_cookie_t,
    >,
    xcb_res_query_client_ids_ids_length:
        LazySymbol<unsafe fn(r: *const xcb_res_query_client_ids_reply_t) -> c_int>,
    xcb_res_query_client_ids_ids_iterator: LazySymbol<
        unsafe fn(r: *const xcb_res_query_client_ids_reply_t) -> xcb_res_client_id_value_iterator_t,
    >,
    xcb_res_query_client_ids_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_res_query_client_ids_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_res_query_client_ids_reply_t,
    >,
    xcb_res_query_resource_bytes_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_res_query_resource_bytes: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client: u32,
            num_specs: u32,
            specs: *const xcb_res_resource_id_spec_t,
        ) -> xcb_res_query_resource_bytes_cookie_t,
    >,
    xcb_res_query_resource_bytes_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client: u32,
            num_specs: u32,
            specs: *const xcb_res_resource_id_spec_t,
        ) -> xcb_res_query_resource_bytes_cookie_t,
    >,
    xcb_res_query_resource_bytes_sizes_length:
        LazySymbol<unsafe fn(r: *const xcb_res_query_resource_bytes_reply_t) -> c_int>,
    xcb_res_query_resource_bytes_sizes_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_res_query_resource_bytes_reply_t,
        ) -> xcb_res_resource_size_value_iterator_t,
    >,
    xcb_res_query_resource_bytes_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_res_query_resource_bytes_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_res_query_resource_bytes_reply_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self.res.$f.get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .res
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_res")]
impl XcbRes {
    pub fn xcb_res_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_res_id) }
    }

    /// Returns `true` iff the symbol `xcb_res_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_id(&self) -> bool {
        has_sym!(self, xcb_res_id)
    }

    pub unsafe fn xcb_res_client_next(&self, i: *mut xcb_res_client_iterator_t) {
        sym!(self, xcb_res_client_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_res_client_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_client_next(&self) -> bool {
        has_sym!(self, xcb_res_client_next)
    }

    pub unsafe fn xcb_res_client_end(
        &self,
        i: xcb_res_client_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_res_client_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_res_client_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_client_end(&self) -> bool {
        has_sym!(self, xcb_res_client_end)
    }

    pub unsafe fn xcb_res_type_next(&self, i: *mut xcb_res_type_iterator_t) {
        sym!(self, xcb_res_type_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_res_type_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_type_next(&self) -> bool {
        has_sym!(self, xcb_res_type_next)
    }

    pub unsafe fn xcb_res_type_end(&self, i: xcb_res_type_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_res_type_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_res_type_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_type_end(&self) -> bool {
        has_sym!(self, xcb_res_type_end)
    }

    pub unsafe fn xcb_res_client_id_spec_next(&self, i: *mut xcb_res_client_id_spec_iterator_t) {
        sym!(self, xcb_res_client_id_spec_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_res_client_id_spec_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_client_id_spec_next(&self) -> bool {
        has_sym!(self, xcb_res_client_id_spec_next)
    }

    pub unsafe fn xcb_res_client_id_spec_end(
        &self,
        i: xcb_res_client_id_spec_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_res_client_id_spec_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_res_client_id_spec_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_client_id_spec_end(&self) -> bool {
        has_sym!(self, xcb_res_client_id_spec_end)
    }

    pub unsafe fn xcb_res_client_id_value_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_res_client_id_value_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_res_client_id_value_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_client_id_value_sizeof(&self) -> bool {
        has_sym!(self, xcb_res_client_id_value_sizeof)
    }

    pub unsafe fn xcb_res_client_id_value_value(
        &self,
        r: *const xcb_res_client_id_value_t,
    ) -> *mut u32 {
        sym!(self, xcb_res_client_id_value_value)(r)
    }

    /// Returns `true` iff the symbol `xcb_res_client_id_value_value` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_client_id_value_value(&self) -> bool {
        has_sym!(self, xcb_res_client_id_value_value)
    }

    pub unsafe fn xcb_res_client_id_value_value_length(
        &self,
        r: *const xcb_res_client_id_value_t,
    ) -> c_int {
        sym!(self, xcb_res_client_id_value_value_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_res_client_id_value_value_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_client_id_value_value_length(&self) -> bool {
        has_sym!(self, xcb_res_client_id_value_value_length)
    }

    pub unsafe fn xcb_res_client_id_value_value_end(
        &self,
        r: *const xcb_res_client_id_value_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_res_client_id_value_value_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_res_client_id_value_value_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_client_id_value_value_end(&self) -> bool {
        has_sym!(self, xcb_res_client_id_value_value_end)
    }

    pub unsafe fn xcb_res_client_id_value_next(&self, i: *mut xcb_res_client_id_value_iterator_t) {
        sym!(self, xcb_res_client_id_value_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_res_client_id_value_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_client_id_value_next(&self) -> bool {
        has_sym!(self, xcb_res_client_id_value_next)
    }

    pub unsafe fn xcb_res_client_id_value_end(
        &self,
        i: xcb_res_client_id_value_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_res_client_id_value_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_res_client_id_value_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_client_id_value_end(&self) -> bool {
        has_sym!(self, xcb_res_client_id_value_end)
    }

    pub unsafe fn xcb_res_resource_id_spec_next(
        &self,
        i: *mut xcb_res_resource_id_spec_iterator_t,
    ) {
        sym!(self, xcb_res_resource_id_spec_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_res_resource_id_spec_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_resource_id_spec_next(&self) -> bool {
        has_sym!(self, xcb_res_resource_id_spec_next)
    }

    pub unsafe fn xcb_res_resource_id_spec_end(
        &self,
        i: xcb_res_resource_id_spec_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_res_resource_id_spec_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_res_resource_id_spec_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_resource_id_spec_end(&self) -> bool {
        has_sym!(self, xcb_res_resource_id_spec_end)
    }

    pub unsafe fn xcb_res_resource_size_spec_next(
        &self,
        i: *mut xcb_res_resource_size_spec_iterator_t,
    ) {
        sym!(self, xcb_res_resource_size_spec_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_res_resource_size_spec_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_resource_size_spec_next(&self) -> bool {
        has_sym!(self, xcb_res_resource_size_spec_next)
    }

    pub unsafe fn xcb_res_resource_size_spec_end(
        &self,
        i: xcb_res_resource_size_spec_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_res_resource_size_spec_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_res_resource_size_spec_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_resource_size_spec_end(&self) -> bool {
        has_sym!(self, xcb_res_resource_size_spec_end)
    }

    pub unsafe fn xcb_res_resource_size_value_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_res_resource_size_value_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_res_resource_size_value_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_resource_size_value_sizeof(&self) -> bool {
        has_sym!(self, xcb_res_resource_size_value_sizeof)
    }

    pub unsafe fn xcb_res_resource_size_value_cross_references(
        &self,
        r: *const xcb_res_resource_size_value_t,
    ) -> *mut xcb_res_resource_size_spec_t {
        sym!(self, xcb_res_resource_size_value_cross_references)(r)
    }

    /// Returns `true` iff the symbol `xcb_res_resource_size_value_cross_references` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_resource_size_value_cross_references(&self) -> bool {
        has_sym!(self, xcb_res_resource_size_value_cross_references)
    }

    pub unsafe fn xcb_res_resource_size_value_cross_references_length(
        &self,
        r: *const xcb_res_resource_size_value_t,
    ) -> c_int {
        sym!(self, xcb_res_resource_size_value_cross_references_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_res_resource_size_value_cross_references_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_resource_size_value_cross_references_length(&self) -> bool {
        has_sym!(self, xcb_res_resource_size_value_cross_references_length)
    }

    pub unsafe fn xcb_res_resource_size_value_cross_references_iterator(
        &self,
        r: *const xcb_res_resource_size_value_t,
    ) -> xcb_res_resource_size_spec_iterator_t {
        sym!(self, xcb_res_resource_size_value_cross_references_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_res_resource_size_value_cross_references_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_resource_size_value_cross_references_iterator(&self) -> bool {
        has_sym!(self, xcb_res_resource_size_value_cross_references_iterator)
    }

    pub unsafe fn xcb_res_resource_size_value_next(
        &self,
        i: *mut xcb_res_resource_size_value_iterator_t,
    ) {
        sym!(self, xcb_res_resource_size_value_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_res_resource_size_value_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_resource_size_value_next(&self) -> bool {
        has_sym!(self, xcb_res_resource_size_value_next)
    }

    pub unsafe fn xcb_res_resource_size_value_end(
        &self,
        i: xcb_res_resource_size_value_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_res_resource_size_value_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_res_resource_size_value_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_resource_size_value_end(&self) -> bool {
        has_sym!(self, xcb_res_resource_size_value_end)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_res_query_version(
        &self,
        c: *mut xcb_connection_t,
        client_major: u8,
        client_minor: u8,
    ) -> xcb_res_query_version_cookie_t {
        sym!(self, xcb_res_query_version)(c, client_major, client_minor)
    }

    /// Returns `true` iff the symbol `xcb_res_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_version(&self) -> bool {
        has_sym!(self, xcb_res_query_version)
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
    pub unsafe fn xcb_res_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        client_major: u8,
        client_minor: u8,
    ) -> xcb_res_query_version_cookie_t {
        sym!(self, xcb_res_query_version_unchecked)(c, client_major, client_minor)
    }

    /// Returns `true` iff the symbol `xcb_res_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_res_query_version_unchecked)
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
     * xcb_res_query_version_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_res_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_res_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_res_query_version_reply_t {
        sym!(self, xcb_res_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_res_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_res_query_version_reply)
    }

    pub unsafe fn xcb_res_query_clients_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_res_query_clients_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_res_query_clients_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_clients_sizeof(&self) -> bool {
        has_sym!(self, xcb_res_query_clients_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_res_query_clients(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_res_query_clients_cookie_t {
        sym!(self, xcb_res_query_clients)(c)
    }

    /// Returns `true` iff the symbol `xcb_res_query_clients` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_clients(&self) -> bool {
        has_sym!(self, xcb_res_query_clients)
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
    pub unsafe fn xcb_res_query_clients_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_res_query_clients_cookie_t {
        sym!(self, xcb_res_query_clients_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_res_query_clients_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_clients_unchecked(&self) -> bool {
        has_sym!(self, xcb_res_query_clients_unchecked)
    }

    pub unsafe fn xcb_res_query_clients_clients(
        &self,
        r: *const xcb_res_query_clients_reply_t,
    ) -> *mut xcb_res_client_t {
        sym!(self, xcb_res_query_clients_clients)(r)
    }

    /// Returns `true` iff the symbol `xcb_res_query_clients_clients` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_clients_clients(&self) -> bool {
        has_sym!(self, xcb_res_query_clients_clients)
    }

    pub unsafe fn xcb_res_query_clients_clients_length(
        &self,
        r: *const xcb_res_query_clients_reply_t,
    ) -> c_int {
        sym!(self, xcb_res_query_clients_clients_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_res_query_clients_clients_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_clients_clients_length(&self) -> bool {
        has_sym!(self, xcb_res_query_clients_clients_length)
    }

    pub unsafe fn xcb_res_query_clients_clients_iterator(
        &self,
        r: *const xcb_res_query_clients_reply_t,
    ) -> xcb_res_client_iterator_t {
        sym!(self, xcb_res_query_clients_clients_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_res_query_clients_clients_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_clients_clients_iterator(&self) -> bool {
        has_sym!(self, xcb_res_query_clients_clients_iterator)
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
     * xcb_res_query_clients_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_res_query_clients_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_res_query_clients_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_res_query_clients_reply_t {
        sym!(self, xcb_res_query_clients_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_res_query_clients_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_clients_reply(&self) -> bool {
        has_sym!(self, xcb_res_query_clients_reply)
    }

    pub unsafe fn xcb_res_query_client_resources_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_res_query_client_resources_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_res_query_client_resources_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_client_resources_sizeof(&self) -> bool {
        has_sym!(self, xcb_res_query_client_resources_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_res_query_client_resources(
        &self,
        c: *mut xcb_connection_t,
        xid: u32,
    ) -> xcb_res_query_client_resources_cookie_t {
        sym!(self, xcb_res_query_client_resources)(c, xid)
    }

    /// Returns `true` iff the symbol `xcb_res_query_client_resources` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_client_resources(&self) -> bool {
        has_sym!(self, xcb_res_query_client_resources)
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
    pub unsafe fn xcb_res_query_client_resources_unchecked(
        &self,
        c: *mut xcb_connection_t,
        xid: u32,
    ) -> xcb_res_query_client_resources_cookie_t {
        sym!(self, xcb_res_query_client_resources_unchecked)(c, xid)
    }

    /// Returns `true` iff the symbol `xcb_res_query_client_resources_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_client_resources_unchecked(&self) -> bool {
        has_sym!(self, xcb_res_query_client_resources_unchecked)
    }

    pub unsafe fn xcb_res_query_client_resources_types(
        &self,
        r: *const xcb_res_query_client_resources_reply_t,
    ) -> *mut xcb_res_type_t {
        sym!(self, xcb_res_query_client_resources_types)(r)
    }

    /// Returns `true` iff the symbol `xcb_res_query_client_resources_types` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_client_resources_types(&self) -> bool {
        has_sym!(self, xcb_res_query_client_resources_types)
    }

    pub unsafe fn xcb_res_query_client_resources_types_length(
        &self,
        r: *const xcb_res_query_client_resources_reply_t,
    ) -> c_int {
        sym!(self, xcb_res_query_client_resources_types_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_res_query_client_resources_types_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_client_resources_types_length(&self) -> bool {
        has_sym!(self, xcb_res_query_client_resources_types_length)
    }

    pub unsafe fn xcb_res_query_client_resources_types_iterator(
        &self,
        r: *const xcb_res_query_client_resources_reply_t,
    ) -> xcb_res_type_iterator_t {
        sym!(self, xcb_res_query_client_resources_types_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_res_query_client_resources_types_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_client_resources_types_iterator(&self) -> bool {
        has_sym!(self, xcb_res_query_client_resources_types_iterator)
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
     * xcb_res_query_client_resources_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_res_query_client_resources_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_res_query_client_resources_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_res_query_client_resources_reply_t {
        sym!(self, xcb_res_query_client_resources_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_res_query_client_resources_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_client_resources_reply(&self) -> bool {
        has_sym!(self, xcb_res_query_client_resources_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_res_query_client_pixmap_bytes(
        &self,
        c: *mut xcb_connection_t,
        xid: u32,
    ) -> xcb_res_query_client_pixmap_bytes_cookie_t {
        sym!(self, xcb_res_query_client_pixmap_bytes)(c, xid)
    }

    /// Returns `true` iff the symbol `xcb_res_query_client_pixmap_bytes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_client_pixmap_bytes(&self) -> bool {
        has_sym!(self, xcb_res_query_client_pixmap_bytes)
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
    pub unsafe fn xcb_res_query_client_pixmap_bytes_unchecked(
        &self,
        c: *mut xcb_connection_t,
        xid: u32,
    ) -> xcb_res_query_client_pixmap_bytes_cookie_t {
        sym!(self, xcb_res_query_client_pixmap_bytes_unchecked)(c, xid)
    }

    /// Returns `true` iff the symbol `xcb_res_query_client_pixmap_bytes_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_client_pixmap_bytes_unchecked(&self) -> bool {
        has_sym!(self, xcb_res_query_client_pixmap_bytes_unchecked)
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
     * xcb_res_query_client_pixmap_bytes_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_res_query_client_pixmap_bytes_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_res_query_client_pixmap_bytes_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_res_query_client_pixmap_bytes_reply_t {
        sym!(self, xcb_res_query_client_pixmap_bytes_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_res_query_client_pixmap_bytes_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_client_pixmap_bytes_reply(&self) -> bool {
        has_sym!(self, xcb_res_query_client_pixmap_bytes_reply)
    }

    pub unsafe fn xcb_res_query_client_ids_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_res_query_client_ids_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_res_query_client_ids_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_client_ids_sizeof(&self) -> bool {
        has_sym!(self, xcb_res_query_client_ids_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_res_query_client_ids(
        &self,
        c: *mut xcb_connection_t,
        num_specs: u32,
        specs: *const xcb_res_client_id_spec_t,
    ) -> xcb_res_query_client_ids_cookie_t {
        sym!(self, xcb_res_query_client_ids)(c, num_specs, specs)
    }

    /// Returns `true` iff the symbol `xcb_res_query_client_ids` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_client_ids(&self) -> bool {
        has_sym!(self, xcb_res_query_client_ids)
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
    pub unsafe fn xcb_res_query_client_ids_unchecked(
        &self,
        c: *mut xcb_connection_t,
        num_specs: u32,
        specs: *const xcb_res_client_id_spec_t,
    ) -> xcb_res_query_client_ids_cookie_t {
        sym!(self, xcb_res_query_client_ids_unchecked)(c, num_specs, specs)
    }

    /// Returns `true` iff the symbol `xcb_res_query_client_ids_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_client_ids_unchecked(&self) -> bool {
        has_sym!(self, xcb_res_query_client_ids_unchecked)
    }

    pub unsafe fn xcb_res_query_client_ids_ids_length(
        &self,
        r: *const xcb_res_query_client_ids_reply_t,
    ) -> c_int {
        sym!(self, xcb_res_query_client_ids_ids_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_res_query_client_ids_ids_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_client_ids_ids_length(&self) -> bool {
        has_sym!(self, xcb_res_query_client_ids_ids_length)
    }

    pub unsafe fn xcb_res_query_client_ids_ids_iterator(
        &self,
        r: *const xcb_res_query_client_ids_reply_t,
    ) -> xcb_res_client_id_value_iterator_t {
        sym!(self, xcb_res_query_client_ids_ids_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_res_query_client_ids_ids_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_client_ids_ids_iterator(&self) -> bool {
        has_sym!(self, xcb_res_query_client_ids_ids_iterator)
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
     * xcb_res_query_client_ids_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_res_query_client_ids_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_res_query_client_ids_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_res_query_client_ids_reply_t {
        sym!(self, xcb_res_query_client_ids_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_res_query_client_ids_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_client_ids_reply(&self) -> bool {
        has_sym!(self, xcb_res_query_client_ids_reply)
    }

    pub unsafe fn xcb_res_query_resource_bytes_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_res_query_resource_bytes_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_res_query_resource_bytes_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_resource_bytes_sizeof(&self) -> bool {
        has_sym!(self, xcb_res_query_resource_bytes_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_res_query_resource_bytes(
        &self,
        c: *mut xcb_connection_t,
        client: u32,
        num_specs: u32,
        specs: *const xcb_res_resource_id_spec_t,
    ) -> xcb_res_query_resource_bytes_cookie_t {
        sym!(self, xcb_res_query_resource_bytes)(c, client, num_specs, specs)
    }

    /// Returns `true` iff the symbol `xcb_res_query_resource_bytes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_resource_bytes(&self) -> bool {
        has_sym!(self, xcb_res_query_resource_bytes)
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
    pub unsafe fn xcb_res_query_resource_bytes_unchecked(
        &self,
        c: *mut xcb_connection_t,
        client: u32,
        num_specs: u32,
        specs: *const xcb_res_resource_id_spec_t,
    ) -> xcb_res_query_resource_bytes_cookie_t {
        sym!(self, xcb_res_query_resource_bytes_unchecked)(c, client, num_specs, specs)
    }

    /// Returns `true` iff the symbol `xcb_res_query_resource_bytes_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_resource_bytes_unchecked(&self) -> bool {
        has_sym!(self, xcb_res_query_resource_bytes_unchecked)
    }

    pub unsafe fn xcb_res_query_resource_bytes_sizes_length(
        &self,
        r: *const xcb_res_query_resource_bytes_reply_t,
    ) -> c_int {
        sym!(self, xcb_res_query_resource_bytes_sizes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_res_query_resource_bytes_sizes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_resource_bytes_sizes_length(&self) -> bool {
        has_sym!(self, xcb_res_query_resource_bytes_sizes_length)
    }

    pub unsafe fn xcb_res_query_resource_bytes_sizes_iterator(
        &self,
        r: *const xcb_res_query_resource_bytes_reply_t,
    ) -> xcb_res_resource_size_value_iterator_t {
        sym!(self, xcb_res_query_resource_bytes_sizes_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_res_query_resource_bytes_sizes_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_resource_bytes_sizes_iterator(&self) -> bool {
        has_sym!(self, xcb_res_query_resource_bytes_sizes_iterator)
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
     * xcb_res_query_resource_bytes_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_res_query_resource_bytes_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_res_query_resource_bytes_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_res_query_resource_bytes_reply_t {
        sym!(self, xcb_res_query_resource_bytes_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_res_query_resource_bytes_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_resource_bytes_reply(&self) -> bool {
        has_sym!(self, xcb_res_query_resource_bytes_reply)
    }
}

#[cfg(feature = "xcb_res")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbRes::load().unwrap() };
        assert!(lib.has_xcb_res_id());
        assert!(lib.has_xcb_res_client_next());
        assert!(lib.has_xcb_res_client_end());
        assert!(lib.has_xcb_res_type_next());
        assert!(lib.has_xcb_res_type_end());
        assert!(lib.has_xcb_res_client_id_spec_next());
        assert!(lib.has_xcb_res_client_id_spec_end());
        assert!(lib.has_xcb_res_client_id_value_sizeof());
        assert!(lib.has_xcb_res_client_id_value_value());
        assert!(lib.has_xcb_res_client_id_value_value_length());
        assert!(lib.has_xcb_res_client_id_value_value_end());
        assert!(lib.has_xcb_res_client_id_value_next());
        assert!(lib.has_xcb_res_client_id_value_end());
        assert!(lib.has_xcb_res_resource_id_spec_next());
        assert!(lib.has_xcb_res_resource_id_spec_end());
        assert!(lib.has_xcb_res_resource_size_spec_next());
        assert!(lib.has_xcb_res_resource_size_spec_end());
        assert!(lib.has_xcb_res_resource_size_value_sizeof());
        assert!(lib.has_xcb_res_resource_size_value_cross_references());
        assert!(lib.has_xcb_res_resource_size_value_cross_references_length());
        assert!(lib.has_xcb_res_resource_size_value_cross_references_iterator());
        assert!(lib.has_xcb_res_resource_size_value_next());
        assert!(lib.has_xcb_res_resource_size_value_end());
        assert!(lib.has_xcb_res_query_version());
        assert!(lib.has_xcb_res_query_version_unchecked());
        assert!(lib.has_xcb_res_query_version_reply());
        assert!(lib.has_xcb_res_query_clients_sizeof());
        assert!(lib.has_xcb_res_query_clients());
        assert!(lib.has_xcb_res_query_clients_unchecked());
        assert!(lib.has_xcb_res_query_clients_clients());
        assert!(lib.has_xcb_res_query_clients_clients_length());
        assert!(lib.has_xcb_res_query_clients_clients_iterator());
        assert!(lib.has_xcb_res_query_clients_reply());
        assert!(lib.has_xcb_res_query_client_resources_sizeof());
        assert!(lib.has_xcb_res_query_client_resources());
        assert!(lib.has_xcb_res_query_client_resources_unchecked());
        assert!(lib.has_xcb_res_query_client_resources_types());
        assert!(lib.has_xcb_res_query_client_resources_types_length());
        assert!(lib.has_xcb_res_query_client_resources_types_iterator());
        assert!(lib.has_xcb_res_query_client_resources_reply());
        assert!(lib.has_xcb_res_query_client_pixmap_bytes());
        assert!(lib.has_xcb_res_query_client_pixmap_bytes_unchecked());
        assert!(lib.has_xcb_res_query_client_pixmap_bytes_reply());
        assert!(lib.has_xcb_res_query_client_ids_sizeof());
        assert!(lib.has_xcb_res_query_client_ids());
        assert!(lib.has_xcb_res_query_client_ids_unchecked());
        assert!(lib.has_xcb_res_query_client_ids_ids_length());
        assert!(lib.has_xcb_res_query_client_ids_ids_iterator());
        assert!(lib.has_xcb_res_query_client_ids_reply());
        assert!(lib.has_xcb_res_query_resource_bytes_sizeof());
        assert!(lib.has_xcb_res_query_resource_bytes());
        assert!(lib.has_xcb_res_query_resource_bytes_unchecked());
        assert!(lib.has_xcb_res_query_resource_bytes_sizes_length());
        assert!(lib.has_xcb_res_query_resource_bytes_sizes_iterator());
        assert!(lib.has_xcb_res_query_resource_bytes_reply());
    }
}
