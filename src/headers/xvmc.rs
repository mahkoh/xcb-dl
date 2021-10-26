// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The `XvMC::CONTEXT` type.
pub type xcb_xvmc_context_t = u32;

/// An iterator over `XvMC::CONTEXT` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_context_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xvmc_context_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xvmc_context_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XvMC::SURFACE` type.
pub type xcb_xvmc_surface_t = u32;

/// An iterator over `XvMC::SURFACE` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_surface_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xvmc_surface_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xvmc_surface_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XvMC::SUBPICTURE` type.
pub type xcb_xvmc_subpicture_t = u32;

/// An iterator over `XvMC::SUBPICTURE` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_subpicture_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xvmc_subpicture_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xvmc_subpicture_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XvMC::SurfaceInfo` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_surface_info_t {
    pub id: xcb_xvmc_surface_t,
    pub chroma_format: u16,
    pub pad0: u16,
    pub max_width: u16,
    pub max_height: u16,
    pub subpicture_max_width: u16,
    pub subpicture_max_height: u16,
    pub mc_type: u32,
    pub flags: u32,
}

impl Default for xcb_xvmc_surface_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `XvMC::SurfaceInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_surface_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xvmc_surface_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xvmc_surface_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `XvMC::QueryVersion` request.
///
/// Pass this cookie to [`xcb_xvmc_query_version_reply`] to retrieve the reply.
///
/// [`xcb_xvmc_query_version_reply`]: XcbXvmc::xcb_xvmc_query_version_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_query_version_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xvmc_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XvMC::QueryVersion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXvmc::xcb_xvmc_id()`], then the type of the request is
/// [`xcb_xvmc_query_version_request_t`].
pub const XCB_XVMC_QUERY_VERSION: u8 = 0i32 as u8;

/// The `XvMC::QueryVersion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_xvmc_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XvMC::QueryVersion` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major: u32,
    pub minor: u32,
}

impl Default for xcb_xvmc_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `XvMC::ListSurfaceTypes` request.
///
/// Pass this cookie to [`xcb_xvmc_list_surface_types_reply`] to retrieve the reply.
///
/// [`xcb_xvmc_list_surface_types_reply`]: XcbXvmc::xcb_xvmc_list_surface_types_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_list_surface_types_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xvmc_list_surface_types_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XvMC::ListSurfaceTypes` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXvmc::xcb_xvmc_id()`], then the type of the request is
/// [`xcb_xvmc_list_surface_types_request_t`].
pub const XCB_XVMC_LIST_SURFACE_TYPES: u8 = 1i32 as u8;

/// The `XvMC::ListSurfaceTypes` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_list_surface_types_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port_id: xcb_xv_port_t,
}

impl Default for xcb_xvmc_list_surface_types_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XvMC::ListSurfaceTypes` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `surfaces`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_list_surface_types_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_xvmc_list_surface_types_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `XvMC::CreateContext` request.
///
/// Pass this cookie to [`xcb_xvmc_create_context_reply`] to retrieve the reply.
///
/// [`xcb_xvmc_create_context_reply`]: XcbXvmc::xcb_xvmc_create_context_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_create_context_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xvmc_create_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XvMC::CreateContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXvmc::xcb_xvmc_id()`], then the type of the request is
/// [`xcb_xvmc_create_context_request_t`].
pub const XCB_XVMC_CREATE_CONTEXT: u8 = 2i32 as u8;

/// The `XvMC::CreateContext` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_create_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_id: xcb_xvmc_context_t,
    pub port_id: xcb_xv_port_t,
    pub surface_id: xcb_xvmc_surface_t,
    pub width: u16,
    pub height: u16,
    pub flags: u32,
}

impl Default for xcb_xvmc_create_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XvMC::CreateContext` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `priv_data`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_create_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub width_actual: u16,
    pub height_actual: u16,
    pub flags_return: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_xvmc_create_context_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XvMC::DestroyContext` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXvmc::xcb_xvmc_id()`], then the type of the request is
/// [`xcb_xvmc_destroy_context_request_t`].
pub const XCB_XVMC_DESTROY_CONTEXT: u8 = 3i32 as u8;

/// The `XvMC::DestroyContext` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_destroy_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_id: xcb_xvmc_context_t,
}

impl Default for xcb_xvmc_destroy_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `XvMC::CreateSurface` request.
///
/// Pass this cookie to [`xcb_xvmc_create_surface_reply`] to retrieve the reply.
///
/// [`xcb_xvmc_create_surface_reply`]: XcbXvmc::xcb_xvmc_create_surface_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_create_surface_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xvmc_create_surface_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XvMC::CreateSurface` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXvmc::xcb_xvmc_id()`], then the type of the request is
/// [`xcb_xvmc_create_surface_request_t`].
pub const XCB_XVMC_CREATE_SURFACE: u8 = 4i32 as u8;

/// The `XvMC::CreateSurface` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_create_surface_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub surface_id: xcb_xvmc_surface_t,
    pub context_id: xcb_xvmc_context_t,
}

impl Default for xcb_xvmc_create_surface_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XvMC::CreateSurface` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `priv_data`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_create_surface_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 24],
}

impl Default for xcb_xvmc_create_surface_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XvMC::DestroySurface` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXvmc::xcb_xvmc_id()`], then the type of the request is
/// [`xcb_xvmc_destroy_surface_request_t`].
pub const XCB_XVMC_DESTROY_SURFACE: u8 = 5i32 as u8;

/// The `XvMC::DestroySurface` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_destroy_surface_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub surface_id: xcb_xvmc_surface_t,
}

impl Default for xcb_xvmc_destroy_surface_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `XvMC::CreateSubpicture` request.
///
/// Pass this cookie to [`xcb_xvmc_create_subpicture_reply`] to retrieve the reply.
///
/// [`xcb_xvmc_create_subpicture_reply`]: XcbXvmc::xcb_xvmc_create_subpicture_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_create_subpicture_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xvmc_create_subpicture_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XvMC::CreateSubpicture` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXvmc::xcb_xvmc_id()`], then the type of the request is
/// [`xcb_xvmc_create_subpicture_request_t`].
pub const XCB_XVMC_CREATE_SUBPICTURE: u8 = 6i32 as u8;

/// The `XvMC::CreateSubpicture` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_create_subpicture_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub subpicture_id: xcb_xvmc_subpicture_t,
    pub context: xcb_xvmc_context_t,
    pub xvimage_id: u32,
    pub width: u16,
    pub height: u16,
}

impl Default for xcb_xvmc_create_subpicture_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XvMC::CreateSubpicture` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `priv_data`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_create_subpicture_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub width_actual: u16,
    pub height_actual: u16,
    pub num_palette_entries: u16,
    pub entry_bytes: u16,
    pub component_order: [u8; 4],
    pub pad1: [u8; 12],
}

impl Default for xcb_xvmc_create_subpicture_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XvMC::DestroySubpicture` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXvmc::xcb_xvmc_id()`], then the type of the request is
/// [`xcb_xvmc_destroy_subpicture_request_t`].
pub const XCB_XVMC_DESTROY_SUBPICTURE: u8 = 7i32 as u8;

/// The `XvMC::DestroySubpicture` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_destroy_subpicture_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub subpicture_id: xcb_xvmc_subpicture_t,
}

impl Default for xcb_xvmc_destroy_subpicture_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `XvMC::ListSubpictureTypes` request.
///
/// Pass this cookie to [`xcb_xvmc_list_subpicture_types_reply`] to retrieve the reply.
///
/// [`xcb_xvmc_list_subpicture_types_reply`]: XcbXvmc::xcb_xvmc_list_subpicture_types_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_list_subpicture_types_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xvmc_list_subpicture_types_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XvMC::ListSubpictureTypes` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXvmc::xcb_xvmc_id()`], then the type of the request is
/// [`xcb_xvmc_list_subpicture_types_request_t`].
pub const XCB_XVMC_LIST_SUBPICTURE_TYPES: u8 = 8i32 as u8;

/// The `XvMC::ListSubpictureTypes` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_list_subpicture_types_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port_id: xcb_xv_port_t,
    pub surface_id: xcb_xvmc_surface_t,
}

impl Default for xcb_xvmc_list_subpicture_types_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XvMC::ListSubpictureTypes` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `types`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_list_subpicture_types_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_xvmc_list_subpicture_types_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_xvmc")]
pub(crate) struct XcbXvmcXvmc {
    xcb_xvmc_id: LazySymbol<*mut xcb_extension_t>,
    xcb_xvmc_context_next: LazySymbol<unsafe fn(i: *mut xcb_xvmc_context_iterator_t)>,
    xcb_xvmc_context_end:
        LazySymbol<unsafe fn(i: xcb_xvmc_context_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xvmc_surface_next: LazySymbol<unsafe fn(i: *mut xcb_xvmc_surface_iterator_t)>,
    xcb_xvmc_surface_end:
        LazySymbol<unsafe fn(i: xcb_xvmc_surface_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xvmc_subpicture_next: LazySymbol<unsafe fn(i: *mut xcb_xvmc_subpicture_iterator_t)>,
    xcb_xvmc_subpicture_end:
        LazySymbol<unsafe fn(i: xcb_xvmc_subpicture_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xvmc_surface_info_next: LazySymbol<unsafe fn(i: *mut xcb_xvmc_surface_info_iterator_t)>,
    xcb_xvmc_surface_info_end:
        LazySymbol<unsafe fn(i: xcb_xvmc_surface_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xvmc_query_version:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xvmc_query_version_cookie_t>,
    xcb_xvmc_query_version_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xvmc_query_version_cookie_t>,
    xcb_xvmc_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xvmc_query_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xvmc_query_version_reply_t,
    >,
    xcb_xvmc_list_surface_types_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xvmc_list_surface_types: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port_id: xcb_xv_port_t,
        ) -> xcb_xvmc_list_surface_types_cookie_t,
    >,
    xcb_xvmc_list_surface_types_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port_id: xcb_xv_port_t,
        ) -> xcb_xvmc_list_surface_types_cookie_t,
    >,
    xcb_xvmc_list_surface_types_surfaces: LazySymbol<
        unsafe fn(r: *const xcb_xvmc_list_surface_types_reply_t) -> *mut xcb_xvmc_surface_info_t,
    >,
    xcb_xvmc_list_surface_types_surfaces_length:
        LazySymbol<unsafe fn(r: *const xcb_xvmc_list_surface_types_reply_t) -> c_int>,
    xcb_xvmc_list_surface_types_surfaces_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xvmc_list_surface_types_reply_t,
        ) -> xcb_xvmc_surface_info_iterator_t,
    >,
    xcb_xvmc_list_surface_types_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xvmc_list_surface_types_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xvmc_list_surface_types_reply_t,
    >,
    xcb_xvmc_create_context_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xvmc_create_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_id: xcb_xvmc_context_t,
            port_id: xcb_xv_port_t,
            surface_id: xcb_xvmc_surface_t,
            width: u16,
            height: u16,
            flags: u32,
        ) -> xcb_xvmc_create_context_cookie_t,
    >,
    xcb_xvmc_create_context_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_id: xcb_xvmc_context_t,
            port_id: xcb_xv_port_t,
            surface_id: xcb_xvmc_surface_t,
            width: u16,
            height: u16,
            flags: u32,
        ) -> xcb_xvmc_create_context_cookie_t,
    >,
    xcb_xvmc_create_context_priv_data:
        LazySymbol<unsafe fn(r: *const xcb_xvmc_create_context_reply_t) -> *mut u32>,
    xcb_xvmc_create_context_priv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_xvmc_create_context_reply_t) -> c_int>,
    xcb_xvmc_create_context_priv_data_end:
        LazySymbol<unsafe fn(r: *const xcb_xvmc_create_context_reply_t) -> xcb_generic_iterator_t>,
    xcb_xvmc_create_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xvmc_create_context_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xvmc_create_context_reply_t,
    >,
    xcb_xvmc_destroy_context_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, context_id: xcb_xvmc_context_t) -> xcb_void_cookie_t,
    >,
    xcb_xvmc_destroy_context: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, context_id: xcb_xvmc_context_t) -> xcb_void_cookie_t,
    >,
    xcb_xvmc_create_surface_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xvmc_create_surface: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            surface_id: xcb_xvmc_surface_t,
            context_id: xcb_xvmc_context_t,
        ) -> xcb_xvmc_create_surface_cookie_t,
    >,
    xcb_xvmc_create_surface_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            surface_id: xcb_xvmc_surface_t,
            context_id: xcb_xvmc_context_t,
        ) -> xcb_xvmc_create_surface_cookie_t,
    >,
    xcb_xvmc_create_surface_priv_data:
        LazySymbol<unsafe fn(r: *const xcb_xvmc_create_surface_reply_t) -> *mut u32>,
    xcb_xvmc_create_surface_priv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_xvmc_create_surface_reply_t) -> c_int>,
    xcb_xvmc_create_surface_priv_data_end:
        LazySymbol<unsafe fn(r: *const xcb_xvmc_create_surface_reply_t) -> xcb_generic_iterator_t>,
    xcb_xvmc_create_surface_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xvmc_create_surface_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xvmc_create_surface_reply_t,
    >,
    xcb_xvmc_destroy_surface_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, surface_id: xcb_xvmc_surface_t) -> xcb_void_cookie_t,
    >,
    xcb_xvmc_destroy_surface: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, surface_id: xcb_xvmc_surface_t) -> xcb_void_cookie_t,
    >,
    xcb_xvmc_create_subpicture_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xvmc_create_subpicture: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            subpicture_id: xcb_xvmc_subpicture_t,
            context: xcb_xvmc_context_t,
            xvimage_id: u32,
            width: u16,
            height: u16,
        ) -> xcb_xvmc_create_subpicture_cookie_t,
    >,
    xcb_xvmc_create_subpicture_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            subpicture_id: xcb_xvmc_subpicture_t,
            context: xcb_xvmc_context_t,
            xvimage_id: u32,
            width: u16,
            height: u16,
        ) -> xcb_xvmc_create_subpicture_cookie_t,
    >,
    xcb_xvmc_create_subpicture_priv_data:
        LazySymbol<unsafe fn(r: *const xcb_xvmc_create_subpicture_reply_t) -> *mut u32>,
    xcb_xvmc_create_subpicture_priv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_xvmc_create_subpicture_reply_t) -> c_int>,
    xcb_xvmc_create_subpicture_priv_data_end: LazySymbol<
        unsafe fn(r: *const xcb_xvmc_create_subpicture_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_xvmc_create_subpicture_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xvmc_create_subpicture_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xvmc_create_subpicture_reply_t,
    >,
    xcb_xvmc_destroy_subpicture_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            subpicture_id: xcb_xvmc_subpicture_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xvmc_destroy_subpicture: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            subpicture_id: xcb_xvmc_subpicture_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xvmc_list_subpicture_types_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xvmc_list_subpicture_types: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port_id: xcb_xv_port_t,
            surface_id: xcb_xvmc_surface_t,
        ) -> xcb_xvmc_list_subpicture_types_cookie_t,
    >,
    xcb_xvmc_list_subpicture_types_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            port_id: xcb_xv_port_t,
            surface_id: xcb_xvmc_surface_t,
        ) -> xcb_xvmc_list_subpicture_types_cookie_t,
    >,
    xcb_xvmc_list_subpicture_types_types: LazySymbol<
        unsafe fn(
            r: *const xcb_xvmc_list_subpicture_types_reply_t,
        ) -> *mut xcb_xv_image_format_info_t,
    >,
    xcb_xvmc_list_subpicture_types_types_length:
        LazySymbol<unsafe fn(r: *const xcb_xvmc_list_subpicture_types_reply_t) -> c_int>,
    xcb_xvmc_list_subpicture_types_types_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xvmc_list_subpicture_types_reply_t,
        ) -> xcb_xv_image_format_info_iterator_t,
    >,
    xcb_xvmc_list_subpicture_types_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xvmc_list_subpicture_types_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xvmc_list_subpicture_types_reply_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self.xvmc.$f.get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .xvmc
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_xvmc")]
impl XcbXvmc {
    /// The libxcb identifier of the `XvMC` extension.
    #[inline]
    pub fn xcb_xvmc_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_xvmc_id) }
    }

    /// Returns `true` iff the symbol `xcb_xvmc_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_id(&self) -> bool {
        has_sym!(self, xcb_xvmc_id)
    }

    /// Advances a `xcb_xvmc_context_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xvmc_context_next(&self, i: *mut xcb_xvmc_context_iterator_t) {
        sym!(self, xcb_xvmc_context_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_context_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_context_next(&self) -> bool {
        has_sym!(self, xcb_xvmc_context_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xvmc_context_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xvmc_context_end(
        &self,
        i: xcb_xvmc_context_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xvmc_context_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_context_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_context_end(&self) -> bool {
        has_sym!(self, xcb_xvmc_context_end)
    }

    /// Advances a `xcb_xvmc_surface_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xvmc_surface_next(&self, i: *mut xcb_xvmc_surface_iterator_t) {
        sym!(self, xcb_xvmc_surface_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_surface_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_surface_next(&self) -> bool {
        has_sym!(self, xcb_xvmc_surface_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xvmc_surface_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xvmc_surface_end(
        &self,
        i: xcb_xvmc_surface_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xvmc_surface_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_surface_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_surface_end(&self) -> bool {
        has_sym!(self, xcb_xvmc_surface_end)
    }

    /// Advances a `xcb_xvmc_subpicture_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xvmc_subpicture_next(&self, i: *mut xcb_xvmc_subpicture_iterator_t) {
        sym!(self, xcb_xvmc_subpicture_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_subpicture_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_subpicture_next(&self) -> bool {
        has_sym!(self, xcb_xvmc_subpicture_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xvmc_subpicture_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xvmc_subpicture_end(
        &self,
        i: xcb_xvmc_subpicture_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xvmc_subpicture_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_subpicture_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_subpicture_end(&self) -> bool {
        has_sym!(self, xcb_xvmc_subpicture_end)
    }

    /// Advances a `xcb_xvmc_surface_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xvmc_surface_info_next(&self, i: *mut xcb_xvmc_surface_info_iterator_t) {
        sym!(self, xcb_xvmc_surface_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_surface_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_surface_info_next(&self) -> bool {
        has_sym!(self, xcb_xvmc_surface_info_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xvmc_surface_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xvmc_surface_info_end(
        &self,
        i: xcb_xvmc_surface_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xvmc_surface_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_surface_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_surface_info_end(&self) -> bool {
        has_sym!(self, xcb_xvmc_surface_info_end)
    }

    /// Sends a `XvMC::QueryVersion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xvmc_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xvmc_query_version_reply`]: Self::xcb_xvmc_query_version_reply
    #[inline]
    pub unsafe fn xcb_xvmc_query_version(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xvmc_query_version_cookie_t {
        sym!(self, xcb_xvmc_query_version)(c)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_query_version(&self) -> bool {
        has_sym!(self, xcb_xvmc_query_version)
    }

    /// Sends a `XvMC::QueryVersion` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xvmc_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xvmc_query_version_reply`]: Self::xcb_xvmc_query_version_reply
    #[inline]
    pub unsafe fn xcb_xvmc_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xvmc_query_version_cookie_t {
        sym!(self, xcb_xvmc_query_version_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_query_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_xvmc_query_version_unchecked)
    }

    /// Waits for the reply to a `XvMC::QueryVersion` request.
    #[inline]
    pub unsafe fn xcb_xvmc_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xvmc_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xvmc_query_version_reply_t {
        sym!(self, xcb_xvmc_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_xvmc_query_version_reply)
    }

    /// Computes the size of a `xcb_xvmc_list_surface_types_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xvmc_list_surface_types_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xvmc_list_surface_types_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_list_surface_types_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_list_surface_types_sizeof(&self) -> bool {
        has_sym!(self, xcb_xvmc_list_surface_types_sizeof)
    }

    /// Sends a `XvMC::ListSurfaceTypes` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xvmc_list_surface_types_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xvmc_list_surface_types_reply`]: Self::xcb_xvmc_list_surface_types_reply
    #[inline]
    pub unsafe fn xcb_xvmc_list_surface_types(
        &self,
        c: *mut xcb_connection_t,
        port_id: xcb_xv_port_t,
    ) -> xcb_xvmc_list_surface_types_cookie_t {
        sym!(self, xcb_xvmc_list_surface_types)(c, port_id)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_list_surface_types` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_list_surface_types(&self) -> bool {
        has_sym!(self, xcb_xvmc_list_surface_types)
    }

    /// Sends a `XvMC::ListSurfaceTypes` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xvmc_list_surface_types_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xvmc_list_surface_types_reply`]: Self::xcb_xvmc_list_surface_types_reply
    #[inline]
    pub unsafe fn xcb_xvmc_list_surface_types_unchecked(
        &self,
        c: *mut xcb_connection_t,
        port_id: xcb_xv_port_t,
    ) -> xcb_xvmc_list_surface_types_cookie_t {
        sym!(self, xcb_xvmc_list_surface_types_unchecked)(c, port_id)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_list_surface_types_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_list_surface_types_unchecked(&self) -> bool {
        has_sym!(self, xcb_xvmc_list_surface_types_unchecked)
    }

    /// Returns a pointer to the `surfaces` field of a `xcb_xvmc_list_surface_types_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xvmc_list_surface_types_surfaces(
        &self,
        r: *const xcb_xvmc_list_surface_types_reply_t,
    ) -> *mut xcb_xvmc_surface_info_t {
        sym!(self, xcb_xvmc_list_surface_types_surfaces)(r)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_list_surface_types_surfaces` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_list_surface_types_surfaces(&self) -> bool {
        has_sym!(self, xcb_xvmc_list_surface_types_surfaces)
    }

    /// Returns the number of elements of the `surfaces` field of a `xcb_xvmc_list_surface_types_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xvmc_list_surface_types_surfaces_length(
        &self,
        r: *const xcb_xvmc_list_surface_types_reply_t,
    ) -> c_int {
        sym!(self, xcb_xvmc_list_surface_types_surfaces_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_list_surface_types_surfaces_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_list_surface_types_surfaces_length(&self) -> bool {
        has_sym!(self, xcb_xvmc_list_surface_types_surfaces_length)
    }

    /// Returns an iterator over the elements of the
    /// `surfaces` field of a `xcb_xvmc_list_surface_types_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xvmc_list_surface_types_surfaces_iterator(
        &self,
        r: *const xcb_xvmc_list_surface_types_reply_t,
    ) -> xcb_xvmc_surface_info_iterator_t {
        sym!(self, xcb_xvmc_list_surface_types_surfaces_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_list_surface_types_surfaces_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_list_surface_types_surfaces_iterator(&self) -> bool {
        has_sym!(self, xcb_xvmc_list_surface_types_surfaces_iterator)
    }

    /// Waits for the reply to a `XvMC::ListSurfaceTypes` request.
    #[inline]
    pub unsafe fn xcb_xvmc_list_surface_types_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xvmc_list_surface_types_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xvmc_list_surface_types_reply_t {
        sym!(self, xcb_xvmc_list_surface_types_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_list_surface_types_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_list_surface_types_reply(&self) -> bool {
        has_sym!(self, xcb_xvmc_list_surface_types_reply)
    }

    /// Computes the size of a `xcb_xvmc_create_context_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xvmc_create_context_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xvmc_create_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_context_sizeof)
    }

    /// Sends a `XvMC::CreateContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xvmc_create_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xvmc_create_context_reply`]: Self::xcb_xvmc_create_context_reply
    #[inline]
    pub unsafe fn xcb_xvmc_create_context(
        &self,
        c: *mut xcb_connection_t,
        context_id: xcb_xvmc_context_t,
        port_id: xcb_xv_port_t,
        surface_id: xcb_xvmc_surface_t,
        width: u16,
        height: u16,
        flags: u32,
    ) -> xcb_xvmc_create_context_cookie_t {
        sym!(self, xcb_xvmc_create_context)(
            c, context_id, port_id, surface_id, width, height, flags,
        )
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_context(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_context)
    }

    /// Sends a `XvMC::CreateContext` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xvmc_create_context_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xvmc_create_context_reply`]: Self::xcb_xvmc_create_context_reply
    #[inline]
    pub unsafe fn xcb_xvmc_create_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_id: xcb_xvmc_context_t,
        port_id: xcb_xv_port_t,
        surface_id: xcb_xvmc_surface_t,
        width: u16,
        height: u16,
        flags: u32,
    ) -> xcb_xvmc_create_context_cookie_t {
        sym!(self, xcb_xvmc_create_context_unchecked)(
            c, context_id, port_id, surface_id, width, height, flags,
        )
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_context_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_context_unchecked(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_context_unchecked)
    }

    /// Returns a pointer to the `priv_data` field of a `xcb_xvmc_create_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xvmc_create_context_priv_data(
        &self,
        r: *const xcb_xvmc_create_context_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_xvmc_create_context_priv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_context_priv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_context_priv_data(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_context_priv_data)
    }

    /// Returns the number of elements of the `priv_data` field of a `xcb_xvmc_create_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xvmc_create_context_priv_data_length(
        &self,
        r: *const xcb_xvmc_create_context_reply_t,
    ) -> c_int {
        sym!(self, xcb_xvmc_create_context_priv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_context_priv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_context_priv_data_length(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_context_priv_data_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `priv_data` field of a `xcb_xvmc_create_context_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xvmc_create_context_priv_data_end(
        &self,
        r: *const xcb_xvmc_create_context_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xvmc_create_context_priv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_context_priv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_context_priv_data_end(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_context_priv_data_end)
    }

    /// Waits for the reply to a `XvMC::CreateContext` request.
    #[inline]
    pub unsafe fn xcb_xvmc_create_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xvmc_create_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xvmc_create_context_reply_t {
        sym!(self, xcb_xvmc_create_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_context_reply(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_context_reply)
    }

    /// Sends a `XvMC::DestroyContext` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xvmc_destroy_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context_id: xcb_xvmc_context_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xvmc_destroy_context_checked)(c, context_id)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_destroy_context_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_destroy_context_checked(&self) -> bool {
        has_sym!(self, xcb_xvmc_destroy_context_checked)
    }

    /// Sends a `XvMC::DestroyContext` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xvmc_destroy_context(
        &self,
        c: *mut xcb_connection_t,
        context_id: xcb_xvmc_context_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xvmc_destroy_context)(c, context_id)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_destroy_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_destroy_context(&self) -> bool {
        has_sym!(self, xcb_xvmc_destroy_context)
    }

    /// Computes the size of a `xcb_xvmc_create_surface_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xvmc_create_surface_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xvmc_create_surface_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_surface_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_surface_sizeof(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_surface_sizeof)
    }

    /// Sends a `XvMC::CreateSurface` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xvmc_create_surface_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xvmc_create_surface_reply`]: Self::xcb_xvmc_create_surface_reply
    #[inline]
    pub unsafe fn xcb_xvmc_create_surface(
        &self,
        c: *mut xcb_connection_t,
        surface_id: xcb_xvmc_surface_t,
        context_id: xcb_xvmc_context_t,
    ) -> xcb_xvmc_create_surface_cookie_t {
        sym!(self, xcb_xvmc_create_surface)(c, surface_id, context_id)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_surface` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_surface(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_surface)
    }

    /// Sends a `XvMC::CreateSurface` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xvmc_create_surface_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xvmc_create_surface_reply`]: Self::xcb_xvmc_create_surface_reply
    #[inline]
    pub unsafe fn xcb_xvmc_create_surface_unchecked(
        &self,
        c: *mut xcb_connection_t,
        surface_id: xcb_xvmc_surface_t,
        context_id: xcb_xvmc_context_t,
    ) -> xcb_xvmc_create_surface_cookie_t {
        sym!(self, xcb_xvmc_create_surface_unchecked)(c, surface_id, context_id)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_surface_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_surface_unchecked(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_surface_unchecked)
    }

    /// Returns a pointer to the `priv_data` field of a `xcb_xvmc_create_surface_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xvmc_create_surface_priv_data(
        &self,
        r: *const xcb_xvmc_create_surface_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_xvmc_create_surface_priv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_surface_priv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_surface_priv_data(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_surface_priv_data)
    }

    /// Returns the number of elements of the `priv_data` field of a `xcb_xvmc_create_surface_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xvmc_create_surface_priv_data_length(
        &self,
        r: *const xcb_xvmc_create_surface_reply_t,
    ) -> c_int {
        sym!(self, xcb_xvmc_create_surface_priv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_surface_priv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_surface_priv_data_length(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_surface_priv_data_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `priv_data` field of a `xcb_xvmc_create_surface_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xvmc_create_surface_priv_data_end(
        &self,
        r: *const xcb_xvmc_create_surface_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xvmc_create_surface_priv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_surface_priv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_surface_priv_data_end(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_surface_priv_data_end)
    }

    /// Waits for the reply to a `XvMC::CreateSurface` request.
    #[inline]
    pub unsafe fn xcb_xvmc_create_surface_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xvmc_create_surface_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xvmc_create_surface_reply_t {
        sym!(self, xcb_xvmc_create_surface_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_surface_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_surface_reply(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_surface_reply)
    }

    /// Sends a `XvMC::DestroySurface` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xvmc_destroy_surface_checked(
        &self,
        c: *mut xcb_connection_t,
        surface_id: xcb_xvmc_surface_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xvmc_destroy_surface_checked)(c, surface_id)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_destroy_surface_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_destroy_surface_checked(&self) -> bool {
        has_sym!(self, xcb_xvmc_destroy_surface_checked)
    }

    /// Sends a `XvMC::DestroySurface` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xvmc_destroy_surface(
        &self,
        c: *mut xcb_connection_t,
        surface_id: xcb_xvmc_surface_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xvmc_destroy_surface)(c, surface_id)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_destroy_surface` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_destroy_surface(&self) -> bool {
        has_sym!(self, xcb_xvmc_destroy_surface)
    }

    /// Computes the size of a `xcb_xvmc_create_subpicture_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xvmc_create_subpicture_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xvmc_create_subpicture_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_subpicture_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_subpicture_sizeof(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_subpicture_sizeof)
    }

    /// Sends a `XvMC::CreateSubpicture` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xvmc_create_subpicture_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xvmc_create_subpicture_reply`]: Self::xcb_xvmc_create_subpicture_reply
    #[inline]
    pub unsafe fn xcb_xvmc_create_subpicture(
        &self,
        c: *mut xcb_connection_t,
        subpicture_id: xcb_xvmc_subpicture_t,
        context: xcb_xvmc_context_t,
        xvimage_id: u32,
        width: u16,
        height: u16,
    ) -> xcb_xvmc_create_subpicture_cookie_t {
        sym!(self, xcb_xvmc_create_subpicture)(c, subpicture_id, context, xvimage_id, width, height)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_subpicture` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_subpicture(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_subpicture)
    }

    /// Sends a `XvMC::CreateSubpicture` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xvmc_create_subpicture_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xvmc_create_subpicture_reply`]: Self::xcb_xvmc_create_subpicture_reply
    #[inline]
    pub unsafe fn xcb_xvmc_create_subpicture_unchecked(
        &self,
        c: *mut xcb_connection_t,
        subpicture_id: xcb_xvmc_subpicture_t,
        context: xcb_xvmc_context_t,
        xvimage_id: u32,
        width: u16,
        height: u16,
    ) -> xcb_xvmc_create_subpicture_cookie_t {
        sym!(self, xcb_xvmc_create_subpicture_unchecked)(
            c,
            subpicture_id,
            context,
            xvimage_id,
            width,
            height,
        )
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_subpicture_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_subpicture_unchecked(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_subpicture_unchecked)
    }

    /// Returns a pointer to the `priv_data` field of a `xcb_xvmc_create_subpicture_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xvmc_create_subpicture_priv_data(
        &self,
        r: *const xcb_xvmc_create_subpicture_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_xvmc_create_subpicture_priv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_subpicture_priv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_subpicture_priv_data(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_subpicture_priv_data)
    }

    /// Returns the number of elements of the `priv_data` field of a `xcb_xvmc_create_subpicture_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xvmc_create_subpicture_priv_data_length(
        &self,
        r: *const xcb_xvmc_create_subpicture_reply_t,
    ) -> c_int {
        sym!(self, xcb_xvmc_create_subpicture_priv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_subpicture_priv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_subpicture_priv_data_length(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_subpicture_priv_data_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `priv_data` field of a `xcb_xvmc_create_subpicture_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xvmc_create_subpicture_priv_data_end(
        &self,
        r: *const xcb_xvmc_create_subpicture_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xvmc_create_subpicture_priv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_subpicture_priv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_subpicture_priv_data_end(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_subpicture_priv_data_end)
    }

    /// Waits for the reply to a `XvMC::CreateSubpicture` request.
    #[inline]
    pub unsafe fn xcb_xvmc_create_subpicture_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xvmc_create_subpicture_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xvmc_create_subpicture_reply_t {
        sym!(self, xcb_xvmc_create_subpicture_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_subpicture_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_subpicture_reply(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_subpicture_reply)
    }

    /// Sends a `XvMC::DestroySubpicture` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xvmc_destroy_subpicture_checked(
        &self,
        c: *mut xcb_connection_t,
        subpicture_id: xcb_xvmc_subpicture_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xvmc_destroy_subpicture_checked)(c, subpicture_id)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_destroy_subpicture_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_destroy_subpicture_checked(&self) -> bool {
        has_sym!(self, xcb_xvmc_destroy_subpicture_checked)
    }

    /// Sends a `XvMC::DestroySubpicture` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xvmc_destroy_subpicture(
        &self,
        c: *mut xcb_connection_t,
        subpicture_id: xcb_xvmc_subpicture_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xvmc_destroy_subpicture)(c, subpicture_id)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_destroy_subpicture` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_destroy_subpicture(&self) -> bool {
        has_sym!(self, xcb_xvmc_destroy_subpicture)
    }

    /// Computes the size of a `xcb_xvmc_list_subpicture_types_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xvmc_list_subpicture_types_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xvmc_list_subpicture_types_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_list_subpicture_types_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_list_subpicture_types_sizeof(&self) -> bool {
        has_sym!(self, xcb_xvmc_list_subpicture_types_sizeof)
    }

    /// Sends a `XvMC::ListSubpictureTypes` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xvmc_list_subpicture_types_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xvmc_list_subpicture_types_reply`]: Self::xcb_xvmc_list_subpicture_types_reply
    #[inline]
    pub unsafe fn xcb_xvmc_list_subpicture_types(
        &self,
        c: *mut xcb_connection_t,
        port_id: xcb_xv_port_t,
        surface_id: xcb_xvmc_surface_t,
    ) -> xcb_xvmc_list_subpicture_types_cookie_t {
        sym!(self, xcb_xvmc_list_subpicture_types)(c, port_id, surface_id)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_list_subpicture_types` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_list_subpicture_types(&self) -> bool {
        has_sym!(self, xcb_xvmc_list_subpicture_types)
    }

    /// Sends a `XvMC::ListSubpictureTypes` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xvmc_list_subpicture_types_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xvmc_list_subpicture_types_reply`]: Self::xcb_xvmc_list_subpicture_types_reply
    #[inline]
    pub unsafe fn xcb_xvmc_list_subpicture_types_unchecked(
        &self,
        c: *mut xcb_connection_t,
        port_id: xcb_xv_port_t,
        surface_id: xcb_xvmc_surface_t,
    ) -> xcb_xvmc_list_subpicture_types_cookie_t {
        sym!(self, xcb_xvmc_list_subpicture_types_unchecked)(c, port_id, surface_id)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_list_subpicture_types_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_list_subpicture_types_unchecked(&self) -> bool {
        has_sym!(self, xcb_xvmc_list_subpicture_types_unchecked)
    }

    /// Returns a pointer to the `types` field of a `xcb_xvmc_list_subpicture_types_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xvmc_list_subpicture_types_types(
        &self,
        r: *const xcb_xvmc_list_subpicture_types_reply_t,
    ) -> *mut xcb_xv_image_format_info_t {
        sym!(self, xcb_xvmc_list_subpicture_types_types)(r)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_list_subpicture_types_types` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_list_subpicture_types_types(&self) -> bool {
        has_sym!(self, xcb_xvmc_list_subpicture_types_types)
    }

    /// Returns the number of elements of the `types` field of a `xcb_xvmc_list_subpicture_types_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xvmc_list_subpicture_types_types_length(
        &self,
        r: *const xcb_xvmc_list_subpicture_types_reply_t,
    ) -> c_int {
        sym!(self, xcb_xvmc_list_subpicture_types_types_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_list_subpicture_types_types_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_list_subpicture_types_types_length(&self) -> bool {
        has_sym!(self, xcb_xvmc_list_subpicture_types_types_length)
    }

    /// Returns an iterator over the elements of the
    /// `types` field of a `xcb_xvmc_list_subpicture_types_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xvmc_list_subpicture_types_types_iterator(
        &self,
        r: *const xcb_xvmc_list_subpicture_types_reply_t,
    ) -> xcb_xv_image_format_info_iterator_t {
        sym!(self, xcb_xvmc_list_subpicture_types_types_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_list_subpicture_types_types_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_list_subpicture_types_types_iterator(&self) -> bool {
        has_sym!(self, xcb_xvmc_list_subpicture_types_types_iterator)
    }

    /// Waits for the reply to a `XvMC::ListSubpictureTypes` request.
    #[inline]
    pub unsafe fn xcb_xvmc_list_subpicture_types_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xvmc_list_subpicture_types_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xvmc_list_subpicture_types_reply_t {
        sym!(self, xcb_xvmc_list_subpicture_types_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_list_subpicture_types_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_list_subpicture_types_reply(&self) -> bool {
        has_sym!(self, xcb_xvmc_list_subpicture_types_reply)
    }
}

#[cfg(feature = "xcb_xvmc")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbXvmc::load().unwrap() };
        assert!(lib.has_xcb_xvmc_id());
        assert!(lib.has_xcb_xvmc_context_next());
        assert!(lib.has_xcb_xvmc_context_end());
        assert!(lib.has_xcb_xvmc_surface_next());
        assert!(lib.has_xcb_xvmc_surface_end());
        assert!(lib.has_xcb_xvmc_subpicture_next());
        assert!(lib.has_xcb_xvmc_subpicture_end());
        assert!(lib.has_xcb_xvmc_surface_info_next());
        assert!(lib.has_xcb_xvmc_surface_info_end());
        assert!(lib.has_xcb_xvmc_query_version());
        assert!(lib.has_xcb_xvmc_query_version_unchecked());
        assert!(lib.has_xcb_xvmc_query_version_reply());
        assert!(lib.has_xcb_xvmc_list_surface_types_sizeof());
        assert!(lib.has_xcb_xvmc_list_surface_types());
        assert!(lib.has_xcb_xvmc_list_surface_types_unchecked());
        assert!(lib.has_xcb_xvmc_list_surface_types_surfaces());
        assert!(lib.has_xcb_xvmc_list_surface_types_surfaces_length());
        assert!(lib.has_xcb_xvmc_list_surface_types_surfaces_iterator());
        assert!(lib.has_xcb_xvmc_list_surface_types_reply());
        assert!(lib.has_xcb_xvmc_create_context_sizeof());
        assert!(lib.has_xcb_xvmc_create_context());
        assert!(lib.has_xcb_xvmc_create_context_unchecked());
        assert!(lib.has_xcb_xvmc_create_context_priv_data());
        assert!(lib.has_xcb_xvmc_create_context_priv_data_length());
        assert!(lib.has_xcb_xvmc_create_context_priv_data_end());
        assert!(lib.has_xcb_xvmc_create_context_reply());
        assert!(lib.has_xcb_xvmc_destroy_context_checked());
        assert!(lib.has_xcb_xvmc_destroy_context());
        assert!(lib.has_xcb_xvmc_create_surface_sizeof());
        assert!(lib.has_xcb_xvmc_create_surface());
        assert!(lib.has_xcb_xvmc_create_surface_unchecked());
        assert!(lib.has_xcb_xvmc_create_surface_priv_data());
        assert!(lib.has_xcb_xvmc_create_surface_priv_data_length());
        assert!(lib.has_xcb_xvmc_create_surface_priv_data_end());
        assert!(lib.has_xcb_xvmc_create_surface_reply());
        assert!(lib.has_xcb_xvmc_destroy_surface_checked());
        assert!(lib.has_xcb_xvmc_destroy_surface());
        assert!(lib.has_xcb_xvmc_create_subpicture_sizeof());
        assert!(lib.has_xcb_xvmc_create_subpicture());
        assert!(lib.has_xcb_xvmc_create_subpicture_unchecked());
        assert!(lib.has_xcb_xvmc_create_subpicture_priv_data());
        assert!(lib.has_xcb_xvmc_create_subpicture_priv_data_length());
        assert!(lib.has_xcb_xvmc_create_subpicture_priv_data_end());
        assert!(lib.has_xcb_xvmc_create_subpicture_reply());
        assert!(lib.has_xcb_xvmc_destroy_subpicture_checked());
        assert!(lib.has_xcb_xvmc_destroy_subpicture());
        assert!(lib.has_xcb_xvmc_list_subpicture_types_sizeof());
        assert!(lib.has_xcb_xvmc_list_subpicture_types());
        assert!(lib.has_xcb_xvmc_list_subpicture_types_unchecked());
        assert!(lib.has_xcb_xvmc_list_subpicture_types_types());
        assert!(lib.has_xcb_xvmc_list_subpicture_types_types_length());
        assert!(lib.has_xcb_xvmc_list_subpicture_types_types_iterator());
        assert!(lib.has_xcb_xvmc_list_subpicture_types_reply());
    }
}
