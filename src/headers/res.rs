// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The name of the `Res` extension.
pub const XCB_RES_NAME: &[u8] = b"X-Resource";

/// The name of the `Res` extension.
pub const XCB_RES_NAME_STR: &str = "X-Resource";

/// The `Res::Client` struct.
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

/// An iterator over `Res::Client` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_client_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_res_client_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_res_client_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Res::Type` struct.
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

/// An iterator over `Res::Type` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_type_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_res_type_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_res_type_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Res::ClientIdMask` enum.
///
/// This enum has the following variants:
///
/// - [`Res::ClientIdMask::ClientXID`](XCB_RES_CLIENT_ID_MASK_CLIENT_XID)
/// - [`Res::ClientIdMask::LocalClientPID`](XCB_RES_CLIENT_ID_MASK_LOCAL_CLIENT_PID)
pub type xcb_res_client_id_mask_t = u32;
/// The `Res::ClientIdMask::ClientXID` enum variant.
///
/// This is a variant of [`xcb_res_client_id_mask_t`].
pub const XCB_RES_CLIENT_ID_MASK_CLIENT_XID: xcb_res_client_id_mask_t = 1;
/// The `Res::ClientIdMask::LocalClientPID` enum variant.
///
/// This is a variant of [`xcb_res_client_id_mask_t`].
pub const XCB_RES_CLIENT_ID_MASK_LOCAL_CLIENT_PID: xcb_res_client_id_mask_t = 2;

/// The `Res::ClientIdSpec` struct.
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

/// An iterator over `Res::ClientIdSpec` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_client_id_spec_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_res_client_id_spec_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_res_client_id_spec_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Res::ClientIdValue` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `value`
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

/// An iterator over `Res::ClientIdValue` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_client_id_value_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_res_client_id_value_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_res_client_id_value_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Res::ResourceIdSpec` struct.
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

/// An iterator over `Res::ResourceIdSpec` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_resource_id_spec_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_res_resource_id_spec_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_res_resource_id_spec_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Res::ResourceSizeSpec` struct.
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

/// An iterator over `Res::ResourceSizeSpec` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_resource_size_spec_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_res_resource_size_spec_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_res_resource_size_spec_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Res::ResourceSizeValue` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `cross_references`
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

/// An iterator over `Res::ResourceSizeValue` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_resource_size_value_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_res_resource_size_value_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_res_resource_size_value_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Res::QueryVersion` request.
///
/// Pass this cookie to [`xcb_res_query_version_reply`] to retrieve the reply.
///
/// [`xcb_res_query_version_reply`]: XcbRes::xcb_res_query_version_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_version_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_res_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Res::QueryVersion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRes::xcb_res_id()`], then the type of the request is
/// [`xcb_res_query_version_request_t`].
pub const XCB_RES_QUERY_VERSION: u8 = 0i32 as u8;

/// The `Res::QueryVersion` request.
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

/// The `Res::QueryVersion` reply.
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

/// The cookie for the reply to a `Res::QueryClients` request.
///
/// Pass this cookie to [`xcb_res_query_clients_reply`] to retrieve the reply.
///
/// [`xcb_res_query_clients_reply`]: XcbRes::xcb_res_query_clients_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_clients_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_res_query_clients_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Res::QueryClients` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRes::xcb_res_id()`], then the type of the request is
/// [`xcb_res_query_clients_request_t`].
pub const XCB_RES_QUERY_CLIENTS: u8 = 1i32 as u8;

/// The `Res::QueryClients` request.
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

/// The `Res::QueryClients` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `clients`
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

/// The cookie for the reply to a `Res::QueryClientResources` request.
///
/// Pass this cookie to [`xcb_res_query_client_resources_reply`] to retrieve the reply.
///
/// [`xcb_res_query_client_resources_reply`]: XcbRes::xcb_res_query_client_resources_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_client_resources_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_res_query_client_resources_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Res::QueryClientResources` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRes::xcb_res_id()`], then the type of the request is
/// [`xcb_res_query_client_resources_request_t`].
pub const XCB_RES_QUERY_CLIENT_RESOURCES: u8 = 2i32 as u8;

/// The `Res::QueryClientResources` request.
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

/// The `Res::QueryClientResources` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `types`
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

/// The cookie for the reply to a `Res::QueryClientPixmapBytes` request.
///
/// Pass this cookie to [`xcb_res_query_client_pixmap_bytes_reply`] to retrieve the reply.
///
/// [`xcb_res_query_client_pixmap_bytes_reply`]: XcbRes::xcb_res_query_client_pixmap_bytes_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_client_pixmap_bytes_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_res_query_client_pixmap_bytes_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Res::QueryClientPixmapBytes` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRes::xcb_res_id()`], then the type of the request is
/// [`xcb_res_query_client_pixmap_bytes_request_t`].
pub const XCB_RES_QUERY_CLIENT_PIXMAP_BYTES: u8 = 3i32 as u8;

/// The `Res::QueryClientPixmapBytes` request.
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

/// The `Res::QueryClientPixmapBytes` reply.
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

/// The cookie for the reply to a `Res::QueryClientIds` request.
///
/// Pass this cookie to [`xcb_res_query_client_ids_reply`] to retrieve the reply.
///
/// [`xcb_res_query_client_ids_reply`]: XcbRes::xcb_res_query_client_ids_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_client_ids_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_res_query_client_ids_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Res::QueryClientIds` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRes::xcb_res_id()`], then the type of the request is
/// [`xcb_res_query_client_ids_request_t`].
pub const XCB_RES_QUERY_CLIENT_IDS: u8 = 4i32 as u8;

/// The `Res::QueryClientIds` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `specs`
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

/// The `Res::QueryClientIds` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `ids`
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

/// The cookie for the reply to a `Res::QueryResourceBytes` request.
///
/// Pass this cookie to [`xcb_res_query_resource_bytes_reply`] to retrieve the reply.
///
/// [`xcb_res_query_resource_bytes_reply`]: XcbRes::xcb_res_query_resource_bytes_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_res_query_resource_bytes_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_res_query_resource_bytes_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Res::QueryResourceBytes` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRes::xcb_res_id()`], then the type of the request is
/// [`xcb_res_query_resource_bytes_request_t`].
pub const XCB_RES_QUERY_RESOURCE_BYTES: u8 = 5i32 as u8;

/// The `Res::QueryResourceBytes` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `specs`
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

/// The `Res::QueryResourceBytes` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `sizes`
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
        ) -> *mut xcb_res_query_version_reply_t,
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
        ) -> *mut xcb_res_query_clients_reply_t,
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
        ) -> *mut xcb_res_query_client_resources_reply_t,
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
        ) -> *mut xcb_res_query_client_pixmap_bytes_reply_t,
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
        ) -> *mut xcb_res_query_client_ids_reply_t,
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
        ) -> *mut xcb_res_query_resource_bytes_reply_t,
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
    /// The libxcb identifier of the `Res` extension.
    #[inline]
    pub fn xcb_res_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_res_id) }
    }

    /// Returns `true` iff the symbol `xcb_res_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_id(&self) -> bool {
        has_sym!(self, xcb_res_id)
    }

    /// Advances a `xcb_res_client_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_res_client_next(&self, i: *mut xcb_res_client_iterator_t) {
        sym!(self, xcb_res_client_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_res_client_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_client_next(&self) -> bool {
        has_sym!(self, xcb_res_client_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_res_client_iterator_t`.
    #[inline]
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

    /// Advances a `xcb_res_type_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_res_type_next(&self, i: *mut xcb_res_type_iterator_t) {
        sym!(self, xcb_res_type_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_res_type_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_type_next(&self) -> bool {
        has_sym!(self, xcb_res_type_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_res_type_iterator_t`.
    #[inline]
    pub unsafe fn xcb_res_type_end(&self, i: xcb_res_type_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_res_type_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_res_type_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_type_end(&self) -> bool {
        has_sym!(self, xcb_res_type_end)
    }

    /// Advances a `xcb_res_client_id_spec_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_res_client_id_spec_next(&self, i: *mut xcb_res_client_id_spec_iterator_t) {
        sym!(self, xcb_res_client_id_spec_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_res_client_id_spec_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_client_id_spec_next(&self) -> bool {
        has_sym!(self, xcb_res_client_id_spec_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_res_client_id_spec_iterator_t`.
    #[inline]
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

    /// Computes the size of a `xcb_res_client_id_value_t` object.
    #[inline]
    pub unsafe fn xcb_res_client_id_value_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_res_client_id_value_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_res_client_id_value_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_client_id_value_sizeof(&self) -> bool {
        has_sym!(self, xcb_res_client_id_value_sizeof)
    }

    /// Returns a pointer to the `value` field of a `xcb_res_client_id_value_t` struct.
    #[inline]
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

    /// Returns the number of elements of the `value` field of a `xcb_res_client_id_value_t` struct.
    #[inline]
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

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `value` field of a `xcb_res_client_id_value_t` struct.
    #[inline]
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

    /// Advances a `xcb_res_client_id_value_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_res_client_id_value_next(&self, i: *mut xcb_res_client_id_value_iterator_t) {
        sym!(self, xcb_res_client_id_value_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_res_client_id_value_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_client_id_value_next(&self) -> bool {
        has_sym!(self, xcb_res_client_id_value_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_res_client_id_value_iterator_t`.
    #[inline]
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

    /// Advances a `xcb_res_resource_id_spec_iterator_t` iterator by 1 element.
    #[inline]
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

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_res_resource_id_spec_iterator_t`.
    #[inline]
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

    /// Advances a `xcb_res_resource_size_spec_iterator_t` iterator by 1 element.
    #[inline]
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

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_res_resource_size_spec_iterator_t`.
    #[inline]
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

    /// Computes the size of a `xcb_res_resource_size_value_t` object.
    #[inline]
    pub unsafe fn xcb_res_resource_size_value_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_res_resource_size_value_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_res_resource_size_value_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_resource_size_value_sizeof(&self) -> bool {
        has_sym!(self, xcb_res_resource_size_value_sizeof)
    }

    /// Returns a pointer to the `cross_references` field of a `xcb_res_resource_size_value_t` struct.
    #[inline]
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

    /// Returns the number of elements of the `cross_references` field of a `xcb_res_resource_size_value_t` struct.
    #[inline]
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

    /// Returns an iterator over the elements of the
    /// `cross_references` field of a `xcb_res_resource_size_value_t` struct.
    #[inline]
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

    /// Advances a `xcb_res_resource_size_value_iterator_t` iterator by 1 element.
    #[inline]
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

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_res_resource_size_value_iterator_t`.
    #[inline]
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

    /// Sends a `Res::QueryVersion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_res_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_res_query_version_reply`]: Self::xcb_res_query_version_reply
    #[inline]
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

    /// Sends a `Res::QueryVersion` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_res_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_res_query_version_reply`]: Self::xcb_res_query_version_reply
    #[inline]
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

    /// Waits for the reply to a `Res::QueryVersion` request.
    #[inline]
    pub unsafe fn xcb_res_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_res_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_res_query_version_reply_t {
        sym!(self, xcb_res_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_res_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_res_query_version_reply)
    }

    /// Computes the size of a `xcb_res_query_clients_reply_t` object.
    #[inline]
    pub unsafe fn xcb_res_query_clients_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_res_query_clients_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_res_query_clients_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_clients_sizeof(&self) -> bool {
        has_sym!(self, xcb_res_query_clients_sizeof)
    }

    /// Sends a `Res::QueryClients` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_res_query_clients_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_res_query_clients_reply`]: Self::xcb_res_query_clients_reply
    #[inline]
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

    /// Sends a `Res::QueryClients` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_res_query_clients_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_res_query_clients_reply`]: Self::xcb_res_query_clients_reply
    #[inline]
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

    /// Returns a pointer to the `clients` field of a `xcb_res_query_clients_reply_t` struct.
    #[inline]
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

    /// Returns the number of elements of the `clients` field of a `xcb_res_query_clients_reply_t` struct.
    #[inline]
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

    /// Returns an iterator over the elements of the
    /// `clients` field of a `xcb_res_query_clients_reply_t` struct.
    #[inline]
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

    /// Waits for the reply to a `Res::QueryClients` request.
    #[inline]
    pub unsafe fn xcb_res_query_clients_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_res_query_clients_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_res_query_clients_reply_t {
        sym!(self, xcb_res_query_clients_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_res_query_clients_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_clients_reply(&self) -> bool {
        has_sym!(self, xcb_res_query_clients_reply)
    }

    /// Computes the size of a `xcb_res_query_client_resources_reply_t` object.
    #[inline]
    pub unsafe fn xcb_res_query_client_resources_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_res_query_client_resources_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_res_query_client_resources_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_client_resources_sizeof(&self) -> bool {
        has_sym!(self, xcb_res_query_client_resources_sizeof)
    }

    /// Sends a `Res::QueryClientResources` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_res_query_client_resources_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_res_query_client_resources_reply`]: Self::xcb_res_query_client_resources_reply
    #[inline]
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

    /// Sends a `Res::QueryClientResources` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_res_query_client_resources_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_res_query_client_resources_reply`]: Self::xcb_res_query_client_resources_reply
    #[inline]
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

    /// Returns a pointer to the `types` field of a `xcb_res_query_client_resources_reply_t` struct.
    #[inline]
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

    /// Returns the number of elements of the `types` field of a `xcb_res_query_client_resources_reply_t` struct.
    #[inline]
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

    /// Returns an iterator over the elements of the
    /// `types` field of a `xcb_res_query_client_resources_reply_t` struct.
    #[inline]
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

    /// Waits for the reply to a `Res::QueryClientResources` request.
    #[inline]
    pub unsafe fn xcb_res_query_client_resources_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_res_query_client_resources_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_res_query_client_resources_reply_t {
        sym!(self, xcb_res_query_client_resources_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_res_query_client_resources_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_client_resources_reply(&self) -> bool {
        has_sym!(self, xcb_res_query_client_resources_reply)
    }

    /// Sends a `Res::QueryClientPixmapBytes` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_res_query_client_pixmap_bytes_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_res_query_client_pixmap_bytes_reply`]: Self::xcb_res_query_client_pixmap_bytes_reply
    #[inline]
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

    /// Sends a `Res::QueryClientPixmapBytes` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_res_query_client_pixmap_bytes_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_res_query_client_pixmap_bytes_reply`]: Self::xcb_res_query_client_pixmap_bytes_reply
    #[inline]
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

    /// Waits for the reply to a `Res::QueryClientPixmapBytes` request.
    #[inline]
    pub unsafe fn xcb_res_query_client_pixmap_bytes_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_res_query_client_pixmap_bytes_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_res_query_client_pixmap_bytes_reply_t {
        sym!(self, xcb_res_query_client_pixmap_bytes_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_res_query_client_pixmap_bytes_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_client_pixmap_bytes_reply(&self) -> bool {
        has_sym!(self, xcb_res_query_client_pixmap_bytes_reply)
    }

    /// Computes the size of a `xcb_res_query_client_ids_request_t` object.
    #[inline]
    pub unsafe fn xcb_res_query_client_ids_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_res_query_client_ids_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_res_query_client_ids_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_client_ids_sizeof(&self) -> bool {
        has_sym!(self, xcb_res_query_client_ids_sizeof)
    }

    /// Sends a `Res::QueryClientIds` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_res_query_client_ids_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_res_query_client_ids_reply`]: Self::xcb_res_query_client_ids_reply
    #[inline]
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

    /// Sends a `Res::QueryClientIds` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_res_query_client_ids_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_res_query_client_ids_reply`]: Self::xcb_res_query_client_ids_reply
    #[inline]
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

    /// Returns the number of elements of the `ids` field of a `xcb_res_query_client_ids_reply_t` struct.
    #[inline]
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

    /// Returns an iterator over the elements of the
    /// `ids` field of a `xcb_res_query_client_ids_reply_t` struct.
    #[inline]
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

    /// Waits for the reply to a `Res::QueryClientIds` request.
    #[inline]
    pub unsafe fn xcb_res_query_client_ids_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_res_query_client_ids_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_res_query_client_ids_reply_t {
        sym!(self, xcb_res_query_client_ids_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_res_query_client_ids_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_client_ids_reply(&self) -> bool {
        has_sym!(self, xcb_res_query_client_ids_reply)
    }

    /// Computes the size of a `xcb_res_query_resource_bytes_request_t` object.
    #[inline]
    pub unsafe fn xcb_res_query_resource_bytes_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_res_query_resource_bytes_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_res_query_resource_bytes_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_res_query_resource_bytes_sizeof(&self) -> bool {
        has_sym!(self, xcb_res_query_resource_bytes_sizeof)
    }

    /// Sends a `Res::QueryResourceBytes` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_res_query_resource_bytes_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_res_query_resource_bytes_reply`]: Self::xcb_res_query_resource_bytes_reply
    #[inline]
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

    /// Sends a `Res::QueryResourceBytes` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_res_query_resource_bytes_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_res_query_resource_bytes_reply`]: Self::xcb_res_query_resource_bytes_reply
    #[inline]
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

    /// Returns the number of elements of the `sizes` field of a `xcb_res_query_resource_bytes_reply_t` struct.
    #[inline]
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

    /// Returns an iterator over the elements of the
    /// `sizes` field of a `xcb_res_query_resource_bytes_reply_t` struct.
    #[inline]
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

    /// Waits for the reply to a `Res::QueryResourceBytes` request.
    #[inline]
    pub unsafe fn xcb_res_query_resource_bytes_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_res_query_resource_bytes_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_res_query_resource_bytes_reply_t {
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
