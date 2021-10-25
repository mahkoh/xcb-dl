// This file was generated using generate.py. Do not edit.

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

pub type xcb_xvmc_context_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_context_iterator_t {
    pub data: *mut xcb_xvmc_context_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xvmc_context_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xvmc_surface_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_surface_iterator_t {
    pub data: *mut xcb_xvmc_surface_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xvmc_surface_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xvmc_subpicture_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_subpicture_iterator_t {
    pub data: *mut xcb_xvmc_subpicture_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xvmc_subpicture_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_surface_info_iterator_t {
    pub data: *mut xcb_xvmc_surface_info_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xvmc_surface_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_query_version_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xvmc_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xvmc_query_version.
pub const XCB_XVMC_QUERY_VERSION: u8 = 0i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_list_surface_types_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xvmc_list_surface_types_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xvmc_list_surface_types.
pub const XCB_XVMC_LIST_SURFACE_TYPES: u8 = 1i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_create_context_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xvmc_create_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xvmc_create_context.
pub const XCB_XVMC_CREATE_CONTEXT: u8 = 2i32 as u8;

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

/// Opcode for xcb_xvmc_destroy_context.
pub const XCB_XVMC_DESTROY_CONTEXT: u8 = 3i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_create_surface_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xvmc_create_surface_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xvmc_create_surface.
pub const XCB_XVMC_CREATE_SURFACE: u8 = 4i32 as u8;

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

/// Opcode for xcb_xvmc_destroy_surface.
pub const XCB_XVMC_DESTROY_SURFACE: u8 = 5i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_create_subpicture_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xvmc_create_subpicture_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xvmc_create_subpicture.
pub const XCB_XVMC_CREATE_SUBPICTURE: u8 = 6i32 as u8;

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

/// Opcode for xcb_xvmc_destroy_subpicture.
pub const XCB_XVMC_DESTROY_SUBPICTURE: u8 = 7i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xvmc_list_subpicture_types_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xvmc_list_subpicture_types_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xvmc_list_subpicture_types.
pub const XCB_XVMC_LIST_SUBPICTURE_TYPES: u8 = 8i32 as u8;

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
        ) -> xcb_xvmc_query_version_reply_t,
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
        ) -> xcb_xvmc_list_surface_types_reply_t,
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
        ) -> xcb_xvmc_create_context_reply_t,
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
        ) -> xcb_xvmc_create_surface_reply_t,
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
        ) -> xcb_xvmc_create_subpicture_reply_t,
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
        ) -> xcb_xvmc_list_subpicture_types_reply_t,
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
    pub fn xcb_xvmc_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_xvmc_id) }
    }

    /// Returns `true` iff the symbol `xcb_xvmc_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_id(&self) -> bool {
        has_sym!(self, xcb_xvmc_id)
    }

    pub unsafe fn xcb_xvmc_context_next(&self, i: *mut xcb_xvmc_context_iterator_t) {
        sym!(self, xcb_xvmc_context_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_context_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_context_next(&self) -> bool {
        has_sym!(self, xcb_xvmc_context_next)
    }

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

    pub unsafe fn xcb_xvmc_surface_next(&self, i: *mut xcb_xvmc_surface_iterator_t) {
        sym!(self, xcb_xvmc_surface_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_surface_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_surface_next(&self) -> bool {
        has_sym!(self, xcb_xvmc_surface_next)
    }

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

    pub unsafe fn xcb_xvmc_subpicture_next(&self, i: *mut xcb_xvmc_subpicture_iterator_t) {
        sym!(self, xcb_xvmc_subpicture_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_subpicture_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_subpicture_next(&self) -> bool {
        has_sym!(self, xcb_xvmc_subpicture_next)
    }

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

    pub unsafe fn xcb_xvmc_surface_info_next(&self, i: *mut xcb_xvmc_surface_info_iterator_t) {
        sym!(self, xcb_xvmc_surface_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_surface_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_surface_info_next(&self) -> bool {
        has_sym!(self, xcb_xvmc_surface_info_next)
    }

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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_xvmc_query_version_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xvmc_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xvmc_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xvmc_query_version_reply_t {
        sym!(self, xcb_xvmc_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_xvmc_query_version_reply)
    }

    pub unsafe fn xcb_xvmc_list_surface_types_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xvmc_list_surface_types_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_list_surface_types_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_list_surface_types_sizeof(&self) -> bool {
        has_sym!(self, xcb_xvmc_list_surface_types_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_xvmc_list_surface_types_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xvmc_list_surface_types_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xvmc_list_surface_types_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xvmc_list_surface_types_reply_t {
        sym!(self, xcb_xvmc_list_surface_types_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_list_surface_types_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_list_surface_types_reply(&self) -> bool {
        has_sym!(self, xcb_xvmc_list_surface_types_reply)
    }

    pub unsafe fn xcb_xvmc_create_context_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xvmc_create_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_context_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_xvmc_create_context_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xvmc_create_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xvmc_create_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xvmc_create_context_reply_t {
        sym!(self, xcb_xvmc_create_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_context_reply(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_context_reply)
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    pub unsafe fn xcb_xvmc_create_surface_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xvmc_create_surface_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_surface_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_surface_sizeof(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_surface_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_xvmc_create_surface_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xvmc_create_surface_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xvmc_create_surface_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xvmc_create_surface_reply_t {
        sym!(self, xcb_xvmc_create_surface_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_surface_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_surface_reply(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_surface_reply)
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    pub unsafe fn xcb_xvmc_create_subpicture_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xvmc_create_subpicture_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_subpicture_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_subpicture_sizeof(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_subpicture_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_xvmc_create_subpicture_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xvmc_create_subpicture_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xvmc_create_subpicture_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xvmc_create_subpicture_reply_t {
        sym!(self, xcb_xvmc_create_subpicture_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_create_subpicture_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_create_subpicture_reply(&self) -> bool {
        has_sym!(self, xcb_xvmc_create_subpicture_reply)
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    pub unsafe fn xcb_xvmc_list_subpicture_types_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xvmc_list_subpicture_types_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xvmc_list_subpicture_types_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xvmc_list_subpicture_types_sizeof(&self) -> bool {
        has_sym!(self, xcb_xvmc_list_subpicture_types_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_xvmc_list_subpicture_types_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xvmc_list_subpicture_types_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xvmc_list_subpicture_types_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xvmc_list_subpicture_types_reply_t {
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
