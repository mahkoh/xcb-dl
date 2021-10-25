// This file was generated using generate.py. Do not edit.

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

pub type xcb_input_event_class_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_event_class_iterator_t {
    pub data: *mut xcb_input_event_class_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_event_class_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_key_code_t = u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_key_code_iterator_t {
    pub data: *mut xcb_input_key_code_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_key_code_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_device_id_t = u16;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_id_iterator_t {
    pub data: *mut xcb_input_device_id_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_device_id_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_fp1616_t = i32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_fp1616_iterator_t {
    pub data: *mut xcb_input_fp1616_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_fp1616_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_fp3232_t {
    pub integral: i32,
    pub frac: u32,
}

impl Default for xcb_input_fp3232_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_fp3232_iterator_t {
    pub data: *mut xcb_input_fp3232_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_fp3232_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_extension_version_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_get_extension_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_get_extension_version.
pub const XCB_INPUT_GET_EXTENSION_VERSION: u8 = 1i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_extension_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub name_len: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_get_extension_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_extension_version_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major: u16,
    pub server_minor: u16,
    pub present: u8,
    pub pad0: [u8; 19],
}

impl Default for xcb_input_get_extension_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_device_use_t = u32;
pub const XCB_INPUT_DEVICE_USE_IS_X_POINTER: xcb_input_device_use_t = 0;
pub const XCB_INPUT_DEVICE_USE_IS_X_KEYBOARD: xcb_input_device_use_t = 1;
pub const XCB_INPUT_DEVICE_USE_IS_X_EXTENSION_DEVICE: xcb_input_device_use_t = 2;
pub const XCB_INPUT_DEVICE_USE_IS_X_EXTENSION_KEYBOARD: xcb_input_device_use_t = 3;
pub const XCB_INPUT_DEVICE_USE_IS_X_EXTENSION_POINTER: xcb_input_device_use_t = 4;

pub type xcb_input_input_class_t = u32;
pub const XCB_INPUT_INPUT_CLASS_KEY: xcb_input_input_class_t = 0;
pub const XCB_INPUT_INPUT_CLASS_BUTTON: xcb_input_input_class_t = 1;
pub const XCB_INPUT_INPUT_CLASS_VALUATOR: xcb_input_input_class_t = 2;
pub const XCB_INPUT_INPUT_CLASS_FEEDBACK: xcb_input_input_class_t = 3;
pub const XCB_INPUT_INPUT_CLASS_PROXIMITY: xcb_input_input_class_t = 4;
pub const XCB_INPUT_INPUT_CLASS_FOCUS: xcb_input_input_class_t = 5;
pub const XCB_INPUT_INPUT_CLASS_OTHER: xcb_input_input_class_t = 6;

pub type xcb_input_valuator_mode_t = u32;
pub const XCB_INPUT_VALUATOR_MODE_RELATIVE: xcb_input_valuator_mode_t = 0;
pub const XCB_INPUT_VALUATOR_MODE_ABSOLUTE: xcb_input_valuator_mode_t = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_info_t {
    pub device_type: xcb_atom_t,
    pub device_id: u8,
    pub num_class_info: u8,
    pub device_use: u8,
    pub pad0: u8,
}

impl Default for xcb_input_device_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_info_iterator_t {
    pub data: *mut xcb_input_device_info_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_device_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_key_info_t {
    pub class_id: u8,
    pub len: u8,
    pub min_keycode: xcb_input_key_code_t,
    pub max_keycode: xcb_input_key_code_t,
    pub num_keys: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_key_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_key_info_iterator_t {
    pub data: *mut xcb_input_key_info_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_key_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_button_info_t {
    pub class_id: u8,
    pub len: u8,
    pub num_buttons: u16,
}

impl Default for xcb_input_button_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_button_info_iterator_t {
    pub data: *mut xcb_input_button_info_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_button_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_axis_info_t {
    pub resolution: u32,
    pub minimum: i32,
    pub maximum: i32,
}

impl Default for xcb_input_axis_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_axis_info_iterator_t {
    pub data: *mut xcb_input_axis_info_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_axis_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_valuator_info_t {
    pub class_id: u8,
    pub len: u8,
    pub axes_len: u8,
    pub mode: u8,
    pub motion_size: u32,
}

impl Default for xcb_input_valuator_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_valuator_info_iterator_t {
    pub data: *mut xcb_input_valuator_info_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_valuator_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_info_info_t__key {
    pub min_keycode: xcb_input_key_code_t,
    pub max_keycode: xcb_input_key_code_t,
    pub num_keys: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_input_info_info_t__key {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_info_info_t__button {
    pub num_buttons: u16,
}

impl Default for xcb_input_input_info_info_t__button {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_info_info_t__valuator {
    pub axes_len: u8,
    pub mode: u8,
    pub motion_size: u32,
    pub axes: *mut xcb_input_axis_info_t,
}

impl Default for xcb_input_input_info_info_t__valuator {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_info_info_t {
    pub key: xcb_input_input_info_info_t__key,
    pub button: xcb_input_input_info_info_t__button,
    pub valuator: xcb_input_input_info_info_t__valuator,
}

impl Default for xcb_input_input_info_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_info_t {
    pub class_id: u8,
    pub len: u8,
}

impl Default for xcb_input_input_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_info_iterator_t {
    pub data: *mut xcb_input_input_info_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_input_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_name_t {
    pub len: u8,
}

impl Default for xcb_input_device_name_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_name_iterator_t {
    pub data: *mut xcb_input_device_name_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_device_name_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_list_input_devices_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_list_input_devices_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_list_input_devices.
pub const XCB_INPUT_LIST_INPUT_DEVICES: u8 = 2i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_list_input_devices_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_input_list_input_devices_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_list_input_devices_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub devices_len: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_list_input_devices_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_event_type_base_t = u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_event_type_base_iterator_t {
    pub data: *mut xcb_input_event_type_base_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_event_type_base_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_class_info_t {
    pub class_id: u8,
    pub event_type_base: xcb_input_event_type_base_t,
}

impl Default for xcb_input_input_class_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_class_info_iterator_t {
    pub data: *mut xcb_input_input_class_info_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_input_class_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_open_device_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_open_device_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_open_device.
pub const XCB_INPUT_OPEN_DEVICE: u8 = 3i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_open_device_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_open_device_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_open_device_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_classes: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_open_device_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_close_device.
pub const XCB_INPUT_CLOSE_DEVICE: u8 = 4i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_close_device_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_close_device_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_mode_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_set_device_mode_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_set_device_mode.
pub const XCB_INPUT_SET_DEVICE_MODE: u8 = 5i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_mode_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub mode: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_set_device_mode_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_mode_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_set_device_mode_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_select_extension_event.
pub const XCB_INPUT_SELECT_EXTENSION_EVENT: u8 = 6i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_select_extension_event_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub num_classes: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_select_extension_event_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_selected_extension_events_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_get_selected_extension_events_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_get_selected_extension_events.
pub const XCB_INPUT_GET_SELECTED_EXTENSION_EVENTS: u8 = 7i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_selected_extension_events_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_input_get_selected_extension_events_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_selected_extension_events_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_this_classes: u16,
    pub num_all_classes: u16,
    pub pad0: [u8; 20],
}

impl Default for xcb_input_get_selected_extension_events_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_propagate_mode_t = u32;
pub const XCB_INPUT_PROPAGATE_MODE_ADD_TO_LIST: xcb_input_propagate_mode_t = 0;
pub const XCB_INPUT_PROPAGATE_MODE_DELETE_FROM_LIST: xcb_input_propagate_mode_t = 1;

/// Opcode for xcb_input_change_device_dont_propagate_list.
pub const XCB_INPUT_CHANGE_DEVICE_DONT_PROPAGATE_LIST: u8 = 8i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_device_dont_propagate_list_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub num_classes: u16,
    pub mode: u8,
    pub pad0: u8,
}

impl Default for xcb_input_change_device_dont_propagate_list_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_dont_propagate_list_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_get_device_dont_propagate_list_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_get_device_dont_propagate_list.
pub const XCB_INPUT_GET_DEVICE_DONT_PROPAGATE_LIST: u8 = 9i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_dont_propagate_list_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_input_get_device_dont_propagate_list_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_dont_propagate_list_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_classes: u16,
    pub pad0: [u8; 22],
}

impl Default for xcb_input_get_device_dont_propagate_list_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_time_coord_t {
    pub time: xcb_timestamp_t,
}

impl Default for xcb_input_device_time_coord_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_time_coord_iterator_t {
    pub data: *mut xcb_input_device_time_coord_t,
    pub rem: c_int,
    pub index: c_int,
    num_axes: u8,
}

impl Default for xcb_input_device_time_coord_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_motion_events_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_get_device_motion_events_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_get_device_motion_events.
pub const XCB_INPUT_GET_DEVICE_MOTION_EVENTS: u8 = 10i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_motion_events_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub start: xcb_timestamp_t,
    pub stop: xcb_timestamp_t,
    pub device_id: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_get_device_motion_events_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_motion_events_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_events: u32,
    pub num_axes: u8,
    pub device_mode: u8,
    pub pad0: [u8; 18],
}

impl Default for xcb_input_get_device_motion_events_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_keyboard_device_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_change_keyboard_device_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_change_keyboard_device.
pub const XCB_INPUT_CHANGE_KEYBOARD_DEVICE: u8 = 11i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_keyboard_device_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_change_keyboard_device_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_keyboard_device_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_change_keyboard_device_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_pointer_device_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_change_pointer_device_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_change_pointer_device.
pub const XCB_INPUT_CHANGE_POINTER_DEVICE: u8 = 12i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_pointer_device_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub x_axis: u8,
    pub y_axis: u8,
    pub device_id: u8,
    pub pad0: u8,
}

impl Default for xcb_input_change_pointer_device_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_pointer_device_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_change_pointer_device_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_grab_device_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_grab_device_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_grab_device.
pub const XCB_INPUT_GRAB_DEVICE: u8 = 13i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_grab_device_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub time: xcb_timestamp_t,
    pub num_classes: u16,
    pub this_device_mode: u8,
    pub other_device_mode: u8,
    pub owner_events: u8,
    pub device_id: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_grab_device_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_grab_device_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_grab_device_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_ungrab_device.
pub const XCB_INPUT_UNGRAB_DEVICE: u8 = 14i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_ungrab_device_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_ungrab_device_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_modifier_device_t = u32;
pub const XCB_INPUT_MODIFIER_DEVICE_USE_X_KEYBOARD: xcb_input_modifier_device_t = 255;

/// Opcode for xcb_input_grab_device_key.
pub const XCB_INPUT_GRAB_DEVICE_KEY: u8 = 15i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_grab_device_key_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub num_classes: u16,
    pub modifiers: u16,
    pub modifier_device: u8,
    pub grabbed_device: u8,
    pub key: u8,
    pub this_device_mode: u8,
    pub other_device_mode: u8,
    pub owner_events: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_grab_device_key_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_ungrab_device_key.
pub const XCB_INPUT_UNGRAB_DEVICE_KEY: u8 = 16i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_ungrab_device_key_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub modifiers: u16,
    pub modifier_device: u8,
    pub key: u8,
    pub grabbed_device: u8,
}

impl Default for xcb_input_ungrab_device_key_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_grab_device_button.
pub const XCB_INPUT_GRAB_DEVICE_BUTTON: u8 = 17i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_grab_device_button_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub grabbed_device: u8,
    pub modifier_device: u8,
    pub num_classes: u16,
    pub modifiers: u16,
    pub this_device_mode: u8,
    pub other_device_mode: u8,
    pub button: u8,
    pub owner_events: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_grab_device_button_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_ungrab_device_button.
pub const XCB_INPUT_UNGRAB_DEVICE_BUTTON: u8 = 18i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_ungrab_device_button_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub modifiers: u16,
    pub modifier_device: u8,
    pub button: u8,
    pub grabbed_device: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_ungrab_device_button_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_device_input_mode_t = u32;
pub const XCB_INPUT_DEVICE_INPUT_MODE_ASYNC_THIS_DEVICE: xcb_input_device_input_mode_t = 0;
pub const XCB_INPUT_DEVICE_INPUT_MODE_SYNC_THIS_DEVICE: xcb_input_device_input_mode_t = 1;
pub const XCB_INPUT_DEVICE_INPUT_MODE_REPLAY_THIS_DEVICE: xcb_input_device_input_mode_t = 2;
pub const XCB_INPUT_DEVICE_INPUT_MODE_ASYNC_OTHER_DEVICES: xcb_input_device_input_mode_t = 3;
pub const XCB_INPUT_DEVICE_INPUT_MODE_ASYNC_ALL: xcb_input_device_input_mode_t = 4;
pub const XCB_INPUT_DEVICE_INPUT_MODE_SYNC_ALL: xcb_input_device_input_mode_t = 5;

/// Opcode for xcb_input_allow_device_events.
pub const XCB_INPUT_ALLOW_DEVICE_EVENTS: u8 = 19i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_allow_device_events_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub time: xcb_timestamp_t,
    pub mode: u8,
    pub device_id: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_allow_device_events_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_focus_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_get_device_focus_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_get_device_focus.
pub const XCB_INPUT_GET_DEVICE_FOCUS: u8 = 20i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_focus_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_get_device_focus_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_focus_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub focus: xcb_window_t,
    pub time: xcb_timestamp_t,
    pub revert_to: u8,
    pub pad0: [u8; 15],
}

impl Default for xcb_input_get_device_focus_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_set_device_focus.
pub const XCB_INPUT_SET_DEVICE_FOCUS: u8 = 21i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_focus_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub focus: xcb_window_t,
    pub time: xcb_timestamp_t,
    pub revert_to: u8,
    pub device_id: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_set_device_focus_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_feedback_class_t = u32;
pub const XCB_INPUT_FEEDBACK_CLASS_KEYBOARD: xcb_input_feedback_class_t = 0;
pub const XCB_INPUT_FEEDBACK_CLASS_POINTER: xcb_input_feedback_class_t = 1;
pub const XCB_INPUT_FEEDBACK_CLASS_STRING: xcb_input_feedback_class_t = 2;
pub const XCB_INPUT_FEEDBACK_CLASS_INTEGER: xcb_input_feedback_class_t = 3;
pub const XCB_INPUT_FEEDBACK_CLASS_LED: xcb_input_feedback_class_t = 4;
pub const XCB_INPUT_FEEDBACK_CLASS_BELL: xcb_input_feedback_class_t = 5;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_kbd_feedback_state_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
    pub pitch: u16,
    pub duration: u16,
    pub led_mask: u32,
    pub led_values: u32,
    pub global_auto_repeat: u8,
    pub click: u8,
    pub percent: u8,
    pub pad0: u8,
    pub auto_repeats: [u8; 32],
}

impl Default for xcb_input_kbd_feedback_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_kbd_feedback_state_iterator_t {
    pub data: *mut xcb_input_kbd_feedback_state_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_kbd_feedback_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_ptr_feedback_state_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
    pub pad0: [u8; 2],
    pub accel_num: u16,
    pub accel_denom: u16,
    pub threshold: u16,
}

impl Default for xcb_input_ptr_feedback_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_ptr_feedback_state_iterator_t {
    pub data: *mut xcb_input_ptr_feedback_state_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_ptr_feedback_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_integer_feedback_state_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
    pub resolution: u32,
    pub min_value: i32,
    pub max_value: i32,
}

impl Default for xcb_input_integer_feedback_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_integer_feedback_state_iterator_t {
    pub data: *mut xcb_input_integer_feedback_state_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_integer_feedback_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_string_feedback_state_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
    pub max_symbols: u16,
    pub num_keysyms: u16,
}

impl Default for xcb_input_string_feedback_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_string_feedback_state_iterator_t {
    pub data: *mut xcb_input_string_feedback_state_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_string_feedback_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_bell_feedback_state_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
    pub percent: u8,
    pub pad0: [u8; 3],
    pub pitch: u16,
    pub duration: u16,
}

impl Default for xcb_input_bell_feedback_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_bell_feedback_state_iterator_t {
    pub data: *mut xcb_input_bell_feedback_state_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_bell_feedback_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_led_feedback_state_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
    pub led_mask: u32,
    pub led_values: u32,
}

impl Default for xcb_input_led_feedback_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_led_feedback_state_iterator_t {
    pub data: *mut xcb_input_led_feedback_state_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_led_feedback_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_state_data_t__keyboard {
    pub pitch: u16,
    pub duration: u16,
    pub led_mask: u32,
    pub led_values: u32,
    pub global_auto_repeat: u8,
    pub click: u8,
    pub percent: u8,
    pub pad0: u8,
    pub auto_repeats: [u8; 32],
}

impl Default for xcb_input_feedback_state_data_t__keyboard {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_state_data_t__pointer {
    pub pad1: [u8; 2],
    pub accel_num: u16,
    pub accel_denom: u16,
    pub threshold: u16,
}

impl Default for xcb_input_feedback_state_data_t__pointer {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_state_data_t__string {
    pub max_symbols: u16,
    pub num_keysyms: u16,
    pub keysyms: *mut xcb_keysym_t,
}

impl Default for xcb_input_feedback_state_data_t__string {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_state_data_t__integer {
    pub resolution: u32,
    pub min_value: i32,
    pub max_value: i32,
}

impl Default for xcb_input_feedback_state_data_t__integer {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_state_data_t__led {
    pub led_mask: u32,
    pub led_values: u32,
}

impl Default for xcb_input_feedback_state_data_t__led {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_state_data_t__bell {
    pub percent: u8,
    pub pad2: [u8; 3],
    pub pitch: u16,
    pub duration: u16,
}

impl Default for xcb_input_feedback_state_data_t__bell {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_state_data_t {
    pub keyboard: xcb_input_feedback_state_data_t__keyboard,
    pub pointer: xcb_input_feedback_state_data_t__pointer,
    pub string: xcb_input_feedback_state_data_t__string,
    pub integer: xcb_input_feedback_state_data_t__integer,
    pub led: xcb_input_feedback_state_data_t__led,
    pub bell: xcb_input_feedback_state_data_t__bell,
}

impl Default for xcb_input_feedback_state_data_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_state_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
}

impl Default for xcb_input_feedback_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_state_iterator_t {
    pub data: *mut xcb_input_feedback_state_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_feedback_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_feedback_control_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_get_feedback_control_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_get_feedback_control.
pub const XCB_INPUT_GET_FEEDBACK_CONTROL: u8 = 22i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_feedback_control_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_get_feedback_control_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_feedback_control_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_feedbacks: u16,
    pub pad0: [u8; 22],
}

impl Default for xcb_input_get_feedback_control_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_kbd_feedback_ctl_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
    pub key: xcb_input_key_code_t,
    pub auto_repeat_mode: u8,
    pub key_click_percent: i8,
    pub bell_percent: i8,
    pub bell_pitch: i16,
    pub bell_duration: i16,
    pub led_mask: u32,
    pub led_values: u32,
}

impl Default for xcb_input_kbd_feedback_ctl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_kbd_feedback_ctl_iterator_t {
    pub data: *mut xcb_input_kbd_feedback_ctl_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_kbd_feedback_ctl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_ptr_feedback_ctl_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
    pub pad0: [u8; 2],
    pub num: i16,
    pub denom: i16,
    pub threshold: i16,
}

impl Default for xcb_input_ptr_feedback_ctl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_ptr_feedback_ctl_iterator_t {
    pub data: *mut xcb_input_ptr_feedback_ctl_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_ptr_feedback_ctl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_integer_feedback_ctl_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
    pub int_to_display: i32,
}

impl Default for xcb_input_integer_feedback_ctl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_integer_feedback_ctl_iterator_t {
    pub data: *mut xcb_input_integer_feedback_ctl_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_integer_feedback_ctl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_string_feedback_ctl_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
    pub pad0: [u8; 2],
    pub num_keysyms: u16,
}

impl Default for xcb_input_string_feedback_ctl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_string_feedback_ctl_iterator_t {
    pub data: *mut xcb_input_string_feedback_ctl_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_string_feedback_ctl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_bell_feedback_ctl_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
    pub percent: i8,
    pub pad0: [u8; 3],
    pub pitch: i16,
    pub duration: i16,
}

impl Default for xcb_input_bell_feedback_ctl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_bell_feedback_ctl_iterator_t {
    pub data: *mut xcb_input_bell_feedback_ctl_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_bell_feedback_ctl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_led_feedback_ctl_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
    pub led_mask: u32,
    pub led_values: u32,
}

impl Default for xcb_input_led_feedback_ctl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_led_feedback_ctl_iterator_t {
    pub data: *mut xcb_input_led_feedback_ctl_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_led_feedback_ctl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_ctl_data_t__keyboard {
    pub key: xcb_input_key_code_t,
    pub auto_repeat_mode: u8,
    pub key_click_percent: i8,
    pub bell_percent: i8,
    pub bell_pitch: i16,
    pub bell_duration: i16,
    pub led_mask: u32,
    pub led_values: u32,
}

impl Default for xcb_input_feedback_ctl_data_t__keyboard {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_ctl_data_t__pointer {
    pub pad0: [u8; 2],
    pub num: i16,
    pub denom: i16,
    pub threshold: i16,
}

impl Default for xcb_input_feedback_ctl_data_t__pointer {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_ctl_data_t__string {
    pub pad1: [u8; 2],
    pub num_keysyms: u16,
    pub keysyms: *mut xcb_keysym_t,
}

impl Default for xcb_input_feedback_ctl_data_t__string {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_ctl_data_t__integer {
    pub int_to_display: i32,
}

impl Default for xcb_input_feedback_ctl_data_t__integer {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_ctl_data_t__led {
    pub led_mask: u32,
    pub led_values: u32,
}

impl Default for xcb_input_feedback_ctl_data_t__led {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_ctl_data_t__bell {
    pub percent: i8,
    pub pad2: [u8; 3],
    pub pitch: i16,
    pub duration: i16,
}

impl Default for xcb_input_feedback_ctl_data_t__bell {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_ctl_data_t {
    pub keyboard: xcb_input_feedback_ctl_data_t__keyboard,
    pub pointer: xcb_input_feedback_ctl_data_t__pointer,
    pub string: xcb_input_feedback_ctl_data_t__string,
    pub integer: xcb_input_feedback_ctl_data_t__integer,
    pub led: xcb_input_feedback_ctl_data_t__led,
    pub bell: xcb_input_feedback_ctl_data_t__bell,
}

impl Default for xcb_input_feedback_ctl_data_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_ctl_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
}

impl Default for xcb_input_feedback_ctl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_ctl_iterator_t {
    pub data: *mut xcb_input_feedback_ctl_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_feedback_ctl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_change_feedback_control_mask_t = u32;
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_KEY_CLICK_PERCENT:
    xcb_input_change_feedback_control_mask_t = 1;
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_PERCENT: xcb_input_change_feedback_control_mask_t =
    2;
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_PITCH: xcb_input_change_feedback_control_mask_t =
    4;
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_DURATION:
    xcb_input_change_feedback_control_mask_t = 8;
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_LED: xcb_input_change_feedback_control_mask_t = 16;
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_LED_MODE:
    xcb_input_change_feedback_control_mask_t = 32;
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_KEY: xcb_input_change_feedback_control_mask_t = 64;
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_AUTO_REPEAT_MODE:
    xcb_input_change_feedback_control_mask_t = 128;
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_STRING: xcb_input_change_feedback_control_mask_t =
    1;
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_INTEGER: xcb_input_change_feedback_control_mask_t =
    1;
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_ACCEL_NUM:
    xcb_input_change_feedback_control_mask_t = 1;
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_ACCEL_DENOM:
    xcb_input_change_feedback_control_mask_t = 2;
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_THRESHOLD:
    xcb_input_change_feedback_control_mask_t = 4;

/// Opcode for xcb_input_change_feedback_control.
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL: u8 = 23i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_feedback_control_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub mask: u32,
    pub device_id: u8,
    pub feedback_id: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_change_feedback_control_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_key_mapping_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_get_device_key_mapping_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_get_device_key_mapping.
pub const XCB_INPUT_GET_DEVICE_KEY_MAPPING: u8 = 24i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_key_mapping_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub first_keycode: xcb_input_key_code_t,
    pub count: u8,
    pub pad0: u8,
}

impl Default for xcb_input_get_device_key_mapping_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_key_mapping_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub keysyms_per_keycode: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_get_device_key_mapping_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_change_device_key_mapping.
pub const XCB_INPUT_CHANGE_DEVICE_KEY_MAPPING: u8 = 25i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_device_key_mapping_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub first_keycode: xcb_input_key_code_t,
    pub keysyms_per_keycode: u8,
    pub keycode_count: u8,
}

impl Default for xcb_input_change_device_key_mapping_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_modifier_mapping_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_get_device_modifier_mapping_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_get_device_modifier_mapping.
pub const XCB_INPUT_GET_DEVICE_MODIFIER_MAPPING: u8 = 26i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_modifier_mapping_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_get_device_modifier_mapping_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_modifier_mapping_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub keycodes_per_modifier: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_get_device_modifier_mapping_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_modifier_mapping_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_set_device_modifier_mapping_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_set_device_modifier_mapping.
pub const XCB_INPUT_SET_DEVICE_MODIFIER_MAPPING: u8 = 27i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_modifier_mapping_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub keycodes_per_modifier: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_set_device_modifier_mapping_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_modifier_mapping_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_set_device_modifier_mapping_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_button_mapping_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_get_device_button_mapping_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_get_device_button_mapping.
pub const XCB_INPUT_GET_DEVICE_BUTTON_MAPPING: u8 = 28i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_button_mapping_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_get_device_button_mapping_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_button_mapping_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub map_size: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_get_device_button_mapping_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_button_mapping_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_set_device_button_mapping_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_set_device_button_mapping.
pub const XCB_INPUT_SET_DEVICE_BUTTON_MAPPING: u8 = 29i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_button_mapping_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub map_size: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_set_device_button_mapping_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_button_mapping_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_set_device_button_mapping_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_key_state_t {
    pub class_id: u8,
    pub len: u8,
    pub num_keys: u8,
    pub pad0: u8,
    pub keys: [u8; 32],
}

impl Default for xcb_input_key_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_key_state_iterator_t {
    pub data: *mut xcb_input_key_state_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_key_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_button_state_t {
    pub class_id: u8,
    pub len: u8,
    pub num_buttons: u8,
    pub pad0: u8,
    pub buttons: [u8; 32],
}

impl Default for xcb_input_button_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_button_state_iterator_t {
    pub data: *mut xcb_input_button_state_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_button_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_valuator_state_mode_mask_t = u32;
pub const XCB_INPUT_VALUATOR_STATE_MODE_MASK_DEVICE_MODE_ABSOLUTE:
    xcb_input_valuator_state_mode_mask_t = 1;
pub const XCB_INPUT_VALUATOR_STATE_MODE_MASK_OUT_OF_PROXIMITY:
    xcb_input_valuator_state_mode_mask_t = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_valuator_state_t {
    pub class_id: u8,
    pub len: u8,
    pub num_valuators: u8,
    pub mode: u8,
}

impl Default for xcb_input_valuator_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_valuator_state_iterator_t {
    pub data: *mut xcb_input_valuator_state_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_valuator_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_state_data_t__key {
    pub num_keys: u8,
    pub pad0: u8,
    pub keys: [u8; 32],
}

impl Default for xcb_input_input_state_data_t__key {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_state_data_t__button {
    pub num_buttons: u8,
    pub pad1: u8,
    pub buttons: [u8; 32],
}

impl Default for xcb_input_input_state_data_t__button {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_state_data_t__valuator {
    pub num_valuators: u8,
    pub mode: u8,
    pub valuators: *mut i32,
}

impl Default for xcb_input_input_state_data_t__valuator {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_state_data_t {
    pub key: xcb_input_input_state_data_t__key,
    pub button: xcb_input_input_state_data_t__button,
    pub valuator: xcb_input_input_state_data_t__valuator,
}

impl Default for xcb_input_input_state_data_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_state_t {
    pub class_id: u8,
    pub len: u8,
}

impl Default for xcb_input_input_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_state_iterator_t {
    pub data: *mut xcb_input_input_state_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_input_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_query_device_state_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_query_device_state_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_query_device_state.
pub const XCB_INPUT_QUERY_DEVICE_STATE: u8 = 30i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_query_device_state_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_query_device_state_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_query_device_state_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_classes: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_query_device_state_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_device_bell.
pub const XCB_INPUT_DEVICE_BELL: u8 = 32i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_bell_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub feedback_id: u8,
    pub feedback_class: u8,
    pub percent: i8,
}

impl Default for xcb_input_device_bell_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_valuators_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_set_device_valuators_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_set_device_valuators.
pub const XCB_INPUT_SET_DEVICE_VALUATORS: u8 = 33i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_valuators_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub first_valuator: u8,
    pub num_valuators: u8,
    pub pad0: u8,
}

impl Default for xcb_input_set_device_valuators_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_valuators_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_set_device_valuators_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_device_control_t = u32;
pub const XCB_INPUT_DEVICE_CONTROL_RESOLUTION: xcb_input_device_control_t = 1;
pub const XCB_INPUT_DEVICE_CONTROL_ABS_CALIB: xcb_input_device_control_t = 2;
pub const XCB_INPUT_DEVICE_CONTROL_CORE: xcb_input_device_control_t = 3;
pub const XCB_INPUT_DEVICE_CONTROL_ENABLE: xcb_input_device_control_t = 4;
pub const XCB_INPUT_DEVICE_CONTROL_ABS_AREA: xcb_input_device_control_t = 5;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_resolution_state_t {
    pub control_id: u16,
    pub len: u16,
    pub num_valuators: u32,
}

impl Default for xcb_input_device_resolution_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_resolution_state_iterator_t {
    pub data: *mut xcb_input_device_resolution_state_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_device_resolution_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_abs_calib_state_t {
    pub control_id: u16,
    pub len: u16,
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub flip_x: u32,
    pub flip_y: u32,
    pub rotation: u32,
    pub button_threshold: u32,
}

impl Default for xcb_input_device_abs_calib_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_abs_calib_state_iterator_t {
    pub data: *mut xcb_input_device_abs_calib_state_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_device_abs_calib_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_abs_area_state_t {
    pub control_id: u16,
    pub len: u16,
    pub offset_x: u32,
    pub offset_y: u32,
    pub width: u32,
    pub height: u32,
    pub screen: u32,
    pub following: u32,
}

impl Default for xcb_input_device_abs_area_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_abs_area_state_iterator_t {
    pub data: *mut xcb_input_device_abs_area_state_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_device_abs_area_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_core_state_t {
    pub control_id: u16,
    pub len: u16,
    pub status: u8,
    pub iscore: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_device_core_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_core_state_iterator_t {
    pub data: *mut xcb_input_device_core_state_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_device_core_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_enable_state_t {
    pub control_id: u16,
    pub len: u16,
    pub enable: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_device_enable_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_enable_state_iterator_t {
    pub data: *mut xcb_input_device_enable_state_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_device_enable_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_state_data_t__resolution {
    pub num_valuators: u32,
    pub resolution_values: *mut u32,
    pub resolution_min: *mut u32,
    pub resolution_max: *mut u32,
}

impl Default for xcb_input_device_state_data_t__resolution {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_state_data_t__abs_calib {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub flip_x: u32,
    pub flip_y: u32,
    pub rotation: u32,
    pub button_threshold: u32,
}

impl Default for xcb_input_device_state_data_t__abs_calib {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_state_data_t__core {
    pub status: u8,
    pub iscore: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_device_state_data_t__core {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_state_data_t__enable {
    pub enable: u8,
    pub pad1: [u8; 3],
}

impl Default for xcb_input_device_state_data_t__enable {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_state_data_t__abs_area {
    pub offset_x: u32,
    pub offset_y: u32,
    pub width: u32,
    pub height: u32,
    pub screen: u32,
    pub following: u32,
}

impl Default for xcb_input_device_state_data_t__abs_area {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_state_data_t {
    pub resolution: xcb_input_device_state_data_t__resolution,
    pub abs_calib: xcb_input_device_state_data_t__abs_calib,
    pub core: xcb_input_device_state_data_t__core,
    pub enable: xcb_input_device_state_data_t__enable,
    pub abs_area: xcb_input_device_state_data_t__abs_area,
}

impl Default for xcb_input_device_state_data_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_state_t {
    pub control_id: u16,
    pub len: u16,
}

impl Default for xcb_input_device_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_state_iterator_t {
    pub data: *mut xcb_input_device_state_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_device_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_control_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_get_device_control_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_get_device_control.
pub const XCB_INPUT_GET_DEVICE_CONTROL: u8 = 34i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_control_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub control_id: u16,
    pub device_id: u8,
    pub pad0: u8,
}

impl Default for xcb_input_get_device_control_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_control_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_get_device_control_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_resolution_ctl_t {
    pub control_id: u16,
    pub len: u16,
    pub first_valuator: u8,
    pub num_valuators: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_device_resolution_ctl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_resolution_ctl_iterator_t {
    pub data: *mut xcb_input_device_resolution_ctl_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_device_resolution_ctl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_abs_calib_ctl_t {
    pub control_id: u16,
    pub len: u16,
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub flip_x: u32,
    pub flip_y: u32,
    pub rotation: u32,
    pub button_threshold: u32,
}

impl Default for xcb_input_device_abs_calib_ctl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_abs_calib_ctl_iterator_t {
    pub data: *mut xcb_input_device_abs_calib_ctl_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_device_abs_calib_ctl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_abs_area_ctrl_t {
    pub control_id: u16,
    pub len: u16,
    pub offset_x: u32,
    pub offset_y: u32,
    pub width: i32,
    pub height: i32,
    pub screen: i32,
    pub following: u32,
}

impl Default for xcb_input_device_abs_area_ctrl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_abs_area_ctrl_iterator_t {
    pub data: *mut xcb_input_device_abs_area_ctrl_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_device_abs_area_ctrl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_core_ctrl_t {
    pub control_id: u16,
    pub len: u16,
    pub status: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_device_core_ctrl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_core_ctrl_iterator_t {
    pub data: *mut xcb_input_device_core_ctrl_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_device_core_ctrl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_enable_ctrl_t {
    pub control_id: u16,
    pub len: u16,
    pub enable: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_device_enable_ctrl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_enable_ctrl_iterator_t {
    pub data: *mut xcb_input_device_enable_ctrl_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_device_enable_ctrl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_ctl_data_t__resolution {
    pub first_valuator: u8,
    pub num_valuators: u8,
    pub pad0: [u8; 2],
    pub resolution_values: *mut u32,
}

impl Default for xcb_input_device_ctl_data_t__resolution {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_ctl_data_t__abs_calib {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub flip_x: u32,
    pub flip_y: u32,
    pub rotation: u32,
    pub button_threshold: u32,
}

impl Default for xcb_input_device_ctl_data_t__abs_calib {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_ctl_data_t__core {
    pub status: u8,
    pub pad1: [u8; 3],
}

impl Default for xcb_input_device_ctl_data_t__core {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_ctl_data_t__enable {
    pub enable: u8,
    pub pad2: [u8; 3],
}

impl Default for xcb_input_device_ctl_data_t__enable {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_ctl_data_t__abs_area {
    pub offset_x: u32,
    pub offset_y: u32,
    pub width: i32,
    pub height: i32,
    pub screen: i32,
    pub following: u32,
}

impl Default for xcb_input_device_ctl_data_t__abs_area {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_ctl_data_t {
    pub resolution: xcb_input_device_ctl_data_t__resolution,
    pub abs_calib: xcb_input_device_ctl_data_t__abs_calib,
    pub core: xcb_input_device_ctl_data_t__core,
    pub enable: xcb_input_device_ctl_data_t__enable,
    pub abs_area: xcb_input_device_ctl_data_t__abs_area,
}

impl Default for xcb_input_device_ctl_data_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_ctl_t {
    pub control_id: u16,
    pub len: u16,
}

impl Default for xcb_input_device_ctl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_ctl_iterator_t {
    pub data: *mut xcb_input_device_ctl_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_device_ctl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_device_control_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_change_device_control_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_change_device_control.
pub const XCB_INPUT_CHANGE_DEVICE_CONTROL: u8 = 35i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_device_control_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub control_id: u16,
    pub device_id: u8,
    pub pad0: u8,
}

impl Default for xcb_input_change_device_control_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_device_control_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_change_device_control_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_list_device_properties_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_list_device_properties_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_list_device_properties.
pub const XCB_INPUT_LIST_DEVICE_PROPERTIES: u8 = 36i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_list_device_properties_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_list_device_properties_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_list_device_properties_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_atoms: u16,
    pub pad0: [u8; 22],
}

impl Default for xcb_input_list_device_properties_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_property_format_t = u32;
pub const XCB_INPUT_PROPERTY_FORMAT_8_BITS: xcb_input_property_format_t = 8;
pub const XCB_INPUT_PROPERTY_FORMAT_16_BITS: xcb_input_property_format_t = 16;
pub const XCB_INPUT_PROPERTY_FORMAT_32_BITS: xcb_input_property_format_t = 32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_device_property_items_t {
    pub data8: *mut u8,
    pub data16: *mut u16,
    pub data32: *mut u32,
}

impl Default for xcb_input_change_device_property_items_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_change_device_property.
pub const XCB_INPUT_CHANGE_DEVICE_PROPERTY: u8 = 37i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_device_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub property: xcb_atom_t,
    pub type_: xcb_atom_t,
    pub device_id: u8,
    pub format: u8,
    pub mode: u8,
    pub pad0: u8,
    pub num_items: u32,
}

impl Default for xcb_input_change_device_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_delete_device_property.
pub const XCB_INPUT_DELETE_DEVICE_PROPERTY: u8 = 38i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_delete_device_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub property: xcb_atom_t,
    pub device_id: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_delete_device_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_property_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_get_device_property_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_get_device_property.
pub const XCB_INPUT_GET_DEVICE_PROPERTY: u8 = 39i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub property: xcb_atom_t,
    pub type_: xcb_atom_t,
    pub offset: u32,
    pub len: u32,
    pub device_id: u8,
    pub delete: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_get_device_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_property_items_t {
    pub data8: *mut u8,
    pub data16: *mut u16,
    pub data32: *mut u32,
}

impl Default for xcb_input_get_device_property_items_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_property_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub type_: xcb_atom_t,
    pub bytes_after: u32,
    pub num_items: u32,
    pub format: u8,
    pub device_id: u8,
    pub pad0: [u8; 10],
}

impl Default for xcb_input_get_device_property_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_device_t = u32;
pub const XCB_INPUT_DEVICE_ALL: xcb_input_device_t = 0;
pub const XCB_INPUT_DEVICE_ALL_MASTER: xcb_input_device_t = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_group_info_t {
    pub base: u8,
    pub latched: u8,
    pub locked: u8,
    pub effective: u8,
}

impl Default for xcb_input_group_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_group_info_iterator_t {
    pub data: *mut xcb_input_group_info_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_group_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_modifier_info_t {
    pub base: u32,
    pub latched: u32,
    pub locked: u32,
    pub effective: u32,
}

impl Default for xcb_input_modifier_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_modifier_info_iterator_t {
    pub data: *mut xcb_input_modifier_info_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_modifier_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_query_pointer_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_xi_query_pointer_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_xi_query_pointer.
pub const XCB_INPUT_XI_QUERY_POINTER: u8 = 40i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_query_pointer_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub deviceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_xi_query_pointer_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_query_pointer_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: xcb_input_fp1616_t,
    pub root_y: xcb_input_fp1616_t,
    pub win_x: xcb_input_fp1616_t,
    pub win_y: xcb_input_fp1616_t,
    pub same_screen: u8,
    pub pad1: u8,
    pub buttons_len: u16,
    pub mods: xcb_input_modifier_info_t,
    pub group: xcb_input_group_info_t,
}

impl Default for xcb_input_xi_query_pointer_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_xi_warp_pointer.
pub const XCB_INPUT_XI_WARP_POINTER: u8 = 41i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_warp_pointer_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub src_win: xcb_window_t,
    pub dst_win: xcb_window_t,
    pub src_x: xcb_input_fp1616_t,
    pub src_y: xcb_input_fp1616_t,
    pub src_width: u16,
    pub src_height: u16,
    pub dst_x: xcb_input_fp1616_t,
    pub dst_y: xcb_input_fp1616_t,
    pub deviceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_xi_warp_pointer_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_xi_change_cursor.
pub const XCB_INPUT_XI_CHANGE_CURSOR: u8 = 42i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_change_cursor_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub cursor: xcb_cursor_t,
    pub deviceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_xi_change_cursor_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_hierarchy_change_type_t = u32;
pub const XCB_INPUT_HIERARCHY_CHANGE_TYPE_ADD_MASTER: xcb_input_hierarchy_change_type_t = 1;
pub const XCB_INPUT_HIERARCHY_CHANGE_TYPE_REMOVE_MASTER: xcb_input_hierarchy_change_type_t = 2;
pub const XCB_INPUT_HIERARCHY_CHANGE_TYPE_ATTACH_SLAVE: xcb_input_hierarchy_change_type_t = 3;
pub const XCB_INPUT_HIERARCHY_CHANGE_TYPE_DETACH_SLAVE: xcb_input_hierarchy_change_type_t = 4;

pub type xcb_input_change_mode_t = u32;
pub const XCB_INPUT_CHANGE_MODE_ATTACH: xcb_input_change_mode_t = 1;
pub const XCB_INPUT_CHANGE_MODE_FLOAT: xcb_input_change_mode_t = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_add_master_t {
    pub type_: u16,
    pub len: u16,
    pub name_len: u16,
    pub send_core: u8,
    pub enable: u8,
}

impl Default for xcb_input_add_master_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_add_master_iterator_t {
    pub data: *mut xcb_input_add_master_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_add_master_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_remove_master_t {
    pub type_: u16,
    pub len: u16,
    pub deviceid: xcb_input_device_id_t,
    pub return_mode: u8,
    pub pad0: u8,
    pub return_pointer: xcb_input_device_id_t,
    pub return_keyboard: xcb_input_device_id_t,
}

impl Default for xcb_input_remove_master_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_remove_master_iterator_t {
    pub data: *mut xcb_input_remove_master_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_remove_master_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_attach_slave_t {
    pub type_: u16,
    pub len: u16,
    pub deviceid: xcb_input_device_id_t,
    pub master: xcb_input_device_id_t,
}

impl Default for xcb_input_attach_slave_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_attach_slave_iterator_t {
    pub data: *mut xcb_input_attach_slave_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_attach_slave_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_detach_slave_t {
    pub type_: u16,
    pub len: u16,
    pub deviceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_detach_slave_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_detach_slave_iterator_t {
    pub data: *mut xcb_input_detach_slave_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_detach_slave_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_hierarchy_change_data_t__add_master {
    pub name_len: u16,
    pub send_core: u8,
    pub enable: u8,
    pub name: *mut c_char,
}

impl Default for xcb_input_hierarchy_change_data_t__add_master {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_hierarchy_change_data_t__remove_master {
    pub deviceid: xcb_input_device_id_t,
    pub return_mode: u8,
    pub pad1: u8,
    pub return_pointer: xcb_input_device_id_t,
    pub return_keyboard: xcb_input_device_id_t,
}

impl Default for xcb_input_hierarchy_change_data_t__remove_master {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_hierarchy_change_data_t__attach_slave {
    pub deviceid: xcb_input_device_id_t,
    pub master: xcb_input_device_id_t,
}

impl Default for xcb_input_hierarchy_change_data_t__attach_slave {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_hierarchy_change_data_t__detach_slave {
    pub deviceid: xcb_input_device_id_t,
    pub pad2: [u8; 2],
}

impl Default for xcb_input_hierarchy_change_data_t__detach_slave {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_hierarchy_change_data_t {
    pub add_master: xcb_input_hierarchy_change_data_t__add_master,
    pub remove_master: xcb_input_hierarchy_change_data_t__remove_master,
    pub attach_slave: xcb_input_hierarchy_change_data_t__attach_slave,
    pub detach_slave: xcb_input_hierarchy_change_data_t__detach_slave,
}

impl Default for xcb_input_hierarchy_change_data_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_hierarchy_change_t {
    pub type_: u16,
    pub len: u16,
}

impl Default for xcb_input_hierarchy_change_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_hierarchy_change_iterator_t {
    pub data: *mut xcb_input_hierarchy_change_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_hierarchy_change_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_xi_change_hierarchy.
pub const XCB_INPUT_XI_CHANGE_HIERARCHY: u8 = 43i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_change_hierarchy_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub num_changes: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_xi_change_hierarchy_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_xi_set_client_pointer.
pub const XCB_INPUT_XI_SET_CLIENT_POINTER: u8 = 44i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_set_client_pointer_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub deviceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_xi_set_client_pointer_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_client_pointer_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_xi_get_client_pointer_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_xi_get_client_pointer.
pub const XCB_INPUT_XI_GET_CLIENT_POINTER: u8 = 45i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_client_pointer_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_input_xi_get_client_pointer_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_client_pointer_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub set: u8,
    pub pad1: u8,
    pub deviceid: xcb_input_device_id_t,
    pub pad2: [u8; 20],
}

impl Default for xcb_input_xi_get_client_pointer_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_xi_event_mask_t = u32;
pub const XCB_INPUT_XI_EVENT_MASK_DEVICE_CHANGED: xcb_input_xi_event_mask_t = 2;
pub const XCB_INPUT_XI_EVENT_MASK_KEY_PRESS: xcb_input_xi_event_mask_t = 4;
pub const XCB_INPUT_XI_EVENT_MASK_KEY_RELEASE: xcb_input_xi_event_mask_t = 8;
pub const XCB_INPUT_XI_EVENT_MASK_BUTTON_PRESS: xcb_input_xi_event_mask_t = 16;
pub const XCB_INPUT_XI_EVENT_MASK_BUTTON_RELEASE: xcb_input_xi_event_mask_t = 32;
pub const XCB_INPUT_XI_EVENT_MASK_MOTION: xcb_input_xi_event_mask_t = 64;
pub const XCB_INPUT_XI_EVENT_MASK_ENTER: xcb_input_xi_event_mask_t = 128;
pub const XCB_INPUT_XI_EVENT_MASK_LEAVE: xcb_input_xi_event_mask_t = 256;
pub const XCB_INPUT_XI_EVENT_MASK_FOCUS_IN: xcb_input_xi_event_mask_t = 512;
pub const XCB_INPUT_XI_EVENT_MASK_FOCUS_OUT: xcb_input_xi_event_mask_t = 1024;
pub const XCB_INPUT_XI_EVENT_MASK_HIERARCHY: xcb_input_xi_event_mask_t = 2048;
pub const XCB_INPUT_XI_EVENT_MASK_PROPERTY: xcb_input_xi_event_mask_t = 4096;
pub const XCB_INPUT_XI_EVENT_MASK_RAW_KEY_PRESS: xcb_input_xi_event_mask_t = 8192;
pub const XCB_INPUT_XI_EVENT_MASK_RAW_KEY_RELEASE: xcb_input_xi_event_mask_t = 16384;
pub const XCB_INPUT_XI_EVENT_MASK_RAW_BUTTON_PRESS: xcb_input_xi_event_mask_t = 32768;
pub const XCB_INPUT_XI_EVENT_MASK_RAW_BUTTON_RELEASE: xcb_input_xi_event_mask_t = 65536;
pub const XCB_INPUT_XI_EVENT_MASK_RAW_MOTION: xcb_input_xi_event_mask_t = 131072;
pub const XCB_INPUT_XI_EVENT_MASK_TOUCH_BEGIN: xcb_input_xi_event_mask_t = 262144;
pub const XCB_INPUT_XI_EVENT_MASK_TOUCH_UPDATE: xcb_input_xi_event_mask_t = 524288;
pub const XCB_INPUT_XI_EVENT_MASK_TOUCH_END: xcb_input_xi_event_mask_t = 1048576;
pub const XCB_INPUT_XI_EVENT_MASK_TOUCH_OWNERSHIP: xcb_input_xi_event_mask_t = 2097152;
pub const XCB_INPUT_XI_EVENT_MASK_RAW_TOUCH_BEGIN: xcb_input_xi_event_mask_t = 4194304;
pub const XCB_INPUT_XI_EVENT_MASK_RAW_TOUCH_UPDATE: xcb_input_xi_event_mask_t = 8388608;
pub const XCB_INPUT_XI_EVENT_MASK_RAW_TOUCH_END: xcb_input_xi_event_mask_t = 16777216;
pub const XCB_INPUT_XI_EVENT_MASK_BARRIER_HIT: xcb_input_xi_event_mask_t = 33554432;
pub const XCB_INPUT_XI_EVENT_MASK_BARRIER_LEAVE: xcb_input_xi_event_mask_t = 67108864;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_event_mask_t {
    pub deviceid: xcb_input_device_id_t,
    pub mask_len: u16,
}

impl Default for xcb_input_event_mask_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_event_mask_iterator_t {
    pub data: *mut xcb_input_event_mask_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_event_mask_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_xi_select_events.
pub const XCB_INPUT_XI_SELECT_EVENTS: u8 = 46i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_select_events_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub num_mask: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_xi_select_events_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_query_version_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_xi_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_xi_query_version.
pub const XCB_INPUT_XI_QUERY_VERSION: u8 = 47i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub major_version: u16,
    pub minor_version: u16,
}

impl Default for xcb_input_xi_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub pad1: [u8; 20],
}

impl Default for xcb_input_xi_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_device_class_type_t = u32;
pub const XCB_INPUT_DEVICE_CLASS_TYPE_KEY: xcb_input_device_class_type_t = 0;
pub const XCB_INPUT_DEVICE_CLASS_TYPE_BUTTON: xcb_input_device_class_type_t = 1;
pub const XCB_INPUT_DEVICE_CLASS_TYPE_VALUATOR: xcb_input_device_class_type_t = 2;
pub const XCB_INPUT_DEVICE_CLASS_TYPE_SCROLL: xcb_input_device_class_type_t = 3;
pub const XCB_INPUT_DEVICE_CLASS_TYPE_TOUCH: xcb_input_device_class_type_t = 8;

pub type xcb_input_device_type_t = u32;
pub const XCB_INPUT_DEVICE_TYPE_MASTER_POINTER: xcb_input_device_type_t = 1;
pub const XCB_INPUT_DEVICE_TYPE_MASTER_KEYBOARD: xcb_input_device_type_t = 2;
pub const XCB_INPUT_DEVICE_TYPE_SLAVE_POINTER: xcb_input_device_type_t = 3;
pub const XCB_INPUT_DEVICE_TYPE_SLAVE_KEYBOARD: xcb_input_device_type_t = 4;
pub const XCB_INPUT_DEVICE_TYPE_FLOATING_SLAVE: xcb_input_device_type_t = 5;

pub type xcb_input_scroll_flags_t = u32;
pub const XCB_INPUT_SCROLL_FLAGS_NO_EMULATION: xcb_input_scroll_flags_t = 1;
pub const XCB_INPUT_SCROLL_FLAGS_PREFERRED: xcb_input_scroll_flags_t = 2;

pub type xcb_input_scroll_type_t = u32;
pub const XCB_INPUT_SCROLL_TYPE_VERTICAL: xcb_input_scroll_type_t = 1;
pub const XCB_INPUT_SCROLL_TYPE_HORIZONTAL: xcb_input_scroll_type_t = 2;

pub type xcb_input_touch_mode_t = u32;
pub const XCB_INPUT_TOUCH_MODE_DIRECT: xcb_input_touch_mode_t = 1;
pub const XCB_INPUT_TOUCH_MODE_DEPENDENT: xcb_input_touch_mode_t = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_button_class_t {
    pub type_: u16,
    pub len: u16,
    pub sourceid: xcb_input_device_id_t,
    pub num_buttons: u16,
}

impl Default for xcb_input_button_class_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_button_class_iterator_t {
    pub data: *mut xcb_input_button_class_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_button_class_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_key_class_t {
    pub type_: u16,
    pub len: u16,
    pub sourceid: xcb_input_device_id_t,
    pub num_keys: u16,
}

impl Default for xcb_input_key_class_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_key_class_iterator_t {
    pub data: *mut xcb_input_key_class_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_key_class_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_scroll_class_t {
    pub type_: u16,
    pub len: u16,
    pub sourceid: xcb_input_device_id_t,
    pub number: u16,
    pub scroll_type: u16,
    pub pad0: [u8; 2],
    pub flags: u32,
    pub increment: xcb_input_fp3232_t,
}

impl Default for xcb_input_scroll_class_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_scroll_class_iterator_t {
    pub data: *mut xcb_input_scroll_class_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_scroll_class_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_touch_class_t {
    pub type_: u16,
    pub len: u16,
    pub sourceid: xcb_input_device_id_t,
    pub mode: u8,
    pub num_touches: u8,
}

impl Default for xcb_input_touch_class_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_touch_class_iterator_t {
    pub data: *mut xcb_input_touch_class_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_touch_class_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_valuator_class_t {
    pub type_: u16,
    pub len: u16,
    pub sourceid: xcb_input_device_id_t,
    pub number: u16,
    pub label: xcb_atom_t,
    pub min: xcb_input_fp3232_t,
    pub max: xcb_input_fp3232_t,
    pub value: xcb_input_fp3232_t,
    pub resolution: u32,
    pub mode: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_valuator_class_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_valuator_class_iterator_t {
    pub data: *mut xcb_input_valuator_class_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_valuator_class_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_class_data_t__key {
    pub num_keys: u16,
    pub keys: *mut u32,
}

impl Default for xcb_input_device_class_data_t__key {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_class_data_t__button {
    pub num_buttons: u16,
    pub state: *mut u32,
    pub labels: *mut xcb_atom_t,
}

impl Default for xcb_input_device_class_data_t__button {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_class_data_t__valuator {
    pub number: u16,
    pub label: xcb_atom_t,
    pub min: xcb_input_fp3232_t,
    pub max: xcb_input_fp3232_t,
    pub value: xcb_input_fp3232_t,
    pub resolution: u32,
    pub mode: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_device_class_data_t__valuator {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_class_data_t__scroll {
    pub number: u16,
    pub scroll_type: u16,
    pub pad1: [u8; 2],
    pub flags: u32,
    pub increment: xcb_input_fp3232_t,
}

impl Default for xcb_input_device_class_data_t__scroll {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_class_data_t__touch {
    pub mode: u8,
    pub num_touches: u8,
}

impl Default for xcb_input_device_class_data_t__touch {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_class_data_t {
    pub key: xcb_input_device_class_data_t__key,
    pub button: xcb_input_device_class_data_t__button,
    pub valuator: xcb_input_device_class_data_t__valuator,
    pub scroll: xcb_input_device_class_data_t__scroll,
    pub touch: xcb_input_device_class_data_t__touch,
}

impl Default for xcb_input_device_class_data_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_class_t {
    pub type_: u16,
    pub len: u16,
    pub sourceid: xcb_input_device_id_t,
}

impl Default for xcb_input_device_class_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_class_iterator_t {
    pub data: *mut xcb_input_device_class_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_device_class_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_device_info_t {
    pub deviceid: xcb_input_device_id_t,
    pub type_: u16,
    pub attachment: xcb_input_device_id_t,
    pub num_classes: u16,
    pub name_len: u16,
    pub enabled: u8,
    pub pad0: u8,
}

impl Default for xcb_input_xi_device_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_device_info_iterator_t {
    pub data: *mut xcb_input_xi_device_info_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_xi_device_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_query_device_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_xi_query_device_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_xi_query_device.
pub const XCB_INPUT_XI_QUERY_DEVICE: u8 = 48i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_query_device_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub deviceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_xi_query_device_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_query_device_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_infos: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_input_xi_query_device_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_xi_set_focus.
pub const XCB_INPUT_XI_SET_FOCUS: u8 = 49i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_set_focus_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub time: xcb_timestamp_t,
    pub deviceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_xi_set_focus_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_focus_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_xi_get_focus_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_xi_get_focus.
pub const XCB_INPUT_XI_GET_FOCUS: u8 = 50i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_focus_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub deviceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_xi_get_focus_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_focus_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub focus: xcb_window_t,
    pub pad1: [u8; 20],
}

impl Default for xcb_input_xi_get_focus_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_grab_owner_t = u32;
pub const XCB_INPUT_GRAB_OWNER_NO_OWNER: xcb_input_grab_owner_t = 0;
pub const XCB_INPUT_GRAB_OWNER_OWNER: xcb_input_grab_owner_t = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_grab_device_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_xi_grab_device_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_xi_grab_device.
pub const XCB_INPUT_XI_GRAB_DEVICE: u8 = 51i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_grab_device_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub time: xcb_timestamp_t,
    pub cursor: xcb_cursor_t,
    pub deviceid: xcb_input_device_id_t,
    pub mode: u8,
    pub paired_device_mode: u8,
    pub owner_events: u8,
    pub pad0: u8,
    pub mask_len: u16,
}

impl Default for xcb_input_xi_grab_device_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_grab_device_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u8,
    pub pad1: [u8; 23],
}

impl Default for xcb_input_xi_grab_device_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_xi_ungrab_device.
pub const XCB_INPUT_XI_UNGRAB_DEVICE: u8 = 52i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_ungrab_device_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub time: xcb_timestamp_t,
    pub deviceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_xi_ungrab_device_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_event_mode_t = u32;
pub const XCB_INPUT_EVENT_MODE_ASYNC_DEVICE: xcb_input_event_mode_t = 0;
pub const XCB_INPUT_EVENT_MODE_SYNC_DEVICE: xcb_input_event_mode_t = 1;
pub const XCB_INPUT_EVENT_MODE_REPLAY_DEVICE: xcb_input_event_mode_t = 2;
pub const XCB_INPUT_EVENT_MODE_ASYNC_PAIRED_DEVICE: xcb_input_event_mode_t = 3;
pub const XCB_INPUT_EVENT_MODE_ASYNC_PAIR: xcb_input_event_mode_t = 4;
pub const XCB_INPUT_EVENT_MODE_SYNC_PAIR: xcb_input_event_mode_t = 5;
pub const XCB_INPUT_EVENT_MODE_ACCEPT_TOUCH: xcb_input_event_mode_t = 6;
pub const XCB_INPUT_EVENT_MODE_REJECT_TOUCH: xcb_input_event_mode_t = 7;

/// Opcode for xcb_input_xi_allow_events.
pub const XCB_INPUT_XI_ALLOW_EVENTS: u8 = 53i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_allow_events_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub time: xcb_timestamp_t,
    pub deviceid: xcb_input_device_id_t,
    pub event_mode: u8,
    pub pad0: u8,
    pub touchid: u32,
    pub grab_window: xcb_window_t,
}

impl Default for xcb_input_xi_allow_events_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_grab_mode_22_t = u32;
pub const XCB_INPUT_GRAB_MODE_22_SYNC: xcb_input_grab_mode_22_t = 0;
pub const XCB_INPUT_GRAB_MODE_22_ASYNC: xcb_input_grab_mode_22_t = 1;
pub const XCB_INPUT_GRAB_MODE_22_TOUCH: xcb_input_grab_mode_22_t = 2;

pub type xcb_input_grab_type_t = u32;
pub const XCB_INPUT_GRAB_TYPE_BUTTON: xcb_input_grab_type_t = 0;
pub const XCB_INPUT_GRAB_TYPE_KEYCODE: xcb_input_grab_type_t = 1;
pub const XCB_INPUT_GRAB_TYPE_ENTER: xcb_input_grab_type_t = 2;
pub const XCB_INPUT_GRAB_TYPE_FOCUS_IN: xcb_input_grab_type_t = 3;
pub const XCB_INPUT_GRAB_TYPE_TOUCH_BEGIN: xcb_input_grab_type_t = 4;

pub type xcb_input_modifier_mask_t = u32;
pub const XCB_INPUT_MODIFIER_MASK_ANY: xcb_input_modifier_mask_t = 2147483648;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_grab_modifier_info_t {
    pub modifiers: u32,
    pub status: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_grab_modifier_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_grab_modifier_info_iterator_t {
    pub data: *mut xcb_input_grab_modifier_info_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_grab_modifier_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_passive_grab_device_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_xi_passive_grab_device_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_xi_passive_grab_device.
pub const XCB_INPUT_XI_PASSIVE_GRAB_DEVICE: u8 = 54i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_passive_grab_device_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub time: xcb_timestamp_t,
    pub grab_window: xcb_window_t,
    pub cursor: xcb_cursor_t,
    pub detail: u32,
    pub deviceid: xcb_input_device_id_t,
    pub num_modifiers: u16,
    pub mask_len: u16,
    pub grab_type: u8,
    pub grab_mode: u8,
    pub paired_device_mode: u8,
    pub owner_events: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_xi_passive_grab_device_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_passive_grab_device_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_modifiers: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_input_xi_passive_grab_device_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_xi_passive_ungrab_device.
pub const XCB_INPUT_XI_PASSIVE_UNGRAB_DEVICE: u8 = 55i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_passive_ungrab_device_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub detail: u32,
    pub deviceid: xcb_input_device_id_t,
    pub num_modifiers: u16,
    pub grab_type: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_xi_passive_ungrab_device_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_list_properties_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_xi_list_properties_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_xi_list_properties.
pub const XCB_INPUT_XI_LIST_PROPERTIES: u8 = 56i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_list_properties_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub deviceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_xi_list_properties_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_list_properties_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_properties: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_input_xi_list_properties_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_change_property_items_t {
    pub data8: *mut u8,
    pub data16: *mut u16,
    pub data32: *mut u32,
}

impl Default for xcb_input_xi_change_property_items_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_xi_change_property.
pub const XCB_INPUT_XI_CHANGE_PROPERTY: u8 = 57i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_change_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub deviceid: xcb_input_device_id_t,
    pub mode: u8,
    pub format: u8,
    pub property: xcb_atom_t,
    pub type_: xcb_atom_t,
    pub num_items: u32,
}

impl Default for xcb_input_xi_change_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_xi_delete_property.
pub const XCB_INPUT_XI_DELETE_PROPERTY: u8 = 58i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_delete_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub deviceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
    pub property: xcb_atom_t,
}

impl Default for xcb_input_xi_delete_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_property_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_xi_get_property_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_xi_get_property.
pub const XCB_INPUT_XI_GET_PROPERTY: u8 = 59i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub deviceid: xcb_input_device_id_t,
    pub delete: u8,
    pub pad0: u8,
    pub property: xcb_atom_t,
    pub type_: xcb_atom_t,
    pub offset: u32,
    pub len: u32,
}

impl Default for xcb_input_xi_get_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_property_items_t {
    pub data8: *mut u8,
    pub data16: *mut u16,
    pub data32: *mut u32,
}

impl Default for xcb_input_xi_get_property_items_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_property_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub type_: xcb_atom_t,
    pub bytes_after: u32,
    pub num_items: u32,
    pub format: u8,
    pub pad1: [u8; 11],
}

impl Default for xcb_input_xi_get_property_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_selected_events_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_input_xi_get_selected_events_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_xi_get_selected_events.
pub const XCB_INPUT_XI_GET_SELECTED_EVENTS: u8 = 60i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_selected_events_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_input_xi_get_selected_events_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_selected_events_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_masks: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_input_xi_get_selected_events_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_barrier_release_pointer_info_t {
    pub deviceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
    pub barrier: xcb_xfixes_barrier_t,
    pub eventid: u32,
}

impl Default for xcb_input_barrier_release_pointer_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_barrier_release_pointer_info_iterator_t {
    pub data: *mut xcb_input_barrier_release_pointer_info_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_barrier_release_pointer_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_xi_barrier_release_pointer.
pub const XCB_INPUT_XI_BARRIER_RELEASE_POINTER: u8 = 61i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_barrier_release_pointer_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub num_barriers: u32,
}

impl Default for xcb_input_xi_barrier_release_pointer_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_device_valuator.
pub const XCB_INPUT_DEVICE_VALUATOR: u8 = 0i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_valuator_event_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub device_state: u16,
    pub num_valuators: u8,
    pub first_valuator: u8,
    pub valuators: [i32; 6],
}

impl Default for xcb_input_device_valuator_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_more_events_mask_t = u32;
pub const XCB_INPUT_MORE_EVENTS_MASK_MORE_EVENTS: xcb_input_more_events_mask_t = 128;

/// Opcode for xcb_input_device_key_press.
pub const XCB_INPUT_DEVICE_KEY_PRESS: u8 = 1i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_key_press_event_t {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: u8,
    pub device_id: u8,
}

impl Default for xcb_input_device_key_press_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_device_key_release.
pub const XCB_INPUT_DEVICE_KEY_RELEASE: u8 = 2i32 as u8;

pub type xcb_input_device_key_release_event_t = xcb_input_device_key_press_event_t;

/// Opcode for xcb_input_device_button_press.
pub const XCB_INPUT_DEVICE_BUTTON_PRESS: u8 = 3i32 as u8;

pub type xcb_input_device_button_press_event_t = xcb_input_device_key_press_event_t;

/// Opcode for xcb_input_device_button_release.
pub const XCB_INPUT_DEVICE_BUTTON_RELEASE: u8 = 4i32 as u8;

pub type xcb_input_device_button_release_event_t = xcb_input_device_key_press_event_t;

/// Opcode for xcb_input_device_motion_notify.
pub const XCB_INPUT_DEVICE_MOTION_NOTIFY: u8 = 5i32 as u8;

pub type xcb_input_device_motion_notify_event_t = xcb_input_device_key_press_event_t;

/// Opcode for xcb_input_device_focus_in.
pub const XCB_INPUT_DEVICE_FOCUS_IN: u8 = 6i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_focus_in_event_t {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub window: xcb_window_t,
    pub mode: u8,
    pub device_id: u8,
    pub pad0: [u8; 18],
}

impl Default for xcb_input_device_focus_in_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_device_focus_out.
pub const XCB_INPUT_DEVICE_FOCUS_OUT: u8 = 7i32 as u8;

pub type xcb_input_device_focus_out_event_t = xcb_input_device_focus_in_event_t;

/// Opcode for xcb_input_proximity_in.
pub const XCB_INPUT_PROXIMITY_IN: u8 = 8i32 as u8;

pub type xcb_input_proximity_in_event_t = xcb_input_device_key_press_event_t;

/// Opcode for xcb_input_proximity_out.
pub const XCB_INPUT_PROXIMITY_OUT: u8 = 9i32 as u8;

pub type xcb_input_proximity_out_event_t = xcb_input_device_key_press_event_t;

pub type xcb_input_classes_reported_mask_t = u32;
pub const XCB_INPUT_CLASSES_REPORTED_MASK_OUT_OF_PROXIMITY: xcb_input_classes_reported_mask_t = 128;
pub const XCB_INPUT_CLASSES_REPORTED_MASK_DEVICE_MODE_ABSOLUTE: xcb_input_classes_reported_mask_t =
    64;
pub const XCB_INPUT_CLASSES_REPORTED_MASK_REPORTING_VALUATORS: xcb_input_classes_reported_mask_t =
    4;
pub const XCB_INPUT_CLASSES_REPORTED_MASK_REPORTING_BUTTONS: xcb_input_classes_reported_mask_t = 2;
pub const XCB_INPUT_CLASSES_REPORTED_MASK_REPORTING_KEYS: xcb_input_classes_reported_mask_t = 1;

/// Opcode for xcb_input_device_state_notify.
pub const XCB_INPUT_DEVICE_STATE_NOTIFY: u8 = 10i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_state_notify_event_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub num_keys: u8,
    pub num_buttons: u8,
    pub num_valuators: u8,
    pub classes_reported: u8,
    pub buttons: [u8; 4],
    pub keys: [u8; 4],
    pub valuators: [u32; 3],
}

impl Default for xcb_input_device_state_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_device_mapping_notify.
pub const XCB_INPUT_DEVICE_MAPPING_NOTIFY: u8 = 11i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_mapping_notify_event_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub request: u8,
    pub first_keycode: xcb_input_key_code_t,
    pub count: u8,
    pub pad0: u8,
    pub time: xcb_timestamp_t,
    pub pad1: [u8; 20],
}

impl Default for xcb_input_device_mapping_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_change_device_t = u32;
pub const XCB_INPUT_CHANGE_DEVICE_NEW_POINTER: xcb_input_change_device_t = 0;
pub const XCB_INPUT_CHANGE_DEVICE_NEW_KEYBOARD: xcb_input_change_device_t = 1;

/// Opcode for xcb_input_change_device_notify.
pub const XCB_INPUT_CHANGE_DEVICE_NOTIFY: u8 = 12i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_device_notify_event_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub request: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_change_device_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_device_key_state_notify.
pub const XCB_INPUT_DEVICE_KEY_STATE_NOTIFY: u8 = 13i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_key_state_notify_event_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub keys: [u8; 28],
}

impl Default for xcb_input_device_key_state_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_device_button_state_notify.
pub const XCB_INPUT_DEVICE_BUTTON_STATE_NOTIFY: u8 = 14i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_button_state_notify_event_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub buttons: [u8; 28],
}

impl Default for xcb_input_device_button_state_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_device_change_t = u32;
pub const XCB_INPUT_DEVICE_CHANGE_ADDED: xcb_input_device_change_t = 0;
pub const XCB_INPUT_DEVICE_CHANGE_REMOVED: xcb_input_device_change_t = 1;
pub const XCB_INPUT_DEVICE_CHANGE_ENABLED: xcb_input_device_change_t = 2;
pub const XCB_INPUT_DEVICE_CHANGE_DISABLED: xcb_input_device_change_t = 3;
pub const XCB_INPUT_DEVICE_CHANGE_UNRECOVERABLE: xcb_input_device_change_t = 4;
pub const XCB_INPUT_DEVICE_CHANGE_CONTROL_CHANGED: xcb_input_device_change_t = 5;

/// Opcode for xcb_input_device_presence_notify.
pub const XCB_INPUT_DEVICE_PRESENCE_NOTIFY: u8 = 15i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_presence_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub devchange: u8,
    pub device_id: u8,
    pub control: u16,
    pub pad1: [u8; 20],
}

impl Default for xcb_input_device_presence_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_device_property_notify.
pub const XCB_INPUT_DEVICE_PROPERTY_NOTIFY: u8 = 16i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_property_notify_event_t {
    pub response_type: u8,
    pub state: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub property: xcb_atom_t,
    pub pad0: [u8; 19],
    pub device_id: u8,
}

impl Default for xcb_input_device_property_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_change_reason_t = u32;
pub const XCB_INPUT_CHANGE_REASON_SLAVE_SWITCH: xcb_input_change_reason_t = 1;
pub const XCB_INPUT_CHANGE_REASON_DEVICE_CHANGE: xcb_input_change_reason_t = 2;

/// Opcode for xcb_input_device_changed.
pub const XCB_INPUT_DEVICE_CHANGED: u8 = 1i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_changed_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub num_classes: u16,
    pub sourceid: xcb_input_device_id_t,
    pub reason: u8,
    pub pad0: [u8; 11],
    pub full_sequence: u32,
}

impl Default for xcb_input_device_changed_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_key_event_flags_t = u32;
pub const XCB_INPUT_KEY_EVENT_FLAGS_KEY_REPEAT: xcb_input_key_event_flags_t = 65536;

/// Opcode for xcb_input_key_press.
pub const XCB_INPUT_KEY_PRESS: u8 = 2i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_key_press_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub detail: u32,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub full_sequence: u32,
    pub root_x: xcb_input_fp1616_t,
    pub root_y: xcb_input_fp1616_t,
    pub event_x: xcb_input_fp1616_t,
    pub event_y: xcb_input_fp1616_t,
    pub buttons_len: u16,
    pub valuators_len: u16,
    pub sourceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
    pub flags: u32,
    pub mods: xcb_input_modifier_info_t,
    pub group: xcb_input_group_info_t,
}

impl Default for xcb_input_key_press_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_key_release.
pub const XCB_INPUT_KEY_RELEASE: u8 = 3i32 as u8;

pub type xcb_input_key_release_event_t = xcb_input_key_press_event_t;

pub type xcb_input_pointer_event_flags_t = u32;
pub const XCB_INPUT_POINTER_EVENT_FLAGS_POINTER_EMULATED: xcb_input_pointer_event_flags_t = 65536;

/// Opcode for xcb_input_button_press.
pub const XCB_INPUT_BUTTON_PRESS: u8 = 4i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_button_press_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub detail: u32,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub full_sequence: u32,
    pub root_x: xcb_input_fp1616_t,
    pub root_y: xcb_input_fp1616_t,
    pub event_x: xcb_input_fp1616_t,
    pub event_y: xcb_input_fp1616_t,
    pub buttons_len: u16,
    pub valuators_len: u16,
    pub sourceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
    pub flags: u32,
    pub mods: xcb_input_modifier_info_t,
    pub group: xcb_input_group_info_t,
}

impl Default for xcb_input_button_press_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_button_release.
pub const XCB_INPUT_BUTTON_RELEASE: u8 = 5i32 as u8;

pub type xcb_input_button_release_event_t = xcb_input_button_press_event_t;

/// Opcode for xcb_input_motion.
pub const XCB_INPUT_MOTION: u8 = 6i32 as u8;

pub type xcb_input_motion_event_t = xcb_input_button_press_event_t;

pub type xcb_input_notify_mode_t = u32;
pub const XCB_INPUT_NOTIFY_MODE_NORMAL: xcb_input_notify_mode_t = 0;
pub const XCB_INPUT_NOTIFY_MODE_GRAB: xcb_input_notify_mode_t = 1;
pub const XCB_INPUT_NOTIFY_MODE_UNGRAB: xcb_input_notify_mode_t = 2;
pub const XCB_INPUT_NOTIFY_MODE_WHILE_GRABBED: xcb_input_notify_mode_t = 3;
pub const XCB_INPUT_NOTIFY_MODE_PASSIVE_GRAB: xcb_input_notify_mode_t = 4;
pub const XCB_INPUT_NOTIFY_MODE_PASSIVE_UNGRAB: xcb_input_notify_mode_t = 5;

pub type xcb_input_notify_detail_t = u32;
pub const XCB_INPUT_NOTIFY_DETAIL_ANCESTOR: xcb_input_notify_detail_t = 0;
pub const XCB_INPUT_NOTIFY_DETAIL_VIRTUAL: xcb_input_notify_detail_t = 1;
pub const XCB_INPUT_NOTIFY_DETAIL_INFERIOR: xcb_input_notify_detail_t = 2;
pub const XCB_INPUT_NOTIFY_DETAIL_NONLINEAR: xcb_input_notify_detail_t = 3;
pub const XCB_INPUT_NOTIFY_DETAIL_NONLINEAR_VIRTUAL: xcb_input_notify_detail_t = 4;
pub const XCB_INPUT_NOTIFY_DETAIL_POINTER: xcb_input_notify_detail_t = 5;
pub const XCB_INPUT_NOTIFY_DETAIL_POINTER_ROOT: xcb_input_notify_detail_t = 6;
pub const XCB_INPUT_NOTIFY_DETAIL_NONE: xcb_input_notify_detail_t = 7;

/// Opcode for xcb_input_enter.
pub const XCB_INPUT_ENTER: u8 = 7i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_enter_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub sourceid: xcb_input_device_id_t,
    pub mode: u8,
    pub detail: u8,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub full_sequence: u32,
    pub root_x: xcb_input_fp1616_t,
    pub root_y: xcb_input_fp1616_t,
    pub event_x: xcb_input_fp1616_t,
    pub event_y: xcb_input_fp1616_t,
    pub same_screen: u8,
    pub focus: u8,
    pub buttons_len: u16,
    pub mods: xcb_input_modifier_info_t,
    pub group: xcb_input_group_info_t,
}

impl Default for xcb_input_enter_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_leave.
pub const XCB_INPUT_LEAVE: u8 = 8i32 as u8;

pub type xcb_input_leave_event_t = xcb_input_enter_event_t;

/// Opcode for xcb_input_focus_in.
pub const XCB_INPUT_FOCUS_IN: u8 = 9i32 as u8;

pub type xcb_input_focus_in_event_t = xcb_input_enter_event_t;

/// Opcode for xcb_input_focus_out.
pub const XCB_INPUT_FOCUS_OUT: u8 = 10i32 as u8;

pub type xcb_input_focus_out_event_t = xcb_input_enter_event_t;

pub type xcb_input_hierarchy_mask_t = u32;
pub const XCB_INPUT_HIERARCHY_MASK_MASTER_ADDED: xcb_input_hierarchy_mask_t = 1;
pub const XCB_INPUT_HIERARCHY_MASK_MASTER_REMOVED: xcb_input_hierarchy_mask_t = 2;
pub const XCB_INPUT_HIERARCHY_MASK_SLAVE_ADDED: xcb_input_hierarchy_mask_t = 4;
pub const XCB_INPUT_HIERARCHY_MASK_SLAVE_REMOVED: xcb_input_hierarchy_mask_t = 8;
pub const XCB_INPUT_HIERARCHY_MASK_SLAVE_ATTACHED: xcb_input_hierarchy_mask_t = 16;
pub const XCB_INPUT_HIERARCHY_MASK_SLAVE_DETACHED: xcb_input_hierarchy_mask_t = 32;
pub const XCB_INPUT_HIERARCHY_MASK_DEVICE_ENABLED: xcb_input_hierarchy_mask_t = 64;
pub const XCB_INPUT_HIERARCHY_MASK_DEVICE_DISABLED: xcb_input_hierarchy_mask_t = 128;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_hierarchy_info_t {
    pub deviceid: xcb_input_device_id_t,
    pub attachment: xcb_input_device_id_t,
    pub type_: u8,
    pub enabled: u8,
    pub pad0: [u8; 2],
    pub flags: u32,
}

impl Default for xcb_input_hierarchy_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_hierarchy_info_iterator_t {
    pub data: *mut xcb_input_hierarchy_info_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_hierarchy_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_hierarchy.
pub const XCB_INPUT_HIERARCHY: u8 = 11i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_hierarchy_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub flags: u32,
    pub num_infos: u16,
    pub pad0: [u8; 10],
    pub full_sequence: u32,
}

impl Default for xcb_input_hierarchy_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_property_flag_t = u32;
pub const XCB_INPUT_PROPERTY_FLAG_DELETED: xcb_input_property_flag_t = 0;
pub const XCB_INPUT_PROPERTY_FLAG_CREATED: xcb_input_property_flag_t = 1;
pub const XCB_INPUT_PROPERTY_FLAG_MODIFIED: xcb_input_property_flag_t = 2;

/// Opcode for xcb_input_property.
pub const XCB_INPUT_PROPERTY: u8 = 12i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_property_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub property: xcb_atom_t,
    pub what: u8,
    pub pad0: [u8; 11],
    pub full_sequence: u32,
}

impl Default for xcb_input_property_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_raw_key_press.
pub const XCB_INPUT_RAW_KEY_PRESS: u8 = 13i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_raw_key_press_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub detail: u32,
    pub sourceid: xcb_input_device_id_t,
    pub valuators_len: u16,
    pub flags: u32,
    pub pad0: [u8; 4],
    pub full_sequence: u32,
}

impl Default for xcb_input_raw_key_press_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_raw_key_release.
pub const XCB_INPUT_RAW_KEY_RELEASE: u8 = 14i32 as u8;

pub type xcb_input_raw_key_release_event_t = xcb_input_raw_key_press_event_t;

/// Opcode for xcb_input_raw_button_press.
pub const XCB_INPUT_RAW_BUTTON_PRESS: u8 = 15i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_raw_button_press_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub detail: u32,
    pub sourceid: xcb_input_device_id_t,
    pub valuators_len: u16,
    pub flags: u32,
    pub pad0: [u8; 4],
    pub full_sequence: u32,
}

impl Default for xcb_input_raw_button_press_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_raw_button_release.
pub const XCB_INPUT_RAW_BUTTON_RELEASE: u8 = 16i32 as u8;

pub type xcb_input_raw_button_release_event_t = xcb_input_raw_button_press_event_t;

/// Opcode for xcb_input_raw_motion.
pub const XCB_INPUT_RAW_MOTION: u8 = 17i32 as u8;

pub type xcb_input_raw_motion_event_t = xcb_input_raw_button_press_event_t;

pub type xcb_input_touch_event_flags_t = u32;
pub const XCB_INPUT_TOUCH_EVENT_FLAGS_TOUCH_PENDING_END: xcb_input_touch_event_flags_t = 65536;
pub const XCB_INPUT_TOUCH_EVENT_FLAGS_TOUCH_EMULATING_POINTER: xcb_input_touch_event_flags_t =
    131072;

/// Opcode for xcb_input_touch_begin.
pub const XCB_INPUT_TOUCH_BEGIN: u8 = 18i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_touch_begin_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub detail: u32,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub full_sequence: u32,
    pub root_x: xcb_input_fp1616_t,
    pub root_y: xcb_input_fp1616_t,
    pub event_x: xcb_input_fp1616_t,
    pub event_y: xcb_input_fp1616_t,
    pub buttons_len: u16,
    pub valuators_len: u16,
    pub sourceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
    pub flags: u32,
    pub mods: xcb_input_modifier_info_t,
    pub group: xcb_input_group_info_t,
}

impl Default for xcb_input_touch_begin_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_touch_update.
pub const XCB_INPUT_TOUCH_UPDATE: u8 = 19i32 as u8;

pub type xcb_input_touch_update_event_t = xcb_input_touch_begin_event_t;

/// Opcode for xcb_input_touch_end.
pub const XCB_INPUT_TOUCH_END: u8 = 20i32 as u8;

pub type xcb_input_touch_end_event_t = xcb_input_touch_begin_event_t;

pub type xcb_input_touch_ownership_flags_t = u32;
pub const XCB_INPUT_TOUCH_OWNERSHIP_FLAGS_NONE: xcb_input_touch_ownership_flags_t = 0;

/// Opcode for xcb_input_touch_ownership.
pub const XCB_INPUT_TOUCH_OWNERSHIP: u8 = 21i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_touch_ownership_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub touchid: u32,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub full_sequence: u32,
    pub sourceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
    pub flags: u32,
    pub pad1: [u8; 8],
}

impl Default for xcb_input_touch_ownership_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_raw_touch_begin.
pub const XCB_INPUT_RAW_TOUCH_BEGIN: u8 = 22i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_raw_touch_begin_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub detail: u32,
    pub sourceid: xcb_input_device_id_t,
    pub valuators_len: u16,
    pub flags: u32,
    pub pad0: [u8; 4],
    pub full_sequence: u32,
}

impl Default for xcb_input_raw_touch_begin_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_raw_touch_update.
pub const XCB_INPUT_RAW_TOUCH_UPDATE: u8 = 23i32 as u8;

pub type xcb_input_raw_touch_update_event_t = xcb_input_raw_touch_begin_event_t;

/// Opcode for xcb_input_raw_touch_end.
pub const XCB_INPUT_RAW_TOUCH_END: u8 = 24i32 as u8;

pub type xcb_input_raw_touch_end_event_t = xcb_input_raw_touch_begin_event_t;

pub type xcb_input_barrier_flags_t = u32;
pub const XCB_INPUT_BARRIER_FLAGS_POINTER_RELEASED: xcb_input_barrier_flags_t = 1;
pub const XCB_INPUT_BARRIER_FLAGS_DEVICE_IS_GRABBED: xcb_input_barrier_flags_t = 2;

/// Opcode for xcb_input_barrier_hit.
pub const XCB_INPUT_BARRIER_HIT: u8 = 25i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_barrier_hit_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub eventid: u32,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub barrier: xcb_xfixes_barrier_t,
    pub full_sequence: u32,
    pub dtime: u32,
    pub flags: u32,
    pub sourceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
    pub root_x: xcb_input_fp1616_t,
    pub root_y: xcb_input_fp1616_t,
    pub dx: xcb_input_fp3232_t,
    pub dy: xcb_input_fp3232_t,
}

impl Default for xcb_input_barrier_hit_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_barrier_leave.
pub const XCB_INPUT_BARRIER_LEAVE: u8 = 26i32 as u8;

pub type xcb_input_barrier_leave_event_t = xcb_input_barrier_hit_event_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub union xcb_input_event_for_send_t {
    pub device_valuator: xcb_input_device_valuator_event_t,
    pub device_key_press: xcb_input_device_key_press_event_t,
    pub device_key_release: xcb_input_device_key_release_event_t,
    pub device_button_press: xcb_input_device_button_press_event_t,
    pub device_button_release: xcb_input_device_button_release_event_t,
    pub device_motion_notify: xcb_input_device_motion_notify_event_t,
    pub device_focus_in: xcb_input_device_focus_in_event_t,
    pub device_focus_out: xcb_input_device_focus_out_event_t,
    pub proximity_in: xcb_input_proximity_in_event_t,
    pub proximity_out: xcb_input_proximity_out_event_t,
    pub device_state_notify: xcb_input_device_state_notify_event_t,
    pub device_mapping_notify: xcb_input_device_mapping_notify_event_t,
    pub change_device_notify: xcb_input_change_device_notify_event_t,
    pub device_key_state_notify: xcb_input_device_key_state_notify_event_t,
    pub device_button_state_notify: xcb_input_device_button_state_notify_event_t,
    pub device_presence_notify: xcb_input_device_presence_notify_event_t,
    pub event_header: xcb_raw_generic_event_t,
}

impl Default for xcb_input_event_for_send_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_event_for_send_iterator_t {
    pub data: *mut xcb_input_event_for_send_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_input_event_for_send_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_send_extension_event.
pub const XCB_INPUT_SEND_EXTENSION_EVENT: u8 = 31i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_send_extension_event_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub destination: xcb_window_t,
    pub device_id: u8,
    pub propagate: u8,
    pub num_classes: u16,
    pub num_events: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_send_extension_event_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_device.
pub const XCB_INPUT_DEVICE: u8 = 0i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_input_device_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_event.
pub const XCB_INPUT_EVENT: u8 = 1i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_event_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_input_event_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_mode.
pub const XCB_INPUT_MODE: u8 = 2i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_mode_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_input_mode_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_device_busy.
pub const XCB_INPUT_DEVICE_BUSY: u8 = 3i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_busy_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_input_device_busy_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_input_class.
pub const XCB_INPUT_CLASS: u8 = 4i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_class_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_input_class_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_xinput")]
pub(crate) struct XcbXinputXinput {
    xcb_input_id: LazySymbol<*mut xcb_extension_t>,
    xcb_input_event_class_next: LazySymbol<unsafe fn(i: *mut xcb_input_event_class_iterator_t)>,
    xcb_input_event_class_end:
        LazySymbol<unsafe fn(i: xcb_input_event_class_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_key_code_next: LazySymbol<unsafe fn(i: *mut xcb_input_key_code_iterator_t)>,
    xcb_input_key_code_end:
        LazySymbol<unsafe fn(i: xcb_input_key_code_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_device_id_next: LazySymbol<unsafe fn(i: *mut xcb_input_device_id_iterator_t)>,
    xcb_input_device_id_end:
        LazySymbol<unsafe fn(i: xcb_input_device_id_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_fp1616_next: LazySymbol<unsafe fn(i: *mut xcb_input_fp1616_iterator_t)>,
    xcb_input_fp1616_end:
        LazySymbol<unsafe fn(i: xcb_input_fp1616_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_fp3232_next: LazySymbol<unsafe fn(i: *mut xcb_input_fp3232_iterator_t)>,
    xcb_input_fp3232_end:
        LazySymbol<unsafe fn(i: xcb_input_fp3232_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_get_extension_version_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_get_extension_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_input_get_extension_version_cookie_t,
    >,
    xcb_input_get_extension_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_input_get_extension_version_cookie_t,
    >,
    xcb_input_get_extension_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_get_extension_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_get_extension_version_reply_t,
    >,
    xcb_input_device_info_next: LazySymbol<unsafe fn(i: *mut xcb_input_device_info_iterator_t)>,
    xcb_input_device_info_end:
        LazySymbol<unsafe fn(i: xcb_input_device_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_key_info_next: LazySymbol<unsafe fn(i: *mut xcb_input_key_info_iterator_t)>,
    xcb_input_key_info_end:
        LazySymbol<unsafe fn(i: xcb_input_key_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_button_info_next: LazySymbol<unsafe fn(i: *mut xcb_input_button_info_iterator_t)>,
    xcb_input_button_info_end:
        LazySymbol<unsafe fn(i: xcb_input_button_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_axis_info_next: LazySymbol<unsafe fn(i: *mut xcb_input_axis_info_iterator_t)>,
    xcb_input_axis_info_end:
        LazySymbol<unsafe fn(i: xcb_input_axis_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_valuator_info_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_valuator_info_axes:
        LazySymbol<unsafe fn(r: *const xcb_input_valuator_info_t) -> *mut xcb_input_axis_info_t>,
    xcb_input_valuator_info_axes_length:
        LazySymbol<unsafe fn(r: *const xcb_input_valuator_info_t) -> c_int>,
    xcb_input_valuator_info_axes_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_valuator_info_t) -> xcb_input_axis_info_iterator_t,
    >,
    xcb_input_valuator_info_next: LazySymbol<unsafe fn(i: *mut xcb_input_valuator_info_iterator_t)>,
    xcb_input_valuator_info_end:
        LazySymbol<unsafe fn(i: xcb_input_valuator_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_input_info_info_valuator_axes:
        LazySymbol<unsafe fn(s: *const xcb_input_input_info_info_t) -> *mut xcb_input_axis_info_t>,
    xcb_input_input_info_info_valuator_axes_length: LazySymbol<
        unsafe fn(r: *const xcb_input_input_info_t, s: *const xcb_input_input_info_info_t) -> c_int,
    >,
    xcb_input_input_info_info_valuator_axes_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_input_info_t,
            s: *const xcb_input_input_info_info_t,
        ) -> xcb_input_axis_info_iterator_t,
    >,
    xcb_input_input_info_info_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            class_id: u8,
            _aux: *const xcb_input_input_info_info_t,
        ) -> c_int,
    >,
    xcb_input_input_info_info_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            class_id: u8,
            _aux: *mut xcb_input_input_info_info_t,
        ) -> c_int,
    >,
    xcb_input_input_info_info_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, class_id: u8) -> c_int>,
    xcb_input_input_info_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_input_info_info:
        LazySymbol<unsafe fn(r: *const xcb_input_input_info_t) -> *mut c_void>,
    xcb_input_input_info_next: LazySymbol<unsafe fn(i: *mut xcb_input_input_info_iterator_t)>,
    xcb_input_input_info_end:
        LazySymbol<unsafe fn(i: xcb_input_input_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_device_name_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_device_name_string:
        LazySymbol<unsafe fn(r: *const xcb_input_device_name_t) -> *mut c_char>,
    xcb_input_device_name_string_length:
        LazySymbol<unsafe fn(r: *const xcb_input_device_name_t) -> c_int>,
    xcb_input_device_name_string_end:
        LazySymbol<unsafe fn(r: *const xcb_input_device_name_t) -> xcb_generic_iterator_t>,
    xcb_input_device_name_next: LazySymbol<unsafe fn(i: *mut xcb_input_device_name_iterator_t)>,
    xcb_input_device_name_end:
        LazySymbol<unsafe fn(i: xcb_input_device_name_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_list_input_devices_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_list_input_devices:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_input_list_input_devices_cookie_t>,
    xcb_input_list_input_devices_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_input_list_input_devices_cookie_t>,
    xcb_input_list_input_devices_devices: LazySymbol<
        unsafe fn(r: *const xcb_input_list_input_devices_reply_t) -> *mut xcb_input_device_info_t,
    >,
    xcb_input_list_input_devices_devices_length:
        LazySymbol<unsafe fn(r: *const xcb_input_list_input_devices_reply_t) -> c_int>,
    xcb_input_list_input_devices_devices_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_list_input_devices_reply_t,
        ) -> xcb_input_device_info_iterator_t,
    >,
    xcb_input_list_input_devices_infos_length:
        LazySymbol<unsafe fn(r: *const xcb_input_list_input_devices_reply_t) -> c_int>,
    xcb_input_list_input_devices_infos_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_list_input_devices_reply_t,
        ) -> xcb_input_input_info_iterator_t,
    >,
    xcb_input_list_input_devices_names_length:
        LazySymbol<unsafe fn(r: *const xcb_input_list_input_devices_reply_t) -> c_int>,
    xcb_input_list_input_devices_names_iterator:
        LazySymbol<unsafe fn(r: *const xcb_input_list_input_devices_reply_t) -> xcb_str_iterator_t>,
    xcb_input_list_input_devices_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_list_input_devices_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_list_input_devices_reply_t,
    >,
    xcb_input_event_type_base_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_event_type_base_iterator_t)>,
    xcb_input_event_type_base_end:
        LazySymbol<unsafe fn(i: xcb_input_event_type_base_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_input_class_info_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_input_class_info_iterator_t)>,
    xcb_input_input_class_info_end:
        LazySymbol<unsafe fn(i: xcb_input_input_class_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_open_device_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_open_device: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, device_id: u8) -> xcb_input_open_device_cookie_t,
    >,
    xcb_input_open_device_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, device_id: u8) -> xcb_input_open_device_cookie_t,
    >,
    xcb_input_open_device_class_info: LazySymbol<
        unsafe fn(r: *const xcb_input_open_device_reply_t) -> *mut xcb_input_input_class_info_t,
    >,
    xcb_input_open_device_class_info_length:
        LazySymbol<unsafe fn(r: *const xcb_input_open_device_reply_t) -> c_int>,
    xcb_input_open_device_class_info_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_open_device_reply_t) -> xcb_input_input_class_info_iterator_t,
    >,
    xcb_input_open_device_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_open_device_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_open_device_reply_t,
    >,
    xcb_input_close_device_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, device_id: u8) -> xcb_void_cookie_t>,
    xcb_input_close_device:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, device_id: u8) -> xcb_void_cookie_t>,
    xcb_input_set_device_mode: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            mode: u8,
        ) -> xcb_input_set_device_mode_cookie_t,
    >,
    xcb_input_set_device_mode_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            mode: u8,
        ) -> xcb_input_set_device_mode_cookie_t,
    >,
    xcb_input_set_device_mode_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_set_device_mode_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_set_device_mode_reply_t,
    >,
    xcb_input_select_extension_event_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_select_extension_event_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            num_classes: u16,
            classes: *const xcb_input_event_class_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_select_extension_event: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            num_classes: u16,
            classes: *const xcb_input_event_class_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_select_extension_event_classes: LazySymbol<
        unsafe fn(
            r: *const xcb_input_select_extension_event_request_t,
        ) -> *mut xcb_input_event_class_t,
    >,
    xcb_input_select_extension_event_classes_length:
        LazySymbol<unsafe fn(r: *const xcb_input_select_extension_event_request_t) -> c_int>,
    xcb_input_select_extension_event_classes_end: LazySymbol<
        unsafe fn(r: *const xcb_input_select_extension_event_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_get_selected_extension_events_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_get_selected_extension_events: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_input_get_selected_extension_events_cookie_t,
    >,
    xcb_input_get_selected_extension_events_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_input_get_selected_extension_events_cookie_t,
    >,
    xcb_input_get_selected_extension_events_this_classes: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_selected_extension_events_reply_t,
        ) -> *mut xcb_input_event_class_t,
    >,
    xcb_input_get_selected_extension_events_this_classes_length:
        LazySymbol<unsafe fn(r: *const xcb_input_get_selected_extension_events_reply_t) -> c_int>,
    xcb_input_get_selected_extension_events_this_classes_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_selected_extension_events_reply_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_get_selected_extension_events_all_classes: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_selected_extension_events_reply_t,
        ) -> *mut xcb_input_event_class_t,
    >,
    xcb_input_get_selected_extension_events_all_classes_length:
        LazySymbol<unsafe fn(r: *const xcb_input_get_selected_extension_events_reply_t) -> c_int>,
    xcb_input_get_selected_extension_events_all_classes_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_selected_extension_events_reply_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_get_selected_extension_events_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_get_selected_extension_events_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_get_selected_extension_events_reply_t,
    >,
    xcb_input_change_device_dont_propagate_list_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_change_device_dont_propagate_list_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            num_classes: u16,
            mode: u8,
            classes: *const xcb_input_event_class_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_change_device_dont_propagate_list: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            num_classes: u16,
            mode: u8,
            classes: *const xcb_input_event_class_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_change_device_dont_propagate_list_classes: LazySymbol<
        unsafe fn(
            r: *const xcb_input_change_device_dont_propagate_list_request_t,
        ) -> *mut xcb_input_event_class_t,
    >,
    xcb_input_change_device_dont_propagate_list_classes_length: LazySymbol<
        unsafe fn(r: *const xcb_input_change_device_dont_propagate_list_request_t) -> c_int,
    >,
    xcb_input_change_device_dont_propagate_list_classes_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_change_device_dont_propagate_list_request_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_get_device_dont_propagate_list_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_get_device_dont_propagate_list: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_input_get_device_dont_propagate_list_cookie_t,
    >,
    xcb_input_get_device_dont_propagate_list_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_input_get_device_dont_propagate_list_cookie_t,
    >,
    xcb_input_get_device_dont_propagate_list_classes: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_device_dont_propagate_list_reply_t,
        ) -> *mut xcb_input_event_class_t,
    >,
    xcb_input_get_device_dont_propagate_list_classes_length:
        LazySymbol<unsafe fn(r: *const xcb_input_get_device_dont_propagate_list_reply_t) -> c_int>,
    xcb_input_get_device_dont_propagate_list_classes_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_device_dont_propagate_list_reply_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_get_device_dont_propagate_list_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_get_device_dont_propagate_list_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_get_device_dont_propagate_list_reply_t,
    >,
    xcb_input_device_time_coord_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, num_axes: u8) -> c_int>,
    xcb_input_device_time_coord_axisvalues:
        LazySymbol<unsafe fn(r: *const xcb_input_device_time_coord_t) -> *mut i32>,
    xcb_input_device_time_coord_axisvalues_length:
        LazySymbol<unsafe fn(r: *const xcb_input_device_time_coord_t, num_axes: u8) -> c_int>,
    xcb_input_device_time_coord_axisvalues_end: LazySymbol<
        unsafe fn(r: *const xcb_input_device_time_coord_t, num_axes: u8) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_time_coord_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_device_time_coord_iterator_t)>,
    xcb_input_device_time_coord_end:
        LazySymbol<unsafe fn(i: xcb_input_device_time_coord_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_get_device_motion_events_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_get_device_motion_events: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            start: xcb_timestamp_t,
            stop: xcb_timestamp_t,
            device_id: u8,
        ) -> xcb_input_get_device_motion_events_cookie_t,
    >,
    xcb_input_get_device_motion_events_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            start: xcb_timestamp_t,
            stop: xcb_timestamp_t,
            device_id: u8,
        ) -> xcb_input_get_device_motion_events_cookie_t,
    >,
    xcb_input_get_device_motion_events_events_length:
        LazySymbol<unsafe fn(r: *const xcb_input_get_device_motion_events_reply_t) -> c_int>,
    xcb_input_get_device_motion_events_events_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_device_motion_events_reply_t,
        ) -> xcb_input_device_time_coord_iterator_t,
    >,
    xcb_input_get_device_motion_events_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_get_device_motion_events_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_get_device_motion_events_reply_t,
    >,
    xcb_input_change_keyboard_device: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
        ) -> xcb_input_change_keyboard_device_cookie_t,
    >,
    xcb_input_change_keyboard_device_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
        ) -> xcb_input_change_keyboard_device_cookie_t,
    >,
    xcb_input_change_keyboard_device_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_change_keyboard_device_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_change_keyboard_device_reply_t,
    >,
    xcb_input_change_pointer_device: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            x_axis: u8,
            y_axis: u8,
            device_id: u8,
        ) -> xcb_input_change_pointer_device_cookie_t,
    >,
    xcb_input_change_pointer_device_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            x_axis: u8,
            y_axis: u8,
            device_id: u8,
        ) -> xcb_input_change_pointer_device_cookie_t,
    >,
    xcb_input_change_pointer_device_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_change_pointer_device_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_change_pointer_device_reply_t,
    >,
    xcb_input_grab_device_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_grab_device: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            grab_window: xcb_window_t,
            time: xcb_timestamp_t,
            num_classes: u16,
            this_device_mode: u8,
            other_device_mode: u8,
            owner_events: u8,
            device_id: u8,
            classes: *const xcb_input_event_class_t,
        ) -> xcb_input_grab_device_cookie_t,
    >,
    xcb_input_grab_device_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            grab_window: xcb_window_t,
            time: xcb_timestamp_t,
            num_classes: u16,
            this_device_mode: u8,
            other_device_mode: u8,
            owner_events: u8,
            device_id: u8,
            classes: *const xcb_input_event_class_t,
        ) -> xcb_input_grab_device_cookie_t,
    >,
    xcb_input_grab_device_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_grab_device_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_grab_device_reply_t,
    >,
    xcb_input_ungrab_device_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            time: xcb_timestamp_t,
            device_id: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_ungrab_device: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            time: xcb_timestamp_t,
            device_id: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_grab_device_key_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_grab_device_key_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            grab_window: xcb_window_t,
            num_classes: u16,
            modifiers: u16,
            modifier_device: u8,
            grabbed_device: u8,
            key: u8,
            this_device_mode: u8,
            other_device_mode: u8,
            owner_events: u8,
            classes: *const xcb_input_event_class_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_grab_device_key: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            grab_window: xcb_window_t,
            num_classes: u16,
            modifiers: u16,
            modifier_device: u8,
            grabbed_device: u8,
            key: u8,
            this_device_mode: u8,
            other_device_mode: u8,
            owner_events: u8,
            classes: *const xcb_input_event_class_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_grab_device_key_classes: LazySymbol<
        unsafe fn(r: *const xcb_input_grab_device_key_request_t) -> *mut xcb_input_event_class_t,
    >,
    xcb_input_grab_device_key_classes_length:
        LazySymbol<unsafe fn(r: *const xcb_input_grab_device_key_request_t) -> c_int>,
    xcb_input_grab_device_key_classes_end: LazySymbol<
        unsafe fn(r: *const xcb_input_grab_device_key_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_ungrab_device_key_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            grab_window: xcb_window_t,
            modifiers: u16,
            modifier_device: u8,
            key: u8,
            grabbed_device: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_ungrab_device_key: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            grab_window: xcb_window_t,
            modifiers: u16,
            modifier_device: u8,
            key: u8,
            grabbed_device: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_grab_device_button_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_grab_device_button_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            grab_window: xcb_window_t,
            grabbed_device: u8,
            modifier_device: u8,
            num_classes: u16,
            modifiers: u16,
            this_device_mode: u8,
            other_device_mode: u8,
            button: u8,
            owner_events: u8,
            classes: *const xcb_input_event_class_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_grab_device_button: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            grab_window: xcb_window_t,
            grabbed_device: u8,
            modifier_device: u8,
            num_classes: u16,
            modifiers: u16,
            this_device_mode: u8,
            other_device_mode: u8,
            button: u8,
            owner_events: u8,
            classes: *const xcb_input_event_class_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_grab_device_button_classes: LazySymbol<
        unsafe fn(r: *const xcb_input_grab_device_button_request_t) -> *mut xcb_input_event_class_t,
    >,
    xcb_input_grab_device_button_classes_length:
        LazySymbol<unsafe fn(r: *const xcb_input_grab_device_button_request_t) -> c_int>,
    xcb_input_grab_device_button_classes_end: LazySymbol<
        unsafe fn(r: *const xcb_input_grab_device_button_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_ungrab_device_button_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            grab_window: xcb_window_t,
            modifiers: u16,
            modifier_device: u8,
            button: u8,
            grabbed_device: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_ungrab_device_button: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            grab_window: xcb_window_t,
            modifiers: u16,
            modifier_device: u8,
            button: u8,
            grabbed_device: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_allow_device_events_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            time: xcb_timestamp_t,
            mode: u8,
            device_id: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_allow_device_events: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            time: xcb_timestamp_t,
            mode: u8,
            device_id: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_get_device_focus: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, device_id: u8) -> xcb_input_get_device_focus_cookie_t,
    >,
    xcb_input_get_device_focus_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, device_id: u8) -> xcb_input_get_device_focus_cookie_t,
    >,
    xcb_input_get_device_focus_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_get_device_focus_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_get_device_focus_reply_t,
    >,
    xcb_input_set_device_focus_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            focus: xcb_window_t,
            time: xcb_timestamp_t,
            revert_to: u8,
            device_id: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_set_device_focus: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            focus: xcb_window_t,
            time: xcb_timestamp_t,
            revert_to: u8,
            device_id: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_kbd_feedback_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_kbd_feedback_state_iterator_t)>,
    xcb_input_kbd_feedback_state_end:
        LazySymbol<unsafe fn(i: xcb_input_kbd_feedback_state_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_ptr_feedback_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_ptr_feedback_state_iterator_t)>,
    xcb_input_ptr_feedback_state_end:
        LazySymbol<unsafe fn(i: xcb_input_ptr_feedback_state_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_integer_feedback_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_integer_feedback_state_iterator_t)>,
    xcb_input_integer_feedback_state_end: LazySymbol<
        unsafe fn(i: xcb_input_integer_feedback_state_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_string_feedback_state_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_string_feedback_state_keysyms:
        LazySymbol<unsafe fn(r: *const xcb_input_string_feedback_state_t) -> *mut xcb_keysym_t>,
    xcb_input_string_feedback_state_keysyms_length:
        LazySymbol<unsafe fn(r: *const xcb_input_string_feedback_state_t) -> c_int>,
    xcb_input_string_feedback_state_keysyms_end: LazySymbol<
        unsafe fn(r: *const xcb_input_string_feedback_state_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_string_feedback_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_string_feedback_state_iterator_t)>,
    xcb_input_string_feedback_state_end: LazySymbol<
        unsafe fn(i: xcb_input_string_feedback_state_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_bell_feedback_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_bell_feedback_state_iterator_t)>,
    xcb_input_bell_feedback_state_end: LazySymbol<
        unsafe fn(i: xcb_input_bell_feedback_state_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_led_feedback_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_led_feedback_state_iterator_t)>,
    xcb_input_led_feedback_state_end:
        LazySymbol<unsafe fn(i: xcb_input_led_feedback_state_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_feedback_state_data_string_keysyms:
        LazySymbol<unsafe fn(s: *const xcb_input_feedback_state_data_t) -> *mut xcb_keysym_t>,
    xcb_input_feedback_state_data_string_keysyms_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_feedback_state_t,
            s: *const xcb_input_feedback_state_data_t,
        ) -> c_int,
    >,
    xcb_input_feedback_state_data_string_keysyms_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_feedback_state_t,
            s: *const xcb_input_feedback_state_data_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_feedback_state_data_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            class_id: u8,
            _aux: *const xcb_input_feedback_state_data_t,
        ) -> c_int,
    >,
    xcb_input_feedback_state_data_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            class_id: u8,
            _aux: *mut xcb_input_feedback_state_data_t,
        ) -> c_int,
    >,
    xcb_input_feedback_state_data_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, class_id: u8) -> c_int>,
    xcb_input_feedback_state_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_feedback_state_data:
        LazySymbol<unsafe fn(r: *const xcb_input_feedback_state_t) -> *mut c_void>,
    xcb_input_feedback_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_feedback_state_iterator_t)>,
    xcb_input_feedback_state_end:
        LazySymbol<unsafe fn(i: xcb_input_feedback_state_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_get_feedback_control_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_get_feedback_control: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
        ) -> xcb_input_get_feedback_control_cookie_t,
    >,
    xcb_input_get_feedback_control_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
        ) -> xcb_input_get_feedback_control_cookie_t,
    >,
    xcb_input_get_feedback_control_feedbacks_length:
        LazySymbol<unsafe fn(r: *const xcb_input_get_feedback_control_reply_t) -> c_int>,
    xcb_input_get_feedback_control_feedbacks_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_feedback_control_reply_t,
        ) -> xcb_input_feedback_state_iterator_t,
    >,
    xcb_input_get_feedback_control_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_get_feedback_control_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_get_feedback_control_reply_t,
    >,
    xcb_input_kbd_feedback_ctl_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_kbd_feedback_ctl_iterator_t)>,
    xcb_input_kbd_feedback_ctl_end:
        LazySymbol<unsafe fn(i: xcb_input_kbd_feedback_ctl_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_ptr_feedback_ctl_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_ptr_feedback_ctl_iterator_t)>,
    xcb_input_ptr_feedback_ctl_end:
        LazySymbol<unsafe fn(i: xcb_input_ptr_feedback_ctl_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_integer_feedback_ctl_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_integer_feedback_ctl_iterator_t)>,
    xcb_input_integer_feedback_ctl_end: LazySymbol<
        unsafe fn(i: xcb_input_integer_feedback_ctl_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_string_feedback_ctl_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_string_feedback_ctl_keysyms:
        LazySymbol<unsafe fn(r: *const xcb_input_string_feedback_ctl_t) -> *mut xcb_keysym_t>,
    xcb_input_string_feedback_ctl_keysyms_length:
        LazySymbol<unsafe fn(r: *const xcb_input_string_feedback_ctl_t) -> c_int>,
    xcb_input_string_feedback_ctl_keysyms_end:
        LazySymbol<unsafe fn(r: *const xcb_input_string_feedback_ctl_t) -> xcb_generic_iterator_t>,
    xcb_input_string_feedback_ctl_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_string_feedback_ctl_iterator_t)>,
    xcb_input_string_feedback_ctl_end: LazySymbol<
        unsafe fn(i: xcb_input_string_feedback_ctl_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_bell_feedback_ctl_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_bell_feedback_ctl_iterator_t)>,
    xcb_input_bell_feedback_ctl_end:
        LazySymbol<unsafe fn(i: xcb_input_bell_feedback_ctl_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_led_feedback_ctl_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_led_feedback_ctl_iterator_t)>,
    xcb_input_led_feedback_ctl_end:
        LazySymbol<unsafe fn(i: xcb_input_led_feedback_ctl_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_feedback_ctl_data_string_keysyms:
        LazySymbol<unsafe fn(s: *const xcb_input_feedback_ctl_data_t) -> *mut xcb_keysym_t>,
    xcb_input_feedback_ctl_data_string_keysyms_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_feedback_ctl_t,
            s: *const xcb_input_feedback_ctl_data_t,
        ) -> c_int,
    >,
    xcb_input_feedback_ctl_data_string_keysyms_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_feedback_ctl_t,
            s: *const xcb_input_feedback_ctl_data_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_feedback_ctl_data_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            class_id: u8,
            _aux: *const xcb_input_feedback_ctl_data_t,
        ) -> c_int,
    >,
    xcb_input_feedback_ctl_data_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            class_id: u8,
            _aux: *mut xcb_input_feedback_ctl_data_t,
        ) -> c_int,
    >,
    xcb_input_feedback_ctl_data_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, class_id: u8) -> c_int>,
    xcb_input_feedback_ctl_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_feedback_ctl_data:
        LazySymbol<unsafe fn(r: *const xcb_input_feedback_ctl_t) -> *mut c_void>,
    xcb_input_feedback_ctl_next: LazySymbol<unsafe fn(i: *mut xcb_input_feedback_ctl_iterator_t)>,
    xcb_input_feedback_ctl_end:
        LazySymbol<unsafe fn(i: xcb_input_feedback_ctl_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_change_feedback_control_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_change_feedback_control_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            mask: u32,
            device_id: u8,
            feedback_id: u8,
            feedback: *mut xcb_input_feedback_ctl_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_change_feedback_control: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            mask: u32,
            device_id: u8,
            feedback_id: u8,
            feedback: *mut xcb_input_feedback_ctl_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_change_feedback_control_feedback: LazySymbol<
        unsafe fn(
            r: *const xcb_input_change_feedback_control_request_t,
        ) -> *mut xcb_input_feedback_ctl_t,
    >,
    xcb_input_get_device_key_mapping_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_get_device_key_mapping: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            first_keycode: xcb_input_key_code_t,
            count: u8,
        ) -> xcb_input_get_device_key_mapping_cookie_t,
    >,
    xcb_input_get_device_key_mapping_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            first_keycode: xcb_input_key_code_t,
            count: u8,
        ) -> xcb_input_get_device_key_mapping_cookie_t,
    >,
    xcb_input_get_device_key_mapping_keysyms: LazySymbol<
        unsafe fn(r: *const xcb_input_get_device_key_mapping_reply_t) -> *mut xcb_keysym_t,
    >,
    xcb_input_get_device_key_mapping_keysyms_length:
        LazySymbol<unsafe fn(r: *const xcb_input_get_device_key_mapping_reply_t) -> c_int>,
    xcb_input_get_device_key_mapping_keysyms_end: LazySymbol<
        unsafe fn(r: *const xcb_input_get_device_key_mapping_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_get_device_key_mapping_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_get_device_key_mapping_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_get_device_key_mapping_reply_t,
    >,
    xcb_input_change_device_key_mapping_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_change_device_key_mapping_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            first_keycode: xcb_input_key_code_t,
            keysyms_per_keycode: u8,
            keycode_count: u8,
            keysyms: *const xcb_keysym_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_change_device_key_mapping: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            first_keycode: xcb_input_key_code_t,
            keysyms_per_keycode: u8,
            keycode_count: u8,
            keysyms: *const xcb_keysym_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_change_device_key_mapping_keysyms: LazySymbol<
        unsafe fn(r: *const xcb_input_change_device_key_mapping_request_t) -> *mut xcb_keysym_t,
    >,
    xcb_input_change_device_key_mapping_keysyms_length:
        LazySymbol<unsafe fn(r: *const xcb_input_change_device_key_mapping_request_t) -> c_int>,
    xcb_input_change_device_key_mapping_keysyms_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_change_device_key_mapping_request_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_get_device_modifier_mapping_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_get_device_modifier_mapping: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
        ) -> xcb_input_get_device_modifier_mapping_cookie_t,
    >,
    xcb_input_get_device_modifier_mapping_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
        ) -> xcb_input_get_device_modifier_mapping_cookie_t,
    >,
    xcb_input_get_device_modifier_mapping_keymaps:
        LazySymbol<unsafe fn(r: *const xcb_input_get_device_modifier_mapping_reply_t) -> *mut u8>,
    xcb_input_get_device_modifier_mapping_keymaps_length:
        LazySymbol<unsafe fn(r: *const xcb_input_get_device_modifier_mapping_reply_t) -> c_int>,
    xcb_input_get_device_modifier_mapping_keymaps_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_device_modifier_mapping_reply_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_get_device_modifier_mapping_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_get_device_modifier_mapping_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_get_device_modifier_mapping_reply_t,
    >,
    xcb_input_set_device_modifier_mapping_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_set_device_modifier_mapping: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            keycodes_per_modifier: u8,
            keymaps: *const u8,
        ) -> xcb_input_set_device_modifier_mapping_cookie_t,
    >,
    xcb_input_set_device_modifier_mapping_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            keycodes_per_modifier: u8,
            keymaps: *const u8,
        ) -> xcb_input_set_device_modifier_mapping_cookie_t,
    >,
    xcb_input_set_device_modifier_mapping_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_set_device_modifier_mapping_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_set_device_modifier_mapping_reply_t,
    >,
    xcb_input_get_device_button_mapping_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_get_device_button_mapping: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
        ) -> xcb_input_get_device_button_mapping_cookie_t,
    >,
    xcb_input_get_device_button_mapping_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
        ) -> xcb_input_get_device_button_mapping_cookie_t,
    >,
    xcb_input_get_device_button_mapping_map:
        LazySymbol<unsafe fn(r: *const xcb_input_get_device_button_mapping_reply_t) -> *mut u8>,
    xcb_input_get_device_button_mapping_map_length:
        LazySymbol<unsafe fn(r: *const xcb_input_get_device_button_mapping_reply_t) -> c_int>,
    xcb_input_get_device_button_mapping_map_end: LazySymbol<
        unsafe fn(r: *const xcb_input_get_device_button_mapping_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_get_device_button_mapping_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_get_device_button_mapping_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_get_device_button_mapping_reply_t,
    >,
    xcb_input_set_device_button_mapping_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_set_device_button_mapping: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            map_size: u8,
            map: *const u8,
        ) -> xcb_input_set_device_button_mapping_cookie_t,
    >,
    xcb_input_set_device_button_mapping_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            map_size: u8,
            map: *const u8,
        ) -> xcb_input_set_device_button_mapping_cookie_t,
    >,
    xcb_input_set_device_button_mapping_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_set_device_button_mapping_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_set_device_button_mapping_reply_t,
    >,
    xcb_input_key_state_next: LazySymbol<unsafe fn(i: *mut xcb_input_key_state_iterator_t)>,
    xcb_input_key_state_end:
        LazySymbol<unsafe fn(i: xcb_input_key_state_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_button_state_next: LazySymbol<unsafe fn(i: *mut xcb_input_button_state_iterator_t)>,
    xcb_input_button_state_end:
        LazySymbol<unsafe fn(i: xcb_input_button_state_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_valuator_state_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_valuator_state_valuators:
        LazySymbol<unsafe fn(r: *const xcb_input_valuator_state_t) -> *mut i32>,
    xcb_input_valuator_state_valuators_length:
        LazySymbol<unsafe fn(r: *const xcb_input_valuator_state_t) -> c_int>,
    xcb_input_valuator_state_valuators_end:
        LazySymbol<unsafe fn(r: *const xcb_input_valuator_state_t) -> xcb_generic_iterator_t>,
    xcb_input_valuator_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_valuator_state_iterator_t)>,
    xcb_input_valuator_state_end:
        LazySymbol<unsafe fn(i: xcb_input_valuator_state_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_input_state_data_valuator_valuators:
        LazySymbol<unsafe fn(s: *const xcb_input_input_state_data_t) -> *mut i32>,
    xcb_input_input_state_data_valuator_valuators_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_input_state_t,
            s: *const xcb_input_input_state_data_t,
        ) -> c_int,
    >,
    xcb_input_input_state_data_valuator_valuators_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_input_state_t,
            s: *const xcb_input_input_state_data_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_input_state_data_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            class_id: u8,
            _aux: *const xcb_input_input_state_data_t,
        ) -> c_int,
    >,
    xcb_input_input_state_data_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            class_id: u8,
            _aux: *mut xcb_input_input_state_data_t,
        ) -> c_int,
    >,
    xcb_input_input_state_data_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, class_id: u8) -> c_int>,
    xcb_input_input_state_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_input_state_data:
        LazySymbol<unsafe fn(r: *const xcb_input_input_state_t) -> *mut c_void>,
    xcb_input_input_state_next: LazySymbol<unsafe fn(i: *mut xcb_input_input_state_iterator_t)>,
    xcb_input_input_state_end:
        LazySymbol<unsafe fn(i: xcb_input_input_state_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_query_device_state_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_query_device_state: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, device_id: u8) -> xcb_input_query_device_state_cookie_t,
    >,
    xcb_input_query_device_state_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, device_id: u8) -> xcb_input_query_device_state_cookie_t,
    >,
    xcb_input_query_device_state_classes_length:
        LazySymbol<unsafe fn(r: *const xcb_input_query_device_state_reply_t) -> c_int>,
    xcb_input_query_device_state_classes_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_query_device_state_reply_t,
        ) -> xcb_input_input_state_iterator_t,
    >,
    xcb_input_query_device_state_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_query_device_state_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_query_device_state_reply_t,
    >,
    xcb_input_device_bell_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            feedback_id: u8,
            feedback_class: u8,
            percent: i8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_device_bell: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            feedback_id: u8,
            feedback_class: u8,
            percent: i8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_set_device_valuators_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_set_device_valuators: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            first_valuator: u8,
            num_valuators: u8,
            valuators: *const i32,
        ) -> xcb_input_set_device_valuators_cookie_t,
    >,
    xcb_input_set_device_valuators_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            first_valuator: u8,
            num_valuators: u8,
            valuators: *const i32,
        ) -> xcb_input_set_device_valuators_cookie_t,
    >,
    xcb_input_set_device_valuators_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_set_device_valuators_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_set_device_valuators_reply_t,
    >,
    xcb_input_device_resolution_state_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_device_resolution_state_resolution_values:
        LazySymbol<unsafe fn(r: *const xcb_input_device_resolution_state_t) -> *mut u32>,
    xcb_input_device_resolution_state_resolution_values_length:
        LazySymbol<unsafe fn(r: *const xcb_input_device_resolution_state_t) -> c_int>,
    xcb_input_device_resolution_state_resolution_values_end: LazySymbol<
        unsafe fn(r: *const xcb_input_device_resolution_state_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_resolution_state_resolution_min:
        LazySymbol<unsafe fn(r: *const xcb_input_device_resolution_state_t) -> *mut u32>,
    xcb_input_device_resolution_state_resolution_min_length:
        LazySymbol<unsafe fn(r: *const xcb_input_device_resolution_state_t) -> c_int>,
    xcb_input_device_resolution_state_resolution_min_end: LazySymbol<
        unsafe fn(r: *const xcb_input_device_resolution_state_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_resolution_state_resolution_max:
        LazySymbol<unsafe fn(r: *const xcb_input_device_resolution_state_t) -> *mut u32>,
    xcb_input_device_resolution_state_resolution_max_length:
        LazySymbol<unsafe fn(r: *const xcb_input_device_resolution_state_t) -> c_int>,
    xcb_input_device_resolution_state_resolution_max_end: LazySymbol<
        unsafe fn(r: *const xcb_input_device_resolution_state_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_resolution_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_device_resolution_state_iterator_t)>,
    xcb_input_device_resolution_state_end: LazySymbol<
        unsafe fn(i: xcb_input_device_resolution_state_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_abs_calib_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_device_abs_calib_state_iterator_t)>,
    xcb_input_device_abs_calib_state_end: LazySymbol<
        unsafe fn(i: xcb_input_device_abs_calib_state_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_abs_area_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_device_abs_area_state_iterator_t)>,
    xcb_input_device_abs_area_state_end: LazySymbol<
        unsafe fn(i: xcb_input_device_abs_area_state_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_core_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_device_core_state_iterator_t)>,
    xcb_input_device_core_state_end:
        LazySymbol<unsafe fn(i: xcb_input_device_core_state_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_device_enable_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_device_enable_state_iterator_t)>,
    xcb_input_device_enable_state_end: LazySymbol<
        unsafe fn(i: xcb_input_device_enable_state_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_state_data_resolution_resolution_values:
        LazySymbol<unsafe fn(s: *const xcb_input_device_state_data_t) -> *mut u32>,
    xcb_input_device_state_data_resolution_resolution_values_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_state_t,
            s: *const xcb_input_device_state_data_t,
        ) -> c_int,
    >,
    xcb_input_device_state_data_resolution_resolution_values_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_state_t,
            s: *const xcb_input_device_state_data_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_state_data_resolution_resolution_min:
        LazySymbol<unsafe fn(s: *const xcb_input_device_state_data_t) -> *mut u32>,
    xcb_input_device_state_data_resolution_resolution_min_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_state_t,
            s: *const xcb_input_device_state_data_t,
        ) -> c_int,
    >,
    xcb_input_device_state_data_resolution_resolution_min_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_state_t,
            s: *const xcb_input_device_state_data_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_state_data_resolution_resolution_max:
        LazySymbol<unsafe fn(s: *const xcb_input_device_state_data_t) -> *mut u32>,
    xcb_input_device_state_data_resolution_resolution_max_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_state_t,
            s: *const xcb_input_device_state_data_t,
        ) -> c_int,
    >,
    xcb_input_device_state_data_resolution_resolution_max_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_state_t,
            s: *const xcb_input_device_state_data_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_state_data_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            control_id: u16,
            _aux: *const xcb_input_device_state_data_t,
        ) -> c_int,
    >,
    xcb_input_device_state_data_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            control_id: u16,
            _aux: *mut xcb_input_device_state_data_t,
        ) -> c_int,
    >,
    xcb_input_device_state_data_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, control_id: u16) -> c_int>,
    xcb_input_device_state_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_device_state_data:
        LazySymbol<unsafe fn(r: *const xcb_input_device_state_t) -> *mut c_void>,
    xcb_input_device_state_next: LazySymbol<unsafe fn(i: *mut xcb_input_device_state_iterator_t)>,
    xcb_input_device_state_end:
        LazySymbol<unsafe fn(i: xcb_input_device_state_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_get_device_control_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_get_device_control: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            control_id: u16,
            device_id: u8,
        ) -> xcb_input_get_device_control_cookie_t,
    >,
    xcb_input_get_device_control_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            control_id: u16,
            device_id: u8,
        ) -> xcb_input_get_device_control_cookie_t,
    >,
    xcb_input_get_device_control_control: LazySymbol<
        unsafe fn(r: *const xcb_input_get_device_control_reply_t) -> *mut xcb_input_device_state_t,
    >,
    xcb_input_get_device_control_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_get_device_control_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_get_device_control_reply_t,
    >,
    xcb_input_device_resolution_ctl_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_device_resolution_ctl_resolution_values:
        LazySymbol<unsafe fn(r: *const xcb_input_device_resolution_ctl_t) -> *mut u32>,
    xcb_input_device_resolution_ctl_resolution_values_length:
        LazySymbol<unsafe fn(r: *const xcb_input_device_resolution_ctl_t) -> c_int>,
    xcb_input_device_resolution_ctl_resolution_values_end: LazySymbol<
        unsafe fn(r: *const xcb_input_device_resolution_ctl_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_resolution_ctl_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_device_resolution_ctl_iterator_t)>,
    xcb_input_device_resolution_ctl_end: LazySymbol<
        unsafe fn(i: xcb_input_device_resolution_ctl_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_abs_calib_ctl_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_device_abs_calib_ctl_iterator_t)>,
    xcb_input_device_abs_calib_ctl_end: LazySymbol<
        unsafe fn(i: xcb_input_device_abs_calib_ctl_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_abs_area_ctrl_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_device_abs_area_ctrl_iterator_t)>,
    xcb_input_device_abs_area_ctrl_end: LazySymbol<
        unsafe fn(i: xcb_input_device_abs_area_ctrl_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_core_ctrl_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_device_core_ctrl_iterator_t)>,
    xcb_input_device_core_ctrl_end:
        LazySymbol<unsafe fn(i: xcb_input_device_core_ctrl_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_device_enable_ctrl_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_device_enable_ctrl_iterator_t)>,
    xcb_input_device_enable_ctrl_end:
        LazySymbol<unsafe fn(i: xcb_input_device_enable_ctrl_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_device_ctl_data_resolution_resolution_values:
        LazySymbol<unsafe fn(s: *const xcb_input_device_ctl_data_t) -> *mut u32>,
    xcb_input_device_ctl_data_resolution_resolution_values_length: LazySymbol<
        unsafe fn(r: *const xcb_input_device_ctl_t, s: *const xcb_input_device_ctl_data_t) -> c_int,
    >,
    xcb_input_device_ctl_data_resolution_resolution_values_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_ctl_t,
            s: *const xcb_input_device_ctl_data_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_ctl_data_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            control_id: u16,
            _aux: *const xcb_input_device_ctl_data_t,
        ) -> c_int,
    >,
    xcb_input_device_ctl_data_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            control_id: u16,
            _aux: *mut xcb_input_device_ctl_data_t,
        ) -> c_int,
    >,
    xcb_input_device_ctl_data_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, control_id: u16) -> c_int>,
    xcb_input_device_ctl_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_device_ctl_data:
        LazySymbol<unsafe fn(r: *const xcb_input_device_ctl_t) -> *mut c_void>,
    xcb_input_device_ctl_next: LazySymbol<unsafe fn(i: *mut xcb_input_device_ctl_iterator_t)>,
    xcb_input_device_ctl_end:
        LazySymbol<unsafe fn(i: xcb_input_device_ctl_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_change_device_control_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_change_device_control: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            control_id: u16,
            device_id: u8,
            control: *mut xcb_input_device_ctl_t,
        ) -> xcb_input_change_device_control_cookie_t,
    >,
    xcb_input_change_device_control_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            control_id: u16,
            device_id: u8,
            control: *mut xcb_input_device_ctl_t,
        ) -> xcb_input_change_device_control_cookie_t,
    >,
    xcb_input_change_device_control_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_change_device_control_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_change_device_control_reply_t,
    >,
    xcb_input_list_device_properties_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_list_device_properties: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
        ) -> xcb_input_list_device_properties_cookie_t,
    >,
    xcb_input_list_device_properties_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
        ) -> xcb_input_list_device_properties_cookie_t,
    >,
    xcb_input_list_device_properties_atoms: LazySymbol<
        unsafe fn(r: *const xcb_input_list_device_properties_reply_t) -> *mut xcb_atom_t,
    >,
    xcb_input_list_device_properties_atoms_length:
        LazySymbol<unsafe fn(r: *const xcb_input_list_device_properties_reply_t) -> c_int>,
    xcb_input_list_device_properties_atoms_end: LazySymbol<
        unsafe fn(r: *const xcb_input_list_device_properties_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_list_device_properties_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_list_device_properties_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_list_device_properties_reply_t,
    >,
    xcb_input_change_device_property_items_data_8:
        LazySymbol<unsafe fn(s: *const xcb_input_change_device_property_items_t) -> *mut u8>,
    xcb_input_change_device_property_items_data_8_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_change_device_property_request_t,
            s: *const xcb_input_change_device_property_items_t,
        ) -> c_int,
    >,
    xcb_input_change_device_property_items_data_8_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_change_device_property_request_t,
            s: *const xcb_input_change_device_property_items_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_change_device_property_items_data_16:
        LazySymbol<unsafe fn(s: *const xcb_input_change_device_property_items_t) -> *mut u16>,
    xcb_input_change_device_property_items_data_16_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_change_device_property_request_t,
            s: *const xcb_input_change_device_property_items_t,
        ) -> c_int,
    >,
    xcb_input_change_device_property_items_data_16_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_change_device_property_request_t,
            s: *const xcb_input_change_device_property_items_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_change_device_property_items_data_32:
        LazySymbol<unsafe fn(s: *const xcb_input_change_device_property_items_t) -> *mut u32>,
    xcb_input_change_device_property_items_data_32_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_change_device_property_request_t,
            s: *const xcb_input_change_device_property_items_t,
        ) -> c_int,
    >,
    xcb_input_change_device_property_items_data_32_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_change_device_property_request_t,
            s: *const xcb_input_change_device_property_items_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_change_device_property_items_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            num_items: u32,
            format: u8,
            _aux: *const xcb_input_change_device_property_items_t,
        ) -> c_int,
    >,
    xcb_input_change_device_property_items_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            num_items: u32,
            format: u8,
            _aux: *mut xcb_input_change_device_property_items_t,
        ) -> c_int,
    >,
    xcb_input_change_device_property_items_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, num_items: u32, format: u8) -> c_int>,
    xcb_input_change_device_property_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_change_device_property_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            device_id: u8,
            format: u8,
            mode: u8,
            num_items: u32,
            items: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_change_device_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            device_id: u8,
            format: u8,
            mode: u8,
            num_items: u32,
            items: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_change_device_property_aux_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            device_id: u8,
            format: u8,
            mode: u8,
            num_items: u32,
            items: *const xcb_input_change_device_property_items_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_change_device_property_aux: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            device_id: u8,
            format: u8,
            mode: u8,
            num_items: u32,
            items: *const xcb_input_change_device_property_items_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_change_device_property_items:
        LazySymbol<unsafe fn(r: *const xcb_input_change_device_property_request_t) -> *mut c_void>,
    xcb_input_delete_device_property_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            property: xcb_atom_t,
            device_id: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_delete_device_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            property: xcb_atom_t,
            device_id: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_get_device_property_items_data_8:
        LazySymbol<unsafe fn(s: *const xcb_input_get_device_property_items_t) -> *mut u8>,
    xcb_input_get_device_property_items_data_8_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_device_property_reply_t,
            s: *const xcb_input_get_device_property_items_t,
        ) -> c_int,
    >,
    xcb_input_get_device_property_items_data_8_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_device_property_reply_t,
            s: *const xcb_input_get_device_property_items_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_get_device_property_items_data_16:
        LazySymbol<unsafe fn(s: *const xcb_input_get_device_property_items_t) -> *mut u16>,
    xcb_input_get_device_property_items_data_16_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_device_property_reply_t,
            s: *const xcb_input_get_device_property_items_t,
        ) -> c_int,
    >,
    xcb_input_get_device_property_items_data_16_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_device_property_reply_t,
            s: *const xcb_input_get_device_property_items_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_get_device_property_items_data_32:
        LazySymbol<unsafe fn(s: *const xcb_input_get_device_property_items_t) -> *mut u32>,
    xcb_input_get_device_property_items_data_32_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_device_property_reply_t,
            s: *const xcb_input_get_device_property_items_t,
        ) -> c_int,
    >,
    xcb_input_get_device_property_items_data_32_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_device_property_reply_t,
            s: *const xcb_input_get_device_property_items_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_get_device_property_items_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            num_items: u32,
            format: u8,
            _aux: *const xcb_input_get_device_property_items_t,
        ) -> c_int,
    >,
    xcb_input_get_device_property_items_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            num_items: u32,
            format: u8,
            _aux: *mut xcb_input_get_device_property_items_t,
        ) -> c_int,
    >,
    xcb_input_get_device_property_items_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, num_items: u32, format: u8) -> c_int>,
    xcb_input_get_device_property_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_get_device_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            offset: u32,
            len: u32,
            device_id: u8,
            delete: u8,
        ) -> xcb_input_get_device_property_cookie_t,
    >,
    xcb_input_get_device_property_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            offset: u32,
            len: u32,
            device_id: u8,
            delete: u8,
        ) -> xcb_input_get_device_property_cookie_t,
    >,
    xcb_input_get_device_property_items:
        LazySymbol<unsafe fn(r: *const xcb_input_get_device_property_reply_t) -> *mut c_void>,
    xcb_input_get_device_property_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_get_device_property_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_get_device_property_reply_t,
    >,
    xcb_input_group_info_next: LazySymbol<unsafe fn(i: *mut xcb_input_group_info_iterator_t)>,
    xcb_input_group_info_end:
        LazySymbol<unsafe fn(i: xcb_input_group_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_modifier_info_next: LazySymbol<unsafe fn(i: *mut xcb_input_modifier_info_iterator_t)>,
    xcb_input_modifier_info_end:
        LazySymbol<unsafe fn(i: xcb_input_modifier_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_xi_query_pointer_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_query_pointer: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_input_xi_query_pointer_cookie_t,
    >,
    xcb_input_xi_query_pointer_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_input_xi_query_pointer_cookie_t,
    >,
    xcb_input_xi_query_pointer_buttons:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_query_pointer_reply_t) -> *mut u32>,
    xcb_input_xi_query_pointer_buttons_length:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_query_pointer_reply_t) -> c_int>,
    xcb_input_xi_query_pointer_buttons_end: LazySymbol<
        unsafe fn(r: *const xcb_input_xi_query_pointer_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_xi_query_pointer_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_xi_query_pointer_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_xi_query_pointer_reply_t,
    >,
    xcb_input_xi_warp_pointer_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src_win: xcb_window_t,
            dst_win: xcb_window_t,
            src_x: xcb_input_fp1616_t,
            src_y: xcb_input_fp1616_t,
            src_width: u16,
            src_height: u16,
            dst_x: xcb_input_fp1616_t,
            dst_y: xcb_input_fp1616_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_warp_pointer: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src_win: xcb_window_t,
            dst_win: xcb_window_t,
            src_x: xcb_input_fp1616_t,
            src_y: xcb_input_fp1616_t,
            src_width: u16,
            src_height: u16,
            dst_x: xcb_input_fp1616_t,
            dst_y: xcb_input_fp1616_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_change_cursor_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            cursor: xcb_cursor_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_change_cursor: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            cursor: xcb_cursor_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_add_master_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_add_master_name:
        LazySymbol<unsafe fn(r: *const xcb_input_add_master_t) -> *mut c_char>,
    xcb_input_add_master_name_length:
        LazySymbol<unsafe fn(r: *const xcb_input_add_master_t) -> c_int>,
    xcb_input_add_master_name_end:
        LazySymbol<unsafe fn(r: *const xcb_input_add_master_t) -> xcb_generic_iterator_t>,
    xcb_input_add_master_next: LazySymbol<unsafe fn(i: *mut xcb_input_add_master_iterator_t)>,
    xcb_input_add_master_end:
        LazySymbol<unsafe fn(i: xcb_input_add_master_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_remove_master_next: LazySymbol<unsafe fn(i: *mut xcb_input_remove_master_iterator_t)>,
    xcb_input_remove_master_end:
        LazySymbol<unsafe fn(i: xcb_input_remove_master_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_attach_slave_next: LazySymbol<unsafe fn(i: *mut xcb_input_attach_slave_iterator_t)>,
    xcb_input_attach_slave_end:
        LazySymbol<unsafe fn(i: xcb_input_attach_slave_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_detach_slave_next: LazySymbol<unsafe fn(i: *mut xcb_input_detach_slave_iterator_t)>,
    xcb_input_detach_slave_end:
        LazySymbol<unsafe fn(i: xcb_input_detach_slave_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_hierarchy_change_data_add_master_name:
        LazySymbol<unsafe fn(s: *const xcb_input_hierarchy_change_data_t) -> *mut c_char>,
    xcb_input_hierarchy_change_data_add_master_name_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_hierarchy_change_t,
            s: *const xcb_input_hierarchy_change_data_t,
        ) -> c_int,
    >,
    xcb_input_hierarchy_change_data_add_master_name_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_hierarchy_change_t,
            s: *const xcb_input_hierarchy_change_data_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_hierarchy_change_data_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            type_: u16,
            _aux: *const xcb_input_hierarchy_change_data_t,
        ) -> c_int,
    >,
    xcb_input_hierarchy_change_data_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            type_: u16,
            _aux: *mut xcb_input_hierarchy_change_data_t,
        ) -> c_int,
    >,
    xcb_input_hierarchy_change_data_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, type_: u16) -> c_int>,
    xcb_input_hierarchy_change_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_hierarchy_change_data:
        LazySymbol<unsafe fn(r: *const xcb_input_hierarchy_change_t) -> *mut c_void>,
    xcb_input_hierarchy_change_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_hierarchy_change_iterator_t)>,
    xcb_input_hierarchy_change_end:
        LazySymbol<unsafe fn(i: xcb_input_hierarchy_change_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_xi_change_hierarchy_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_change_hierarchy_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            num_changes: u8,
            changes: *const xcb_input_hierarchy_change_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_change_hierarchy: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            num_changes: u8,
            changes: *const xcb_input_hierarchy_change_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_change_hierarchy_changes_length:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_change_hierarchy_request_t) -> c_int>,
    xcb_input_xi_change_hierarchy_changes_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_change_hierarchy_request_t,
        ) -> xcb_input_hierarchy_change_iterator_t,
    >,
    xcb_input_xi_set_client_pointer_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_set_client_pointer: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_get_client_pointer: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_input_xi_get_client_pointer_cookie_t,
    >,
    xcb_input_xi_get_client_pointer_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_input_xi_get_client_pointer_cookie_t,
    >,
    xcb_input_xi_get_client_pointer_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_xi_get_client_pointer_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_xi_get_client_pointer_reply_t,
    >,
    xcb_input_event_mask_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_event_mask_mask: LazySymbol<unsafe fn(r: *const xcb_input_event_mask_t) -> *mut u32>,
    xcb_input_event_mask_mask_length:
        LazySymbol<unsafe fn(r: *const xcb_input_event_mask_t) -> c_int>,
    xcb_input_event_mask_mask_end:
        LazySymbol<unsafe fn(r: *const xcb_input_event_mask_t) -> xcb_generic_iterator_t>,
    xcb_input_event_mask_next: LazySymbol<unsafe fn(i: *mut xcb_input_event_mask_iterator_t)>,
    xcb_input_event_mask_end:
        LazySymbol<unsafe fn(i: xcb_input_event_mask_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_xi_select_events_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_select_events_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            num_mask: u16,
            masks: *const xcb_input_event_mask_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_select_events: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            num_mask: u16,
            masks: *const xcb_input_event_mask_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_select_events_masks_length:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_select_events_request_t) -> c_int>,
    xcb_input_xi_select_events_masks_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_select_events_request_t,
        ) -> xcb_input_event_mask_iterator_t,
    >,
    xcb_input_xi_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u16,
            minor_version: u16,
        ) -> xcb_input_xi_query_version_cookie_t,
    >,
    xcb_input_xi_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u16,
            minor_version: u16,
        ) -> xcb_input_xi_query_version_cookie_t,
    >,
    xcb_input_xi_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_xi_query_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_xi_query_version_reply_t,
    >,
    xcb_input_button_class_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_button_class_state:
        LazySymbol<unsafe fn(r: *const xcb_input_button_class_t) -> *mut u32>,
    xcb_input_button_class_state_length:
        LazySymbol<unsafe fn(r: *const xcb_input_button_class_t) -> c_int>,
    xcb_input_button_class_state_end:
        LazySymbol<unsafe fn(r: *const xcb_input_button_class_t) -> xcb_generic_iterator_t>,
    xcb_input_button_class_labels:
        LazySymbol<unsafe fn(r: *const xcb_input_button_class_t) -> *mut xcb_atom_t>,
    xcb_input_button_class_labels_length:
        LazySymbol<unsafe fn(r: *const xcb_input_button_class_t) -> c_int>,
    xcb_input_button_class_labels_end:
        LazySymbol<unsafe fn(r: *const xcb_input_button_class_t) -> xcb_generic_iterator_t>,
    xcb_input_button_class_next: LazySymbol<unsafe fn(i: *mut xcb_input_button_class_iterator_t)>,
    xcb_input_button_class_end:
        LazySymbol<unsafe fn(i: xcb_input_button_class_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_key_class_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_key_class_keys: LazySymbol<unsafe fn(r: *const xcb_input_key_class_t) -> *mut u32>,
    xcb_input_key_class_keys_length:
        LazySymbol<unsafe fn(r: *const xcb_input_key_class_t) -> c_int>,
    xcb_input_key_class_keys_end:
        LazySymbol<unsafe fn(r: *const xcb_input_key_class_t) -> xcb_generic_iterator_t>,
    xcb_input_key_class_next: LazySymbol<unsafe fn(i: *mut xcb_input_key_class_iterator_t)>,
    xcb_input_key_class_end:
        LazySymbol<unsafe fn(i: xcb_input_key_class_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_scroll_class_next: LazySymbol<unsafe fn(i: *mut xcb_input_scroll_class_iterator_t)>,
    xcb_input_scroll_class_end:
        LazySymbol<unsafe fn(i: xcb_input_scroll_class_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_touch_class_next: LazySymbol<unsafe fn(i: *mut xcb_input_touch_class_iterator_t)>,
    xcb_input_touch_class_end:
        LazySymbol<unsafe fn(i: xcb_input_touch_class_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_valuator_class_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_valuator_class_iterator_t)>,
    xcb_input_valuator_class_end:
        LazySymbol<unsafe fn(i: xcb_input_valuator_class_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_device_class_data_key_keys:
        LazySymbol<unsafe fn(s: *const xcb_input_device_class_data_t) -> *mut u32>,
    xcb_input_device_class_data_key_keys_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_class_t,
            s: *const xcb_input_device_class_data_t,
        ) -> c_int,
    >,
    xcb_input_device_class_data_key_keys_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_class_t,
            s: *const xcb_input_device_class_data_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_class_data_button_state:
        LazySymbol<unsafe fn(s: *const xcb_input_device_class_data_t) -> *mut u32>,
    xcb_input_device_class_data_button_state_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_class_t,
            s: *const xcb_input_device_class_data_t,
        ) -> c_int,
    >,
    xcb_input_device_class_data_button_state_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_class_t,
            s: *const xcb_input_device_class_data_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_class_data_button_labels:
        LazySymbol<unsafe fn(s: *const xcb_input_device_class_data_t) -> *mut xcb_atom_t>,
    xcb_input_device_class_data_button_labels_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_class_t,
            s: *const xcb_input_device_class_data_t,
        ) -> c_int,
    >,
    xcb_input_device_class_data_button_labels_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_class_t,
            s: *const xcb_input_device_class_data_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_class_data_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            type_: u16,
            _aux: *const xcb_input_device_class_data_t,
        ) -> c_int,
    >,
    xcb_input_device_class_data_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            type_: u16,
            _aux: *mut xcb_input_device_class_data_t,
        ) -> c_int,
    >,
    xcb_input_device_class_data_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, type_: u16) -> c_int>,
    xcb_input_device_class_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_device_class_data:
        LazySymbol<unsafe fn(r: *const xcb_input_device_class_t) -> *mut c_void>,
    xcb_input_device_class_next: LazySymbol<unsafe fn(i: *mut xcb_input_device_class_iterator_t)>,
    xcb_input_device_class_end:
        LazySymbol<unsafe fn(i: xcb_input_device_class_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_xi_device_info_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_device_info_name:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_device_info_t) -> *mut c_char>,
    xcb_input_xi_device_info_name_length:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_device_info_t) -> c_int>,
    xcb_input_xi_device_info_name_end:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_device_info_t) -> xcb_generic_iterator_t>,
    xcb_input_xi_device_info_classes_length:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_device_info_t) -> c_int>,
    xcb_input_xi_device_info_classes_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_xi_device_info_t) -> xcb_input_device_class_iterator_t,
    >,
    xcb_input_xi_device_info_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_xi_device_info_iterator_t)>,
    xcb_input_xi_device_info_end:
        LazySymbol<unsafe fn(i: xcb_input_xi_device_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_xi_query_device_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_query_device: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_input_xi_query_device_cookie_t,
    >,
    xcb_input_xi_query_device_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_input_xi_query_device_cookie_t,
    >,
    xcb_input_xi_query_device_infos_length:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_query_device_reply_t) -> c_int>,
    xcb_input_xi_query_device_infos_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_query_device_reply_t,
        ) -> xcb_input_xi_device_info_iterator_t,
    >,
    xcb_input_xi_query_device_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_xi_query_device_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_xi_query_device_reply_t,
    >,
    xcb_input_xi_set_focus_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            time: xcb_timestamp_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_set_focus: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            time: xcb_timestamp_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_get_focus: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_input_xi_get_focus_cookie_t,
    >,
    xcb_input_xi_get_focus_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_input_xi_get_focus_cookie_t,
    >,
    xcb_input_xi_get_focus_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_xi_get_focus_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_xi_get_focus_reply_t,
    >,
    xcb_input_xi_grab_device_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_grab_device: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            time: xcb_timestamp_t,
            cursor: xcb_cursor_t,
            deviceid: xcb_input_device_id_t,
            mode: u8,
            paired_device_mode: u8,
            owner_events: u8,
            mask_len: u16,
            mask: *const u32,
        ) -> xcb_input_xi_grab_device_cookie_t,
    >,
    xcb_input_xi_grab_device_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            time: xcb_timestamp_t,
            cursor: xcb_cursor_t,
            deviceid: xcb_input_device_id_t,
            mode: u8,
            paired_device_mode: u8,
            owner_events: u8,
            mask_len: u16,
            mask: *const u32,
        ) -> xcb_input_xi_grab_device_cookie_t,
    >,
    xcb_input_xi_grab_device_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_xi_grab_device_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_xi_grab_device_reply_t,
    >,
    xcb_input_xi_ungrab_device_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            time: xcb_timestamp_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_ungrab_device: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            time: xcb_timestamp_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_allow_events_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            time: xcb_timestamp_t,
            deviceid: xcb_input_device_id_t,
            event_mode: u8,
            touchid: u32,
            grab_window: xcb_window_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_allow_events: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            time: xcb_timestamp_t,
            deviceid: xcb_input_device_id_t,
            event_mode: u8,
            touchid: u32,
            grab_window: xcb_window_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_grab_modifier_info_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_grab_modifier_info_iterator_t)>,
    xcb_input_grab_modifier_info_end:
        LazySymbol<unsafe fn(i: xcb_input_grab_modifier_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_xi_passive_grab_device_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_passive_grab_device: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            time: xcb_timestamp_t,
            grab_window: xcb_window_t,
            cursor: xcb_cursor_t,
            detail: u32,
            deviceid: xcb_input_device_id_t,
            num_modifiers: u16,
            mask_len: u16,
            grab_type: u8,
            grab_mode: u8,
            paired_device_mode: u8,
            owner_events: u8,
            mask: *const u32,
            modifiers: *const u32,
        ) -> xcb_input_xi_passive_grab_device_cookie_t,
    >,
    xcb_input_xi_passive_grab_device_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            time: xcb_timestamp_t,
            grab_window: xcb_window_t,
            cursor: xcb_cursor_t,
            detail: u32,
            deviceid: xcb_input_device_id_t,
            num_modifiers: u16,
            mask_len: u16,
            grab_type: u8,
            grab_mode: u8,
            paired_device_mode: u8,
            owner_events: u8,
            mask: *const u32,
            modifiers: *const u32,
        ) -> xcb_input_xi_passive_grab_device_cookie_t,
    >,
    xcb_input_xi_passive_grab_device_modifiers: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_passive_grab_device_reply_t,
        ) -> *mut xcb_input_grab_modifier_info_t,
    >,
    xcb_input_xi_passive_grab_device_modifiers_length:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_passive_grab_device_reply_t) -> c_int>,
    xcb_input_xi_passive_grab_device_modifiers_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_passive_grab_device_reply_t,
        ) -> xcb_input_grab_modifier_info_iterator_t,
    >,
    xcb_input_xi_passive_grab_device_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_xi_passive_grab_device_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_xi_passive_grab_device_reply_t,
    >,
    xcb_input_xi_passive_ungrab_device_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_passive_ungrab_device_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            grab_window: xcb_window_t,
            detail: u32,
            deviceid: xcb_input_device_id_t,
            num_modifiers: u16,
            grab_type: u8,
            modifiers: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_passive_ungrab_device: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            grab_window: xcb_window_t,
            detail: u32,
            deviceid: xcb_input_device_id_t,
            num_modifiers: u16,
            grab_type: u8,
            modifiers: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_passive_ungrab_device_modifiers:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_passive_ungrab_device_request_t) -> *mut u32>,
    xcb_input_xi_passive_ungrab_device_modifiers_length:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_passive_ungrab_device_request_t) -> c_int>,
    xcb_input_xi_passive_ungrab_device_modifiers_end: LazySymbol<
        unsafe fn(r: *const xcb_input_xi_passive_ungrab_device_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_xi_list_properties_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_list_properties: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_input_xi_list_properties_cookie_t,
    >,
    xcb_input_xi_list_properties_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_input_xi_list_properties_cookie_t,
    >,
    xcb_input_xi_list_properties_properties:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_list_properties_reply_t) -> *mut xcb_atom_t>,
    xcb_input_xi_list_properties_properties_length:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_list_properties_reply_t) -> c_int>,
    xcb_input_xi_list_properties_properties_end: LazySymbol<
        unsafe fn(r: *const xcb_input_xi_list_properties_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_xi_list_properties_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_xi_list_properties_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_xi_list_properties_reply_t,
    >,
    xcb_input_xi_change_property_items_data_8:
        LazySymbol<unsafe fn(s: *const xcb_input_xi_change_property_items_t) -> *mut u8>,
    xcb_input_xi_change_property_items_data_8_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_change_property_request_t,
            s: *const xcb_input_xi_change_property_items_t,
        ) -> c_int,
    >,
    xcb_input_xi_change_property_items_data_8_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_change_property_request_t,
            s: *const xcb_input_xi_change_property_items_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_xi_change_property_items_data_16:
        LazySymbol<unsafe fn(s: *const xcb_input_xi_change_property_items_t) -> *mut u16>,
    xcb_input_xi_change_property_items_data_16_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_change_property_request_t,
            s: *const xcb_input_xi_change_property_items_t,
        ) -> c_int,
    >,
    xcb_input_xi_change_property_items_data_16_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_change_property_request_t,
            s: *const xcb_input_xi_change_property_items_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_xi_change_property_items_data_32:
        LazySymbol<unsafe fn(s: *const xcb_input_xi_change_property_items_t) -> *mut u32>,
    xcb_input_xi_change_property_items_data_32_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_change_property_request_t,
            s: *const xcb_input_xi_change_property_items_t,
        ) -> c_int,
    >,
    xcb_input_xi_change_property_items_data_32_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_change_property_request_t,
            s: *const xcb_input_xi_change_property_items_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_xi_change_property_items_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            num_items: u32,
            format: u8,
            _aux: *const xcb_input_xi_change_property_items_t,
        ) -> c_int,
    >,
    xcb_input_xi_change_property_items_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            num_items: u32,
            format: u8,
            _aux: *mut xcb_input_xi_change_property_items_t,
        ) -> c_int,
    >,
    xcb_input_xi_change_property_items_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, num_items: u32, format: u8) -> c_int>,
    xcb_input_xi_change_property_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_change_property_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
            mode: u8,
            format: u8,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            num_items: u32,
            items: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_change_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
            mode: u8,
            format: u8,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            num_items: u32,
            items: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_change_property_aux_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
            mode: u8,
            format: u8,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            num_items: u32,
            items: *const xcb_input_xi_change_property_items_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_change_property_aux: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
            mode: u8,
            format: u8,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            num_items: u32,
            items: *const xcb_input_xi_change_property_items_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_change_property_items:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_change_property_request_t) -> *mut c_void>,
    xcb_input_xi_delete_property_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
            property: xcb_atom_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_delete_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
            property: xcb_atom_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_get_property_items_data_8:
        LazySymbol<unsafe fn(s: *const xcb_input_xi_get_property_items_t) -> *mut u8>,
    xcb_input_xi_get_property_items_data_8_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_get_property_reply_t,
            s: *const xcb_input_xi_get_property_items_t,
        ) -> c_int,
    >,
    xcb_input_xi_get_property_items_data_8_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_get_property_reply_t,
            s: *const xcb_input_xi_get_property_items_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_xi_get_property_items_data_16:
        LazySymbol<unsafe fn(s: *const xcb_input_xi_get_property_items_t) -> *mut u16>,
    xcb_input_xi_get_property_items_data_16_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_get_property_reply_t,
            s: *const xcb_input_xi_get_property_items_t,
        ) -> c_int,
    >,
    xcb_input_xi_get_property_items_data_16_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_get_property_reply_t,
            s: *const xcb_input_xi_get_property_items_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_xi_get_property_items_data_32:
        LazySymbol<unsafe fn(s: *const xcb_input_xi_get_property_items_t) -> *mut u32>,
    xcb_input_xi_get_property_items_data_32_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_get_property_reply_t,
            s: *const xcb_input_xi_get_property_items_t,
        ) -> c_int,
    >,
    xcb_input_xi_get_property_items_data_32_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_get_property_reply_t,
            s: *const xcb_input_xi_get_property_items_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_xi_get_property_items_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            num_items: u32,
            format: u8,
            _aux: *const xcb_input_xi_get_property_items_t,
        ) -> c_int,
    >,
    xcb_input_xi_get_property_items_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            num_items: u32,
            format: u8,
            _aux: *mut xcb_input_xi_get_property_items_t,
        ) -> c_int,
    >,
    xcb_input_xi_get_property_items_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, num_items: u32, format: u8) -> c_int>,
    xcb_input_xi_get_property_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_get_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
            delete: u8,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            offset: u32,
            len: u32,
        ) -> xcb_input_xi_get_property_cookie_t,
    >,
    xcb_input_xi_get_property_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
            delete: u8,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            offset: u32,
            len: u32,
        ) -> xcb_input_xi_get_property_cookie_t,
    >,
    xcb_input_xi_get_property_items:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_get_property_reply_t) -> *mut c_void>,
    xcb_input_xi_get_property_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_xi_get_property_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_xi_get_property_reply_t,
    >,
    xcb_input_xi_get_selected_events_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_get_selected_events: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_input_xi_get_selected_events_cookie_t,
    >,
    xcb_input_xi_get_selected_events_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_input_xi_get_selected_events_cookie_t,
    >,
    xcb_input_xi_get_selected_events_masks_length:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_get_selected_events_reply_t) -> c_int>,
    xcb_input_xi_get_selected_events_masks_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_get_selected_events_reply_t,
        ) -> xcb_input_event_mask_iterator_t,
    >,
    xcb_input_xi_get_selected_events_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_xi_get_selected_events_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_input_xi_get_selected_events_reply_t,
    >,
    xcb_input_barrier_release_pointer_info_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_barrier_release_pointer_info_iterator_t)>,
    xcb_input_barrier_release_pointer_info_end: LazySymbol<
        unsafe fn(i: xcb_input_barrier_release_pointer_info_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_xi_barrier_release_pointer_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_barrier_release_pointer_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            num_barriers: u32,
            barriers: *const xcb_input_barrier_release_pointer_info_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_barrier_release_pointer: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            num_barriers: u32,
            barriers: *const xcb_input_barrier_release_pointer_info_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_barrier_release_pointer_barriers: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_barrier_release_pointer_request_t,
        ) -> *mut xcb_input_barrier_release_pointer_info_t,
    >,
    xcb_input_xi_barrier_release_pointer_barriers_length:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_barrier_release_pointer_request_t) -> c_int>,
    xcb_input_xi_barrier_release_pointer_barriers_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_barrier_release_pointer_request_t,
        ) -> xcb_input_barrier_release_pointer_info_iterator_t,
    >,
    xcb_input_device_changed_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_device_changed_classes_length:
        LazySymbol<unsafe fn(r: *const xcb_input_device_changed_event_t) -> c_int>,
    xcb_input_device_changed_classes_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_device_changed_event_t) -> xcb_input_device_class_iterator_t,
    >,
    xcb_input_key_press_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_key_press_button_mask:
        LazySymbol<unsafe fn(r: *const xcb_input_key_press_event_t) -> *mut u32>,
    xcb_input_key_press_button_mask_length:
        LazySymbol<unsafe fn(r: *const xcb_input_key_press_event_t) -> c_int>,
    xcb_input_key_press_button_mask_end:
        LazySymbol<unsafe fn(r: *const xcb_input_key_press_event_t) -> xcb_generic_iterator_t>,
    xcb_input_key_press_valuator_mask:
        LazySymbol<unsafe fn(r: *const xcb_input_key_press_event_t) -> *mut u32>,
    xcb_input_key_press_valuator_mask_length:
        LazySymbol<unsafe fn(r: *const xcb_input_key_press_event_t) -> c_int>,
    xcb_input_key_press_valuator_mask_end:
        LazySymbol<unsafe fn(r: *const xcb_input_key_press_event_t) -> xcb_generic_iterator_t>,
    xcb_input_key_press_axisvalues:
        LazySymbol<unsafe fn(r: *const xcb_input_key_press_event_t) -> *mut xcb_input_fp3232_t>,
    xcb_input_key_press_axisvalues_length:
        LazySymbol<unsafe fn(r: *const xcb_input_key_press_event_t) -> c_int>,
    xcb_input_key_press_axisvalues_iterator:
        LazySymbol<unsafe fn(r: *const xcb_input_key_press_event_t) -> xcb_input_fp3232_iterator_t>,
    xcb_input_key_release_sizeof: LazySymbol<unsafe fn(buffer: *const c_void) -> c_int>,
    xcb_input_button_press_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_button_press_button_mask:
        LazySymbol<unsafe fn(r: *const xcb_input_button_press_event_t) -> *mut u32>,
    xcb_input_button_press_button_mask_length:
        LazySymbol<unsafe fn(r: *const xcb_input_button_press_event_t) -> c_int>,
    xcb_input_button_press_button_mask_end:
        LazySymbol<unsafe fn(r: *const xcb_input_button_press_event_t) -> xcb_generic_iterator_t>,
    xcb_input_button_press_valuator_mask:
        LazySymbol<unsafe fn(r: *const xcb_input_button_press_event_t) -> *mut u32>,
    xcb_input_button_press_valuator_mask_length:
        LazySymbol<unsafe fn(r: *const xcb_input_button_press_event_t) -> c_int>,
    xcb_input_button_press_valuator_mask_end:
        LazySymbol<unsafe fn(r: *const xcb_input_button_press_event_t) -> xcb_generic_iterator_t>,
    xcb_input_button_press_axisvalues:
        LazySymbol<unsafe fn(r: *const xcb_input_button_press_event_t) -> *mut xcb_input_fp3232_t>,
    xcb_input_button_press_axisvalues_length:
        LazySymbol<unsafe fn(r: *const xcb_input_button_press_event_t) -> c_int>,
    xcb_input_button_press_axisvalues_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_button_press_event_t) -> xcb_input_fp3232_iterator_t,
    >,
    xcb_input_button_release_sizeof: LazySymbol<unsafe fn(buffer: *const c_void) -> c_int>,
    xcb_input_motion_sizeof: LazySymbol<unsafe fn(buffer: *const c_void) -> c_int>,
    xcb_input_enter_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_enter_buttons: LazySymbol<unsafe fn(r: *const xcb_input_enter_event_t) -> *mut u32>,
    xcb_input_enter_buttons_length:
        LazySymbol<unsafe fn(r: *const xcb_input_enter_event_t) -> c_int>,
    xcb_input_enter_buttons_end:
        LazySymbol<unsafe fn(r: *const xcb_input_enter_event_t) -> xcb_generic_iterator_t>,
    xcb_input_leave_sizeof: LazySymbol<unsafe fn(buffer: *const c_void) -> c_int>,
    xcb_input_focus_in_sizeof: LazySymbol<unsafe fn(buffer: *const c_void) -> c_int>,
    xcb_input_focus_out_sizeof: LazySymbol<unsafe fn(buffer: *const c_void) -> c_int>,
    xcb_input_hierarchy_info_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_hierarchy_info_iterator_t)>,
    xcb_input_hierarchy_info_end:
        LazySymbol<unsafe fn(i: xcb_input_hierarchy_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_hierarchy_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_hierarchy_infos: LazySymbol<
        unsafe fn(r: *const xcb_input_hierarchy_event_t) -> *mut xcb_input_hierarchy_info_t,
    >,
    xcb_input_hierarchy_infos_length:
        LazySymbol<unsafe fn(r: *const xcb_input_hierarchy_event_t) -> c_int>,
    xcb_input_hierarchy_infos_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_hierarchy_event_t) -> xcb_input_hierarchy_info_iterator_t,
    >,
    xcb_input_raw_key_press_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_raw_key_press_valuator_mask:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_key_press_event_t) -> *mut u32>,
    xcb_input_raw_key_press_valuator_mask_length:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_key_press_event_t) -> c_int>,
    xcb_input_raw_key_press_valuator_mask_end:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_key_press_event_t) -> xcb_generic_iterator_t>,
    xcb_input_raw_key_press_axisvalues:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_key_press_event_t) -> *mut xcb_input_fp3232_t>,
    xcb_input_raw_key_press_axisvalues_length:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_key_press_event_t) -> c_int>,
    xcb_input_raw_key_press_axisvalues_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_raw_key_press_event_t) -> xcb_input_fp3232_iterator_t,
    >,
    xcb_input_raw_key_press_axisvalues_raw:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_key_press_event_t) -> *mut xcb_input_fp3232_t>,
    xcb_input_raw_key_press_axisvalues_raw_length:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_key_press_event_t) -> c_int>,
    xcb_input_raw_key_press_axisvalues_raw_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_raw_key_press_event_t) -> xcb_input_fp3232_iterator_t,
    >,
    xcb_input_raw_key_release_sizeof: LazySymbol<unsafe fn(buffer: *const c_void) -> c_int>,
    xcb_input_raw_button_press_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_raw_button_press_valuator_mask:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_button_press_event_t) -> *mut u32>,
    xcb_input_raw_button_press_valuator_mask_length:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_button_press_event_t) -> c_int>,
    xcb_input_raw_button_press_valuator_mask_end: LazySymbol<
        unsafe fn(r: *const xcb_input_raw_button_press_event_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_raw_button_press_axisvalues: LazySymbol<
        unsafe fn(r: *const xcb_input_raw_button_press_event_t) -> *mut xcb_input_fp3232_t,
    >,
    xcb_input_raw_button_press_axisvalues_length:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_button_press_event_t) -> c_int>,
    xcb_input_raw_button_press_axisvalues_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_raw_button_press_event_t) -> xcb_input_fp3232_iterator_t,
    >,
    xcb_input_raw_button_press_axisvalues_raw: LazySymbol<
        unsafe fn(r: *const xcb_input_raw_button_press_event_t) -> *mut xcb_input_fp3232_t,
    >,
    xcb_input_raw_button_press_axisvalues_raw_length:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_button_press_event_t) -> c_int>,
    xcb_input_raw_button_press_axisvalues_raw_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_raw_button_press_event_t) -> xcb_input_fp3232_iterator_t,
    >,
    xcb_input_raw_button_release_sizeof: LazySymbol<unsafe fn(buffer: *const c_void) -> c_int>,
    xcb_input_raw_motion_sizeof: LazySymbol<unsafe fn(buffer: *const c_void) -> c_int>,
    xcb_input_touch_begin_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_touch_begin_button_mask:
        LazySymbol<unsafe fn(r: *const xcb_input_touch_begin_event_t) -> *mut u32>,
    xcb_input_touch_begin_button_mask_length:
        LazySymbol<unsafe fn(r: *const xcb_input_touch_begin_event_t) -> c_int>,
    xcb_input_touch_begin_button_mask_end:
        LazySymbol<unsafe fn(r: *const xcb_input_touch_begin_event_t) -> xcb_generic_iterator_t>,
    xcb_input_touch_begin_valuator_mask:
        LazySymbol<unsafe fn(r: *const xcb_input_touch_begin_event_t) -> *mut u32>,
    xcb_input_touch_begin_valuator_mask_length:
        LazySymbol<unsafe fn(r: *const xcb_input_touch_begin_event_t) -> c_int>,
    xcb_input_touch_begin_valuator_mask_end:
        LazySymbol<unsafe fn(r: *const xcb_input_touch_begin_event_t) -> xcb_generic_iterator_t>,
    xcb_input_touch_begin_axisvalues:
        LazySymbol<unsafe fn(r: *const xcb_input_touch_begin_event_t) -> *mut xcb_input_fp3232_t>,
    xcb_input_touch_begin_axisvalues_length:
        LazySymbol<unsafe fn(r: *const xcb_input_touch_begin_event_t) -> c_int>,
    xcb_input_touch_begin_axisvalues_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_touch_begin_event_t) -> xcb_input_fp3232_iterator_t,
    >,
    xcb_input_touch_update_sizeof: LazySymbol<unsafe fn(buffer: *const c_void) -> c_int>,
    xcb_input_touch_end_sizeof: LazySymbol<unsafe fn(buffer: *const c_void) -> c_int>,
    xcb_input_raw_touch_begin_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_raw_touch_begin_valuator_mask:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_touch_begin_event_t) -> *mut u32>,
    xcb_input_raw_touch_begin_valuator_mask_length:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_touch_begin_event_t) -> c_int>,
    xcb_input_raw_touch_begin_valuator_mask_end: LazySymbol<
        unsafe fn(r: *const xcb_input_raw_touch_begin_event_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_raw_touch_begin_axisvalues: LazySymbol<
        unsafe fn(r: *const xcb_input_raw_touch_begin_event_t) -> *mut xcb_input_fp3232_t,
    >,
    xcb_input_raw_touch_begin_axisvalues_length:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_touch_begin_event_t) -> c_int>,
    xcb_input_raw_touch_begin_axisvalues_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_raw_touch_begin_event_t) -> xcb_input_fp3232_iterator_t,
    >,
    xcb_input_raw_touch_begin_axisvalues_raw: LazySymbol<
        unsafe fn(r: *const xcb_input_raw_touch_begin_event_t) -> *mut xcb_input_fp3232_t,
    >,
    xcb_input_raw_touch_begin_axisvalues_raw_length:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_touch_begin_event_t) -> c_int>,
    xcb_input_raw_touch_begin_axisvalues_raw_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_raw_touch_begin_event_t) -> xcb_input_fp3232_iterator_t,
    >,
    xcb_input_raw_touch_update_sizeof: LazySymbol<unsafe fn(buffer: *const c_void) -> c_int>,
    xcb_input_raw_touch_end_sizeof: LazySymbol<unsafe fn(buffer: *const c_void) -> c_int>,
    xcb_input_event_for_send_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_event_for_send_iterator_t)>,
    xcb_input_event_for_send_end:
        LazySymbol<unsafe fn(i: xcb_input_event_for_send_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_send_extension_event_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_send_extension_event_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            destination: xcb_window_t,
            device_id: u8,
            propagate: u8,
            num_classes: u16,
            num_events: u8,
            events: *const xcb_input_event_for_send_t,
            classes: *const xcb_input_event_class_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_send_extension_event: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            destination: xcb_window_t,
            device_id: u8,
            propagate: u8,
            num_classes: u16,
            num_events: u8,
            events: *const xcb_input_event_for_send_t,
            classes: *const xcb_input_event_class_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_send_extension_event_events: LazySymbol<
        unsafe fn(
            r: *const xcb_input_send_extension_event_request_t,
        ) -> *mut xcb_input_event_for_send_t,
    >,
    xcb_input_send_extension_event_events_length:
        LazySymbol<unsafe fn(r: *const xcb_input_send_extension_event_request_t) -> c_int>,
    xcb_input_send_extension_event_events_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_send_extension_event_request_t,
        ) -> xcb_input_event_for_send_iterator_t,
    >,
    xcb_input_send_extension_event_classes: LazySymbol<
        unsafe fn(
            r: *const xcb_input_send_extension_event_request_t,
        ) -> *mut xcb_input_event_class_t,
    >,
    xcb_input_send_extension_event_classes_length:
        LazySymbol<unsafe fn(r: *const xcb_input_send_extension_event_request_t) -> c_int>,
    xcb_input_send_extension_event_classes_end: LazySymbol<
        unsafe fn(r: *const xcb_input_send_extension_event_request_t) -> xcb_generic_iterator_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self
            .xinput
            .$f
            .get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .xinput
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_xinput")]
impl XcbXinput {
    pub fn xcb_input_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_input_id) }
    }

    /// Returns `true` iff the symbol `xcb_input_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_id(&self) -> bool {
        has_sym!(self, xcb_input_id)
    }

    pub unsafe fn xcb_input_event_class_next(&self, i: *mut xcb_input_event_class_iterator_t) {
        sym!(self, xcb_input_event_class_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_event_class_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_event_class_next(&self) -> bool {
        has_sym!(self, xcb_input_event_class_next)
    }

    pub unsafe fn xcb_input_event_class_end(
        &self,
        i: xcb_input_event_class_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_event_class_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_event_class_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_event_class_end(&self) -> bool {
        has_sym!(self, xcb_input_event_class_end)
    }

    pub unsafe fn xcb_input_key_code_next(&self, i: *mut xcb_input_key_code_iterator_t) {
        sym!(self, xcb_input_key_code_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_key_code_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_code_next(&self) -> bool {
        has_sym!(self, xcb_input_key_code_next)
    }

    pub unsafe fn xcb_input_key_code_end(
        &self,
        i: xcb_input_key_code_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_key_code_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_key_code_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_code_end(&self) -> bool {
        has_sym!(self, xcb_input_key_code_end)
    }

    pub unsafe fn xcb_input_device_id_next(&self, i: *mut xcb_input_device_id_iterator_t) {
        sym!(self, xcb_input_device_id_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_id_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_id_next(&self) -> bool {
        has_sym!(self, xcb_input_device_id_next)
    }

    pub unsafe fn xcb_input_device_id_end(
        &self,
        i: xcb_input_device_id_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_id_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_id_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_id_end(&self) -> bool {
        has_sym!(self, xcb_input_device_id_end)
    }

    pub unsafe fn xcb_input_fp1616_next(&self, i: *mut xcb_input_fp1616_iterator_t) {
        sym!(self, xcb_input_fp1616_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_fp1616_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_fp1616_next(&self) -> bool {
        has_sym!(self, xcb_input_fp1616_next)
    }

    pub unsafe fn xcb_input_fp1616_end(
        &self,
        i: xcb_input_fp1616_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_fp1616_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_fp1616_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_fp1616_end(&self) -> bool {
        has_sym!(self, xcb_input_fp1616_end)
    }

    pub unsafe fn xcb_input_fp3232_next(&self, i: *mut xcb_input_fp3232_iterator_t) {
        sym!(self, xcb_input_fp3232_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_fp3232_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_fp3232_next(&self) -> bool {
        has_sym!(self, xcb_input_fp3232_next)
    }

    pub unsafe fn xcb_input_fp3232_end(
        &self,
        i: xcb_input_fp3232_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_fp3232_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_fp3232_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_fp3232_end(&self) -> bool {
        has_sym!(self, xcb_input_fp3232_end)
    }

    pub unsafe fn xcb_input_get_extension_version_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_get_extension_version_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_get_extension_version_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_extension_version_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_get_extension_version_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_get_extension_version(
        &self,
        c: *mut xcb_connection_t,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_input_get_extension_version_cookie_t {
        sym!(self, xcb_input_get_extension_version)(c, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_input_get_extension_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_extension_version(&self) -> bool {
        has_sym!(self, xcb_input_get_extension_version)
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
    pub unsafe fn xcb_input_get_extension_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_input_get_extension_version_cookie_t {
        sym!(self, xcb_input_get_extension_version_unchecked)(c, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_input_get_extension_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_extension_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_get_extension_version_unchecked)
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
     * xcb_input_get_extension_version_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_get_extension_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_get_extension_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_get_extension_version_reply_t {
        sym!(self, xcb_input_get_extension_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_get_extension_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_extension_version_reply(&self) -> bool {
        has_sym!(self, xcb_input_get_extension_version_reply)
    }

    pub unsafe fn xcb_input_device_info_next(&self, i: *mut xcb_input_device_info_iterator_t) {
        sym!(self, xcb_input_device_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_info_next(&self) -> bool {
        has_sym!(self, xcb_input_device_info_next)
    }

    pub unsafe fn xcb_input_device_info_end(
        &self,
        i: xcb_input_device_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_info_end(&self) -> bool {
        has_sym!(self, xcb_input_device_info_end)
    }

    pub unsafe fn xcb_input_key_info_next(&self, i: *mut xcb_input_key_info_iterator_t) {
        sym!(self, xcb_input_key_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_key_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_info_next(&self) -> bool {
        has_sym!(self, xcb_input_key_info_next)
    }

    pub unsafe fn xcb_input_key_info_end(
        &self,
        i: xcb_input_key_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_key_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_key_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_info_end(&self) -> bool {
        has_sym!(self, xcb_input_key_info_end)
    }

    pub unsafe fn xcb_input_button_info_next(&self, i: *mut xcb_input_button_info_iterator_t) {
        sym!(self, xcb_input_button_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_button_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_info_next(&self) -> bool {
        has_sym!(self, xcb_input_button_info_next)
    }

    pub unsafe fn xcb_input_button_info_end(
        &self,
        i: xcb_input_button_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_button_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_button_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_info_end(&self) -> bool {
        has_sym!(self, xcb_input_button_info_end)
    }

    pub unsafe fn xcb_input_axis_info_next(&self, i: *mut xcb_input_axis_info_iterator_t) {
        sym!(self, xcb_input_axis_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_axis_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_axis_info_next(&self) -> bool {
        has_sym!(self, xcb_input_axis_info_next)
    }

    pub unsafe fn xcb_input_axis_info_end(
        &self,
        i: xcb_input_axis_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_axis_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_axis_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_axis_info_end(&self) -> bool {
        has_sym!(self, xcb_input_axis_info_end)
    }

    pub unsafe fn xcb_input_valuator_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_valuator_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_valuator_info_sizeof)
    }

    pub unsafe fn xcb_input_valuator_info_axes(
        &self,
        r: *const xcb_input_valuator_info_t,
    ) -> *mut xcb_input_axis_info_t {
        sym!(self, xcb_input_valuator_info_axes)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_info_axes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_info_axes(&self) -> bool {
        has_sym!(self, xcb_input_valuator_info_axes)
    }

    pub unsafe fn xcb_input_valuator_info_axes_length(
        &self,
        r: *const xcb_input_valuator_info_t,
    ) -> c_int {
        sym!(self, xcb_input_valuator_info_axes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_info_axes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_info_axes_length(&self) -> bool {
        has_sym!(self, xcb_input_valuator_info_axes_length)
    }

    pub unsafe fn xcb_input_valuator_info_axes_iterator(
        &self,
        r: *const xcb_input_valuator_info_t,
    ) -> xcb_input_axis_info_iterator_t {
        sym!(self, xcb_input_valuator_info_axes_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_info_axes_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_info_axes_iterator(&self) -> bool {
        has_sym!(self, xcb_input_valuator_info_axes_iterator)
    }

    pub unsafe fn xcb_input_valuator_info_next(&self, i: *mut xcb_input_valuator_info_iterator_t) {
        sym!(self, xcb_input_valuator_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_info_next(&self) -> bool {
        has_sym!(self, xcb_input_valuator_info_next)
    }

    pub unsafe fn xcb_input_valuator_info_end(
        &self,
        i: xcb_input_valuator_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_valuator_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_info_end(&self) -> bool {
        has_sym!(self, xcb_input_valuator_info_end)
    }

    pub unsafe fn xcb_input_input_info_info_valuator_axes(
        &self,
        s: *const xcb_input_input_info_info_t,
    ) -> *mut xcb_input_axis_info_t {
        sym!(self, xcb_input_input_info_info_valuator_axes)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_input_info_info_valuator_axes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_info_info_valuator_axes(&self) -> bool {
        has_sym!(self, xcb_input_input_info_info_valuator_axes)
    }

    pub unsafe fn xcb_input_input_info_info_valuator_axes_length(
        &self,
        r: *const xcb_input_input_info_t,
        s: *const xcb_input_input_info_info_t,
    ) -> c_int {
        sym!(self, xcb_input_input_info_info_valuator_axes_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_input_info_info_valuator_axes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_info_info_valuator_axes_length(&self) -> bool {
        has_sym!(self, xcb_input_input_info_info_valuator_axes_length)
    }

    pub unsafe fn xcb_input_input_info_info_valuator_axes_iterator(
        &self,
        r: *const xcb_input_input_info_t,
        s: *const xcb_input_input_info_info_t,
    ) -> xcb_input_axis_info_iterator_t {
        sym!(self, xcb_input_input_info_info_valuator_axes_iterator)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_input_info_info_valuator_axes_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_info_info_valuator_axes_iterator(&self) -> bool {
        has_sym!(self, xcb_input_input_info_info_valuator_axes_iterator)
    }

    pub unsafe fn xcb_input_input_info_info_serialize(
        &self,
        _buffer: *mut *mut c_void,
        class_id: u8,
        _aux: *const xcb_input_input_info_info_t,
    ) -> c_int {
        sym!(self, xcb_input_input_info_info_serialize)(_buffer, class_id, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_input_info_info_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_info_info_serialize(&self) -> bool {
        has_sym!(self, xcb_input_input_info_info_serialize)
    }

    pub unsafe fn xcb_input_input_info_info_unpack(
        &self,
        _buffer: *const c_void,
        class_id: u8,
        _aux: *mut xcb_input_input_info_info_t,
    ) -> c_int {
        sym!(self, xcb_input_input_info_info_unpack)(_buffer, class_id, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_input_info_info_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_info_info_unpack(&self) -> bool {
        has_sym!(self, xcb_input_input_info_info_unpack)
    }

    pub unsafe fn xcb_input_input_info_info_sizeof(
        &self,
        _buffer: *const c_void,
        class_id: u8,
    ) -> c_int {
        sym!(self, xcb_input_input_info_info_sizeof)(_buffer, class_id)
    }

    /// Returns `true` iff the symbol `xcb_input_input_info_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_info_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_input_info_info_sizeof)
    }

    pub unsafe fn xcb_input_input_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_input_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_input_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_input_info_sizeof)
    }

    pub unsafe fn xcb_input_input_info_info(
        &self,
        r: *const xcb_input_input_info_t,
    ) -> *mut c_void {
        sym!(self, xcb_input_input_info_info)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_input_info_info` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_info_info(&self) -> bool {
        has_sym!(self, xcb_input_input_info_info)
    }

    pub unsafe fn xcb_input_input_info_next(&self, i: *mut xcb_input_input_info_iterator_t) {
        sym!(self, xcb_input_input_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_input_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_info_next(&self) -> bool {
        has_sym!(self, xcb_input_input_info_next)
    }

    pub unsafe fn xcb_input_input_info_end(
        &self,
        i: xcb_input_input_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_input_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_input_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_info_end(&self) -> bool {
        has_sym!(self, xcb_input_input_info_end)
    }

    pub unsafe fn xcb_input_device_name_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_device_name_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_device_name_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_name_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_device_name_sizeof)
    }

    pub unsafe fn xcb_input_device_name_string(
        &self,
        r: *const xcb_input_device_name_t,
    ) -> *mut c_char {
        sym!(self, xcb_input_device_name_string)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_name_string` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_name_string(&self) -> bool {
        has_sym!(self, xcb_input_device_name_string)
    }

    pub unsafe fn xcb_input_device_name_string_length(
        &self,
        r: *const xcb_input_device_name_t,
    ) -> c_int {
        sym!(self, xcb_input_device_name_string_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_name_string_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_name_string_length(&self) -> bool {
        has_sym!(self, xcb_input_device_name_string_length)
    }

    pub unsafe fn xcb_input_device_name_string_end(
        &self,
        r: *const xcb_input_device_name_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_name_string_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_name_string_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_name_string_end(&self) -> bool {
        has_sym!(self, xcb_input_device_name_string_end)
    }

    pub unsafe fn xcb_input_device_name_next(&self, i: *mut xcb_input_device_name_iterator_t) {
        sym!(self, xcb_input_device_name_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_name_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_name_next(&self) -> bool {
        has_sym!(self, xcb_input_device_name_next)
    }

    pub unsafe fn xcb_input_device_name_end(
        &self,
        i: xcb_input_device_name_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_name_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_name_end(&self) -> bool {
        has_sym!(self, xcb_input_device_name_end)
    }

    pub unsafe fn xcb_input_list_input_devices_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_list_input_devices_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_list_input_devices_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_input_devices_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_list_input_devices_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_list_input_devices(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_input_list_input_devices_cookie_t {
        sym!(self, xcb_input_list_input_devices)(c)
    }

    /// Returns `true` iff the symbol `xcb_input_list_input_devices` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_input_devices(&self) -> bool {
        has_sym!(self, xcb_input_list_input_devices)
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
    pub unsafe fn xcb_input_list_input_devices_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_input_list_input_devices_cookie_t {
        sym!(self, xcb_input_list_input_devices_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_input_list_input_devices_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_input_devices_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_list_input_devices_unchecked)
    }

    pub unsafe fn xcb_input_list_input_devices_devices(
        &self,
        r: *const xcb_input_list_input_devices_reply_t,
    ) -> *mut xcb_input_device_info_t {
        sym!(self, xcb_input_list_input_devices_devices)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_list_input_devices_devices` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_input_devices_devices(&self) -> bool {
        has_sym!(self, xcb_input_list_input_devices_devices)
    }

    pub unsafe fn xcb_input_list_input_devices_devices_length(
        &self,
        r: *const xcb_input_list_input_devices_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_list_input_devices_devices_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_list_input_devices_devices_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_input_devices_devices_length(&self) -> bool {
        has_sym!(self, xcb_input_list_input_devices_devices_length)
    }

    pub unsafe fn xcb_input_list_input_devices_devices_iterator(
        &self,
        r: *const xcb_input_list_input_devices_reply_t,
    ) -> xcb_input_device_info_iterator_t {
        sym!(self, xcb_input_list_input_devices_devices_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_list_input_devices_devices_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_input_devices_devices_iterator(&self) -> bool {
        has_sym!(self, xcb_input_list_input_devices_devices_iterator)
    }

    pub unsafe fn xcb_input_list_input_devices_infos_length(
        &self,
        r: *const xcb_input_list_input_devices_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_list_input_devices_infos_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_list_input_devices_infos_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_input_devices_infos_length(&self) -> bool {
        has_sym!(self, xcb_input_list_input_devices_infos_length)
    }

    pub unsafe fn xcb_input_list_input_devices_infos_iterator(
        &self,
        r: *const xcb_input_list_input_devices_reply_t,
    ) -> xcb_input_input_info_iterator_t {
        sym!(self, xcb_input_list_input_devices_infos_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_list_input_devices_infos_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_input_devices_infos_iterator(&self) -> bool {
        has_sym!(self, xcb_input_list_input_devices_infos_iterator)
    }

    pub unsafe fn xcb_input_list_input_devices_names_length(
        &self,
        r: *const xcb_input_list_input_devices_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_list_input_devices_names_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_list_input_devices_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_input_devices_names_length(&self) -> bool {
        has_sym!(self, xcb_input_list_input_devices_names_length)
    }

    pub unsafe fn xcb_input_list_input_devices_names_iterator(
        &self,
        r: *const xcb_input_list_input_devices_reply_t,
    ) -> xcb_str_iterator_t {
        sym!(self, xcb_input_list_input_devices_names_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_list_input_devices_names_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_input_devices_names_iterator(&self) -> bool {
        has_sym!(self, xcb_input_list_input_devices_names_iterator)
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
     * xcb_input_list_input_devices_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_list_input_devices_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_list_input_devices_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_list_input_devices_reply_t {
        sym!(self, xcb_input_list_input_devices_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_list_input_devices_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_input_devices_reply(&self) -> bool {
        has_sym!(self, xcb_input_list_input_devices_reply)
    }

    pub unsafe fn xcb_input_event_type_base_next(
        &self,
        i: *mut xcb_input_event_type_base_iterator_t,
    ) {
        sym!(self, xcb_input_event_type_base_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_event_type_base_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_event_type_base_next(&self) -> bool {
        has_sym!(self, xcb_input_event_type_base_next)
    }

    pub unsafe fn xcb_input_event_type_base_end(
        &self,
        i: xcb_input_event_type_base_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_event_type_base_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_event_type_base_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_event_type_base_end(&self) -> bool {
        has_sym!(self, xcb_input_event_type_base_end)
    }

    pub unsafe fn xcb_input_input_class_info_next(
        &self,
        i: *mut xcb_input_input_class_info_iterator_t,
    ) {
        sym!(self, xcb_input_input_class_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_input_class_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_class_info_next(&self) -> bool {
        has_sym!(self, xcb_input_input_class_info_next)
    }

    pub unsafe fn xcb_input_input_class_info_end(
        &self,
        i: xcb_input_input_class_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_input_class_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_input_class_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_class_info_end(&self) -> bool {
        has_sym!(self, xcb_input_input_class_info_end)
    }

    pub unsafe fn xcb_input_open_device_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_open_device_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_open_device_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_open_device_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_open_device_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_open_device(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_open_device_cookie_t {
        sym!(self, xcb_input_open_device)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_open_device` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_open_device(&self) -> bool {
        has_sym!(self, xcb_input_open_device)
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
    pub unsafe fn xcb_input_open_device_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_open_device_cookie_t {
        sym!(self, xcb_input_open_device_unchecked)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_open_device_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_open_device_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_open_device_unchecked)
    }

    pub unsafe fn xcb_input_open_device_class_info(
        &self,
        r: *const xcb_input_open_device_reply_t,
    ) -> *mut xcb_input_input_class_info_t {
        sym!(self, xcb_input_open_device_class_info)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_open_device_class_info` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_open_device_class_info(&self) -> bool {
        has_sym!(self, xcb_input_open_device_class_info)
    }

    pub unsafe fn xcb_input_open_device_class_info_length(
        &self,
        r: *const xcb_input_open_device_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_open_device_class_info_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_open_device_class_info_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_open_device_class_info_length(&self) -> bool {
        has_sym!(self, xcb_input_open_device_class_info_length)
    }

    pub unsafe fn xcb_input_open_device_class_info_iterator(
        &self,
        r: *const xcb_input_open_device_reply_t,
    ) -> xcb_input_input_class_info_iterator_t {
        sym!(self, xcb_input_open_device_class_info_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_open_device_class_info_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_open_device_class_info_iterator(&self) -> bool {
        has_sym!(self, xcb_input_open_device_class_info_iterator)
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
     * xcb_input_open_device_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_open_device_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_open_device_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_open_device_reply_t {
        sym!(self, xcb_input_open_device_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_open_device_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_open_device_reply(&self) -> bool {
        has_sym!(self, xcb_input_open_device_reply)
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
    pub unsafe fn xcb_input_close_device_checked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_close_device_checked)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_close_device_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_close_device_checked(&self) -> bool {
        has_sym!(self, xcb_input_close_device_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_close_device(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_close_device)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_close_device` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_close_device(&self) -> bool {
        has_sym!(self, xcb_input_close_device)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_set_device_mode(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        mode: u8,
    ) -> xcb_input_set_device_mode_cookie_t {
        sym!(self, xcb_input_set_device_mode)(c, device_id, mode)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_mode` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_mode(&self) -> bool {
        has_sym!(self, xcb_input_set_device_mode)
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
    pub unsafe fn xcb_input_set_device_mode_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        mode: u8,
    ) -> xcb_input_set_device_mode_cookie_t {
        sym!(self, xcb_input_set_device_mode_unchecked)(c, device_id, mode)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_mode_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_mode_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_set_device_mode_unchecked)
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
     * xcb_input_set_device_mode_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_set_device_mode_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_set_device_mode_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_set_device_mode_reply_t {
        sym!(self, xcb_input_set_device_mode_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_mode_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_mode_reply(&self) -> bool {
        has_sym!(self, xcb_input_set_device_mode_reply)
    }

    pub unsafe fn xcb_input_select_extension_event_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_select_extension_event_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_select_extension_event_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_select_extension_event_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_select_extension_event_sizeof)
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
    pub unsafe fn xcb_input_select_extension_event_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        num_classes: u16,
        classes: *const xcb_input_event_class_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_select_extension_event_checked)(c, window, num_classes, classes)
    }

    /// Returns `true` iff the symbol `xcb_input_select_extension_event_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_select_extension_event_checked(&self) -> bool {
        has_sym!(self, xcb_input_select_extension_event_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_select_extension_event(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        num_classes: u16,
        classes: *const xcb_input_event_class_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_select_extension_event)(c, window, num_classes, classes)
    }

    /// Returns `true` iff the symbol `xcb_input_select_extension_event` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_select_extension_event(&self) -> bool {
        has_sym!(self, xcb_input_select_extension_event)
    }

    pub unsafe fn xcb_input_select_extension_event_classes(
        &self,
        r: *const xcb_input_select_extension_event_request_t,
    ) -> *mut xcb_input_event_class_t {
        sym!(self, xcb_input_select_extension_event_classes)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_select_extension_event_classes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_select_extension_event_classes(&self) -> bool {
        has_sym!(self, xcb_input_select_extension_event_classes)
    }

    pub unsafe fn xcb_input_select_extension_event_classes_length(
        &self,
        r: *const xcb_input_select_extension_event_request_t,
    ) -> c_int {
        sym!(self, xcb_input_select_extension_event_classes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_select_extension_event_classes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_select_extension_event_classes_length(&self) -> bool {
        has_sym!(self, xcb_input_select_extension_event_classes_length)
    }

    pub unsafe fn xcb_input_select_extension_event_classes_end(
        &self,
        r: *const xcb_input_select_extension_event_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_select_extension_event_classes_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_select_extension_event_classes_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_select_extension_event_classes_end(&self) -> bool {
        has_sym!(self, xcb_input_select_extension_event_classes_end)
    }

    pub unsafe fn xcb_input_get_selected_extension_events_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_input_get_selected_extension_events_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_get_selected_extension_events_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_selected_extension_events_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_get_selected_extension_events_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_get_selected_extension_events(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_input_get_selected_extension_events_cookie_t {
        sym!(self, xcb_input_get_selected_extension_events)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_input_get_selected_extension_events` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_selected_extension_events(&self) -> bool {
        has_sym!(self, xcb_input_get_selected_extension_events)
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
    pub unsafe fn xcb_input_get_selected_extension_events_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_input_get_selected_extension_events_cookie_t {
        sym!(self, xcb_input_get_selected_extension_events_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_input_get_selected_extension_events_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_selected_extension_events_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_get_selected_extension_events_unchecked)
    }

    pub unsafe fn xcb_input_get_selected_extension_events_this_classes(
        &self,
        r: *const xcb_input_get_selected_extension_events_reply_t,
    ) -> *mut xcb_input_event_class_t {
        sym!(self, xcb_input_get_selected_extension_events_this_classes)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_selected_extension_events_this_classes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_selected_extension_events_this_classes(&self) -> bool {
        has_sym!(self, xcb_input_get_selected_extension_events_this_classes)
    }

    pub unsafe fn xcb_input_get_selected_extension_events_this_classes_length(
        &self,
        r: *const xcb_input_get_selected_extension_events_reply_t,
    ) -> c_int {
        sym!(
            self,
            xcb_input_get_selected_extension_events_this_classes_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_selected_extension_events_this_classes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_selected_extension_events_this_classes_length(&self) -> bool {
        has_sym!(
            self,
            xcb_input_get_selected_extension_events_this_classes_length
        )
    }

    pub unsafe fn xcb_input_get_selected_extension_events_this_classes_end(
        &self,
        r: *const xcb_input_get_selected_extension_events_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_input_get_selected_extension_events_this_classes_end
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_selected_extension_events_this_classes_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_selected_extension_events_this_classes_end(&self) -> bool {
        has_sym!(
            self,
            xcb_input_get_selected_extension_events_this_classes_end
        )
    }

    pub unsafe fn xcb_input_get_selected_extension_events_all_classes(
        &self,
        r: *const xcb_input_get_selected_extension_events_reply_t,
    ) -> *mut xcb_input_event_class_t {
        sym!(self, xcb_input_get_selected_extension_events_all_classes)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_selected_extension_events_all_classes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_selected_extension_events_all_classes(&self) -> bool {
        has_sym!(self, xcb_input_get_selected_extension_events_all_classes)
    }

    pub unsafe fn xcb_input_get_selected_extension_events_all_classes_length(
        &self,
        r: *const xcb_input_get_selected_extension_events_reply_t,
    ) -> c_int {
        sym!(
            self,
            xcb_input_get_selected_extension_events_all_classes_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_selected_extension_events_all_classes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_selected_extension_events_all_classes_length(&self) -> bool {
        has_sym!(
            self,
            xcb_input_get_selected_extension_events_all_classes_length
        )
    }

    pub unsafe fn xcb_input_get_selected_extension_events_all_classes_end(
        &self,
        r: *const xcb_input_get_selected_extension_events_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_input_get_selected_extension_events_all_classes_end
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_selected_extension_events_all_classes_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_selected_extension_events_all_classes_end(&self) -> bool {
        has_sym!(
            self,
            xcb_input_get_selected_extension_events_all_classes_end
        )
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
     * xcb_input_get_selected_extension_events_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_get_selected_extension_events_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_get_selected_extension_events_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_get_selected_extension_events_reply_t {
        sym!(self, xcb_input_get_selected_extension_events_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_get_selected_extension_events_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_selected_extension_events_reply(&self) -> bool {
        has_sym!(self, xcb_input_get_selected_extension_events_reply)
    }

    pub unsafe fn xcb_input_change_device_dont_propagate_list_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_input_change_device_dont_propagate_list_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_dont_propagate_list_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_dont_propagate_list_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_change_device_dont_propagate_list_sizeof)
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
    pub unsafe fn xcb_input_change_device_dont_propagate_list_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        num_classes: u16,
        mode: u8,
        classes: *const xcb_input_event_class_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_change_device_dont_propagate_list_checked)(
            c,
            window,
            num_classes,
            mode,
            classes,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_dont_propagate_list_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_dont_propagate_list_checked(&self) -> bool {
        has_sym!(self, xcb_input_change_device_dont_propagate_list_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_change_device_dont_propagate_list(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        num_classes: u16,
        mode: u8,
        classes: *const xcb_input_event_class_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_change_device_dont_propagate_list)(
            c,
            window,
            num_classes,
            mode,
            classes,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_dont_propagate_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_dont_propagate_list(&self) -> bool {
        has_sym!(self, xcb_input_change_device_dont_propagate_list)
    }

    pub unsafe fn xcb_input_change_device_dont_propagate_list_classes(
        &self,
        r: *const xcb_input_change_device_dont_propagate_list_request_t,
    ) -> *mut xcb_input_event_class_t {
        sym!(self, xcb_input_change_device_dont_propagate_list_classes)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_dont_propagate_list_classes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_dont_propagate_list_classes(&self) -> bool {
        has_sym!(self, xcb_input_change_device_dont_propagate_list_classes)
    }

    pub unsafe fn xcb_input_change_device_dont_propagate_list_classes_length(
        &self,
        r: *const xcb_input_change_device_dont_propagate_list_request_t,
    ) -> c_int {
        sym!(
            self,
            xcb_input_change_device_dont_propagate_list_classes_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_dont_propagate_list_classes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_dont_propagate_list_classes_length(&self) -> bool {
        has_sym!(
            self,
            xcb_input_change_device_dont_propagate_list_classes_length
        )
    }

    pub unsafe fn xcb_input_change_device_dont_propagate_list_classes_end(
        &self,
        r: *const xcb_input_change_device_dont_propagate_list_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_input_change_device_dont_propagate_list_classes_end
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_dont_propagate_list_classes_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_dont_propagate_list_classes_end(&self) -> bool {
        has_sym!(
            self,
            xcb_input_change_device_dont_propagate_list_classes_end
        )
    }

    pub unsafe fn xcb_input_get_device_dont_propagate_list_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_input_get_device_dont_propagate_list_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_dont_propagate_list_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_dont_propagate_list_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_get_device_dont_propagate_list_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_get_device_dont_propagate_list(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_input_get_device_dont_propagate_list_cookie_t {
        sym!(self, xcb_input_get_device_dont_propagate_list)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_dont_propagate_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_dont_propagate_list(&self) -> bool {
        has_sym!(self, xcb_input_get_device_dont_propagate_list)
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
    pub unsafe fn xcb_input_get_device_dont_propagate_list_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_input_get_device_dont_propagate_list_cookie_t {
        sym!(self, xcb_input_get_device_dont_propagate_list_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_dont_propagate_list_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_dont_propagate_list_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_get_device_dont_propagate_list_unchecked)
    }

    pub unsafe fn xcb_input_get_device_dont_propagate_list_classes(
        &self,
        r: *const xcb_input_get_device_dont_propagate_list_reply_t,
    ) -> *mut xcb_input_event_class_t {
        sym!(self, xcb_input_get_device_dont_propagate_list_classes)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_dont_propagate_list_classes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_dont_propagate_list_classes(&self) -> bool {
        has_sym!(self, xcb_input_get_device_dont_propagate_list_classes)
    }

    pub unsafe fn xcb_input_get_device_dont_propagate_list_classes_length(
        &self,
        r: *const xcb_input_get_device_dont_propagate_list_reply_t,
    ) -> c_int {
        sym!(
            self,
            xcb_input_get_device_dont_propagate_list_classes_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_dont_propagate_list_classes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_dont_propagate_list_classes_length(&self) -> bool {
        has_sym!(
            self,
            xcb_input_get_device_dont_propagate_list_classes_length
        )
    }

    pub unsafe fn xcb_input_get_device_dont_propagate_list_classes_end(
        &self,
        r: *const xcb_input_get_device_dont_propagate_list_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_get_device_dont_propagate_list_classes_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_dont_propagate_list_classes_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_dont_propagate_list_classes_end(&self) -> bool {
        has_sym!(self, xcb_input_get_device_dont_propagate_list_classes_end)
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
     * xcb_input_get_device_dont_propagate_list_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_get_device_dont_propagate_list_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_get_device_dont_propagate_list_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_get_device_dont_propagate_list_reply_t {
        sym!(self, xcb_input_get_device_dont_propagate_list_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_dont_propagate_list_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_dont_propagate_list_reply(&self) -> bool {
        has_sym!(self, xcb_input_get_device_dont_propagate_list_reply)
    }

    pub unsafe fn xcb_input_device_time_coord_sizeof(
        &self,
        _buffer: *const c_void,
        num_axes: u8,
    ) -> c_int {
        sym!(self, xcb_input_device_time_coord_sizeof)(_buffer, num_axes)
    }

    /// Returns `true` iff the symbol `xcb_input_device_time_coord_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_time_coord_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_device_time_coord_sizeof)
    }

    pub unsafe fn xcb_input_device_time_coord_axisvalues(
        &self,
        r: *const xcb_input_device_time_coord_t,
    ) -> *mut i32 {
        sym!(self, xcb_input_device_time_coord_axisvalues)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_time_coord_axisvalues` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_time_coord_axisvalues(&self) -> bool {
        has_sym!(self, xcb_input_device_time_coord_axisvalues)
    }

    pub unsafe fn xcb_input_device_time_coord_axisvalues_length(
        &self,
        r: *const xcb_input_device_time_coord_t,
        num_axes: u8,
    ) -> c_int {
        sym!(self, xcb_input_device_time_coord_axisvalues_length)(r, num_axes)
    }

    /// Returns `true` iff the symbol `xcb_input_device_time_coord_axisvalues_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_time_coord_axisvalues_length(&self) -> bool {
        has_sym!(self, xcb_input_device_time_coord_axisvalues_length)
    }

    pub unsafe fn xcb_input_device_time_coord_axisvalues_end(
        &self,
        r: *const xcb_input_device_time_coord_t,
        num_axes: u8,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_time_coord_axisvalues_end)(r, num_axes)
    }

    /// Returns `true` iff the symbol `xcb_input_device_time_coord_axisvalues_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_time_coord_axisvalues_end(&self) -> bool {
        has_sym!(self, xcb_input_device_time_coord_axisvalues_end)
    }

    pub unsafe fn xcb_input_device_time_coord_next(
        &self,
        i: *mut xcb_input_device_time_coord_iterator_t,
    ) {
        sym!(self, xcb_input_device_time_coord_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_time_coord_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_time_coord_next(&self) -> bool {
        has_sym!(self, xcb_input_device_time_coord_next)
    }

    pub unsafe fn xcb_input_device_time_coord_end(
        &self,
        i: xcb_input_device_time_coord_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_time_coord_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_time_coord_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_time_coord_end(&self) -> bool {
        has_sym!(self, xcb_input_device_time_coord_end)
    }

    pub unsafe fn xcb_input_get_device_motion_events_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_input_get_device_motion_events_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_motion_events_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_motion_events_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_get_device_motion_events_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_get_device_motion_events(
        &self,
        c: *mut xcb_connection_t,
        start: xcb_timestamp_t,
        stop: xcb_timestamp_t,
        device_id: u8,
    ) -> xcb_input_get_device_motion_events_cookie_t {
        sym!(self, xcb_input_get_device_motion_events)(c, start, stop, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_motion_events` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_motion_events(&self) -> bool {
        has_sym!(self, xcb_input_get_device_motion_events)
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
    pub unsafe fn xcb_input_get_device_motion_events_unchecked(
        &self,
        c: *mut xcb_connection_t,
        start: xcb_timestamp_t,
        stop: xcb_timestamp_t,
        device_id: u8,
    ) -> xcb_input_get_device_motion_events_cookie_t {
        sym!(self, xcb_input_get_device_motion_events_unchecked)(c, start, stop, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_motion_events_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_motion_events_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_get_device_motion_events_unchecked)
    }

    pub unsafe fn xcb_input_get_device_motion_events_events_length(
        &self,
        r: *const xcb_input_get_device_motion_events_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_get_device_motion_events_events_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_motion_events_events_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_motion_events_events_length(&self) -> bool {
        has_sym!(self, xcb_input_get_device_motion_events_events_length)
    }

    pub unsafe fn xcb_input_get_device_motion_events_events_iterator(
        &self,
        r: *const xcb_input_get_device_motion_events_reply_t,
    ) -> xcb_input_device_time_coord_iterator_t {
        sym!(self, xcb_input_get_device_motion_events_events_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_motion_events_events_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_motion_events_events_iterator(&self) -> bool {
        has_sym!(self, xcb_input_get_device_motion_events_events_iterator)
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
     * xcb_input_get_device_motion_events_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_get_device_motion_events_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_get_device_motion_events_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_get_device_motion_events_reply_t {
        sym!(self, xcb_input_get_device_motion_events_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_motion_events_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_motion_events_reply(&self) -> bool {
        has_sym!(self, xcb_input_get_device_motion_events_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_change_keyboard_device(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_change_keyboard_device_cookie_t {
        sym!(self, xcb_input_change_keyboard_device)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_change_keyboard_device` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_keyboard_device(&self) -> bool {
        has_sym!(self, xcb_input_change_keyboard_device)
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
    pub unsafe fn xcb_input_change_keyboard_device_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_change_keyboard_device_cookie_t {
        sym!(self, xcb_input_change_keyboard_device_unchecked)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_change_keyboard_device_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_keyboard_device_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_change_keyboard_device_unchecked)
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
     * xcb_input_change_keyboard_device_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_change_keyboard_device_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_change_keyboard_device_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_change_keyboard_device_reply_t {
        sym!(self, xcb_input_change_keyboard_device_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_change_keyboard_device_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_keyboard_device_reply(&self) -> bool {
        has_sym!(self, xcb_input_change_keyboard_device_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_change_pointer_device(
        &self,
        c: *mut xcb_connection_t,
        x_axis: u8,
        y_axis: u8,
        device_id: u8,
    ) -> xcb_input_change_pointer_device_cookie_t {
        sym!(self, xcb_input_change_pointer_device)(c, x_axis, y_axis, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_change_pointer_device` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_pointer_device(&self) -> bool {
        has_sym!(self, xcb_input_change_pointer_device)
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
    pub unsafe fn xcb_input_change_pointer_device_unchecked(
        &self,
        c: *mut xcb_connection_t,
        x_axis: u8,
        y_axis: u8,
        device_id: u8,
    ) -> xcb_input_change_pointer_device_cookie_t {
        sym!(self, xcb_input_change_pointer_device_unchecked)(c, x_axis, y_axis, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_change_pointer_device_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_pointer_device_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_change_pointer_device_unchecked)
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
     * xcb_input_change_pointer_device_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_change_pointer_device_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_change_pointer_device_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_change_pointer_device_reply_t {
        sym!(self, xcb_input_change_pointer_device_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_change_pointer_device_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_pointer_device_reply(&self) -> bool {
        has_sym!(self, xcb_input_change_pointer_device_reply)
    }

    pub unsafe fn xcb_input_grab_device_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_grab_device_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_grab_device(
        &self,
        c: *mut xcb_connection_t,
        grab_window: xcb_window_t,
        time: xcb_timestamp_t,
        num_classes: u16,
        this_device_mode: u8,
        other_device_mode: u8,
        owner_events: u8,
        device_id: u8,
        classes: *const xcb_input_event_class_t,
    ) -> xcb_input_grab_device_cookie_t {
        sym!(self, xcb_input_grab_device)(
            c,
            grab_window,
            time,
            num_classes,
            this_device_mode,
            other_device_mode,
            owner_events,
            device_id,
            classes,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device(&self) -> bool {
        has_sym!(self, xcb_input_grab_device)
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
    pub unsafe fn xcb_input_grab_device_unchecked(
        &self,
        c: *mut xcb_connection_t,
        grab_window: xcb_window_t,
        time: xcb_timestamp_t,
        num_classes: u16,
        this_device_mode: u8,
        other_device_mode: u8,
        owner_events: u8,
        device_id: u8,
        classes: *const xcb_input_event_class_t,
    ) -> xcb_input_grab_device_cookie_t {
        sym!(self, xcb_input_grab_device_unchecked)(
            c,
            grab_window,
            time,
            num_classes,
            this_device_mode,
            other_device_mode,
            owner_events,
            device_id,
            classes,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_unchecked)
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
     * xcb_input_grab_device_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_grab_device_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_grab_device_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_grab_device_reply_t {
        sym!(self, xcb_input_grab_device_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_reply(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_reply)
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
    pub unsafe fn xcb_input_ungrab_device_checked(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
        device_id: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_ungrab_device_checked)(c, time, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_ungrab_device_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_ungrab_device_checked(&self) -> bool {
        has_sym!(self, xcb_input_ungrab_device_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_ungrab_device(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
        device_id: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_ungrab_device)(c, time, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_ungrab_device` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_ungrab_device(&self) -> bool {
        has_sym!(self, xcb_input_ungrab_device)
    }

    pub unsafe fn xcb_input_grab_device_key_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_grab_device_key_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_key_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_key_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_key_sizeof)
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
    pub unsafe fn xcb_input_grab_device_key_checked(
        &self,
        c: *mut xcb_connection_t,
        grab_window: xcb_window_t,
        num_classes: u16,
        modifiers: u16,
        modifier_device: u8,
        grabbed_device: u8,
        key: u8,
        this_device_mode: u8,
        other_device_mode: u8,
        owner_events: u8,
        classes: *const xcb_input_event_class_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_grab_device_key_checked)(
            c,
            grab_window,
            num_classes,
            modifiers,
            modifier_device,
            grabbed_device,
            key,
            this_device_mode,
            other_device_mode,
            owner_events,
            classes,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_key_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_key_checked(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_key_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_grab_device_key(
        &self,
        c: *mut xcb_connection_t,
        grab_window: xcb_window_t,
        num_classes: u16,
        modifiers: u16,
        modifier_device: u8,
        grabbed_device: u8,
        key: u8,
        this_device_mode: u8,
        other_device_mode: u8,
        owner_events: u8,
        classes: *const xcb_input_event_class_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_grab_device_key)(
            c,
            grab_window,
            num_classes,
            modifiers,
            modifier_device,
            grabbed_device,
            key,
            this_device_mode,
            other_device_mode,
            owner_events,
            classes,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_key` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_key(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_key)
    }

    pub unsafe fn xcb_input_grab_device_key_classes(
        &self,
        r: *const xcb_input_grab_device_key_request_t,
    ) -> *mut xcb_input_event_class_t {
        sym!(self, xcb_input_grab_device_key_classes)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_key_classes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_key_classes(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_key_classes)
    }

    pub unsafe fn xcb_input_grab_device_key_classes_length(
        &self,
        r: *const xcb_input_grab_device_key_request_t,
    ) -> c_int {
        sym!(self, xcb_input_grab_device_key_classes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_key_classes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_key_classes_length(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_key_classes_length)
    }

    pub unsafe fn xcb_input_grab_device_key_classes_end(
        &self,
        r: *const xcb_input_grab_device_key_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_grab_device_key_classes_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_key_classes_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_key_classes_end(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_key_classes_end)
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
    pub unsafe fn xcb_input_ungrab_device_key_checked(
        &self,
        c: *mut xcb_connection_t,
        grab_window: xcb_window_t,
        modifiers: u16,
        modifier_device: u8,
        key: u8,
        grabbed_device: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_ungrab_device_key_checked)(
            c,
            grab_window,
            modifiers,
            modifier_device,
            key,
            grabbed_device,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_ungrab_device_key_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_ungrab_device_key_checked(&self) -> bool {
        has_sym!(self, xcb_input_ungrab_device_key_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_ungrab_device_key(
        &self,
        c: *mut xcb_connection_t,
        grab_window: xcb_window_t,
        modifiers: u16,
        modifier_device: u8,
        key: u8,
        grabbed_device: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_ungrab_device_key)(
            c,
            grab_window,
            modifiers,
            modifier_device,
            key,
            grabbed_device,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_ungrab_device_key` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_ungrab_device_key(&self) -> bool {
        has_sym!(self, xcb_input_ungrab_device_key)
    }

    pub unsafe fn xcb_input_grab_device_button_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_grab_device_button_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_button_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_button_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_button_sizeof)
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
    pub unsafe fn xcb_input_grab_device_button_checked(
        &self,
        c: *mut xcb_connection_t,
        grab_window: xcb_window_t,
        grabbed_device: u8,
        modifier_device: u8,
        num_classes: u16,
        modifiers: u16,
        this_device_mode: u8,
        other_device_mode: u8,
        button: u8,
        owner_events: u8,
        classes: *const xcb_input_event_class_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_grab_device_button_checked)(
            c,
            grab_window,
            grabbed_device,
            modifier_device,
            num_classes,
            modifiers,
            this_device_mode,
            other_device_mode,
            button,
            owner_events,
            classes,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_button_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_button_checked(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_button_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_grab_device_button(
        &self,
        c: *mut xcb_connection_t,
        grab_window: xcb_window_t,
        grabbed_device: u8,
        modifier_device: u8,
        num_classes: u16,
        modifiers: u16,
        this_device_mode: u8,
        other_device_mode: u8,
        button: u8,
        owner_events: u8,
        classes: *const xcb_input_event_class_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_grab_device_button)(
            c,
            grab_window,
            grabbed_device,
            modifier_device,
            num_classes,
            modifiers,
            this_device_mode,
            other_device_mode,
            button,
            owner_events,
            classes,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_button` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_button(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_button)
    }

    pub unsafe fn xcb_input_grab_device_button_classes(
        &self,
        r: *const xcb_input_grab_device_button_request_t,
    ) -> *mut xcb_input_event_class_t {
        sym!(self, xcb_input_grab_device_button_classes)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_button_classes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_button_classes(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_button_classes)
    }

    pub unsafe fn xcb_input_grab_device_button_classes_length(
        &self,
        r: *const xcb_input_grab_device_button_request_t,
    ) -> c_int {
        sym!(self, xcb_input_grab_device_button_classes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_button_classes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_button_classes_length(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_button_classes_length)
    }

    pub unsafe fn xcb_input_grab_device_button_classes_end(
        &self,
        r: *const xcb_input_grab_device_button_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_grab_device_button_classes_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_button_classes_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_button_classes_end(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_button_classes_end)
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
    pub unsafe fn xcb_input_ungrab_device_button_checked(
        &self,
        c: *mut xcb_connection_t,
        grab_window: xcb_window_t,
        modifiers: u16,
        modifier_device: u8,
        button: u8,
        grabbed_device: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_ungrab_device_button_checked)(
            c,
            grab_window,
            modifiers,
            modifier_device,
            button,
            grabbed_device,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_ungrab_device_button_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_ungrab_device_button_checked(&self) -> bool {
        has_sym!(self, xcb_input_ungrab_device_button_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_ungrab_device_button(
        &self,
        c: *mut xcb_connection_t,
        grab_window: xcb_window_t,
        modifiers: u16,
        modifier_device: u8,
        button: u8,
        grabbed_device: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_ungrab_device_button)(
            c,
            grab_window,
            modifiers,
            modifier_device,
            button,
            grabbed_device,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_ungrab_device_button` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_ungrab_device_button(&self) -> bool {
        has_sym!(self, xcb_input_ungrab_device_button)
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
    pub unsafe fn xcb_input_allow_device_events_checked(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
        mode: u8,
        device_id: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_allow_device_events_checked)(c, time, mode, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_allow_device_events_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_allow_device_events_checked(&self) -> bool {
        has_sym!(self, xcb_input_allow_device_events_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_allow_device_events(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
        mode: u8,
        device_id: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_allow_device_events)(c, time, mode, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_allow_device_events` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_allow_device_events(&self) -> bool {
        has_sym!(self, xcb_input_allow_device_events)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_get_device_focus(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_get_device_focus_cookie_t {
        sym!(self, xcb_input_get_device_focus)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_focus` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_focus(&self) -> bool {
        has_sym!(self, xcb_input_get_device_focus)
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
    pub unsafe fn xcb_input_get_device_focus_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_get_device_focus_cookie_t {
        sym!(self, xcb_input_get_device_focus_unchecked)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_focus_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_focus_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_get_device_focus_unchecked)
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
     * xcb_input_get_device_focus_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_get_device_focus_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_get_device_focus_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_get_device_focus_reply_t {
        sym!(self, xcb_input_get_device_focus_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_focus_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_focus_reply(&self) -> bool {
        has_sym!(self, xcb_input_get_device_focus_reply)
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
    pub unsafe fn xcb_input_set_device_focus_checked(
        &self,
        c: *mut xcb_connection_t,
        focus: xcb_window_t,
        time: xcb_timestamp_t,
        revert_to: u8,
        device_id: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_set_device_focus_checked)(c, focus, time, revert_to, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_focus_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_focus_checked(&self) -> bool {
        has_sym!(self, xcb_input_set_device_focus_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_set_device_focus(
        &self,
        c: *mut xcb_connection_t,
        focus: xcb_window_t,
        time: xcb_timestamp_t,
        revert_to: u8,
        device_id: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_set_device_focus)(c, focus, time, revert_to, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_focus` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_focus(&self) -> bool {
        has_sym!(self, xcb_input_set_device_focus)
    }

    pub unsafe fn xcb_input_kbd_feedback_state_next(
        &self,
        i: *mut xcb_input_kbd_feedback_state_iterator_t,
    ) {
        sym!(self, xcb_input_kbd_feedback_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_kbd_feedback_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_kbd_feedback_state_next(&self) -> bool {
        has_sym!(self, xcb_input_kbd_feedback_state_next)
    }

    pub unsafe fn xcb_input_kbd_feedback_state_end(
        &self,
        i: xcb_input_kbd_feedback_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_kbd_feedback_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_kbd_feedback_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_kbd_feedback_state_end(&self) -> bool {
        has_sym!(self, xcb_input_kbd_feedback_state_end)
    }

    pub unsafe fn xcb_input_ptr_feedback_state_next(
        &self,
        i: *mut xcb_input_ptr_feedback_state_iterator_t,
    ) {
        sym!(self, xcb_input_ptr_feedback_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_ptr_feedback_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_ptr_feedback_state_next(&self) -> bool {
        has_sym!(self, xcb_input_ptr_feedback_state_next)
    }

    pub unsafe fn xcb_input_ptr_feedback_state_end(
        &self,
        i: xcb_input_ptr_feedback_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_ptr_feedback_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_ptr_feedback_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_ptr_feedback_state_end(&self) -> bool {
        has_sym!(self, xcb_input_ptr_feedback_state_end)
    }

    pub unsafe fn xcb_input_integer_feedback_state_next(
        &self,
        i: *mut xcb_input_integer_feedback_state_iterator_t,
    ) {
        sym!(self, xcb_input_integer_feedback_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_integer_feedback_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_integer_feedback_state_next(&self) -> bool {
        has_sym!(self, xcb_input_integer_feedback_state_next)
    }

    pub unsafe fn xcb_input_integer_feedback_state_end(
        &self,
        i: xcb_input_integer_feedback_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_integer_feedback_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_integer_feedback_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_integer_feedback_state_end(&self) -> bool {
        has_sym!(self, xcb_input_integer_feedback_state_end)
    }

    pub unsafe fn xcb_input_string_feedback_state_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_string_feedback_state_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_string_feedback_state_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_string_feedback_state_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_string_feedback_state_sizeof)
    }

    pub unsafe fn xcb_input_string_feedback_state_keysyms(
        &self,
        r: *const xcb_input_string_feedback_state_t,
    ) -> *mut xcb_keysym_t {
        sym!(self, xcb_input_string_feedback_state_keysyms)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_string_feedback_state_keysyms` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_string_feedback_state_keysyms(&self) -> bool {
        has_sym!(self, xcb_input_string_feedback_state_keysyms)
    }

    pub unsafe fn xcb_input_string_feedback_state_keysyms_length(
        &self,
        r: *const xcb_input_string_feedback_state_t,
    ) -> c_int {
        sym!(self, xcb_input_string_feedback_state_keysyms_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_string_feedback_state_keysyms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_string_feedback_state_keysyms_length(&self) -> bool {
        has_sym!(self, xcb_input_string_feedback_state_keysyms_length)
    }

    pub unsafe fn xcb_input_string_feedback_state_keysyms_end(
        &self,
        r: *const xcb_input_string_feedback_state_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_string_feedback_state_keysyms_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_string_feedback_state_keysyms_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_string_feedback_state_keysyms_end(&self) -> bool {
        has_sym!(self, xcb_input_string_feedback_state_keysyms_end)
    }

    pub unsafe fn xcb_input_string_feedback_state_next(
        &self,
        i: *mut xcb_input_string_feedback_state_iterator_t,
    ) {
        sym!(self, xcb_input_string_feedback_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_string_feedback_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_string_feedback_state_next(&self) -> bool {
        has_sym!(self, xcb_input_string_feedback_state_next)
    }

    pub unsafe fn xcb_input_string_feedback_state_end(
        &self,
        i: xcb_input_string_feedback_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_string_feedback_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_string_feedback_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_string_feedback_state_end(&self) -> bool {
        has_sym!(self, xcb_input_string_feedback_state_end)
    }

    pub unsafe fn xcb_input_bell_feedback_state_next(
        &self,
        i: *mut xcb_input_bell_feedback_state_iterator_t,
    ) {
        sym!(self, xcb_input_bell_feedback_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_bell_feedback_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_bell_feedback_state_next(&self) -> bool {
        has_sym!(self, xcb_input_bell_feedback_state_next)
    }

    pub unsafe fn xcb_input_bell_feedback_state_end(
        &self,
        i: xcb_input_bell_feedback_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_bell_feedback_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_bell_feedback_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_bell_feedback_state_end(&self) -> bool {
        has_sym!(self, xcb_input_bell_feedback_state_end)
    }

    pub unsafe fn xcb_input_led_feedback_state_next(
        &self,
        i: *mut xcb_input_led_feedback_state_iterator_t,
    ) {
        sym!(self, xcb_input_led_feedback_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_led_feedback_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_led_feedback_state_next(&self) -> bool {
        has_sym!(self, xcb_input_led_feedback_state_next)
    }

    pub unsafe fn xcb_input_led_feedback_state_end(
        &self,
        i: xcb_input_led_feedback_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_led_feedback_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_led_feedback_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_led_feedback_state_end(&self) -> bool {
        has_sym!(self, xcb_input_led_feedback_state_end)
    }

    pub unsafe fn xcb_input_feedback_state_data_string_keysyms(
        &self,
        s: *const xcb_input_feedback_state_data_t,
    ) -> *mut xcb_keysym_t {
        sym!(self, xcb_input_feedback_state_data_string_keysyms)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_state_data_string_keysyms` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_state_data_string_keysyms(&self) -> bool {
        has_sym!(self, xcb_input_feedback_state_data_string_keysyms)
    }

    pub unsafe fn xcb_input_feedback_state_data_string_keysyms_length(
        &self,
        r: *const xcb_input_feedback_state_t,
        s: *const xcb_input_feedback_state_data_t,
    ) -> c_int {
        sym!(self, xcb_input_feedback_state_data_string_keysyms_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_state_data_string_keysyms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_state_data_string_keysyms_length(&self) -> bool {
        has_sym!(self, xcb_input_feedback_state_data_string_keysyms_length)
    }

    pub unsafe fn xcb_input_feedback_state_data_string_keysyms_end(
        &self,
        r: *const xcb_input_feedback_state_t,
        s: *const xcb_input_feedback_state_data_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_feedback_state_data_string_keysyms_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_state_data_string_keysyms_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_state_data_string_keysyms_end(&self) -> bool {
        has_sym!(self, xcb_input_feedback_state_data_string_keysyms_end)
    }

    pub unsafe fn xcb_input_feedback_state_data_serialize(
        &self,
        _buffer: *mut *mut c_void,
        class_id: u8,
        _aux: *const xcb_input_feedback_state_data_t,
    ) -> c_int {
        sym!(self, xcb_input_feedback_state_data_serialize)(_buffer, class_id, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_state_data_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_state_data_serialize(&self) -> bool {
        has_sym!(self, xcb_input_feedback_state_data_serialize)
    }

    pub unsafe fn xcb_input_feedback_state_data_unpack(
        &self,
        _buffer: *const c_void,
        class_id: u8,
        _aux: *mut xcb_input_feedback_state_data_t,
    ) -> c_int {
        sym!(self, xcb_input_feedback_state_data_unpack)(_buffer, class_id, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_state_data_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_state_data_unpack(&self) -> bool {
        has_sym!(self, xcb_input_feedback_state_data_unpack)
    }

    pub unsafe fn xcb_input_feedback_state_data_sizeof(
        &self,
        _buffer: *const c_void,
        class_id: u8,
    ) -> c_int {
        sym!(self, xcb_input_feedback_state_data_sizeof)(_buffer, class_id)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_state_data_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_state_data_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_feedback_state_data_sizeof)
    }

    pub unsafe fn xcb_input_feedback_state_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_feedback_state_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_state_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_state_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_feedback_state_sizeof)
    }

    pub unsafe fn xcb_input_feedback_state_data(
        &self,
        r: *const xcb_input_feedback_state_t,
    ) -> *mut c_void {
        sym!(self, xcb_input_feedback_state_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_state_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_state_data(&self) -> bool {
        has_sym!(self, xcb_input_feedback_state_data)
    }

    pub unsafe fn xcb_input_feedback_state_next(
        &self,
        i: *mut xcb_input_feedback_state_iterator_t,
    ) {
        sym!(self, xcb_input_feedback_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_state_next(&self) -> bool {
        has_sym!(self, xcb_input_feedback_state_next)
    }

    pub unsafe fn xcb_input_feedback_state_end(
        &self,
        i: xcb_input_feedback_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_feedback_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_state_end(&self) -> bool {
        has_sym!(self, xcb_input_feedback_state_end)
    }

    pub unsafe fn xcb_input_get_feedback_control_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_get_feedback_control_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_get_feedback_control_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_feedback_control_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_get_feedback_control_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_get_feedback_control(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_get_feedback_control_cookie_t {
        sym!(self, xcb_input_get_feedback_control)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_get_feedback_control` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_feedback_control(&self) -> bool {
        has_sym!(self, xcb_input_get_feedback_control)
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
    pub unsafe fn xcb_input_get_feedback_control_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_get_feedback_control_cookie_t {
        sym!(self, xcb_input_get_feedback_control_unchecked)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_get_feedback_control_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_feedback_control_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_get_feedback_control_unchecked)
    }

    pub unsafe fn xcb_input_get_feedback_control_feedbacks_length(
        &self,
        r: *const xcb_input_get_feedback_control_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_get_feedback_control_feedbacks_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_feedback_control_feedbacks_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_feedback_control_feedbacks_length(&self) -> bool {
        has_sym!(self, xcb_input_get_feedback_control_feedbacks_length)
    }

    pub unsafe fn xcb_input_get_feedback_control_feedbacks_iterator(
        &self,
        r: *const xcb_input_get_feedback_control_reply_t,
    ) -> xcb_input_feedback_state_iterator_t {
        sym!(self, xcb_input_get_feedback_control_feedbacks_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_feedback_control_feedbacks_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_feedback_control_feedbacks_iterator(&self) -> bool {
        has_sym!(self, xcb_input_get_feedback_control_feedbacks_iterator)
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
     * xcb_input_get_feedback_control_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_get_feedback_control_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_get_feedback_control_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_get_feedback_control_reply_t {
        sym!(self, xcb_input_get_feedback_control_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_get_feedback_control_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_feedback_control_reply(&self) -> bool {
        has_sym!(self, xcb_input_get_feedback_control_reply)
    }

    pub unsafe fn xcb_input_kbd_feedback_ctl_next(
        &self,
        i: *mut xcb_input_kbd_feedback_ctl_iterator_t,
    ) {
        sym!(self, xcb_input_kbd_feedback_ctl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_kbd_feedback_ctl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_kbd_feedback_ctl_next(&self) -> bool {
        has_sym!(self, xcb_input_kbd_feedback_ctl_next)
    }

    pub unsafe fn xcb_input_kbd_feedback_ctl_end(
        &self,
        i: xcb_input_kbd_feedback_ctl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_kbd_feedback_ctl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_kbd_feedback_ctl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_kbd_feedback_ctl_end(&self) -> bool {
        has_sym!(self, xcb_input_kbd_feedback_ctl_end)
    }

    pub unsafe fn xcb_input_ptr_feedback_ctl_next(
        &self,
        i: *mut xcb_input_ptr_feedback_ctl_iterator_t,
    ) {
        sym!(self, xcb_input_ptr_feedback_ctl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_ptr_feedback_ctl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_ptr_feedback_ctl_next(&self) -> bool {
        has_sym!(self, xcb_input_ptr_feedback_ctl_next)
    }

    pub unsafe fn xcb_input_ptr_feedback_ctl_end(
        &self,
        i: xcb_input_ptr_feedback_ctl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_ptr_feedback_ctl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_ptr_feedback_ctl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_ptr_feedback_ctl_end(&self) -> bool {
        has_sym!(self, xcb_input_ptr_feedback_ctl_end)
    }

    pub unsafe fn xcb_input_integer_feedback_ctl_next(
        &self,
        i: *mut xcb_input_integer_feedback_ctl_iterator_t,
    ) {
        sym!(self, xcb_input_integer_feedback_ctl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_integer_feedback_ctl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_integer_feedback_ctl_next(&self) -> bool {
        has_sym!(self, xcb_input_integer_feedback_ctl_next)
    }

    pub unsafe fn xcb_input_integer_feedback_ctl_end(
        &self,
        i: xcb_input_integer_feedback_ctl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_integer_feedback_ctl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_integer_feedback_ctl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_integer_feedback_ctl_end(&self) -> bool {
        has_sym!(self, xcb_input_integer_feedback_ctl_end)
    }

    pub unsafe fn xcb_input_string_feedback_ctl_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_string_feedback_ctl_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_string_feedback_ctl_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_string_feedback_ctl_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_string_feedback_ctl_sizeof)
    }

    pub unsafe fn xcb_input_string_feedback_ctl_keysyms(
        &self,
        r: *const xcb_input_string_feedback_ctl_t,
    ) -> *mut xcb_keysym_t {
        sym!(self, xcb_input_string_feedback_ctl_keysyms)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_string_feedback_ctl_keysyms` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_string_feedback_ctl_keysyms(&self) -> bool {
        has_sym!(self, xcb_input_string_feedback_ctl_keysyms)
    }

    pub unsafe fn xcb_input_string_feedback_ctl_keysyms_length(
        &self,
        r: *const xcb_input_string_feedback_ctl_t,
    ) -> c_int {
        sym!(self, xcb_input_string_feedback_ctl_keysyms_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_string_feedback_ctl_keysyms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_string_feedback_ctl_keysyms_length(&self) -> bool {
        has_sym!(self, xcb_input_string_feedback_ctl_keysyms_length)
    }

    pub unsafe fn xcb_input_string_feedback_ctl_keysyms_end(
        &self,
        r: *const xcb_input_string_feedback_ctl_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_string_feedback_ctl_keysyms_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_string_feedback_ctl_keysyms_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_string_feedback_ctl_keysyms_end(&self) -> bool {
        has_sym!(self, xcb_input_string_feedback_ctl_keysyms_end)
    }

    pub unsafe fn xcb_input_string_feedback_ctl_next(
        &self,
        i: *mut xcb_input_string_feedback_ctl_iterator_t,
    ) {
        sym!(self, xcb_input_string_feedback_ctl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_string_feedback_ctl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_string_feedback_ctl_next(&self) -> bool {
        has_sym!(self, xcb_input_string_feedback_ctl_next)
    }

    pub unsafe fn xcb_input_string_feedback_ctl_end(
        &self,
        i: xcb_input_string_feedback_ctl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_string_feedback_ctl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_string_feedback_ctl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_string_feedback_ctl_end(&self) -> bool {
        has_sym!(self, xcb_input_string_feedback_ctl_end)
    }

    pub unsafe fn xcb_input_bell_feedback_ctl_next(
        &self,
        i: *mut xcb_input_bell_feedback_ctl_iterator_t,
    ) {
        sym!(self, xcb_input_bell_feedback_ctl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_bell_feedback_ctl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_bell_feedback_ctl_next(&self) -> bool {
        has_sym!(self, xcb_input_bell_feedback_ctl_next)
    }

    pub unsafe fn xcb_input_bell_feedback_ctl_end(
        &self,
        i: xcb_input_bell_feedback_ctl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_bell_feedback_ctl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_bell_feedback_ctl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_bell_feedback_ctl_end(&self) -> bool {
        has_sym!(self, xcb_input_bell_feedback_ctl_end)
    }

    pub unsafe fn xcb_input_led_feedback_ctl_next(
        &self,
        i: *mut xcb_input_led_feedback_ctl_iterator_t,
    ) {
        sym!(self, xcb_input_led_feedback_ctl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_led_feedback_ctl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_led_feedback_ctl_next(&self) -> bool {
        has_sym!(self, xcb_input_led_feedback_ctl_next)
    }

    pub unsafe fn xcb_input_led_feedback_ctl_end(
        &self,
        i: xcb_input_led_feedback_ctl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_led_feedback_ctl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_led_feedback_ctl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_led_feedback_ctl_end(&self) -> bool {
        has_sym!(self, xcb_input_led_feedback_ctl_end)
    }

    pub unsafe fn xcb_input_feedback_ctl_data_string_keysyms(
        &self,
        s: *const xcb_input_feedback_ctl_data_t,
    ) -> *mut xcb_keysym_t {
        sym!(self, xcb_input_feedback_ctl_data_string_keysyms)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_ctl_data_string_keysyms` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_ctl_data_string_keysyms(&self) -> bool {
        has_sym!(self, xcb_input_feedback_ctl_data_string_keysyms)
    }

    pub unsafe fn xcb_input_feedback_ctl_data_string_keysyms_length(
        &self,
        r: *const xcb_input_feedback_ctl_t,
        s: *const xcb_input_feedback_ctl_data_t,
    ) -> c_int {
        sym!(self, xcb_input_feedback_ctl_data_string_keysyms_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_ctl_data_string_keysyms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_ctl_data_string_keysyms_length(&self) -> bool {
        has_sym!(self, xcb_input_feedback_ctl_data_string_keysyms_length)
    }

    pub unsafe fn xcb_input_feedback_ctl_data_string_keysyms_end(
        &self,
        r: *const xcb_input_feedback_ctl_t,
        s: *const xcb_input_feedback_ctl_data_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_feedback_ctl_data_string_keysyms_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_ctl_data_string_keysyms_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_ctl_data_string_keysyms_end(&self) -> bool {
        has_sym!(self, xcb_input_feedback_ctl_data_string_keysyms_end)
    }

    pub unsafe fn xcb_input_feedback_ctl_data_serialize(
        &self,
        _buffer: *mut *mut c_void,
        class_id: u8,
        _aux: *const xcb_input_feedback_ctl_data_t,
    ) -> c_int {
        sym!(self, xcb_input_feedback_ctl_data_serialize)(_buffer, class_id, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_ctl_data_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_ctl_data_serialize(&self) -> bool {
        has_sym!(self, xcb_input_feedback_ctl_data_serialize)
    }

    pub unsafe fn xcb_input_feedback_ctl_data_unpack(
        &self,
        _buffer: *const c_void,
        class_id: u8,
        _aux: *mut xcb_input_feedback_ctl_data_t,
    ) -> c_int {
        sym!(self, xcb_input_feedback_ctl_data_unpack)(_buffer, class_id, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_ctl_data_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_ctl_data_unpack(&self) -> bool {
        has_sym!(self, xcb_input_feedback_ctl_data_unpack)
    }

    pub unsafe fn xcb_input_feedback_ctl_data_sizeof(
        &self,
        _buffer: *const c_void,
        class_id: u8,
    ) -> c_int {
        sym!(self, xcb_input_feedback_ctl_data_sizeof)(_buffer, class_id)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_ctl_data_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_ctl_data_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_feedback_ctl_data_sizeof)
    }

    pub unsafe fn xcb_input_feedback_ctl_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_feedback_ctl_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_ctl_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_ctl_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_feedback_ctl_sizeof)
    }

    pub unsafe fn xcb_input_feedback_ctl_data(
        &self,
        r: *const xcb_input_feedback_ctl_t,
    ) -> *mut c_void {
        sym!(self, xcb_input_feedback_ctl_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_ctl_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_ctl_data(&self) -> bool {
        has_sym!(self, xcb_input_feedback_ctl_data)
    }

    pub unsafe fn xcb_input_feedback_ctl_next(&self, i: *mut xcb_input_feedback_ctl_iterator_t) {
        sym!(self, xcb_input_feedback_ctl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_ctl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_ctl_next(&self) -> bool {
        has_sym!(self, xcb_input_feedback_ctl_next)
    }

    pub unsafe fn xcb_input_feedback_ctl_end(
        &self,
        i: xcb_input_feedback_ctl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_feedback_ctl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_ctl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_ctl_end(&self) -> bool {
        has_sym!(self, xcb_input_feedback_ctl_end)
    }

    pub unsafe fn xcb_input_change_feedback_control_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_change_feedback_control_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_change_feedback_control_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_feedback_control_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_change_feedback_control_sizeof)
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
    pub unsafe fn xcb_input_change_feedback_control_checked(
        &self,
        c: *mut xcb_connection_t,
        mask: u32,
        device_id: u8,
        feedback_id: u8,
        feedback: *mut xcb_input_feedback_ctl_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_change_feedback_control_checked)(
            c,
            mask,
            device_id,
            feedback_id,
            feedback,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_change_feedback_control_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_feedback_control_checked(&self) -> bool {
        has_sym!(self, xcb_input_change_feedback_control_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_change_feedback_control(
        &self,
        c: *mut xcb_connection_t,
        mask: u32,
        device_id: u8,
        feedback_id: u8,
        feedback: *mut xcb_input_feedback_ctl_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_change_feedback_control)(c, mask, device_id, feedback_id, feedback)
    }

    /// Returns `true` iff the symbol `xcb_input_change_feedback_control` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_feedback_control(&self) -> bool {
        has_sym!(self, xcb_input_change_feedback_control)
    }

    pub unsafe fn xcb_input_change_feedback_control_feedback(
        &self,
        r: *const xcb_input_change_feedback_control_request_t,
    ) -> *mut xcb_input_feedback_ctl_t {
        sym!(self, xcb_input_change_feedback_control_feedback)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_change_feedback_control_feedback` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_feedback_control_feedback(&self) -> bool {
        has_sym!(self, xcb_input_change_feedback_control_feedback)
    }

    pub unsafe fn xcb_input_get_device_key_mapping_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_get_device_key_mapping_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_key_mapping_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_key_mapping_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_get_device_key_mapping_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_get_device_key_mapping(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        first_keycode: xcb_input_key_code_t,
        count: u8,
    ) -> xcb_input_get_device_key_mapping_cookie_t {
        sym!(self, xcb_input_get_device_key_mapping)(c, device_id, first_keycode, count)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_key_mapping` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_key_mapping(&self) -> bool {
        has_sym!(self, xcb_input_get_device_key_mapping)
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
    pub unsafe fn xcb_input_get_device_key_mapping_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        first_keycode: xcb_input_key_code_t,
        count: u8,
    ) -> xcb_input_get_device_key_mapping_cookie_t {
        sym!(self, xcb_input_get_device_key_mapping_unchecked)(c, device_id, first_keycode, count)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_key_mapping_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_key_mapping_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_get_device_key_mapping_unchecked)
    }

    pub unsafe fn xcb_input_get_device_key_mapping_keysyms(
        &self,
        r: *const xcb_input_get_device_key_mapping_reply_t,
    ) -> *mut xcb_keysym_t {
        sym!(self, xcb_input_get_device_key_mapping_keysyms)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_key_mapping_keysyms` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_key_mapping_keysyms(&self) -> bool {
        has_sym!(self, xcb_input_get_device_key_mapping_keysyms)
    }

    pub unsafe fn xcb_input_get_device_key_mapping_keysyms_length(
        &self,
        r: *const xcb_input_get_device_key_mapping_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_get_device_key_mapping_keysyms_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_key_mapping_keysyms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_key_mapping_keysyms_length(&self) -> bool {
        has_sym!(self, xcb_input_get_device_key_mapping_keysyms_length)
    }

    pub unsafe fn xcb_input_get_device_key_mapping_keysyms_end(
        &self,
        r: *const xcb_input_get_device_key_mapping_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_get_device_key_mapping_keysyms_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_key_mapping_keysyms_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_key_mapping_keysyms_end(&self) -> bool {
        has_sym!(self, xcb_input_get_device_key_mapping_keysyms_end)
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
     * xcb_input_get_device_key_mapping_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_get_device_key_mapping_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_get_device_key_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_get_device_key_mapping_reply_t {
        sym!(self, xcb_input_get_device_key_mapping_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_key_mapping_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_key_mapping_reply(&self) -> bool {
        has_sym!(self, xcb_input_get_device_key_mapping_reply)
    }

    pub unsafe fn xcb_input_change_device_key_mapping_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_input_change_device_key_mapping_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_key_mapping_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_key_mapping_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_change_device_key_mapping_sizeof)
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
    pub unsafe fn xcb_input_change_device_key_mapping_checked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        first_keycode: xcb_input_key_code_t,
        keysyms_per_keycode: u8,
        keycode_count: u8,
        keysyms: *const xcb_keysym_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_change_device_key_mapping_checked)(
            c,
            device_id,
            first_keycode,
            keysyms_per_keycode,
            keycode_count,
            keysyms,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_key_mapping_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_key_mapping_checked(&self) -> bool {
        has_sym!(self, xcb_input_change_device_key_mapping_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_change_device_key_mapping(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        first_keycode: xcb_input_key_code_t,
        keysyms_per_keycode: u8,
        keycode_count: u8,
        keysyms: *const xcb_keysym_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_change_device_key_mapping)(
            c,
            device_id,
            first_keycode,
            keysyms_per_keycode,
            keycode_count,
            keysyms,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_key_mapping` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_key_mapping(&self) -> bool {
        has_sym!(self, xcb_input_change_device_key_mapping)
    }

    pub unsafe fn xcb_input_change_device_key_mapping_keysyms(
        &self,
        r: *const xcb_input_change_device_key_mapping_request_t,
    ) -> *mut xcb_keysym_t {
        sym!(self, xcb_input_change_device_key_mapping_keysyms)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_key_mapping_keysyms` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_key_mapping_keysyms(&self) -> bool {
        has_sym!(self, xcb_input_change_device_key_mapping_keysyms)
    }

    pub unsafe fn xcb_input_change_device_key_mapping_keysyms_length(
        &self,
        r: *const xcb_input_change_device_key_mapping_request_t,
    ) -> c_int {
        sym!(self, xcb_input_change_device_key_mapping_keysyms_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_key_mapping_keysyms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_key_mapping_keysyms_length(&self) -> bool {
        has_sym!(self, xcb_input_change_device_key_mapping_keysyms_length)
    }

    pub unsafe fn xcb_input_change_device_key_mapping_keysyms_end(
        &self,
        r: *const xcb_input_change_device_key_mapping_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_change_device_key_mapping_keysyms_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_key_mapping_keysyms_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_key_mapping_keysyms_end(&self) -> bool {
        has_sym!(self, xcb_input_change_device_key_mapping_keysyms_end)
    }

    pub unsafe fn xcb_input_get_device_modifier_mapping_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_input_get_device_modifier_mapping_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_modifier_mapping_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_modifier_mapping_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_get_device_modifier_mapping_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_get_device_modifier_mapping(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_get_device_modifier_mapping_cookie_t {
        sym!(self, xcb_input_get_device_modifier_mapping)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_modifier_mapping` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_modifier_mapping(&self) -> bool {
        has_sym!(self, xcb_input_get_device_modifier_mapping)
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
    pub unsafe fn xcb_input_get_device_modifier_mapping_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_get_device_modifier_mapping_cookie_t {
        sym!(self, xcb_input_get_device_modifier_mapping_unchecked)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_modifier_mapping_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_modifier_mapping_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_get_device_modifier_mapping_unchecked)
    }

    pub unsafe fn xcb_input_get_device_modifier_mapping_keymaps(
        &self,
        r: *const xcb_input_get_device_modifier_mapping_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_input_get_device_modifier_mapping_keymaps)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_modifier_mapping_keymaps` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_modifier_mapping_keymaps(&self) -> bool {
        has_sym!(self, xcb_input_get_device_modifier_mapping_keymaps)
    }

    pub unsafe fn xcb_input_get_device_modifier_mapping_keymaps_length(
        &self,
        r: *const xcb_input_get_device_modifier_mapping_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_get_device_modifier_mapping_keymaps_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_modifier_mapping_keymaps_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_modifier_mapping_keymaps_length(&self) -> bool {
        has_sym!(self, xcb_input_get_device_modifier_mapping_keymaps_length)
    }

    pub unsafe fn xcb_input_get_device_modifier_mapping_keymaps_end(
        &self,
        r: *const xcb_input_get_device_modifier_mapping_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_get_device_modifier_mapping_keymaps_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_modifier_mapping_keymaps_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_modifier_mapping_keymaps_end(&self) -> bool {
        has_sym!(self, xcb_input_get_device_modifier_mapping_keymaps_end)
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
     * xcb_input_get_device_modifier_mapping_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_get_device_modifier_mapping_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_get_device_modifier_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_get_device_modifier_mapping_reply_t {
        sym!(self, xcb_input_get_device_modifier_mapping_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_modifier_mapping_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_modifier_mapping_reply(&self) -> bool {
        has_sym!(self, xcb_input_get_device_modifier_mapping_reply)
    }

    pub unsafe fn xcb_input_set_device_modifier_mapping_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_input_set_device_modifier_mapping_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_modifier_mapping_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_modifier_mapping_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_set_device_modifier_mapping_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_set_device_modifier_mapping(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        keycodes_per_modifier: u8,
        keymaps: *const u8,
    ) -> xcb_input_set_device_modifier_mapping_cookie_t {
        sym!(self, xcb_input_set_device_modifier_mapping)(
            c,
            device_id,
            keycodes_per_modifier,
            keymaps,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_modifier_mapping` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_modifier_mapping(&self) -> bool {
        has_sym!(self, xcb_input_set_device_modifier_mapping)
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
    pub unsafe fn xcb_input_set_device_modifier_mapping_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        keycodes_per_modifier: u8,
        keymaps: *const u8,
    ) -> xcb_input_set_device_modifier_mapping_cookie_t {
        sym!(self, xcb_input_set_device_modifier_mapping_unchecked)(
            c,
            device_id,
            keycodes_per_modifier,
            keymaps,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_modifier_mapping_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_modifier_mapping_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_set_device_modifier_mapping_unchecked)
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
     * xcb_input_set_device_modifier_mapping_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_set_device_modifier_mapping_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_set_device_modifier_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_set_device_modifier_mapping_reply_t {
        sym!(self, xcb_input_set_device_modifier_mapping_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_modifier_mapping_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_modifier_mapping_reply(&self) -> bool {
        has_sym!(self, xcb_input_set_device_modifier_mapping_reply)
    }

    pub unsafe fn xcb_input_get_device_button_mapping_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_input_get_device_button_mapping_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_button_mapping_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_button_mapping_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_get_device_button_mapping_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_get_device_button_mapping(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_get_device_button_mapping_cookie_t {
        sym!(self, xcb_input_get_device_button_mapping)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_button_mapping` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_button_mapping(&self) -> bool {
        has_sym!(self, xcb_input_get_device_button_mapping)
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
    pub unsafe fn xcb_input_get_device_button_mapping_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_get_device_button_mapping_cookie_t {
        sym!(self, xcb_input_get_device_button_mapping_unchecked)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_button_mapping_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_button_mapping_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_get_device_button_mapping_unchecked)
    }

    pub unsafe fn xcb_input_get_device_button_mapping_map(
        &self,
        r: *const xcb_input_get_device_button_mapping_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_input_get_device_button_mapping_map)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_button_mapping_map` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_button_mapping_map(&self) -> bool {
        has_sym!(self, xcb_input_get_device_button_mapping_map)
    }

    pub unsafe fn xcb_input_get_device_button_mapping_map_length(
        &self,
        r: *const xcb_input_get_device_button_mapping_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_get_device_button_mapping_map_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_button_mapping_map_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_button_mapping_map_length(&self) -> bool {
        has_sym!(self, xcb_input_get_device_button_mapping_map_length)
    }

    pub unsafe fn xcb_input_get_device_button_mapping_map_end(
        &self,
        r: *const xcb_input_get_device_button_mapping_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_get_device_button_mapping_map_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_button_mapping_map_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_button_mapping_map_end(&self) -> bool {
        has_sym!(self, xcb_input_get_device_button_mapping_map_end)
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
     * xcb_input_get_device_button_mapping_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_get_device_button_mapping_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_get_device_button_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_get_device_button_mapping_reply_t {
        sym!(self, xcb_input_get_device_button_mapping_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_button_mapping_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_button_mapping_reply(&self) -> bool {
        has_sym!(self, xcb_input_get_device_button_mapping_reply)
    }

    pub unsafe fn xcb_input_set_device_button_mapping_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_input_set_device_button_mapping_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_button_mapping_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_button_mapping_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_set_device_button_mapping_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_set_device_button_mapping(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        map_size: u8,
        map: *const u8,
    ) -> xcb_input_set_device_button_mapping_cookie_t {
        sym!(self, xcb_input_set_device_button_mapping)(c, device_id, map_size, map)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_button_mapping` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_button_mapping(&self) -> bool {
        has_sym!(self, xcb_input_set_device_button_mapping)
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
    pub unsafe fn xcb_input_set_device_button_mapping_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        map_size: u8,
        map: *const u8,
    ) -> xcb_input_set_device_button_mapping_cookie_t {
        sym!(self, xcb_input_set_device_button_mapping_unchecked)(c, device_id, map_size, map)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_button_mapping_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_button_mapping_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_set_device_button_mapping_unchecked)
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
     * xcb_input_set_device_button_mapping_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_set_device_button_mapping_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_set_device_button_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_set_device_button_mapping_reply_t {
        sym!(self, xcb_input_set_device_button_mapping_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_button_mapping_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_button_mapping_reply(&self) -> bool {
        has_sym!(self, xcb_input_set_device_button_mapping_reply)
    }

    pub unsafe fn xcb_input_key_state_next(&self, i: *mut xcb_input_key_state_iterator_t) {
        sym!(self, xcb_input_key_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_key_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_state_next(&self) -> bool {
        has_sym!(self, xcb_input_key_state_next)
    }

    pub unsafe fn xcb_input_key_state_end(
        &self,
        i: xcb_input_key_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_key_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_key_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_state_end(&self) -> bool {
        has_sym!(self, xcb_input_key_state_end)
    }

    pub unsafe fn xcb_input_button_state_next(&self, i: *mut xcb_input_button_state_iterator_t) {
        sym!(self, xcb_input_button_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_button_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_state_next(&self) -> bool {
        has_sym!(self, xcb_input_button_state_next)
    }

    pub unsafe fn xcb_input_button_state_end(
        &self,
        i: xcb_input_button_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_button_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_button_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_state_end(&self) -> bool {
        has_sym!(self, xcb_input_button_state_end)
    }

    pub unsafe fn xcb_input_valuator_state_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_valuator_state_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_state_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_state_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_valuator_state_sizeof)
    }

    pub unsafe fn xcb_input_valuator_state_valuators(
        &self,
        r: *const xcb_input_valuator_state_t,
    ) -> *mut i32 {
        sym!(self, xcb_input_valuator_state_valuators)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_state_valuators` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_state_valuators(&self) -> bool {
        has_sym!(self, xcb_input_valuator_state_valuators)
    }

    pub unsafe fn xcb_input_valuator_state_valuators_length(
        &self,
        r: *const xcb_input_valuator_state_t,
    ) -> c_int {
        sym!(self, xcb_input_valuator_state_valuators_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_state_valuators_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_state_valuators_length(&self) -> bool {
        has_sym!(self, xcb_input_valuator_state_valuators_length)
    }

    pub unsafe fn xcb_input_valuator_state_valuators_end(
        &self,
        r: *const xcb_input_valuator_state_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_valuator_state_valuators_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_state_valuators_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_state_valuators_end(&self) -> bool {
        has_sym!(self, xcb_input_valuator_state_valuators_end)
    }

    pub unsafe fn xcb_input_valuator_state_next(
        &self,
        i: *mut xcb_input_valuator_state_iterator_t,
    ) {
        sym!(self, xcb_input_valuator_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_state_next(&self) -> bool {
        has_sym!(self, xcb_input_valuator_state_next)
    }

    pub unsafe fn xcb_input_valuator_state_end(
        &self,
        i: xcb_input_valuator_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_valuator_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_state_end(&self) -> bool {
        has_sym!(self, xcb_input_valuator_state_end)
    }

    pub unsafe fn xcb_input_input_state_data_valuator_valuators(
        &self,
        s: *const xcb_input_input_state_data_t,
    ) -> *mut i32 {
        sym!(self, xcb_input_input_state_data_valuator_valuators)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_input_state_data_valuator_valuators` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_state_data_valuator_valuators(&self) -> bool {
        has_sym!(self, xcb_input_input_state_data_valuator_valuators)
    }

    pub unsafe fn xcb_input_input_state_data_valuator_valuators_length(
        &self,
        r: *const xcb_input_input_state_t,
        s: *const xcb_input_input_state_data_t,
    ) -> c_int {
        sym!(self, xcb_input_input_state_data_valuator_valuators_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_input_state_data_valuator_valuators_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_state_data_valuator_valuators_length(&self) -> bool {
        has_sym!(self, xcb_input_input_state_data_valuator_valuators_length)
    }

    pub unsafe fn xcb_input_input_state_data_valuator_valuators_end(
        &self,
        r: *const xcb_input_input_state_t,
        s: *const xcb_input_input_state_data_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_input_state_data_valuator_valuators_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_input_state_data_valuator_valuators_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_state_data_valuator_valuators_end(&self) -> bool {
        has_sym!(self, xcb_input_input_state_data_valuator_valuators_end)
    }

    pub unsafe fn xcb_input_input_state_data_serialize(
        &self,
        _buffer: *mut *mut c_void,
        class_id: u8,
        _aux: *const xcb_input_input_state_data_t,
    ) -> c_int {
        sym!(self, xcb_input_input_state_data_serialize)(_buffer, class_id, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_input_state_data_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_state_data_serialize(&self) -> bool {
        has_sym!(self, xcb_input_input_state_data_serialize)
    }

    pub unsafe fn xcb_input_input_state_data_unpack(
        &self,
        _buffer: *const c_void,
        class_id: u8,
        _aux: *mut xcb_input_input_state_data_t,
    ) -> c_int {
        sym!(self, xcb_input_input_state_data_unpack)(_buffer, class_id, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_input_state_data_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_state_data_unpack(&self) -> bool {
        has_sym!(self, xcb_input_input_state_data_unpack)
    }

    pub unsafe fn xcb_input_input_state_data_sizeof(
        &self,
        _buffer: *const c_void,
        class_id: u8,
    ) -> c_int {
        sym!(self, xcb_input_input_state_data_sizeof)(_buffer, class_id)
    }

    /// Returns `true` iff the symbol `xcb_input_input_state_data_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_state_data_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_input_state_data_sizeof)
    }

    pub unsafe fn xcb_input_input_state_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_input_state_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_input_state_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_state_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_input_state_sizeof)
    }

    pub unsafe fn xcb_input_input_state_data(
        &self,
        r: *const xcb_input_input_state_t,
    ) -> *mut c_void {
        sym!(self, xcb_input_input_state_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_input_state_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_state_data(&self) -> bool {
        has_sym!(self, xcb_input_input_state_data)
    }

    pub unsafe fn xcb_input_input_state_next(&self, i: *mut xcb_input_input_state_iterator_t) {
        sym!(self, xcb_input_input_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_input_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_state_next(&self) -> bool {
        has_sym!(self, xcb_input_input_state_next)
    }

    pub unsafe fn xcb_input_input_state_end(
        &self,
        i: xcb_input_input_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_input_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_input_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_state_end(&self) -> bool {
        has_sym!(self, xcb_input_input_state_end)
    }

    pub unsafe fn xcb_input_query_device_state_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_query_device_state_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_query_device_state_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_query_device_state_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_query_device_state_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_query_device_state(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_query_device_state_cookie_t {
        sym!(self, xcb_input_query_device_state)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_query_device_state` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_query_device_state(&self) -> bool {
        has_sym!(self, xcb_input_query_device_state)
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
    pub unsafe fn xcb_input_query_device_state_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_query_device_state_cookie_t {
        sym!(self, xcb_input_query_device_state_unchecked)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_query_device_state_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_query_device_state_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_query_device_state_unchecked)
    }

    pub unsafe fn xcb_input_query_device_state_classes_length(
        &self,
        r: *const xcb_input_query_device_state_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_query_device_state_classes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_query_device_state_classes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_query_device_state_classes_length(&self) -> bool {
        has_sym!(self, xcb_input_query_device_state_classes_length)
    }

    pub unsafe fn xcb_input_query_device_state_classes_iterator(
        &self,
        r: *const xcb_input_query_device_state_reply_t,
    ) -> xcb_input_input_state_iterator_t {
        sym!(self, xcb_input_query_device_state_classes_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_query_device_state_classes_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_query_device_state_classes_iterator(&self) -> bool {
        has_sym!(self, xcb_input_query_device_state_classes_iterator)
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
     * xcb_input_query_device_state_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_query_device_state_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_query_device_state_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_query_device_state_reply_t {
        sym!(self, xcb_input_query_device_state_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_query_device_state_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_query_device_state_reply(&self) -> bool {
        has_sym!(self, xcb_input_query_device_state_reply)
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
    pub unsafe fn xcb_input_device_bell_checked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        feedback_id: u8,
        feedback_class: u8,
        percent: i8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_device_bell_checked)(
            c,
            device_id,
            feedback_id,
            feedback_class,
            percent,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_device_bell_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_bell_checked(&self) -> bool {
        has_sym!(self, xcb_input_device_bell_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_device_bell(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        feedback_id: u8,
        feedback_class: u8,
        percent: i8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_device_bell)(c, device_id, feedback_id, feedback_class, percent)
    }

    /// Returns `true` iff the symbol `xcb_input_device_bell` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_bell(&self) -> bool {
        has_sym!(self, xcb_input_device_bell)
    }

    pub unsafe fn xcb_input_set_device_valuators_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_set_device_valuators_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_valuators_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_valuators_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_set_device_valuators_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_set_device_valuators(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        first_valuator: u8,
        num_valuators: u8,
        valuators: *const i32,
    ) -> xcb_input_set_device_valuators_cookie_t {
        sym!(self, xcb_input_set_device_valuators)(
            c,
            device_id,
            first_valuator,
            num_valuators,
            valuators,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_valuators` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_valuators(&self) -> bool {
        has_sym!(self, xcb_input_set_device_valuators)
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
    pub unsafe fn xcb_input_set_device_valuators_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        first_valuator: u8,
        num_valuators: u8,
        valuators: *const i32,
    ) -> xcb_input_set_device_valuators_cookie_t {
        sym!(self, xcb_input_set_device_valuators_unchecked)(
            c,
            device_id,
            first_valuator,
            num_valuators,
            valuators,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_valuators_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_valuators_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_set_device_valuators_unchecked)
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
     * xcb_input_set_device_valuators_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_set_device_valuators_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_set_device_valuators_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_set_device_valuators_reply_t {
        sym!(self, xcb_input_set_device_valuators_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_valuators_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_valuators_reply(&self) -> bool {
        has_sym!(self, xcb_input_set_device_valuators_reply)
    }

    pub unsafe fn xcb_input_device_resolution_state_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_device_resolution_state_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_state_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_state_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_state_sizeof)
    }

    pub unsafe fn xcb_input_device_resolution_state_resolution_values(
        &self,
        r: *const xcb_input_device_resolution_state_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_device_resolution_state_resolution_values)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_state_resolution_values` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_state_resolution_values(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_state_resolution_values)
    }

    pub unsafe fn xcb_input_device_resolution_state_resolution_values_length(
        &self,
        r: *const xcb_input_device_resolution_state_t,
    ) -> c_int {
        sym!(
            self,
            xcb_input_device_resolution_state_resolution_values_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_state_resolution_values_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_state_resolution_values_length(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_resolution_state_resolution_values_length
        )
    }

    pub unsafe fn xcb_input_device_resolution_state_resolution_values_end(
        &self,
        r: *const xcb_input_device_resolution_state_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_input_device_resolution_state_resolution_values_end
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_state_resolution_values_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_state_resolution_values_end(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_resolution_state_resolution_values_end
        )
    }

    pub unsafe fn xcb_input_device_resolution_state_resolution_min(
        &self,
        r: *const xcb_input_device_resolution_state_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_device_resolution_state_resolution_min)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_state_resolution_min` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_state_resolution_min(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_state_resolution_min)
    }

    pub unsafe fn xcb_input_device_resolution_state_resolution_min_length(
        &self,
        r: *const xcb_input_device_resolution_state_t,
    ) -> c_int {
        sym!(
            self,
            xcb_input_device_resolution_state_resolution_min_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_state_resolution_min_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_state_resolution_min_length(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_resolution_state_resolution_min_length
        )
    }

    pub unsafe fn xcb_input_device_resolution_state_resolution_min_end(
        &self,
        r: *const xcb_input_device_resolution_state_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_resolution_state_resolution_min_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_state_resolution_min_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_state_resolution_min_end(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_state_resolution_min_end)
    }

    pub unsafe fn xcb_input_device_resolution_state_resolution_max(
        &self,
        r: *const xcb_input_device_resolution_state_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_device_resolution_state_resolution_max)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_state_resolution_max` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_state_resolution_max(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_state_resolution_max)
    }

    pub unsafe fn xcb_input_device_resolution_state_resolution_max_length(
        &self,
        r: *const xcb_input_device_resolution_state_t,
    ) -> c_int {
        sym!(
            self,
            xcb_input_device_resolution_state_resolution_max_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_state_resolution_max_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_state_resolution_max_length(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_resolution_state_resolution_max_length
        )
    }

    pub unsafe fn xcb_input_device_resolution_state_resolution_max_end(
        &self,
        r: *const xcb_input_device_resolution_state_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_resolution_state_resolution_max_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_state_resolution_max_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_state_resolution_max_end(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_state_resolution_max_end)
    }

    pub unsafe fn xcb_input_device_resolution_state_next(
        &self,
        i: *mut xcb_input_device_resolution_state_iterator_t,
    ) {
        sym!(self, xcb_input_device_resolution_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_state_next(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_state_next)
    }

    pub unsafe fn xcb_input_device_resolution_state_end(
        &self,
        i: xcb_input_device_resolution_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_resolution_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_state_end(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_state_end)
    }

    pub unsafe fn xcb_input_device_abs_calib_state_next(
        &self,
        i: *mut xcb_input_device_abs_calib_state_iterator_t,
    ) {
        sym!(self, xcb_input_device_abs_calib_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_abs_calib_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_abs_calib_state_next(&self) -> bool {
        has_sym!(self, xcb_input_device_abs_calib_state_next)
    }

    pub unsafe fn xcb_input_device_abs_calib_state_end(
        &self,
        i: xcb_input_device_abs_calib_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_abs_calib_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_abs_calib_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_abs_calib_state_end(&self) -> bool {
        has_sym!(self, xcb_input_device_abs_calib_state_end)
    }

    pub unsafe fn xcb_input_device_abs_area_state_next(
        &self,
        i: *mut xcb_input_device_abs_area_state_iterator_t,
    ) {
        sym!(self, xcb_input_device_abs_area_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_abs_area_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_abs_area_state_next(&self) -> bool {
        has_sym!(self, xcb_input_device_abs_area_state_next)
    }

    pub unsafe fn xcb_input_device_abs_area_state_end(
        &self,
        i: xcb_input_device_abs_area_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_abs_area_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_abs_area_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_abs_area_state_end(&self) -> bool {
        has_sym!(self, xcb_input_device_abs_area_state_end)
    }

    pub unsafe fn xcb_input_device_core_state_next(
        &self,
        i: *mut xcb_input_device_core_state_iterator_t,
    ) {
        sym!(self, xcb_input_device_core_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_core_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_core_state_next(&self) -> bool {
        has_sym!(self, xcb_input_device_core_state_next)
    }

    pub unsafe fn xcb_input_device_core_state_end(
        &self,
        i: xcb_input_device_core_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_core_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_core_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_core_state_end(&self) -> bool {
        has_sym!(self, xcb_input_device_core_state_end)
    }

    pub unsafe fn xcb_input_device_enable_state_next(
        &self,
        i: *mut xcb_input_device_enable_state_iterator_t,
    ) {
        sym!(self, xcb_input_device_enable_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_enable_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_enable_state_next(&self) -> bool {
        has_sym!(self, xcb_input_device_enable_state_next)
    }

    pub unsafe fn xcb_input_device_enable_state_end(
        &self,
        i: xcb_input_device_enable_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_enable_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_enable_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_enable_state_end(&self) -> bool {
        has_sym!(self, xcb_input_device_enable_state_end)
    }

    pub unsafe fn xcb_input_device_state_data_resolution_resolution_values(
        &self,
        s: *const xcb_input_device_state_data_t,
    ) -> *mut u32 {
        sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_values
        )(s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data_resolution_resolution_values` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data_resolution_resolution_values(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_values
        )
    }

    pub unsafe fn xcb_input_device_state_data_resolution_resolution_values_length(
        &self,
        r: *const xcb_input_device_state_t,
        s: *const xcb_input_device_state_data_t,
    ) -> c_int {
        sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_values_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data_resolution_resolution_values_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data_resolution_resolution_values_length(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_values_length
        )
    }

    pub unsafe fn xcb_input_device_state_data_resolution_resolution_values_end(
        &self,
        r: *const xcb_input_device_state_t,
        s: *const xcb_input_device_state_data_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_values_end
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data_resolution_resolution_values_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data_resolution_resolution_values_end(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_values_end
        )
    }

    pub unsafe fn xcb_input_device_state_data_resolution_resolution_min(
        &self,
        s: *const xcb_input_device_state_data_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_device_state_data_resolution_resolution_min)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data_resolution_resolution_min` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data_resolution_resolution_min(&self) -> bool {
        has_sym!(self, xcb_input_device_state_data_resolution_resolution_min)
    }

    pub unsafe fn xcb_input_device_state_data_resolution_resolution_min_length(
        &self,
        r: *const xcb_input_device_state_t,
        s: *const xcb_input_device_state_data_t,
    ) -> c_int {
        sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_min_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data_resolution_resolution_min_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data_resolution_resolution_min_length(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_min_length
        )
    }

    pub unsafe fn xcb_input_device_state_data_resolution_resolution_min_end(
        &self,
        r: *const xcb_input_device_state_t,
        s: *const xcb_input_device_state_data_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_min_end
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data_resolution_resolution_min_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data_resolution_resolution_min_end(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_min_end
        )
    }

    pub unsafe fn xcb_input_device_state_data_resolution_resolution_max(
        &self,
        s: *const xcb_input_device_state_data_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_device_state_data_resolution_resolution_max)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data_resolution_resolution_max` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data_resolution_resolution_max(&self) -> bool {
        has_sym!(self, xcb_input_device_state_data_resolution_resolution_max)
    }

    pub unsafe fn xcb_input_device_state_data_resolution_resolution_max_length(
        &self,
        r: *const xcb_input_device_state_t,
        s: *const xcb_input_device_state_data_t,
    ) -> c_int {
        sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_max_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data_resolution_resolution_max_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data_resolution_resolution_max_length(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_max_length
        )
    }

    pub unsafe fn xcb_input_device_state_data_resolution_resolution_max_end(
        &self,
        r: *const xcb_input_device_state_t,
        s: *const xcb_input_device_state_data_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_max_end
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data_resolution_resolution_max_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data_resolution_resolution_max_end(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_max_end
        )
    }

    pub unsafe fn xcb_input_device_state_data_serialize(
        &self,
        _buffer: *mut *mut c_void,
        control_id: u16,
        _aux: *const xcb_input_device_state_data_t,
    ) -> c_int {
        sym!(self, xcb_input_device_state_data_serialize)(_buffer, control_id, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data_serialize(&self) -> bool {
        has_sym!(self, xcb_input_device_state_data_serialize)
    }

    pub unsafe fn xcb_input_device_state_data_unpack(
        &self,
        _buffer: *const c_void,
        control_id: u16,
        _aux: *mut xcb_input_device_state_data_t,
    ) -> c_int {
        sym!(self, xcb_input_device_state_data_unpack)(_buffer, control_id, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data_unpack(&self) -> bool {
        has_sym!(self, xcb_input_device_state_data_unpack)
    }

    pub unsafe fn xcb_input_device_state_data_sizeof(
        &self,
        _buffer: *const c_void,
        control_id: u16,
    ) -> c_int {
        sym!(self, xcb_input_device_state_data_sizeof)(_buffer, control_id)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_device_state_data_sizeof)
    }

    pub unsafe fn xcb_input_device_state_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_device_state_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_device_state_sizeof)
    }

    pub unsafe fn xcb_input_device_state_data(
        &self,
        r: *const xcb_input_device_state_t,
    ) -> *mut c_void {
        sym!(self, xcb_input_device_state_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data(&self) -> bool {
        has_sym!(self, xcb_input_device_state_data)
    }

    pub unsafe fn xcb_input_device_state_next(&self, i: *mut xcb_input_device_state_iterator_t) {
        sym!(self, xcb_input_device_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_next(&self) -> bool {
        has_sym!(self, xcb_input_device_state_next)
    }

    pub unsafe fn xcb_input_device_state_end(
        &self,
        i: xcb_input_device_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_end(&self) -> bool {
        has_sym!(self, xcb_input_device_state_end)
    }

    pub unsafe fn xcb_input_get_device_control_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_get_device_control_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_control_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_control_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_get_device_control_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_get_device_control(
        &self,
        c: *mut xcb_connection_t,
        control_id: u16,
        device_id: u8,
    ) -> xcb_input_get_device_control_cookie_t {
        sym!(self, xcb_input_get_device_control)(c, control_id, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_control` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_control(&self) -> bool {
        has_sym!(self, xcb_input_get_device_control)
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
    pub unsafe fn xcb_input_get_device_control_unchecked(
        &self,
        c: *mut xcb_connection_t,
        control_id: u16,
        device_id: u8,
    ) -> xcb_input_get_device_control_cookie_t {
        sym!(self, xcb_input_get_device_control_unchecked)(c, control_id, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_control_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_control_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_get_device_control_unchecked)
    }

    pub unsafe fn xcb_input_get_device_control_control(
        &self,
        r: *const xcb_input_get_device_control_reply_t,
    ) -> *mut xcb_input_device_state_t {
        sym!(self, xcb_input_get_device_control_control)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_control_control` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_control_control(&self) -> bool {
        has_sym!(self, xcb_input_get_device_control_control)
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
     * xcb_input_get_device_control_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_get_device_control_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_get_device_control_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_get_device_control_reply_t {
        sym!(self, xcb_input_get_device_control_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_control_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_control_reply(&self) -> bool {
        has_sym!(self, xcb_input_get_device_control_reply)
    }

    pub unsafe fn xcb_input_device_resolution_ctl_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_device_resolution_ctl_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_ctl_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_ctl_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_ctl_sizeof)
    }

    pub unsafe fn xcb_input_device_resolution_ctl_resolution_values(
        &self,
        r: *const xcb_input_device_resolution_ctl_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_device_resolution_ctl_resolution_values)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_ctl_resolution_values` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_ctl_resolution_values(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_ctl_resolution_values)
    }

    pub unsafe fn xcb_input_device_resolution_ctl_resolution_values_length(
        &self,
        r: *const xcb_input_device_resolution_ctl_t,
    ) -> c_int {
        sym!(
            self,
            xcb_input_device_resolution_ctl_resolution_values_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_ctl_resolution_values_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_ctl_resolution_values_length(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_resolution_ctl_resolution_values_length
        )
    }

    pub unsafe fn xcb_input_device_resolution_ctl_resolution_values_end(
        &self,
        r: *const xcb_input_device_resolution_ctl_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_resolution_ctl_resolution_values_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_ctl_resolution_values_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_ctl_resolution_values_end(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_ctl_resolution_values_end)
    }

    pub unsafe fn xcb_input_device_resolution_ctl_next(
        &self,
        i: *mut xcb_input_device_resolution_ctl_iterator_t,
    ) {
        sym!(self, xcb_input_device_resolution_ctl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_ctl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_ctl_next(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_ctl_next)
    }

    pub unsafe fn xcb_input_device_resolution_ctl_end(
        &self,
        i: xcb_input_device_resolution_ctl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_resolution_ctl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_ctl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_ctl_end(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_ctl_end)
    }

    pub unsafe fn xcb_input_device_abs_calib_ctl_next(
        &self,
        i: *mut xcb_input_device_abs_calib_ctl_iterator_t,
    ) {
        sym!(self, xcb_input_device_abs_calib_ctl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_abs_calib_ctl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_abs_calib_ctl_next(&self) -> bool {
        has_sym!(self, xcb_input_device_abs_calib_ctl_next)
    }

    pub unsafe fn xcb_input_device_abs_calib_ctl_end(
        &self,
        i: xcb_input_device_abs_calib_ctl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_abs_calib_ctl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_abs_calib_ctl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_abs_calib_ctl_end(&self) -> bool {
        has_sym!(self, xcb_input_device_abs_calib_ctl_end)
    }

    pub unsafe fn xcb_input_device_abs_area_ctrl_next(
        &self,
        i: *mut xcb_input_device_abs_area_ctrl_iterator_t,
    ) {
        sym!(self, xcb_input_device_abs_area_ctrl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_abs_area_ctrl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_abs_area_ctrl_next(&self) -> bool {
        has_sym!(self, xcb_input_device_abs_area_ctrl_next)
    }

    pub unsafe fn xcb_input_device_abs_area_ctrl_end(
        &self,
        i: xcb_input_device_abs_area_ctrl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_abs_area_ctrl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_abs_area_ctrl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_abs_area_ctrl_end(&self) -> bool {
        has_sym!(self, xcb_input_device_abs_area_ctrl_end)
    }

    pub unsafe fn xcb_input_device_core_ctrl_next(
        &self,
        i: *mut xcb_input_device_core_ctrl_iterator_t,
    ) {
        sym!(self, xcb_input_device_core_ctrl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_core_ctrl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_core_ctrl_next(&self) -> bool {
        has_sym!(self, xcb_input_device_core_ctrl_next)
    }

    pub unsafe fn xcb_input_device_core_ctrl_end(
        &self,
        i: xcb_input_device_core_ctrl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_core_ctrl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_core_ctrl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_core_ctrl_end(&self) -> bool {
        has_sym!(self, xcb_input_device_core_ctrl_end)
    }

    pub unsafe fn xcb_input_device_enable_ctrl_next(
        &self,
        i: *mut xcb_input_device_enable_ctrl_iterator_t,
    ) {
        sym!(self, xcb_input_device_enable_ctrl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_enable_ctrl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_enable_ctrl_next(&self) -> bool {
        has_sym!(self, xcb_input_device_enable_ctrl_next)
    }

    pub unsafe fn xcb_input_device_enable_ctrl_end(
        &self,
        i: xcb_input_device_enable_ctrl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_enable_ctrl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_enable_ctrl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_enable_ctrl_end(&self) -> bool {
        has_sym!(self, xcb_input_device_enable_ctrl_end)
    }

    pub unsafe fn xcb_input_device_ctl_data_resolution_resolution_values(
        &self,
        s: *const xcb_input_device_ctl_data_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_device_ctl_data_resolution_resolution_values)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_ctl_data_resolution_resolution_values` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_ctl_data_resolution_resolution_values(&self) -> bool {
        has_sym!(self, xcb_input_device_ctl_data_resolution_resolution_values)
    }

    pub unsafe fn xcb_input_device_ctl_data_resolution_resolution_values_length(
        &self,
        r: *const xcb_input_device_ctl_t,
        s: *const xcb_input_device_ctl_data_t,
    ) -> c_int {
        sym!(
            self,
            xcb_input_device_ctl_data_resolution_resolution_values_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_ctl_data_resolution_resolution_values_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_ctl_data_resolution_resolution_values_length(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_ctl_data_resolution_resolution_values_length
        )
    }

    pub unsafe fn xcb_input_device_ctl_data_resolution_resolution_values_end(
        &self,
        r: *const xcb_input_device_ctl_t,
        s: *const xcb_input_device_ctl_data_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_input_device_ctl_data_resolution_resolution_values_end
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_ctl_data_resolution_resolution_values_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_ctl_data_resolution_resolution_values_end(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_ctl_data_resolution_resolution_values_end
        )
    }

    pub unsafe fn xcb_input_device_ctl_data_serialize(
        &self,
        _buffer: *mut *mut c_void,
        control_id: u16,
        _aux: *const xcb_input_device_ctl_data_t,
    ) -> c_int {
        sym!(self, xcb_input_device_ctl_data_serialize)(_buffer, control_id, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_device_ctl_data_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_ctl_data_serialize(&self) -> bool {
        has_sym!(self, xcb_input_device_ctl_data_serialize)
    }

    pub unsafe fn xcb_input_device_ctl_data_unpack(
        &self,
        _buffer: *const c_void,
        control_id: u16,
        _aux: *mut xcb_input_device_ctl_data_t,
    ) -> c_int {
        sym!(self, xcb_input_device_ctl_data_unpack)(_buffer, control_id, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_device_ctl_data_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_ctl_data_unpack(&self) -> bool {
        has_sym!(self, xcb_input_device_ctl_data_unpack)
    }

    pub unsafe fn xcb_input_device_ctl_data_sizeof(
        &self,
        _buffer: *const c_void,
        control_id: u16,
    ) -> c_int {
        sym!(self, xcb_input_device_ctl_data_sizeof)(_buffer, control_id)
    }

    /// Returns `true` iff the symbol `xcb_input_device_ctl_data_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_ctl_data_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_device_ctl_data_sizeof)
    }

    pub unsafe fn xcb_input_device_ctl_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_device_ctl_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_device_ctl_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_ctl_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_device_ctl_sizeof)
    }

    pub unsafe fn xcb_input_device_ctl_data(
        &self,
        r: *const xcb_input_device_ctl_t,
    ) -> *mut c_void {
        sym!(self, xcb_input_device_ctl_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_ctl_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_ctl_data(&self) -> bool {
        has_sym!(self, xcb_input_device_ctl_data)
    }

    pub unsafe fn xcb_input_device_ctl_next(&self, i: *mut xcb_input_device_ctl_iterator_t) {
        sym!(self, xcb_input_device_ctl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_ctl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_ctl_next(&self) -> bool {
        has_sym!(self, xcb_input_device_ctl_next)
    }

    pub unsafe fn xcb_input_device_ctl_end(
        &self,
        i: xcb_input_device_ctl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_ctl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_ctl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_ctl_end(&self) -> bool {
        has_sym!(self, xcb_input_device_ctl_end)
    }

    pub unsafe fn xcb_input_change_device_control_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_change_device_control_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_control_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_control_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_change_device_control_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_change_device_control(
        &self,
        c: *mut xcb_connection_t,
        control_id: u16,
        device_id: u8,
        control: *mut xcb_input_device_ctl_t,
    ) -> xcb_input_change_device_control_cookie_t {
        sym!(self, xcb_input_change_device_control)(c, control_id, device_id, control)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_control` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_control(&self) -> bool {
        has_sym!(self, xcb_input_change_device_control)
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
    pub unsafe fn xcb_input_change_device_control_unchecked(
        &self,
        c: *mut xcb_connection_t,
        control_id: u16,
        device_id: u8,
        control: *mut xcb_input_device_ctl_t,
    ) -> xcb_input_change_device_control_cookie_t {
        sym!(self, xcb_input_change_device_control_unchecked)(c, control_id, device_id, control)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_control_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_control_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_change_device_control_unchecked)
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
     * xcb_input_change_device_control_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_change_device_control_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_change_device_control_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_change_device_control_reply_t {
        sym!(self, xcb_input_change_device_control_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_control_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_control_reply(&self) -> bool {
        has_sym!(self, xcb_input_change_device_control_reply)
    }

    pub unsafe fn xcb_input_list_device_properties_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_list_device_properties_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_list_device_properties_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_device_properties_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_list_device_properties_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_list_device_properties(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_list_device_properties_cookie_t {
        sym!(self, xcb_input_list_device_properties)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_list_device_properties` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_device_properties(&self) -> bool {
        has_sym!(self, xcb_input_list_device_properties)
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
    pub unsafe fn xcb_input_list_device_properties_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_list_device_properties_cookie_t {
        sym!(self, xcb_input_list_device_properties_unchecked)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_list_device_properties_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_device_properties_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_list_device_properties_unchecked)
    }

    pub unsafe fn xcb_input_list_device_properties_atoms(
        &self,
        r: *const xcb_input_list_device_properties_reply_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_input_list_device_properties_atoms)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_list_device_properties_atoms` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_device_properties_atoms(&self) -> bool {
        has_sym!(self, xcb_input_list_device_properties_atoms)
    }

    pub unsafe fn xcb_input_list_device_properties_atoms_length(
        &self,
        r: *const xcb_input_list_device_properties_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_list_device_properties_atoms_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_list_device_properties_atoms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_device_properties_atoms_length(&self) -> bool {
        has_sym!(self, xcb_input_list_device_properties_atoms_length)
    }

    pub unsafe fn xcb_input_list_device_properties_atoms_end(
        &self,
        r: *const xcb_input_list_device_properties_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_list_device_properties_atoms_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_list_device_properties_atoms_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_device_properties_atoms_end(&self) -> bool {
        has_sym!(self, xcb_input_list_device_properties_atoms_end)
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
     * xcb_input_list_device_properties_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_list_device_properties_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_list_device_properties_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_list_device_properties_reply_t {
        sym!(self, xcb_input_list_device_properties_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_list_device_properties_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_device_properties_reply(&self) -> bool {
        has_sym!(self, xcb_input_list_device_properties_reply)
    }

    pub unsafe fn xcb_input_change_device_property_items_data_8(
        &self,
        s: *const xcb_input_change_device_property_items_t,
    ) -> *mut u8 {
        sym!(self, xcb_input_change_device_property_items_data_8)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items_data_8` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items_data_8(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items_data_8)
    }

    pub unsafe fn xcb_input_change_device_property_items_data_8_length(
        &self,
        r: *const xcb_input_change_device_property_request_t,
        s: *const xcb_input_change_device_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_change_device_property_items_data_8_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items_data_8_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items_data_8_length(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items_data_8_length)
    }

    pub unsafe fn xcb_input_change_device_property_items_data_8_end(
        &self,
        r: *const xcb_input_change_device_property_request_t,
        s: *const xcb_input_change_device_property_items_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_change_device_property_items_data_8_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items_data_8_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items_data_8_end(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items_data_8_end)
    }

    pub unsafe fn xcb_input_change_device_property_items_data_16(
        &self,
        s: *const xcb_input_change_device_property_items_t,
    ) -> *mut u16 {
        sym!(self, xcb_input_change_device_property_items_data_16)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items_data_16` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items_data_16(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items_data_16)
    }

    pub unsafe fn xcb_input_change_device_property_items_data_16_length(
        &self,
        r: *const xcb_input_change_device_property_request_t,
        s: *const xcb_input_change_device_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_change_device_property_items_data_16_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items_data_16_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items_data_16_length(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items_data_16_length)
    }

    pub unsafe fn xcb_input_change_device_property_items_data_16_end(
        &self,
        r: *const xcb_input_change_device_property_request_t,
        s: *const xcb_input_change_device_property_items_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_change_device_property_items_data_16_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items_data_16_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items_data_16_end(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items_data_16_end)
    }

    pub unsafe fn xcb_input_change_device_property_items_data_32(
        &self,
        s: *const xcb_input_change_device_property_items_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_change_device_property_items_data_32)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items_data_32` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items_data_32(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items_data_32)
    }

    pub unsafe fn xcb_input_change_device_property_items_data_32_length(
        &self,
        r: *const xcb_input_change_device_property_request_t,
        s: *const xcb_input_change_device_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_change_device_property_items_data_32_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items_data_32_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items_data_32_length(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items_data_32_length)
    }

    pub unsafe fn xcb_input_change_device_property_items_data_32_end(
        &self,
        r: *const xcb_input_change_device_property_request_t,
        s: *const xcb_input_change_device_property_items_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_change_device_property_items_data_32_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items_data_32_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items_data_32_end(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items_data_32_end)
    }

    pub unsafe fn xcb_input_change_device_property_items_serialize(
        &self,
        _buffer: *mut *mut c_void,
        num_items: u32,
        format: u8,
        _aux: *const xcb_input_change_device_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_change_device_property_items_serialize)(
            _buffer, num_items, format, _aux,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items_serialize(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items_serialize)
    }

    pub unsafe fn xcb_input_change_device_property_items_unpack(
        &self,
        _buffer: *const c_void,
        num_items: u32,
        format: u8,
        _aux: *mut xcb_input_change_device_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_change_device_property_items_unpack)(_buffer, num_items, format, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items_unpack(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items_unpack)
    }

    pub unsafe fn xcb_input_change_device_property_items_sizeof(
        &self,
        _buffer: *const c_void,
        num_items: u32,
        format: u8,
    ) -> c_int {
        sym!(self, xcb_input_change_device_property_items_sizeof)(_buffer, num_items, format)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items_sizeof)
    }

    pub unsafe fn xcb_input_change_device_property_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_change_device_property_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_sizeof)
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
    pub unsafe fn xcb_input_change_device_property_checked(
        &self,
        c: *mut xcb_connection_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        device_id: u8,
        format: u8,
        mode: u8,
        num_items: u32,
        items: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_change_device_property_checked)(
            c, property, type_, device_id, format, mode, num_items, items,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_checked(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_change_device_property(
        &self,
        c: *mut xcb_connection_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        device_id: u8,
        format: u8,
        mode: u8,
        num_items: u32,
        items: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_change_device_property)(
            c, property, type_, device_id, format, mode, num_items, items,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property)
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
    pub unsafe fn xcb_input_change_device_property_aux_checked(
        &self,
        c: *mut xcb_connection_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        device_id: u8,
        format: u8,
        mode: u8,
        num_items: u32,
        items: *const xcb_input_change_device_property_items_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_change_device_property_aux_checked)(
            c, property, type_, device_id, format, mode, num_items, items,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_aux_checked(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_aux_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_change_device_property_aux(
        &self,
        c: *mut xcb_connection_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        device_id: u8,
        format: u8,
        mode: u8,
        num_items: u32,
        items: *const xcb_input_change_device_property_items_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_change_device_property_aux)(
            c, property, type_, device_id, format, mode, num_items, items,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_aux(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_aux)
    }

    pub unsafe fn xcb_input_change_device_property_items(
        &self,
        r: *const xcb_input_change_device_property_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_input_change_device_property_items)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items)
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
    pub unsafe fn xcb_input_delete_device_property_checked(
        &self,
        c: *mut xcb_connection_t,
        property: xcb_atom_t,
        device_id: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_delete_device_property_checked)(c, property, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_delete_device_property_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_delete_device_property_checked(&self) -> bool {
        has_sym!(self, xcb_input_delete_device_property_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_delete_device_property(
        &self,
        c: *mut xcb_connection_t,
        property: xcb_atom_t,
        device_id: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_delete_device_property)(c, property, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_delete_device_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_delete_device_property(&self) -> bool {
        has_sym!(self, xcb_input_delete_device_property)
    }

    pub unsafe fn xcb_input_get_device_property_items_data_8(
        &self,
        s: *const xcb_input_get_device_property_items_t,
    ) -> *mut u8 {
        sym!(self, xcb_input_get_device_property_items_data_8)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items_data_8` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items_data_8(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items_data_8)
    }

    pub unsafe fn xcb_input_get_device_property_items_data_8_length(
        &self,
        r: *const xcb_input_get_device_property_reply_t,
        s: *const xcb_input_get_device_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_get_device_property_items_data_8_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items_data_8_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items_data_8_length(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items_data_8_length)
    }

    pub unsafe fn xcb_input_get_device_property_items_data_8_end(
        &self,
        r: *const xcb_input_get_device_property_reply_t,
        s: *const xcb_input_get_device_property_items_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_get_device_property_items_data_8_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items_data_8_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items_data_8_end(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items_data_8_end)
    }

    pub unsafe fn xcb_input_get_device_property_items_data_16(
        &self,
        s: *const xcb_input_get_device_property_items_t,
    ) -> *mut u16 {
        sym!(self, xcb_input_get_device_property_items_data_16)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items_data_16` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items_data_16(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items_data_16)
    }

    pub unsafe fn xcb_input_get_device_property_items_data_16_length(
        &self,
        r: *const xcb_input_get_device_property_reply_t,
        s: *const xcb_input_get_device_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_get_device_property_items_data_16_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items_data_16_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items_data_16_length(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items_data_16_length)
    }

    pub unsafe fn xcb_input_get_device_property_items_data_16_end(
        &self,
        r: *const xcb_input_get_device_property_reply_t,
        s: *const xcb_input_get_device_property_items_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_get_device_property_items_data_16_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items_data_16_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items_data_16_end(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items_data_16_end)
    }

    pub unsafe fn xcb_input_get_device_property_items_data_32(
        &self,
        s: *const xcb_input_get_device_property_items_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_get_device_property_items_data_32)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items_data_32` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items_data_32(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items_data_32)
    }

    pub unsafe fn xcb_input_get_device_property_items_data_32_length(
        &self,
        r: *const xcb_input_get_device_property_reply_t,
        s: *const xcb_input_get_device_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_get_device_property_items_data_32_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items_data_32_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items_data_32_length(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items_data_32_length)
    }

    pub unsafe fn xcb_input_get_device_property_items_data_32_end(
        &self,
        r: *const xcb_input_get_device_property_reply_t,
        s: *const xcb_input_get_device_property_items_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_get_device_property_items_data_32_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items_data_32_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items_data_32_end(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items_data_32_end)
    }

    pub unsafe fn xcb_input_get_device_property_items_serialize(
        &self,
        _buffer: *mut *mut c_void,
        num_items: u32,
        format: u8,
        _aux: *const xcb_input_get_device_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_get_device_property_items_serialize)(_buffer, num_items, format, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items_serialize(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items_serialize)
    }

    pub unsafe fn xcb_input_get_device_property_items_unpack(
        &self,
        _buffer: *const c_void,
        num_items: u32,
        format: u8,
        _aux: *mut xcb_input_get_device_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_get_device_property_items_unpack)(_buffer, num_items, format, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items_unpack(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items_unpack)
    }

    pub unsafe fn xcb_input_get_device_property_items_sizeof(
        &self,
        _buffer: *const c_void,
        num_items: u32,
        format: u8,
    ) -> c_int {
        sym!(self, xcb_input_get_device_property_items_sizeof)(_buffer, num_items, format)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items_sizeof)
    }

    pub unsafe fn xcb_input_get_device_property_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_get_device_property_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_get_device_property(
        &self,
        c: *mut xcb_connection_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        offset: u32,
        len: u32,
        device_id: u8,
        delete: u8,
    ) -> xcb_input_get_device_property_cookie_t {
        sym!(self, xcb_input_get_device_property)(
            c, property, type_, offset, len, device_id, delete,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property)
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
    pub unsafe fn xcb_input_get_device_property_unchecked(
        &self,
        c: *mut xcb_connection_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        offset: u32,
        len: u32,
        device_id: u8,
        delete: u8,
    ) -> xcb_input_get_device_property_cookie_t {
        sym!(self, xcb_input_get_device_property_unchecked)(
            c, property, type_, offset, len, device_id, delete,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_unchecked)
    }

    pub unsafe fn xcb_input_get_device_property_items(
        &self,
        r: *const xcb_input_get_device_property_reply_t,
    ) -> *mut c_void {
        sym!(self, xcb_input_get_device_property_items)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items)
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
     * xcb_input_get_device_property_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_get_device_property_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_get_device_property_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_get_device_property_reply_t {
        sym!(self, xcb_input_get_device_property_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_reply(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_reply)
    }

    pub unsafe fn xcb_input_group_info_next(&self, i: *mut xcb_input_group_info_iterator_t) {
        sym!(self, xcb_input_group_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_group_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_group_info_next(&self) -> bool {
        has_sym!(self, xcb_input_group_info_next)
    }

    pub unsafe fn xcb_input_group_info_end(
        &self,
        i: xcb_input_group_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_group_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_group_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_group_info_end(&self) -> bool {
        has_sym!(self, xcb_input_group_info_end)
    }

    pub unsafe fn xcb_input_modifier_info_next(&self, i: *mut xcb_input_modifier_info_iterator_t) {
        sym!(self, xcb_input_modifier_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_modifier_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_modifier_info_next(&self) -> bool {
        has_sym!(self, xcb_input_modifier_info_next)
    }

    pub unsafe fn xcb_input_modifier_info_end(
        &self,
        i: xcb_input_modifier_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_modifier_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_modifier_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_modifier_info_end(&self) -> bool {
        has_sym!(self, xcb_input_modifier_info_end)
    }

    pub unsafe fn xcb_input_xi_query_pointer_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_xi_query_pointer_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_pointer_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_pointer_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_pointer_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_xi_query_pointer(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_input_xi_query_pointer_cookie_t {
        sym!(self, xcb_input_xi_query_pointer)(c, window, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_pointer` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_pointer(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_pointer)
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
    pub unsafe fn xcb_input_xi_query_pointer_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_input_xi_query_pointer_cookie_t {
        sym!(self, xcb_input_xi_query_pointer_unchecked)(c, window, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_pointer_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_pointer_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_pointer_unchecked)
    }

    pub unsafe fn xcb_input_xi_query_pointer_buttons(
        &self,
        r: *const xcb_input_xi_query_pointer_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_xi_query_pointer_buttons)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_pointer_buttons` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_pointer_buttons(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_pointer_buttons)
    }

    pub unsafe fn xcb_input_xi_query_pointer_buttons_length(
        &self,
        r: *const xcb_input_xi_query_pointer_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_query_pointer_buttons_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_pointer_buttons_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_pointer_buttons_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_pointer_buttons_length)
    }

    pub unsafe fn xcb_input_xi_query_pointer_buttons_end(
        &self,
        r: *const xcb_input_xi_query_pointer_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_xi_query_pointer_buttons_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_pointer_buttons_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_pointer_buttons_end(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_pointer_buttons_end)
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
     * xcb_input_xi_query_pointer_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_xi_query_pointer_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_xi_query_pointer_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_xi_query_pointer_reply_t {
        sym!(self, xcb_input_xi_query_pointer_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_pointer_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_pointer_reply(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_pointer_reply)
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
    pub unsafe fn xcb_input_xi_warp_pointer_checked(
        &self,
        c: *mut xcb_connection_t,
        src_win: xcb_window_t,
        dst_win: xcb_window_t,
        src_x: xcb_input_fp1616_t,
        src_y: xcb_input_fp1616_t,
        src_width: u16,
        src_height: u16,
        dst_x: xcb_input_fp1616_t,
        dst_y: xcb_input_fp1616_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_warp_pointer_checked)(
            c, src_win, dst_win, src_x, src_y, src_width, src_height, dst_x, dst_y, deviceid,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_warp_pointer_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_warp_pointer_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_warp_pointer_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_xi_warp_pointer(
        &self,
        c: *mut xcb_connection_t,
        src_win: xcb_window_t,
        dst_win: xcb_window_t,
        src_x: xcb_input_fp1616_t,
        src_y: xcb_input_fp1616_t,
        src_width: u16,
        src_height: u16,
        dst_x: xcb_input_fp1616_t,
        dst_y: xcb_input_fp1616_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_warp_pointer)(
            c, src_win, dst_win, src_x, src_y, src_width, src_height, dst_x, dst_y, deviceid,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_warp_pointer` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_warp_pointer(&self) -> bool {
        has_sym!(self, xcb_input_xi_warp_pointer)
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
    pub unsafe fn xcb_input_xi_change_cursor_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        cursor: xcb_cursor_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_change_cursor_checked)(c, window, cursor, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_cursor_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_cursor_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_cursor_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_xi_change_cursor(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        cursor: xcb_cursor_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_change_cursor)(c, window, cursor, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_cursor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_cursor(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_cursor)
    }

    pub unsafe fn xcb_input_add_master_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_add_master_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_add_master_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_add_master_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_add_master_sizeof)
    }

    pub unsafe fn xcb_input_add_master_name(
        &self,
        r: *const xcb_input_add_master_t,
    ) -> *mut c_char {
        sym!(self, xcb_input_add_master_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_add_master_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_add_master_name(&self) -> bool {
        has_sym!(self, xcb_input_add_master_name)
    }

    pub unsafe fn xcb_input_add_master_name_length(
        &self,
        r: *const xcb_input_add_master_t,
    ) -> c_int {
        sym!(self, xcb_input_add_master_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_add_master_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_add_master_name_length(&self) -> bool {
        has_sym!(self, xcb_input_add_master_name_length)
    }

    pub unsafe fn xcb_input_add_master_name_end(
        &self,
        r: *const xcb_input_add_master_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_add_master_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_add_master_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_add_master_name_end(&self) -> bool {
        has_sym!(self, xcb_input_add_master_name_end)
    }

    pub unsafe fn xcb_input_add_master_next(&self, i: *mut xcb_input_add_master_iterator_t) {
        sym!(self, xcb_input_add_master_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_add_master_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_add_master_next(&self) -> bool {
        has_sym!(self, xcb_input_add_master_next)
    }

    pub unsafe fn xcb_input_add_master_end(
        &self,
        i: xcb_input_add_master_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_add_master_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_add_master_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_add_master_end(&self) -> bool {
        has_sym!(self, xcb_input_add_master_end)
    }

    pub unsafe fn xcb_input_remove_master_next(&self, i: *mut xcb_input_remove_master_iterator_t) {
        sym!(self, xcb_input_remove_master_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_remove_master_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_remove_master_next(&self) -> bool {
        has_sym!(self, xcb_input_remove_master_next)
    }

    pub unsafe fn xcb_input_remove_master_end(
        &self,
        i: xcb_input_remove_master_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_remove_master_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_remove_master_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_remove_master_end(&self) -> bool {
        has_sym!(self, xcb_input_remove_master_end)
    }

    pub unsafe fn xcb_input_attach_slave_next(&self, i: *mut xcb_input_attach_slave_iterator_t) {
        sym!(self, xcb_input_attach_slave_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_attach_slave_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_attach_slave_next(&self) -> bool {
        has_sym!(self, xcb_input_attach_slave_next)
    }

    pub unsafe fn xcb_input_attach_slave_end(
        &self,
        i: xcb_input_attach_slave_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_attach_slave_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_attach_slave_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_attach_slave_end(&self) -> bool {
        has_sym!(self, xcb_input_attach_slave_end)
    }

    pub unsafe fn xcb_input_detach_slave_next(&self, i: *mut xcb_input_detach_slave_iterator_t) {
        sym!(self, xcb_input_detach_slave_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_detach_slave_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_detach_slave_next(&self) -> bool {
        has_sym!(self, xcb_input_detach_slave_next)
    }

    pub unsafe fn xcb_input_detach_slave_end(
        &self,
        i: xcb_input_detach_slave_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_detach_slave_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_detach_slave_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_detach_slave_end(&self) -> bool {
        has_sym!(self, xcb_input_detach_slave_end)
    }

    pub unsafe fn xcb_input_hierarchy_change_data_add_master_name(
        &self,
        s: *const xcb_input_hierarchy_change_data_t,
    ) -> *mut c_char {
        sym!(self, xcb_input_hierarchy_change_data_add_master_name)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_change_data_add_master_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_change_data_add_master_name(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_change_data_add_master_name)
    }

    pub unsafe fn xcb_input_hierarchy_change_data_add_master_name_length(
        &self,
        r: *const xcb_input_hierarchy_change_t,
        s: *const xcb_input_hierarchy_change_data_t,
    ) -> c_int {
        sym!(self, xcb_input_hierarchy_change_data_add_master_name_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_change_data_add_master_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_change_data_add_master_name_length(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_change_data_add_master_name_length)
    }

    pub unsafe fn xcb_input_hierarchy_change_data_add_master_name_end(
        &self,
        r: *const xcb_input_hierarchy_change_t,
        s: *const xcb_input_hierarchy_change_data_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_hierarchy_change_data_add_master_name_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_change_data_add_master_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_change_data_add_master_name_end(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_change_data_add_master_name_end)
    }

    pub unsafe fn xcb_input_hierarchy_change_data_serialize(
        &self,
        _buffer: *mut *mut c_void,
        type_: u16,
        _aux: *const xcb_input_hierarchy_change_data_t,
    ) -> c_int {
        sym!(self, xcb_input_hierarchy_change_data_serialize)(_buffer, type_, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_change_data_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_change_data_serialize(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_change_data_serialize)
    }

    pub unsafe fn xcb_input_hierarchy_change_data_unpack(
        &self,
        _buffer: *const c_void,
        type_: u16,
        _aux: *mut xcb_input_hierarchy_change_data_t,
    ) -> c_int {
        sym!(self, xcb_input_hierarchy_change_data_unpack)(_buffer, type_, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_change_data_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_change_data_unpack(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_change_data_unpack)
    }

    pub unsafe fn xcb_input_hierarchy_change_data_sizeof(
        &self,
        _buffer: *const c_void,
        type_: u16,
    ) -> c_int {
        sym!(self, xcb_input_hierarchy_change_data_sizeof)(_buffer, type_)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_change_data_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_change_data_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_change_data_sizeof)
    }

    pub unsafe fn xcb_input_hierarchy_change_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_hierarchy_change_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_change_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_change_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_change_sizeof)
    }

    pub unsafe fn xcb_input_hierarchy_change_data(
        &self,
        r: *const xcb_input_hierarchy_change_t,
    ) -> *mut c_void {
        sym!(self, xcb_input_hierarchy_change_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_change_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_change_data(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_change_data)
    }

    pub unsafe fn xcb_input_hierarchy_change_next(
        &self,
        i: *mut xcb_input_hierarchy_change_iterator_t,
    ) {
        sym!(self, xcb_input_hierarchy_change_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_change_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_change_next(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_change_next)
    }

    pub unsafe fn xcb_input_hierarchy_change_end(
        &self,
        i: xcb_input_hierarchy_change_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_hierarchy_change_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_change_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_change_end(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_change_end)
    }

    pub unsafe fn xcb_input_xi_change_hierarchy_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_xi_change_hierarchy_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_hierarchy_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_hierarchy_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_hierarchy_sizeof)
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
    pub unsafe fn xcb_input_xi_change_hierarchy_checked(
        &self,
        c: *mut xcb_connection_t,
        num_changes: u8,
        changes: *const xcb_input_hierarchy_change_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_change_hierarchy_checked)(c, num_changes, changes)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_hierarchy_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_hierarchy_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_hierarchy_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_xi_change_hierarchy(
        &self,
        c: *mut xcb_connection_t,
        num_changes: u8,
        changes: *const xcb_input_hierarchy_change_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_change_hierarchy)(c, num_changes, changes)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_hierarchy` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_hierarchy(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_hierarchy)
    }

    pub unsafe fn xcb_input_xi_change_hierarchy_changes_length(
        &self,
        r: *const xcb_input_xi_change_hierarchy_request_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_change_hierarchy_changes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_hierarchy_changes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_hierarchy_changes_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_hierarchy_changes_length)
    }

    pub unsafe fn xcb_input_xi_change_hierarchy_changes_iterator(
        &self,
        r: *const xcb_input_xi_change_hierarchy_request_t,
    ) -> xcb_input_hierarchy_change_iterator_t {
        sym!(self, xcb_input_xi_change_hierarchy_changes_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_hierarchy_changes_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_hierarchy_changes_iterator(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_hierarchy_changes_iterator)
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
    pub unsafe fn xcb_input_xi_set_client_pointer_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_set_client_pointer_checked)(c, window, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_set_client_pointer_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_set_client_pointer_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_set_client_pointer_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_xi_set_client_pointer(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_set_client_pointer)(c, window, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_set_client_pointer` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_set_client_pointer(&self) -> bool {
        has_sym!(self, xcb_input_xi_set_client_pointer)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_xi_get_client_pointer(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_input_xi_get_client_pointer_cookie_t {
        sym!(self, xcb_input_xi_get_client_pointer)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_client_pointer` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_client_pointer(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_client_pointer)
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
    pub unsafe fn xcb_input_xi_get_client_pointer_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_input_xi_get_client_pointer_cookie_t {
        sym!(self, xcb_input_xi_get_client_pointer_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_client_pointer_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_client_pointer_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_client_pointer_unchecked)
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
     * xcb_input_xi_get_client_pointer_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_xi_get_client_pointer_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_xi_get_client_pointer_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_xi_get_client_pointer_reply_t {
        sym!(self, xcb_input_xi_get_client_pointer_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_client_pointer_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_client_pointer_reply(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_client_pointer_reply)
    }

    pub unsafe fn xcb_input_event_mask_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_event_mask_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_event_mask_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_event_mask_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_event_mask_sizeof)
    }

    pub unsafe fn xcb_input_event_mask_mask(&self, r: *const xcb_input_event_mask_t) -> *mut u32 {
        sym!(self, xcb_input_event_mask_mask)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_event_mask_mask` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_event_mask_mask(&self) -> bool {
        has_sym!(self, xcb_input_event_mask_mask)
    }

    pub unsafe fn xcb_input_event_mask_mask_length(
        &self,
        r: *const xcb_input_event_mask_t,
    ) -> c_int {
        sym!(self, xcb_input_event_mask_mask_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_event_mask_mask_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_event_mask_mask_length(&self) -> bool {
        has_sym!(self, xcb_input_event_mask_mask_length)
    }

    pub unsafe fn xcb_input_event_mask_mask_end(
        &self,
        r: *const xcb_input_event_mask_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_event_mask_mask_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_event_mask_mask_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_event_mask_mask_end(&self) -> bool {
        has_sym!(self, xcb_input_event_mask_mask_end)
    }

    pub unsafe fn xcb_input_event_mask_next(&self, i: *mut xcb_input_event_mask_iterator_t) {
        sym!(self, xcb_input_event_mask_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_event_mask_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_event_mask_next(&self) -> bool {
        has_sym!(self, xcb_input_event_mask_next)
    }

    pub unsafe fn xcb_input_event_mask_end(
        &self,
        i: xcb_input_event_mask_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_event_mask_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_event_mask_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_event_mask_end(&self) -> bool {
        has_sym!(self, xcb_input_event_mask_end)
    }

    pub unsafe fn xcb_input_xi_select_events_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_xi_select_events_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_select_events_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_select_events_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_select_events_sizeof)
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
    pub unsafe fn xcb_input_xi_select_events_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        num_mask: u16,
        masks: *const xcb_input_event_mask_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_select_events_checked)(c, window, num_mask, masks)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_select_events_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_select_events_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_select_events_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_xi_select_events(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        num_mask: u16,
        masks: *const xcb_input_event_mask_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_select_events)(c, window, num_mask, masks)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_select_events` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_select_events(&self) -> bool {
        has_sym!(self, xcb_input_xi_select_events)
    }

    pub unsafe fn xcb_input_xi_select_events_masks_length(
        &self,
        r: *const xcb_input_xi_select_events_request_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_select_events_masks_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_select_events_masks_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_select_events_masks_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_select_events_masks_length)
    }

    pub unsafe fn xcb_input_xi_select_events_masks_iterator(
        &self,
        r: *const xcb_input_xi_select_events_request_t,
    ) -> xcb_input_event_mask_iterator_t {
        sym!(self, xcb_input_xi_select_events_masks_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_select_events_masks_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_select_events_masks_iterator(&self) -> bool {
        has_sym!(self, xcb_input_xi_select_events_masks_iterator)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_xi_query_version(
        &self,
        c: *mut xcb_connection_t,
        major_version: u16,
        minor_version: u16,
    ) -> xcb_input_xi_query_version_cookie_t {
        sym!(self, xcb_input_xi_query_version)(c, major_version, minor_version)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_version(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_version)
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
    pub unsafe fn xcb_input_xi_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        major_version: u16,
        minor_version: u16,
    ) -> xcb_input_xi_query_version_cookie_t {
        sym!(self, xcb_input_xi_query_version_unchecked)(c, major_version, minor_version)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_version_unchecked)
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
     * xcb_input_xi_query_version_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_xi_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_xi_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_xi_query_version_reply_t {
        sym!(self, xcb_input_xi_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_version_reply)
    }

    pub unsafe fn xcb_input_button_class_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_button_class_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_button_class_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_class_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_button_class_sizeof)
    }

    pub unsafe fn xcb_input_button_class_state(
        &self,
        r: *const xcb_input_button_class_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_button_class_state)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_class_state` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_class_state(&self) -> bool {
        has_sym!(self, xcb_input_button_class_state)
    }

    pub unsafe fn xcb_input_button_class_state_length(
        &self,
        r: *const xcb_input_button_class_t,
    ) -> c_int {
        sym!(self, xcb_input_button_class_state_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_class_state_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_class_state_length(&self) -> bool {
        has_sym!(self, xcb_input_button_class_state_length)
    }

    pub unsafe fn xcb_input_button_class_state_end(
        &self,
        r: *const xcb_input_button_class_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_button_class_state_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_class_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_class_state_end(&self) -> bool {
        has_sym!(self, xcb_input_button_class_state_end)
    }

    pub unsafe fn xcb_input_button_class_labels(
        &self,
        r: *const xcb_input_button_class_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_input_button_class_labels)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_class_labels` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_class_labels(&self) -> bool {
        has_sym!(self, xcb_input_button_class_labels)
    }

    pub unsafe fn xcb_input_button_class_labels_length(
        &self,
        r: *const xcb_input_button_class_t,
    ) -> c_int {
        sym!(self, xcb_input_button_class_labels_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_class_labels_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_class_labels_length(&self) -> bool {
        has_sym!(self, xcb_input_button_class_labels_length)
    }

    pub unsafe fn xcb_input_button_class_labels_end(
        &self,
        r: *const xcb_input_button_class_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_button_class_labels_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_class_labels_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_class_labels_end(&self) -> bool {
        has_sym!(self, xcb_input_button_class_labels_end)
    }

    pub unsafe fn xcb_input_button_class_next(&self, i: *mut xcb_input_button_class_iterator_t) {
        sym!(self, xcb_input_button_class_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_button_class_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_class_next(&self) -> bool {
        has_sym!(self, xcb_input_button_class_next)
    }

    pub unsafe fn xcb_input_button_class_end(
        &self,
        i: xcb_input_button_class_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_button_class_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_button_class_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_class_end(&self) -> bool {
        has_sym!(self, xcb_input_button_class_end)
    }

    pub unsafe fn xcb_input_key_class_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_key_class_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_key_class_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_class_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_key_class_sizeof)
    }

    pub unsafe fn xcb_input_key_class_keys(&self, r: *const xcb_input_key_class_t) -> *mut u32 {
        sym!(self, xcb_input_key_class_keys)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_key_class_keys` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_class_keys(&self) -> bool {
        has_sym!(self, xcb_input_key_class_keys)
    }

    pub unsafe fn xcb_input_key_class_keys_length(&self, r: *const xcb_input_key_class_t) -> c_int {
        sym!(self, xcb_input_key_class_keys_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_key_class_keys_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_class_keys_length(&self) -> bool {
        has_sym!(self, xcb_input_key_class_keys_length)
    }

    pub unsafe fn xcb_input_key_class_keys_end(
        &self,
        r: *const xcb_input_key_class_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_key_class_keys_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_key_class_keys_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_class_keys_end(&self) -> bool {
        has_sym!(self, xcb_input_key_class_keys_end)
    }

    pub unsafe fn xcb_input_key_class_next(&self, i: *mut xcb_input_key_class_iterator_t) {
        sym!(self, xcb_input_key_class_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_key_class_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_class_next(&self) -> bool {
        has_sym!(self, xcb_input_key_class_next)
    }

    pub unsafe fn xcb_input_key_class_end(
        &self,
        i: xcb_input_key_class_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_key_class_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_key_class_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_class_end(&self) -> bool {
        has_sym!(self, xcb_input_key_class_end)
    }

    pub unsafe fn xcb_input_scroll_class_next(&self, i: *mut xcb_input_scroll_class_iterator_t) {
        sym!(self, xcb_input_scroll_class_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_scroll_class_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_scroll_class_next(&self) -> bool {
        has_sym!(self, xcb_input_scroll_class_next)
    }

    pub unsafe fn xcb_input_scroll_class_end(
        &self,
        i: xcb_input_scroll_class_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_scroll_class_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_scroll_class_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_scroll_class_end(&self) -> bool {
        has_sym!(self, xcb_input_scroll_class_end)
    }

    pub unsafe fn xcb_input_touch_class_next(&self, i: *mut xcb_input_touch_class_iterator_t) {
        sym!(self, xcb_input_touch_class_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_class_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_class_next(&self) -> bool {
        has_sym!(self, xcb_input_touch_class_next)
    }

    pub unsafe fn xcb_input_touch_class_end(
        &self,
        i: xcb_input_touch_class_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_touch_class_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_class_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_class_end(&self) -> bool {
        has_sym!(self, xcb_input_touch_class_end)
    }

    pub unsafe fn xcb_input_valuator_class_next(
        &self,
        i: *mut xcb_input_valuator_class_iterator_t,
    ) {
        sym!(self, xcb_input_valuator_class_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_class_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_class_next(&self) -> bool {
        has_sym!(self, xcb_input_valuator_class_next)
    }

    pub unsafe fn xcb_input_valuator_class_end(
        &self,
        i: xcb_input_valuator_class_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_valuator_class_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_class_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_class_end(&self) -> bool {
        has_sym!(self, xcb_input_valuator_class_end)
    }

    pub unsafe fn xcb_input_device_class_data_key_keys(
        &self,
        s: *const xcb_input_device_class_data_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_device_class_data_key_keys)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data_key_keys` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data_key_keys(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data_key_keys)
    }

    pub unsafe fn xcb_input_device_class_data_key_keys_length(
        &self,
        r: *const xcb_input_device_class_t,
        s: *const xcb_input_device_class_data_t,
    ) -> c_int {
        sym!(self, xcb_input_device_class_data_key_keys_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data_key_keys_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data_key_keys_length(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data_key_keys_length)
    }

    pub unsafe fn xcb_input_device_class_data_key_keys_end(
        &self,
        r: *const xcb_input_device_class_t,
        s: *const xcb_input_device_class_data_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_class_data_key_keys_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data_key_keys_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data_key_keys_end(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data_key_keys_end)
    }

    pub unsafe fn xcb_input_device_class_data_button_state(
        &self,
        s: *const xcb_input_device_class_data_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_device_class_data_button_state)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data_button_state` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data_button_state(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data_button_state)
    }

    pub unsafe fn xcb_input_device_class_data_button_state_length(
        &self,
        r: *const xcb_input_device_class_t,
        s: *const xcb_input_device_class_data_t,
    ) -> c_int {
        sym!(self, xcb_input_device_class_data_button_state_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data_button_state_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data_button_state_length(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data_button_state_length)
    }

    pub unsafe fn xcb_input_device_class_data_button_state_end(
        &self,
        r: *const xcb_input_device_class_t,
        s: *const xcb_input_device_class_data_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_class_data_button_state_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data_button_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data_button_state_end(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data_button_state_end)
    }

    pub unsafe fn xcb_input_device_class_data_button_labels(
        &self,
        s: *const xcb_input_device_class_data_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_input_device_class_data_button_labels)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data_button_labels` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data_button_labels(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data_button_labels)
    }

    pub unsafe fn xcb_input_device_class_data_button_labels_length(
        &self,
        r: *const xcb_input_device_class_t,
        s: *const xcb_input_device_class_data_t,
    ) -> c_int {
        sym!(self, xcb_input_device_class_data_button_labels_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data_button_labels_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data_button_labels_length(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data_button_labels_length)
    }

    pub unsafe fn xcb_input_device_class_data_button_labels_end(
        &self,
        r: *const xcb_input_device_class_t,
        s: *const xcb_input_device_class_data_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_class_data_button_labels_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data_button_labels_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data_button_labels_end(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data_button_labels_end)
    }

    pub unsafe fn xcb_input_device_class_data_serialize(
        &self,
        _buffer: *mut *mut c_void,
        type_: u16,
        _aux: *const xcb_input_device_class_data_t,
    ) -> c_int {
        sym!(self, xcb_input_device_class_data_serialize)(_buffer, type_, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data_serialize(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data_serialize)
    }

    pub unsafe fn xcb_input_device_class_data_unpack(
        &self,
        _buffer: *const c_void,
        type_: u16,
        _aux: *mut xcb_input_device_class_data_t,
    ) -> c_int {
        sym!(self, xcb_input_device_class_data_unpack)(_buffer, type_, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data_unpack(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data_unpack)
    }

    pub unsafe fn xcb_input_device_class_data_sizeof(
        &self,
        _buffer: *const c_void,
        type_: u16,
    ) -> c_int {
        sym!(self, xcb_input_device_class_data_sizeof)(_buffer, type_)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data_sizeof)
    }

    pub unsafe fn xcb_input_device_class_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_device_class_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_device_class_sizeof)
    }

    pub unsafe fn xcb_input_device_class_data(
        &self,
        r: *const xcb_input_device_class_t,
    ) -> *mut c_void {
        sym!(self, xcb_input_device_class_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data)
    }

    pub unsafe fn xcb_input_device_class_next(&self, i: *mut xcb_input_device_class_iterator_t) {
        sym!(self, xcb_input_device_class_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_next(&self) -> bool {
        has_sym!(self, xcb_input_device_class_next)
    }

    pub unsafe fn xcb_input_device_class_end(
        &self,
        i: xcb_input_device_class_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_class_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_end(&self) -> bool {
        has_sym!(self, xcb_input_device_class_end)
    }

    pub unsafe fn xcb_input_xi_device_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_xi_device_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_device_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_device_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_device_info_sizeof)
    }

    pub unsafe fn xcb_input_xi_device_info_name(
        &self,
        r: *const xcb_input_xi_device_info_t,
    ) -> *mut c_char {
        sym!(self, xcb_input_xi_device_info_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_device_info_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_device_info_name(&self) -> bool {
        has_sym!(self, xcb_input_xi_device_info_name)
    }

    pub unsafe fn xcb_input_xi_device_info_name_length(
        &self,
        r: *const xcb_input_xi_device_info_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_device_info_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_device_info_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_device_info_name_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_device_info_name_length)
    }

    pub unsafe fn xcb_input_xi_device_info_name_end(
        &self,
        r: *const xcb_input_xi_device_info_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_xi_device_info_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_device_info_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_device_info_name_end(&self) -> bool {
        has_sym!(self, xcb_input_xi_device_info_name_end)
    }

    pub unsafe fn xcb_input_xi_device_info_classes_length(
        &self,
        r: *const xcb_input_xi_device_info_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_device_info_classes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_device_info_classes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_device_info_classes_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_device_info_classes_length)
    }

    pub unsafe fn xcb_input_xi_device_info_classes_iterator(
        &self,
        r: *const xcb_input_xi_device_info_t,
    ) -> xcb_input_device_class_iterator_t {
        sym!(self, xcb_input_xi_device_info_classes_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_device_info_classes_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_device_info_classes_iterator(&self) -> bool {
        has_sym!(self, xcb_input_xi_device_info_classes_iterator)
    }

    pub unsafe fn xcb_input_xi_device_info_next(
        &self,
        i: *mut xcb_input_xi_device_info_iterator_t,
    ) {
        sym!(self, xcb_input_xi_device_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_device_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_device_info_next(&self) -> bool {
        has_sym!(self, xcb_input_xi_device_info_next)
    }

    pub unsafe fn xcb_input_xi_device_info_end(
        &self,
        i: xcb_input_xi_device_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_xi_device_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_device_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_device_info_end(&self) -> bool {
        has_sym!(self, xcb_input_xi_device_info_end)
    }

    pub unsafe fn xcb_input_xi_query_device_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_xi_query_device_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_device_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_device_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_device_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_xi_query_device(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_input_xi_query_device_cookie_t {
        sym!(self, xcb_input_xi_query_device)(c, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_device` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_device(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_device)
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
    pub unsafe fn xcb_input_xi_query_device_unchecked(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_input_xi_query_device_cookie_t {
        sym!(self, xcb_input_xi_query_device_unchecked)(c, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_device_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_device_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_device_unchecked)
    }

    pub unsafe fn xcb_input_xi_query_device_infos_length(
        &self,
        r: *const xcb_input_xi_query_device_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_query_device_infos_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_device_infos_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_device_infos_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_device_infos_length)
    }

    pub unsafe fn xcb_input_xi_query_device_infos_iterator(
        &self,
        r: *const xcb_input_xi_query_device_reply_t,
    ) -> xcb_input_xi_device_info_iterator_t {
        sym!(self, xcb_input_xi_query_device_infos_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_device_infos_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_device_infos_iterator(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_device_infos_iterator)
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
     * xcb_input_xi_query_device_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_xi_query_device_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_xi_query_device_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_xi_query_device_reply_t {
        sym!(self, xcb_input_xi_query_device_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_device_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_device_reply(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_device_reply)
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
    pub unsafe fn xcb_input_xi_set_focus_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        time: xcb_timestamp_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_set_focus_checked)(c, window, time, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_set_focus_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_set_focus_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_set_focus_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_xi_set_focus(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        time: xcb_timestamp_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_set_focus)(c, window, time, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_set_focus` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_set_focus(&self) -> bool {
        has_sym!(self, xcb_input_xi_set_focus)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_xi_get_focus(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_input_xi_get_focus_cookie_t {
        sym!(self, xcb_input_xi_get_focus)(c, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_focus` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_focus(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_focus)
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
    pub unsafe fn xcb_input_xi_get_focus_unchecked(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_input_xi_get_focus_cookie_t {
        sym!(self, xcb_input_xi_get_focus_unchecked)(c, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_focus_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_focus_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_focus_unchecked)
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
     * xcb_input_xi_get_focus_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_xi_get_focus_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_xi_get_focus_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_xi_get_focus_reply_t {
        sym!(self, xcb_input_xi_get_focus_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_focus_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_focus_reply(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_focus_reply)
    }

    pub unsafe fn xcb_input_xi_grab_device_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_xi_grab_device_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_grab_device_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_grab_device_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_grab_device_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_xi_grab_device(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        time: xcb_timestamp_t,
        cursor: xcb_cursor_t,
        deviceid: xcb_input_device_id_t,
        mode: u8,
        paired_device_mode: u8,
        owner_events: u8,
        mask_len: u16,
        mask: *const u32,
    ) -> xcb_input_xi_grab_device_cookie_t {
        sym!(self, xcb_input_xi_grab_device)(
            c,
            window,
            time,
            cursor,
            deviceid,
            mode,
            paired_device_mode,
            owner_events,
            mask_len,
            mask,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_grab_device` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_grab_device(&self) -> bool {
        has_sym!(self, xcb_input_xi_grab_device)
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
    pub unsafe fn xcb_input_xi_grab_device_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        time: xcb_timestamp_t,
        cursor: xcb_cursor_t,
        deviceid: xcb_input_device_id_t,
        mode: u8,
        paired_device_mode: u8,
        owner_events: u8,
        mask_len: u16,
        mask: *const u32,
    ) -> xcb_input_xi_grab_device_cookie_t {
        sym!(self, xcb_input_xi_grab_device_unchecked)(
            c,
            window,
            time,
            cursor,
            deviceid,
            mode,
            paired_device_mode,
            owner_events,
            mask_len,
            mask,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_grab_device_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_grab_device_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_xi_grab_device_unchecked)
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
     * xcb_input_xi_grab_device_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_xi_grab_device_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_xi_grab_device_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_xi_grab_device_reply_t {
        sym!(self, xcb_input_xi_grab_device_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_grab_device_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_grab_device_reply(&self) -> bool {
        has_sym!(self, xcb_input_xi_grab_device_reply)
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
    pub unsafe fn xcb_input_xi_ungrab_device_checked(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_ungrab_device_checked)(c, time, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_ungrab_device_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_ungrab_device_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_ungrab_device_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_xi_ungrab_device(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_ungrab_device)(c, time, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_ungrab_device` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_ungrab_device(&self) -> bool {
        has_sym!(self, xcb_input_xi_ungrab_device)
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
    pub unsafe fn xcb_input_xi_allow_events_checked(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
        deviceid: xcb_input_device_id_t,
        event_mode: u8,
        touchid: u32,
        grab_window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_allow_events_checked)(
            c,
            time,
            deviceid,
            event_mode,
            touchid,
            grab_window,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_allow_events_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_allow_events_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_allow_events_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_xi_allow_events(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
        deviceid: xcb_input_device_id_t,
        event_mode: u8,
        touchid: u32,
        grab_window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_allow_events)(c, time, deviceid, event_mode, touchid, grab_window)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_allow_events` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_allow_events(&self) -> bool {
        has_sym!(self, xcb_input_xi_allow_events)
    }

    pub unsafe fn xcb_input_grab_modifier_info_next(
        &self,
        i: *mut xcb_input_grab_modifier_info_iterator_t,
    ) {
        sym!(self, xcb_input_grab_modifier_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_grab_modifier_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_modifier_info_next(&self) -> bool {
        has_sym!(self, xcb_input_grab_modifier_info_next)
    }

    pub unsafe fn xcb_input_grab_modifier_info_end(
        &self,
        i: xcb_input_grab_modifier_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_grab_modifier_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_grab_modifier_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_modifier_info_end(&self) -> bool {
        has_sym!(self, xcb_input_grab_modifier_info_end)
    }

    pub unsafe fn xcb_input_xi_passive_grab_device_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_xi_passive_grab_device_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_grab_device_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_grab_device_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_grab_device_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_xi_passive_grab_device(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
        grab_window: xcb_window_t,
        cursor: xcb_cursor_t,
        detail: u32,
        deviceid: xcb_input_device_id_t,
        num_modifiers: u16,
        mask_len: u16,
        grab_type: u8,
        grab_mode: u8,
        paired_device_mode: u8,
        owner_events: u8,
        mask: *const u32,
        modifiers: *const u32,
    ) -> xcb_input_xi_passive_grab_device_cookie_t {
        sym!(self, xcb_input_xi_passive_grab_device)(
            c,
            time,
            grab_window,
            cursor,
            detail,
            deviceid,
            num_modifiers,
            mask_len,
            grab_type,
            grab_mode,
            paired_device_mode,
            owner_events,
            mask,
            modifiers,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_grab_device` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_grab_device(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_grab_device)
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
    pub unsafe fn xcb_input_xi_passive_grab_device_unchecked(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
        grab_window: xcb_window_t,
        cursor: xcb_cursor_t,
        detail: u32,
        deviceid: xcb_input_device_id_t,
        num_modifiers: u16,
        mask_len: u16,
        grab_type: u8,
        grab_mode: u8,
        paired_device_mode: u8,
        owner_events: u8,
        mask: *const u32,
        modifiers: *const u32,
    ) -> xcb_input_xi_passive_grab_device_cookie_t {
        sym!(self, xcb_input_xi_passive_grab_device_unchecked)(
            c,
            time,
            grab_window,
            cursor,
            detail,
            deviceid,
            num_modifiers,
            mask_len,
            grab_type,
            grab_mode,
            paired_device_mode,
            owner_events,
            mask,
            modifiers,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_grab_device_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_grab_device_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_grab_device_unchecked)
    }

    pub unsafe fn xcb_input_xi_passive_grab_device_modifiers(
        &self,
        r: *const xcb_input_xi_passive_grab_device_reply_t,
    ) -> *mut xcb_input_grab_modifier_info_t {
        sym!(self, xcb_input_xi_passive_grab_device_modifiers)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_grab_device_modifiers` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_grab_device_modifiers(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_grab_device_modifiers)
    }

    pub unsafe fn xcb_input_xi_passive_grab_device_modifiers_length(
        &self,
        r: *const xcb_input_xi_passive_grab_device_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_passive_grab_device_modifiers_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_grab_device_modifiers_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_grab_device_modifiers_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_grab_device_modifiers_length)
    }

    pub unsafe fn xcb_input_xi_passive_grab_device_modifiers_iterator(
        &self,
        r: *const xcb_input_xi_passive_grab_device_reply_t,
    ) -> xcb_input_grab_modifier_info_iterator_t {
        sym!(self, xcb_input_xi_passive_grab_device_modifiers_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_grab_device_modifiers_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_grab_device_modifiers_iterator(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_grab_device_modifiers_iterator)
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
     * xcb_input_xi_passive_grab_device_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_xi_passive_grab_device_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_xi_passive_grab_device_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_xi_passive_grab_device_reply_t {
        sym!(self, xcb_input_xi_passive_grab_device_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_grab_device_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_grab_device_reply(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_grab_device_reply)
    }

    pub unsafe fn xcb_input_xi_passive_ungrab_device_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_input_xi_passive_ungrab_device_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_ungrab_device_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_ungrab_device_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_ungrab_device_sizeof)
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
    pub unsafe fn xcb_input_xi_passive_ungrab_device_checked(
        &self,
        c: *mut xcb_connection_t,
        grab_window: xcb_window_t,
        detail: u32,
        deviceid: xcb_input_device_id_t,
        num_modifiers: u16,
        grab_type: u8,
        modifiers: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_passive_ungrab_device_checked)(
            c,
            grab_window,
            detail,
            deviceid,
            num_modifiers,
            grab_type,
            modifiers,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_ungrab_device_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_ungrab_device_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_ungrab_device_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_xi_passive_ungrab_device(
        &self,
        c: *mut xcb_connection_t,
        grab_window: xcb_window_t,
        detail: u32,
        deviceid: xcb_input_device_id_t,
        num_modifiers: u16,
        grab_type: u8,
        modifiers: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_passive_ungrab_device)(
            c,
            grab_window,
            detail,
            deviceid,
            num_modifiers,
            grab_type,
            modifiers,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_ungrab_device` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_ungrab_device(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_ungrab_device)
    }

    pub unsafe fn xcb_input_xi_passive_ungrab_device_modifiers(
        &self,
        r: *const xcb_input_xi_passive_ungrab_device_request_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_xi_passive_ungrab_device_modifiers)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_ungrab_device_modifiers` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_ungrab_device_modifiers(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_ungrab_device_modifiers)
    }

    pub unsafe fn xcb_input_xi_passive_ungrab_device_modifiers_length(
        &self,
        r: *const xcb_input_xi_passive_ungrab_device_request_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_passive_ungrab_device_modifiers_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_ungrab_device_modifiers_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_ungrab_device_modifiers_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_ungrab_device_modifiers_length)
    }

    pub unsafe fn xcb_input_xi_passive_ungrab_device_modifiers_end(
        &self,
        r: *const xcb_input_xi_passive_ungrab_device_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_xi_passive_ungrab_device_modifiers_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_ungrab_device_modifiers_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_ungrab_device_modifiers_end(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_ungrab_device_modifiers_end)
    }

    pub unsafe fn xcb_input_xi_list_properties_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_xi_list_properties_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_list_properties_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_list_properties_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_list_properties_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_xi_list_properties(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_input_xi_list_properties_cookie_t {
        sym!(self, xcb_input_xi_list_properties)(c, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_list_properties` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_list_properties(&self) -> bool {
        has_sym!(self, xcb_input_xi_list_properties)
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
    pub unsafe fn xcb_input_xi_list_properties_unchecked(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_input_xi_list_properties_cookie_t {
        sym!(self, xcb_input_xi_list_properties_unchecked)(c, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_list_properties_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_list_properties_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_xi_list_properties_unchecked)
    }

    pub unsafe fn xcb_input_xi_list_properties_properties(
        &self,
        r: *const xcb_input_xi_list_properties_reply_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_input_xi_list_properties_properties)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_list_properties_properties` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_list_properties_properties(&self) -> bool {
        has_sym!(self, xcb_input_xi_list_properties_properties)
    }

    pub unsafe fn xcb_input_xi_list_properties_properties_length(
        &self,
        r: *const xcb_input_xi_list_properties_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_list_properties_properties_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_list_properties_properties_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_list_properties_properties_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_list_properties_properties_length)
    }

    pub unsafe fn xcb_input_xi_list_properties_properties_end(
        &self,
        r: *const xcb_input_xi_list_properties_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_xi_list_properties_properties_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_list_properties_properties_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_list_properties_properties_end(&self) -> bool {
        has_sym!(self, xcb_input_xi_list_properties_properties_end)
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
     * xcb_input_xi_list_properties_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_xi_list_properties_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_xi_list_properties_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_xi_list_properties_reply_t {
        sym!(self, xcb_input_xi_list_properties_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_list_properties_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_list_properties_reply(&self) -> bool {
        has_sym!(self, xcb_input_xi_list_properties_reply)
    }

    pub unsafe fn xcb_input_xi_change_property_items_data_8(
        &self,
        s: *const xcb_input_xi_change_property_items_t,
    ) -> *mut u8 {
        sym!(self, xcb_input_xi_change_property_items_data_8)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items_data_8` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items_data_8(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items_data_8)
    }

    pub unsafe fn xcb_input_xi_change_property_items_data_8_length(
        &self,
        r: *const xcb_input_xi_change_property_request_t,
        s: *const xcb_input_xi_change_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_change_property_items_data_8_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items_data_8_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items_data_8_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items_data_8_length)
    }

    pub unsafe fn xcb_input_xi_change_property_items_data_8_end(
        &self,
        r: *const xcb_input_xi_change_property_request_t,
        s: *const xcb_input_xi_change_property_items_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_xi_change_property_items_data_8_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items_data_8_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items_data_8_end(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items_data_8_end)
    }

    pub unsafe fn xcb_input_xi_change_property_items_data_16(
        &self,
        s: *const xcb_input_xi_change_property_items_t,
    ) -> *mut u16 {
        sym!(self, xcb_input_xi_change_property_items_data_16)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items_data_16` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items_data_16(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items_data_16)
    }

    pub unsafe fn xcb_input_xi_change_property_items_data_16_length(
        &self,
        r: *const xcb_input_xi_change_property_request_t,
        s: *const xcb_input_xi_change_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_change_property_items_data_16_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items_data_16_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items_data_16_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items_data_16_length)
    }

    pub unsafe fn xcb_input_xi_change_property_items_data_16_end(
        &self,
        r: *const xcb_input_xi_change_property_request_t,
        s: *const xcb_input_xi_change_property_items_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_xi_change_property_items_data_16_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items_data_16_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items_data_16_end(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items_data_16_end)
    }

    pub unsafe fn xcb_input_xi_change_property_items_data_32(
        &self,
        s: *const xcb_input_xi_change_property_items_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_xi_change_property_items_data_32)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items_data_32` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items_data_32(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items_data_32)
    }

    pub unsafe fn xcb_input_xi_change_property_items_data_32_length(
        &self,
        r: *const xcb_input_xi_change_property_request_t,
        s: *const xcb_input_xi_change_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_change_property_items_data_32_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items_data_32_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items_data_32_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items_data_32_length)
    }

    pub unsafe fn xcb_input_xi_change_property_items_data_32_end(
        &self,
        r: *const xcb_input_xi_change_property_request_t,
        s: *const xcb_input_xi_change_property_items_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_xi_change_property_items_data_32_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items_data_32_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items_data_32_end(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items_data_32_end)
    }

    pub unsafe fn xcb_input_xi_change_property_items_serialize(
        &self,
        _buffer: *mut *mut c_void,
        num_items: u32,
        format: u8,
        _aux: *const xcb_input_xi_change_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_change_property_items_serialize)(_buffer, num_items, format, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items_serialize(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items_serialize)
    }

    pub unsafe fn xcb_input_xi_change_property_items_unpack(
        &self,
        _buffer: *const c_void,
        num_items: u32,
        format: u8,
        _aux: *mut xcb_input_xi_change_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_change_property_items_unpack)(_buffer, num_items, format, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items_unpack(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items_unpack)
    }

    pub unsafe fn xcb_input_xi_change_property_items_sizeof(
        &self,
        _buffer: *const c_void,
        num_items: u32,
        format: u8,
    ) -> c_int {
        sym!(self, xcb_input_xi_change_property_items_sizeof)(_buffer, num_items, format)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items_sizeof)
    }

    pub unsafe fn xcb_input_xi_change_property_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_xi_change_property_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_sizeof)
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
    pub unsafe fn xcb_input_xi_change_property_checked(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
        mode: u8,
        format: u8,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        num_items: u32,
        items: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_change_property_checked)(
            c, deviceid, mode, format, property, type_, num_items, items,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_xi_change_property(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
        mode: u8,
        format: u8,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        num_items: u32,
        items: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_change_property)(
            c, deviceid, mode, format, property, type_, num_items, items,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property)
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
    pub unsafe fn xcb_input_xi_change_property_aux_checked(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
        mode: u8,
        format: u8,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        num_items: u32,
        items: *const xcb_input_xi_change_property_items_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_change_property_aux_checked)(
            c, deviceid, mode, format, property, type_, num_items, items,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_aux_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_aux_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_xi_change_property_aux(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
        mode: u8,
        format: u8,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        num_items: u32,
        items: *const xcb_input_xi_change_property_items_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_change_property_aux)(
            c, deviceid, mode, format, property, type_, num_items, items,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_aux(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_aux)
    }

    pub unsafe fn xcb_input_xi_change_property_items(
        &self,
        r: *const xcb_input_xi_change_property_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_input_xi_change_property_items)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items)
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
    pub unsafe fn xcb_input_xi_delete_property_checked(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
        property: xcb_atom_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_delete_property_checked)(c, deviceid, property)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_delete_property_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_delete_property_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_delete_property_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_xi_delete_property(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
        property: xcb_atom_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_delete_property)(c, deviceid, property)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_delete_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_delete_property(&self) -> bool {
        has_sym!(self, xcb_input_xi_delete_property)
    }

    pub unsafe fn xcb_input_xi_get_property_items_data_8(
        &self,
        s: *const xcb_input_xi_get_property_items_t,
    ) -> *mut u8 {
        sym!(self, xcb_input_xi_get_property_items_data_8)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items_data_8` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items_data_8(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items_data_8)
    }

    pub unsafe fn xcb_input_xi_get_property_items_data_8_length(
        &self,
        r: *const xcb_input_xi_get_property_reply_t,
        s: *const xcb_input_xi_get_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_get_property_items_data_8_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items_data_8_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items_data_8_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items_data_8_length)
    }

    pub unsafe fn xcb_input_xi_get_property_items_data_8_end(
        &self,
        r: *const xcb_input_xi_get_property_reply_t,
        s: *const xcb_input_xi_get_property_items_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_xi_get_property_items_data_8_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items_data_8_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items_data_8_end(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items_data_8_end)
    }

    pub unsafe fn xcb_input_xi_get_property_items_data_16(
        &self,
        s: *const xcb_input_xi_get_property_items_t,
    ) -> *mut u16 {
        sym!(self, xcb_input_xi_get_property_items_data_16)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items_data_16` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items_data_16(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items_data_16)
    }

    pub unsafe fn xcb_input_xi_get_property_items_data_16_length(
        &self,
        r: *const xcb_input_xi_get_property_reply_t,
        s: *const xcb_input_xi_get_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_get_property_items_data_16_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items_data_16_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items_data_16_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items_data_16_length)
    }

    pub unsafe fn xcb_input_xi_get_property_items_data_16_end(
        &self,
        r: *const xcb_input_xi_get_property_reply_t,
        s: *const xcb_input_xi_get_property_items_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_xi_get_property_items_data_16_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items_data_16_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items_data_16_end(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items_data_16_end)
    }

    pub unsafe fn xcb_input_xi_get_property_items_data_32(
        &self,
        s: *const xcb_input_xi_get_property_items_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_xi_get_property_items_data_32)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items_data_32` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items_data_32(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items_data_32)
    }

    pub unsafe fn xcb_input_xi_get_property_items_data_32_length(
        &self,
        r: *const xcb_input_xi_get_property_reply_t,
        s: *const xcb_input_xi_get_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_get_property_items_data_32_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items_data_32_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items_data_32_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items_data_32_length)
    }

    pub unsafe fn xcb_input_xi_get_property_items_data_32_end(
        &self,
        r: *const xcb_input_xi_get_property_reply_t,
        s: *const xcb_input_xi_get_property_items_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_xi_get_property_items_data_32_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items_data_32_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items_data_32_end(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items_data_32_end)
    }

    pub unsafe fn xcb_input_xi_get_property_items_serialize(
        &self,
        _buffer: *mut *mut c_void,
        num_items: u32,
        format: u8,
        _aux: *const xcb_input_xi_get_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_get_property_items_serialize)(_buffer, num_items, format, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items_serialize(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items_serialize)
    }

    pub unsafe fn xcb_input_xi_get_property_items_unpack(
        &self,
        _buffer: *const c_void,
        num_items: u32,
        format: u8,
        _aux: *mut xcb_input_xi_get_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_get_property_items_unpack)(_buffer, num_items, format, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items_unpack(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items_unpack)
    }

    pub unsafe fn xcb_input_xi_get_property_items_sizeof(
        &self,
        _buffer: *const c_void,
        num_items: u32,
        format: u8,
    ) -> c_int {
        sym!(self, xcb_input_xi_get_property_items_sizeof)(_buffer, num_items, format)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items_sizeof)
    }

    pub unsafe fn xcb_input_xi_get_property_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_xi_get_property_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_xi_get_property(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
        delete: u8,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        offset: u32,
        len: u32,
    ) -> xcb_input_xi_get_property_cookie_t {
        sym!(self, xcb_input_xi_get_property)(c, deviceid, delete, property, type_, offset, len)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property)
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
    pub unsafe fn xcb_input_xi_get_property_unchecked(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
        delete: u8,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        offset: u32,
        len: u32,
    ) -> xcb_input_xi_get_property_cookie_t {
        sym!(self, xcb_input_xi_get_property_unchecked)(
            c, deviceid, delete, property, type_, offset, len,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_unchecked)
    }

    pub unsafe fn xcb_input_xi_get_property_items(
        &self,
        r: *const xcb_input_xi_get_property_reply_t,
    ) -> *mut c_void {
        sym!(self, xcb_input_xi_get_property_items)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items)
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
     * xcb_input_xi_get_property_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_xi_get_property_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_xi_get_property_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_xi_get_property_reply_t {
        sym!(self, xcb_input_xi_get_property_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_reply(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_reply)
    }

    pub unsafe fn xcb_input_xi_get_selected_events_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_xi_get_selected_events_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_selected_events_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_selected_events_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_selected_events_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_xi_get_selected_events(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_input_xi_get_selected_events_cookie_t {
        sym!(self, xcb_input_xi_get_selected_events)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_selected_events` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_selected_events(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_selected_events)
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
    pub unsafe fn xcb_input_xi_get_selected_events_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_input_xi_get_selected_events_cookie_t {
        sym!(self, xcb_input_xi_get_selected_events_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_selected_events_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_selected_events_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_selected_events_unchecked)
    }

    pub unsafe fn xcb_input_xi_get_selected_events_masks_length(
        &self,
        r: *const xcb_input_xi_get_selected_events_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_get_selected_events_masks_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_selected_events_masks_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_selected_events_masks_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_selected_events_masks_length)
    }

    pub unsafe fn xcb_input_xi_get_selected_events_masks_iterator(
        &self,
        r: *const xcb_input_xi_get_selected_events_reply_t,
    ) -> xcb_input_event_mask_iterator_t {
        sym!(self, xcb_input_xi_get_selected_events_masks_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_selected_events_masks_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_selected_events_masks_iterator(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_selected_events_masks_iterator)
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
     * xcb_input_xi_get_selected_events_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_input_xi_get_selected_events_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_xi_get_selected_events_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_input_xi_get_selected_events_reply_t {
        sym!(self, xcb_input_xi_get_selected_events_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_selected_events_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_selected_events_reply(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_selected_events_reply)
    }

    pub unsafe fn xcb_input_barrier_release_pointer_info_next(
        &self,
        i: *mut xcb_input_barrier_release_pointer_info_iterator_t,
    ) {
        sym!(self, xcb_input_barrier_release_pointer_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_barrier_release_pointer_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_barrier_release_pointer_info_next(&self) -> bool {
        has_sym!(self, xcb_input_barrier_release_pointer_info_next)
    }

    pub unsafe fn xcb_input_barrier_release_pointer_info_end(
        &self,
        i: xcb_input_barrier_release_pointer_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_barrier_release_pointer_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_barrier_release_pointer_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_barrier_release_pointer_info_end(&self) -> bool {
        has_sym!(self, xcb_input_barrier_release_pointer_info_end)
    }

    pub unsafe fn xcb_input_xi_barrier_release_pointer_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_input_xi_barrier_release_pointer_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_barrier_release_pointer_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_barrier_release_pointer_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_barrier_release_pointer_sizeof)
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
    pub unsafe fn xcb_input_xi_barrier_release_pointer_checked(
        &self,
        c: *mut xcb_connection_t,
        num_barriers: u32,
        barriers: *const xcb_input_barrier_release_pointer_info_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_barrier_release_pointer_checked)(c, num_barriers, barriers)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_barrier_release_pointer_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_barrier_release_pointer_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_barrier_release_pointer_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_xi_barrier_release_pointer(
        &self,
        c: *mut xcb_connection_t,
        num_barriers: u32,
        barriers: *const xcb_input_barrier_release_pointer_info_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_barrier_release_pointer)(c, num_barriers, barriers)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_barrier_release_pointer` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_barrier_release_pointer(&self) -> bool {
        has_sym!(self, xcb_input_xi_barrier_release_pointer)
    }

    pub unsafe fn xcb_input_xi_barrier_release_pointer_barriers(
        &self,
        r: *const xcb_input_xi_barrier_release_pointer_request_t,
    ) -> *mut xcb_input_barrier_release_pointer_info_t {
        sym!(self, xcb_input_xi_barrier_release_pointer_barriers)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_barrier_release_pointer_barriers` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_barrier_release_pointer_barriers(&self) -> bool {
        has_sym!(self, xcb_input_xi_barrier_release_pointer_barriers)
    }

    pub unsafe fn xcb_input_xi_barrier_release_pointer_barriers_length(
        &self,
        r: *const xcb_input_xi_barrier_release_pointer_request_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_barrier_release_pointer_barriers_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_barrier_release_pointer_barriers_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_barrier_release_pointer_barriers_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_barrier_release_pointer_barriers_length)
    }

    pub unsafe fn xcb_input_xi_barrier_release_pointer_barriers_iterator(
        &self,
        r: *const xcb_input_xi_barrier_release_pointer_request_t,
    ) -> xcb_input_barrier_release_pointer_info_iterator_t {
        sym!(self, xcb_input_xi_barrier_release_pointer_barriers_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_barrier_release_pointer_barriers_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_barrier_release_pointer_barriers_iterator(&self) -> bool {
        has_sym!(self, xcb_input_xi_barrier_release_pointer_barriers_iterator)
    }

    pub unsafe fn xcb_input_device_changed_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_device_changed_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_device_changed_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_changed_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_device_changed_sizeof)
    }

    pub unsafe fn xcb_input_device_changed_classes_length(
        &self,
        r: *const xcb_input_device_changed_event_t,
    ) -> c_int {
        sym!(self, xcb_input_device_changed_classes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_changed_classes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_changed_classes_length(&self) -> bool {
        has_sym!(self, xcb_input_device_changed_classes_length)
    }

    pub unsafe fn xcb_input_device_changed_classes_iterator(
        &self,
        r: *const xcb_input_device_changed_event_t,
    ) -> xcb_input_device_class_iterator_t {
        sym!(self, xcb_input_device_changed_classes_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_changed_classes_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_changed_classes_iterator(&self) -> bool {
        has_sym!(self, xcb_input_device_changed_classes_iterator)
    }

    pub unsafe fn xcb_input_key_press_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_key_press_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_key_press_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_press_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_key_press_sizeof)
    }

    pub unsafe fn xcb_input_key_press_button_mask(
        &self,
        r: *const xcb_input_key_press_event_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_key_press_button_mask)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_key_press_button_mask` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_press_button_mask(&self) -> bool {
        has_sym!(self, xcb_input_key_press_button_mask)
    }

    pub unsafe fn xcb_input_key_press_button_mask_length(
        &self,
        r: *const xcb_input_key_press_event_t,
    ) -> c_int {
        sym!(self, xcb_input_key_press_button_mask_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_key_press_button_mask_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_press_button_mask_length(&self) -> bool {
        has_sym!(self, xcb_input_key_press_button_mask_length)
    }

    pub unsafe fn xcb_input_key_press_button_mask_end(
        &self,
        r: *const xcb_input_key_press_event_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_key_press_button_mask_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_key_press_button_mask_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_press_button_mask_end(&self) -> bool {
        has_sym!(self, xcb_input_key_press_button_mask_end)
    }

    pub unsafe fn xcb_input_key_press_valuator_mask(
        &self,
        r: *const xcb_input_key_press_event_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_key_press_valuator_mask)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_key_press_valuator_mask` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_press_valuator_mask(&self) -> bool {
        has_sym!(self, xcb_input_key_press_valuator_mask)
    }

    pub unsafe fn xcb_input_key_press_valuator_mask_length(
        &self,
        r: *const xcb_input_key_press_event_t,
    ) -> c_int {
        sym!(self, xcb_input_key_press_valuator_mask_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_key_press_valuator_mask_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_press_valuator_mask_length(&self) -> bool {
        has_sym!(self, xcb_input_key_press_valuator_mask_length)
    }

    pub unsafe fn xcb_input_key_press_valuator_mask_end(
        &self,
        r: *const xcb_input_key_press_event_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_key_press_valuator_mask_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_key_press_valuator_mask_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_press_valuator_mask_end(&self) -> bool {
        has_sym!(self, xcb_input_key_press_valuator_mask_end)
    }

    pub unsafe fn xcb_input_key_press_axisvalues(
        &self,
        r: *const xcb_input_key_press_event_t,
    ) -> *mut xcb_input_fp3232_t {
        sym!(self, xcb_input_key_press_axisvalues)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_key_press_axisvalues` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_press_axisvalues(&self) -> bool {
        has_sym!(self, xcb_input_key_press_axisvalues)
    }

    pub unsafe fn xcb_input_key_press_axisvalues_length(
        &self,
        r: *const xcb_input_key_press_event_t,
    ) -> c_int {
        sym!(self, xcb_input_key_press_axisvalues_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_key_press_axisvalues_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_press_axisvalues_length(&self) -> bool {
        has_sym!(self, xcb_input_key_press_axisvalues_length)
    }

    pub unsafe fn xcb_input_key_press_axisvalues_iterator(
        &self,
        r: *const xcb_input_key_press_event_t,
    ) -> xcb_input_fp3232_iterator_t {
        sym!(self, xcb_input_key_press_axisvalues_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_key_press_axisvalues_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_press_axisvalues_iterator(&self) -> bool {
        has_sym!(self, xcb_input_key_press_axisvalues_iterator)
    }
    pub unsafe fn xcb_input_key_release_sizeof(&self, buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_key_release_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_key_release_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_release_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_key_release_sizeof)
    }

    pub unsafe fn xcb_input_button_press_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_button_press_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_button_press_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_press_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_button_press_sizeof)
    }

    pub unsafe fn xcb_input_button_press_button_mask(
        &self,
        r: *const xcb_input_button_press_event_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_button_press_button_mask)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_press_button_mask` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_press_button_mask(&self) -> bool {
        has_sym!(self, xcb_input_button_press_button_mask)
    }

    pub unsafe fn xcb_input_button_press_button_mask_length(
        &self,
        r: *const xcb_input_button_press_event_t,
    ) -> c_int {
        sym!(self, xcb_input_button_press_button_mask_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_press_button_mask_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_press_button_mask_length(&self) -> bool {
        has_sym!(self, xcb_input_button_press_button_mask_length)
    }

    pub unsafe fn xcb_input_button_press_button_mask_end(
        &self,
        r: *const xcb_input_button_press_event_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_button_press_button_mask_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_press_button_mask_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_press_button_mask_end(&self) -> bool {
        has_sym!(self, xcb_input_button_press_button_mask_end)
    }

    pub unsafe fn xcb_input_button_press_valuator_mask(
        &self,
        r: *const xcb_input_button_press_event_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_button_press_valuator_mask)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_press_valuator_mask` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_press_valuator_mask(&self) -> bool {
        has_sym!(self, xcb_input_button_press_valuator_mask)
    }

    pub unsafe fn xcb_input_button_press_valuator_mask_length(
        &self,
        r: *const xcb_input_button_press_event_t,
    ) -> c_int {
        sym!(self, xcb_input_button_press_valuator_mask_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_press_valuator_mask_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_press_valuator_mask_length(&self) -> bool {
        has_sym!(self, xcb_input_button_press_valuator_mask_length)
    }

    pub unsafe fn xcb_input_button_press_valuator_mask_end(
        &self,
        r: *const xcb_input_button_press_event_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_button_press_valuator_mask_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_press_valuator_mask_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_press_valuator_mask_end(&self) -> bool {
        has_sym!(self, xcb_input_button_press_valuator_mask_end)
    }

    pub unsafe fn xcb_input_button_press_axisvalues(
        &self,
        r: *const xcb_input_button_press_event_t,
    ) -> *mut xcb_input_fp3232_t {
        sym!(self, xcb_input_button_press_axisvalues)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_press_axisvalues` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_press_axisvalues(&self) -> bool {
        has_sym!(self, xcb_input_button_press_axisvalues)
    }

    pub unsafe fn xcb_input_button_press_axisvalues_length(
        &self,
        r: *const xcb_input_button_press_event_t,
    ) -> c_int {
        sym!(self, xcb_input_button_press_axisvalues_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_press_axisvalues_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_press_axisvalues_length(&self) -> bool {
        has_sym!(self, xcb_input_button_press_axisvalues_length)
    }

    pub unsafe fn xcb_input_button_press_axisvalues_iterator(
        &self,
        r: *const xcb_input_button_press_event_t,
    ) -> xcb_input_fp3232_iterator_t {
        sym!(self, xcb_input_button_press_axisvalues_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_press_axisvalues_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_press_axisvalues_iterator(&self) -> bool {
        has_sym!(self, xcb_input_button_press_axisvalues_iterator)
    }
    pub unsafe fn xcb_input_button_release_sizeof(&self, buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_button_release_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_button_release_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_release_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_button_release_sizeof)
    }
    pub unsafe fn xcb_input_motion_sizeof(&self, buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_motion_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_motion_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_motion_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_motion_sizeof)
    }

    pub unsafe fn xcb_input_enter_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_enter_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_enter_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_enter_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_enter_sizeof)
    }

    pub unsafe fn xcb_input_enter_buttons(&self, r: *const xcb_input_enter_event_t) -> *mut u32 {
        sym!(self, xcb_input_enter_buttons)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_enter_buttons` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_enter_buttons(&self) -> bool {
        has_sym!(self, xcb_input_enter_buttons)
    }

    pub unsafe fn xcb_input_enter_buttons_length(
        &self,
        r: *const xcb_input_enter_event_t,
    ) -> c_int {
        sym!(self, xcb_input_enter_buttons_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_enter_buttons_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_enter_buttons_length(&self) -> bool {
        has_sym!(self, xcb_input_enter_buttons_length)
    }

    pub unsafe fn xcb_input_enter_buttons_end(
        &self,
        r: *const xcb_input_enter_event_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_enter_buttons_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_enter_buttons_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_enter_buttons_end(&self) -> bool {
        has_sym!(self, xcb_input_enter_buttons_end)
    }
    pub unsafe fn xcb_input_leave_sizeof(&self, buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_leave_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_leave_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_leave_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_leave_sizeof)
    }
    pub unsafe fn xcb_input_focus_in_sizeof(&self, buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_focus_in_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_focus_in_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_focus_in_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_focus_in_sizeof)
    }
    pub unsafe fn xcb_input_focus_out_sizeof(&self, buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_focus_out_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_focus_out_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_focus_out_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_focus_out_sizeof)
    }

    pub unsafe fn xcb_input_hierarchy_info_next(
        &self,
        i: *mut xcb_input_hierarchy_info_iterator_t,
    ) {
        sym!(self, xcb_input_hierarchy_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_info_next(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_info_next)
    }

    pub unsafe fn xcb_input_hierarchy_info_end(
        &self,
        i: xcb_input_hierarchy_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_hierarchy_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_info_end(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_info_end)
    }

    pub unsafe fn xcb_input_hierarchy_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_hierarchy_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_sizeof)
    }

    pub unsafe fn xcb_input_hierarchy_infos(
        &self,
        r: *const xcb_input_hierarchy_event_t,
    ) -> *mut xcb_input_hierarchy_info_t {
        sym!(self, xcb_input_hierarchy_infos)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_infos` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_infos(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_infos)
    }

    pub unsafe fn xcb_input_hierarchy_infos_length(
        &self,
        r: *const xcb_input_hierarchy_event_t,
    ) -> c_int {
        sym!(self, xcb_input_hierarchy_infos_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_infos_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_infos_length(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_infos_length)
    }

    pub unsafe fn xcb_input_hierarchy_infos_iterator(
        &self,
        r: *const xcb_input_hierarchy_event_t,
    ) -> xcb_input_hierarchy_info_iterator_t {
        sym!(self, xcb_input_hierarchy_infos_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_infos_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_infos_iterator(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_infos_iterator)
    }

    pub unsafe fn xcb_input_raw_key_press_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_raw_key_press_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_key_press_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_key_press_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_raw_key_press_sizeof)
    }

    pub unsafe fn xcb_input_raw_key_press_valuator_mask(
        &self,
        r: *const xcb_input_raw_key_press_event_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_raw_key_press_valuator_mask)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_key_press_valuator_mask` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_key_press_valuator_mask(&self) -> bool {
        has_sym!(self, xcb_input_raw_key_press_valuator_mask)
    }

    pub unsafe fn xcb_input_raw_key_press_valuator_mask_length(
        &self,
        r: *const xcb_input_raw_key_press_event_t,
    ) -> c_int {
        sym!(self, xcb_input_raw_key_press_valuator_mask_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_key_press_valuator_mask_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_key_press_valuator_mask_length(&self) -> bool {
        has_sym!(self, xcb_input_raw_key_press_valuator_mask_length)
    }

    pub unsafe fn xcb_input_raw_key_press_valuator_mask_end(
        &self,
        r: *const xcb_input_raw_key_press_event_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_raw_key_press_valuator_mask_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_key_press_valuator_mask_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_key_press_valuator_mask_end(&self) -> bool {
        has_sym!(self, xcb_input_raw_key_press_valuator_mask_end)
    }

    pub unsafe fn xcb_input_raw_key_press_axisvalues(
        &self,
        r: *const xcb_input_raw_key_press_event_t,
    ) -> *mut xcb_input_fp3232_t {
        sym!(self, xcb_input_raw_key_press_axisvalues)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_key_press_axisvalues` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_key_press_axisvalues(&self) -> bool {
        has_sym!(self, xcb_input_raw_key_press_axisvalues)
    }

    pub unsafe fn xcb_input_raw_key_press_axisvalues_length(
        &self,
        r: *const xcb_input_raw_key_press_event_t,
    ) -> c_int {
        sym!(self, xcb_input_raw_key_press_axisvalues_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_key_press_axisvalues_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_key_press_axisvalues_length(&self) -> bool {
        has_sym!(self, xcb_input_raw_key_press_axisvalues_length)
    }

    pub unsafe fn xcb_input_raw_key_press_axisvalues_iterator(
        &self,
        r: *const xcb_input_raw_key_press_event_t,
    ) -> xcb_input_fp3232_iterator_t {
        sym!(self, xcb_input_raw_key_press_axisvalues_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_key_press_axisvalues_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_key_press_axisvalues_iterator(&self) -> bool {
        has_sym!(self, xcb_input_raw_key_press_axisvalues_iterator)
    }

    pub unsafe fn xcb_input_raw_key_press_axisvalues_raw(
        &self,
        r: *const xcb_input_raw_key_press_event_t,
    ) -> *mut xcb_input_fp3232_t {
        sym!(self, xcb_input_raw_key_press_axisvalues_raw)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_key_press_axisvalues_raw` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_key_press_axisvalues_raw(&self) -> bool {
        has_sym!(self, xcb_input_raw_key_press_axisvalues_raw)
    }

    pub unsafe fn xcb_input_raw_key_press_axisvalues_raw_length(
        &self,
        r: *const xcb_input_raw_key_press_event_t,
    ) -> c_int {
        sym!(self, xcb_input_raw_key_press_axisvalues_raw_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_key_press_axisvalues_raw_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_key_press_axisvalues_raw_length(&self) -> bool {
        has_sym!(self, xcb_input_raw_key_press_axisvalues_raw_length)
    }

    pub unsafe fn xcb_input_raw_key_press_axisvalues_raw_iterator(
        &self,
        r: *const xcb_input_raw_key_press_event_t,
    ) -> xcb_input_fp3232_iterator_t {
        sym!(self, xcb_input_raw_key_press_axisvalues_raw_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_key_press_axisvalues_raw_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_key_press_axisvalues_raw_iterator(&self) -> bool {
        has_sym!(self, xcb_input_raw_key_press_axisvalues_raw_iterator)
    }
    pub unsafe fn xcb_input_raw_key_release_sizeof(&self, buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_raw_key_release_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_key_release_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_key_release_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_raw_key_release_sizeof)
    }

    pub unsafe fn xcb_input_raw_button_press_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_raw_button_press_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_button_press_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_button_press_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_raw_button_press_sizeof)
    }

    pub unsafe fn xcb_input_raw_button_press_valuator_mask(
        &self,
        r: *const xcb_input_raw_button_press_event_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_raw_button_press_valuator_mask)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_button_press_valuator_mask` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_button_press_valuator_mask(&self) -> bool {
        has_sym!(self, xcb_input_raw_button_press_valuator_mask)
    }

    pub unsafe fn xcb_input_raw_button_press_valuator_mask_length(
        &self,
        r: *const xcb_input_raw_button_press_event_t,
    ) -> c_int {
        sym!(self, xcb_input_raw_button_press_valuator_mask_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_button_press_valuator_mask_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_button_press_valuator_mask_length(&self) -> bool {
        has_sym!(self, xcb_input_raw_button_press_valuator_mask_length)
    }

    pub unsafe fn xcb_input_raw_button_press_valuator_mask_end(
        &self,
        r: *const xcb_input_raw_button_press_event_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_raw_button_press_valuator_mask_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_button_press_valuator_mask_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_button_press_valuator_mask_end(&self) -> bool {
        has_sym!(self, xcb_input_raw_button_press_valuator_mask_end)
    }

    pub unsafe fn xcb_input_raw_button_press_axisvalues(
        &self,
        r: *const xcb_input_raw_button_press_event_t,
    ) -> *mut xcb_input_fp3232_t {
        sym!(self, xcb_input_raw_button_press_axisvalues)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_button_press_axisvalues` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_button_press_axisvalues(&self) -> bool {
        has_sym!(self, xcb_input_raw_button_press_axisvalues)
    }

    pub unsafe fn xcb_input_raw_button_press_axisvalues_length(
        &self,
        r: *const xcb_input_raw_button_press_event_t,
    ) -> c_int {
        sym!(self, xcb_input_raw_button_press_axisvalues_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_button_press_axisvalues_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_button_press_axisvalues_length(&self) -> bool {
        has_sym!(self, xcb_input_raw_button_press_axisvalues_length)
    }

    pub unsafe fn xcb_input_raw_button_press_axisvalues_iterator(
        &self,
        r: *const xcb_input_raw_button_press_event_t,
    ) -> xcb_input_fp3232_iterator_t {
        sym!(self, xcb_input_raw_button_press_axisvalues_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_button_press_axisvalues_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_button_press_axisvalues_iterator(&self) -> bool {
        has_sym!(self, xcb_input_raw_button_press_axisvalues_iterator)
    }

    pub unsafe fn xcb_input_raw_button_press_axisvalues_raw(
        &self,
        r: *const xcb_input_raw_button_press_event_t,
    ) -> *mut xcb_input_fp3232_t {
        sym!(self, xcb_input_raw_button_press_axisvalues_raw)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_button_press_axisvalues_raw` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_button_press_axisvalues_raw(&self) -> bool {
        has_sym!(self, xcb_input_raw_button_press_axisvalues_raw)
    }

    pub unsafe fn xcb_input_raw_button_press_axisvalues_raw_length(
        &self,
        r: *const xcb_input_raw_button_press_event_t,
    ) -> c_int {
        sym!(self, xcb_input_raw_button_press_axisvalues_raw_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_button_press_axisvalues_raw_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_button_press_axisvalues_raw_length(&self) -> bool {
        has_sym!(self, xcb_input_raw_button_press_axisvalues_raw_length)
    }

    pub unsafe fn xcb_input_raw_button_press_axisvalues_raw_iterator(
        &self,
        r: *const xcb_input_raw_button_press_event_t,
    ) -> xcb_input_fp3232_iterator_t {
        sym!(self, xcb_input_raw_button_press_axisvalues_raw_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_button_press_axisvalues_raw_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_button_press_axisvalues_raw_iterator(&self) -> bool {
        has_sym!(self, xcb_input_raw_button_press_axisvalues_raw_iterator)
    }
    pub unsafe fn xcb_input_raw_button_release_sizeof(&self, buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_raw_button_release_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_button_release_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_button_release_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_raw_button_release_sizeof)
    }
    pub unsafe fn xcb_input_raw_motion_sizeof(&self, buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_raw_motion_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_motion_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_motion_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_raw_motion_sizeof)
    }

    pub unsafe fn xcb_input_touch_begin_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_touch_begin_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_begin_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_begin_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_touch_begin_sizeof)
    }

    pub unsafe fn xcb_input_touch_begin_button_mask(
        &self,
        r: *const xcb_input_touch_begin_event_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_touch_begin_button_mask)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_begin_button_mask` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_begin_button_mask(&self) -> bool {
        has_sym!(self, xcb_input_touch_begin_button_mask)
    }

    pub unsafe fn xcb_input_touch_begin_button_mask_length(
        &self,
        r: *const xcb_input_touch_begin_event_t,
    ) -> c_int {
        sym!(self, xcb_input_touch_begin_button_mask_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_begin_button_mask_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_begin_button_mask_length(&self) -> bool {
        has_sym!(self, xcb_input_touch_begin_button_mask_length)
    }

    pub unsafe fn xcb_input_touch_begin_button_mask_end(
        &self,
        r: *const xcb_input_touch_begin_event_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_touch_begin_button_mask_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_begin_button_mask_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_begin_button_mask_end(&self) -> bool {
        has_sym!(self, xcb_input_touch_begin_button_mask_end)
    }

    pub unsafe fn xcb_input_touch_begin_valuator_mask(
        &self,
        r: *const xcb_input_touch_begin_event_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_touch_begin_valuator_mask)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_begin_valuator_mask` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_begin_valuator_mask(&self) -> bool {
        has_sym!(self, xcb_input_touch_begin_valuator_mask)
    }

    pub unsafe fn xcb_input_touch_begin_valuator_mask_length(
        &self,
        r: *const xcb_input_touch_begin_event_t,
    ) -> c_int {
        sym!(self, xcb_input_touch_begin_valuator_mask_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_begin_valuator_mask_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_begin_valuator_mask_length(&self) -> bool {
        has_sym!(self, xcb_input_touch_begin_valuator_mask_length)
    }

    pub unsafe fn xcb_input_touch_begin_valuator_mask_end(
        &self,
        r: *const xcb_input_touch_begin_event_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_touch_begin_valuator_mask_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_begin_valuator_mask_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_begin_valuator_mask_end(&self) -> bool {
        has_sym!(self, xcb_input_touch_begin_valuator_mask_end)
    }

    pub unsafe fn xcb_input_touch_begin_axisvalues(
        &self,
        r: *const xcb_input_touch_begin_event_t,
    ) -> *mut xcb_input_fp3232_t {
        sym!(self, xcb_input_touch_begin_axisvalues)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_begin_axisvalues` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_begin_axisvalues(&self) -> bool {
        has_sym!(self, xcb_input_touch_begin_axisvalues)
    }

    pub unsafe fn xcb_input_touch_begin_axisvalues_length(
        &self,
        r: *const xcb_input_touch_begin_event_t,
    ) -> c_int {
        sym!(self, xcb_input_touch_begin_axisvalues_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_begin_axisvalues_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_begin_axisvalues_length(&self) -> bool {
        has_sym!(self, xcb_input_touch_begin_axisvalues_length)
    }

    pub unsafe fn xcb_input_touch_begin_axisvalues_iterator(
        &self,
        r: *const xcb_input_touch_begin_event_t,
    ) -> xcb_input_fp3232_iterator_t {
        sym!(self, xcb_input_touch_begin_axisvalues_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_begin_axisvalues_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_begin_axisvalues_iterator(&self) -> bool {
        has_sym!(self, xcb_input_touch_begin_axisvalues_iterator)
    }
    pub unsafe fn xcb_input_touch_update_sizeof(&self, buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_touch_update_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_update_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_update_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_touch_update_sizeof)
    }
    pub unsafe fn xcb_input_touch_end_sizeof(&self, buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_touch_end_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_end_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_end_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_touch_end_sizeof)
    }

    pub unsafe fn xcb_input_raw_touch_begin_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_raw_touch_begin_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_touch_begin_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_touch_begin_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_raw_touch_begin_sizeof)
    }

    pub unsafe fn xcb_input_raw_touch_begin_valuator_mask(
        &self,
        r: *const xcb_input_raw_touch_begin_event_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_raw_touch_begin_valuator_mask)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_touch_begin_valuator_mask` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_touch_begin_valuator_mask(&self) -> bool {
        has_sym!(self, xcb_input_raw_touch_begin_valuator_mask)
    }

    pub unsafe fn xcb_input_raw_touch_begin_valuator_mask_length(
        &self,
        r: *const xcb_input_raw_touch_begin_event_t,
    ) -> c_int {
        sym!(self, xcb_input_raw_touch_begin_valuator_mask_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_touch_begin_valuator_mask_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_touch_begin_valuator_mask_length(&self) -> bool {
        has_sym!(self, xcb_input_raw_touch_begin_valuator_mask_length)
    }

    pub unsafe fn xcb_input_raw_touch_begin_valuator_mask_end(
        &self,
        r: *const xcb_input_raw_touch_begin_event_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_raw_touch_begin_valuator_mask_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_touch_begin_valuator_mask_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_touch_begin_valuator_mask_end(&self) -> bool {
        has_sym!(self, xcb_input_raw_touch_begin_valuator_mask_end)
    }

    pub unsafe fn xcb_input_raw_touch_begin_axisvalues(
        &self,
        r: *const xcb_input_raw_touch_begin_event_t,
    ) -> *mut xcb_input_fp3232_t {
        sym!(self, xcb_input_raw_touch_begin_axisvalues)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_touch_begin_axisvalues` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_touch_begin_axisvalues(&self) -> bool {
        has_sym!(self, xcb_input_raw_touch_begin_axisvalues)
    }

    pub unsafe fn xcb_input_raw_touch_begin_axisvalues_length(
        &self,
        r: *const xcb_input_raw_touch_begin_event_t,
    ) -> c_int {
        sym!(self, xcb_input_raw_touch_begin_axisvalues_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_touch_begin_axisvalues_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_touch_begin_axisvalues_length(&self) -> bool {
        has_sym!(self, xcb_input_raw_touch_begin_axisvalues_length)
    }

    pub unsafe fn xcb_input_raw_touch_begin_axisvalues_iterator(
        &self,
        r: *const xcb_input_raw_touch_begin_event_t,
    ) -> xcb_input_fp3232_iterator_t {
        sym!(self, xcb_input_raw_touch_begin_axisvalues_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_touch_begin_axisvalues_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_touch_begin_axisvalues_iterator(&self) -> bool {
        has_sym!(self, xcb_input_raw_touch_begin_axisvalues_iterator)
    }

    pub unsafe fn xcb_input_raw_touch_begin_axisvalues_raw(
        &self,
        r: *const xcb_input_raw_touch_begin_event_t,
    ) -> *mut xcb_input_fp3232_t {
        sym!(self, xcb_input_raw_touch_begin_axisvalues_raw)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_touch_begin_axisvalues_raw` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_touch_begin_axisvalues_raw(&self) -> bool {
        has_sym!(self, xcb_input_raw_touch_begin_axisvalues_raw)
    }

    pub unsafe fn xcb_input_raw_touch_begin_axisvalues_raw_length(
        &self,
        r: *const xcb_input_raw_touch_begin_event_t,
    ) -> c_int {
        sym!(self, xcb_input_raw_touch_begin_axisvalues_raw_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_touch_begin_axisvalues_raw_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_touch_begin_axisvalues_raw_length(&self) -> bool {
        has_sym!(self, xcb_input_raw_touch_begin_axisvalues_raw_length)
    }

    pub unsafe fn xcb_input_raw_touch_begin_axisvalues_raw_iterator(
        &self,
        r: *const xcb_input_raw_touch_begin_event_t,
    ) -> xcb_input_fp3232_iterator_t {
        sym!(self, xcb_input_raw_touch_begin_axisvalues_raw_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_touch_begin_axisvalues_raw_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_touch_begin_axisvalues_raw_iterator(&self) -> bool {
        has_sym!(self, xcb_input_raw_touch_begin_axisvalues_raw_iterator)
    }
    pub unsafe fn xcb_input_raw_touch_update_sizeof(&self, buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_raw_touch_update_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_touch_update_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_touch_update_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_raw_touch_update_sizeof)
    }
    pub unsafe fn xcb_input_raw_touch_end_sizeof(&self, buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_raw_touch_end_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_touch_end_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_touch_end_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_raw_touch_end_sizeof)
    }

    pub unsafe fn xcb_input_event_for_send_next(
        &self,
        i: *mut xcb_input_event_for_send_iterator_t,
    ) {
        sym!(self, xcb_input_event_for_send_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_event_for_send_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_event_for_send_next(&self) -> bool {
        has_sym!(self, xcb_input_event_for_send_next)
    }

    pub unsafe fn xcb_input_event_for_send_end(
        &self,
        i: xcb_input_event_for_send_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_event_for_send_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_event_for_send_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_event_for_send_end(&self) -> bool {
        has_sym!(self, xcb_input_event_for_send_end)
    }

    pub unsafe fn xcb_input_send_extension_event_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_send_extension_event_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_send_extension_event_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_send_extension_event_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_send_extension_event_sizeof)
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
    pub unsafe fn xcb_input_send_extension_event_checked(
        &self,
        c: *mut xcb_connection_t,
        destination: xcb_window_t,
        device_id: u8,
        propagate: u8,
        num_classes: u16,
        num_events: u8,
        events: *const xcb_input_event_for_send_t,
        classes: *const xcb_input_event_class_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_send_extension_event_checked)(
            c,
            destination,
            device_id,
            propagate,
            num_classes,
            num_events,
            events,
            classes,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_send_extension_event_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_send_extension_event_checked(&self) -> bool {
        has_sym!(self, xcb_input_send_extension_event_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_input_send_extension_event(
        &self,
        c: *mut xcb_connection_t,
        destination: xcb_window_t,
        device_id: u8,
        propagate: u8,
        num_classes: u16,
        num_events: u8,
        events: *const xcb_input_event_for_send_t,
        classes: *const xcb_input_event_class_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_send_extension_event)(
            c,
            destination,
            device_id,
            propagate,
            num_classes,
            num_events,
            events,
            classes,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_send_extension_event` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_send_extension_event(&self) -> bool {
        has_sym!(self, xcb_input_send_extension_event)
    }

    pub unsafe fn xcb_input_send_extension_event_events(
        &self,
        r: *const xcb_input_send_extension_event_request_t,
    ) -> *mut xcb_input_event_for_send_t {
        sym!(self, xcb_input_send_extension_event_events)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_send_extension_event_events` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_send_extension_event_events(&self) -> bool {
        has_sym!(self, xcb_input_send_extension_event_events)
    }

    pub unsafe fn xcb_input_send_extension_event_events_length(
        &self,
        r: *const xcb_input_send_extension_event_request_t,
    ) -> c_int {
        sym!(self, xcb_input_send_extension_event_events_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_send_extension_event_events_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_send_extension_event_events_length(&self) -> bool {
        has_sym!(self, xcb_input_send_extension_event_events_length)
    }

    pub unsafe fn xcb_input_send_extension_event_events_iterator(
        &self,
        r: *const xcb_input_send_extension_event_request_t,
    ) -> xcb_input_event_for_send_iterator_t {
        sym!(self, xcb_input_send_extension_event_events_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_send_extension_event_events_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_send_extension_event_events_iterator(&self) -> bool {
        has_sym!(self, xcb_input_send_extension_event_events_iterator)
    }

    pub unsafe fn xcb_input_send_extension_event_classes(
        &self,
        r: *const xcb_input_send_extension_event_request_t,
    ) -> *mut xcb_input_event_class_t {
        sym!(self, xcb_input_send_extension_event_classes)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_send_extension_event_classes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_send_extension_event_classes(&self) -> bool {
        has_sym!(self, xcb_input_send_extension_event_classes)
    }

    pub unsafe fn xcb_input_send_extension_event_classes_length(
        &self,
        r: *const xcb_input_send_extension_event_request_t,
    ) -> c_int {
        sym!(self, xcb_input_send_extension_event_classes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_send_extension_event_classes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_send_extension_event_classes_length(&self) -> bool {
        has_sym!(self, xcb_input_send_extension_event_classes_length)
    }

    pub unsafe fn xcb_input_send_extension_event_classes_end(
        &self,
        r: *const xcb_input_send_extension_event_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_send_extension_event_classes_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_send_extension_event_classes_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_send_extension_event_classes_end(&self) -> bool {
        has_sym!(self, xcb_input_send_extension_event_classes_end)
    }
}

#[cfg(feature = "xcb_xinput")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbXinput::load().unwrap() };
        assert!(lib.has_xcb_input_id());
        assert!(lib.has_xcb_input_event_class_next());
        assert!(lib.has_xcb_input_event_class_end());
        assert!(lib.has_xcb_input_key_code_next());
        assert!(lib.has_xcb_input_key_code_end());
        assert!(lib.has_xcb_input_device_id_next());
        assert!(lib.has_xcb_input_device_id_end());
        assert!(lib.has_xcb_input_fp1616_next());
        assert!(lib.has_xcb_input_fp1616_end());
        assert!(lib.has_xcb_input_fp3232_next());
        assert!(lib.has_xcb_input_fp3232_end());
        assert!(lib.has_xcb_input_get_extension_version_sizeof());
        assert!(lib.has_xcb_input_get_extension_version());
        assert!(lib.has_xcb_input_get_extension_version_unchecked());
        assert!(lib.has_xcb_input_get_extension_version_reply());
        assert!(lib.has_xcb_input_device_info_next());
        assert!(lib.has_xcb_input_device_info_end());
        assert!(lib.has_xcb_input_key_info_next());
        assert!(lib.has_xcb_input_key_info_end());
        assert!(lib.has_xcb_input_button_info_next());
        assert!(lib.has_xcb_input_button_info_end());
        assert!(lib.has_xcb_input_axis_info_next());
        assert!(lib.has_xcb_input_axis_info_end());
        assert!(lib.has_xcb_input_valuator_info_sizeof());
        assert!(lib.has_xcb_input_valuator_info_axes());
        assert!(lib.has_xcb_input_valuator_info_axes_length());
        assert!(lib.has_xcb_input_valuator_info_axes_iterator());
        assert!(lib.has_xcb_input_valuator_info_next());
        assert!(lib.has_xcb_input_valuator_info_end());
        assert!(lib.has_xcb_input_input_info_info_valuator_axes());
        assert!(lib.has_xcb_input_input_info_info_valuator_axes_length());
        assert!(lib.has_xcb_input_input_info_info_valuator_axes_iterator());
        assert!(lib.has_xcb_input_input_info_info_serialize());
        assert!(lib.has_xcb_input_input_info_info_unpack());
        assert!(lib.has_xcb_input_input_info_info_sizeof());
        assert!(lib.has_xcb_input_input_info_sizeof());
        assert!(lib.has_xcb_input_input_info_info());
        assert!(lib.has_xcb_input_input_info_next());
        assert!(lib.has_xcb_input_input_info_end());
        assert!(lib.has_xcb_input_device_name_sizeof());
        assert!(lib.has_xcb_input_device_name_string());
        assert!(lib.has_xcb_input_device_name_string_length());
        assert!(lib.has_xcb_input_device_name_string_end());
        assert!(lib.has_xcb_input_device_name_next());
        assert!(lib.has_xcb_input_device_name_end());
        assert!(lib.has_xcb_input_list_input_devices_sizeof());
        assert!(lib.has_xcb_input_list_input_devices());
        assert!(lib.has_xcb_input_list_input_devices_unchecked());
        assert!(lib.has_xcb_input_list_input_devices_devices());
        assert!(lib.has_xcb_input_list_input_devices_devices_length());
        assert!(lib.has_xcb_input_list_input_devices_devices_iterator());
        assert!(lib.has_xcb_input_list_input_devices_infos_length());
        assert!(lib.has_xcb_input_list_input_devices_infos_iterator());
        assert!(lib.has_xcb_input_list_input_devices_names_length());
        assert!(lib.has_xcb_input_list_input_devices_names_iterator());
        assert!(lib.has_xcb_input_list_input_devices_reply());
        assert!(lib.has_xcb_input_event_type_base_next());
        assert!(lib.has_xcb_input_event_type_base_end());
        assert!(lib.has_xcb_input_input_class_info_next());
        assert!(lib.has_xcb_input_input_class_info_end());
        assert!(lib.has_xcb_input_open_device_sizeof());
        assert!(lib.has_xcb_input_open_device());
        assert!(lib.has_xcb_input_open_device_unchecked());
        assert!(lib.has_xcb_input_open_device_class_info());
        assert!(lib.has_xcb_input_open_device_class_info_length());
        assert!(lib.has_xcb_input_open_device_class_info_iterator());
        assert!(lib.has_xcb_input_open_device_reply());
        assert!(lib.has_xcb_input_close_device_checked());
        assert!(lib.has_xcb_input_close_device());
        assert!(lib.has_xcb_input_set_device_mode());
        assert!(lib.has_xcb_input_set_device_mode_unchecked());
        assert!(lib.has_xcb_input_set_device_mode_reply());
        assert!(lib.has_xcb_input_select_extension_event_sizeof());
        assert!(lib.has_xcb_input_select_extension_event_checked());
        assert!(lib.has_xcb_input_select_extension_event());
        assert!(lib.has_xcb_input_select_extension_event_classes());
        assert!(lib.has_xcb_input_select_extension_event_classes_length());
        assert!(lib.has_xcb_input_select_extension_event_classes_end());
        assert!(lib.has_xcb_input_get_selected_extension_events_sizeof());
        assert!(lib.has_xcb_input_get_selected_extension_events());
        assert!(lib.has_xcb_input_get_selected_extension_events_unchecked());
        assert!(lib.has_xcb_input_get_selected_extension_events_this_classes());
        assert!(lib.has_xcb_input_get_selected_extension_events_this_classes_length());
        assert!(lib.has_xcb_input_get_selected_extension_events_this_classes_end());
        assert!(lib.has_xcb_input_get_selected_extension_events_all_classes());
        assert!(lib.has_xcb_input_get_selected_extension_events_all_classes_length());
        assert!(lib.has_xcb_input_get_selected_extension_events_all_classes_end());
        assert!(lib.has_xcb_input_get_selected_extension_events_reply());
        assert!(lib.has_xcb_input_change_device_dont_propagate_list_sizeof());
        assert!(lib.has_xcb_input_change_device_dont_propagate_list_checked());
        assert!(lib.has_xcb_input_change_device_dont_propagate_list());
        assert!(lib.has_xcb_input_change_device_dont_propagate_list_classes());
        assert!(lib.has_xcb_input_change_device_dont_propagate_list_classes_length());
        assert!(lib.has_xcb_input_change_device_dont_propagate_list_classes_end());
        assert!(lib.has_xcb_input_get_device_dont_propagate_list_sizeof());
        assert!(lib.has_xcb_input_get_device_dont_propagate_list());
        assert!(lib.has_xcb_input_get_device_dont_propagate_list_unchecked());
        assert!(lib.has_xcb_input_get_device_dont_propagate_list_classes());
        assert!(lib.has_xcb_input_get_device_dont_propagate_list_classes_length());
        assert!(lib.has_xcb_input_get_device_dont_propagate_list_classes_end());
        assert!(lib.has_xcb_input_get_device_dont_propagate_list_reply());
        assert!(lib.has_xcb_input_device_time_coord_sizeof());
        assert!(lib.has_xcb_input_device_time_coord_axisvalues());
        assert!(lib.has_xcb_input_device_time_coord_axisvalues_length());
        assert!(lib.has_xcb_input_device_time_coord_axisvalues_end());
        assert!(lib.has_xcb_input_device_time_coord_next());
        assert!(lib.has_xcb_input_device_time_coord_end());
        assert!(lib.has_xcb_input_get_device_motion_events_sizeof());
        assert!(lib.has_xcb_input_get_device_motion_events());
        assert!(lib.has_xcb_input_get_device_motion_events_unchecked());
        assert!(lib.has_xcb_input_get_device_motion_events_events_length());
        assert!(lib.has_xcb_input_get_device_motion_events_events_iterator());
        assert!(lib.has_xcb_input_get_device_motion_events_reply());
        assert!(lib.has_xcb_input_change_keyboard_device());
        assert!(lib.has_xcb_input_change_keyboard_device_unchecked());
        assert!(lib.has_xcb_input_change_keyboard_device_reply());
        assert!(lib.has_xcb_input_change_pointer_device());
        assert!(lib.has_xcb_input_change_pointer_device_unchecked());
        assert!(lib.has_xcb_input_change_pointer_device_reply());
        assert!(lib.has_xcb_input_grab_device_sizeof());
        assert!(lib.has_xcb_input_grab_device());
        assert!(lib.has_xcb_input_grab_device_unchecked());
        assert!(lib.has_xcb_input_grab_device_reply());
        assert!(lib.has_xcb_input_ungrab_device_checked());
        assert!(lib.has_xcb_input_ungrab_device());
        assert!(lib.has_xcb_input_grab_device_key_sizeof());
        assert!(lib.has_xcb_input_grab_device_key_checked());
        assert!(lib.has_xcb_input_grab_device_key());
        assert!(lib.has_xcb_input_grab_device_key_classes());
        assert!(lib.has_xcb_input_grab_device_key_classes_length());
        assert!(lib.has_xcb_input_grab_device_key_classes_end());
        assert!(lib.has_xcb_input_ungrab_device_key_checked());
        assert!(lib.has_xcb_input_ungrab_device_key());
        assert!(lib.has_xcb_input_grab_device_button_sizeof());
        assert!(lib.has_xcb_input_grab_device_button_checked());
        assert!(lib.has_xcb_input_grab_device_button());
        assert!(lib.has_xcb_input_grab_device_button_classes());
        assert!(lib.has_xcb_input_grab_device_button_classes_length());
        assert!(lib.has_xcb_input_grab_device_button_classes_end());
        assert!(lib.has_xcb_input_ungrab_device_button_checked());
        assert!(lib.has_xcb_input_ungrab_device_button());
        assert!(lib.has_xcb_input_allow_device_events_checked());
        assert!(lib.has_xcb_input_allow_device_events());
        assert!(lib.has_xcb_input_get_device_focus());
        assert!(lib.has_xcb_input_get_device_focus_unchecked());
        assert!(lib.has_xcb_input_get_device_focus_reply());
        assert!(lib.has_xcb_input_set_device_focus_checked());
        assert!(lib.has_xcb_input_set_device_focus());
        assert!(lib.has_xcb_input_kbd_feedback_state_next());
        assert!(lib.has_xcb_input_kbd_feedback_state_end());
        assert!(lib.has_xcb_input_ptr_feedback_state_next());
        assert!(lib.has_xcb_input_ptr_feedback_state_end());
        assert!(lib.has_xcb_input_integer_feedback_state_next());
        assert!(lib.has_xcb_input_integer_feedback_state_end());
        assert!(lib.has_xcb_input_string_feedback_state_sizeof());
        assert!(lib.has_xcb_input_string_feedback_state_keysyms());
        assert!(lib.has_xcb_input_string_feedback_state_keysyms_length());
        assert!(lib.has_xcb_input_string_feedback_state_keysyms_end());
        assert!(lib.has_xcb_input_string_feedback_state_next());
        assert!(lib.has_xcb_input_string_feedback_state_end());
        assert!(lib.has_xcb_input_bell_feedback_state_next());
        assert!(lib.has_xcb_input_bell_feedback_state_end());
        assert!(lib.has_xcb_input_led_feedback_state_next());
        assert!(lib.has_xcb_input_led_feedback_state_end());
        assert!(lib.has_xcb_input_feedback_state_data_string_keysyms());
        assert!(lib.has_xcb_input_feedback_state_data_string_keysyms_length());
        assert!(lib.has_xcb_input_feedback_state_data_string_keysyms_end());
        assert!(lib.has_xcb_input_feedback_state_data_serialize());
        assert!(lib.has_xcb_input_feedback_state_data_unpack());
        assert!(lib.has_xcb_input_feedback_state_data_sizeof());
        assert!(lib.has_xcb_input_feedback_state_sizeof());
        assert!(lib.has_xcb_input_feedback_state_data());
        assert!(lib.has_xcb_input_feedback_state_next());
        assert!(lib.has_xcb_input_feedback_state_end());
        assert!(lib.has_xcb_input_get_feedback_control_sizeof());
        assert!(lib.has_xcb_input_get_feedback_control());
        assert!(lib.has_xcb_input_get_feedback_control_unchecked());
        assert!(lib.has_xcb_input_get_feedback_control_feedbacks_length());
        assert!(lib.has_xcb_input_get_feedback_control_feedbacks_iterator());
        assert!(lib.has_xcb_input_get_feedback_control_reply());
        assert!(lib.has_xcb_input_kbd_feedback_ctl_next());
        assert!(lib.has_xcb_input_kbd_feedback_ctl_end());
        assert!(lib.has_xcb_input_ptr_feedback_ctl_next());
        assert!(lib.has_xcb_input_ptr_feedback_ctl_end());
        assert!(lib.has_xcb_input_integer_feedback_ctl_next());
        assert!(lib.has_xcb_input_integer_feedback_ctl_end());
        assert!(lib.has_xcb_input_string_feedback_ctl_sizeof());
        assert!(lib.has_xcb_input_string_feedback_ctl_keysyms());
        assert!(lib.has_xcb_input_string_feedback_ctl_keysyms_length());
        assert!(lib.has_xcb_input_string_feedback_ctl_keysyms_end());
        assert!(lib.has_xcb_input_string_feedback_ctl_next());
        assert!(lib.has_xcb_input_string_feedback_ctl_end());
        assert!(lib.has_xcb_input_bell_feedback_ctl_next());
        assert!(lib.has_xcb_input_bell_feedback_ctl_end());
        assert!(lib.has_xcb_input_led_feedback_ctl_next());
        assert!(lib.has_xcb_input_led_feedback_ctl_end());
        assert!(lib.has_xcb_input_feedback_ctl_data_string_keysyms());
        assert!(lib.has_xcb_input_feedback_ctl_data_string_keysyms_length());
        assert!(lib.has_xcb_input_feedback_ctl_data_string_keysyms_end());
        assert!(lib.has_xcb_input_feedback_ctl_data_serialize());
        assert!(lib.has_xcb_input_feedback_ctl_data_unpack());
        assert!(lib.has_xcb_input_feedback_ctl_data_sizeof());
        assert!(lib.has_xcb_input_feedback_ctl_sizeof());
        assert!(lib.has_xcb_input_feedback_ctl_data());
        assert!(lib.has_xcb_input_feedback_ctl_next());
        assert!(lib.has_xcb_input_feedback_ctl_end());
        assert!(lib.has_xcb_input_change_feedback_control_sizeof());
        assert!(lib.has_xcb_input_change_feedback_control_checked());
        assert!(lib.has_xcb_input_change_feedback_control());
        assert!(lib.has_xcb_input_change_feedback_control_feedback());
        assert!(lib.has_xcb_input_get_device_key_mapping_sizeof());
        assert!(lib.has_xcb_input_get_device_key_mapping());
        assert!(lib.has_xcb_input_get_device_key_mapping_unchecked());
        assert!(lib.has_xcb_input_get_device_key_mapping_keysyms());
        assert!(lib.has_xcb_input_get_device_key_mapping_keysyms_length());
        assert!(lib.has_xcb_input_get_device_key_mapping_keysyms_end());
        assert!(lib.has_xcb_input_get_device_key_mapping_reply());
        assert!(lib.has_xcb_input_change_device_key_mapping_sizeof());
        assert!(lib.has_xcb_input_change_device_key_mapping_checked());
        assert!(lib.has_xcb_input_change_device_key_mapping());
        assert!(lib.has_xcb_input_change_device_key_mapping_keysyms());
        assert!(lib.has_xcb_input_change_device_key_mapping_keysyms_length());
        assert!(lib.has_xcb_input_change_device_key_mapping_keysyms_end());
        assert!(lib.has_xcb_input_get_device_modifier_mapping_sizeof());
        assert!(lib.has_xcb_input_get_device_modifier_mapping());
        assert!(lib.has_xcb_input_get_device_modifier_mapping_unchecked());
        assert!(lib.has_xcb_input_get_device_modifier_mapping_keymaps());
        assert!(lib.has_xcb_input_get_device_modifier_mapping_keymaps_length());
        assert!(lib.has_xcb_input_get_device_modifier_mapping_keymaps_end());
        assert!(lib.has_xcb_input_get_device_modifier_mapping_reply());
        assert!(lib.has_xcb_input_set_device_modifier_mapping_sizeof());
        assert!(lib.has_xcb_input_set_device_modifier_mapping());
        assert!(lib.has_xcb_input_set_device_modifier_mapping_unchecked());
        assert!(lib.has_xcb_input_set_device_modifier_mapping_reply());
        assert!(lib.has_xcb_input_get_device_button_mapping_sizeof());
        assert!(lib.has_xcb_input_get_device_button_mapping());
        assert!(lib.has_xcb_input_get_device_button_mapping_unchecked());
        assert!(lib.has_xcb_input_get_device_button_mapping_map());
        assert!(lib.has_xcb_input_get_device_button_mapping_map_length());
        assert!(lib.has_xcb_input_get_device_button_mapping_map_end());
        assert!(lib.has_xcb_input_get_device_button_mapping_reply());
        assert!(lib.has_xcb_input_set_device_button_mapping_sizeof());
        assert!(lib.has_xcb_input_set_device_button_mapping());
        assert!(lib.has_xcb_input_set_device_button_mapping_unchecked());
        assert!(lib.has_xcb_input_set_device_button_mapping_reply());
        assert!(lib.has_xcb_input_key_state_next());
        assert!(lib.has_xcb_input_key_state_end());
        assert!(lib.has_xcb_input_button_state_next());
        assert!(lib.has_xcb_input_button_state_end());
        assert!(lib.has_xcb_input_valuator_state_sizeof());
        assert!(lib.has_xcb_input_valuator_state_valuators());
        assert!(lib.has_xcb_input_valuator_state_valuators_length());
        assert!(lib.has_xcb_input_valuator_state_valuators_end());
        assert!(lib.has_xcb_input_valuator_state_next());
        assert!(lib.has_xcb_input_valuator_state_end());
        assert!(lib.has_xcb_input_input_state_data_valuator_valuators());
        assert!(lib.has_xcb_input_input_state_data_valuator_valuators_length());
        assert!(lib.has_xcb_input_input_state_data_valuator_valuators_end());
        assert!(lib.has_xcb_input_input_state_data_serialize());
        assert!(lib.has_xcb_input_input_state_data_unpack());
        assert!(lib.has_xcb_input_input_state_data_sizeof());
        assert!(lib.has_xcb_input_input_state_sizeof());
        assert!(lib.has_xcb_input_input_state_data());
        assert!(lib.has_xcb_input_input_state_next());
        assert!(lib.has_xcb_input_input_state_end());
        assert!(lib.has_xcb_input_query_device_state_sizeof());
        assert!(lib.has_xcb_input_query_device_state());
        assert!(lib.has_xcb_input_query_device_state_unchecked());
        assert!(lib.has_xcb_input_query_device_state_classes_length());
        assert!(lib.has_xcb_input_query_device_state_classes_iterator());
        assert!(lib.has_xcb_input_query_device_state_reply());
        assert!(lib.has_xcb_input_device_bell_checked());
        assert!(lib.has_xcb_input_device_bell());
        assert!(lib.has_xcb_input_set_device_valuators_sizeof());
        assert!(lib.has_xcb_input_set_device_valuators());
        assert!(lib.has_xcb_input_set_device_valuators_unchecked());
        assert!(lib.has_xcb_input_set_device_valuators_reply());
        assert!(lib.has_xcb_input_device_resolution_state_sizeof());
        assert!(lib.has_xcb_input_device_resolution_state_resolution_values());
        assert!(lib.has_xcb_input_device_resolution_state_resolution_values_length());
        assert!(lib.has_xcb_input_device_resolution_state_resolution_values_end());
        assert!(lib.has_xcb_input_device_resolution_state_resolution_min());
        assert!(lib.has_xcb_input_device_resolution_state_resolution_min_length());
        assert!(lib.has_xcb_input_device_resolution_state_resolution_min_end());
        assert!(lib.has_xcb_input_device_resolution_state_resolution_max());
        assert!(lib.has_xcb_input_device_resolution_state_resolution_max_length());
        assert!(lib.has_xcb_input_device_resolution_state_resolution_max_end());
        assert!(lib.has_xcb_input_device_resolution_state_next());
        assert!(lib.has_xcb_input_device_resolution_state_end());
        assert!(lib.has_xcb_input_device_abs_calib_state_next());
        assert!(lib.has_xcb_input_device_abs_calib_state_end());
        assert!(lib.has_xcb_input_device_abs_area_state_next());
        assert!(lib.has_xcb_input_device_abs_area_state_end());
        assert!(lib.has_xcb_input_device_core_state_next());
        assert!(lib.has_xcb_input_device_core_state_end());
        assert!(lib.has_xcb_input_device_enable_state_next());
        assert!(lib.has_xcb_input_device_enable_state_end());
        assert!(lib.has_xcb_input_device_state_data_resolution_resolution_values());
        assert!(lib.has_xcb_input_device_state_data_resolution_resolution_values_length());
        assert!(lib.has_xcb_input_device_state_data_resolution_resolution_values_end());
        assert!(lib.has_xcb_input_device_state_data_resolution_resolution_min());
        assert!(lib.has_xcb_input_device_state_data_resolution_resolution_min_length());
        assert!(lib.has_xcb_input_device_state_data_resolution_resolution_min_end());
        assert!(lib.has_xcb_input_device_state_data_resolution_resolution_max());
        assert!(lib.has_xcb_input_device_state_data_resolution_resolution_max_length());
        assert!(lib.has_xcb_input_device_state_data_resolution_resolution_max_end());
        assert!(lib.has_xcb_input_device_state_data_serialize());
        assert!(lib.has_xcb_input_device_state_data_unpack());
        assert!(lib.has_xcb_input_device_state_data_sizeof());
        assert!(lib.has_xcb_input_device_state_sizeof());
        assert!(lib.has_xcb_input_device_state_data());
        assert!(lib.has_xcb_input_device_state_next());
        assert!(lib.has_xcb_input_device_state_end());
        assert!(lib.has_xcb_input_get_device_control_sizeof());
        assert!(lib.has_xcb_input_get_device_control());
        assert!(lib.has_xcb_input_get_device_control_unchecked());
        assert!(lib.has_xcb_input_get_device_control_control());
        assert!(lib.has_xcb_input_get_device_control_reply());
        assert!(lib.has_xcb_input_device_resolution_ctl_sizeof());
        assert!(lib.has_xcb_input_device_resolution_ctl_resolution_values());
        assert!(lib.has_xcb_input_device_resolution_ctl_resolution_values_length());
        assert!(lib.has_xcb_input_device_resolution_ctl_resolution_values_end());
        assert!(lib.has_xcb_input_device_resolution_ctl_next());
        assert!(lib.has_xcb_input_device_resolution_ctl_end());
        assert!(lib.has_xcb_input_device_abs_calib_ctl_next());
        assert!(lib.has_xcb_input_device_abs_calib_ctl_end());
        assert!(lib.has_xcb_input_device_abs_area_ctrl_next());
        assert!(lib.has_xcb_input_device_abs_area_ctrl_end());
        assert!(lib.has_xcb_input_device_core_ctrl_next());
        assert!(lib.has_xcb_input_device_core_ctrl_end());
        assert!(lib.has_xcb_input_device_enable_ctrl_next());
        assert!(lib.has_xcb_input_device_enable_ctrl_end());
        assert!(lib.has_xcb_input_device_ctl_data_resolution_resolution_values());
        assert!(lib.has_xcb_input_device_ctl_data_resolution_resolution_values_length());
        assert!(lib.has_xcb_input_device_ctl_data_resolution_resolution_values_end());
        assert!(lib.has_xcb_input_device_ctl_data_serialize());
        assert!(lib.has_xcb_input_device_ctl_data_unpack());
        assert!(lib.has_xcb_input_device_ctl_data_sizeof());
        assert!(lib.has_xcb_input_device_ctl_sizeof());
        assert!(lib.has_xcb_input_device_ctl_data());
        assert!(lib.has_xcb_input_device_ctl_next());
        assert!(lib.has_xcb_input_device_ctl_end());
        assert!(lib.has_xcb_input_change_device_control_sizeof());
        assert!(lib.has_xcb_input_change_device_control());
        assert!(lib.has_xcb_input_change_device_control_unchecked());
        assert!(lib.has_xcb_input_change_device_control_reply());
        assert!(lib.has_xcb_input_list_device_properties_sizeof());
        assert!(lib.has_xcb_input_list_device_properties());
        assert!(lib.has_xcb_input_list_device_properties_unchecked());
        assert!(lib.has_xcb_input_list_device_properties_atoms());
        assert!(lib.has_xcb_input_list_device_properties_atoms_length());
        assert!(lib.has_xcb_input_list_device_properties_atoms_end());
        assert!(lib.has_xcb_input_list_device_properties_reply());
        assert!(lib.has_xcb_input_change_device_property_items_data_8());
        assert!(lib.has_xcb_input_change_device_property_items_data_8_length());
        assert!(lib.has_xcb_input_change_device_property_items_data_8_end());
        assert!(lib.has_xcb_input_change_device_property_items_data_16());
        assert!(lib.has_xcb_input_change_device_property_items_data_16_length());
        assert!(lib.has_xcb_input_change_device_property_items_data_16_end());
        assert!(lib.has_xcb_input_change_device_property_items_data_32());
        assert!(lib.has_xcb_input_change_device_property_items_data_32_length());
        assert!(lib.has_xcb_input_change_device_property_items_data_32_end());
        assert!(lib.has_xcb_input_change_device_property_items_serialize());
        assert!(lib.has_xcb_input_change_device_property_items_unpack());
        assert!(lib.has_xcb_input_change_device_property_items_sizeof());
        assert!(lib.has_xcb_input_change_device_property_sizeof());
        assert!(lib.has_xcb_input_change_device_property_checked());
        assert!(lib.has_xcb_input_change_device_property());
        assert!(lib.has_xcb_input_change_device_property_aux_checked());
        assert!(lib.has_xcb_input_change_device_property_aux());
        assert!(lib.has_xcb_input_change_device_property_items());
        assert!(lib.has_xcb_input_delete_device_property_checked());
        assert!(lib.has_xcb_input_delete_device_property());
        assert!(lib.has_xcb_input_get_device_property_items_data_8());
        assert!(lib.has_xcb_input_get_device_property_items_data_8_length());
        assert!(lib.has_xcb_input_get_device_property_items_data_8_end());
        assert!(lib.has_xcb_input_get_device_property_items_data_16());
        assert!(lib.has_xcb_input_get_device_property_items_data_16_length());
        assert!(lib.has_xcb_input_get_device_property_items_data_16_end());
        assert!(lib.has_xcb_input_get_device_property_items_data_32());
        assert!(lib.has_xcb_input_get_device_property_items_data_32_length());
        assert!(lib.has_xcb_input_get_device_property_items_data_32_end());
        assert!(lib.has_xcb_input_get_device_property_items_serialize());
        assert!(lib.has_xcb_input_get_device_property_items_unpack());
        assert!(lib.has_xcb_input_get_device_property_items_sizeof());
        assert!(lib.has_xcb_input_get_device_property_sizeof());
        assert!(lib.has_xcb_input_get_device_property());
        assert!(lib.has_xcb_input_get_device_property_unchecked());
        assert!(lib.has_xcb_input_get_device_property_items());
        assert!(lib.has_xcb_input_get_device_property_reply());
        assert!(lib.has_xcb_input_group_info_next());
        assert!(lib.has_xcb_input_group_info_end());
        assert!(lib.has_xcb_input_modifier_info_next());
        assert!(lib.has_xcb_input_modifier_info_end());
        assert!(lib.has_xcb_input_xi_query_pointer_sizeof());
        assert!(lib.has_xcb_input_xi_query_pointer());
        assert!(lib.has_xcb_input_xi_query_pointer_unchecked());
        assert!(lib.has_xcb_input_xi_query_pointer_buttons());
        assert!(lib.has_xcb_input_xi_query_pointer_buttons_length());
        assert!(lib.has_xcb_input_xi_query_pointer_buttons_end());
        assert!(lib.has_xcb_input_xi_query_pointer_reply());
        assert!(lib.has_xcb_input_xi_warp_pointer_checked());
        assert!(lib.has_xcb_input_xi_warp_pointer());
        assert!(lib.has_xcb_input_xi_change_cursor_checked());
        assert!(lib.has_xcb_input_xi_change_cursor());
        assert!(lib.has_xcb_input_add_master_sizeof());
        assert!(lib.has_xcb_input_add_master_name());
        assert!(lib.has_xcb_input_add_master_name_length());
        assert!(lib.has_xcb_input_add_master_name_end());
        assert!(lib.has_xcb_input_add_master_next());
        assert!(lib.has_xcb_input_add_master_end());
        assert!(lib.has_xcb_input_remove_master_next());
        assert!(lib.has_xcb_input_remove_master_end());
        assert!(lib.has_xcb_input_attach_slave_next());
        assert!(lib.has_xcb_input_attach_slave_end());
        assert!(lib.has_xcb_input_detach_slave_next());
        assert!(lib.has_xcb_input_detach_slave_end());
        assert!(lib.has_xcb_input_hierarchy_change_data_add_master_name());
        assert!(lib.has_xcb_input_hierarchy_change_data_add_master_name_length());
        assert!(lib.has_xcb_input_hierarchy_change_data_add_master_name_end());
        assert!(lib.has_xcb_input_hierarchy_change_data_serialize());
        assert!(lib.has_xcb_input_hierarchy_change_data_unpack());
        assert!(lib.has_xcb_input_hierarchy_change_data_sizeof());
        assert!(lib.has_xcb_input_hierarchy_change_sizeof());
        assert!(lib.has_xcb_input_hierarchy_change_data());
        assert!(lib.has_xcb_input_hierarchy_change_next());
        assert!(lib.has_xcb_input_hierarchy_change_end());
        assert!(lib.has_xcb_input_xi_change_hierarchy_sizeof());
        assert!(lib.has_xcb_input_xi_change_hierarchy_checked());
        assert!(lib.has_xcb_input_xi_change_hierarchy());
        assert!(lib.has_xcb_input_xi_change_hierarchy_changes_length());
        assert!(lib.has_xcb_input_xi_change_hierarchy_changes_iterator());
        assert!(lib.has_xcb_input_xi_set_client_pointer_checked());
        assert!(lib.has_xcb_input_xi_set_client_pointer());
        assert!(lib.has_xcb_input_xi_get_client_pointer());
        assert!(lib.has_xcb_input_xi_get_client_pointer_unchecked());
        assert!(lib.has_xcb_input_xi_get_client_pointer_reply());
        assert!(lib.has_xcb_input_event_mask_sizeof());
        assert!(lib.has_xcb_input_event_mask_mask());
        assert!(lib.has_xcb_input_event_mask_mask_length());
        assert!(lib.has_xcb_input_event_mask_mask_end());
        assert!(lib.has_xcb_input_event_mask_next());
        assert!(lib.has_xcb_input_event_mask_end());
        assert!(lib.has_xcb_input_xi_select_events_sizeof());
        assert!(lib.has_xcb_input_xi_select_events_checked());
        assert!(lib.has_xcb_input_xi_select_events());
        assert!(lib.has_xcb_input_xi_select_events_masks_length());
        assert!(lib.has_xcb_input_xi_select_events_masks_iterator());
        assert!(lib.has_xcb_input_xi_query_version());
        assert!(lib.has_xcb_input_xi_query_version_unchecked());
        assert!(lib.has_xcb_input_xi_query_version_reply());
        assert!(lib.has_xcb_input_button_class_sizeof());
        assert!(lib.has_xcb_input_button_class_state());
        assert!(lib.has_xcb_input_button_class_state_length());
        assert!(lib.has_xcb_input_button_class_state_end());
        assert!(lib.has_xcb_input_button_class_labels());
        assert!(lib.has_xcb_input_button_class_labels_length());
        assert!(lib.has_xcb_input_button_class_labels_end());
        assert!(lib.has_xcb_input_button_class_next());
        assert!(lib.has_xcb_input_button_class_end());
        assert!(lib.has_xcb_input_key_class_sizeof());
        assert!(lib.has_xcb_input_key_class_keys());
        assert!(lib.has_xcb_input_key_class_keys_length());
        assert!(lib.has_xcb_input_key_class_keys_end());
        assert!(lib.has_xcb_input_key_class_next());
        assert!(lib.has_xcb_input_key_class_end());
        assert!(lib.has_xcb_input_scroll_class_next());
        assert!(lib.has_xcb_input_scroll_class_end());
        assert!(lib.has_xcb_input_touch_class_next());
        assert!(lib.has_xcb_input_touch_class_end());
        assert!(lib.has_xcb_input_valuator_class_next());
        assert!(lib.has_xcb_input_valuator_class_end());
        assert!(lib.has_xcb_input_device_class_data_key_keys());
        assert!(lib.has_xcb_input_device_class_data_key_keys_length());
        assert!(lib.has_xcb_input_device_class_data_key_keys_end());
        assert!(lib.has_xcb_input_device_class_data_button_state());
        assert!(lib.has_xcb_input_device_class_data_button_state_length());
        assert!(lib.has_xcb_input_device_class_data_button_state_end());
        assert!(lib.has_xcb_input_device_class_data_button_labels());
        assert!(lib.has_xcb_input_device_class_data_button_labels_length());
        assert!(lib.has_xcb_input_device_class_data_button_labels_end());
        assert!(lib.has_xcb_input_device_class_data_serialize());
        assert!(lib.has_xcb_input_device_class_data_unpack());
        assert!(lib.has_xcb_input_device_class_data_sizeof());
        assert!(lib.has_xcb_input_device_class_sizeof());
        assert!(lib.has_xcb_input_device_class_data());
        assert!(lib.has_xcb_input_device_class_next());
        assert!(lib.has_xcb_input_device_class_end());
        assert!(lib.has_xcb_input_xi_device_info_sizeof());
        assert!(lib.has_xcb_input_xi_device_info_name());
        assert!(lib.has_xcb_input_xi_device_info_name_length());
        assert!(lib.has_xcb_input_xi_device_info_name_end());
        assert!(lib.has_xcb_input_xi_device_info_classes_length());
        assert!(lib.has_xcb_input_xi_device_info_classes_iterator());
        assert!(lib.has_xcb_input_xi_device_info_next());
        assert!(lib.has_xcb_input_xi_device_info_end());
        assert!(lib.has_xcb_input_xi_query_device_sizeof());
        assert!(lib.has_xcb_input_xi_query_device());
        assert!(lib.has_xcb_input_xi_query_device_unchecked());
        assert!(lib.has_xcb_input_xi_query_device_infos_length());
        assert!(lib.has_xcb_input_xi_query_device_infos_iterator());
        assert!(lib.has_xcb_input_xi_query_device_reply());
        assert!(lib.has_xcb_input_xi_set_focus_checked());
        assert!(lib.has_xcb_input_xi_set_focus());
        assert!(lib.has_xcb_input_xi_get_focus());
        assert!(lib.has_xcb_input_xi_get_focus_unchecked());
        assert!(lib.has_xcb_input_xi_get_focus_reply());
        assert!(lib.has_xcb_input_xi_grab_device_sizeof());
        assert!(lib.has_xcb_input_xi_grab_device());
        assert!(lib.has_xcb_input_xi_grab_device_unchecked());
        assert!(lib.has_xcb_input_xi_grab_device_reply());
        assert!(lib.has_xcb_input_xi_ungrab_device_checked());
        assert!(lib.has_xcb_input_xi_ungrab_device());
        assert!(lib.has_xcb_input_xi_allow_events_checked());
        assert!(lib.has_xcb_input_xi_allow_events());
        assert!(lib.has_xcb_input_grab_modifier_info_next());
        assert!(lib.has_xcb_input_grab_modifier_info_end());
        assert!(lib.has_xcb_input_xi_passive_grab_device_sizeof());
        assert!(lib.has_xcb_input_xi_passive_grab_device());
        assert!(lib.has_xcb_input_xi_passive_grab_device_unchecked());
        assert!(lib.has_xcb_input_xi_passive_grab_device_modifiers());
        assert!(lib.has_xcb_input_xi_passive_grab_device_modifiers_length());
        assert!(lib.has_xcb_input_xi_passive_grab_device_modifiers_iterator());
        assert!(lib.has_xcb_input_xi_passive_grab_device_reply());
        assert!(lib.has_xcb_input_xi_passive_ungrab_device_sizeof());
        assert!(lib.has_xcb_input_xi_passive_ungrab_device_checked());
        assert!(lib.has_xcb_input_xi_passive_ungrab_device());
        assert!(lib.has_xcb_input_xi_passive_ungrab_device_modifiers());
        assert!(lib.has_xcb_input_xi_passive_ungrab_device_modifiers_length());
        assert!(lib.has_xcb_input_xi_passive_ungrab_device_modifiers_end());
        assert!(lib.has_xcb_input_xi_list_properties_sizeof());
        assert!(lib.has_xcb_input_xi_list_properties());
        assert!(lib.has_xcb_input_xi_list_properties_unchecked());
        assert!(lib.has_xcb_input_xi_list_properties_properties());
        assert!(lib.has_xcb_input_xi_list_properties_properties_length());
        assert!(lib.has_xcb_input_xi_list_properties_properties_end());
        assert!(lib.has_xcb_input_xi_list_properties_reply());
        assert!(lib.has_xcb_input_xi_change_property_items_data_8());
        assert!(lib.has_xcb_input_xi_change_property_items_data_8_length());
        assert!(lib.has_xcb_input_xi_change_property_items_data_8_end());
        assert!(lib.has_xcb_input_xi_change_property_items_data_16());
        assert!(lib.has_xcb_input_xi_change_property_items_data_16_length());
        assert!(lib.has_xcb_input_xi_change_property_items_data_16_end());
        assert!(lib.has_xcb_input_xi_change_property_items_data_32());
        assert!(lib.has_xcb_input_xi_change_property_items_data_32_length());
        assert!(lib.has_xcb_input_xi_change_property_items_data_32_end());
        assert!(lib.has_xcb_input_xi_change_property_items_serialize());
        assert!(lib.has_xcb_input_xi_change_property_items_unpack());
        assert!(lib.has_xcb_input_xi_change_property_items_sizeof());
        assert!(lib.has_xcb_input_xi_change_property_sizeof());
        assert!(lib.has_xcb_input_xi_change_property_checked());
        assert!(lib.has_xcb_input_xi_change_property());
        assert!(lib.has_xcb_input_xi_change_property_aux_checked());
        assert!(lib.has_xcb_input_xi_change_property_aux());
        assert!(lib.has_xcb_input_xi_change_property_items());
        assert!(lib.has_xcb_input_xi_delete_property_checked());
        assert!(lib.has_xcb_input_xi_delete_property());
        assert!(lib.has_xcb_input_xi_get_property_items_data_8());
        assert!(lib.has_xcb_input_xi_get_property_items_data_8_length());
        assert!(lib.has_xcb_input_xi_get_property_items_data_8_end());
        assert!(lib.has_xcb_input_xi_get_property_items_data_16());
        assert!(lib.has_xcb_input_xi_get_property_items_data_16_length());
        assert!(lib.has_xcb_input_xi_get_property_items_data_16_end());
        assert!(lib.has_xcb_input_xi_get_property_items_data_32());
        assert!(lib.has_xcb_input_xi_get_property_items_data_32_length());
        assert!(lib.has_xcb_input_xi_get_property_items_data_32_end());
        assert!(lib.has_xcb_input_xi_get_property_items_serialize());
        assert!(lib.has_xcb_input_xi_get_property_items_unpack());
        assert!(lib.has_xcb_input_xi_get_property_items_sizeof());
        assert!(lib.has_xcb_input_xi_get_property_sizeof());
        assert!(lib.has_xcb_input_xi_get_property());
        assert!(lib.has_xcb_input_xi_get_property_unchecked());
        assert!(lib.has_xcb_input_xi_get_property_items());
        assert!(lib.has_xcb_input_xi_get_property_reply());
        assert!(lib.has_xcb_input_xi_get_selected_events_sizeof());
        assert!(lib.has_xcb_input_xi_get_selected_events());
        assert!(lib.has_xcb_input_xi_get_selected_events_unchecked());
        assert!(lib.has_xcb_input_xi_get_selected_events_masks_length());
        assert!(lib.has_xcb_input_xi_get_selected_events_masks_iterator());
        assert!(lib.has_xcb_input_xi_get_selected_events_reply());
        assert!(lib.has_xcb_input_barrier_release_pointer_info_next());
        assert!(lib.has_xcb_input_barrier_release_pointer_info_end());
        assert!(lib.has_xcb_input_xi_barrier_release_pointer_sizeof());
        assert!(lib.has_xcb_input_xi_barrier_release_pointer_checked());
        assert!(lib.has_xcb_input_xi_barrier_release_pointer());
        assert!(lib.has_xcb_input_xi_barrier_release_pointer_barriers());
        assert!(lib.has_xcb_input_xi_barrier_release_pointer_barriers_length());
        assert!(lib.has_xcb_input_xi_barrier_release_pointer_barriers_iterator());
        assert!(lib.has_xcb_input_device_changed_sizeof());
        assert!(lib.has_xcb_input_device_changed_classes_length());
        assert!(lib.has_xcb_input_device_changed_classes_iterator());
        assert!(lib.has_xcb_input_key_press_sizeof());
        assert!(lib.has_xcb_input_key_press_button_mask());
        assert!(lib.has_xcb_input_key_press_button_mask_length());
        assert!(lib.has_xcb_input_key_press_button_mask_end());
        assert!(lib.has_xcb_input_key_press_valuator_mask());
        assert!(lib.has_xcb_input_key_press_valuator_mask_length());
        assert!(lib.has_xcb_input_key_press_valuator_mask_end());
        assert!(lib.has_xcb_input_key_press_axisvalues());
        assert!(lib.has_xcb_input_key_press_axisvalues_length());
        assert!(lib.has_xcb_input_key_press_axisvalues_iterator());
        assert!(lib.has_xcb_input_key_release_sizeof());
        assert!(lib.has_xcb_input_button_press_sizeof());
        assert!(lib.has_xcb_input_button_press_button_mask());
        assert!(lib.has_xcb_input_button_press_button_mask_length());
        assert!(lib.has_xcb_input_button_press_button_mask_end());
        assert!(lib.has_xcb_input_button_press_valuator_mask());
        assert!(lib.has_xcb_input_button_press_valuator_mask_length());
        assert!(lib.has_xcb_input_button_press_valuator_mask_end());
        assert!(lib.has_xcb_input_button_press_axisvalues());
        assert!(lib.has_xcb_input_button_press_axisvalues_length());
        assert!(lib.has_xcb_input_button_press_axisvalues_iterator());
        assert!(lib.has_xcb_input_button_release_sizeof());
        assert!(lib.has_xcb_input_motion_sizeof());
        assert!(lib.has_xcb_input_enter_sizeof());
        assert!(lib.has_xcb_input_enter_buttons());
        assert!(lib.has_xcb_input_enter_buttons_length());
        assert!(lib.has_xcb_input_enter_buttons_end());
        assert!(lib.has_xcb_input_leave_sizeof());
        assert!(lib.has_xcb_input_focus_in_sizeof());
        assert!(lib.has_xcb_input_focus_out_sizeof());
        assert!(lib.has_xcb_input_hierarchy_info_next());
        assert!(lib.has_xcb_input_hierarchy_info_end());
        assert!(lib.has_xcb_input_hierarchy_sizeof());
        assert!(lib.has_xcb_input_hierarchy_infos());
        assert!(lib.has_xcb_input_hierarchy_infos_length());
        assert!(lib.has_xcb_input_hierarchy_infos_iterator());
        assert!(lib.has_xcb_input_raw_key_press_sizeof());
        assert!(lib.has_xcb_input_raw_key_press_valuator_mask());
        assert!(lib.has_xcb_input_raw_key_press_valuator_mask_length());
        assert!(lib.has_xcb_input_raw_key_press_valuator_mask_end());
        assert!(lib.has_xcb_input_raw_key_press_axisvalues());
        assert!(lib.has_xcb_input_raw_key_press_axisvalues_length());
        assert!(lib.has_xcb_input_raw_key_press_axisvalues_iterator());
        assert!(lib.has_xcb_input_raw_key_press_axisvalues_raw());
        assert!(lib.has_xcb_input_raw_key_press_axisvalues_raw_length());
        assert!(lib.has_xcb_input_raw_key_press_axisvalues_raw_iterator());
        assert!(lib.has_xcb_input_raw_key_release_sizeof());
        assert!(lib.has_xcb_input_raw_button_press_sizeof());
        assert!(lib.has_xcb_input_raw_button_press_valuator_mask());
        assert!(lib.has_xcb_input_raw_button_press_valuator_mask_length());
        assert!(lib.has_xcb_input_raw_button_press_valuator_mask_end());
        assert!(lib.has_xcb_input_raw_button_press_axisvalues());
        assert!(lib.has_xcb_input_raw_button_press_axisvalues_length());
        assert!(lib.has_xcb_input_raw_button_press_axisvalues_iterator());
        assert!(lib.has_xcb_input_raw_button_press_axisvalues_raw());
        assert!(lib.has_xcb_input_raw_button_press_axisvalues_raw_length());
        assert!(lib.has_xcb_input_raw_button_press_axisvalues_raw_iterator());
        assert!(lib.has_xcb_input_raw_button_release_sizeof());
        assert!(lib.has_xcb_input_raw_motion_sizeof());
        assert!(lib.has_xcb_input_touch_begin_sizeof());
        assert!(lib.has_xcb_input_touch_begin_button_mask());
        assert!(lib.has_xcb_input_touch_begin_button_mask_length());
        assert!(lib.has_xcb_input_touch_begin_button_mask_end());
        assert!(lib.has_xcb_input_touch_begin_valuator_mask());
        assert!(lib.has_xcb_input_touch_begin_valuator_mask_length());
        assert!(lib.has_xcb_input_touch_begin_valuator_mask_end());
        assert!(lib.has_xcb_input_touch_begin_axisvalues());
        assert!(lib.has_xcb_input_touch_begin_axisvalues_length());
        assert!(lib.has_xcb_input_touch_begin_axisvalues_iterator());
        assert!(lib.has_xcb_input_touch_update_sizeof());
        assert!(lib.has_xcb_input_touch_end_sizeof());
        assert!(lib.has_xcb_input_raw_touch_begin_sizeof());
        assert!(lib.has_xcb_input_raw_touch_begin_valuator_mask());
        assert!(lib.has_xcb_input_raw_touch_begin_valuator_mask_length());
        assert!(lib.has_xcb_input_raw_touch_begin_valuator_mask_end());
        assert!(lib.has_xcb_input_raw_touch_begin_axisvalues());
        assert!(lib.has_xcb_input_raw_touch_begin_axisvalues_length());
        assert!(lib.has_xcb_input_raw_touch_begin_axisvalues_iterator());
        assert!(lib.has_xcb_input_raw_touch_begin_axisvalues_raw());
        assert!(lib.has_xcb_input_raw_touch_begin_axisvalues_raw_length());
        assert!(lib.has_xcb_input_raw_touch_begin_axisvalues_raw_iterator());
        assert!(lib.has_xcb_input_raw_touch_update_sizeof());
        assert!(lib.has_xcb_input_raw_touch_end_sizeof());
        assert!(lib.has_xcb_input_event_for_send_next());
        assert!(lib.has_xcb_input_event_for_send_end());
        assert!(lib.has_xcb_input_send_extension_event_sizeof());
        assert!(lib.has_xcb_input_send_extension_event_checked());
        assert!(lib.has_xcb_input_send_extension_event());
        assert!(lib.has_xcb_input_send_extension_event_events());
        assert!(lib.has_xcb_input_send_extension_event_events_length());
        assert!(lib.has_xcb_input_send_extension_event_events_iterator());
        assert!(lib.has_xcb_input_send_extension_event_classes());
        assert!(lib.has_xcb_input_send_extension_event_classes_length());
        assert!(lib.has_xcb_input_send_extension_event_classes_end());
    }
}
