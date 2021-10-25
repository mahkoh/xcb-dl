// This file was generated using generate.py. Do not edit.

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_query_version_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_selinux_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_selinux_query_version.
pub const XCB_SELINUX_QUERY_VERSION: u8 = 0i32 as u8;

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

/// Opcode for xcb_selinux_set_device_create_context.
pub const XCB_SELINUX_SET_DEVICE_CREATE_CONTEXT: u8 = 1i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_device_create_context_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_device_create_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_selinux_get_device_create_context.
pub const XCB_SELINUX_GET_DEVICE_CREATE_CONTEXT: u8 = 2i32 as u8;

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

/// Opcode for xcb_selinux_set_device_context.
pub const XCB_SELINUX_SET_DEVICE_CONTEXT: u8 = 3i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_device_context_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_device_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_selinux_get_device_context.
pub const XCB_SELINUX_GET_DEVICE_CONTEXT: u8 = 4i32 as u8;

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

/// Opcode for xcb_selinux_set_window_create_context.
pub const XCB_SELINUX_SET_WINDOW_CREATE_CONTEXT: u8 = 5i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_window_create_context_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_window_create_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_selinux_get_window_create_context.
pub const XCB_SELINUX_GET_WINDOW_CREATE_CONTEXT: u8 = 6i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_window_context_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_window_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_selinux_get_window_context.
pub const XCB_SELINUX_GET_WINDOW_CONTEXT: u8 = 7i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_list_item_iterator_t {
    pub data: *mut xcb_selinux_list_item_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_selinux_list_item_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_selinux_set_property_create_context.
pub const XCB_SELINUX_SET_PROPERTY_CREATE_CONTEXT: u8 = 8i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_create_context_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_property_create_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_selinux_get_property_create_context.
pub const XCB_SELINUX_GET_PROPERTY_CREATE_CONTEXT: u8 = 9i32 as u8;

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

/// Opcode for xcb_selinux_set_property_use_context.
pub const XCB_SELINUX_SET_PROPERTY_USE_CONTEXT: u8 = 10i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_use_context_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_property_use_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_selinux_get_property_use_context.
pub const XCB_SELINUX_GET_PROPERTY_USE_CONTEXT: u8 = 11i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_context_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_property_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_selinux_get_property_context.
pub const XCB_SELINUX_GET_PROPERTY_CONTEXT: u8 = 12i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_data_context_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_property_data_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_selinux_get_property_data_context.
pub const XCB_SELINUX_GET_PROPERTY_DATA_CONTEXT: u8 = 13i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_list_properties_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_selinux_list_properties_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_selinux_list_properties.
pub const XCB_SELINUX_LIST_PROPERTIES: u8 = 14i32 as u8;

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

/// Opcode for xcb_selinux_set_selection_create_context.
pub const XCB_SELINUX_SET_SELECTION_CREATE_CONTEXT: u8 = 15i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_create_context_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_selection_create_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_selinux_get_selection_create_context.
pub const XCB_SELINUX_GET_SELECTION_CREATE_CONTEXT: u8 = 16i32 as u8;

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

/// Opcode for xcb_selinux_set_selection_use_context.
pub const XCB_SELINUX_SET_SELECTION_USE_CONTEXT: u8 = 17i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_use_context_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_selection_use_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_selinux_get_selection_use_context.
pub const XCB_SELINUX_GET_SELECTION_USE_CONTEXT: u8 = 18i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_context_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_selection_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_selinux_get_selection_context.
pub const XCB_SELINUX_GET_SELECTION_CONTEXT: u8 = 19i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_data_context_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_selection_data_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_selinux_get_selection_data_context.
pub const XCB_SELINUX_GET_SELECTION_DATA_CONTEXT: u8 = 20i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_list_selections_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_selinux_list_selections_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_selinux_list_selections.
pub const XCB_SELINUX_LIST_SELECTIONS: u8 = 21i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_client_context_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_selinux_get_client_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_selinux_get_client_context.
pub const XCB_SELINUX_GET_CLIENT_CONTEXT: u8 = 22i32 as u8;

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
        ) -> xcb_selinux_query_version_reply_t,
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
        ) -> xcb_selinux_get_device_create_context_reply_t,
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
        ) -> xcb_selinux_get_device_context_reply_t,
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
        ) -> xcb_selinux_get_window_create_context_reply_t,
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
        ) -> xcb_selinux_get_window_context_reply_t,
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
        ) -> xcb_selinux_get_property_create_context_reply_t,
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
        ) -> xcb_selinux_get_property_use_context_reply_t,
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
        ) -> xcb_selinux_get_property_context_reply_t,
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
        ) -> xcb_selinux_get_property_data_context_reply_t,
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
        ) -> xcb_selinux_list_properties_reply_t,
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
        ) -> xcb_selinux_get_selection_create_context_reply_t,
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
        ) -> xcb_selinux_get_selection_use_context_reply_t,
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
        ) -> xcb_selinux_get_selection_context_reply_t,
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
        ) -> xcb_selinux_get_selection_data_context_reply_t,
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
        ) -> xcb_selinux_list_selections_reply_t,
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
        ) -> xcb_selinux_get_client_context_reply_t,
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
    pub fn xcb_selinux_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_selinux_id) }
    }

    /// Returns `true` iff the symbol `xcb_selinux_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_id(&self) -> bool {
        has_sym!(self, xcb_selinux_id)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_selinux_query_version_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_selinux_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_selinux_query_version_reply_t {
        sym!(self, xcb_selinux_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_query_version_reply)
    }

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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_selinux_get_device_create_context_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_selinux_get_device_create_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_device_create_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_selinux_get_device_create_context_reply_t {
        sym!(self, xcb_selinux_get_device_create_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_device_create_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_device_create_context_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_get_device_create_context_reply)
    }

    pub unsafe fn xcb_selinux_set_device_context_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_selinux_set_device_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_set_device_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_set_device_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_set_device_context_sizeof)
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    pub unsafe fn xcb_selinux_get_device_context_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_selinux_get_device_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_device_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_device_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_get_device_context_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_selinux_get_device_context_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_selinux_get_device_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_device_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_selinux_get_device_context_reply_t {
        sym!(self, xcb_selinux_get_device_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_device_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_device_context_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_get_device_context_reply)
    }

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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_selinux_get_window_create_context_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_selinux_get_window_create_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_window_create_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_selinux_get_window_create_context_reply_t {
        sym!(self, xcb_selinux_get_window_create_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_window_create_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_window_create_context_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_get_window_create_context_reply)
    }

    pub unsafe fn xcb_selinux_get_window_context_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_selinux_get_window_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_window_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_window_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_get_window_context_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_selinux_get_window_context_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_selinux_get_window_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_window_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_selinux_get_window_context_reply_t {
        sym!(self, xcb_selinux_get_window_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_window_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_window_context_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_get_window_context_reply)
    }

    pub unsafe fn xcb_selinux_list_item_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_selinux_list_item_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_item_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_item_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_list_item_sizeof)
    }

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

    pub unsafe fn xcb_selinux_list_item_next(&self, i: *mut xcb_selinux_list_item_iterator_t) {
        sym!(self, xcb_selinux_list_item_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_item_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_item_next(&self) -> bool {
        has_sym!(self, xcb_selinux_list_item_next)
    }

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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_selinux_get_property_create_context_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_selinux_get_property_create_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_property_create_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_selinux_get_property_create_context_reply_t {
        sym!(self, xcb_selinux_get_property_create_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_create_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_create_context_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_create_context_reply)
    }

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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_selinux_get_property_use_context_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_selinux_get_property_use_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_property_use_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_selinux_get_property_use_context_reply_t {
        sym!(self, xcb_selinux_get_property_use_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_use_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_use_context_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_use_context_reply)
    }

    pub unsafe fn xcb_selinux_get_property_context_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_selinux_get_property_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_context_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_selinux_get_property_context_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_selinux_get_property_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_property_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_selinux_get_property_context_reply_t {
        sym!(self, xcb_selinux_get_property_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_context_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_context_reply)
    }

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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_selinux_get_property_data_context_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_selinux_get_property_data_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_property_data_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_selinux_get_property_data_context_reply_t {
        sym!(self, xcb_selinux_get_property_data_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_property_data_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_property_data_context_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_get_property_data_context_reply)
    }

    pub unsafe fn xcb_selinux_list_properties_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_selinux_list_properties_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_properties_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_properties_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_list_properties_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_selinux_list_properties_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_selinux_list_properties_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_list_properties_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_selinux_list_properties_reply_t {
        sym!(self, xcb_selinux_list_properties_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_properties_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_properties_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_list_properties_reply)
    }

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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_selinux_get_selection_create_context_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_selinux_get_selection_create_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_selection_create_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_selinux_get_selection_create_context_reply_t {
        sym!(self, xcb_selinux_get_selection_create_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_create_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_create_context_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_create_context_reply)
    }

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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_selinux_get_selection_use_context_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_selinux_get_selection_use_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_selection_use_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_selinux_get_selection_use_context_reply_t {
        sym!(self, xcb_selinux_get_selection_use_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_use_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_use_context_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_use_context_reply)
    }

    pub unsafe fn xcb_selinux_get_selection_context_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_selinux_get_selection_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_context_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_selinux_get_selection_context_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_selinux_get_selection_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_selection_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_selinux_get_selection_context_reply_t {
        sym!(self, xcb_selinux_get_selection_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_context_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_context_reply)
    }

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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_selinux_get_selection_data_context_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_selinux_get_selection_data_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_selection_data_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_selinux_get_selection_data_context_reply_t {
        sym!(self, xcb_selinux_get_selection_data_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_selection_data_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_selection_data_context_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_get_selection_data_context_reply)
    }

    pub unsafe fn xcb_selinux_list_selections_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_selinux_list_selections_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_selections_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_selections_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_list_selections_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_selinux_list_selections_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_selinux_list_selections_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_list_selections_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_selinux_list_selections_reply_t {
        sym!(self, xcb_selinux_list_selections_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_selinux_list_selections_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_list_selections_reply(&self) -> bool {
        has_sym!(self, xcb_selinux_list_selections_reply)
    }

    pub unsafe fn xcb_selinux_get_client_context_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_selinux_get_client_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_selinux_get_client_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_selinux_get_client_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_selinux_get_client_context_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_selinux_get_client_context_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_selinux_get_client_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_client_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_selinux_get_client_context_reply_t {
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
