// This file was generated using generate.py. Do not edit.

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_char2b_t {
    pub byte1: u8,
    pub byte2: u8,
}

impl Default for xcb_char2b_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_char2b_iterator_t {
    pub data: *mut xcb_char2b_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_char2b_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_window_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_window_iterator_t {
    pub data: *mut xcb_window_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_window_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_pixmap_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_pixmap_iterator_t {
    pub data: *mut xcb_pixmap_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_pixmap_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_cursor_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_cursor_iterator_t {
    pub data: *mut xcb_cursor_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_cursor_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_font_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_font_iterator_t {
    pub data: *mut xcb_font_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_font_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_gcontext_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_gcontext_iterator_t {
    pub data: *mut xcb_gcontext_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_gcontext_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_colormap_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_colormap_iterator_t {
    pub data: *mut xcb_colormap_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_colormap_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_atom_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_atom_iterator_t {
    pub data: *mut xcb_atom_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_atom_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_drawable_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_drawable_iterator_t {
    pub data: *mut xcb_drawable_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_drawable_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_fontable_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_fontable_iterator_t {
    pub data: *mut xcb_fontable_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_fontable_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_bool32_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_bool32_iterator_t {
    pub data: *mut xcb_bool32_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_bool32_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_visualid_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_visualid_iterator_t {
    pub data: *mut xcb_visualid_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_visualid_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_timestamp_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_timestamp_iterator_t {
    pub data: *mut xcb_timestamp_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_timestamp_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_keysym_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_keysym_iterator_t {
    pub data: *mut xcb_keysym_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_keysym_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_keycode_t = u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_keycode_iterator_t {
    pub data: *mut xcb_keycode_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_keycode_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_keycode32_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_keycode32_iterator_t {
    pub data: *mut xcb_keycode32_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_keycode32_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_button_t = u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_button_iterator_t {
    pub data: *mut xcb_button_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_button_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_point_t {
    pub x: i16,
    pub y: i16,
}

impl Default for xcb_point_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_point_iterator_t {
    pub data: *mut xcb_point_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_point_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_rectangle_t {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

impl Default for xcb_rectangle_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_rectangle_iterator_t {
    pub data: *mut xcb_rectangle_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_rectangle_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_arc_t {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub angle1: i16,
    pub angle2: i16,
}

impl Default for xcb_arc_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_arc_iterator_t {
    pub data: *mut xcb_arc_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_arc_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_format_t {
    pub depth: u8,
    pub bits_per_pixel: u8,
    pub scanline_pad: u8,
    pub pad0: [u8; 5],
}

impl Default for xcb_format_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_format_iterator_t {
    pub data: *mut xcb_format_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_format_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_visual_class_t = u32;
pub const XCB_VISUAL_CLASS_STATIC_GRAY: xcb_visual_class_t = 0;
pub const XCB_VISUAL_CLASS_GRAY_SCALE: xcb_visual_class_t = 1;
pub const XCB_VISUAL_CLASS_STATIC_COLOR: xcb_visual_class_t = 2;
pub const XCB_VISUAL_CLASS_PSEUDO_COLOR: xcb_visual_class_t = 3;
pub const XCB_VISUAL_CLASS_TRUE_COLOR: xcb_visual_class_t = 4;
pub const XCB_VISUAL_CLASS_DIRECT_COLOR: xcb_visual_class_t = 5;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_visualtype_t {
    pub visual_id: xcb_visualid_t,
    pub class: u8,
    pub bits_per_rgb_value: u8,
    pub colormap_entries: u16,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub pad0: [u8; 4],
}

impl Default for xcb_visualtype_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_visualtype_iterator_t {
    pub data: *mut xcb_visualtype_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_visualtype_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_depth_t {
    pub depth: u8,
    pub pad0: u8,
    pub visuals_len: u16,
    pub pad1: [u8; 4],
}

impl Default for xcb_depth_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_depth_iterator_t {
    pub data: *mut xcb_depth_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_depth_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_event_mask_t = u32;
pub const XCB_EVENT_MASK_NO_EVENT: xcb_event_mask_t = 0;
pub const XCB_EVENT_MASK_KEY_PRESS: xcb_event_mask_t = 1;
pub const XCB_EVENT_MASK_KEY_RELEASE: xcb_event_mask_t = 2;
pub const XCB_EVENT_MASK_BUTTON_PRESS: xcb_event_mask_t = 4;
pub const XCB_EVENT_MASK_BUTTON_RELEASE: xcb_event_mask_t = 8;
pub const XCB_EVENT_MASK_ENTER_WINDOW: xcb_event_mask_t = 16;
pub const XCB_EVENT_MASK_LEAVE_WINDOW: xcb_event_mask_t = 32;
pub const XCB_EVENT_MASK_POINTER_MOTION: xcb_event_mask_t = 64;
pub const XCB_EVENT_MASK_POINTER_MOTION_HINT: xcb_event_mask_t = 128;
pub const XCB_EVENT_MASK_BUTTON_1_MOTION: xcb_event_mask_t = 256;
pub const XCB_EVENT_MASK_BUTTON_2_MOTION: xcb_event_mask_t = 512;
pub const XCB_EVENT_MASK_BUTTON_3_MOTION: xcb_event_mask_t = 1024;
pub const XCB_EVENT_MASK_BUTTON_4_MOTION: xcb_event_mask_t = 2048;
pub const XCB_EVENT_MASK_BUTTON_5_MOTION: xcb_event_mask_t = 4096;
pub const XCB_EVENT_MASK_BUTTON_MOTION: xcb_event_mask_t = 8192;
pub const XCB_EVENT_MASK_KEYMAP_STATE: xcb_event_mask_t = 16384;
pub const XCB_EVENT_MASK_EXPOSURE: xcb_event_mask_t = 32768;
pub const XCB_EVENT_MASK_VISIBILITY_CHANGE: xcb_event_mask_t = 65536;
pub const XCB_EVENT_MASK_STRUCTURE_NOTIFY: xcb_event_mask_t = 131072;
pub const XCB_EVENT_MASK_RESIZE_REDIRECT: xcb_event_mask_t = 262144;
pub const XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY: xcb_event_mask_t = 524288;
pub const XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT: xcb_event_mask_t = 1048576;
pub const XCB_EVENT_MASK_FOCUS_CHANGE: xcb_event_mask_t = 2097152;
pub const XCB_EVENT_MASK_PROPERTY_CHANGE: xcb_event_mask_t = 4194304;
pub const XCB_EVENT_MASK_COLOR_MAP_CHANGE: xcb_event_mask_t = 8388608;
pub const XCB_EVENT_MASK_OWNER_GRAB_BUTTON: xcb_event_mask_t = 16777216;

pub type xcb_backing_store_t = u32;
pub const XCB_BACKING_STORE_NOT_USEFUL: xcb_backing_store_t = 0;
pub const XCB_BACKING_STORE_WHEN_MAPPED: xcb_backing_store_t = 1;
pub const XCB_BACKING_STORE_ALWAYS: xcb_backing_store_t = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screen_t {
    pub root: xcb_window_t,
    pub default_colormap: xcb_colormap_t,
    pub white_pixel: u32,
    pub black_pixel: u32,
    pub current_input_masks: u32,
    pub width_in_pixels: u16,
    pub height_in_pixels: u16,
    pub width_in_millimeters: u16,
    pub height_in_millimeters: u16,
    pub min_installed_maps: u16,
    pub max_installed_maps: u16,
    pub root_visual: xcb_visualid_t,
    pub backing_stores: u8,
    pub save_unders: u8,
    pub root_depth: u8,
    pub allowed_depths_len: u8,
}

impl Default for xcb_screen_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screen_iterator_t {
    pub data: *mut xcb_screen_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_screen_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_setup_request_t {
    pub byte_order: u8,
    pub pad0: u8,
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub authorization_protocol_name_len: u16,
    pub authorization_protocol_data_len: u16,
    pub pad1: [u8; 2],
}

impl Default for xcb_setup_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_setup_request_iterator_t {
    pub data: *mut xcb_setup_request_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_setup_request_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_setup_failed_t {
    pub status: u8,
    pub reason_len: u8,
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub length: u16,
}

impl Default for xcb_setup_failed_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_setup_failed_iterator_t {
    pub data: *mut xcb_setup_failed_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_setup_failed_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_setup_authenticate_t {
    pub status: u8,
    pub pad0: [u8; 5],
    pub length: u16,
}

impl Default for xcb_setup_authenticate_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_setup_authenticate_iterator_t {
    pub data: *mut xcb_setup_authenticate_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_setup_authenticate_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_image_order_t = u32;
pub const XCB_IMAGE_ORDER_LSB_FIRST: xcb_image_order_t = 0;
pub const XCB_IMAGE_ORDER_MSB_FIRST: xcb_image_order_t = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_setup_t {
    pub status: u8,
    pub pad0: u8,
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub length: u16,
    pub release_number: u32,
    pub resource_id_base: u32,
    pub resource_id_mask: u32,
    pub motion_buffer_size: u32,
    pub vendor_len: u16,
    pub maximum_request_length: u16,
    pub roots_len: u8,
    pub pixmap_formats_len: u8,
    pub image_byte_order: u8,
    pub bitmap_format_bit_order: u8,
    pub bitmap_format_scanline_unit: u8,
    pub bitmap_format_scanline_pad: u8,
    pub min_keycode: xcb_keycode_t,
    pub max_keycode: xcb_keycode_t,
    pub pad1: [u8; 4],
}

impl Default for xcb_setup_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_setup_iterator_t {
    pub data: *mut xcb_setup_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_setup_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_mod_mask_t = u32;
pub const XCB_MOD_MASK_SHIFT: xcb_mod_mask_t = 1;
pub const XCB_MOD_MASK_LOCK: xcb_mod_mask_t = 2;
pub const XCB_MOD_MASK_CONTROL: xcb_mod_mask_t = 4;
pub const XCB_MOD_MASK_1: xcb_mod_mask_t = 8;
pub const XCB_MOD_MASK_2: xcb_mod_mask_t = 16;
pub const XCB_MOD_MASK_3: xcb_mod_mask_t = 32;
pub const XCB_MOD_MASK_4: xcb_mod_mask_t = 64;
pub const XCB_MOD_MASK_5: xcb_mod_mask_t = 128;
pub const XCB_MOD_MASK_ANY: xcb_mod_mask_t = 32768;

pub type xcb_key_but_mask_t = u32;
pub const XCB_KEY_BUT_MASK_SHIFT: xcb_key_but_mask_t = 1;
pub const XCB_KEY_BUT_MASK_LOCK: xcb_key_but_mask_t = 2;
pub const XCB_KEY_BUT_MASK_CONTROL: xcb_key_but_mask_t = 4;
pub const XCB_KEY_BUT_MASK_MOD_1: xcb_key_but_mask_t = 8;
pub const XCB_KEY_BUT_MASK_MOD_2: xcb_key_but_mask_t = 16;
pub const XCB_KEY_BUT_MASK_MOD_3: xcb_key_but_mask_t = 32;
pub const XCB_KEY_BUT_MASK_MOD_4: xcb_key_but_mask_t = 64;
pub const XCB_KEY_BUT_MASK_MOD_5: xcb_key_but_mask_t = 128;
pub const XCB_KEY_BUT_MASK_BUTTON_1: xcb_key_but_mask_t = 256;
pub const XCB_KEY_BUT_MASK_BUTTON_2: xcb_key_but_mask_t = 512;
pub const XCB_KEY_BUT_MASK_BUTTON_3: xcb_key_but_mask_t = 1024;
pub const XCB_KEY_BUT_MASK_BUTTON_4: xcb_key_but_mask_t = 2048;
pub const XCB_KEY_BUT_MASK_BUTTON_5: xcb_key_but_mask_t = 4096;

pub type xcb_window_enum_t = u32;
pub const XCB_WINDOW_NONE: xcb_window_enum_t = 0;

/// Opcode for xcb_key_press.
pub const XCB_KEY_PRESS: u8 = 2i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_key_press_event_t {
    pub response_type: u8,
    pub detail: xcb_keycode_t,
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
    pub pad0: u8,
}

impl Default for xcb_key_press_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_key_release.
pub const XCB_KEY_RELEASE: u8 = 3i32 as u8;

pub type xcb_key_release_event_t = xcb_key_press_event_t;

pub type xcb_button_mask_t = u32;
pub const XCB_BUTTON_MASK_1: xcb_button_mask_t = 256;
pub const XCB_BUTTON_MASK_2: xcb_button_mask_t = 512;
pub const XCB_BUTTON_MASK_3: xcb_button_mask_t = 1024;
pub const XCB_BUTTON_MASK_4: xcb_button_mask_t = 2048;
pub const XCB_BUTTON_MASK_5: xcb_button_mask_t = 4096;
pub const XCB_BUTTON_MASK_ANY: xcb_button_mask_t = 32768;

/// Opcode for xcb_button_press.
pub const XCB_BUTTON_PRESS: u8 = 4i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_button_press_event_t {
    pub response_type: u8,
    pub detail: xcb_button_t,
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
    pub pad0: u8,
}

impl Default for xcb_button_press_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_button_release.
pub const XCB_BUTTON_RELEASE: u8 = 5i32 as u8;

pub type xcb_button_release_event_t = xcb_button_press_event_t;

pub type xcb_motion_t = u32;
pub const XCB_MOTION_NORMAL: xcb_motion_t = 0;
pub const XCB_MOTION_HINT: xcb_motion_t = 1;

/// Opcode for xcb_motion_notify.
pub const XCB_MOTION_NOTIFY: u8 = 6i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_motion_notify_event_t {
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
    pub pad0: u8,
}

impl Default for xcb_motion_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_notify_detail_t = u32;
pub const XCB_NOTIFY_DETAIL_ANCESTOR: xcb_notify_detail_t = 0;
pub const XCB_NOTIFY_DETAIL_VIRTUAL: xcb_notify_detail_t = 1;
pub const XCB_NOTIFY_DETAIL_INFERIOR: xcb_notify_detail_t = 2;
pub const XCB_NOTIFY_DETAIL_NONLINEAR: xcb_notify_detail_t = 3;
pub const XCB_NOTIFY_DETAIL_NONLINEAR_VIRTUAL: xcb_notify_detail_t = 4;
pub const XCB_NOTIFY_DETAIL_POINTER: xcb_notify_detail_t = 5;
pub const XCB_NOTIFY_DETAIL_POINTER_ROOT: xcb_notify_detail_t = 6;
pub const XCB_NOTIFY_DETAIL_NONE: xcb_notify_detail_t = 7;

pub type xcb_notify_mode_t = u32;
pub const XCB_NOTIFY_MODE_NORMAL: xcb_notify_mode_t = 0;
pub const XCB_NOTIFY_MODE_GRAB: xcb_notify_mode_t = 1;
pub const XCB_NOTIFY_MODE_UNGRAB: xcb_notify_mode_t = 2;
pub const XCB_NOTIFY_MODE_WHILE_GRABBED: xcb_notify_mode_t = 3;

/// Opcode for xcb_enter_notify.
pub const XCB_ENTER_NOTIFY: u8 = 7i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_enter_notify_event_t {
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
    pub mode: u8,
    pub same_screen_focus: u8,
}

impl Default for xcb_enter_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_leave_notify.
pub const XCB_LEAVE_NOTIFY: u8 = 8i32 as u8;

pub type xcb_leave_notify_event_t = xcb_enter_notify_event_t;

/// Opcode for xcb_focus_in.
pub const XCB_FOCUS_IN: u8 = 9i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_focus_in_event_t {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub mode: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_focus_in_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_focus_out.
pub const XCB_FOCUS_OUT: u8 = 10i32 as u8;

pub type xcb_focus_out_event_t = xcb_focus_in_event_t;

/// Opcode for xcb_keymap_notify.
pub const XCB_KEYMAP_NOTIFY: u8 = 11i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_keymap_notify_event_t {
    pub response_type: u8,
    pub keys: [u8; 31],
}

impl Default for xcb_keymap_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_expose.
pub const XCB_EXPOSE: u8 = 12i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_expose_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub count: u16,
    pub pad1: [u8; 2],
}

impl Default for xcb_expose_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_graphics_exposure.
pub const XCB_GRAPHICS_EXPOSURE: u8 = 13i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_graphics_exposure_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub drawable: xcb_drawable_t,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub minor_opcode: u16,
    pub count: u16,
    pub major_opcode: u8,
    pub pad1: [u8; 3],
}

impl Default for xcb_graphics_exposure_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_no_exposure.
pub const XCB_NO_EXPOSURE: u8 = 14i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_no_exposure_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub drawable: xcb_drawable_t,
    pub minor_opcode: u16,
    pub major_opcode: u8,
    pub pad1: u8,
}

impl Default for xcb_no_exposure_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_visibility_t = u32;
pub const XCB_VISIBILITY_UNOBSCURED: xcb_visibility_t = 0;
pub const XCB_VISIBILITY_PARTIALLY_OBSCURED: xcb_visibility_t = 1;
pub const XCB_VISIBILITY_FULLY_OBSCURED: xcb_visibility_t = 2;

/// Opcode for xcb_visibility_notify.
pub const XCB_VISIBILITY_NOTIFY: u8 = 15i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_visibility_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub state: u8,
    pub pad1: [u8; 3],
}

impl Default for xcb_visibility_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_create_notify.
pub const XCB_CREATE_NOTIFY: u8 = 16i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_create_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub parent: xcb_window_t,
    pub window: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub override_redirect: u8,
    pub pad1: u8,
}

impl Default for xcb_create_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_destroy_notify.
pub const XCB_DESTROY_NOTIFY: u8 = 17i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_destroy_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
}

impl Default for xcb_destroy_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_unmap_notify.
pub const XCB_UNMAP_NOTIFY: u8 = 18i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_unmap_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub from_configure: u8,
    pub pad1: [u8; 3],
}

impl Default for xcb_unmap_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_map_notify.
pub const XCB_MAP_NOTIFY: u8 = 19i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_map_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub override_redirect: u8,
    pub pad1: [u8; 3],
}

impl Default for xcb_map_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_map_request.
pub const XCB_MAP_REQUEST: u8 = 20i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_map_request_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub parent: xcb_window_t,
    pub window: xcb_window_t,
}

impl Default for xcb_map_request_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_reparent_notify.
pub const XCB_REPARENT_NOTIFY: u8 = 21i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_reparent_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub parent: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub override_redirect: u8,
    pub pad1: [u8; 3],
}

impl Default for xcb_reparent_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_configure_notify.
pub const XCB_CONFIGURE_NOTIFY: u8 = 22i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_configure_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub above_sibling: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub override_redirect: u8,
    pub pad1: u8,
}

impl Default for xcb_configure_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_configure_request.
pub const XCB_CONFIGURE_REQUEST: u8 = 23i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_configure_request_event_t {
    pub response_type: u8,
    pub stack_mode: u8,
    pub sequence: u16,
    pub parent: xcb_window_t,
    pub window: xcb_window_t,
    pub sibling: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub value_mask: u16,
}

impl Default for xcb_configure_request_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_gravity_notify.
pub const XCB_GRAVITY_NOTIFY: u8 = 24i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_gravity_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub x: i16,
    pub y: i16,
}

impl Default for xcb_gravity_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_resize_request.
pub const XCB_RESIZE_REQUEST: u8 = 25i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_resize_request_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub width: u16,
    pub height: u16,
}

impl Default for xcb_resize_request_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_place_t = u32;
/// The window is now on top of all siblings.
pub const XCB_PLACE_ON_TOP: xcb_place_t = 0;
/// The window is now below all siblings.
pub const XCB_PLACE_ON_BOTTOM: xcb_place_t = 1;

/// Opcode for xcb_circulate_notify.
pub const XCB_CIRCULATE_NOTIFY: u8 = 26i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_circulate_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub pad1: [u8; 4],
    pub place: u8,
    pub pad2: [u8; 3],
}

impl Default for xcb_circulate_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_circulate_request.
pub const XCB_CIRCULATE_REQUEST: u8 = 27i32 as u8;

pub type xcb_circulate_request_event_t = xcb_circulate_notify_event_t;

pub type xcb_property_t = u32;
pub const XCB_PROPERTY_NEW_VALUE: xcb_property_t = 0;
pub const XCB_PROPERTY_DELETE: xcb_property_t = 1;

/// Opcode for xcb_property_notify.
pub const XCB_PROPERTY_NOTIFY: u8 = 28i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_property_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub atom: xcb_atom_t,
    pub time: xcb_timestamp_t,
    pub state: u8,
    pub pad1: [u8; 3],
}

impl Default for xcb_property_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_selection_clear.
pub const XCB_SELECTION_CLEAR: u8 = 29i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selection_clear_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub owner: xcb_window_t,
    pub selection: xcb_atom_t,
}

impl Default for xcb_selection_clear_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_time_t = u32;
pub const XCB_TIME_CURRENT_TIME: xcb_time_t = 0;

pub type xcb_atom_enum_t = u32;
pub const XCB_ATOM_NONE: xcb_atom_enum_t = 0;
pub const XCB_ATOM_ANY: xcb_atom_enum_t = 0;
pub const XCB_ATOM_PRIMARY: xcb_atom_enum_t = 1;
pub const XCB_ATOM_SECONDARY: xcb_atom_enum_t = 2;
pub const XCB_ATOM_ARC: xcb_atom_enum_t = 3;
pub const XCB_ATOM_ATOM: xcb_atom_enum_t = 4;
pub const XCB_ATOM_BITMAP: xcb_atom_enum_t = 5;
pub const XCB_ATOM_CARDINAL: xcb_atom_enum_t = 6;
pub const XCB_ATOM_COLORMAP: xcb_atom_enum_t = 7;
pub const XCB_ATOM_CURSOR: xcb_atom_enum_t = 8;
pub const XCB_ATOM_CUT_BUFFER0: xcb_atom_enum_t = 9;
pub const XCB_ATOM_CUT_BUFFER1: xcb_atom_enum_t = 10;
pub const XCB_ATOM_CUT_BUFFER2: xcb_atom_enum_t = 11;
pub const XCB_ATOM_CUT_BUFFER3: xcb_atom_enum_t = 12;
pub const XCB_ATOM_CUT_BUFFER4: xcb_atom_enum_t = 13;
pub const XCB_ATOM_CUT_BUFFER5: xcb_atom_enum_t = 14;
pub const XCB_ATOM_CUT_BUFFER6: xcb_atom_enum_t = 15;
pub const XCB_ATOM_CUT_BUFFER7: xcb_atom_enum_t = 16;
pub const XCB_ATOM_DRAWABLE: xcb_atom_enum_t = 17;
pub const XCB_ATOM_FONT: xcb_atom_enum_t = 18;
pub const XCB_ATOM_INTEGER: xcb_atom_enum_t = 19;
pub const XCB_ATOM_PIXMAP: xcb_atom_enum_t = 20;
pub const XCB_ATOM_POINT: xcb_atom_enum_t = 21;
pub const XCB_ATOM_RECTANGLE: xcb_atom_enum_t = 22;
pub const XCB_ATOM_RESOURCE_MANAGER: xcb_atom_enum_t = 23;
pub const XCB_ATOM_RGB_COLOR_MAP: xcb_atom_enum_t = 24;
pub const XCB_ATOM_RGB_BEST_MAP: xcb_atom_enum_t = 25;
pub const XCB_ATOM_RGB_BLUE_MAP: xcb_atom_enum_t = 26;
pub const XCB_ATOM_RGB_DEFAULT_MAP: xcb_atom_enum_t = 27;
pub const XCB_ATOM_RGB_GRAY_MAP: xcb_atom_enum_t = 28;
pub const XCB_ATOM_RGB_GREEN_MAP: xcb_atom_enum_t = 29;
pub const XCB_ATOM_RGB_RED_MAP: xcb_atom_enum_t = 30;
pub const XCB_ATOM_STRING: xcb_atom_enum_t = 31;
pub const XCB_ATOM_VISUALID: xcb_atom_enum_t = 32;
pub const XCB_ATOM_WINDOW: xcb_atom_enum_t = 33;
pub const XCB_ATOM_WM_COMMAND: xcb_atom_enum_t = 34;
pub const XCB_ATOM_WM_HINTS: xcb_atom_enum_t = 35;
pub const XCB_ATOM_WM_CLIENT_MACHINE: xcb_atom_enum_t = 36;
pub const XCB_ATOM_WM_ICON_NAME: xcb_atom_enum_t = 37;
pub const XCB_ATOM_WM_ICON_SIZE: xcb_atom_enum_t = 38;
pub const XCB_ATOM_WM_NAME: xcb_atom_enum_t = 39;
pub const XCB_ATOM_WM_NORMAL_HINTS: xcb_atom_enum_t = 40;
pub const XCB_ATOM_WM_SIZE_HINTS: xcb_atom_enum_t = 41;
pub const XCB_ATOM_WM_ZOOM_HINTS: xcb_atom_enum_t = 42;
pub const XCB_ATOM_MIN_SPACE: xcb_atom_enum_t = 43;
pub const XCB_ATOM_NORM_SPACE: xcb_atom_enum_t = 44;
pub const XCB_ATOM_MAX_SPACE: xcb_atom_enum_t = 45;
pub const XCB_ATOM_END_SPACE: xcb_atom_enum_t = 46;
pub const XCB_ATOM_SUPERSCRIPT_X: xcb_atom_enum_t = 47;
pub const XCB_ATOM_SUPERSCRIPT_Y: xcb_atom_enum_t = 48;
pub const XCB_ATOM_SUBSCRIPT_X: xcb_atom_enum_t = 49;
pub const XCB_ATOM_SUBSCRIPT_Y: xcb_atom_enum_t = 50;
pub const XCB_ATOM_UNDERLINE_POSITION: xcb_atom_enum_t = 51;
pub const XCB_ATOM_UNDERLINE_THICKNESS: xcb_atom_enum_t = 52;
pub const XCB_ATOM_STRIKEOUT_ASCENT: xcb_atom_enum_t = 53;
pub const XCB_ATOM_STRIKEOUT_DESCENT: xcb_atom_enum_t = 54;
pub const XCB_ATOM_ITALIC_ANGLE: xcb_atom_enum_t = 55;
pub const XCB_ATOM_X_HEIGHT: xcb_atom_enum_t = 56;
pub const XCB_ATOM_QUAD_WIDTH: xcb_atom_enum_t = 57;
pub const XCB_ATOM_WEIGHT: xcb_atom_enum_t = 58;
pub const XCB_ATOM_POINT_SIZE: xcb_atom_enum_t = 59;
pub const XCB_ATOM_RESOLUTION: xcb_atom_enum_t = 60;
pub const XCB_ATOM_COPYRIGHT: xcb_atom_enum_t = 61;
pub const XCB_ATOM_NOTICE: xcb_atom_enum_t = 62;
pub const XCB_ATOM_FONT_NAME: xcb_atom_enum_t = 63;
pub const XCB_ATOM_FAMILY_NAME: xcb_atom_enum_t = 64;
pub const XCB_ATOM_FULL_NAME: xcb_atom_enum_t = 65;
pub const XCB_ATOM_CAP_HEIGHT: xcb_atom_enum_t = 66;
pub const XCB_ATOM_WM_CLASS: xcb_atom_enum_t = 67;
pub const XCB_ATOM_WM_TRANSIENT_FOR: xcb_atom_enum_t = 68;

/// Opcode for xcb_selection_request.
pub const XCB_SELECTION_REQUEST: u8 = 30i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selection_request_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub owner: xcb_window_t,
    pub requestor: xcb_window_t,
    pub selection: xcb_atom_t,
    pub target: xcb_atom_t,
    pub property: xcb_atom_t,
}

impl Default for xcb_selection_request_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_selection_notify.
pub const XCB_SELECTION_NOTIFY: u8 = 31i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selection_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub requestor: xcb_window_t,
    pub selection: xcb_atom_t,
    pub target: xcb_atom_t,
    pub property: xcb_atom_t,
}

impl Default for xcb_selection_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_colormap_state_t = u32;
/// The colormap was uninstalled.
pub const XCB_COLORMAP_STATE_UNINSTALLED: xcb_colormap_state_t = 0;
/// The colormap was installed.
pub const XCB_COLORMAP_STATE_INSTALLED: xcb_colormap_state_t = 1;

pub type xcb_colormap_enum_t = u32;
pub const XCB_COLORMAP_NONE: xcb_colormap_enum_t = 0;

/// Opcode for xcb_colormap_notify.
pub const XCB_COLORMAP_NOTIFY: u8 = 32i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_colormap_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub colormap: xcb_colormap_t,
    pub new: u8,
    pub state: u8,
    pub pad1: [u8; 2],
}

impl Default for xcb_colormap_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union xcb_client_message_data_t {
    pub data8: [u8; 20],
    pub data16: [u16; 10],
    pub data32: [u32; 5],
}

impl Default for xcb_client_message_data_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_client_message_data_iterator_t {
    pub data: *mut xcb_client_message_data_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_client_message_data_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_client_message.
pub const XCB_CLIENT_MESSAGE: u8 = 33i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_client_message_event_t {
    pub response_type: u8,
    pub format: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub type_: xcb_atom_t,
    pub data: xcb_client_message_data_t,
}

impl Default for xcb_client_message_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_mapping_t = u32;
pub const XCB_MAPPING_MODIFIER: xcb_mapping_t = 0;
pub const XCB_MAPPING_KEYBOARD: xcb_mapping_t = 1;
pub const XCB_MAPPING_POINTER: xcb_mapping_t = 2;

/// Opcode for xcb_mapping_notify.
pub const XCB_MAPPING_NOTIFY: u8 = 34i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_mapping_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub request: u8,
    pub first_keycode: xcb_keycode_t,
    pub count: u8,
    pub pad1: u8,
}

impl Default for xcb_mapping_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_ge_generic.
pub const XCB_GE_GENERIC: u8 = 35i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_ge_generic_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub pad0: [u8; 22],
    pub full_sequence: u32,
}

impl Default for xcb_ge_generic_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_request.
pub const XCB_REQUEST: u8 = 1i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_request_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
    pub pad0: u8,
}

impl Default for xcb_request_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_value.
pub const XCB_VALUE: u8 = 2i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_value_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
    pub pad0: u8,
}

impl Default for xcb_value_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_window.
pub const XCB_WINDOW: u8 = 3i32 as u8;

pub type xcb_window_error_t = xcb_value_error_t;

/// Opcode for xcb_pixmap.
pub const XCB_PIXMAP: u8 = 4i32 as u8;

pub type xcb_pixmap_error_t = xcb_value_error_t;

/// Opcode for xcb_atom.
pub const XCB_ATOM: u8 = 5i32 as u8;

pub type xcb_atom_error_t = xcb_value_error_t;

/// Opcode for xcb_cursor.
pub const XCB_CURSOR: u8 = 6i32 as u8;

pub type xcb_cursor_error_t = xcb_value_error_t;

/// Opcode for xcb_font.
pub const XCB_FONT: u8 = 7i32 as u8;

pub type xcb_font_error_t = xcb_value_error_t;

/// Opcode for xcb_match.
pub const XCB_MATCH: u8 = 8i32 as u8;

pub type xcb_match_error_t = xcb_request_error_t;

/// Opcode for xcb_drawable.
pub const XCB_DRAWABLE: u8 = 9i32 as u8;

pub type xcb_drawable_error_t = xcb_value_error_t;

/// Opcode for xcb_access.
pub const XCB_ACCESS: u8 = 10i32 as u8;

pub type xcb_access_error_t = xcb_request_error_t;

/// Opcode for xcb_alloc.
pub const XCB_ALLOC: u8 = 11i32 as u8;

pub type xcb_alloc_error_t = xcb_request_error_t;

/// Opcode for xcb_colormap.
pub const XCB_COLORMAP: u8 = 12i32 as u8;

pub type xcb_colormap_error_t = xcb_value_error_t;

/// Opcode for xcb_g_context.
pub const XCB_G_CONTEXT: u8 = 13i32 as u8;

pub type xcb_g_context_error_t = xcb_value_error_t;

/// Opcode for xcb_id_choice.
pub const XCB_ID_CHOICE: u8 = 14i32 as u8;

pub type xcb_id_choice_error_t = xcb_value_error_t;

/// Opcode for xcb_name.
pub const XCB_NAME: u8 = 15i32 as u8;

pub type xcb_name_error_t = xcb_request_error_t;

/// Opcode for xcb_length.
pub const XCB_LENGTH: u8 = 16i32 as u8;

pub type xcb_length_error_t = xcb_request_error_t;

/// Opcode for xcb_implementation.
pub const XCB_IMPLEMENTATION: u8 = 17i32 as u8;

pub type xcb_implementation_error_t = xcb_request_error_t;

pub type xcb_window_class_t = u32;
pub const XCB_WINDOW_CLASS_COPY_FROM_PARENT: xcb_window_class_t = 0;
pub const XCB_WINDOW_CLASS_INPUT_OUTPUT: xcb_window_class_t = 1;
pub const XCB_WINDOW_CLASS_INPUT_ONLY: xcb_window_class_t = 2;

pub type xcb_cw_t = u32;
/// Overrides the default background-pixmap. The background pixmap and window must
/// have the same root and same depth. Any size pixmap can be used, although some
/// sizes may be faster than others.
///
/// If `XCB_BACK_PIXMAP_NONE` is specified, the window has no defined background.
/// The server may fill the contents with the previous screen contents or with
/// contents of its own choosing.
///
/// If `XCB_BACK_PIXMAP_PARENT_RELATIVE` is specified, the parent's background is
/// used, but the window must have the same depth as the parent (or a Match error
/// results).   The parent's background is tracked, and the current version is
/// used each time the window background is required.
pub const XCB_CW_BACK_PIXMAP: xcb_cw_t = 1;
/// Overrides `BackPixmap`. A pixmap of undefined size filled with the specified
/// background pixel is used for the background. Range-checking is not performed,
/// the background pixel is truncated to the appropriate number of bits.
pub const XCB_CW_BACK_PIXEL: xcb_cw_t = 2;
/// Overrides the default border-pixmap. The border pixmap and window must have the
/// same root and the same depth. Any size pixmap can be used, although some sizes
/// may be faster than others.
///
/// The special value `XCB_COPY_FROM_PARENT` means the parent's border pixmap is
/// copied (subsequent changes to the parent's border attribute do not affect the
/// child), but the window must have the same depth as the parent.
pub const XCB_CW_BORDER_PIXMAP: xcb_cw_t = 4;
/// Overrides `BorderPixmap`. A pixmap of undefined size filled with the specified
/// border pixel is used for the border. Range checking is not performed on the
/// border-pixel value, it is truncated to the appropriate number of bits.
pub const XCB_CW_BORDER_PIXEL: xcb_cw_t = 8;
/// Defines which region of the window should be retained if the window is resized.
pub const XCB_CW_BIT_GRAVITY: xcb_cw_t = 16;
/// Defines how the window should be repositioned if the parent is resized (see
/// `ConfigureWindow`).
pub const XCB_CW_WIN_GRAVITY: xcb_cw_t = 32;
/// A backing-store of `WhenMapped` advises the server that maintaining contents of
/// obscured regions when the window is mapped would be beneficial. A backing-store
/// of `Always` advises the server that maintaining contents even when the window
/// is unmapped would be beneficial. In this case, the server may generate an
/// exposure event when the window is created. A value of `NotUseful` advises the
/// server that maintaining contents is unnecessary, although a server may still
/// choose to maintain contents while the window is mapped. Note that if the server
/// maintains contents, then the server should maintain complete contents not just
/// the region within the parent boundaries, even if the window is larger than its
/// parent. While the server maintains contents, exposure events will not normally
/// be generated, but the server may stop maintaining contents at any time.
pub const XCB_CW_BACKING_STORE: xcb_cw_t = 64;
/// The backing-planes indicates (with bits set to 1) which bit planes of the
/// window hold dynamic data that must be preserved in backing-stores and during
/// save-unders.
pub const XCB_CW_BACKING_PLANES: xcb_cw_t = 128;
/// The backing-pixel specifies what value to use in planes not covered by
/// backing-planes. The server is free to save only the specified bit planes in the
/// backing-store or save-under and regenerate the remaining planes with the
/// specified pixel value. Any bits beyond the specified depth of the window in
/// these values are simply ignored.
pub const XCB_CW_BACKING_PIXEL: xcb_cw_t = 256;
/// The override-redirect specifies whether map and configure requests on this
/// window should override a SubstructureRedirect on the parent, typically to
/// inform a window manager not to tamper with the window.
pub const XCB_CW_OVERRIDE_REDIRECT: xcb_cw_t = 512;
/// If 1, the server is advised that when this window is mapped, saving the
/// contents of windows it obscures would be beneficial.
pub const XCB_CW_SAVE_UNDER: xcb_cw_t = 1024;
/// The event-mask defines which events the client is interested in for this window
/// (or for some event types, inferiors of the window).
pub const XCB_CW_EVENT_MASK: xcb_cw_t = 2048;
/// The do-not-propagate-mask defines which events should not be propagated to
/// ancestor windows when no client has the event type selected in this window.
pub const XCB_CW_DONT_PROPAGATE: xcb_cw_t = 4096;
/// The colormap specifies the colormap that best reflects the true colors of the window. Servers
/// capable of supporting multiple hardware colormaps may use this information, and window man-
/// agers may use it for InstallColormap requests. The colormap must have the same visual type
/// and root as the window (or a Match error results). If CopyFromParent is specified, the parent's
/// colormap is copied (subsequent changes to the parent's colormap attribute do not affect the child).
/// However, the window must have the same visual type as the parent (or a Match error results),
/// and the parent must not have a colormap of None (or a Match error results). For an explanation
/// of None, see FreeColormap request. The colormap is copied by sharing the colormap object
/// between the child and the parent, not by making a complete copy of the colormap contents.
pub const XCB_CW_COLORMAP: xcb_cw_t = 8192;
/// If a cursor is specified, it will be used whenever the pointer is in the window. If None is speci-
/// fied, the parent's cursor will be used when the pointer is in the window, and any change in the
/// parent's cursor will cause an immediate change in the displayed cursor.
pub const XCB_CW_CURSOR: xcb_cw_t = 16384;

pub type xcb_back_pixmap_t = u32;
pub const XCB_BACK_PIXMAP_NONE: xcb_back_pixmap_t = 0;
pub const XCB_BACK_PIXMAP_PARENT_RELATIVE: xcb_back_pixmap_t = 1;

pub type xcb_gravity_t = u32;
pub const XCB_GRAVITY_BIT_FORGET: xcb_gravity_t = 0;
pub const XCB_GRAVITY_WIN_UNMAP: xcb_gravity_t = 0;
pub const XCB_GRAVITY_NORTH_WEST: xcb_gravity_t = 1;
pub const XCB_GRAVITY_NORTH: xcb_gravity_t = 2;
pub const XCB_GRAVITY_NORTH_EAST: xcb_gravity_t = 3;
pub const XCB_GRAVITY_WEST: xcb_gravity_t = 4;
pub const XCB_GRAVITY_CENTER: xcb_gravity_t = 5;
pub const XCB_GRAVITY_EAST: xcb_gravity_t = 6;
pub const XCB_GRAVITY_SOUTH_WEST: xcb_gravity_t = 7;
pub const XCB_GRAVITY_SOUTH: xcb_gravity_t = 8;
pub const XCB_GRAVITY_SOUTH_EAST: xcb_gravity_t = 9;
pub const XCB_GRAVITY_STATIC: xcb_gravity_t = 10;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_create_window_value_list_t {
    pub background_pixmap: xcb_pixmap_t,
    pub background_pixel: u32,
    pub border_pixmap: xcb_pixmap_t,
    pub border_pixel: u32,
    pub bit_gravity: u32,
    pub win_gravity: u32,
    pub backing_store: u32,
    pub backing_planes: u32,
    pub backing_pixel: u32,
    pub override_redirect: xcb_bool32_t,
    pub save_under: xcb_bool32_t,
    pub event_mask: u32,
    pub do_not_propogate_mask: u32,
    pub colormap: xcb_colormap_t,
    pub cursor: xcb_cursor_t,
}

impl Default for xcb_create_window_value_list_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_create_window.
pub const XCB_CREATE_WINDOW: u8 = 1i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_create_window_request_t {
    pub major_opcode: u8,
    pub depth: u8,
    pub length: u16,
    pub wid: xcb_window_t,
    pub parent: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub class: u16,
    pub visual: xcb_visualid_t,
    pub value_mask: u32,
}

impl Default for xcb_create_window_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_change_window_attributes_value_list_t {
    pub background_pixmap: xcb_pixmap_t,
    pub background_pixel: u32,
    pub border_pixmap: xcb_pixmap_t,
    pub border_pixel: u32,
    pub bit_gravity: u32,
    pub win_gravity: u32,
    pub backing_store: u32,
    pub backing_planes: u32,
    pub backing_pixel: u32,
    pub override_redirect: xcb_bool32_t,
    pub save_under: xcb_bool32_t,
    pub event_mask: u32,
    pub do_not_propogate_mask: u32,
    pub colormap: xcb_colormap_t,
    pub cursor: xcb_cursor_t,
}

impl Default for xcb_change_window_attributes_value_list_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_change_window_attributes.
pub const XCB_CHANGE_WINDOW_ATTRIBUTES: u8 = 2i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_change_window_attributes_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub value_mask: u32,
}

impl Default for xcb_change_window_attributes_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_map_state_t = u32;
pub const XCB_MAP_STATE_UNMAPPED: xcb_map_state_t = 0;
pub const XCB_MAP_STATE_UNVIEWABLE: xcb_map_state_t = 1;
pub const XCB_MAP_STATE_VIEWABLE: xcb_map_state_t = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_window_attributes_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_get_window_attributes_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_get_window_attributes.
pub const XCB_GET_WINDOW_ATTRIBUTES: u8 = 3i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_window_attributes_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_get_window_attributes_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_window_attributes_reply_t {
    pub response_type: u8,
    pub backing_store: u8,
    pub sequence: u16,
    pub length: u32,
    pub visual: xcb_visualid_t,
    pub class: u16,
    pub bit_gravity: u8,
    pub win_gravity: u8,
    pub backing_planes: u32,
    pub backing_pixel: u32,
    pub save_under: u8,
    pub map_is_installed: u8,
    pub map_state: u8,
    pub override_redirect: u8,
    pub colormap: xcb_colormap_t,
    pub all_event_masks: u32,
    pub your_event_mask: u32,
    pub do_not_propagate_mask: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_get_window_attributes_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_destroy_window.
pub const XCB_DESTROY_WINDOW: u8 = 4i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_destroy_window_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_destroy_window_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_destroy_subwindows.
pub const XCB_DESTROY_SUBWINDOWS: u8 = 5i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_destroy_subwindows_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_destroy_subwindows_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_set_mode_t = u32;
pub const XCB_SET_MODE_INSERT: xcb_set_mode_t = 0;
pub const XCB_SET_MODE_DELETE: xcb_set_mode_t = 1;

/// Opcode for xcb_change_save_set.
pub const XCB_CHANGE_SAVE_SET: u8 = 6i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_change_save_set_request_t {
    pub major_opcode: u8,
    pub mode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_change_save_set_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_reparent_window.
pub const XCB_REPARENT_WINDOW: u8 = 7i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_reparent_window_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub parent: xcb_window_t,
    pub x: i16,
    pub y: i16,
}

impl Default for xcb_reparent_window_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_map_window.
pub const XCB_MAP_WINDOW: u8 = 8i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_map_window_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_map_window_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_map_subwindows.
pub const XCB_MAP_SUBWINDOWS: u8 = 9i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_map_subwindows_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_map_subwindows_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_unmap_window.
pub const XCB_UNMAP_WINDOW: u8 = 10i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_unmap_window_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_unmap_window_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_unmap_subwindows.
pub const XCB_UNMAP_SUBWINDOWS: u8 = 11i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_unmap_subwindows_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_unmap_subwindows_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_config_window_t = u32;
pub const XCB_CONFIG_WINDOW_X: xcb_config_window_t = 1;
pub const XCB_CONFIG_WINDOW_Y: xcb_config_window_t = 2;
pub const XCB_CONFIG_WINDOW_WIDTH: xcb_config_window_t = 4;
pub const XCB_CONFIG_WINDOW_HEIGHT: xcb_config_window_t = 8;
pub const XCB_CONFIG_WINDOW_BORDER_WIDTH: xcb_config_window_t = 16;
pub const XCB_CONFIG_WINDOW_SIBLING: xcb_config_window_t = 32;
pub const XCB_CONFIG_WINDOW_STACK_MODE: xcb_config_window_t = 64;

pub type xcb_stack_mode_t = u32;
pub const XCB_STACK_MODE_ABOVE: xcb_stack_mode_t = 0;
pub const XCB_STACK_MODE_BELOW: xcb_stack_mode_t = 1;
pub const XCB_STACK_MODE_TOP_IF: xcb_stack_mode_t = 2;
pub const XCB_STACK_MODE_BOTTOM_IF: xcb_stack_mode_t = 3;
pub const XCB_STACK_MODE_OPPOSITE: xcb_stack_mode_t = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_configure_window_value_list_t {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub border_width: u32,
    pub sibling: xcb_window_t,
    pub stack_mode: u32,
}

impl Default for xcb_configure_window_value_list_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_configure_window.
pub const XCB_CONFIGURE_WINDOW: u8 = 12i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_configure_window_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub value_mask: u16,
    pub pad1: [u8; 2],
}

impl Default for xcb_configure_window_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_circulate_t = u32;
pub const XCB_CIRCULATE_RAISE_LOWEST: xcb_circulate_t = 0;
pub const XCB_CIRCULATE_LOWER_HIGHEST: xcb_circulate_t = 1;

/// Opcode for xcb_circulate_window.
pub const XCB_CIRCULATE_WINDOW: u8 = 13i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_circulate_window_request_t {
    pub major_opcode: u8,
    pub direction: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_circulate_window_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_geometry_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_get_geometry_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_get_geometry.
pub const XCB_GET_GEOMETRY: u8 = 14i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_geometry_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
}

impl Default for xcb_get_geometry_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_geometry_reply_t {
    pub response_type: u8,
    pub depth: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_get_geometry_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_tree_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_query_tree_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_query_tree.
pub const XCB_QUERY_TREE: u8 = 15i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_tree_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_query_tree_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_tree_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: xcb_window_t,
    pub parent: xcb_window_t,
    pub children_len: u16,
    pub pad1: [u8; 14],
}

impl Default for xcb_query_tree_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_intern_atom_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_intern_atom_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_intern_atom.
pub const XCB_INTERN_ATOM: u8 = 16i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_intern_atom_request_t {
    pub major_opcode: u8,
    pub only_if_exists: u8,
    pub length: u16,
    pub name_len: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_intern_atom_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_intern_atom_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub atom: xcb_atom_t,
}

impl Default for xcb_intern_atom_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_atom_name_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_get_atom_name_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_get_atom_name.
pub const XCB_GET_ATOM_NAME: u8 = 17i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_atom_name_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub atom: xcb_atom_t,
}

impl Default for xcb_get_atom_name_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_atom_name_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub name_len: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_get_atom_name_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_prop_mode_t = u32;
/// Discard the previous property value and store the new data.
pub const XCB_PROP_MODE_REPLACE: xcb_prop_mode_t = 0;
/// Insert the new data before the beginning of existing data. The `format` must
/// match existing property value. If the property is undefined, it is treated as
/// defined with the correct type and format with zero-length data.
pub const XCB_PROP_MODE_PREPEND: xcb_prop_mode_t = 1;
/// Insert the new data after the beginning of existing data. The `format` must
/// match existing property value. If the property is undefined, it is treated as
/// defined with the correct type and format with zero-length data.
pub const XCB_PROP_MODE_APPEND: xcb_prop_mode_t = 2;

/// Opcode for xcb_change_property.
pub const XCB_CHANGE_PROPERTY: u8 = 18i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_change_property_request_t {
    pub major_opcode: u8,
    pub mode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub property: xcb_atom_t,
    pub type_: xcb_atom_t,
    pub format: u8,
    pub pad0: [u8; 3],
    pub data_len: u32,
}

impl Default for xcb_change_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_delete_property.
pub const XCB_DELETE_PROPERTY: u8 = 19i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_delete_property_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub property: xcb_atom_t,
}

impl Default for xcb_delete_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_get_property_type_t = u32;
pub const XCB_GET_PROPERTY_TYPE_ANY: xcb_get_property_type_t = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_property_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_get_property_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_get_property.
pub const XCB_GET_PROPERTY: u8 = 20i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_property_request_t {
    pub major_opcode: u8,
    pub delete: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub property: xcb_atom_t,
    pub type_: xcb_atom_t,
    pub long_offset: u32,
    pub long_length: u32,
}

impl Default for xcb_get_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_property_reply_t {
    pub response_type: u8,
    pub format: u8,
    pub sequence: u16,
    pub length: u32,
    pub type_: xcb_atom_t,
    pub bytes_after: u32,
    pub value_len: u32,
    pub pad0: [u8; 12],
}

impl Default for xcb_get_property_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_properties_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_list_properties_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_list_properties.
pub const XCB_LIST_PROPERTIES: u8 = 21i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_properties_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_list_properties_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_properties_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub atoms_len: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_list_properties_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_set_selection_owner.
pub const XCB_SET_SELECTION_OWNER: u8 = 22i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_selection_owner_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub owner: xcb_window_t,
    pub selection: xcb_atom_t,
    pub time: xcb_timestamp_t,
}

impl Default for xcb_set_selection_owner_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_selection_owner_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_get_selection_owner_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_get_selection_owner.
pub const XCB_GET_SELECTION_OWNER: u8 = 23i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_selection_owner_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub selection: xcb_atom_t,
}

impl Default for xcb_get_selection_owner_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_selection_owner_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub owner: xcb_window_t,
}

impl Default for xcb_get_selection_owner_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_convert_selection.
pub const XCB_CONVERT_SELECTION: u8 = 24i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_convert_selection_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub requestor: xcb_window_t,
    pub selection: xcb_atom_t,
    pub target: xcb_atom_t,
    pub property: xcb_atom_t,
    pub time: xcb_timestamp_t,
}

impl Default for xcb_convert_selection_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_send_event_dest_t = u32;
pub const XCB_SEND_EVENT_DEST_POINTER_WINDOW: xcb_send_event_dest_t = 0;
pub const XCB_SEND_EVENT_DEST_ITEM_FOCUS: xcb_send_event_dest_t = 1;

/// Opcode for xcb_send_event.
pub const XCB_SEND_EVENT: u8 = 25i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_send_event_request_t {
    pub major_opcode: u8,
    pub propagate: u8,
    pub length: u16,
    pub destination: xcb_window_t,
    pub event_mask: u32,
    pub event: [c_char; 32],
}

impl Default for xcb_send_event_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_grab_mode_t = u32;
/// The state of the keyboard appears to freeze: No further keyboard events are
/// generated by the server until the grabbing client issues a releasing
/// `AllowEvents` request or until the keyboard grab is released.
pub const XCB_GRAB_MODE_SYNC: xcb_grab_mode_t = 0;
/// Keyboard event processing continues normally.
pub const XCB_GRAB_MODE_ASYNC: xcb_grab_mode_t = 1;

pub type xcb_grab_status_t = u32;
pub const XCB_GRAB_STATUS_SUCCESS: xcb_grab_status_t = 0;
pub const XCB_GRAB_STATUS_ALREADY_GRABBED: xcb_grab_status_t = 1;
pub const XCB_GRAB_STATUS_INVALID_TIME: xcb_grab_status_t = 2;
pub const XCB_GRAB_STATUS_NOT_VIEWABLE: xcb_grab_status_t = 3;
pub const XCB_GRAB_STATUS_FROZEN: xcb_grab_status_t = 4;

pub type xcb_cursor_enum_t = u32;
pub const XCB_CURSOR_NONE: xcb_cursor_enum_t = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_grab_pointer_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_grab_pointer_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_grab_pointer.
pub const XCB_GRAB_POINTER: u8 = 26i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_grab_pointer_request_t {
    pub major_opcode: u8,
    pub owner_events: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub event_mask: u16,
    pub pointer_mode: u8,
    pub keyboard_mode: u8,
    pub confine_to: xcb_window_t,
    pub cursor: xcb_cursor_t,
    pub time: xcb_timestamp_t,
}

impl Default for xcb_grab_pointer_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_grab_pointer_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
}

impl Default for xcb_grab_pointer_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_ungrab_pointer.
pub const XCB_UNGRAB_POINTER: u8 = 27i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_ungrab_pointer_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub time: xcb_timestamp_t,
}

impl Default for xcb_ungrab_pointer_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_button_index_t = u32;
/// Any of the following (or none):
pub const XCB_BUTTON_INDEX_ANY: xcb_button_index_t = 0;
/// The left mouse button.
pub const XCB_BUTTON_INDEX_1: xcb_button_index_t = 1;
/// The right mouse button.
pub const XCB_BUTTON_INDEX_2: xcb_button_index_t = 2;
/// The middle mouse button.
pub const XCB_BUTTON_INDEX_3: xcb_button_index_t = 3;
/// Scroll wheel. TODO: direction?
pub const XCB_BUTTON_INDEX_4: xcb_button_index_t = 4;
/// Scroll wheel. TODO: direction?
pub const XCB_BUTTON_INDEX_5: xcb_button_index_t = 5;

/// Opcode for xcb_grab_button.
pub const XCB_GRAB_BUTTON: u8 = 28i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_grab_button_request_t {
    pub major_opcode: u8,
    pub owner_events: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub event_mask: u16,
    pub pointer_mode: u8,
    pub keyboard_mode: u8,
    pub confine_to: xcb_window_t,
    pub cursor: xcb_cursor_t,
    pub button: u8,
    pub pad0: u8,
    pub modifiers: u16,
}

impl Default for xcb_grab_button_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_ungrab_button.
pub const XCB_UNGRAB_BUTTON: u8 = 29i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_ungrab_button_request_t {
    pub major_opcode: u8,
    pub button: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub modifiers: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_ungrab_button_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_change_active_pointer_grab.
pub const XCB_CHANGE_ACTIVE_POINTER_GRAB: u8 = 30i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_change_active_pointer_grab_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cursor: xcb_cursor_t,
    pub time: xcb_timestamp_t,
    pub event_mask: u16,
    pub pad1: [u8; 2],
}

impl Default for xcb_change_active_pointer_grab_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_grab_keyboard_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_grab_keyboard_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_grab_keyboard.
pub const XCB_GRAB_KEYBOARD: u8 = 31i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_grab_keyboard_request_t {
    pub major_opcode: u8,
    pub owner_events: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub time: xcb_timestamp_t,
    pub pointer_mode: u8,
    pub keyboard_mode: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_grab_keyboard_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_grab_keyboard_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
}

impl Default for xcb_grab_keyboard_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_ungrab_keyboard.
pub const XCB_UNGRAB_KEYBOARD: u8 = 32i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_ungrab_keyboard_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub time: xcb_timestamp_t,
}

impl Default for xcb_ungrab_keyboard_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_grab_t = u32;
pub const XCB_GRAB_ANY: xcb_grab_t = 0;

/// Opcode for xcb_grab_key.
pub const XCB_GRAB_KEY: u8 = 33i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_grab_key_request_t {
    pub major_opcode: u8,
    pub owner_events: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub modifiers: u16,
    pub key: xcb_keycode_t,
    pub pointer_mode: u8,
    pub keyboard_mode: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_grab_key_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_ungrab_key.
pub const XCB_UNGRAB_KEY: u8 = 34i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_ungrab_key_request_t {
    pub major_opcode: u8,
    pub key: xcb_keycode_t,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub modifiers: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_ungrab_key_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_allow_t = u32;
/// For AsyncPointer, if the pointer is frozen by the client, pointer event
/// processing continues normally. If the pointer is frozen twice by the client on
/// behalf of two separate grabs, AsyncPointer thaws for both. AsyncPointer has no
/// effect if the pointer is not frozen by the client, but the pointer need not be
/// grabbed by the client.
///
/// TODO: rewrite this in more understandable terms.
pub const XCB_ALLOW_ASYNC_POINTER: xcb_allow_t = 0;
/// For SyncPointer, if the pointer is frozen and actively grabbed by the client,
/// pointer event processing continues normally until the next ButtonPress or
/// ButtonRelease event is reported to the client, at which time the pointer again
/// appears to freeze. However, if the reported event causes the pointer grab to be
/// released, then the pointer does not freeze. SyncPointer has no effect if the
/// pointer is not frozen by the client or if the pointer is not grabbed by the
/// client.
pub const XCB_ALLOW_SYNC_POINTER: xcb_allow_t = 1;
/// For ReplayPointer, if the pointer is actively grabbed by the client and is
/// frozen as the result of an event having been sent to the client (either from
/// the activation of a GrabButton or from a previous AllowEvents with mode
/// SyncPointer but not from a GrabPointer), then the pointer grab is released and
/// that event is completely reprocessed, this time ignoring any passive grabs at
/// or above (towards the root) the grab-window of the grab just released. The
/// request has no effect if the pointer is not grabbed by the client or if the
/// pointer is not frozen as the result of an event.
pub const XCB_ALLOW_REPLAY_POINTER: xcb_allow_t = 2;
/// For AsyncKeyboard, if the keyboard is frozen by the client, keyboard event
/// processing continues normally. If the keyboard is frozen twice by the client on
/// behalf of two separate grabs, AsyncKeyboard thaws for both. AsyncKeyboard has
/// no effect if the keyboard is not frozen by the client, but the keyboard need
/// not be grabbed by the client.
pub const XCB_ALLOW_ASYNC_KEYBOARD: xcb_allow_t = 3;
/// For SyncKeyboard, if the keyboard is frozen and actively grabbed by the client,
/// keyboard event processing continues normally until the next KeyPress or
/// KeyRelease event is reported to the client, at which time the keyboard again
/// appears to freeze. However, if the reported event causes the keyboard grab to
/// be released, then the keyboard does not freeze. SyncKeyboard has no effect if
/// the keyboard is not frozen by the client or if the keyboard is not grabbed by
/// the client.
pub const XCB_ALLOW_SYNC_KEYBOARD: xcb_allow_t = 4;
/// For ReplayKeyboard, if the keyboard is actively grabbed by the client and is
/// frozen as the result of an event having been sent to the client (either from
/// the activation of a GrabKey or from a previous AllowEvents with mode
/// SyncKeyboard but not from a GrabKeyboard), then the keyboard grab is released
/// and that event is completely reprocessed, this time ignoring any passive grabs
/// at or above (towards the root) the grab-window of the grab just released. The
/// request has no effect if the keyboard is not grabbed by the client or if the
/// keyboard is not frozen as the result of an event.
pub const XCB_ALLOW_REPLAY_KEYBOARD: xcb_allow_t = 5;
/// For AsyncBoth, if the pointer and the keyboard are frozen by the client, event
/// processing for both devices continues normally. If a device is frozen twice by
/// the client on behalf of two separate grabs, AsyncBoth thaws for both. AsyncBoth
/// has no effect unless both pointer and keyboard are frozen by the client.
pub const XCB_ALLOW_ASYNC_BOTH: xcb_allow_t = 6;
/// For SyncBoth, if both pointer and keyboard are frozen by the client, event
/// processing (for both devices) continues normally until the next ButtonPress,
/// ButtonRelease, KeyPress, or KeyRelease event is reported to the client for a
/// grabbed device (button event for the pointer, key event for the keyboard), at
/// which time the devices again appear to freeze. However, if the reported event
/// causes the grab to be released, then the devices do not freeze (but if the
/// other device is still grabbed, then a subsequent event for it will still cause
/// both devices to freeze). SyncBoth has no effect unless both pointer and
/// keyboard are frozen by the client. If the pointer or keyboard is frozen twice
/// by the client on behalf of two separate grabs, SyncBoth thaws for both (but a
/// subsequent freeze for SyncBoth will only freeze each device once).
pub const XCB_ALLOW_SYNC_BOTH: xcb_allow_t = 7;

/// Opcode for xcb_allow_events.
pub const XCB_ALLOW_EVENTS: u8 = 35i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_allow_events_request_t {
    pub major_opcode: u8,
    pub mode: u8,
    pub length: u16,
    pub time: xcb_timestamp_t,
}

impl Default for xcb_allow_events_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_grab_server.
pub const XCB_GRAB_SERVER: u8 = 36i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_grab_server_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_grab_server_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_ungrab_server.
pub const XCB_UNGRAB_SERVER: u8 = 37i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_ungrab_server_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_ungrab_server_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_pointer_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_query_pointer_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_query_pointer.
pub const XCB_QUERY_POINTER: u8 = 38i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_pointer_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_query_pointer_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_pointer_reply_t {
    pub response_type: u8,
    pub same_screen: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub win_x: i16,
    pub win_y: i16,
    pub mask: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_query_pointer_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_timecoord_t {
    pub time: xcb_timestamp_t,
    pub x: i16,
    pub y: i16,
}

impl Default for xcb_timecoord_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_timecoord_iterator_t {
    pub data: *mut xcb_timecoord_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_timecoord_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_motion_events_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_get_motion_events_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_get_motion_events.
pub const XCB_GET_MOTION_EVENTS: u8 = 39i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_motion_events_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub start: xcb_timestamp_t,
    pub stop: xcb_timestamp_t,
}

impl Default for xcb_get_motion_events_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_motion_events_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub events_len: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_get_motion_events_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_translate_coordinates_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_translate_coordinates_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_translate_coordinates.
pub const XCB_TRANSLATE_COORDINATES: u8 = 40i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_translate_coordinates_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub src_window: xcb_window_t,
    pub dst_window: xcb_window_t,
    pub src_x: i16,
    pub src_y: i16,
}

impl Default for xcb_translate_coordinates_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_translate_coordinates_reply_t {
    pub response_type: u8,
    pub same_screen: u8,
    pub sequence: u16,
    pub length: u32,
    pub child: xcb_window_t,
    pub dst_x: i16,
    pub dst_y: i16,
}

impl Default for xcb_translate_coordinates_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_warp_pointer.
pub const XCB_WARP_POINTER: u8 = 41i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_warp_pointer_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub src_window: xcb_window_t,
    pub dst_window: xcb_window_t,
    pub src_x: i16,
    pub src_y: i16,
    pub src_width: u16,
    pub src_height: u16,
    pub dst_x: i16,
    pub dst_y: i16,
}

impl Default for xcb_warp_pointer_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_input_focus_t = u32;
/// The focus reverts to `XCB_NONE`, so no window will have the input focus.
pub const XCB_INPUT_FOCUS_NONE: xcb_input_focus_t = 0;
/// The focus reverts to `XCB_POINTER_ROOT` respectively. When the focus reverts,
/// FocusIn and FocusOut events are generated, but the last-focus-change time is
/// not changed.
pub const XCB_INPUT_FOCUS_POINTER_ROOT: xcb_input_focus_t = 1;
/// The focus reverts to the parent (or closest viewable ancestor) and the new
/// revert_to value is `XCB_INPUT_FOCUS_NONE`.
pub const XCB_INPUT_FOCUS_PARENT: xcb_input_focus_t = 2;
/// NOT YET DOCUMENTED. Only relevant for the xinput extension.
pub const XCB_INPUT_FOCUS_FOLLOW_KEYBOARD: xcb_input_focus_t = 3;

/// Opcode for xcb_set_input_focus.
pub const XCB_SET_INPUT_FOCUS: u8 = 42i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_input_focus_request_t {
    pub major_opcode: u8,
    pub revert_to: u8,
    pub length: u16,
    pub focus: xcb_window_t,
    pub time: xcb_timestamp_t,
}

impl Default for xcb_set_input_focus_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_input_focus_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_get_input_focus_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_get_input_focus.
pub const XCB_GET_INPUT_FOCUS: u8 = 43i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_input_focus_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_get_input_focus_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_input_focus_reply_t {
    pub response_type: u8,
    pub revert_to: u8,
    pub sequence: u16,
    pub length: u32,
    pub focus: xcb_window_t,
}

impl Default for xcb_get_input_focus_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_keymap_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_query_keymap_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_query_keymap.
pub const XCB_QUERY_KEYMAP: u8 = 44i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_keymap_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_query_keymap_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_keymap_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub keys: [u8; 32],
}

impl Default for xcb_query_keymap_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_open_font.
pub const XCB_OPEN_FONT: u8 = 45i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_open_font_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub fid: xcb_font_t,
    pub name_len: u16,
    pub pad1: [u8; 2],
}

impl Default for xcb_open_font_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_close_font.
pub const XCB_CLOSE_FONT: u8 = 46i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_close_font_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub font: xcb_font_t,
}

impl Default for xcb_close_font_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_font_draw_t = u32;
pub const XCB_FONT_DRAW_LEFT_TO_RIGHT: xcb_font_draw_t = 0;
pub const XCB_FONT_DRAW_RIGHT_TO_LEFT: xcb_font_draw_t = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_fontprop_t {
    pub name: xcb_atom_t,
    pub value: u32,
}

impl Default for xcb_fontprop_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_fontprop_iterator_t {
    pub data: *mut xcb_fontprop_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_fontprop_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_charinfo_t {
    pub left_side_bearing: i16,
    pub right_side_bearing: i16,
    pub character_width: i16,
    pub ascent: i16,
    pub descent: i16,
    pub attributes: u16,
}

impl Default for xcb_charinfo_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_charinfo_iterator_t {
    pub data: *mut xcb_charinfo_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_charinfo_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_font_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_query_font_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_query_font.
pub const XCB_QUERY_FONT: u8 = 47i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_font_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub font: xcb_fontable_t,
}

impl Default for xcb_query_font_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_font_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub min_bounds: xcb_charinfo_t,
    pub pad1: [u8; 4],
    pub max_bounds: xcb_charinfo_t,
    pub pad2: [u8; 4],
    pub min_char_or_byte2: u16,
    pub max_char_or_byte2: u16,
    pub default_char: u16,
    pub properties_len: u16,
    pub draw_direction: u8,
    pub min_byte1: u8,
    pub max_byte1: u8,
    pub all_chars_exist: u8,
    pub font_ascent: i16,
    pub font_descent: i16,
    pub char_infos_len: u32,
}

impl Default for xcb_query_font_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_text_extents_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_query_text_extents_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_query_text_extents.
pub const XCB_QUERY_TEXT_EXTENTS: u8 = 48i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_text_extents_request_t {
    pub major_opcode: u8,
    pub odd_length: u8,
    pub length: u16,
    pub font: xcb_fontable_t,
}

impl Default for xcb_query_text_extents_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_text_extents_reply_t {
    pub response_type: u8,
    pub draw_direction: u8,
    pub sequence: u16,
    pub length: u32,
    pub font_ascent: i16,
    pub font_descent: i16,
    pub overall_ascent: i16,
    pub overall_descent: i16,
    pub overall_width: i32,
    pub overall_left: i32,
    pub overall_right: i32,
}

impl Default for xcb_query_text_extents_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_str_t {
    pub name_len: u8,
}

impl Default for xcb_str_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_str_iterator_t {
    pub data: *mut xcb_str_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_str_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_fonts_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_list_fonts_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_list_fonts.
pub const XCB_LIST_FONTS: u8 = 49i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_fonts_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub max_names: u16,
    pub pattern_len: u16,
}

impl Default for xcb_list_fonts_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_fonts_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub names_len: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_list_fonts_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_fonts_with_info_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_list_fonts_with_info_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_list_fonts_with_info.
pub const XCB_LIST_FONTS_WITH_INFO: u8 = 50i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_fonts_with_info_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub max_names: u16,
    pub pattern_len: u16,
}

impl Default for xcb_list_fonts_with_info_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_fonts_with_info_reply_t {
    pub response_type: u8,
    pub name_len: u8,
    pub sequence: u16,
    pub length: u32,
    pub min_bounds: xcb_charinfo_t,
    pub pad0: [u8; 4],
    pub max_bounds: xcb_charinfo_t,
    pub pad1: [u8; 4],
    pub min_char_or_byte2: u16,
    pub max_char_or_byte2: u16,
    pub default_char: u16,
    pub properties_len: u16,
    pub draw_direction: u8,
    pub min_byte1: u8,
    pub max_byte1: u8,
    pub all_chars_exist: u8,
    pub font_ascent: i16,
    pub font_descent: i16,
    pub replies_hint: u32,
}

impl Default for xcb_list_fonts_with_info_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_set_font_path.
pub const XCB_SET_FONT_PATH: u8 = 51i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_font_path_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub font_qty: u16,
    pub pad1: [u8; 2],
}

impl Default for xcb_set_font_path_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_font_path_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_get_font_path_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_get_font_path.
pub const XCB_GET_FONT_PATH: u8 = 52i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_font_path_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_get_font_path_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_font_path_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub path_len: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_get_font_path_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_create_pixmap.
pub const XCB_CREATE_PIXMAP: u8 = 53i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_create_pixmap_request_t {
    pub major_opcode: u8,
    pub depth: u8,
    pub length: u16,
    pub pid: xcb_pixmap_t,
    pub drawable: xcb_drawable_t,
    pub width: u16,
    pub height: u16,
}

impl Default for xcb_create_pixmap_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_free_pixmap.
pub const XCB_FREE_PIXMAP: u8 = 54i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_free_pixmap_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub pixmap: xcb_pixmap_t,
}

impl Default for xcb_free_pixmap_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_gc_t = u32;
/// TODO: Refer to GX
pub const XCB_GC_FUNCTION: xcb_gc_t = 1;
/// In graphics operations, given a source and destination pixel, the result is
/// computed bitwise on corresponding bits of the pixels; that is, a Boolean
/// operation is performed in each bit plane. The plane-mask restricts the
/// operation to a subset of planes, so the result is:
///
///         ((src FUNC dst) AND plane-mask) OR (dst AND (NOT plane-mask))
pub const XCB_GC_PLANE_MASK: xcb_gc_t = 2;
/// Foreground colorpixel.
pub const XCB_GC_FOREGROUND: xcb_gc_t = 4;
/// Background colorpixel.
pub const XCB_GC_BACKGROUND: xcb_gc_t = 8;
/// The line-width is measured in pixels and can be greater than or equal to one, a wide line, or the
/// special value zero, a thin line.
pub const XCB_GC_LINE_WIDTH: xcb_gc_t = 16;
/// The line-style defines which sections of a line are drawn:
/// Solid                The full path of the line is drawn.
/// DoubleDash           The full path of the line is drawn, but the even dashes are filled differently
///                      than the odd dashes (see fill-style), with Butt cap-style used where even and
///                      odd dashes meet.
/// OnOffDash            Only the even dashes are drawn, and cap-style applies to all internal ends of
///                      the individual dashes (except NotLast is treated as Butt).
pub const XCB_GC_LINE_STYLE: xcb_gc_t = 32;
/// The cap-style defines how the endpoints of a path are drawn:
/// NotLast    The result is equivalent to Butt, except that for a line-width of zero the final
///            endpoint is not drawn.
/// Butt       The result is square at the endpoint (perpendicular to the slope of the line)
///            with no projection beyond.
/// Round      The result is a circular arc with its diameter equal to the line-width, centered
///            on the endpoint; it is equivalent to Butt for line-width zero.
/// Projecting The result is square at the end, but the path continues beyond the endpoint for
///            a distance equal to half the line-width; it is equivalent to Butt for line-width
///            zero.
pub const XCB_GC_CAP_STYLE: xcb_gc_t = 64;
/// The join-style defines how corners are drawn for wide lines:
/// Miter               The outer edges of the two lines extend to meet at an angle. However, if the
///                     angle is less than 11 degrees, a Bevel join-style is used instead.
/// Round               The result is a circular arc with a diameter equal to the line-width, centered
///                     on the joinpoint.
/// Bevel               The result is Butt endpoint styles, and then the triangular notch is filled.
pub const XCB_GC_JOIN_STYLE: xcb_gc_t = 128;
/// The fill-style defines the contents of the source for line, text, and fill requests. For all text and fill
/// requests (for example, PolyText8, PolyText16, PolyFillRectangle, FillPoly, and PolyFillArc)
/// as well as for line requests with line-style Solid, (for example, PolyLine, PolySegment,
/// PolyRectangle, PolyArc) and for the even dashes for line requests with line-style OnOffDash
/// or DoubleDash:
/// Solid                     Foreground
/// Tiled                     Tile
/// OpaqueStippled            A tile with the same width and height as stipple but with background
///                           everywhere stipple has a zero and with foreground everywhere stipple
///                           has a one
/// Stippled                  Foreground masked by stipple
/// For the odd dashes for line requests with line-style DoubleDash:
/// Solid                     Background
/// Tiled                     Same as for even dashes
/// OpaqueStippled            Same as for even dashes
/// Stippled                  Background masked by stipple
pub const XCB_GC_FILL_STYLE: xcb_gc_t = 256;
///
pub const XCB_GC_FILL_RULE: xcb_gc_t = 512;
/// The tile/stipple represents an infinite two-dimensional plane with the tile/stipple replicated in all
/// dimensions. When that plane is superimposed on the drawable for use in a graphics operation,
/// the upper-left corner of some instance of the tile/stipple is at the coordinates within the drawable
/// specified by the tile/stipple origin. The tile/stipple and clip origins are interpreted relative to the
/// origin of whatever destination drawable is specified in a graphics request.
/// The tile pixmap must have the same root and depth as the gcontext (or a Match error results).
/// The stipple pixmap must have depth one and must have the same root as the gcontext (or a
/// Match error results). For fill-style Stippled (but not fill-style
/// OpaqueStippled), the stipple pattern is tiled in a single plane and acts as an
/// additional clip mask to be ANDed with the clip-mask.
/// Any size pixmap can be used for tiling or stippling, although some sizes may be faster to use than
/// others.
pub const XCB_GC_TILE: xcb_gc_t = 1024;
/// The tile/stipple represents an infinite two-dimensional plane with the tile/stipple replicated in all
/// dimensions. When that plane is superimposed on the drawable for use in a graphics operation,
/// the upper-left corner of some instance of the tile/stipple is at the coordinates within the drawable
/// specified by the tile/stipple origin. The tile/stipple and clip origins are interpreted relative to the
/// origin of whatever destination drawable is specified in a graphics request.
/// The tile pixmap must have the same root and depth as the gcontext (or a Match error results).
/// The stipple pixmap must have depth one and must have the same root as the gcontext (or a
/// Match error results). For fill-style Stippled (but not fill-style
/// OpaqueStippled), the stipple pattern is tiled in a single plane and acts as an
/// additional clip mask to be ANDed with the clip-mask.
/// Any size pixmap can be used for tiling or stippling, although some sizes may be faster to use than
/// others.
pub const XCB_GC_STIPPLE: xcb_gc_t = 2048;
/// TODO
pub const XCB_GC_TILE_STIPPLE_ORIGIN_X: xcb_gc_t = 4096;
/// TODO
pub const XCB_GC_TILE_STIPPLE_ORIGIN_Y: xcb_gc_t = 8192;
/// Which font to use for the `ImageText8` and `ImageText16` requests.
pub const XCB_GC_FONT: xcb_gc_t = 16384;
/// For ClipByChildren, both source and destination windows are additionally
/// clipped by all viewable InputOutput children. For IncludeInferiors, neither
/// source nor destination window is
/// clipped by inferiors. This will result in including subwindow contents in the source and drawing
/// through subwindow boundaries of the destination. The use of IncludeInferiors with a source or
/// destination window of one depth with mapped inferiors of differing depth is not illegal, but the
/// semantics is undefined by the core protocol.
pub const XCB_GC_SUBWINDOW_MODE: xcb_gc_t = 32768;
/// Whether ExposureEvents should be generated (1) or not (0).
///
/// The default is 1.
pub const XCB_GC_GRAPHICS_EXPOSURES: xcb_gc_t = 65536;
/// TODO
pub const XCB_GC_CLIP_ORIGIN_X: xcb_gc_t = 131072;
/// TODO
pub const XCB_GC_CLIP_ORIGIN_Y: xcb_gc_t = 262144;
/// The clip-mask restricts writes to the destination drawable. Only pixels where the clip-mask has
/// bits set to 1 are drawn. Pixels are not drawn outside the area covered by the clip-mask or where
/// the clip-mask has bits set to 0. The clip-mask affects all graphics requests, but it does not clip
/// sources. The clip-mask origin is interpreted relative to the origin of whatever destination drawable is specified in a graphics request. If a pixmap is specified as the clip-mask, it must have
/// depth 1 and have the same root as the gcontext (or a Match error results). If clip-mask is None,
/// then pixels are always drawn, regardless of the clip origin. The clip-mask can also be set with the
/// SetClipRectangles request.
pub const XCB_GC_CLIP_MASK: xcb_gc_t = 524288;
/// TODO
pub const XCB_GC_DASH_OFFSET: xcb_gc_t = 1048576;
/// TODO
pub const XCB_GC_DASH_LIST: xcb_gc_t = 2097152;
/// TODO
pub const XCB_GC_ARC_MODE: xcb_gc_t = 4194304;

pub type xcb_gx_t = u32;
pub const XCB_GX_CLEAR: xcb_gx_t = 0;
pub const XCB_GX_AND: xcb_gx_t = 1;
pub const XCB_GX_AND_REVERSE: xcb_gx_t = 2;
pub const XCB_GX_COPY: xcb_gx_t = 3;
pub const XCB_GX_AND_INVERTED: xcb_gx_t = 4;
pub const XCB_GX_NOOP: xcb_gx_t = 5;
pub const XCB_GX_XOR: xcb_gx_t = 6;
pub const XCB_GX_OR: xcb_gx_t = 7;
pub const XCB_GX_NOR: xcb_gx_t = 8;
pub const XCB_GX_EQUIV: xcb_gx_t = 9;
pub const XCB_GX_INVERT: xcb_gx_t = 10;
pub const XCB_GX_OR_REVERSE: xcb_gx_t = 11;
pub const XCB_GX_COPY_INVERTED: xcb_gx_t = 12;
pub const XCB_GX_OR_INVERTED: xcb_gx_t = 13;
pub const XCB_GX_NAND: xcb_gx_t = 14;
pub const XCB_GX_SET: xcb_gx_t = 15;

pub type xcb_line_style_t = u32;
pub const XCB_LINE_STYLE_SOLID: xcb_line_style_t = 0;
pub const XCB_LINE_STYLE_ON_OFF_DASH: xcb_line_style_t = 1;
pub const XCB_LINE_STYLE_DOUBLE_DASH: xcb_line_style_t = 2;

pub type xcb_cap_style_t = u32;
pub const XCB_CAP_STYLE_NOT_LAST: xcb_cap_style_t = 0;
pub const XCB_CAP_STYLE_BUTT: xcb_cap_style_t = 1;
pub const XCB_CAP_STYLE_ROUND: xcb_cap_style_t = 2;
pub const XCB_CAP_STYLE_PROJECTING: xcb_cap_style_t = 3;

pub type xcb_join_style_t = u32;
pub const XCB_JOIN_STYLE_MITER: xcb_join_style_t = 0;
pub const XCB_JOIN_STYLE_ROUND: xcb_join_style_t = 1;
pub const XCB_JOIN_STYLE_BEVEL: xcb_join_style_t = 2;

pub type xcb_fill_style_t = u32;
pub const XCB_FILL_STYLE_SOLID: xcb_fill_style_t = 0;
pub const XCB_FILL_STYLE_TILED: xcb_fill_style_t = 1;
pub const XCB_FILL_STYLE_STIPPLED: xcb_fill_style_t = 2;
pub const XCB_FILL_STYLE_OPAQUE_STIPPLED: xcb_fill_style_t = 3;

pub type xcb_fill_rule_t = u32;
pub const XCB_FILL_RULE_EVEN_ODD: xcb_fill_rule_t = 0;
pub const XCB_FILL_RULE_WINDING: xcb_fill_rule_t = 1;

pub type xcb_subwindow_mode_t = u32;
pub const XCB_SUBWINDOW_MODE_CLIP_BY_CHILDREN: xcb_subwindow_mode_t = 0;
pub const XCB_SUBWINDOW_MODE_INCLUDE_INFERIORS: xcb_subwindow_mode_t = 1;

pub type xcb_arc_mode_t = u32;
pub const XCB_ARC_MODE_CHORD: xcb_arc_mode_t = 0;
pub const XCB_ARC_MODE_PIE_SLICE: xcb_arc_mode_t = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_create_gc_value_list_t {
    pub function: u32,
    pub plane_mask: u32,
    pub foreground: u32,
    pub background: u32,
    pub line_width: u32,
    pub line_style: u32,
    pub cap_style: u32,
    pub join_style: u32,
    pub fill_style: u32,
    pub fill_rule: u32,
    pub tile: xcb_pixmap_t,
    pub stipple: xcb_pixmap_t,
    pub tile_stipple_x_origin: i32,
    pub tile_stipple_y_origin: i32,
    pub font: xcb_font_t,
    pub subwindow_mode: u32,
    pub graphics_exposures: xcb_bool32_t,
    pub clip_x_origin: i32,
    pub clip_y_origin: i32,
    pub clip_mask: xcb_pixmap_t,
    pub dash_offset: u32,
    pub dashes: u32,
    pub arc_mode: u32,
}

impl Default for xcb_create_gc_value_list_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_create_gc.
pub const XCB_CREATE_GC: u8 = 55i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_create_gc_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cid: xcb_gcontext_t,
    pub drawable: xcb_drawable_t,
    pub value_mask: u32,
}

impl Default for xcb_create_gc_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_change_gc_value_list_t {
    pub function: u32,
    pub plane_mask: u32,
    pub foreground: u32,
    pub background: u32,
    pub line_width: u32,
    pub line_style: u32,
    pub cap_style: u32,
    pub join_style: u32,
    pub fill_style: u32,
    pub fill_rule: u32,
    pub tile: xcb_pixmap_t,
    pub stipple: xcb_pixmap_t,
    pub tile_stipple_x_origin: i32,
    pub tile_stipple_y_origin: i32,
    pub font: xcb_font_t,
    pub subwindow_mode: u32,
    pub graphics_exposures: xcb_bool32_t,
    pub clip_x_origin: i32,
    pub clip_y_origin: i32,
    pub clip_mask: xcb_pixmap_t,
    pub dash_offset: u32,
    pub dashes: u32,
    pub arc_mode: u32,
}

impl Default for xcb_change_gc_value_list_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_change_gc.
pub const XCB_CHANGE_GC: u8 = 56i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_change_gc_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub gc: xcb_gcontext_t,
    pub value_mask: u32,
}

impl Default for xcb_change_gc_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_copy_gc.
pub const XCB_COPY_GC: u8 = 57i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_copy_gc_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub src_gc: xcb_gcontext_t,
    pub dst_gc: xcb_gcontext_t,
    pub value_mask: u32,
}

impl Default for xcb_copy_gc_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_set_dashes.
pub const XCB_SET_DASHES: u8 = 58i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_dashes_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub gc: xcb_gcontext_t,
    pub dash_offset: u16,
    pub dashes_len: u16,
}

impl Default for xcb_set_dashes_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_clip_ordering_t = u32;
pub const XCB_CLIP_ORDERING_UNSORTED: xcb_clip_ordering_t = 0;
pub const XCB_CLIP_ORDERING_Y_SORTED: xcb_clip_ordering_t = 1;
pub const XCB_CLIP_ORDERING_YX_SORTED: xcb_clip_ordering_t = 2;
pub const XCB_CLIP_ORDERING_YX_BANDED: xcb_clip_ordering_t = 3;

/// Opcode for xcb_set_clip_rectangles.
pub const XCB_SET_CLIP_RECTANGLES: u8 = 59i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_clip_rectangles_request_t {
    pub major_opcode: u8,
    pub ordering: u8,
    pub length: u16,
    pub gc: xcb_gcontext_t,
    pub clip_x_origin: i16,
    pub clip_y_origin: i16,
}

impl Default for xcb_set_clip_rectangles_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_free_gc.
pub const XCB_FREE_GC: u8 = 60i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_free_gc_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub gc: xcb_gcontext_t,
}

impl Default for xcb_free_gc_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_clear_area.
pub const XCB_CLEAR_AREA: u8 = 61i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_clear_area_request_t {
    pub major_opcode: u8,
    pub exposures: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

impl Default for xcb_clear_area_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_copy_area.
pub const XCB_COPY_AREA: u8 = 62i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_copy_area_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub src_drawable: xcb_drawable_t,
    pub dst_drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub src_x: i16,
    pub src_y: i16,
    pub dst_x: i16,
    pub dst_y: i16,
    pub width: u16,
    pub height: u16,
}

impl Default for xcb_copy_area_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_copy_plane.
pub const XCB_COPY_PLANE: u8 = 63i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_copy_plane_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub src_drawable: xcb_drawable_t,
    pub dst_drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub src_x: i16,
    pub src_y: i16,
    pub dst_x: i16,
    pub dst_y: i16,
    pub width: u16,
    pub height: u16,
    pub bit_plane: u32,
}

impl Default for xcb_copy_plane_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_coord_mode_t = u32;
/// Treats all coordinates as relative to the origin.
pub const XCB_COORD_MODE_ORIGIN: xcb_coord_mode_t = 0;
/// Treats all coordinates after the first as relative to the previous coordinate.
pub const XCB_COORD_MODE_PREVIOUS: xcb_coord_mode_t = 1;

/// Opcode for xcb_poly_point.
pub const XCB_POLY_POINT: u8 = 64i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_poly_point_request_t {
    pub major_opcode: u8,
    pub coordinate_mode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}

impl Default for xcb_poly_point_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_poly_line.
pub const XCB_POLY_LINE: u8 = 65i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_poly_line_request_t {
    pub major_opcode: u8,
    pub coordinate_mode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}

impl Default for xcb_poly_line_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_segment_t {
    pub x1: i16,
    pub y1: i16,
    pub x2: i16,
    pub y2: i16,
}

impl Default for xcb_segment_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_segment_iterator_t {
    pub data: *mut xcb_segment_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_segment_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_poly_segment.
pub const XCB_POLY_SEGMENT: u8 = 66i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_poly_segment_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}

impl Default for xcb_poly_segment_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_poly_rectangle.
pub const XCB_POLY_RECTANGLE: u8 = 67i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_poly_rectangle_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}

impl Default for xcb_poly_rectangle_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_poly_arc.
pub const XCB_POLY_ARC: u8 = 68i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_poly_arc_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}

impl Default for xcb_poly_arc_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_poly_shape_t = u32;
pub const XCB_POLY_SHAPE_COMPLEX: xcb_poly_shape_t = 0;
pub const XCB_POLY_SHAPE_NONCONVEX: xcb_poly_shape_t = 1;
pub const XCB_POLY_SHAPE_CONVEX: xcb_poly_shape_t = 2;

/// Opcode for xcb_fill_poly.
pub const XCB_FILL_POLY: u8 = 69i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_fill_poly_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub shape: u8,
    pub coordinate_mode: u8,
    pub pad1: [u8; 2],
}

impl Default for xcb_fill_poly_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_poly_fill_rectangle.
pub const XCB_POLY_FILL_RECTANGLE: u8 = 70i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_poly_fill_rectangle_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}

impl Default for xcb_poly_fill_rectangle_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_poly_fill_arc.
pub const XCB_POLY_FILL_ARC: u8 = 71i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_poly_fill_arc_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}

impl Default for xcb_poly_fill_arc_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_image_format_t = u32;
pub const XCB_IMAGE_FORMAT_XY_BITMAP: xcb_image_format_t = 0;
pub const XCB_IMAGE_FORMAT_XY_PIXMAP: xcb_image_format_t = 1;
pub const XCB_IMAGE_FORMAT_Z_PIXMAP: xcb_image_format_t = 2;

/// Opcode for xcb_put_image.
pub const XCB_PUT_IMAGE: u8 = 72i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_put_image_request_t {
    pub major_opcode: u8,
    pub format: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub width: u16,
    pub height: u16,
    pub dst_x: i16,
    pub dst_y: i16,
    pub left_pad: u8,
    pub depth: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_put_image_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_image_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_get_image_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_get_image.
pub const XCB_GET_IMAGE: u8 = 73i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_image_request_t {
    pub major_opcode: u8,
    pub format: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub plane_mask: u32,
}

impl Default for xcb_get_image_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_image_reply_t {
    pub response_type: u8,
    pub depth: u8,
    pub sequence: u16,
    pub length: u32,
    pub visual: xcb_visualid_t,
    pub pad0: [u8; 20],
}

impl Default for xcb_get_image_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_poly_text_8.
pub const XCB_POLY_TEXT_8: u8 = 74i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_poly_text_8_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub x: i16,
    pub y: i16,
}

impl Default for xcb_poly_text_8_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_poly_text_16.
pub const XCB_POLY_TEXT_16: u8 = 75i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_poly_text_16_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub x: i16,
    pub y: i16,
}

impl Default for xcb_poly_text_16_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_image_text_8.
pub const XCB_IMAGE_TEXT_8: u8 = 76i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_image_text_8_request_t {
    pub major_opcode: u8,
    pub string_len: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub x: i16,
    pub y: i16,
}

impl Default for xcb_image_text_8_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_image_text_16.
pub const XCB_IMAGE_TEXT_16: u8 = 77i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_image_text_16_request_t {
    pub major_opcode: u8,
    pub string_len: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub x: i16,
    pub y: i16,
}

impl Default for xcb_image_text_16_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_colormap_alloc_t = u32;
pub const XCB_COLORMAP_ALLOC_NONE: xcb_colormap_alloc_t = 0;
pub const XCB_COLORMAP_ALLOC_ALL: xcb_colormap_alloc_t = 1;

/// Opcode for xcb_create_colormap.
pub const XCB_CREATE_COLORMAP: u8 = 78i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_create_colormap_request_t {
    pub major_opcode: u8,
    pub alloc: u8,
    pub length: u16,
    pub mid: xcb_colormap_t,
    pub window: xcb_window_t,
    pub visual: xcb_visualid_t,
}

impl Default for xcb_create_colormap_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_free_colormap.
pub const XCB_FREE_COLORMAP: u8 = 79i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_free_colormap_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
}

impl Default for xcb_free_colormap_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_copy_colormap_and_free.
pub const XCB_COPY_COLORMAP_AND_FREE: u8 = 80i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_copy_colormap_and_free_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub mid: xcb_colormap_t,
    pub src_cmap: xcb_colormap_t,
}

impl Default for xcb_copy_colormap_and_free_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_install_colormap.
pub const XCB_INSTALL_COLORMAP: u8 = 81i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_install_colormap_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
}

impl Default for xcb_install_colormap_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_uninstall_colormap.
pub const XCB_UNINSTALL_COLORMAP: u8 = 82i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_uninstall_colormap_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
}

impl Default for xcb_uninstall_colormap_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_installed_colormaps_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_list_installed_colormaps_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_list_installed_colormaps.
pub const XCB_LIST_INSTALLED_COLORMAPS: u8 = 83i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_installed_colormaps_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_list_installed_colormaps_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_installed_colormaps_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub cmaps_len: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_list_installed_colormaps_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_alloc_color_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_alloc_color_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_alloc_color.
pub const XCB_ALLOC_COLOR: u8 = 84i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_alloc_color_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub pad1: [u8; 2],
}

impl Default for xcb_alloc_color_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_alloc_color_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub pad1: [u8; 2],
    pub pixel: u32,
}

impl Default for xcb_alloc_color_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_alloc_named_color_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_alloc_named_color_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_alloc_named_color.
pub const XCB_ALLOC_NAMED_COLOR: u8 = 85i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_alloc_named_color_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
    pub name_len: u16,
    pub pad1: [u8; 2],
}

impl Default for xcb_alloc_named_color_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_alloc_named_color_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pixel: u32,
    pub exact_red: u16,
    pub exact_green: u16,
    pub exact_blue: u16,
    pub visual_red: u16,
    pub visual_green: u16,
    pub visual_blue: u16,
}

impl Default for xcb_alloc_named_color_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_alloc_color_cells_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_alloc_color_cells_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_alloc_color_cells.
pub const XCB_ALLOC_COLOR_CELLS: u8 = 86i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_alloc_color_cells_request_t {
    pub major_opcode: u8,
    pub contiguous: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
    pub colors: u16,
    pub planes: u16,
}

impl Default for xcb_alloc_color_cells_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_alloc_color_cells_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pixels_len: u16,
    pub masks_len: u16,
    pub pad1: [u8; 20],
}

impl Default for xcb_alloc_color_cells_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_alloc_color_planes_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_alloc_color_planes_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_alloc_color_planes.
pub const XCB_ALLOC_COLOR_PLANES: u8 = 87i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_alloc_color_planes_request_t {
    pub major_opcode: u8,
    pub contiguous: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
    pub colors: u16,
    pub reds: u16,
    pub greens: u16,
    pub blues: u16,
}

impl Default for xcb_alloc_color_planes_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_alloc_color_planes_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pixels_len: u16,
    pub pad1: [u8; 2],
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub pad2: [u8; 8],
}

impl Default for xcb_alloc_color_planes_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_free_colors.
pub const XCB_FREE_COLORS: u8 = 88i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_free_colors_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
    pub plane_mask: u32,
}

impl Default for xcb_free_colors_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_color_flag_t = u32;
pub const XCB_COLOR_FLAG_RED: xcb_color_flag_t = 1;
pub const XCB_COLOR_FLAG_GREEN: xcb_color_flag_t = 2;
pub const XCB_COLOR_FLAG_BLUE: xcb_color_flag_t = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_coloritem_t {
    pub pixel: u32,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub flags: u8,
    pub pad0: u8,
}

impl Default for xcb_coloritem_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_coloritem_iterator_t {
    pub data: *mut xcb_coloritem_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_coloritem_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_store_colors.
pub const XCB_STORE_COLORS: u8 = 89i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_store_colors_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
}

impl Default for xcb_store_colors_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_store_named_color.
pub const XCB_STORE_NAMED_COLOR: u8 = 90i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_store_named_color_request_t {
    pub major_opcode: u8,
    pub flags: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
    pub pixel: u32,
    pub name_len: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_store_named_color_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_rgb_t {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_rgb_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_rgb_iterator_t {
    pub data: *mut xcb_rgb_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_rgb_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_colors_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_query_colors_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_query_colors.
pub const XCB_QUERY_COLORS: u8 = 91i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_colors_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
}

impl Default for xcb_query_colors_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_colors_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub colors_len: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_query_colors_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_lookup_color_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_lookup_color_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_lookup_color.
pub const XCB_LOOKUP_COLOR: u8 = 92i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_lookup_color_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
    pub name_len: u16,
    pub pad1: [u8; 2],
}

impl Default for xcb_lookup_color_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_lookup_color_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub exact_red: u16,
    pub exact_green: u16,
    pub exact_blue: u16,
    pub visual_red: u16,
    pub visual_green: u16,
    pub visual_blue: u16,
}

impl Default for xcb_lookup_color_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_pixmap_enum_t = u32;
pub const XCB_PIXMAP_NONE: xcb_pixmap_enum_t = 0;

/// Opcode for xcb_create_cursor.
pub const XCB_CREATE_CURSOR: u8 = 93i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_create_cursor_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cid: xcb_cursor_t,
    pub source: xcb_pixmap_t,
    pub mask: xcb_pixmap_t,
    pub fore_red: u16,
    pub fore_green: u16,
    pub fore_blue: u16,
    pub back_red: u16,
    pub back_green: u16,
    pub back_blue: u16,
    pub x: u16,
    pub y: u16,
}

impl Default for xcb_create_cursor_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_font_enum_t = u32;
pub const XCB_FONT_NONE: xcb_font_enum_t = 0;

/// Opcode for xcb_create_glyph_cursor.
pub const XCB_CREATE_GLYPH_CURSOR: u8 = 94i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_create_glyph_cursor_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cid: xcb_cursor_t,
    pub source_font: xcb_font_t,
    pub mask_font: xcb_font_t,
    pub source_char: u16,
    pub mask_char: u16,
    pub fore_red: u16,
    pub fore_green: u16,
    pub fore_blue: u16,
    pub back_red: u16,
    pub back_green: u16,
    pub back_blue: u16,
}

impl Default for xcb_create_glyph_cursor_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_free_cursor.
pub const XCB_FREE_CURSOR: u8 = 95i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_free_cursor_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cursor: xcb_cursor_t,
}

impl Default for xcb_free_cursor_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_recolor_cursor.
pub const XCB_RECOLOR_CURSOR: u8 = 96i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_recolor_cursor_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cursor: xcb_cursor_t,
    pub fore_red: u16,
    pub fore_green: u16,
    pub fore_blue: u16,
    pub back_red: u16,
    pub back_green: u16,
    pub back_blue: u16,
}

impl Default for xcb_recolor_cursor_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_query_shape_of_t = u32;
pub const XCB_QUERY_SHAPE_OF_LARGEST_CURSOR: xcb_query_shape_of_t = 0;
pub const XCB_QUERY_SHAPE_OF_FASTEST_TILE: xcb_query_shape_of_t = 1;
pub const XCB_QUERY_SHAPE_OF_FASTEST_STIPPLE: xcb_query_shape_of_t = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_best_size_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_query_best_size_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_query_best_size.
pub const XCB_QUERY_BEST_SIZE: u8 = 97i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_best_size_request_t {
    pub major_opcode: u8,
    pub class: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub width: u16,
    pub height: u16,
}

impl Default for xcb_query_best_size_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_best_size_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: u16,
    pub height: u16,
}

impl Default for xcb_query_best_size_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_extension_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_query_extension_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_query_extension.
pub const XCB_QUERY_EXTENSION: u8 = 98i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_extension_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub name_len: u16,
    pub pad1: [u8; 2],
}

impl Default for xcb_query_extension_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_extension_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub present: u8,
    pub major_opcode: u8,
    pub first_event: u8,
    pub first_error: u8,
}

impl Default for xcb_query_extension_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_extensions_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_list_extensions_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_list_extensions.
pub const XCB_LIST_EXTENSIONS: u8 = 99i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_extensions_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_list_extensions_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_extensions_reply_t {
    pub response_type: u8,
    pub names_len: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad0: [u8; 24],
}

impl Default for xcb_list_extensions_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_change_keyboard_mapping.
pub const XCB_CHANGE_KEYBOARD_MAPPING: u8 = 100i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_change_keyboard_mapping_request_t {
    pub major_opcode: u8,
    pub keycode_count: u8,
    pub length: u16,
    pub first_keycode: xcb_keycode_t,
    pub keysyms_per_keycode: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_change_keyboard_mapping_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_keyboard_mapping_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_get_keyboard_mapping_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_get_keyboard_mapping.
pub const XCB_GET_KEYBOARD_MAPPING: u8 = 101i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_keyboard_mapping_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub first_keycode: xcb_keycode_t,
    pub count: u8,
}

impl Default for xcb_get_keyboard_mapping_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_keyboard_mapping_reply_t {
    pub response_type: u8,
    pub keysyms_per_keycode: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad0: [u8; 24],
}

impl Default for xcb_get_keyboard_mapping_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_kb_t = u32;
pub const XCB_KB_KEY_CLICK_PERCENT: xcb_kb_t = 1;
pub const XCB_KB_BELL_PERCENT: xcb_kb_t = 2;
pub const XCB_KB_BELL_PITCH: xcb_kb_t = 4;
pub const XCB_KB_BELL_DURATION: xcb_kb_t = 8;
pub const XCB_KB_LED: xcb_kb_t = 16;
pub const XCB_KB_LED_MODE: xcb_kb_t = 32;
pub const XCB_KB_KEY: xcb_kb_t = 64;
pub const XCB_KB_AUTO_REPEAT_MODE: xcb_kb_t = 128;

pub type xcb_led_mode_t = u32;
pub const XCB_LED_MODE_OFF: xcb_led_mode_t = 0;
pub const XCB_LED_MODE_ON: xcb_led_mode_t = 1;

pub type xcb_auto_repeat_mode_t = u32;
pub const XCB_AUTO_REPEAT_MODE_OFF: xcb_auto_repeat_mode_t = 0;
pub const XCB_AUTO_REPEAT_MODE_ON: xcb_auto_repeat_mode_t = 1;
pub const XCB_AUTO_REPEAT_MODE_DEFAULT: xcb_auto_repeat_mode_t = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_change_keyboard_control_value_list_t {
    pub key_click_percent: i32,
    pub bell_percent: i32,
    pub bell_pitch: i32,
    pub bell_duration: i32,
    pub led: u32,
    pub led_mode: u32,
    pub key: xcb_keycode32_t,
    pub auto_repeat_mode: u32,
}

impl Default for xcb_change_keyboard_control_value_list_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_change_keyboard_control.
pub const XCB_CHANGE_KEYBOARD_CONTROL: u8 = 102i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_change_keyboard_control_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub value_mask: u32,
}

impl Default for xcb_change_keyboard_control_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_keyboard_control_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_get_keyboard_control_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_get_keyboard_control.
pub const XCB_GET_KEYBOARD_CONTROL: u8 = 103i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_keyboard_control_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_get_keyboard_control_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_keyboard_control_reply_t {
    pub response_type: u8,
    pub global_auto_repeat: u8,
    pub sequence: u16,
    pub length: u32,
    pub led_mask: u32,
    pub key_click_percent: u8,
    pub bell_percent: u8,
    pub bell_pitch: u16,
    pub bell_duration: u16,
    pub pad0: [u8; 2],
    pub auto_repeats: [u8; 32],
}

impl Default for xcb_get_keyboard_control_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_bell.
pub const XCB_BELL: u8 = 104i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_bell_request_t {
    pub major_opcode: u8,
    pub percent: i8,
    pub length: u16,
}

impl Default for xcb_bell_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_change_pointer_control.
pub const XCB_CHANGE_POINTER_CONTROL: u8 = 105i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_change_pointer_control_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub acceleration_numerator: i16,
    pub acceleration_denominator: i16,
    pub threshold: i16,
    pub do_acceleration: u8,
    pub do_threshold: u8,
}

impl Default for xcb_change_pointer_control_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_pointer_control_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_get_pointer_control_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_get_pointer_control.
pub const XCB_GET_POINTER_CONTROL: u8 = 106i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_pointer_control_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_get_pointer_control_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_pointer_control_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub acceleration_numerator: u16,
    pub acceleration_denominator: u16,
    pub threshold: u16,
    pub pad1: [u8; 18],
}

impl Default for xcb_get_pointer_control_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_blanking_t = u32;
pub const XCB_BLANKING_NOT_PREFERRED: xcb_blanking_t = 0;
pub const XCB_BLANKING_PREFERRED: xcb_blanking_t = 1;
pub const XCB_BLANKING_DEFAULT: xcb_blanking_t = 2;

pub type xcb_exposures_t = u32;
pub const XCB_EXPOSURES_NOT_ALLOWED: xcb_exposures_t = 0;
pub const XCB_EXPOSURES_ALLOWED: xcb_exposures_t = 1;
pub const XCB_EXPOSURES_DEFAULT: xcb_exposures_t = 2;

/// Opcode for xcb_set_screen_saver.
pub const XCB_SET_SCREEN_SAVER: u8 = 107i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_screen_saver_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub timeout: i16,
    pub interval: i16,
    pub prefer_blanking: u8,
    pub allow_exposures: u8,
}

impl Default for xcb_set_screen_saver_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_screen_saver_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_get_screen_saver_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_get_screen_saver.
pub const XCB_GET_SCREEN_SAVER: u8 = 108i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_screen_saver_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_get_screen_saver_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_screen_saver_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub timeout: u16,
    pub interval: u16,
    pub prefer_blanking: u8,
    pub allow_exposures: u8,
    pub pad1: [u8; 18],
}

impl Default for xcb_get_screen_saver_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_host_mode_t = u32;
pub const XCB_HOST_MODE_INSERT: xcb_host_mode_t = 0;
pub const XCB_HOST_MODE_DELETE: xcb_host_mode_t = 1;

pub type xcb_family_t = u32;
pub const XCB_FAMILY_INTERNET: xcb_family_t = 0;
pub const XCB_FAMILY_DECNET: xcb_family_t = 1;
pub const XCB_FAMILY_CHAOS: xcb_family_t = 2;
pub const XCB_FAMILY_SERVER_INTERPRETED: xcb_family_t = 5;
pub const XCB_FAMILY_INTERNET_6: xcb_family_t = 6;

/// Opcode for xcb_change_hosts.
pub const XCB_CHANGE_HOSTS: u8 = 109i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_change_hosts_request_t {
    pub major_opcode: u8,
    pub mode: u8,
    pub length: u16,
    pub family: u8,
    pub pad0: u8,
    pub address_len: u16,
}

impl Default for xcb_change_hosts_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_host_t {
    pub family: u8,
    pub pad0: u8,
    pub address_len: u16,
}

impl Default for xcb_host_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_host_iterator_t {
    pub data: *mut xcb_host_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_host_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_hosts_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_list_hosts_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_list_hosts.
pub const XCB_LIST_HOSTS: u8 = 110i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_hosts_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_list_hosts_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_hosts_reply_t {
    pub response_type: u8,
    pub mode: u8,
    pub sequence: u16,
    pub length: u32,
    pub hosts_len: u16,
    pub pad0: [u8; 22],
}

impl Default for xcb_list_hosts_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_access_control_t = u32;
pub const XCB_ACCESS_CONTROL_DISABLE: xcb_access_control_t = 0;
pub const XCB_ACCESS_CONTROL_ENABLE: xcb_access_control_t = 1;

/// Opcode for xcb_set_access_control.
pub const XCB_SET_ACCESS_CONTROL: u8 = 111i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_access_control_request_t {
    pub major_opcode: u8,
    pub mode: u8,
    pub length: u16,
}

impl Default for xcb_set_access_control_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_close_down_t = u32;
pub const XCB_CLOSE_DOWN_DESTROY_ALL: xcb_close_down_t = 0;
pub const XCB_CLOSE_DOWN_RETAIN_PERMANENT: xcb_close_down_t = 1;
pub const XCB_CLOSE_DOWN_RETAIN_TEMPORARY: xcb_close_down_t = 2;

/// Opcode for xcb_set_close_down_mode.
pub const XCB_SET_CLOSE_DOWN_MODE: u8 = 112i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_close_down_mode_request_t {
    pub major_opcode: u8,
    pub mode: u8,
    pub length: u16,
}

impl Default for xcb_set_close_down_mode_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_kill_t = u32;
pub const XCB_KILL_ALL_TEMPORARY: xcb_kill_t = 0;

/// Opcode for xcb_kill_client.
pub const XCB_KILL_CLIENT: u8 = 113i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_kill_client_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub resource: u32,
}

impl Default for xcb_kill_client_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_rotate_properties.
pub const XCB_ROTATE_PROPERTIES: u8 = 114i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_rotate_properties_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub atoms_len: u16,
    pub delta: i16,
}

impl Default for xcb_rotate_properties_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_screen_saver_t = u32;
pub const XCB_SCREEN_SAVER_RESET: xcb_screen_saver_t = 0;
pub const XCB_SCREEN_SAVER_ACTIVE: xcb_screen_saver_t = 1;

/// Opcode for xcb_force_screen_saver.
pub const XCB_FORCE_SCREEN_SAVER: u8 = 115i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_force_screen_saver_request_t {
    pub major_opcode: u8,
    pub mode: u8,
    pub length: u16,
}

impl Default for xcb_force_screen_saver_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_mapping_status_t = u32;
pub const XCB_MAPPING_STATUS_SUCCESS: xcb_mapping_status_t = 0;
pub const XCB_MAPPING_STATUS_BUSY: xcb_mapping_status_t = 1;
pub const XCB_MAPPING_STATUS_FAILURE: xcb_mapping_status_t = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_pointer_mapping_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_set_pointer_mapping_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_set_pointer_mapping.
pub const XCB_SET_POINTER_MAPPING: u8 = 116i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_pointer_mapping_request_t {
    pub major_opcode: u8,
    pub map_len: u8,
    pub length: u16,
}

impl Default for xcb_set_pointer_mapping_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_pointer_mapping_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
}

impl Default for xcb_set_pointer_mapping_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_pointer_mapping_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_get_pointer_mapping_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_get_pointer_mapping.
pub const XCB_GET_POINTER_MAPPING: u8 = 117i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_pointer_mapping_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_get_pointer_mapping_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_pointer_mapping_reply_t {
    pub response_type: u8,
    pub map_len: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad0: [u8; 24],
}

impl Default for xcb_get_pointer_mapping_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_map_index_t = u32;
pub const XCB_MAP_INDEX_SHIFT: xcb_map_index_t = 0;
pub const XCB_MAP_INDEX_LOCK: xcb_map_index_t = 1;
pub const XCB_MAP_INDEX_CONTROL: xcb_map_index_t = 2;
pub const XCB_MAP_INDEX_1: xcb_map_index_t = 3;
pub const XCB_MAP_INDEX_2: xcb_map_index_t = 4;
pub const XCB_MAP_INDEX_3: xcb_map_index_t = 5;
pub const XCB_MAP_INDEX_4: xcb_map_index_t = 6;
pub const XCB_MAP_INDEX_5: xcb_map_index_t = 7;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_modifier_mapping_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_set_modifier_mapping_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_set_modifier_mapping.
pub const XCB_SET_MODIFIER_MAPPING: u8 = 118i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_modifier_mapping_request_t {
    pub major_opcode: u8,
    pub keycodes_per_modifier: u8,
    pub length: u16,
}

impl Default for xcb_set_modifier_mapping_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_modifier_mapping_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
}

impl Default for xcb_set_modifier_mapping_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_modifier_mapping_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_get_modifier_mapping_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_get_modifier_mapping.
pub const XCB_GET_MODIFIER_MAPPING: u8 = 119i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_modifier_mapping_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_get_modifier_mapping_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_modifier_mapping_reply_t {
    pub response_type: u8,
    pub keycodes_per_modifier: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad0: [u8; 24],
}

impl Default for xcb_get_modifier_mapping_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_no_operation.
pub const XCB_NO_OPERATION: u8 = 127i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_no_operation_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_no_operation_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub(crate) struct XcbXproto {
    xcb_char2b_next: LazySymbol<unsafe fn(i: *mut xcb_char2b_iterator_t)>,
    xcb_char2b_end: LazySymbol<unsafe fn(i: xcb_char2b_iterator_t) -> xcb_generic_iterator_t>,
    xcb_window_next: LazySymbol<unsafe fn(i: *mut xcb_window_iterator_t)>,
    xcb_window_end: LazySymbol<unsafe fn(i: xcb_window_iterator_t) -> xcb_generic_iterator_t>,
    xcb_pixmap_next: LazySymbol<unsafe fn(i: *mut xcb_pixmap_iterator_t)>,
    xcb_pixmap_end: LazySymbol<unsafe fn(i: xcb_pixmap_iterator_t) -> xcb_generic_iterator_t>,
    xcb_cursor_next: LazySymbol<unsafe fn(i: *mut xcb_cursor_iterator_t)>,
    xcb_cursor_end: LazySymbol<unsafe fn(i: xcb_cursor_iterator_t) -> xcb_generic_iterator_t>,
    xcb_font_next: LazySymbol<unsafe fn(i: *mut xcb_font_iterator_t)>,
    xcb_font_end: LazySymbol<unsafe fn(i: xcb_font_iterator_t) -> xcb_generic_iterator_t>,
    xcb_gcontext_next: LazySymbol<unsafe fn(i: *mut xcb_gcontext_iterator_t)>,
    xcb_gcontext_end: LazySymbol<unsafe fn(i: xcb_gcontext_iterator_t) -> xcb_generic_iterator_t>,
    xcb_colormap_next: LazySymbol<unsafe fn(i: *mut xcb_colormap_iterator_t)>,
    xcb_colormap_end: LazySymbol<unsafe fn(i: xcb_colormap_iterator_t) -> xcb_generic_iterator_t>,
    xcb_atom_next: LazySymbol<unsafe fn(i: *mut xcb_atom_iterator_t)>,
    xcb_atom_end: LazySymbol<unsafe fn(i: xcb_atom_iterator_t) -> xcb_generic_iterator_t>,
    xcb_drawable_next: LazySymbol<unsafe fn(i: *mut xcb_drawable_iterator_t)>,
    xcb_drawable_end: LazySymbol<unsafe fn(i: xcb_drawable_iterator_t) -> xcb_generic_iterator_t>,
    xcb_fontable_next: LazySymbol<unsafe fn(i: *mut xcb_fontable_iterator_t)>,
    xcb_fontable_end: LazySymbol<unsafe fn(i: xcb_fontable_iterator_t) -> xcb_generic_iterator_t>,
    xcb_bool32_next: LazySymbol<unsafe fn(i: *mut xcb_bool32_iterator_t)>,
    xcb_bool32_end: LazySymbol<unsafe fn(i: xcb_bool32_iterator_t) -> xcb_generic_iterator_t>,
    xcb_visualid_next: LazySymbol<unsafe fn(i: *mut xcb_visualid_iterator_t)>,
    xcb_visualid_end: LazySymbol<unsafe fn(i: xcb_visualid_iterator_t) -> xcb_generic_iterator_t>,
    xcb_timestamp_next: LazySymbol<unsafe fn(i: *mut xcb_timestamp_iterator_t)>,
    xcb_timestamp_end: LazySymbol<unsafe fn(i: xcb_timestamp_iterator_t) -> xcb_generic_iterator_t>,
    xcb_keysym_next: LazySymbol<unsafe fn(i: *mut xcb_keysym_iterator_t)>,
    xcb_keysym_end: LazySymbol<unsafe fn(i: xcb_keysym_iterator_t) -> xcb_generic_iterator_t>,
    xcb_keycode_next: LazySymbol<unsafe fn(i: *mut xcb_keycode_iterator_t)>,
    xcb_keycode_end: LazySymbol<unsafe fn(i: xcb_keycode_iterator_t) -> xcb_generic_iterator_t>,
    xcb_keycode32_next: LazySymbol<unsafe fn(i: *mut xcb_keycode32_iterator_t)>,
    xcb_keycode32_end: LazySymbol<unsafe fn(i: xcb_keycode32_iterator_t) -> xcb_generic_iterator_t>,
    xcb_button_next: LazySymbol<unsafe fn(i: *mut xcb_button_iterator_t)>,
    xcb_button_end: LazySymbol<unsafe fn(i: xcb_button_iterator_t) -> xcb_generic_iterator_t>,
    xcb_point_next: LazySymbol<unsafe fn(i: *mut xcb_point_iterator_t)>,
    xcb_point_end: LazySymbol<unsafe fn(i: xcb_point_iterator_t) -> xcb_generic_iterator_t>,
    xcb_rectangle_next: LazySymbol<unsafe fn(i: *mut xcb_rectangle_iterator_t)>,
    xcb_rectangle_end: LazySymbol<unsafe fn(i: xcb_rectangle_iterator_t) -> xcb_generic_iterator_t>,
    xcb_arc_next: LazySymbol<unsafe fn(i: *mut xcb_arc_iterator_t)>,
    xcb_arc_end: LazySymbol<unsafe fn(i: xcb_arc_iterator_t) -> xcb_generic_iterator_t>,
    xcb_format_next: LazySymbol<unsafe fn(i: *mut xcb_format_iterator_t)>,
    xcb_format_end: LazySymbol<unsafe fn(i: xcb_format_iterator_t) -> xcb_generic_iterator_t>,
    xcb_visualtype_next: LazySymbol<unsafe fn(i: *mut xcb_visualtype_iterator_t)>,
    xcb_visualtype_end:
        LazySymbol<unsafe fn(i: xcb_visualtype_iterator_t) -> xcb_generic_iterator_t>,
    xcb_depth_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_depth_visuals: LazySymbol<unsafe fn(r: *const xcb_depth_t) -> *mut xcb_visualtype_t>,
    xcb_depth_visuals_length: LazySymbol<unsafe fn(r: *const xcb_depth_t) -> c_int>,
    xcb_depth_visuals_iterator:
        LazySymbol<unsafe fn(r: *const xcb_depth_t) -> xcb_visualtype_iterator_t>,
    xcb_depth_next: LazySymbol<unsafe fn(i: *mut xcb_depth_iterator_t)>,
    xcb_depth_end: LazySymbol<unsafe fn(i: xcb_depth_iterator_t) -> xcb_generic_iterator_t>,
    xcb_screen_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_screen_allowed_depths_length: LazySymbol<unsafe fn(r: *const xcb_screen_t) -> c_int>,
    xcb_screen_allowed_depths_iterator:
        LazySymbol<unsafe fn(r: *const xcb_screen_t) -> xcb_depth_iterator_t>,
    xcb_screen_next: LazySymbol<unsafe fn(i: *mut xcb_screen_iterator_t)>,
    xcb_screen_end: LazySymbol<unsafe fn(i: xcb_screen_iterator_t) -> xcb_generic_iterator_t>,
    xcb_setup_request_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_setup_request_authorization_protocol_name:
        LazySymbol<unsafe fn(r: *const xcb_setup_request_t) -> *mut c_char>,
    xcb_setup_request_authorization_protocol_name_length:
        LazySymbol<unsafe fn(r: *const xcb_setup_request_t) -> c_int>,
    xcb_setup_request_authorization_protocol_name_end:
        LazySymbol<unsafe fn(r: *const xcb_setup_request_t) -> xcb_generic_iterator_t>,
    xcb_setup_request_authorization_protocol_data:
        LazySymbol<unsafe fn(r: *const xcb_setup_request_t) -> *mut c_char>,
    xcb_setup_request_authorization_protocol_data_length:
        LazySymbol<unsafe fn(r: *const xcb_setup_request_t) -> c_int>,
    xcb_setup_request_authorization_protocol_data_end:
        LazySymbol<unsafe fn(r: *const xcb_setup_request_t) -> xcb_generic_iterator_t>,
    xcb_setup_request_next: LazySymbol<unsafe fn(i: *mut xcb_setup_request_iterator_t)>,
    xcb_setup_request_end:
        LazySymbol<unsafe fn(i: xcb_setup_request_iterator_t) -> xcb_generic_iterator_t>,
    xcb_setup_failed_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_setup_failed_reason: LazySymbol<unsafe fn(r: *const xcb_setup_failed_t) -> *mut c_char>,
    xcb_setup_failed_reason_length: LazySymbol<unsafe fn(r: *const xcb_setup_failed_t) -> c_int>,
    xcb_setup_failed_reason_end:
        LazySymbol<unsafe fn(r: *const xcb_setup_failed_t) -> xcb_generic_iterator_t>,
    xcb_setup_failed_next: LazySymbol<unsafe fn(i: *mut xcb_setup_failed_iterator_t)>,
    xcb_setup_failed_end:
        LazySymbol<unsafe fn(i: xcb_setup_failed_iterator_t) -> xcb_generic_iterator_t>,
    xcb_setup_authenticate_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_setup_authenticate_reason:
        LazySymbol<unsafe fn(r: *const xcb_setup_authenticate_t) -> *mut c_char>,
    xcb_setup_authenticate_reason_length:
        LazySymbol<unsafe fn(r: *const xcb_setup_authenticate_t) -> c_int>,
    xcb_setup_authenticate_reason_end:
        LazySymbol<unsafe fn(r: *const xcb_setup_authenticate_t) -> xcb_generic_iterator_t>,
    xcb_setup_authenticate_next: LazySymbol<unsafe fn(i: *mut xcb_setup_authenticate_iterator_t)>,
    xcb_setup_authenticate_end:
        LazySymbol<unsafe fn(i: xcb_setup_authenticate_iterator_t) -> xcb_generic_iterator_t>,
    xcb_setup_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_setup_vendor: LazySymbol<unsafe fn(r: *const xcb_setup_t) -> *mut c_char>,
    xcb_setup_vendor_length: LazySymbol<unsafe fn(r: *const xcb_setup_t) -> c_int>,
    xcb_setup_vendor_end: LazySymbol<unsafe fn(r: *const xcb_setup_t) -> xcb_generic_iterator_t>,
    xcb_setup_pixmap_formats: LazySymbol<unsafe fn(r: *const xcb_setup_t) -> *mut xcb_format_t>,
    xcb_setup_pixmap_formats_length: LazySymbol<unsafe fn(r: *const xcb_setup_t) -> c_int>,
    xcb_setup_pixmap_formats_iterator:
        LazySymbol<unsafe fn(r: *const xcb_setup_t) -> xcb_format_iterator_t>,
    xcb_setup_roots_length: LazySymbol<unsafe fn(r: *const xcb_setup_t) -> c_int>,
    xcb_setup_roots_iterator: LazySymbol<unsafe fn(r: *const xcb_setup_t) -> xcb_screen_iterator_t>,
    xcb_setup_next: LazySymbol<unsafe fn(i: *mut xcb_setup_iterator_t)>,
    xcb_setup_end: LazySymbol<unsafe fn(i: xcb_setup_iterator_t) -> xcb_generic_iterator_t>,
    xcb_client_message_data_next: LazySymbol<unsafe fn(i: *mut xcb_client_message_data_iterator_t)>,
    xcb_client_message_data_end:
        LazySymbol<unsafe fn(i: xcb_client_message_data_iterator_t) -> xcb_generic_iterator_t>,
    xcb_create_window_value_list_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            value_mask: u32,
            _aux: *const xcb_create_window_value_list_t,
        ) -> c_int,
    >,
    xcb_create_window_value_list_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            value_mask: u32,
            _aux: *mut xcb_create_window_value_list_t,
        ) -> c_int,
    >,
    xcb_create_window_value_list_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, value_mask: u32) -> c_int>,
    xcb_create_window_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_create_window_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            depth: u8,
            wid: xcb_window_t,
            parent: xcb_window_t,
            x: i16,
            y: i16,
            width: u16,
            height: u16,
            border_width: u16,
            class: u16,
            visual: xcb_visualid_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_window: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            depth: u8,
            wid: xcb_window_t,
            parent: xcb_window_t,
            x: i16,
            y: i16,
            width: u16,
            height: u16,
            border_width: u16,
            class: u16,
            visual: xcb_visualid_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_window_aux_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            depth: u8,
            wid: xcb_window_t,
            parent: xcb_window_t,
            x: i16,
            y: i16,
            width: u16,
            height: u16,
            border_width: u16,
            class: u16,
            visual: xcb_visualid_t,
            value_mask: u32,
            value_list: *const xcb_create_window_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_window_aux: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            depth: u8,
            wid: xcb_window_t,
            parent: xcb_window_t,
            x: i16,
            y: i16,
            width: u16,
            height: u16,
            border_width: u16,
            class: u16,
            visual: xcb_visualid_t,
            value_mask: u32,
            value_list: *const xcb_create_window_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_window_value_list:
        LazySymbol<unsafe fn(r: *const xcb_create_window_request_t) -> *mut c_void>,
    xcb_change_window_attributes_value_list_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            value_mask: u32,
            _aux: *const xcb_change_window_attributes_value_list_t,
        ) -> c_int,
    >,
    xcb_change_window_attributes_value_list_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            value_mask: u32,
            _aux: *mut xcb_change_window_attributes_value_list_t,
        ) -> c_int,
    >,
    xcb_change_window_attributes_value_list_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, value_mask: u32) -> c_int>,
    xcb_change_window_attributes_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_change_window_attributes_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_window_attributes: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_window_attributes_aux_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            value_mask: u32,
            value_list: *const xcb_change_window_attributes_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_window_attributes_aux: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            value_mask: u32,
            value_list: *const xcb_change_window_attributes_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_window_attributes_value_list:
        LazySymbol<unsafe fn(r: *const xcb_change_window_attributes_request_t) -> *mut c_void>,
    xcb_get_window_attributes: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_get_window_attributes_cookie_t,
    >,
    xcb_get_window_attributes_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_get_window_attributes_cookie_t,
    >,
    xcb_get_window_attributes_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_window_attributes_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_get_window_attributes_reply_t,
    >,
    xcb_destroy_window_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_destroy_window:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_destroy_subwindows_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_destroy_subwindows:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_change_save_set_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, mode: u8, window: xcb_window_t) -> xcb_void_cookie_t,
    >,
    xcb_change_save_set: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, mode: u8, window: xcb_window_t) -> xcb_void_cookie_t,
    >,
    xcb_reparent_window_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            parent: xcb_window_t,
            x: i16,
            y: i16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_reparent_window: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            parent: xcb_window_t,
            x: i16,
            y: i16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_map_window_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_map_window:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_map_subwindows_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_map_subwindows:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_unmap_window_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_unmap_window:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_unmap_subwindows_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_unmap_subwindows:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_configure_window_value_list_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            value_mask: u16,
            _aux: *const xcb_configure_window_value_list_t,
        ) -> c_int,
    >,
    xcb_configure_window_value_list_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            value_mask: u16,
            _aux: *mut xcb_configure_window_value_list_t,
        ) -> c_int,
    >,
    xcb_configure_window_value_list_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, value_mask: u16) -> c_int>,
    xcb_configure_window_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_configure_window_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            value_mask: u16,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_configure_window: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            value_mask: u16,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_configure_window_aux_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            value_mask: u16,
            value_list: *const xcb_configure_window_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_configure_window_aux: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            value_mask: u16,
            value_list: *const xcb_configure_window_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_configure_window_value_list:
        LazySymbol<unsafe fn(r: *const xcb_configure_window_request_t) -> *mut c_void>,
    xcb_circulate_window_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            direction: u8,
            window: xcb_window_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_circulate_window: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            direction: u8,
            window: xcb_window_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_get_geometry: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t) -> xcb_get_geometry_cookie_t,
    >,
    xcb_get_geometry_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t) -> xcb_get_geometry_cookie_t,
    >,
    xcb_get_geometry_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_geometry_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_get_geometry_reply_t,
    >,
    xcb_query_tree_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_query_tree: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_query_tree_cookie_t,
    >,
    xcb_query_tree_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_query_tree_cookie_t,
    >,
    xcb_query_tree_children:
        LazySymbol<unsafe fn(r: *const xcb_query_tree_reply_t) -> *mut xcb_window_t>,
    xcb_query_tree_children_length:
        LazySymbol<unsafe fn(r: *const xcb_query_tree_reply_t) -> c_int>,
    xcb_query_tree_children_end:
        LazySymbol<unsafe fn(r: *const xcb_query_tree_reply_t) -> xcb_generic_iterator_t>,
    xcb_query_tree_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_query_tree_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_query_tree_reply_t,
    >,
    xcb_intern_atom_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_intern_atom: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            only_if_exists: u8,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_intern_atom_cookie_t,
    >,
    xcb_intern_atom_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            only_if_exists: u8,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_intern_atom_cookie_t,
    >,
    xcb_intern_atom_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_intern_atom_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_intern_atom_reply_t,
    >,
    xcb_get_atom_name_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_get_atom_name: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, atom: xcb_atom_t) -> xcb_get_atom_name_cookie_t,
    >,
    xcb_get_atom_name_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, atom: xcb_atom_t) -> xcb_get_atom_name_cookie_t,
    >,
    xcb_get_atom_name_name:
        LazySymbol<unsafe fn(r: *const xcb_get_atom_name_reply_t) -> *mut c_char>,
    xcb_get_atom_name_name_length:
        LazySymbol<unsafe fn(r: *const xcb_get_atom_name_reply_t) -> c_int>,
    xcb_get_atom_name_name_end:
        LazySymbol<unsafe fn(r: *const xcb_get_atom_name_reply_t) -> xcb_generic_iterator_t>,
    xcb_get_atom_name_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_atom_name_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_get_atom_name_reply_t,
    >,
    xcb_change_property_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_change_property_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            mode: u8,
            window: xcb_window_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            format: u8,
            data_len: u32,
            data: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            mode: u8,
            window: xcb_window_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            format: u8,
            data_len: u32,
            data: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_property_data:
        LazySymbol<unsafe fn(r: *const xcb_change_property_request_t) -> *mut c_void>,
    xcb_change_property_data_length:
        LazySymbol<unsafe fn(r: *const xcb_change_property_request_t) -> c_int>,
    xcb_change_property_data_end:
        LazySymbol<unsafe fn(r: *const xcb_change_property_request_t) -> xcb_generic_iterator_t>,
    xcb_delete_property_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            property: xcb_atom_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_delete_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            property: xcb_atom_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_get_property_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_get_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            delete: u8,
            window: xcb_window_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            long_offset: u32,
            long_length: u32,
        ) -> xcb_get_property_cookie_t,
    >,
    xcb_get_property_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            delete: u8,
            window: xcb_window_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            long_offset: u32,
            long_length: u32,
        ) -> xcb_get_property_cookie_t,
    >,
    xcb_get_property_value:
        LazySymbol<unsafe fn(r: *const xcb_get_property_reply_t) -> *mut c_void>,
    xcb_get_property_value_length:
        LazySymbol<unsafe fn(r: *const xcb_get_property_reply_t) -> c_int>,
    xcb_get_property_value_end:
        LazySymbol<unsafe fn(r: *const xcb_get_property_reply_t) -> xcb_generic_iterator_t>,
    xcb_get_property_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_property_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_get_property_reply_t,
    >,
    xcb_list_properties_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_list_properties: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_list_properties_cookie_t,
    >,
    xcb_list_properties_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_list_properties_cookie_t,
    >,
    xcb_list_properties_atoms:
        LazySymbol<unsafe fn(r: *const xcb_list_properties_reply_t) -> *mut xcb_atom_t>,
    xcb_list_properties_atoms_length:
        LazySymbol<unsafe fn(r: *const xcb_list_properties_reply_t) -> c_int>,
    xcb_list_properties_atoms_end:
        LazySymbol<unsafe fn(r: *const xcb_list_properties_reply_t) -> xcb_generic_iterator_t>,
    xcb_list_properties_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_list_properties_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_list_properties_reply_t,
    >,
    xcb_set_selection_owner_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            owner: xcb_window_t,
            selection: xcb_atom_t,
            time: xcb_timestamp_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_set_selection_owner: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            owner: xcb_window_t,
            selection: xcb_atom_t,
            time: xcb_timestamp_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_get_selection_owner: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            selection: xcb_atom_t,
        ) -> xcb_get_selection_owner_cookie_t,
    >,
    xcb_get_selection_owner_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            selection: xcb_atom_t,
        ) -> xcb_get_selection_owner_cookie_t,
    >,
    xcb_get_selection_owner_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_selection_owner_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_get_selection_owner_reply_t,
    >,
    xcb_convert_selection_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            requestor: xcb_window_t,
            selection: xcb_atom_t,
            target: xcb_atom_t,
            property: xcb_atom_t,
            time: xcb_timestamp_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_convert_selection: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            requestor: xcb_window_t,
            selection: xcb_atom_t,
            target: xcb_atom_t,
            property: xcb_atom_t,
            time: xcb_timestamp_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_send_event_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            propagate: u8,
            destination: xcb_window_t,
            event_mask: u32,
            event: *const [c_char; 32],
        ) -> xcb_void_cookie_t,
    >,
    xcb_send_event: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            propagate: u8,
            destination: xcb_window_t,
            event_mask: u32,
            event: *const [c_char; 32],
        ) -> xcb_void_cookie_t,
    >,
    xcb_grab_pointer: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            owner_events: u8,
            grab_window: xcb_window_t,
            event_mask: u16,
            pointer_mode: u8,
            keyboard_mode: u8,
            confine_to: xcb_window_t,
            cursor: xcb_cursor_t,
            time: xcb_timestamp_t,
        ) -> xcb_grab_pointer_cookie_t,
    >,
    xcb_grab_pointer_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            owner_events: u8,
            grab_window: xcb_window_t,
            event_mask: u16,
            pointer_mode: u8,
            keyboard_mode: u8,
            confine_to: xcb_window_t,
            cursor: xcb_cursor_t,
            time: xcb_timestamp_t,
        ) -> xcb_grab_pointer_cookie_t,
    >,
    xcb_grab_pointer_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_grab_pointer_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_grab_pointer_reply_t,
    >,
    xcb_ungrab_pointer_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, time: xcb_timestamp_t) -> xcb_void_cookie_t>,
    xcb_ungrab_pointer:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, time: xcb_timestamp_t) -> xcb_void_cookie_t>,
    xcb_grab_button_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            owner_events: u8,
            grab_window: xcb_window_t,
            event_mask: u16,
            pointer_mode: u8,
            keyboard_mode: u8,
            confine_to: xcb_window_t,
            cursor: xcb_cursor_t,
            button: u8,
            modifiers: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_grab_button: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            owner_events: u8,
            grab_window: xcb_window_t,
            event_mask: u16,
            pointer_mode: u8,
            keyboard_mode: u8,
            confine_to: xcb_window_t,
            cursor: xcb_cursor_t,
            button: u8,
            modifiers: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_ungrab_button_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            button: u8,
            grab_window: xcb_window_t,
            modifiers: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_ungrab_button: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            button: u8,
            grab_window: xcb_window_t,
            modifiers: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_active_pointer_grab_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cursor: xcb_cursor_t,
            time: xcb_timestamp_t,
            event_mask: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_active_pointer_grab: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cursor: xcb_cursor_t,
            time: xcb_timestamp_t,
            event_mask: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_grab_keyboard: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            owner_events: u8,
            grab_window: xcb_window_t,
            time: xcb_timestamp_t,
            pointer_mode: u8,
            keyboard_mode: u8,
        ) -> xcb_grab_keyboard_cookie_t,
    >,
    xcb_grab_keyboard_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            owner_events: u8,
            grab_window: xcb_window_t,
            time: xcb_timestamp_t,
            pointer_mode: u8,
            keyboard_mode: u8,
        ) -> xcb_grab_keyboard_cookie_t,
    >,
    xcb_grab_keyboard_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_grab_keyboard_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_grab_keyboard_reply_t,
    >,
    xcb_ungrab_keyboard_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, time: xcb_timestamp_t) -> xcb_void_cookie_t>,
    xcb_ungrab_keyboard:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, time: xcb_timestamp_t) -> xcb_void_cookie_t>,
    xcb_grab_key_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            owner_events: u8,
            grab_window: xcb_window_t,
            modifiers: u16,
            key: xcb_keycode_t,
            pointer_mode: u8,
            keyboard_mode: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_grab_key: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            owner_events: u8,
            grab_window: xcb_window_t,
            modifiers: u16,
            key: xcb_keycode_t,
            pointer_mode: u8,
            keyboard_mode: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_ungrab_key_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            key: xcb_keycode_t,
            grab_window: xcb_window_t,
            modifiers: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_ungrab_key: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            key: xcb_keycode_t,
            grab_window: xcb_window_t,
            modifiers: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_allow_events_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, mode: u8, time: xcb_timestamp_t) -> xcb_void_cookie_t,
    >,
    xcb_allow_events: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, mode: u8, time: xcb_timestamp_t) -> xcb_void_cookie_t,
    >,
    xcb_grab_server_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    xcb_grab_server: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    xcb_ungrab_server_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    xcb_ungrab_server: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    xcb_query_pointer: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_query_pointer_cookie_t,
    >,
    xcb_query_pointer_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_query_pointer_cookie_t,
    >,
    xcb_query_pointer_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_query_pointer_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_query_pointer_reply_t,
    >,
    xcb_timecoord_next: LazySymbol<unsafe fn(i: *mut xcb_timecoord_iterator_t)>,
    xcb_timecoord_end: LazySymbol<unsafe fn(i: xcb_timecoord_iterator_t) -> xcb_generic_iterator_t>,
    xcb_get_motion_events_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_get_motion_events: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            start: xcb_timestamp_t,
            stop: xcb_timestamp_t,
        ) -> xcb_get_motion_events_cookie_t,
    >,
    xcb_get_motion_events_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            start: xcb_timestamp_t,
            stop: xcb_timestamp_t,
        ) -> xcb_get_motion_events_cookie_t,
    >,
    xcb_get_motion_events_events:
        LazySymbol<unsafe fn(r: *const xcb_get_motion_events_reply_t) -> *mut xcb_timecoord_t>,
    xcb_get_motion_events_events_length:
        LazySymbol<unsafe fn(r: *const xcb_get_motion_events_reply_t) -> c_int>,
    xcb_get_motion_events_events_iterator:
        LazySymbol<unsafe fn(r: *const xcb_get_motion_events_reply_t) -> xcb_timecoord_iterator_t>,
    xcb_get_motion_events_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_motion_events_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_get_motion_events_reply_t,
    >,
    xcb_translate_coordinates: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src_window: xcb_window_t,
            dst_window: xcb_window_t,
            src_x: i16,
            src_y: i16,
        ) -> xcb_translate_coordinates_cookie_t,
    >,
    xcb_translate_coordinates_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src_window: xcb_window_t,
            dst_window: xcb_window_t,
            src_x: i16,
            src_y: i16,
        ) -> xcb_translate_coordinates_cookie_t,
    >,
    xcb_translate_coordinates_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_translate_coordinates_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_translate_coordinates_reply_t,
    >,
    xcb_warp_pointer_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src_window: xcb_window_t,
            dst_window: xcb_window_t,
            src_x: i16,
            src_y: i16,
            src_width: u16,
            src_height: u16,
            dst_x: i16,
            dst_y: i16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_warp_pointer: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src_window: xcb_window_t,
            dst_window: xcb_window_t,
            src_x: i16,
            src_y: i16,
            src_width: u16,
            src_height: u16,
            dst_x: i16,
            dst_y: i16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_set_input_focus_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            revert_to: u8,
            focus: xcb_window_t,
            time: xcb_timestamp_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_set_input_focus: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            revert_to: u8,
            focus: xcb_window_t,
            time: xcb_timestamp_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_get_input_focus:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_input_focus_cookie_t>,
    xcb_get_input_focus_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_input_focus_cookie_t>,
    xcb_get_input_focus_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_input_focus_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_get_input_focus_reply_t,
    >,
    xcb_query_keymap: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_query_keymap_cookie_t>,
    xcb_query_keymap_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_query_keymap_cookie_t>,
    xcb_query_keymap_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_query_keymap_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_query_keymap_reply_t,
    >,
    xcb_open_font_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_open_font_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            fid: xcb_font_t,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_open_font: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            fid: xcb_font_t,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_open_font_name: LazySymbol<unsafe fn(r: *const xcb_open_font_request_t) -> *mut c_char>,
    xcb_open_font_name_length: LazySymbol<unsafe fn(r: *const xcb_open_font_request_t) -> c_int>,
    xcb_open_font_name_end:
        LazySymbol<unsafe fn(r: *const xcb_open_font_request_t) -> xcb_generic_iterator_t>,
    xcb_close_font_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, font: xcb_font_t) -> xcb_void_cookie_t>,
    xcb_close_font:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, font: xcb_font_t) -> xcb_void_cookie_t>,
    xcb_fontprop_next: LazySymbol<unsafe fn(i: *mut xcb_fontprop_iterator_t)>,
    xcb_fontprop_end: LazySymbol<unsafe fn(i: xcb_fontprop_iterator_t) -> xcb_generic_iterator_t>,
    xcb_charinfo_next: LazySymbol<unsafe fn(i: *mut xcb_charinfo_iterator_t)>,
    xcb_charinfo_end: LazySymbol<unsafe fn(i: xcb_charinfo_iterator_t) -> xcb_generic_iterator_t>,
    xcb_query_font_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_query_font: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, font: xcb_fontable_t) -> xcb_query_font_cookie_t,
    >,
    xcb_query_font_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, font: xcb_fontable_t) -> xcb_query_font_cookie_t,
    >,
    xcb_query_font_properties:
        LazySymbol<unsafe fn(r: *const xcb_query_font_reply_t) -> *mut xcb_fontprop_t>,
    xcb_query_font_properties_length:
        LazySymbol<unsafe fn(r: *const xcb_query_font_reply_t) -> c_int>,
    xcb_query_font_properties_iterator:
        LazySymbol<unsafe fn(r: *const xcb_query_font_reply_t) -> xcb_fontprop_iterator_t>,
    xcb_query_font_char_infos:
        LazySymbol<unsafe fn(r: *const xcb_query_font_reply_t) -> *mut xcb_charinfo_t>,
    xcb_query_font_char_infos_length:
        LazySymbol<unsafe fn(r: *const xcb_query_font_reply_t) -> c_int>,
    xcb_query_font_char_infos_iterator:
        LazySymbol<unsafe fn(r: *const xcb_query_font_reply_t) -> xcb_charinfo_iterator_t>,
    xcb_query_font_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_query_font_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_query_font_reply_t,
    >,
    xcb_query_text_extents_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, string_len: u32) -> c_int>,
    xcb_query_text_extents: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            font: xcb_fontable_t,
            string_len: u32,
            string: *const xcb_char2b_t,
        ) -> xcb_query_text_extents_cookie_t,
    >,
    xcb_query_text_extents_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            font: xcb_fontable_t,
            string_len: u32,
            string: *const xcb_char2b_t,
        ) -> xcb_query_text_extents_cookie_t,
    >,
    xcb_query_text_extents_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_query_text_extents_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_query_text_extents_reply_t,
    >,
    xcb_str_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_str_name: LazySymbol<unsafe fn(r: *const xcb_str_t) -> *mut c_char>,
    xcb_str_name_length: LazySymbol<unsafe fn(r: *const xcb_str_t) -> c_int>,
    xcb_str_name_end: LazySymbol<unsafe fn(r: *const xcb_str_t) -> xcb_generic_iterator_t>,
    xcb_str_next: LazySymbol<unsafe fn(i: *mut xcb_str_iterator_t)>,
    xcb_str_end: LazySymbol<unsafe fn(i: xcb_str_iterator_t) -> xcb_generic_iterator_t>,
    xcb_list_fonts_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_list_fonts: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            max_names: u16,
            pattern_len: u16,
            pattern: *const c_char,
        ) -> xcb_list_fonts_cookie_t,
    >,
    xcb_list_fonts_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            max_names: u16,
            pattern_len: u16,
            pattern: *const c_char,
        ) -> xcb_list_fonts_cookie_t,
    >,
    xcb_list_fonts_names_length: LazySymbol<unsafe fn(r: *const xcb_list_fonts_reply_t) -> c_int>,
    xcb_list_fonts_names_iterator:
        LazySymbol<unsafe fn(r: *const xcb_list_fonts_reply_t) -> xcb_str_iterator_t>,
    xcb_list_fonts_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_list_fonts_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_list_fonts_reply_t,
    >,
    xcb_list_fonts_with_info_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_list_fonts_with_info: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            max_names: u16,
            pattern_len: u16,
            pattern: *const c_char,
        ) -> xcb_list_fonts_with_info_cookie_t,
    >,
    xcb_list_fonts_with_info_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            max_names: u16,
            pattern_len: u16,
            pattern: *const c_char,
        ) -> xcb_list_fonts_with_info_cookie_t,
    >,
    xcb_list_fonts_with_info_properties:
        LazySymbol<unsafe fn(r: *const xcb_list_fonts_with_info_reply_t) -> *mut xcb_fontprop_t>,
    xcb_list_fonts_with_info_properties_length:
        LazySymbol<unsafe fn(r: *const xcb_list_fonts_with_info_reply_t) -> c_int>,
    xcb_list_fonts_with_info_properties_iterator: LazySymbol<
        unsafe fn(r: *const xcb_list_fonts_with_info_reply_t) -> xcb_fontprop_iterator_t,
    >,
    xcb_list_fonts_with_info_name:
        LazySymbol<unsafe fn(r: *const xcb_list_fonts_with_info_reply_t) -> *mut c_char>,
    xcb_list_fonts_with_info_name_length:
        LazySymbol<unsafe fn(r: *const xcb_list_fonts_with_info_reply_t) -> c_int>,
    xcb_list_fonts_with_info_name_end:
        LazySymbol<unsafe fn(r: *const xcb_list_fonts_with_info_reply_t) -> xcb_generic_iterator_t>,
    xcb_list_fonts_with_info_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_list_fonts_with_info_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_list_fonts_with_info_reply_t,
    >,
    xcb_set_font_path_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_set_font_path_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            font_qty: u16,
            font: *const xcb_str_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_set_font_path: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            font_qty: u16,
            font: *const xcb_str_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_set_font_path_font_length:
        LazySymbol<unsafe fn(r: *const xcb_set_font_path_request_t) -> c_int>,
    xcb_set_font_path_font_iterator:
        LazySymbol<unsafe fn(r: *const xcb_set_font_path_request_t) -> xcb_str_iterator_t>,
    xcb_get_font_path_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_get_font_path:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_font_path_cookie_t>,
    xcb_get_font_path_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_font_path_cookie_t>,
    xcb_get_font_path_path_length:
        LazySymbol<unsafe fn(r: *const xcb_get_font_path_reply_t) -> c_int>,
    xcb_get_font_path_path_iterator:
        LazySymbol<unsafe fn(r: *const xcb_get_font_path_reply_t) -> xcb_str_iterator_t>,
    xcb_get_font_path_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_font_path_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_get_font_path_reply_t,
    >,
    xcb_create_pixmap_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            depth: u8,
            pid: xcb_pixmap_t,
            drawable: xcb_drawable_t,
            width: u16,
            height: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_pixmap: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            depth: u8,
            pid: xcb_pixmap_t,
            drawable: xcb_drawable_t,
            width: u16,
            height: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_free_pixmap_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, pixmap: xcb_pixmap_t) -> xcb_void_cookie_t>,
    xcb_free_pixmap:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, pixmap: xcb_pixmap_t) -> xcb_void_cookie_t>,
    xcb_create_gc_value_list_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            value_mask: u32,
            _aux: *const xcb_create_gc_value_list_t,
        ) -> c_int,
    >,
    xcb_create_gc_value_list_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            value_mask: u32,
            _aux: *mut xcb_create_gc_value_list_t,
        ) -> c_int,
    >,
    xcb_create_gc_value_list_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, value_mask: u32) -> c_int>,
    xcb_create_gc_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_create_gc_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cid: xcb_gcontext_t,
            drawable: xcb_drawable_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_gc: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cid: xcb_gcontext_t,
            drawable: xcb_drawable_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_gc_aux_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cid: xcb_gcontext_t,
            drawable: xcb_drawable_t,
            value_mask: u32,
            value_list: *const xcb_create_gc_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_gc_aux: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cid: xcb_gcontext_t,
            drawable: xcb_drawable_t,
            value_mask: u32,
            value_list: *const xcb_create_gc_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_gc_value_list:
        LazySymbol<unsafe fn(r: *const xcb_create_gc_request_t) -> *mut c_void>,
    xcb_change_gc_value_list_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            value_mask: u32,
            _aux: *const xcb_change_gc_value_list_t,
        ) -> c_int,
    >,
    xcb_change_gc_value_list_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            value_mask: u32,
            _aux: *mut xcb_change_gc_value_list_t,
        ) -> c_int,
    >,
    xcb_change_gc_value_list_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, value_mask: u32) -> c_int>,
    xcb_change_gc_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_change_gc_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gc: xcb_gcontext_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_gc: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gc: xcb_gcontext_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_gc_aux_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gc: xcb_gcontext_t,
            value_mask: u32,
            value_list: *const xcb_change_gc_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_gc_aux: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gc: xcb_gcontext_t,
            value_mask: u32,
            value_list: *const xcb_change_gc_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_gc_value_list:
        LazySymbol<unsafe fn(r: *const xcb_change_gc_request_t) -> *mut c_void>,
    xcb_copy_gc_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src_gc: xcb_gcontext_t,
            dst_gc: xcb_gcontext_t,
            value_mask: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_copy_gc: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src_gc: xcb_gcontext_t,
            dst_gc: xcb_gcontext_t,
            value_mask: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_set_dashes_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_set_dashes_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gc: xcb_gcontext_t,
            dash_offset: u16,
            dashes_len: u16,
            dashes: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_set_dashes: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gc: xcb_gcontext_t,
            dash_offset: u16,
            dashes_len: u16,
            dashes: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_set_dashes_dashes: LazySymbol<unsafe fn(r: *const xcb_set_dashes_request_t) -> *mut u8>,
    xcb_set_dashes_dashes_length:
        LazySymbol<unsafe fn(r: *const xcb_set_dashes_request_t) -> c_int>,
    xcb_set_dashes_dashes_end:
        LazySymbol<unsafe fn(r: *const xcb_set_dashes_request_t) -> xcb_generic_iterator_t>,
    xcb_set_clip_rectangles_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, rectangles_len: u32) -> c_int>,
    xcb_set_clip_rectangles_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            ordering: u8,
            gc: xcb_gcontext_t,
            clip_x_origin: i16,
            clip_y_origin: i16,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_set_clip_rectangles: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            ordering: u8,
            gc: xcb_gcontext_t,
            clip_x_origin: i16,
            clip_y_origin: i16,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_set_clip_rectangles_rectangles:
        LazySymbol<unsafe fn(r: *const xcb_set_clip_rectangles_request_t) -> *mut xcb_rectangle_t>,
    xcb_set_clip_rectangles_rectangles_length:
        LazySymbol<unsafe fn(r: *const xcb_set_clip_rectangles_request_t) -> c_int>,
    xcb_set_clip_rectangles_rectangles_iterator: LazySymbol<
        unsafe fn(r: *const xcb_set_clip_rectangles_request_t) -> xcb_rectangle_iterator_t,
    >,
    xcb_free_gc_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, gc: xcb_gcontext_t) -> xcb_void_cookie_t>,
    xcb_free_gc:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, gc: xcb_gcontext_t) -> xcb_void_cookie_t>,
    xcb_clear_area_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            exposures: u8,
            window: xcb_window_t,
            x: i16,
            y: i16,
            width: u16,
            height: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_clear_area: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            exposures: u8,
            window: xcb_window_t,
            x: i16,
            y: i16,
            width: u16,
            height: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_copy_area_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src_drawable: xcb_drawable_t,
            dst_drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            src_x: i16,
            src_y: i16,
            dst_x: i16,
            dst_y: i16,
            width: u16,
            height: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_copy_area: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src_drawable: xcb_drawable_t,
            dst_drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            src_x: i16,
            src_y: i16,
            dst_x: i16,
            dst_y: i16,
            width: u16,
            height: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_copy_plane_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src_drawable: xcb_drawable_t,
            dst_drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            src_x: i16,
            src_y: i16,
            dst_x: i16,
            dst_y: i16,
            width: u16,
            height: u16,
            bit_plane: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_copy_plane: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src_drawable: xcb_drawable_t,
            dst_drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            src_x: i16,
            src_y: i16,
            dst_x: i16,
            dst_y: i16,
            width: u16,
            height: u16,
            bit_plane: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_point_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void, points_len: u32) -> c_int>,
    xcb_poly_point_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            coordinate_mode: u8,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            points_len: u32,
            points: *const xcb_point_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_point: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            coordinate_mode: u8,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            points_len: u32,
            points: *const xcb_point_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_point_points:
        LazySymbol<unsafe fn(r: *const xcb_poly_point_request_t) -> *mut xcb_point_t>,
    xcb_poly_point_points_length:
        LazySymbol<unsafe fn(r: *const xcb_poly_point_request_t) -> c_int>,
    xcb_poly_point_points_iterator:
        LazySymbol<unsafe fn(r: *const xcb_poly_point_request_t) -> xcb_point_iterator_t>,
    xcb_poly_line_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void, points_len: u32) -> c_int>,
    xcb_poly_line_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            coordinate_mode: u8,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            points_len: u32,
            points: *const xcb_point_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_line: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            coordinate_mode: u8,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            points_len: u32,
            points: *const xcb_point_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_line_points:
        LazySymbol<unsafe fn(r: *const xcb_poly_line_request_t) -> *mut xcb_point_t>,
    xcb_poly_line_points_length: LazySymbol<unsafe fn(r: *const xcb_poly_line_request_t) -> c_int>,
    xcb_poly_line_points_iterator:
        LazySymbol<unsafe fn(r: *const xcb_poly_line_request_t) -> xcb_point_iterator_t>,
    xcb_segment_next: LazySymbol<unsafe fn(i: *mut xcb_segment_iterator_t)>,
    xcb_segment_end: LazySymbol<unsafe fn(i: xcb_segment_iterator_t) -> xcb_generic_iterator_t>,
    xcb_poly_segment_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, segments_len: u32) -> c_int>,
    xcb_poly_segment_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            segments_len: u32,
            segments: *const xcb_segment_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_segment: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            segments_len: u32,
            segments: *const xcb_segment_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_segment_segments:
        LazySymbol<unsafe fn(r: *const xcb_poly_segment_request_t) -> *mut xcb_segment_t>,
    xcb_poly_segment_segments_length:
        LazySymbol<unsafe fn(r: *const xcb_poly_segment_request_t) -> c_int>,
    xcb_poly_segment_segments_iterator:
        LazySymbol<unsafe fn(r: *const xcb_poly_segment_request_t) -> xcb_segment_iterator_t>,
    xcb_poly_rectangle_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, rectangles_len: u32) -> c_int>,
    xcb_poly_rectangle_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_rectangle: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_rectangle_rectangles:
        LazySymbol<unsafe fn(r: *const xcb_poly_rectangle_request_t) -> *mut xcb_rectangle_t>,
    xcb_poly_rectangle_rectangles_length:
        LazySymbol<unsafe fn(r: *const xcb_poly_rectangle_request_t) -> c_int>,
    xcb_poly_rectangle_rectangles_iterator:
        LazySymbol<unsafe fn(r: *const xcb_poly_rectangle_request_t) -> xcb_rectangle_iterator_t>,
    xcb_poly_arc_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void, arcs_len: u32) -> c_int>,
    xcb_poly_arc_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            arcs_len: u32,
            arcs: *const xcb_arc_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_arc: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            arcs_len: u32,
            arcs: *const xcb_arc_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_arc_arcs: LazySymbol<unsafe fn(r: *const xcb_poly_arc_request_t) -> *mut xcb_arc_t>,
    xcb_poly_arc_arcs_length: LazySymbol<unsafe fn(r: *const xcb_poly_arc_request_t) -> c_int>,
    xcb_poly_arc_arcs_iterator:
        LazySymbol<unsafe fn(r: *const xcb_poly_arc_request_t) -> xcb_arc_iterator_t>,
    xcb_fill_poly_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void, points_len: u32) -> c_int>,
    xcb_fill_poly_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            shape: u8,
            coordinate_mode: u8,
            points_len: u32,
            points: *const xcb_point_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_fill_poly: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            shape: u8,
            coordinate_mode: u8,
            points_len: u32,
            points: *const xcb_point_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_fill_poly_points:
        LazySymbol<unsafe fn(r: *const xcb_fill_poly_request_t) -> *mut xcb_point_t>,
    xcb_fill_poly_points_length: LazySymbol<unsafe fn(r: *const xcb_fill_poly_request_t) -> c_int>,
    xcb_fill_poly_points_iterator:
        LazySymbol<unsafe fn(r: *const xcb_fill_poly_request_t) -> xcb_point_iterator_t>,
    xcb_poly_fill_rectangle_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, rectangles_len: u32) -> c_int>,
    xcb_poly_fill_rectangle_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_fill_rectangle: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_fill_rectangle_rectangles:
        LazySymbol<unsafe fn(r: *const xcb_poly_fill_rectangle_request_t) -> *mut xcb_rectangle_t>,
    xcb_poly_fill_rectangle_rectangles_length:
        LazySymbol<unsafe fn(r: *const xcb_poly_fill_rectangle_request_t) -> c_int>,
    xcb_poly_fill_rectangle_rectangles_iterator: LazySymbol<
        unsafe fn(r: *const xcb_poly_fill_rectangle_request_t) -> xcb_rectangle_iterator_t,
    >,
    xcb_poly_fill_arc_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void, arcs_len: u32) -> c_int>,
    xcb_poly_fill_arc_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            arcs_len: u32,
            arcs: *const xcb_arc_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_fill_arc: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            arcs_len: u32,
            arcs: *const xcb_arc_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_fill_arc_arcs:
        LazySymbol<unsafe fn(r: *const xcb_poly_fill_arc_request_t) -> *mut xcb_arc_t>,
    xcb_poly_fill_arc_arcs_length:
        LazySymbol<unsafe fn(r: *const xcb_poly_fill_arc_request_t) -> c_int>,
    xcb_poly_fill_arc_arcs_iterator:
        LazySymbol<unsafe fn(r: *const xcb_poly_fill_arc_request_t) -> xcb_arc_iterator_t>,
    xcb_put_image_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void, data_len: u32) -> c_int>,
    xcb_put_image_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            format: u8,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            width: u16,
            height: u16,
            dst_x: i16,
            dst_y: i16,
            left_pad: u8,
            depth: u8,
            data_len: u32,
            data: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_put_image: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            format: u8,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            width: u16,
            height: u16,
            dst_x: i16,
            dst_y: i16,
            left_pad: u8,
            depth: u8,
            data_len: u32,
            data: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_put_image_data: LazySymbol<unsafe fn(r: *const xcb_put_image_request_t) -> *mut u8>,
    xcb_put_image_data_length: LazySymbol<unsafe fn(r: *const xcb_put_image_request_t) -> c_int>,
    xcb_put_image_data_end:
        LazySymbol<unsafe fn(r: *const xcb_put_image_request_t) -> xcb_generic_iterator_t>,
    xcb_get_image_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_get_image: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            format: u8,
            drawable: xcb_drawable_t,
            x: i16,
            y: i16,
            width: u16,
            height: u16,
            plane_mask: u32,
        ) -> xcb_get_image_cookie_t,
    >,
    xcb_get_image_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            format: u8,
            drawable: xcb_drawable_t,
            x: i16,
            y: i16,
            width: u16,
            height: u16,
            plane_mask: u32,
        ) -> xcb_get_image_cookie_t,
    >,
    xcb_get_image_data: LazySymbol<unsafe fn(r: *const xcb_get_image_reply_t) -> *mut u8>,
    xcb_get_image_data_length: LazySymbol<unsafe fn(r: *const xcb_get_image_reply_t) -> c_int>,
    xcb_get_image_data_end:
        LazySymbol<unsafe fn(r: *const xcb_get_image_reply_t) -> xcb_generic_iterator_t>,
    xcb_get_image_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_image_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_get_image_reply_t,
    >,
    xcb_poly_text_8_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void, items_len: u32) -> c_int>,
    xcb_poly_text_8_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            x: i16,
            y: i16,
            items_len: u32,
            items: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_text_8: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            x: i16,
            y: i16,
            items_len: u32,
            items: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_text_8_items: LazySymbol<unsafe fn(r: *const xcb_poly_text_8_request_t) -> *mut u8>,
    xcb_poly_text_8_items_length:
        LazySymbol<unsafe fn(r: *const xcb_poly_text_8_request_t) -> c_int>,
    xcb_poly_text_8_items_end:
        LazySymbol<unsafe fn(r: *const xcb_poly_text_8_request_t) -> xcb_generic_iterator_t>,
    xcb_poly_text_16_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void, items_len: u32) -> c_int>,
    xcb_poly_text_16_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            x: i16,
            y: i16,
            items_len: u32,
            items: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_text_16: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            x: i16,
            y: i16,
            items_len: u32,
            items: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_text_16_items: LazySymbol<unsafe fn(r: *const xcb_poly_text_16_request_t) -> *mut u8>,
    xcb_poly_text_16_items_length:
        LazySymbol<unsafe fn(r: *const xcb_poly_text_16_request_t) -> c_int>,
    xcb_poly_text_16_items_end:
        LazySymbol<unsafe fn(r: *const xcb_poly_text_16_request_t) -> xcb_generic_iterator_t>,
    xcb_image_text_8_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_image_text_8_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            string_len: u8,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            x: i16,
            y: i16,
            string: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_image_text_8: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            string_len: u8,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            x: i16,
            y: i16,
            string: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_image_text_8_string:
        LazySymbol<unsafe fn(r: *const xcb_image_text_8_request_t) -> *mut c_char>,
    xcb_image_text_8_string_length:
        LazySymbol<unsafe fn(r: *const xcb_image_text_8_request_t) -> c_int>,
    xcb_image_text_8_string_end:
        LazySymbol<unsafe fn(r: *const xcb_image_text_8_request_t) -> xcb_generic_iterator_t>,
    xcb_image_text_16_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_image_text_16_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            string_len: u8,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            x: i16,
            y: i16,
            string: *const xcb_char2b_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_image_text_16: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            string_len: u8,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            x: i16,
            y: i16,
            string: *const xcb_char2b_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_image_text_16_string:
        LazySymbol<unsafe fn(r: *const xcb_image_text_16_request_t) -> *mut xcb_char2b_t>,
    xcb_image_text_16_string_length:
        LazySymbol<unsafe fn(r: *const xcb_image_text_16_request_t) -> c_int>,
    xcb_image_text_16_string_iterator:
        LazySymbol<unsafe fn(r: *const xcb_image_text_16_request_t) -> xcb_char2b_iterator_t>,
    xcb_create_colormap_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            alloc: u8,
            mid: xcb_colormap_t,
            window: xcb_window_t,
            visual: xcb_visualid_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_colormap: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            alloc: u8,
            mid: xcb_colormap_t,
            window: xcb_window_t,
            visual: xcb_visualid_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_free_colormap_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cmap: xcb_colormap_t) -> xcb_void_cookie_t>,
    xcb_free_colormap:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cmap: xcb_colormap_t) -> xcb_void_cookie_t>,
    xcb_copy_colormap_and_free_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            mid: xcb_colormap_t,
            src_cmap: xcb_colormap_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_copy_colormap_and_free: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            mid: xcb_colormap_t,
            src_cmap: xcb_colormap_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_install_colormap_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cmap: xcb_colormap_t) -> xcb_void_cookie_t>,
    xcb_install_colormap:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cmap: xcb_colormap_t) -> xcb_void_cookie_t>,
    xcb_uninstall_colormap_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cmap: xcb_colormap_t) -> xcb_void_cookie_t>,
    xcb_uninstall_colormap:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cmap: xcb_colormap_t) -> xcb_void_cookie_t>,
    xcb_list_installed_colormaps_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_list_installed_colormaps: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_list_installed_colormaps_cookie_t,
    >,
    xcb_list_installed_colormaps_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_list_installed_colormaps_cookie_t,
    >,
    xcb_list_installed_colormaps_cmaps: LazySymbol<
        unsafe fn(r: *const xcb_list_installed_colormaps_reply_t) -> *mut xcb_colormap_t,
    >,
    xcb_list_installed_colormaps_cmaps_length:
        LazySymbol<unsafe fn(r: *const xcb_list_installed_colormaps_reply_t) -> c_int>,
    xcb_list_installed_colormaps_cmaps_end: LazySymbol<
        unsafe fn(r: *const xcb_list_installed_colormaps_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_list_installed_colormaps_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_list_installed_colormaps_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_list_installed_colormaps_reply_t,
    >,
    xcb_alloc_color: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cmap: xcb_colormap_t,
            red: u16,
            green: u16,
            blue: u16,
        ) -> xcb_alloc_color_cookie_t,
    >,
    xcb_alloc_color_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cmap: xcb_colormap_t,
            red: u16,
            green: u16,
            blue: u16,
        ) -> xcb_alloc_color_cookie_t,
    >,
    xcb_alloc_color_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_alloc_color_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_alloc_color_reply_t,
    >,
    xcb_alloc_named_color_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_alloc_named_color: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cmap: xcb_colormap_t,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_alloc_named_color_cookie_t,
    >,
    xcb_alloc_named_color_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cmap: xcb_colormap_t,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_alloc_named_color_cookie_t,
    >,
    xcb_alloc_named_color_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_alloc_named_color_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_alloc_named_color_reply_t,
    >,
    xcb_alloc_color_cells_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_alloc_color_cells: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            contiguous: u8,
            cmap: xcb_colormap_t,
            colors: u16,
            planes: u16,
        ) -> xcb_alloc_color_cells_cookie_t,
    >,
    xcb_alloc_color_cells_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            contiguous: u8,
            cmap: xcb_colormap_t,
            colors: u16,
            planes: u16,
        ) -> xcb_alloc_color_cells_cookie_t,
    >,
    xcb_alloc_color_cells_pixels:
        LazySymbol<unsafe fn(r: *const xcb_alloc_color_cells_reply_t) -> *mut u32>,
    xcb_alloc_color_cells_pixels_length:
        LazySymbol<unsafe fn(r: *const xcb_alloc_color_cells_reply_t) -> c_int>,
    xcb_alloc_color_cells_pixels_end:
        LazySymbol<unsafe fn(r: *const xcb_alloc_color_cells_reply_t) -> xcb_generic_iterator_t>,
    xcb_alloc_color_cells_masks:
        LazySymbol<unsafe fn(r: *const xcb_alloc_color_cells_reply_t) -> *mut u32>,
    xcb_alloc_color_cells_masks_length:
        LazySymbol<unsafe fn(r: *const xcb_alloc_color_cells_reply_t) -> c_int>,
    xcb_alloc_color_cells_masks_end:
        LazySymbol<unsafe fn(r: *const xcb_alloc_color_cells_reply_t) -> xcb_generic_iterator_t>,
    xcb_alloc_color_cells_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_alloc_color_cells_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_alloc_color_cells_reply_t,
    >,
    xcb_alloc_color_planes_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_alloc_color_planes: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            contiguous: u8,
            cmap: xcb_colormap_t,
            colors: u16,
            reds: u16,
            greens: u16,
            blues: u16,
        ) -> xcb_alloc_color_planes_cookie_t,
    >,
    xcb_alloc_color_planes_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            contiguous: u8,
            cmap: xcb_colormap_t,
            colors: u16,
            reds: u16,
            greens: u16,
            blues: u16,
        ) -> xcb_alloc_color_planes_cookie_t,
    >,
    xcb_alloc_color_planes_pixels:
        LazySymbol<unsafe fn(r: *const xcb_alloc_color_planes_reply_t) -> *mut u32>,
    xcb_alloc_color_planes_pixels_length:
        LazySymbol<unsafe fn(r: *const xcb_alloc_color_planes_reply_t) -> c_int>,
    xcb_alloc_color_planes_pixels_end:
        LazySymbol<unsafe fn(r: *const xcb_alloc_color_planes_reply_t) -> xcb_generic_iterator_t>,
    xcb_alloc_color_planes_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_alloc_color_planes_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_alloc_color_planes_reply_t,
    >,
    xcb_free_colors_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void, pixels_len: u32) -> c_int>,
    xcb_free_colors_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cmap: xcb_colormap_t,
            plane_mask: u32,
            pixels_len: u32,
            pixels: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_free_colors: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cmap: xcb_colormap_t,
            plane_mask: u32,
            pixels_len: u32,
            pixels: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_free_colors_pixels: LazySymbol<unsafe fn(r: *const xcb_free_colors_request_t) -> *mut u32>,
    xcb_free_colors_pixels_length:
        LazySymbol<unsafe fn(r: *const xcb_free_colors_request_t) -> c_int>,
    xcb_free_colors_pixels_end:
        LazySymbol<unsafe fn(r: *const xcb_free_colors_request_t) -> xcb_generic_iterator_t>,
    xcb_coloritem_next: LazySymbol<unsafe fn(i: *mut xcb_coloritem_iterator_t)>,
    xcb_coloritem_end: LazySymbol<unsafe fn(i: xcb_coloritem_iterator_t) -> xcb_generic_iterator_t>,
    xcb_store_colors_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void, items_len: u32) -> c_int>,
    xcb_store_colors_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cmap: xcb_colormap_t,
            items_len: u32,
            items: *const xcb_coloritem_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_store_colors: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cmap: xcb_colormap_t,
            items_len: u32,
            items: *const xcb_coloritem_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_store_colors_items:
        LazySymbol<unsafe fn(r: *const xcb_store_colors_request_t) -> *mut xcb_coloritem_t>,
    xcb_store_colors_items_length:
        LazySymbol<unsafe fn(r: *const xcb_store_colors_request_t) -> c_int>,
    xcb_store_colors_items_iterator:
        LazySymbol<unsafe fn(r: *const xcb_store_colors_request_t) -> xcb_coloritem_iterator_t>,
    xcb_store_named_color_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_store_named_color_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            flags: u8,
            cmap: xcb_colormap_t,
            pixel: u32,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_store_named_color: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            flags: u8,
            cmap: xcb_colormap_t,
            pixel: u32,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_store_named_color_name:
        LazySymbol<unsafe fn(r: *const xcb_store_named_color_request_t) -> *mut c_char>,
    xcb_store_named_color_name_length:
        LazySymbol<unsafe fn(r: *const xcb_store_named_color_request_t) -> c_int>,
    xcb_store_named_color_name_end:
        LazySymbol<unsafe fn(r: *const xcb_store_named_color_request_t) -> xcb_generic_iterator_t>,
    xcb_rgb_next: LazySymbol<unsafe fn(i: *mut xcb_rgb_iterator_t)>,
    xcb_rgb_end: LazySymbol<unsafe fn(i: xcb_rgb_iterator_t) -> xcb_generic_iterator_t>,
    xcb_query_colors_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, pixels_len: u32) -> c_int>,
    xcb_query_colors: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cmap: xcb_colormap_t,
            pixels_len: u32,
            pixels: *const u32,
        ) -> xcb_query_colors_cookie_t,
    >,
    xcb_query_colors_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cmap: xcb_colormap_t,
            pixels_len: u32,
            pixels: *const u32,
        ) -> xcb_query_colors_cookie_t,
    >,
    xcb_query_colors_colors:
        LazySymbol<unsafe fn(r: *const xcb_query_colors_reply_t) -> *mut xcb_rgb_t>,
    xcb_query_colors_colors_length:
        LazySymbol<unsafe fn(r: *const xcb_query_colors_reply_t) -> c_int>,
    xcb_query_colors_colors_iterator:
        LazySymbol<unsafe fn(r: *const xcb_query_colors_reply_t) -> xcb_rgb_iterator_t>,
    xcb_query_colors_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_query_colors_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_query_colors_reply_t,
    >,
    xcb_lookup_color_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_lookup_color: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cmap: xcb_colormap_t,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_lookup_color_cookie_t,
    >,
    xcb_lookup_color_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cmap: xcb_colormap_t,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_lookup_color_cookie_t,
    >,
    xcb_lookup_color_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_lookup_color_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_lookup_color_reply_t,
    >,
    xcb_create_cursor_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cid: xcb_cursor_t,
            source: xcb_pixmap_t,
            mask: xcb_pixmap_t,
            fore_red: u16,
            fore_green: u16,
            fore_blue: u16,
            back_red: u16,
            back_green: u16,
            back_blue: u16,
            x: u16,
            y: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_cursor: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cid: xcb_cursor_t,
            source: xcb_pixmap_t,
            mask: xcb_pixmap_t,
            fore_red: u16,
            fore_green: u16,
            fore_blue: u16,
            back_red: u16,
            back_green: u16,
            back_blue: u16,
            x: u16,
            y: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_glyph_cursor_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cid: xcb_cursor_t,
            source_font: xcb_font_t,
            mask_font: xcb_font_t,
            source_char: u16,
            mask_char: u16,
            fore_red: u16,
            fore_green: u16,
            fore_blue: u16,
            back_red: u16,
            back_green: u16,
            back_blue: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_glyph_cursor: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cid: xcb_cursor_t,
            source_font: xcb_font_t,
            mask_font: xcb_font_t,
            source_char: u16,
            mask_char: u16,
            fore_red: u16,
            fore_green: u16,
            fore_blue: u16,
            back_red: u16,
            back_green: u16,
            back_blue: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_free_cursor_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cursor: xcb_cursor_t) -> xcb_void_cookie_t>,
    xcb_free_cursor:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cursor: xcb_cursor_t) -> xcb_void_cookie_t>,
    xcb_recolor_cursor_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cursor: xcb_cursor_t,
            fore_red: u16,
            fore_green: u16,
            fore_blue: u16,
            back_red: u16,
            back_green: u16,
            back_blue: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_recolor_cursor: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cursor: xcb_cursor_t,
            fore_red: u16,
            fore_green: u16,
            fore_blue: u16,
            back_red: u16,
            back_green: u16,
            back_blue: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_query_best_size: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            class: u8,
            drawable: xcb_drawable_t,
            width: u16,
            height: u16,
        ) -> xcb_query_best_size_cookie_t,
    >,
    xcb_query_best_size_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            class: u8,
            drawable: xcb_drawable_t,
            width: u16,
            height: u16,
        ) -> xcb_query_best_size_cookie_t,
    >,
    xcb_query_best_size_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_query_best_size_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_query_best_size_reply_t,
    >,
    xcb_query_extension_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_query_extension: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_query_extension_cookie_t,
    >,
    xcb_query_extension_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_query_extension_cookie_t,
    >,
    xcb_query_extension_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_query_extension_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_query_extension_reply_t,
    >,
    xcb_list_extensions_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_list_extensions:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_list_extensions_cookie_t>,
    xcb_list_extensions_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_list_extensions_cookie_t>,
    xcb_list_extensions_names_length:
        LazySymbol<unsafe fn(r: *const xcb_list_extensions_reply_t) -> c_int>,
    xcb_list_extensions_names_iterator:
        LazySymbol<unsafe fn(r: *const xcb_list_extensions_reply_t) -> xcb_str_iterator_t>,
    xcb_list_extensions_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_list_extensions_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_list_extensions_reply_t,
    >,
    xcb_change_keyboard_mapping_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_change_keyboard_mapping_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            keycode_count: u8,
            first_keycode: xcb_keycode_t,
            keysyms_per_keycode: u8,
            keysyms: *const xcb_keysym_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_keyboard_mapping: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            keycode_count: u8,
            first_keycode: xcb_keycode_t,
            keysyms_per_keycode: u8,
            keysyms: *const xcb_keysym_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_keyboard_mapping_keysyms:
        LazySymbol<unsafe fn(r: *const xcb_change_keyboard_mapping_request_t) -> *mut xcb_keysym_t>,
    xcb_change_keyboard_mapping_keysyms_length:
        LazySymbol<unsafe fn(r: *const xcb_change_keyboard_mapping_request_t) -> c_int>,
    xcb_change_keyboard_mapping_keysyms_end: LazySymbol<
        unsafe fn(r: *const xcb_change_keyboard_mapping_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_get_keyboard_mapping_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_get_keyboard_mapping: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            first_keycode: xcb_keycode_t,
            count: u8,
        ) -> xcb_get_keyboard_mapping_cookie_t,
    >,
    xcb_get_keyboard_mapping_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            first_keycode: xcb_keycode_t,
            count: u8,
        ) -> xcb_get_keyboard_mapping_cookie_t,
    >,
    xcb_get_keyboard_mapping_keysyms:
        LazySymbol<unsafe fn(r: *const xcb_get_keyboard_mapping_reply_t) -> *mut xcb_keysym_t>,
    xcb_get_keyboard_mapping_keysyms_length:
        LazySymbol<unsafe fn(r: *const xcb_get_keyboard_mapping_reply_t) -> c_int>,
    xcb_get_keyboard_mapping_keysyms_end:
        LazySymbol<unsafe fn(r: *const xcb_get_keyboard_mapping_reply_t) -> xcb_generic_iterator_t>,
    xcb_get_keyboard_mapping_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_keyboard_mapping_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_get_keyboard_mapping_reply_t,
    >,
    xcb_change_keyboard_control_value_list_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            value_mask: u32,
            _aux: *const xcb_change_keyboard_control_value_list_t,
        ) -> c_int,
    >,
    xcb_change_keyboard_control_value_list_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            value_mask: u32,
            _aux: *mut xcb_change_keyboard_control_value_list_t,
        ) -> c_int,
    >,
    xcb_change_keyboard_control_value_list_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, value_mask: u32) -> c_int>,
    xcb_change_keyboard_control_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_change_keyboard_control_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_keyboard_control: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_keyboard_control_aux_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            value_mask: u32,
            value_list: *const xcb_change_keyboard_control_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_keyboard_control_aux: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            value_mask: u32,
            value_list: *const xcb_change_keyboard_control_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_keyboard_control_value_list:
        LazySymbol<unsafe fn(r: *const xcb_change_keyboard_control_request_t) -> *mut c_void>,
    xcb_get_keyboard_control:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_keyboard_control_cookie_t>,
    xcb_get_keyboard_control_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_keyboard_control_cookie_t>,
    xcb_get_keyboard_control_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_keyboard_control_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_get_keyboard_control_reply_t,
    >,
    xcb_bell_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, percent: i8) -> xcb_void_cookie_t>,
    xcb_bell: LazySymbol<unsafe fn(c: *mut xcb_connection_t, percent: i8) -> xcb_void_cookie_t>,
    xcb_change_pointer_control_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            acceleration_numerator: i16,
            acceleration_denominator: i16,
            threshold: i16,
            do_acceleration: u8,
            do_threshold: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_pointer_control: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            acceleration_numerator: i16,
            acceleration_denominator: i16,
            threshold: i16,
            do_acceleration: u8,
            do_threshold: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_get_pointer_control:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_pointer_control_cookie_t>,
    xcb_get_pointer_control_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_pointer_control_cookie_t>,
    xcb_get_pointer_control_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_pointer_control_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_get_pointer_control_reply_t,
    >,
    xcb_set_screen_saver_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            timeout: i16,
            interval: i16,
            prefer_blanking: u8,
            allow_exposures: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_set_screen_saver: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            timeout: i16,
            interval: i16,
            prefer_blanking: u8,
            allow_exposures: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_get_screen_saver:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_screen_saver_cookie_t>,
    xcb_get_screen_saver_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_screen_saver_cookie_t>,
    xcb_get_screen_saver_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_screen_saver_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_get_screen_saver_reply_t,
    >,
    xcb_change_hosts_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_change_hosts_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            mode: u8,
            family: u8,
            address_len: u16,
            address: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_hosts: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            mode: u8,
            family: u8,
            address_len: u16,
            address: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_hosts_address:
        LazySymbol<unsafe fn(r: *const xcb_change_hosts_request_t) -> *mut u8>,
    xcb_change_hosts_address_length:
        LazySymbol<unsafe fn(r: *const xcb_change_hosts_request_t) -> c_int>,
    xcb_change_hosts_address_end:
        LazySymbol<unsafe fn(r: *const xcb_change_hosts_request_t) -> xcb_generic_iterator_t>,
    xcb_host_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_host_address: LazySymbol<unsafe fn(r: *const xcb_host_t) -> *mut u8>,
    xcb_host_address_length: LazySymbol<unsafe fn(r: *const xcb_host_t) -> c_int>,
    xcb_host_address_end: LazySymbol<unsafe fn(r: *const xcb_host_t) -> xcb_generic_iterator_t>,
    xcb_host_next: LazySymbol<unsafe fn(i: *mut xcb_host_iterator_t)>,
    xcb_host_end: LazySymbol<unsafe fn(i: xcb_host_iterator_t) -> xcb_generic_iterator_t>,
    xcb_list_hosts_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_list_hosts: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_list_hosts_cookie_t>,
    xcb_list_hosts_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_list_hosts_cookie_t>,
    xcb_list_hosts_hosts_length: LazySymbol<unsafe fn(r: *const xcb_list_hosts_reply_t) -> c_int>,
    xcb_list_hosts_hosts_iterator:
        LazySymbol<unsafe fn(r: *const xcb_list_hosts_reply_t) -> xcb_host_iterator_t>,
    xcb_list_hosts_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_list_hosts_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_list_hosts_reply_t,
    >,
    xcb_set_access_control_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, mode: u8) -> xcb_void_cookie_t>,
    xcb_set_access_control:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, mode: u8) -> xcb_void_cookie_t>,
    xcb_set_close_down_mode_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, mode: u8) -> xcb_void_cookie_t>,
    xcb_set_close_down_mode:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, mode: u8) -> xcb_void_cookie_t>,
    xcb_kill_client_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, resource: u32) -> xcb_void_cookie_t>,
    xcb_kill_client:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, resource: u32) -> xcb_void_cookie_t>,
    xcb_rotate_properties_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_rotate_properties_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            atoms_len: u16,
            delta: i16,
            atoms: *const xcb_atom_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_rotate_properties: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            atoms_len: u16,
            delta: i16,
            atoms: *const xcb_atom_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_rotate_properties_atoms:
        LazySymbol<unsafe fn(r: *const xcb_rotate_properties_request_t) -> *mut xcb_atom_t>,
    xcb_rotate_properties_atoms_length:
        LazySymbol<unsafe fn(r: *const xcb_rotate_properties_request_t) -> c_int>,
    xcb_rotate_properties_atoms_end:
        LazySymbol<unsafe fn(r: *const xcb_rotate_properties_request_t) -> xcb_generic_iterator_t>,
    xcb_force_screen_saver_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, mode: u8) -> xcb_void_cookie_t>,
    xcb_force_screen_saver:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, mode: u8) -> xcb_void_cookie_t>,
    xcb_set_pointer_mapping_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_set_pointer_mapping: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            map_len: u8,
            map: *const u8,
        ) -> xcb_set_pointer_mapping_cookie_t,
    >,
    xcb_set_pointer_mapping_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            map_len: u8,
            map: *const u8,
        ) -> xcb_set_pointer_mapping_cookie_t,
    >,
    xcb_set_pointer_mapping_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_set_pointer_mapping_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_set_pointer_mapping_reply_t,
    >,
    xcb_get_pointer_mapping_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_get_pointer_mapping:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_pointer_mapping_cookie_t>,
    xcb_get_pointer_mapping_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_pointer_mapping_cookie_t>,
    xcb_get_pointer_mapping_map:
        LazySymbol<unsafe fn(r: *const xcb_get_pointer_mapping_reply_t) -> *mut u8>,
    xcb_get_pointer_mapping_map_length:
        LazySymbol<unsafe fn(r: *const xcb_get_pointer_mapping_reply_t) -> c_int>,
    xcb_get_pointer_mapping_map_end:
        LazySymbol<unsafe fn(r: *const xcb_get_pointer_mapping_reply_t) -> xcb_generic_iterator_t>,
    xcb_get_pointer_mapping_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_pointer_mapping_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_get_pointer_mapping_reply_t,
    >,
    xcb_set_modifier_mapping_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_set_modifier_mapping: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            keycodes_per_modifier: u8,
            keycodes: *const xcb_keycode_t,
        ) -> xcb_set_modifier_mapping_cookie_t,
    >,
    xcb_set_modifier_mapping_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            keycodes_per_modifier: u8,
            keycodes: *const xcb_keycode_t,
        ) -> xcb_set_modifier_mapping_cookie_t,
    >,
    xcb_set_modifier_mapping_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_set_modifier_mapping_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_set_modifier_mapping_reply_t,
    >,
    xcb_get_modifier_mapping_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_get_modifier_mapping:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_modifier_mapping_cookie_t>,
    xcb_get_modifier_mapping_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_modifier_mapping_cookie_t>,
    xcb_get_modifier_mapping_keycodes:
        LazySymbol<unsafe fn(r: *const xcb_get_modifier_mapping_reply_t) -> *mut xcb_keycode_t>,
    xcb_get_modifier_mapping_keycodes_length:
        LazySymbol<unsafe fn(r: *const xcb_get_modifier_mapping_reply_t) -> c_int>,
    xcb_get_modifier_mapping_keycodes_end:
        LazySymbol<unsafe fn(r: *const xcb_get_modifier_mapping_reply_t) -> xcb_generic_iterator_t>,
    xcb_get_modifier_mapping_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_modifier_mapping_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_get_modifier_mapping_reply_t,
    >,
    xcb_no_operation_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    xcb_no_operation: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self
            .xproto
            .$f
            .get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .xproto
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

impl Xcb {
    pub unsafe fn xcb_char2b_next(&self, i: *mut xcb_char2b_iterator_t) {
        sym!(self, xcb_char2b_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_char2b_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_char2b_next(&self) -> bool {
        has_sym!(self, xcb_char2b_next)
    }

    pub unsafe fn xcb_char2b_end(&self, i: xcb_char2b_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_char2b_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_char2b_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_char2b_end(&self) -> bool {
        has_sym!(self, xcb_char2b_end)
    }

    pub unsafe fn xcb_window_next(&self, i: *mut xcb_window_iterator_t) {
        sym!(self, xcb_window_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_window_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_window_next(&self) -> bool {
        has_sym!(self, xcb_window_next)
    }

    pub unsafe fn xcb_window_end(&self, i: xcb_window_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_window_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_window_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_window_end(&self) -> bool {
        has_sym!(self, xcb_window_end)
    }

    pub unsafe fn xcb_pixmap_next(&self, i: *mut xcb_pixmap_iterator_t) {
        sym!(self, xcb_pixmap_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_pixmap_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_pixmap_next(&self) -> bool {
        has_sym!(self, xcb_pixmap_next)
    }

    pub unsafe fn xcb_pixmap_end(&self, i: xcb_pixmap_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_pixmap_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_pixmap_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_pixmap_end(&self) -> bool {
        has_sym!(self, xcb_pixmap_end)
    }

    pub unsafe fn xcb_cursor_next(&self, i: *mut xcb_cursor_iterator_t) {
        sym!(self, xcb_cursor_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_cursor_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_cursor_next(&self) -> bool {
        has_sym!(self, xcb_cursor_next)
    }

    pub unsafe fn xcb_cursor_end(&self, i: xcb_cursor_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_cursor_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_cursor_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_cursor_end(&self) -> bool {
        has_sym!(self, xcb_cursor_end)
    }

    pub unsafe fn xcb_font_next(&self, i: *mut xcb_font_iterator_t) {
        sym!(self, xcb_font_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_font_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_font_next(&self) -> bool {
        has_sym!(self, xcb_font_next)
    }

    pub unsafe fn xcb_font_end(&self, i: xcb_font_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_font_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_font_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_font_end(&self) -> bool {
        has_sym!(self, xcb_font_end)
    }

    pub unsafe fn xcb_gcontext_next(&self, i: *mut xcb_gcontext_iterator_t) {
        sym!(self, xcb_gcontext_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_gcontext_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_gcontext_next(&self) -> bool {
        has_sym!(self, xcb_gcontext_next)
    }

    pub unsafe fn xcb_gcontext_end(&self, i: xcb_gcontext_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_gcontext_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_gcontext_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_gcontext_end(&self) -> bool {
        has_sym!(self, xcb_gcontext_end)
    }

    pub unsafe fn xcb_colormap_next(&self, i: *mut xcb_colormap_iterator_t) {
        sym!(self, xcb_colormap_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_colormap_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_colormap_next(&self) -> bool {
        has_sym!(self, xcb_colormap_next)
    }

    pub unsafe fn xcb_colormap_end(&self, i: xcb_colormap_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_colormap_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_colormap_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_colormap_end(&self) -> bool {
        has_sym!(self, xcb_colormap_end)
    }

    pub unsafe fn xcb_atom_next(&self, i: *mut xcb_atom_iterator_t) {
        sym!(self, xcb_atom_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_atom_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_atom_next(&self) -> bool {
        has_sym!(self, xcb_atom_next)
    }

    pub unsafe fn xcb_atom_end(&self, i: xcb_atom_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_atom_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_atom_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_atom_end(&self) -> bool {
        has_sym!(self, xcb_atom_end)
    }

    pub unsafe fn xcb_drawable_next(&self, i: *mut xcb_drawable_iterator_t) {
        sym!(self, xcb_drawable_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_drawable_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_drawable_next(&self) -> bool {
        has_sym!(self, xcb_drawable_next)
    }

    pub unsafe fn xcb_drawable_end(&self, i: xcb_drawable_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_drawable_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_drawable_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_drawable_end(&self) -> bool {
        has_sym!(self, xcb_drawable_end)
    }

    pub unsafe fn xcb_fontable_next(&self, i: *mut xcb_fontable_iterator_t) {
        sym!(self, xcb_fontable_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_fontable_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_fontable_next(&self) -> bool {
        has_sym!(self, xcb_fontable_next)
    }

    pub unsafe fn xcb_fontable_end(&self, i: xcb_fontable_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_fontable_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_fontable_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_fontable_end(&self) -> bool {
        has_sym!(self, xcb_fontable_end)
    }

    pub unsafe fn xcb_bool32_next(&self, i: *mut xcb_bool32_iterator_t) {
        sym!(self, xcb_bool32_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_bool32_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_bool32_next(&self) -> bool {
        has_sym!(self, xcb_bool32_next)
    }

    pub unsafe fn xcb_bool32_end(&self, i: xcb_bool32_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_bool32_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_bool32_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_bool32_end(&self) -> bool {
        has_sym!(self, xcb_bool32_end)
    }

    pub unsafe fn xcb_visualid_next(&self, i: *mut xcb_visualid_iterator_t) {
        sym!(self, xcb_visualid_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_visualid_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_visualid_next(&self) -> bool {
        has_sym!(self, xcb_visualid_next)
    }

    pub unsafe fn xcb_visualid_end(&self, i: xcb_visualid_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_visualid_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_visualid_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_visualid_end(&self) -> bool {
        has_sym!(self, xcb_visualid_end)
    }

    pub unsafe fn xcb_timestamp_next(&self, i: *mut xcb_timestamp_iterator_t) {
        sym!(self, xcb_timestamp_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_timestamp_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_timestamp_next(&self) -> bool {
        has_sym!(self, xcb_timestamp_next)
    }

    pub unsafe fn xcb_timestamp_end(&self, i: xcb_timestamp_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_timestamp_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_timestamp_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_timestamp_end(&self) -> bool {
        has_sym!(self, xcb_timestamp_end)
    }

    pub unsafe fn xcb_keysym_next(&self, i: *mut xcb_keysym_iterator_t) {
        sym!(self, xcb_keysym_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_keysym_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_keysym_next(&self) -> bool {
        has_sym!(self, xcb_keysym_next)
    }

    pub unsafe fn xcb_keysym_end(&self, i: xcb_keysym_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_keysym_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_keysym_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_keysym_end(&self) -> bool {
        has_sym!(self, xcb_keysym_end)
    }

    pub unsafe fn xcb_keycode_next(&self, i: *mut xcb_keycode_iterator_t) {
        sym!(self, xcb_keycode_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_keycode_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_keycode_next(&self) -> bool {
        has_sym!(self, xcb_keycode_next)
    }

    pub unsafe fn xcb_keycode_end(&self, i: xcb_keycode_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_keycode_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_keycode_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_keycode_end(&self) -> bool {
        has_sym!(self, xcb_keycode_end)
    }

    pub unsafe fn xcb_keycode32_next(&self, i: *mut xcb_keycode32_iterator_t) {
        sym!(self, xcb_keycode32_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_keycode32_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_keycode32_next(&self) -> bool {
        has_sym!(self, xcb_keycode32_next)
    }

    pub unsafe fn xcb_keycode32_end(&self, i: xcb_keycode32_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_keycode32_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_keycode32_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_keycode32_end(&self) -> bool {
        has_sym!(self, xcb_keycode32_end)
    }

    pub unsafe fn xcb_button_next(&self, i: *mut xcb_button_iterator_t) {
        sym!(self, xcb_button_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_button_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_button_next(&self) -> bool {
        has_sym!(self, xcb_button_next)
    }

    pub unsafe fn xcb_button_end(&self, i: xcb_button_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_button_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_button_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_button_end(&self) -> bool {
        has_sym!(self, xcb_button_end)
    }

    pub unsafe fn xcb_point_next(&self, i: *mut xcb_point_iterator_t) {
        sym!(self, xcb_point_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_point_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_point_next(&self) -> bool {
        has_sym!(self, xcb_point_next)
    }

    pub unsafe fn xcb_point_end(&self, i: xcb_point_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_point_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_point_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_point_end(&self) -> bool {
        has_sym!(self, xcb_point_end)
    }

    pub unsafe fn xcb_rectangle_next(&self, i: *mut xcb_rectangle_iterator_t) {
        sym!(self, xcb_rectangle_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_rectangle_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_rectangle_next(&self) -> bool {
        has_sym!(self, xcb_rectangle_next)
    }

    pub unsafe fn xcb_rectangle_end(&self, i: xcb_rectangle_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_rectangle_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_rectangle_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_rectangle_end(&self) -> bool {
        has_sym!(self, xcb_rectangle_end)
    }

    pub unsafe fn xcb_arc_next(&self, i: *mut xcb_arc_iterator_t) {
        sym!(self, xcb_arc_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_arc_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_arc_next(&self) -> bool {
        has_sym!(self, xcb_arc_next)
    }

    pub unsafe fn xcb_arc_end(&self, i: xcb_arc_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_arc_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_arc_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_arc_end(&self) -> bool {
        has_sym!(self, xcb_arc_end)
    }

    pub unsafe fn xcb_format_next(&self, i: *mut xcb_format_iterator_t) {
        sym!(self, xcb_format_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_format_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_format_next(&self) -> bool {
        has_sym!(self, xcb_format_next)
    }

    pub unsafe fn xcb_format_end(&self, i: xcb_format_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_format_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_format_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_format_end(&self) -> bool {
        has_sym!(self, xcb_format_end)
    }

    pub unsafe fn xcb_visualtype_next(&self, i: *mut xcb_visualtype_iterator_t) {
        sym!(self, xcb_visualtype_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_visualtype_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_visualtype_next(&self) -> bool {
        has_sym!(self, xcb_visualtype_next)
    }

    pub unsafe fn xcb_visualtype_end(
        &self,
        i: xcb_visualtype_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_visualtype_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_visualtype_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_visualtype_end(&self) -> bool {
        has_sym!(self, xcb_visualtype_end)
    }

    pub unsafe fn xcb_depth_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_depth_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_depth_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_depth_sizeof(&self) -> bool {
        has_sym!(self, xcb_depth_sizeof)
    }

    pub unsafe fn xcb_depth_visuals(&self, r: *const xcb_depth_t) -> *mut xcb_visualtype_t {
        sym!(self, xcb_depth_visuals)(r)
    }

    /// Returns `true` iff the symbol `xcb_depth_visuals` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_depth_visuals(&self) -> bool {
        has_sym!(self, xcb_depth_visuals)
    }

    pub unsafe fn xcb_depth_visuals_length(&self, r: *const xcb_depth_t) -> c_int {
        sym!(self, xcb_depth_visuals_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_depth_visuals_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_depth_visuals_length(&self) -> bool {
        has_sym!(self, xcb_depth_visuals_length)
    }

    pub unsafe fn xcb_depth_visuals_iterator(
        &self,
        r: *const xcb_depth_t,
    ) -> xcb_visualtype_iterator_t {
        sym!(self, xcb_depth_visuals_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_depth_visuals_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_depth_visuals_iterator(&self) -> bool {
        has_sym!(self, xcb_depth_visuals_iterator)
    }

    pub unsafe fn xcb_depth_next(&self, i: *mut xcb_depth_iterator_t) {
        sym!(self, xcb_depth_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_depth_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_depth_next(&self) -> bool {
        has_sym!(self, xcb_depth_next)
    }

    pub unsafe fn xcb_depth_end(&self, i: xcb_depth_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_depth_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_depth_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_depth_end(&self) -> bool {
        has_sym!(self, xcb_depth_end)
    }

    pub unsafe fn xcb_screen_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_screen_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_screen_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screen_sizeof(&self) -> bool {
        has_sym!(self, xcb_screen_sizeof)
    }

    pub unsafe fn xcb_screen_allowed_depths_length(&self, r: *const xcb_screen_t) -> c_int {
        sym!(self, xcb_screen_allowed_depths_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_screen_allowed_depths_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screen_allowed_depths_length(&self) -> bool {
        has_sym!(self, xcb_screen_allowed_depths_length)
    }

    pub unsafe fn xcb_screen_allowed_depths_iterator(
        &self,
        r: *const xcb_screen_t,
    ) -> xcb_depth_iterator_t {
        sym!(self, xcb_screen_allowed_depths_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_screen_allowed_depths_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screen_allowed_depths_iterator(&self) -> bool {
        has_sym!(self, xcb_screen_allowed_depths_iterator)
    }

    pub unsafe fn xcb_screen_next(&self, i: *mut xcb_screen_iterator_t) {
        sym!(self, xcb_screen_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_screen_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screen_next(&self) -> bool {
        has_sym!(self, xcb_screen_next)
    }

    pub unsafe fn xcb_screen_end(&self, i: xcb_screen_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_screen_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_screen_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screen_end(&self) -> bool {
        has_sym!(self, xcb_screen_end)
    }

    pub unsafe fn xcb_setup_request_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_setup_request_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_setup_request_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_request_sizeof(&self) -> bool {
        has_sym!(self, xcb_setup_request_sizeof)
    }

    pub unsafe fn xcb_setup_request_authorization_protocol_name(
        &self,
        r: *const xcb_setup_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_setup_request_authorization_protocol_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_request_authorization_protocol_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_request_authorization_protocol_name(&self) -> bool {
        has_sym!(self, xcb_setup_request_authorization_protocol_name)
    }

    pub unsafe fn xcb_setup_request_authorization_protocol_name_length(
        &self,
        r: *const xcb_setup_request_t,
    ) -> c_int {
        sym!(self, xcb_setup_request_authorization_protocol_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_request_authorization_protocol_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_request_authorization_protocol_name_length(&self) -> bool {
        has_sym!(self, xcb_setup_request_authorization_protocol_name_length)
    }

    pub unsafe fn xcb_setup_request_authorization_protocol_name_end(
        &self,
        r: *const xcb_setup_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_setup_request_authorization_protocol_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_request_authorization_protocol_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_request_authorization_protocol_name_end(&self) -> bool {
        has_sym!(self, xcb_setup_request_authorization_protocol_name_end)
    }

    pub unsafe fn xcb_setup_request_authorization_protocol_data(
        &self,
        r: *const xcb_setup_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_setup_request_authorization_protocol_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_request_authorization_protocol_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_request_authorization_protocol_data(&self) -> bool {
        has_sym!(self, xcb_setup_request_authorization_protocol_data)
    }

    pub unsafe fn xcb_setup_request_authorization_protocol_data_length(
        &self,
        r: *const xcb_setup_request_t,
    ) -> c_int {
        sym!(self, xcb_setup_request_authorization_protocol_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_request_authorization_protocol_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_request_authorization_protocol_data_length(&self) -> bool {
        has_sym!(self, xcb_setup_request_authorization_protocol_data_length)
    }

    pub unsafe fn xcb_setup_request_authorization_protocol_data_end(
        &self,
        r: *const xcb_setup_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_setup_request_authorization_protocol_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_request_authorization_protocol_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_request_authorization_protocol_data_end(&self) -> bool {
        has_sym!(self, xcb_setup_request_authorization_protocol_data_end)
    }

    pub unsafe fn xcb_setup_request_next(&self, i: *mut xcb_setup_request_iterator_t) {
        sym!(self, xcb_setup_request_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_setup_request_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_request_next(&self) -> bool {
        has_sym!(self, xcb_setup_request_next)
    }

    pub unsafe fn xcb_setup_request_end(
        &self,
        i: xcb_setup_request_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_setup_request_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_setup_request_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_request_end(&self) -> bool {
        has_sym!(self, xcb_setup_request_end)
    }

    pub unsafe fn xcb_setup_failed_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_setup_failed_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_setup_failed_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_failed_sizeof(&self) -> bool {
        has_sym!(self, xcb_setup_failed_sizeof)
    }

    pub unsafe fn xcb_setup_failed_reason(&self, r: *const xcb_setup_failed_t) -> *mut c_char {
        sym!(self, xcb_setup_failed_reason)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_failed_reason` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_failed_reason(&self) -> bool {
        has_sym!(self, xcb_setup_failed_reason)
    }

    pub unsafe fn xcb_setup_failed_reason_length(&self, r: *const xcb_setup_failed_t) -> c_int {
        sym!(self, xcb_setup_failed_reason_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_failed_reason_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_failed_reason_length(&self) -> bool {
        has_sym!(self, xcb_setup_failed_reason_length)
    }

    pub unsafe fn xcb_setup_failed_reason_end(
        &self,
        r: *const xcb_setup_failed_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_setup_failed_reason_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_failed_reason_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_failed_reason_end(&self) -> bool {
        has_sym!(self, xcb_setup_failed_reason_end)
    }

    pub unsafe fn xcb_setup_failed_next(&self, i: *mut xcb_setup_failed_iterator_t) {
        sym!(self, xcb_setup_failed_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_setup_failed_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_failed_next(&self) -> bool {
        has_sym!(self, xcb_setup_failed_next)
    }

    pub unsafe fn xcb_setup_failed_end(
        &self,
        i: xcb_setup_failed_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_setup_failed_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_setup_failed_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_failed_end(&self) -> bool {
        has_sym!(self, xcb_setup_failed_end)
    }

    pub unsafe fn xcb_setup_authenticate_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_setup_authenticate_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_setup_authenticate_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_authenticate_sizeof(&self) -> bool {
        has_sym!(self, xcb_setup_authenticate_sizeof)
    }

    pub unsafe fn xcb_setup_authenticate_reason(
        &self,
        r: *const xcb_setup_authenticate_t,
    ) -> *mut c_char {
        sym!(self, xcb_setup_authenticate_reason)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_authenticate_reason` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_authenticate_reason(&self) -> bool {
        has_sym!(self, xcb_setup_authenticate_reason)
    }

    pub unsafe fn xcb_setup_authenticate_reason_length(
        &self,
        r: *const xcb_setup_authenticate_t,
    ) -> c_int {
        sym!(self, xcb_setup_authenticate_reason_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_authenticate_reason_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_authenticate_reason_length(&self) -> bool {
        has_sym!(self, xcb_setup_authenticate_reason_length)
    }

    pub unsafe fn xcb_setup_authenticate_reason_end(
        &self,
        r: *const xcb_setup_authenticate_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_setup_authenticate_reason_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_authenticate_reason_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_authenticate_reason_end(&self) -> bool {
        has_sym!(self, xcb_setup_authenticate_reason_end)
    }

    pub unsafe fn xcb_setup_authenticate_next(&self, i: *mut xcb_setup_authenticate_iterator_t) {
        sym!(self, xcb_setup_authenticate_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_setup_authenticate_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_authenticate_next(&self) -> bool {
        has_sym!(self, xcb_setup_authenticate_next)
    }

    pub unsafe fn xcb_setup_authenticate_end(
        &self,
        i: xcb_setup_authenticate_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_setup_authenticate_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_setup_authenticate_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_authenticate_end(&self) -> bool {
        has_sym!(self, xcb_setup_authenticate_end)
    }

    pub unsafe fn xcb_setup_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_setup_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_setup_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_sizeof(&self) -> bool {
        has_sym!(self, xcb_setup_sizeof)
    }

    pub unsafe fn xcb_setup_vendor(&self, r: *const xcb_setup_t) -> *mut c_char {
        sym!(self, xcb_setup_vendor)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_vendor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_vendor(&self) -> bool {
        has_sym!(self, xcb_setup_vendor)
    }

    pub unsafe fn xcb_setup_vendor_length(&self, r: *const xcb_setup_t) -> c_int {
        sym!(self, xcb_setup_vendor_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_vendor_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_vendor_length(&self) -> bool {
        has_sym!(self, xcb_setup_vendor_length)
    }

    pub unsafe fn xcb_setup_vendor_end(&self, r: *const xcb_setup_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_setup_vendor_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_vendor_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_vendor_end(&self) -> bool {
        has_sym!(self, xcb_setup_vendor_end)
    }

    pub unsafe fn xcb_setup_pixmap_formats(&self, r: *const xcb_setup_t) -> *mut xcb_format_t {
        sym!(self, xcb_setup_pixmap_formats)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_pixmap_formats` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_pixmap_formats(&self) -> bool {
        has_sym!(self, xcb_setup_pixmap_formats)
    }

    pub unsafe fn xcb_setup_pixmap_formats_length(&self, r: *const xcb_setup_t) -> c_int {
        sym!(self, xcb_setup_pixmap_formats_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_pixmap_formats_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_pixmap_formats_length(&self) -> bool {
        has_sym!(self, xcb_setup_pixmap_formats_length)
    }

    pub unsafe fn xcb_setup_pixmap_formats_iterator(
        &self,
        r: *const xcb_setup_t,
    ) -> xcb_format_iterator_t {
        sym!(self, xcb_setup_pixmap_formats_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_pixmap_formats_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_pixmap_formats_iterator(&self) -> bool {
        has_sym!(self, xcb_setup_pixmap_formats_iterator)
    }

    pub unsafe fn xcb_setup_roots_length(&self, r: *const xcb_setup_t) -> c_int {
        sym!(self, xcb_setup_roots_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_roots_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_roots_length(&self) -> bool {
        has_sym!(self, xcb_setup_roots_length)
    }

    pub unsafe fn xcb_setup_roots_iterator(&self, r: *const xcb_setup_t) -> xcb_screen_iterator_t {
        sym!(self, xcb_setup_roots_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_roots_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_roots_iterator(&self) -> bool {
        has_sym!(self, xcb_setup_roots_iterator)
    }

    pub unsafe fn xcb_setup_next(&self, i: *mut xcb_setup_iterator_t) {
        sym!(self, xcb_setup_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_setup_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_next(&self) -> bool {
        has_sym!(self, xcb_setup_next)
    }

    pub unsafe fn xcb_setup_end(&self, i: xcb_setup_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_setup_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_setup_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_end(&self) -> bool {
        has_sym!(self, xcb_setup_end)
    }

    pub unsafe fn xcb_client_message_data_next(&self, i: *mut xcb_client_message_data_iterator_t) {
        sym!(self, xcb_client_message_data_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_client_message_data_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_client_message_data_next(&self) -> bool {
        has_sym!(self, xcb_client_message_data_next)
    }

    pub unsafe fn xcb_client_message_data_end(
        &self,
        i: xcb_client_message_data_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_client_message_data_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_client_message_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_client_message_data_end(&self) -> bool {
        has_sym!(self, xcb_client_message_data_end)
    }

    pub unsafe fn xcb_create_window_value_list_serialize(
        &self,
        _buffer: *mut *mut c_void,
        value_mask: u32,
        _aux: *const xcb_create_window_value_list_t,
    ) -> c_int {
        sym!(self, xcb_create_window_value_list_serialize)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_create_window_value_list_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_window_value_list_serialize(&self) -> bool {
        has_sym!(self, xcb_create_window_value_list_serialize)
    }

    pub unsafe fn xcb_create_window_value_list_unpack(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
        _aux: *mut xcb_create_window_value_list_t,
    ) -> c_int {
        sym!(self, xcb_create_window_value_list_unpack)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_create_window_value_list_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_window_value_list_unpack(&self) -> bool {
        has_sym!(self, xcb_create_window_value_list_unpack)
    }

    pub unsafe fn xcb_create_window_value_list_sizeof(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
    ) -> c_int {
        sym!(self, xcb_create_window_value_list_sizeof)(_buffer, value_mask)
    }

    /// Returns `true` iff the symbol `xcb_create_window_value_list_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_window_value_list_sizeof(&self) -> bool {
        has_sym!(self, xcb_create_window_value_list_sizeof)
    }

    pub unsafe fn xcb_create_window_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_create_window_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_create_window_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_window_sizeof(&self) -> bool {
        has_sym!(self, xcb_create_window_sizeof)
    }

    /**
     * @brief Creates a window
     *
     * @param c The connection
     * @param depth Specifies the new window's depth (TODO: what unit?).
     * \n
     * The special value `XCB_COPY_FROM_PARENT` means the depth is taken from the
     * \a parent window.
     * @param wid The ID with which you will refer to the new window, created by
     * `xcb_generate_id`.
     * @param parent The parent window of the new window.
     * @param x The X coordinate of the new window.
     * @param y The Y coordinate of the new window.
     * @param width The width of the new window.
     * @param height The height of the new window.
     * @param border_width TODO:
     * \n
     * Must be zero if the \a class is `InputOnly` or a `xcb_match_error_t` occurs.
     * @param class A bitmask of #xcb_window_class_t values.
     * @param class \n
     * @param visual Specifies the id for the new window's visual.
     * \n
     * The special value `XCB_COPY_FROM_PARENT` means the visual is taken from the
     * \a parent window.
     * @param value_mask A bitmask of #xcb_cw_t values.
     * @return A cookie
     *
     * Creates an unmapped window as child of the specified \a parent window. A
     * CreateNotify event will be generated. The new window is placed on top in the
     * stacking order with respect to siblings.
     *
     * The coordinate system has the X axis horizontal and the Y axis vertical with
     * the origin [0, 0] at the upper-left corner. Coordinates are integral, in terms
     * of pixels, and coincide with pixel centers. Each window and pixmap has its own
     * coordinate system. For a window, the origin is inside the border at the inside,
     * upper-left corner.
     *
     * The created window is not yet displayed (mapped), call `xcb_map_window` to
     * display it.
     *
     * The created window will initially use the same cursor as its parent.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_create_window_checked(
        &self,
        c: *mut xcb_connection_t,
        depth: u8,
        wid: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        class: u16,
        visual: xcb_visualid_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_window_checked)(
            c,
            depth,
            wid,
            parent,
            x,
            y,
            width,
            height,
            border_width,
            class,
            visual,
            value_mask,
            value_list,
        )
    }

    /// Returns `true` iff the symbol `xcb_create_window_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_window_checked(&self) -> bool {
        has_sym!(self, xcb_create_window_checked)
    }

    /**
     * @brief Creates a window
     *
     * @param c The connection
     * @param depth Specifies the new window's depth (TODO: what unit?).
     * \n
     * The special value `XCB_COPY_FROM_PARENT` means the depth is taken from the
     * \a parent window.
     * @param wid The ID with which you will refer to the new window, created by
     * `xcb_generate_id`.
     * @param parent The parent window of the new window.
     * @param x The X coordinate of the new window.
     * @param y The Y coordinate of the new window.
     * @param width The width of the new window.
     * @param height The height of the new window.
     * @param border_width TODO:
     * \n
     * Must be zero if the \a class is `InputOnly` or a `xcb_match_error_t` occurs.
     * @param class A bitmask of #xcb_window_class_t values.
     * @param class \n
     * @param visual Specifies the id for the new window's visual.
     * \n
     * The special value `XCB_COPY_FROM_PARENT` means the visual is taken from the
     * \a parent window.
     * @param value_mask A bitmask of #xcb_cw_t values.
     * @return A cookie
     *
     * Creates an unmapped window as child of the specified \a parent window. A
     * CreateNotify event will be generated. The new window is placed on top in the
     * stacking order with respect to siblings.
     *
     * The coordinate system has the X axis horizontal and the Y axis vertical with
     * the origin [0, 0] at the upper-left corner. Coordinates are integral, in terms
     * of pixels, and coincide with pixel centers. Each window and pixmap has its own
     * coordinate system. For a window, the origin is inside the border at the inside,
     * upper-left corner.
     *
     * The created window is not yet displayed (mapped), call `xcb_map_window` to
     * display it.
     *
     * The created window will initially use the same cursor as its parent.
     *
     */
    pub unsafe fn xcb_create_window(
        &self,
        c: *mut xcb_connection_t,
        depth: u8,
        wid: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        class: u16,
        visual: xcb_visualid_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_window)(
            c,
            depth,
            wid,
            parent,
            x,
            y,
            width,
            height,
            border_width,
            class,
            visual,
            value_mask,
            value_list,
        )
    }

    /// Returns `true` iff the symbol `xcb_create_window` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_window(&self) -> bool {
        has_sym!(self, xcb_create_window)
    }

    /**
     * @brief Creates a window
     *
     * @param c The connection
     * @param depth Specifies the new window's depth (TODO: what unit?).
     * \n
     * The special value `XCB_COPY_FROM_PARENT` means the depth is taken from the
     * \a parent window.
     * @param wid The ID with which you will refer to the new window, created by
     * `xcb_generate_id`.
     * @param parent The parent window of the new window.
     * @param x The X coordinate of the new window.
     * @param y The Y coordinate of the new window.
     * @param width The width of the new window.
     * @param height The height of the new window.
     * @param border_width TODO:
     * \n
     * Must be zero if the \a class is `InputOnly` or a `xcb_match_error_t` occurs.
     * @param class A bitmask of #xcb_window_class_t values.
     * @param class \n
     * @param visual Specifies the id for the new window's visual.
     * \n
     * The special value `XCB_COPY_FROM_PARENT` means the visual is taken from the
     * \a parent window.
     * @param value_mask A bitmask of #xcb_cw_t values.
     * @return A cookie
     *
     * Creates an unmapped window as child of the specified \a parent window. A
     * CreateNotify event will be generated. The new window is placed on top in the
     * stacking order with respect to siblings.
     *
     * The coordinate system has the X axis horizontal and the Y axis vertical with
     * the origin [0, 0] at the upper-left corner. Coordinates are integral, in terms
     * of pixels, and coincide with pixel centers. Each window and pixmap has its own
     * coordinate system. For a window, the origin is inside the border at the inside,
     * upper-left corner.
     *
     * The created window is not yet displayed (mapped), call `xcb_map_window` to
     * display it.
     *
     * The created window will initially use the same cursor as its parent.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_create_window_aux_checked(
        &self,
        c: *mut xcb_connection_t,
        depth: u8,
        wid: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        class: u16,
        visual: xcb_visualid_t,
        value_mask: u32,
        value_list: *const xcb_create_window_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_window_aux_checked)(
            c,
            depth,
            wid,
            parent,
            x,
            y,
            width,
            height,
            border_width,
            class,
            visual,
            value_mask,
            value_list,
        )
    }

    /// Returns `true` iff the symbol `xcb_create_window_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_window_aux_checked(&self) -> bool {
        has_sym!(self, xcb_create_window_aux_checked)
    }

    /**
     * @brief Creates a window
     *
     * @param c The connection
     * @param depth Specifies the new window's depth (TODO: what unit?).
     * \n
     * The special value `XCB_COPY_FROM_PARENT` means the depth is taken from the
     * \a parent window.
     * @param wid The ID with which you will refer to the new window, created by
     * `xcb_generate_id`.
     * @param parent The parent window of the new window.
     * @param x The X coordinate of the new window.
     * @param y The Y coordinate of the new window.
     * @param width The width of the new window.
     * @param height The height of the new window.
     * @param border_width TODO:
     * \n
     * Must be zero if the \a class is `InputOnly` or a `xcb_match_error_t` occurs.
     * @param class A bitmask of #xcb_window_class_t values.
     * @param class \n
     * @param visual Specifies the id for the new window's visual.
     * \n
     * The special value `XCB_COPY_FROM_PARENT` means the visual is taken from the
     * \a parent window.
     * @param value_mask A bitmask of #xcb_cw_t values.
     * @return A cookie
     *
     * Creates an unmapped window as child of the specified \a parent window. A
     * CreateNotify event will be generated. The new window is placed on top in the
     * stacking order with respect to siblings.
     *
     * The coordinate system has the X axis horizontal and the Y axis vertical with
     * the origin [0, 0] at the upper-left corner. Coordinates are integral, in terms
     * of pixels, and coincide with pixel centers. Each window and pixmap has its own
     * coordinate system. For a window, the origin is inside the border at the inside,
     * upper-left corner.
     *
     * The created window is not yet displayed (mapped), call `xcb_map_window` to
     * display it.
     *
     * The created window will initially use the same cursor as its parent.
     *
     */
    pub unsafe fn xcb_create_window_aux(
        &self,
        c: *mut xcb_connection_t,
        depth: u8,
        wid: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        class: u16,
        visual: xcb_visualid_t,
        value_mask: u32,
        value_list: *const xcb_create_window_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_window_aux)(
            c,
            depth,
            wid,
            parent,
            x,
            y,
            width,
            height,
            border_width,
            class,
            visual,
            value_mask,
            value_list,
        )
    }

    /// Returns `true` iff the symbol `xcb_create_window_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_window_aux(&self) -> bool {
        has_sym!(self, xcb_create_window_aux)
    }

    pub unsafe fn xcb_create_window_value_list(
        &self,
        r: *const xcb_create_window_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_create_window_value_list)(r)
    }

    /// Returns `true` iff the symbol `xcb_create_window_value_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_window_value_list(&self) -> bool {
        has_sym!(self, xcb_create_window_value_list)
    }

    pub unsafe fn xcb_change_window_attributes_value_list_serialize(
        &self,
        _buffer: *mut *mut c_void,
        value_mask: u32,
        _aux: *const xcb_change_window_attributes_value_list_t,
    ) -> c_int {
        sym!(self, xcb_change_window_attributes_value_list_serialize)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_change_window_attributes_value_list_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_window_attributes_value_list_serialize(&self) -> bool {
        has_sym!(self, xcb_change_window_attributes_value_list_serialize)
    }

    pub unsafe fn xcb_change_window_attributes_value_list_unpack(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
        _aux: *mut xcb_change_window_attributes_value_list_t,
    ) -> c_int {
        sym!(self, xcb_change_window_attributes_value_list_unpack)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_change_window_attributes_value_list_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_window_attributes_value_list_unpack(&self) -> bool {
        has_sym!(self, xcb_change_window_attributes_value_list_unpack)
    }

    pub unsafe fn xcb_change_window_attributes_value_list_sizeof(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
    ) -> c_int {
        sym!(self, xcb_change_window_attributes_value_list_sizeof)(_buffer, value_mask)
    }

    /// Returns `true` iff the symbol `xcb_change_window_attributes_value_list_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_window_attributes_value_list_sizeof(&self) -> bool {
        has_sym!(self, xcb_change_window_attributes_value_list_sizeof)
    }

    pub unsafe fn xcb_change_window_attributes_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_change_window_attributes_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_change_window_attributes_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_window_attributes_sizeof(&self) -> bool {
        has_sym!(self, xcb_change_window_attributes_sizeof)
    }

    /**
     * @brief change window attributes
     *
     * @param c The connection
     * @param window The window to change.
     * @param value_mask A bitmask of #xcb_cw_t values.
     * @param value_mask \n
     * @param value_list Values for each of the attributes specified in the bitmask \a value_mask. The
     * order has to correspond to the order of possible \a value_mask bits. See the
     * example.
     * @return A cookie
     *
     * Changes the attributes specified by \a value_mask for the specified \a window.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_change_window_attributes_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_window_attributes_checked)(c, window, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_change_window_attributes_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_window_attributes_checked(&self) -> bool {
        has_sym!(self, xcb_change_window_attributes_checked)
    }

    /**
     * @brief change window attributes
     *
     * @param c The connection
     * @param window The window to change.
     * @param value_mask A bitmask of #xcb_cw_t values.
     * @param value_mask \n
     * @param value_list Values for each of the attributes specified in the bitmask \a value_mask. The
     * order has to correspond to the order of possible \a value_mask bits. See the
     * example.
     * @return A cookie
     *
     * Changes the attributes specified by \a value_mask for the specified \a window.
     *
     */
    pub unsafe fn xcb_change_window_attributes(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_window_attributes)(c, window, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_change_window_attributes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_window_attributes(&self) -> bool {
        has_sym!(self, xcb_change_window_attributes)
    }

    /**
     * @brief change window attributes
     *
     * @param c The connection
     * @param window The window to change.
     * @param value_mask A bitmask of #xcb_cw_t values.
     * @param value_mask \n
     * @param value_list Values for each of the attributes specified in the bitmask \a value_mask. The
     * order has to correspond to the order of possible \a value_mask bits. See the
     * example.
     * @return A cookie
     *
     * Changes the attributes specified by \a value_mask for the specified \a window.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_change_window_attributes_aux_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u32,
        value_list: *const xcb_change_window_attributes_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_window_attributes_aux_checked)(c, window, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_change_window_attributes_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_window_attributes_aux_checked(&self) -> bool {
        has_sym!(self, xcb_change_window_attributes_aux_checked)
    }

    /**
     * @brief change window attributes
     *
     * @param c The connection
     * @param window The window to change.
     * @param value_mask A bitmask of #xcb_cw_t values.
     * @param value_mask \n
     * @param value_list Values for each of the attributes specified in the bitmask \a value_mask. The
     * order has to correspond to the order of possible \a value_mask bits. See the
     * example.
     * @return A cookie
     *
     * Changes the attributes specified by \a value_mask for the specified \a window.
     *
     */
    pub unsafe fn xcb_change_window_attributes_aux(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u32,
        value_list: *const xcb_change_window_attributes_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_window_attributes_aux)(c, window, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_change_window_attributes_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_window_attributes_aux(&self) -> bool {
        has_sym!(self, xcb_change_window_attributes_aux)
    }

    pub unsafe fn xcb_change_window_attributes_value_list(
        &self,
        r: *const xcb_change_window_attributes_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_change_window_attributes_value_list)(r)
    }

    /// Returns `true` iff the symbol `xcb_change_window_attributes_value_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_window_attributes_value_list(&self) -> bool {
        has_sym!(self, xcb_change_window_attributes_value_list)
    }

    /**
     * @brief Gets window attributes
     *
     * @param c The connection
     * @param window The window to get the attributes from.
     * @return A cookie
     *
     * Gets the current attributes for the specified \a window.
     *
     */
    pub unsafe fn xcb_get_window_attributes(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_get_window_attributes_cookie_t {
        sym!(self, xcb_get_window_attributes)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_get_window_attributes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_window_attributes(&self) -> bool {
        has_sym!(self, xcb_get_window_attributes)
    }

    /**
     * @brief Gets window attributes
     *
     * @param c The connection
     * @param window The window to get the attributes from.
     * @return A cookie
     *
     * Gets the current attributes for the specified \a window.
     *
     * This form can be used only if the request will cause
     * a reply to be generated. Any returned error will be
     * placed in the event queue.
     */
    pub unsafe fn xcb_get_window_attributes_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_get_window_attributes_cookie_t {
        sym!(self, xcb_get_window_attributes_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_get_window_attributes_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_window_attributes_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_window_attributes_unchecked)
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
     * xcb_get_window_attributes_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_get_window_attributes_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_window_attributes_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_get_window_attributes_reply_t {
        sym!(self, xcb_get_window_attributes_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_window_attributes_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_window_attributes_reply(&self) -> bool {
        has_sym!(self, xcb_get_window_attributes_reply)
    }

    /**
     * @brief Destroys a window
     *
     * @param c The connection
     * @param window The window to destroy.
     * @return A cookie
     *
     * Destroys the specified window and all of its subwindows. A DestroyNotify event
     * is generated for each destroyed window (a DestroyNotify event is first generated
     * for any given window's inferiors). If the window was mapped, it will be
     * automatically unmapped before destroying.
     *
     * Calling DestroyWindow on the root window will do nothing.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_destroy_window_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_destroy_window_checked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_destroy_window_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_destroy_window_checked(&self) -> bool {
        has_sym!(self, xcb_destroy_window_checked)
    }

    /**
     * @brief Destroys a window
     *
     * @param c The connection
     * @param window The window to destroy.
     * @return A cookie
     *
     * Destroys the specified window and all of its subwindows. A DestroyNotify event
     * is generated for each destroyed window (a DestroyNotify event is first generated
     * for any given window's inferiors). If the window was mapped, it will be
     * automatically unmapped before destroying.
     *
     * Calling DestroyWindow on the root window will do nothing.
     *
     */
    pub unsafe fn xcb_destroy_window(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_destroy_window)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_destroy_window` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_destroy_window(&self) -> bool {
        has_sym!(self, xcb_destroy_window)
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
    pub unsafe fn xcb_destroy_subwindows_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_destroy_subwindows_checked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_destroy_subwindows_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_destroy_subwindows_checked(&self) -> bool {
        has_sym!(self, xcb_destroy_subwindows_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_destroy_subwindows(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_destroy_subwindows)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_destroy_subwindows` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_destroy_subwindows(&self) -> bool {
        has_sym!(self, xcb_destroy_subwindows)
    }

    /**
     * @brief Changes a client's save set
     *
     * @param c The connection
     * @param mode A bitmask of #xcb_set_mode_t values.
     * @param mode Insert to add the specified window to the save set or Delete to delete it from the save set.
     * @param window The window to add or delete to/from your save set.
     * @return A cookie
     *
     * TODO: explain what the save set is for.
     *
     * This function either adds or removes the specified window to the client's (your
     * application's) save set.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_change_save_set_checked(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_save_set_checked)(c, mode, window)
    }

    /// Returns `true` iff the symbol `xcb_change_save_set_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_save_set_checked(&self) -> bool {
        has_sym!(self, xcb_change_save_set_checked)
    }

    /**
     * @brief Changes a client's save set
     *
     * @param c The connection
     * @param mode A bitmask of #xcb_set_mode_t values.
     * @param mode Insert to add the specified window to the save set or Delete to delete it from the save set.
     * @param window The window to add or delete to/from your save set.
     * @return A cookie
     *
     * TODO: explain what the save set is for.
     *
     * This function either adds or removes the specified window to the client's (your
     * application's) save set.
     *
     */
    pub unsafe fn xcb_change_save_set(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_save_set)(c, mode, window)
    }

    /// Returns `true` iff the symbol `xcb_change_save_set` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_save_set(&self) -> bool {
        has_sym!(self, xcb_change_save_set)
    }

    /**
     * @brief Reparents a window
     *
     * @param c The connection
     * @param window The window to reparent.
     * @param parent The new parent of the window.
     * @param x The X position of the window within its new parent.
     * @param y The Y position of the window within its new parent.
     * @return A cookie
     *
     * Makes the specified window a child of the specified parent window. If the
     * window is mapped, it will automatically be unmapped before reparenting and
     * re-mapped after reparenting. The window is placed in the stacking order on top
     * with respect to sibling windows.
     *
     * After reparenting, a ReparentNotify event is generated.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_reparent_window_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_reparent_window_checked)(c, window, parent, x, y)
    }

    /// Returns `true` iff the symbol `xcb_reparent_window_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_reparent_window_checked(&self) -> bool {
        has_sym!(self, xcb_reparent_window_checked)
    }

    /**
     * @brief Reparents a window
     *
     * @param c The connection
     * @param window The window to reparent.
     * @param parent The new parent of the window.
     * @param x The X position of the window within its new parent.
     * @param y The Y position of the window within its new parent.
     * @return A cookie
     *
     * Makes the specified window a child of the specified parent window. If the
     * window is mapped, it will automatically be unmapped before reparenting and
     * re-mapped after reparenting. The window is placed in the stacking order on top
     * with respect to sibling windows.
     *
     * After reparenting, a ReparentNotify event is generated.
     *
     */
    pub unsafe fn xcb_reparent_window(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_reparent_window)(c, window, parent, x, y)
    }

    /// Returns `true` iff the symbol `xcb_reparent_window` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_reparent_window(&self) -> bool {
        has_sym!(self, xcb_reparent_window)
    }

    /**
     * @brief Makes a window visible
     *
     * @param c The connection
     * @param window The window to make visible.
     * @return A cookie
     *
     * Maps the specified window. This means making the window visible (as long as its
     * parent is visible).
     *
     * This MapWindow request will be translated to a MapRequest request if a window
     * manager is running. The window manager then decides to either map the window or
     * not. Set the override-redirect window attribute to true if you want to bypass
     * this mechanism.
     *
     * If the window manager decides to map the window (or if no window manager is
     * running), a MapNotify event is generated.
     *
     * If the window becomes viewable and no earlier contents for it are remembered,
     * the X server tiles the window with its background. If the window's background
     * is undefined, the existing screen contents are not altered, and the X server
     * generates zero or more Expose events.
     *
     * If the window type is InputOutput, an Expose event will be generated when the
     * window becomes visible. The normal response to an Expose event should be to
     * repaint the window.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_map_window_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_map_window_checked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_map_window_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_map_window_checked(&self) -> bool {
        has_sym!(self, xcb_map_window_checked)
    }

    /**
     * @brief Makes a window visible
     *
     * @param c The connection
     * @param window The window to make visible.
     * @return A cookie
     *
     * Maps the specified window. This means making the window visible (as long as its
     * parent is visible).
     *
     * This MapWindow request will be translated to a MapRequest request if a window
     * manager is running. The window manager then decides to either map the window or
     * not. Set the override-redirect window attribute to true if you want to bypass
     * this mechanism.
     *
     * If the window manager decides to map the window (or if no window manager is
     * running), a MapNotify event is generated.
     *
     * If the window becomes viewable and no earlier contents for it are remembered,
     * the X server tiles the window with its background. If the window's background
     * is undefined, the existing screen contents are not altered, and the X server
     * generates zero or more Expose events.
     *
     * If the window type is InputOutput, an Expose event will be generated when the
     * window becomes visible. The normal response to an Expose event should be to
     * repaint the window.
     *
     */
    pub unsafe fn xcb_map_window(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_map_window)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_map_window` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_map_window(&self) -> bool {
        has_sym!(self, xcb_map_window)
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
    pub unsafe fn xcb_map_subwindows_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_map_subwindows_checked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_map_subwindows_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_map_subwindows_checked(&self) -> bool {
        has_sym!(self, xcb_map_subwindows_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_map_subwindows(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_map_subwindows)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_map_subwindows` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_map_subwindows(&self) -> bool {
        has_sym!(self, xcb_map_subwindows)
    }

    /**
     * @brief Makes a window invisible
     *
     * @param c The connection
     * @param window The window to make invisible.
     * @return A cookie
     *
     * Unmaps the specified window. This means making the window invisible (and all
     * its child windows).
     *
     * Unmapping a window leads to the `UnmapNotify` event being generated. Also,
     * `Expose` events are generated for formerly obscured windows.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_unmap_window_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_unmap_window_checked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_unmap_window_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_unmap_window_checked(&self) -> bool {
        has_sym!(self, xcb_unmap_window_checked)
    }

    /**
     * @brief Makes a window invisible
     *
     * @param c The connection
     * @param window The window to make invisible.
     * @return A cookie
     *
     * Unmaps the specified window. This means making the window invisible (and all
     * its child windows).
     *
     * Unmapping a window leads to the `UnmapNotify` event being generated. Also,
     * `Expose` events are generated for formerly obscured windows.
     *
     */
    pub unsafe fn xcb_unmap_window(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_unmap_window)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_unmap_window` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_unmap_window(&self) -> bool {
        has_sym!(self, xcb_unmap_window)
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
    pub unsafe fn xcb_unmap_subwindows_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_unmap_subwindows_checked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_unmap_subwindows_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_unmap_subwindows_checked(&self) -> bool {
        has_sym!(self, xcb_unmap_subwindows_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_unmap_subwindows(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_unmap_subwindows)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_unmap_subwindows` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_unmap_subwindows(&self) -> bool {
        has_sym!(self, xcb_unmap_subwindows)
    }

    pub unsafe fn xcb_configure_window_value_list_serialize(
        &self,
        _buffer: *mut *mut c_void,
        value_mask: u16,
        _aux: *const xcb_configure_window_value_list_t,
    ) -> c_int {
        sym!(self, xcb_configure_window_value_list_serialize)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_configure_window_value_list_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_configure_window_value_list_serialize(&self) -> bool {
        has_sym!(self, xcb_configure_window_value_list_serialize)
    }

    pub unsafe fn xcb_configure_window_value_list_unpack(
        &self,
        _buffer: *const c_void,
        value_mask: u16,
        _aux: *mut xcb_configure_window_value_list_t,
    ) -> c_int {
        sym!(self, xcb_configure_window_value_list_unpack)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_configure_window_value_list_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_configure_window_value_list_unpack(&self) -> bool {
        has_sym!(self, xcb_configure_window_value_list_unpack)
    }

    pub unsafe fn xcb_configure_window_value_list_sizeof(
        &self,
        _buffer: *const c_void,
        value_mask: u16,
    ) -> c_int {
        sym!(self, xcb_configure_window_value_list_sizeof)(_buffer, value_mask)
    }

    /// Returns `true` iff the symbol `xcb_configure_window_value_list_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_configure_window_value_list_sizeof(&self) -> bool {
        has_sym!(self, xcb_configure_window_value_list_sizeof)
    }

    pub unsafe fn xcb_configure_window_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_configure_window_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_configure_window_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_configure_window_sizeof(&self) -> bool {
        has_sym!(self, xcb_configure_window_sizeof)
    }

    /**
     * @brief Configures window attributes
     *
     * @param c The connection
     * @param window The window to configure.
     * @param value_mask Bitmask of attributes to change.
     * @param value_list New values, corresponding to the attributes in value_mask. The order has to
     * correspond to the order of possible \a value_mask bits. See the example.
     * @return A cookie
     *
     * Configures a window's size, position, border width and stacking order.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_configure_window_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u16,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_configure_window_checked)(c, window, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_configure_window_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_configure_window_checked(&self) -> bool {
        has_sym!(self, xcb_configure_window_checked)
    }

    /**
     * @brief Configures window attributes
     *
     * @param c The connection
     * @param window The window to configure.
     * @param value_mask Bitmask of attributes to change.
     * @param value_list New values, corresponding to the attributes in value_mask. The order has to
     * correspond to the order of possible \a value_mask bits. See the example.
     * @return A cookie
     *
     * Configures a window's size, position, border width and stacking order.
     *
     */
    pub unsafe fn xcb_configure_window(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u16,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_configure_window)(c, window, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_configure_window` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_configure_window(&self) -> bool {
        has_sym!(self, xcb_configure_window)
    }

    /**
     * @brief Configures window attributes
     *
     * @param c The connection
     * @param window The window to configure.
     * @param value_mask Bitmask of attributes to change.
     * @param value_list New values, corresponding to the attributes in value_mask. The order has to
     * correspond to the order of possible \a value_mask bits. See the example.
     * @return A cookie
     *
     * Configures a window's size, position, border width and stacking order.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_configure_window_aux_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u16,
        value_list: *const xcb_configure_window_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_configure_window_aux_checked)(c, window, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_configure_window_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_configure_window_aux_checked(&self) -> bool {
        has_sym!(self, xcb_configure_window_aux_checked)
    }

    /**
     * @brief Configures window attributes
     *
     * @param c The connection
     * @param window The window to configure.
     * @param value_mask Bitmask of attributes to change.
     * @param value_list New values, corresponding to the attributes in value_mask. The order has to
     * correspond to the order of possible \a value_mask bits. See the example.
     * @return A cookie
     *
     * Configures a window's size, position, border width and stacking order.
     *
     */
    pub unsafe fn xcb_configure_window_aux(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u16,
        value_list: *const xcb_configure_window_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_configure_window_aux)(c, window, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_configure_window_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_configure_window_aux(&self) -> bool {
        has_sym!(self, xcb_configure_window_aux)
    }

    pub unsafe fn xcb_configure_window_value_list(
        &self,
        r: *const xcb_configure_window_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_configure_window_value_list)(r)
    }

    /// Returns `true` iff the symbol `xcb_configure_window_value_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_configure_window_value_list(&self) -> bool {
        has_sym!(self, xcb_configure_window_value_list)
    }

    /**
     * @brief Change window stacking order
     *
     * @param c The connection
     * @param direction A bitmask of #xcb_circulate_t values.
     * @param direction \n
     * @param window The window to raise/lower (depending on \a direction).
     * @return A cookie
     *
     * If \a direction is `XCB_CIRCULATE_RAISE_LOWEST`, the lowest mapped child (if
     * any) will be raised to the top of the stack.
     *
     * If \a direction is `XCB_CIRCULATE_LOWER_HIGHEST`, the highest mapped child will
     * be lowered to the bottom of the stack.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_circulate_window_checked(
        &self,
        c: *mut xcb_connection_t,
        direction: u8,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_circulate_window_checked)(c, direction, window)
    }

    /// Returns `true` iff the symbol `xcb_circulate_window_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_circulate_window_checked(&self) -> bool {
        has_sym!(self, xcb_circulate_window_checked)
    }

    /**
     * @brief Change window stacking order
     *
     * @param c The connection
     * @param direction A bitmask of #xcb_circulate_t values.
     * @param direction \n
     * @param window The window to raise/lower (depending on \a direction).
     * @return A cookie
     *
     * If \a direction is `XCB_CIRCULATE_RAISE_LOWEST`, the lowest mapped child (if
     * any) will be raised to the top of the stack.
     *
     * If \a direction is `XCB_CIRCULATE_LOWER_HIGHEST`, the highest mapped child will
     * be lowered to the bottom of the stack.
     *
     */
    pub unsafe fn xcb_circulate_window(
        &self,
        c: *mut xcb_connection_t,
        direction: u8,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_circulate_window)(c, direction, window)
    }

    /// Returns `true` iff the symbol `xcb_circulate_window` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_circulate_window(&self) -> bool {
        has_sym!(self, xcb_circulate_window)
    }

    /**
     * @brief Get current window geometry
     *
     * @param c The connection
     * @param drawable The drawable (`Window` or `Pixmap`) of which the geometry will be received.
     * @return A cookie
     *
     * Gets the current geometry of the specified drawable (either `Window` or `Pixmap`).
     *
     */
    pub unsafe fn xcb_get_geometry(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_get_geometry_cookie_t {
        sym!(self, xcb_get_geometry)(c, drawable)
    }

    /// Returns `true` iff the symbol `xcb_get_geometry` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_geometry(&self) -> bool {
        has_sym!(self, xcb_get_geometry)
    }

    /**
     * @brief Get current window geometry
     *
     * @param c The connection
     * @param drawable The drawable (`Window` or `Pixmap`) of which the geometry will be received.
     * @return A cookie
     *
     * Gets the current geometry of the specified drawable (either `Window` or `Pixmap`).
     *
     * This form can be used only if the request will cause
     * a reply to be generated. Any returned error will be
     * placed in the event queue.
     */
    pub unsafe fn xcb_get_geometry_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_get_geometry_cookie_t {
        sym!(self, xcb_get_geometry_unchecked)(c, drawable)
    }

    /// Returns `true` iff the symbol `xcb_get_geometry_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_geometry_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_geometry_unchecked)
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
     * xcb_get_geometry_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_get_geometry_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_geometry_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_get_geometry_reply_t {
        sym!(self, xcb_get_geometry_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_geometry_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_geometry_reply(&self) -> bool {
        has_sym!(self, xcb_get_geometry_reply)
    }

    pub unsafe fn xcb_query_tree_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_query_tree_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_query_tree_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_tree_sizeof(&self) -> bool {
        has_sym!(self, xcb_query_tree_sizeof)
    }

    /**
     * @brief query the window tree
     *
     * @param c The connection
     * @param window The \a window to query.
     * @return A cookie
     *
     * Gets the root window ID, parent window ID and list of children windows for the
     * specified \a window. The children are listed in bottom-to-top stacking order.
     *
     */
    pub unsafe fn xcb_query_tree(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_query_tree_cookie_t {
        sym!(self, xcb_query_tree)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_query_tree` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_tree(&self) -> bool {
        has_sym!(self, xcb_query_tree)
    }

    /**
     * @brief query the window tree
     *
     * @param c The connection
     * @param window The \a window to query.
     * @return A cookie
     *
     * Gets the root window ID, parent window ID and list of children windows for the
     * specified \a window. The children are listed in bottom-to-top stacking order.
     *
     * This form can be used only if the request will cause
     * a reply to be generated. Any returned error will be
     * placed in the event queue.
     */
    pub unsafe fn xcb_query_tree_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_query_tree_cookie_t {
        sym!(self, xcb_query_tree_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_query_tree_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_tree_unchecked(&self) -> bool {
        has_sym!(self, xcb_query_tree_unchecked)
    }

    pub unsafe fn xcb_query_tree_children(
        &self,
        r: *const xcb_query_tree_reply_t,
    ) -> *mut xcb_window_t {
        sym!(self, xcb_query_tree_children)(r)
    }

    /// Returns `true` iff the symbol `xcb_query_tree_children` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_tree_children(&self) -> bool {
        has_sym!(self, xcb_query_tree_children)
    }

    pub unsafe fn xcb_query_tree_children_length(&self, r: *const xcb_query_tree_reply_t) -> c_int {
        sym!(self, xcb_query_tree_children_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_query_tree_children_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_tree_children_length(&self) -> bool {
        has_sym!(self, xcb_query_tree_children_length)
    }

    pub unsafe fn xcb_query_tree_children_end(
        &self,
        r: *const xcb_query_tree_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_query_tree_children_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_query_tree_children_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_tree_children_end(&self) -> bool {
        has_sym!(self, xcb_query_tree_children_end)
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
     * xcb_query_tree_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_query_tree_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_query_tree_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_query_tree_reply_t {
        sym!(self, xcb_query_tree_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_query_tree_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_tree_reply(&self) -> bool {
        has_sym!(self, xcb_query_tree_reply)
    }

    pub unsafe fn xcb_intern_atom_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_intern_atom_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_intern_atom_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_intern_atom_sizeof(&self) -> bool {
        has_sym!(self, xcb_intern_atom_sizeof)
    }

    /**
     * @brief Get atom identifier by name
     *
     * @param c The connection
     * @param only_if_exists Return a valid atom id only if the atom already exists.
     * @param name_len The length of the following \a name.
     * @param name The name of the atom.
     * @return A cookie
     *
     * Retrieves the identifier (xcb_atom_t TODO) for the atom with the specified
     * name. Atoms are used in protocols like EWMH, for example to store window titles
     * (`_NET_WM_NAME` atom) as property of a window.
     *
     * If \a only_if_exists is 0, the atom will be created if it does not already exist.
     * If \a only_if_exists is 1, `XCB_ATOM_NONE` will be returned if the atom does
     * not yet exist.
     *
     */
    pub unsafe fn xcb_intern_atom(
        &self,
        c: *mut xcb_connection_t,
        only_if_exists: u8,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_intern_atom_cookie_t {
        sym!(self, xcb_intern_atom)(c, only_if_exists, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_intern_atom` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_intern_atom(&self) -> bool {
        has_sym!(self, xcb_intern_atom)
    }

    /**
     * @brief Get atom identifier by name
     *
     * @param c The connection
     * @param only_if_exists Return a valid atom id only if the atom already exists.
     * @param name_len The length of the following \a name.
     * @param name The name of the atom.
     * @return A cookie
     *
     * Retrieves the identifier (xcb_atom_t TODO) for the atom with the specified
     * name. Atoms are used in protocols like EWMH, for example to store window titles
     * (`_NET_WM_NAME` atom) as property of a window.
     *
     * If \a only_if_exists is 0, the atom will be created if it does not already exist.
     * If \a only_if_exists is 1, `XCB_ATOM_NONE` will be returned if the atom does
     * not yet exist.
     *
     * This form can be used only if the request will cause
     * a reply to be generated. Any returned error will be
     * placed in the event queue.
     */
    pub unsafe fn xcb_intern_atom_unchecked(
        &self,
        c: *mut xcb_connection_t,
        only_if_exists: u8,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_intern_atom_cookie_t {
        sym!(self, xcb_intern_atom_unchecked)(c, only_if_exists, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_intern_atom_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_intern_atom_unchecked(&self) -> bool {
        has_sym!(self, xcb_intern_atom_unchecked)
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
     * xcb_intern_atom_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_intern_atom_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_intern_atom_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_intern_atom_reply_t {
        sym!(self, xcb_intern_atom_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_intern_atom_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_intern_atom_reply(&self) -> bool {
        has_sym!(self, xcb_intern_atom_reply)
    }

    pub unsafe fn xcb_get_atom_name_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_get_atom_name_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_get_atom_name_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_atom_name_sizeof(&self) -> bool {
        has_sym!(self, xcb_get_atom_name_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_get_atom_name(
        &self,
        c: *mut xcb_connection_t,
        atom: xcb_atom_t,
    ) -> xcb_get_atom_name_cookie_t {
        sym!(self, xcb_get_atom_name)(c, atom)
    }

    /// Returns `true` iff the symbol `xcb_get_atom_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_atom_name(&self) -> bool {
        has_sym!(self, xcb_get_atom_name)
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
    pub unsafe fn xcb_get_atom_name_unchecked(
        &self,
        c: *mut xcb_connection_t,
        atom: xcb_atom_t,
    ) -> xcb_get_atom_name_cookie_t {
        sym!(self, xcb_get_atom_name_unchecked)(c, atom)
    }

    /// Returns `true` iff the symbol `xcb_get_atom_name_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_atom_name_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_atom_name_unchecked)
    }

    pub unsafe fn xcb_get_atom_name_name(
        &self,
        r: *const xcb_get_atom_name_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_get_atom_name_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_atom_name_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_atom_name_name(&self) -> bool {
        has_sym!(self, xcb_get_atom_name_name)
    }

    pub unsafe fn xcb_get_atom_name_name_length(
        &self,
        r: *const xcb_get_atom_name_reply_t,
    ) -> c_int {
        sym!(self, xcb_get_atom_name_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_atom_name_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_atom_name_name_length(&self) -> bool {
        has_sym!(self, xcb_get_atom_name_name_length)
    }

    pub unsafe fn xcb_get_atom_name_name_end(
        &self,
        r: *const xcb_get_atom_name_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_get_atom_name_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_atom_name_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_atom_name_name_end(&self) -> bool {
        has_sym!(self, xcb_get_atom_name_name_end)
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
     * xcb_get_atom_name_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_get_atom_name_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_atom_name_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_get_atom_name_reply_t {
        sym!(self, xcb_get_atom_name_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_atom_name_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_atom_name_reply(&self) -> bool {
        has_sym!(self, xcb_get_atom_name_reply)
    }

    pub unsafe fn xcb_change_property_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_change_property_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_change_property_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_property_sizeof(&self) -> bool {
        has_sym!(self, xcb_change_property_sizeof)
    }

    /**
     * @brief Changes a window property
     *
     * @param c The connection
     * @param mode A bitmask of #xcb_prop_mode_t values.
     * @param mode \n
     * @param window The window whose property you want to change.
     * @param property The property you want to change (an atom).
     * @param type_ The type of the property you want to change (an atom).
     * @param format Specifies whether the data should be viewed as a list of 8-bit, 16-bit or
     * 32-bit quantities. Possible values are 8, 16 and 32. This information allows
     * the X server to correctly perform byte-swap operations as necessary.
     * @param data_len Specifies the number of elements (see \a format).
     * @param data The property data.
     * @return A cookie
     *
     * Sets or updates a property on the specified \a window. Properties are for
     * example the window title (`WM_NAME`) or its minimum size (`WM_NORMAL_HINTS`).
     * Protocols such as EWMH also use properties - for example EWMH defines the
     * window title, encoded as UTF-8 string, in the `_NET_WM_NAME` property.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_change_property_checked(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        format: u8,
        data_len: u32,
        data: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_property_checked)(
            c, mode, window, property, type_, format, data_len, data,
        )
    }

    /// Returns `true` iff the symbol `xcb_change_property_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_property_checked(&self) -> bool {
        has_sym!(self, xcb_change_property_checked)
    }

    /**
     * @brief Changes a window property
     *
     * @param c The connection
     * @param mode A bitmask of #xcb_prop_mode_t values.
     * @param mode \n
     * @param window The window whose property you want to change.
     * @param property The property you want to change (an atom).
     * @param type_ The type of the property you want to change (an atom).
     * @param format Specifies whether the data should be viewed as a list of 8-bit, 16-bit or
     * 32-bit quantities. Possible values are 8, 16 and 32. This information allows
     * the X server to correctly perform byte-swap operations as necessary.
     * @param data_len Specifies the number of elements (see \a format).
     * @param data The property data.
     * @return A cookie
     *
     * Sets or updates a property on the specified \a window. Properties are for
     * example the window title (`WM_NAME`) or its minimum size (`WM_NORMAL_HINTS`).
     * Protocols such as EWMH also use properties - for example EWMH defines the
     * window title, encoded as UTF-8 string, in the `_NET_WM_NAME` property.
     *
     */
    pub unsafe fn xcb_change_property(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        format: u8,
        data_len: u32,
        data: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_property)(c, mode, window, property, type_, format, data_len, data)
    }

    /// Returns `true` iff the symbol `xcb_change_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_property(&self) -> bool {
        has_sym!(self, xcb_change_property)
    }

    pub unsafe fn xcb_change_property_data(
        &self,
        r: *const xcb_change_property_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_change_property_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_change_property_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_property_data(&self) -> bool {
        has_sym!(self, xcb_change_property_data)
    }

    pub unsafe fn xcb_change_property_data_length(
        &self,
        r: *const xcb_change_property_request_t,
    ) -> c_int {
        sym!(self, xcb_change_property_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_change_property_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_property_data_length(&self) -> bool {
        has_sym!(self, xcb_change_property_data_length)
    }

    pub unsafe fn xcb_change_property_data_end(
        &self,
        r: *const xcb_change_property_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_change_property_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_change_property_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_property_data_end(&self) -> bool {
        has_sym!(self, xcb_change_property_data_end)
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
    pub unsafe fn xcb_delete_property_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        property: xcb_atom_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_delete_property_checked)(c, window, property)
    }

    /// Returns `true` iff the symbol `xcb_delete_property_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_delete_property_checked(&self) -> bool {
        has_sym!(self, xcb_delete_property_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_delete_property(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        property: xcb_atom_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_delete_property)(c, window, property)
    }

    /// Returns `true` iff the symbol `xcb_delete_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_delete_property(&self) -> bool {
        has_sym!(self, xcb_delete_property)
    }

    pub unsafe fn xcb_get_property_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_get_property_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_get_property_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_property_sizeof(&self) -> bool {
        has_sym!(self, xcb_get_property_sizeof)
    }

    /**
     * @brief Gets a window property
     *
     * @param c The connection
     * @param delete Whether the property should actually be deleted. For deleting a property, the
     * specified `type` has to match the actual property type.
     * @param window The window whose property you want to get.
     * @param property The property you want to get (an atom).
     * @param type_ The type of the property you want to get (an atom).
     * @param long_offset Specifies the offset (in 32-bit multiples) in the specified property where the
     * data is to be retrieved.
     * @param long_length Specifies how many 32-bit multiples of data should be retrieved (e.g. if you
     * set \a long_length to 4, you will receive 16 bytes of data).
     * @return A cookie
     *
     * Gets the specified \a property from the specified \a window. Properties are for
     * example the window title (`WM_NAME`) or its minimum size (`WM_NORMAL_HINTS`).
     * Protocols such as EWMH also use properties - for example EWMH defines the
     * window title, encoded as UTF-8 string, in the `_NET_WM_NAME` property.
     *
     * TODO: talk about `type`
     *
     * TODO: talk about \a delete
     *
     * TODO: talk about the offset/length thing. what's a valid use case?
     *
     */
    pub unsafe fn xcb_get_property(
        &self,
        c: *mut xcb_connection_t,
        delete: u8,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        long_offset: u32,
        long_length: u32,
    ) -> xcb_get_property_cookie_t {
        sym!(self, xcb_get_property)(c, delete, window, property, type_, long_offset, long_length)
    }

    /// Returns `true` iff the symbol `xcb_get_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_property(&self) -> bool {
        has_sym!(self, xcb_get_property)
    }

    /**
     * @brief Gets a window property
     *
     * @param c The connection
     * @param delete Whether the property should actually be deleted. For deleting a property, the
     * specified `type` has to match the actual property type.
     * @param window The window whose property you want to get.
     * @param property The property you want to get (an atom).
     * @param type_ The type of the property you want to get (an atom).
     * @param long_offset Specifies the offset (in 32-bit multiples) in the specified property where the
     * data is to be retrieved.
     * @param long_length Specifies how many 32-bit multiples of data should be retrieved (e.g. if you
     * set \a long_length to 4, you will receive 16 bytes of data).
     * @return A cookie
     *
     * Gets the specified \a property from the specified \a window. Properties are for
     * example the window title (`WM_NAME`) or its minimum size (`WM_NORMAL_HINTS`).
     * Protocols such as EWMH also use properties - for example EWMH defines the
     * window title, encoded as UTF-8 string, in the `_NET_WM_NAME` property.
     *
     * TODO: talk about `type`
     *
     * TODO: talk about \a delete
     *
     * TODO: talk about the offset/length thing. what's a valid use case?
     *
     * This form can be used only if the request will cause
     * a reply to be generated. Any returned error will be
     * placed in the event queue.
     */
    pub unsafe fn xcb_get_property_unchecked(
        &self,
        c: *mut xcb_connection_t,
        delete: u8,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        long_offset: u32,
        long_length: u32,
    ) -> xcb_get_property_cookie_t {
        sym!(self, xcb_get_property_unchecked)(
            c,
            delete,
            window,
            property,
            type_,
            long_offset,
            long_length,
        )
    }

    /// Returns `true` iff the symbol `xcb_get_property_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_property_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_property_unchecked)
    }

    pub unsafe fn xcb_get_property_value(&self, r: *const xcb_get_property_reply_t) -> *mut c_void {
        sym!(self, xcb_get_property_value)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_property_value` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_property_value(&self) -> bool {
        has_sym!(self, xcb_get_property_value)
    }

    pub unsafe fn xcb_get_property_value_length(
        &self,
        r: *const xcb_get_property_reply_t,
    ) -> c_int {
        sym!(self, xcb_get_property_value_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_property_value_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_property_value_length(&self) -> bool {
        has_sym!(self, xcb_get_property_value_length)
    }

    pub unsafe fn xcb_get_property_value_end(
        &self,
        r: *const xcb_get_property_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_get_property_value_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_property_value_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_property_value_end(&self) -> bool {
        has_sym!(self, xcb_get_property_value_end)
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
     * xcb_get_property_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_get_property_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_property_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_get_property_reply_t {
        sym!(self, xcb_get_property_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_property_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_property_reply(&self) -> bool {
        has_sym!(self, xcb_get_property_reply)
    }

    pub unsafe fn xcb_list_properties_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_list_properties_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_list_properties_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_properties_sizeof(&self) -> bool {
        has_sym!(self, xcb_list_properties_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_list_properties(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_list_properties_cookie_t {
        sym!(self, xcb_list_properties)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_list_properties` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_properties(&self) -> bool {
        has_sym!(self, xcb_list_properties)
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
    pub unsafe fn xcb_list_properties_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_list_properties_cookie_t {
        sym!(self, xcb_list_properties_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_list_properties_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_properties_unchecked(&self) -> bool {
        has_sym!(self, xcb_list_properties_unchecked)
    }

    pub unsafe fn xcb_list_properties_atoms(
        &self,
        r: *const xcb_list_properties_reply_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_list_properties_atoms)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_properties_atoms` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_properties_atoms(&self) -> bool {
        has_sym!(self, xcb_list_properties_atoms)
    }

    pub unsafe fn xcb_list_properties_atoms_length(
        &self,
        r: *const xcb_list_properties_reply_t,
    ) -> c_int {
        sym!(self, xcb_list_properties_atoms_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_properties_atoms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_properties_atoms_length(&self) -> bool {
        has_sym!(self, xcb_list_properties_atoms_length)
    }

    pub unsafe fn xcb_list_properties_atoms_end(
        &self,
        r: *const xcb_list_properties_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_list_properties_atoms_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_properties_atoms_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_properties_atoms_end(&self) -> bool {
        has_sym!(self, xcb_list_properties_atoms_end)
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
     * xcb_list_properties_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_list_properties_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_list_properties_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_list_properties_reply_t {
        sym!(self, xcb_list_properties_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_list_properties_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_properties_reply(&self) -> bool {
        has_sym!(self, xcb_list_properties_reply)
    }

    /**
     * @brief Sets the owner of a selection
     *
     * @param c The connection
     * @param owner The new owner of the selection.
     * \n
     * The special value `XCB_NONE` means that the selection will have no owner.
     * @param selection The selection.
     * @param time Timestamp to avoid race conditions when running X over the network.
     * \n
     * The selection will not be changed if \a time is earlier than the current
     * last-change time of the \a selection or is later than the current X server time.
     * Otherwise, the last-change time is set to the specified time.
     * \n
     * The special value `XCB_CURRENT_TIME` will be replaced with the current server
     * time.
     * @return A cookie
     *
     * Makes `window` the owner of the selection \a selection and updates the
     * last-change time of the specified selection.
     *
     * TODO: briefly explain what a selection is.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_set_selection_owner_checked(
        &self,
        c: *mut xcb_connection_t,
        owner: xcb_window_t,
        selection: xcb_atom_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_selection_owner_checked)(c, owner, selection, time)
    }

    /// Returns `true` iff the symbol `xcb_set_selection_owner_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_selection_owner_checked(&self) -> bool {
        has_sym!(self, xcb_set_selection_owner_checked)
    }

    /**
     * @brief Sets the owner of a selection
     *
     * @param c The connection
     * @param owner The new owner of the selection.
     * \n
     * The special value `XCB_NONE` means that the selection will have no owner.
     * @param selection The selection.
     * @param time Timestamp to avoid race conditions when running X over the network.
     * \n
     * The selection will not be changed if \a time is earlier than the current
     * last-change time of the \a selection or is later than the current X server time.
     * Otherwise, the last-change time is set to the specified time.
     * \n
     * The special value `XCB_CURRENT_TIME` will be replaced with the current server
     * time.
     * @return A cookie
     *
     * Makes `window` the owner of the selection \a selection and updates the
     * last-change time of the specified selection.
     *
     * TODO: briefly explain what a selection is.
     *
     */
    pub unsafe fn xcb_set_selection_owner(
        &self,
        c: *mut xcb_connection_t,
        owner: xcb_window_t,
        selection: xcb_atom_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_selection_owner)(c, owner, selection, time)
    }

    /// Returns `true` iff the symbol `xcb_set_selection_owner` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_selection_owner(&self) -> bool {
        has_sym!(self, xcb_set_selection_owner)
    }

    /**
     * @brief Gets the owner of a selection
     *
     * @param c The connection
     * @param selection The selection.
     * @return A cookie
     *
     * Gets the owner of the specified selection.
     *
     * TODO: briefly explain what a selection is.
     *
     */
    pub unsafe fn xcb_get_selection_owner(
        &self,
        c: *mut xcb_connection_t,
        selection: xcb_atom_t,
    ) -> xcb_get_selection_owner_cookie_t {
        sym!(self, xcb_get_selection_owner)(c, selection)
    }

    /// Returns `true` iff the symbol `xcb_get_selection_owner` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_selection_owner(&self) -> bool {
        has_sym!(self, xcb_get_selection_owner)
    }

    /**
     * @brief Gets the owner of a selection
     *
     * @param c The connection
     * @param selection The selection.
     * @return A cookie
     *
     * Gets the owner of the specified selection.
     *
     * TODO: briefly explain what a selection is.
     *
     * This form can be used only if the request will cause
     * a reply to be generated. Any returned error will be
     * placed in the event queue.
     */
    pub unsafe fn xcb_get_selection_owner_unchecked(
        &self,
        c: *mut xcb_connection_t,
        selection: xcb_atom_t,
    ) -> xcb_get_selection_owner_cookie_t {
        sym!(self, xcb_get_selection_owner_unchecked)(c, selection)
    }

    /// Returns `true` iff the symbol `xcb_get_selection_owner_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_selection_owner_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_selection_owner_unchecked)
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
     * xcb_get_selection_owner_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_get_selection_owner_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_selection_owner_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_get_selection_owner_reply_t {
        sym!(self, xcb_get_selection_owner_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_selection_owner_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_selection_owner_reply(&self) -> bool {
        has_sym!(self, xcb_get_selection_owner_reply)
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
    pub unsafe fn xcb_convert_selection_checked(
        &self,
        c: *mut xcb_connection_t,
        requestor: xcb_window_t,
        selection: xcb_atom_t,
        target: xcb_atom_t,
        property: xcb_atom_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_convert_selection_checked)(c, requestor, selection, target, property, time)
    }

    /// Returns `true` iff the symbol `xcb_convert_selection_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_convert_selection_checked(&self) -> bool {
        has_sym!(self, xcb_convert_selection_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_convert_selection(
        &self,
        c: *mut xcb_connection_t,
        requestor: xcb_window_t,
        selection: xcb_atom_t,
        target: xcb_atom_t,
        property: xcb_atom_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_convert_selection)(c, requestor, selection, target, property, time)
    }

    /// Returns `true` iff the symbol `xcb_convert_selection` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_convert_selection(&self) -> bool {
        has_sym!(self, xcb_convert_selection)
    }

    /**
     * @brief send an event
     *
     * @param c The connection
     * @param propagate If \a propagate is true and no clients have selected any event on \a destination,
     * the destination is replaced with the closest ancestor of \a destination for
     * which some client has selected a type in \a event_mask and for which no
     * intervening window has that type in its do-not-propagate-mask. If no such
     * window exists or if the window is an ancestor of the focus window and
     * `InputFocus` was originally specified as the destination, the event is not sent
     * to any clients. Otherwise, the event is reported to every client selecting on
     * the final destination any of the types specified in \a event_mask.
     * @param destination The window to send this event to. Every client which selects any event within
     * \a event_mask on \a destination will get the event.
     * \n
     * The special value `XCB_SEND_EVENT_DEST_POINTER_WINDOW` refers to the window
     * that contains the mouse pointer.
     * \n
     * The special value `XCB_SEND_EVENT_DEST_ITEM_FOCUS` refers to the window which
     * has the keyboard focus.
     * @param event_mask Event_mask for determining which clients should receive the specified event.
     * See \a destination and \a propagate.
     * @param event The event to send to the specified \a destination.
     * @return A cookie
     *
     * Identifies the \a destination window, determines which clients should receive
     * the specified event and ignores any active grabs.
     *
     * The \a event must be one of the core events or an event defined by an extension,
     * so that the X server can correctly byte-swap the contents as necessary. The
     * contents of \a event are otherwise unaltered and unchecked except for the
     * `send_event` field which is forced to 'true'.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_send_event_checked(
        &self,
        c: *mut xcb_connection_t,
        propagate: u8,
        destination: xcb_window_t,
        event_mask: u32,
        event: *const [c_char; 32],
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_send_event_checked)(c, propagate, destination, event_mask, event)
    }

    /// Returns `true` iff the symbol `xcb_send_event_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_send_event_checked(&self) -> bool {
        has_sym!(self, xcb_send_event_checked)
    }

    /**
     * @brief send an event
     *
     * @param c The connection
     * @param propagate If \a propagate is true and no clients have selected any event on \a destination,
     * the destination is replaced with the closest ancestor of \a destination for
     * which some client has selected a type in \a event_mask and for which no
     * intervening window has that type in its do-not-propagate-mask. If no such
     * window exists or if the window is an ancestor of the focus window and
     * `InputFocus` was originally specified as the destination, the event is not sent
     * to any clients. Otherwise, the event is reported to every client selecting on
     * the final destination any of the types specified in \a event_mask.
     * @param destination The window to send this event to. Every client which selects any event within
     * \a event_mask on \a destination will get the event.
     * \n
     * The special value `XCB_SEND_EVENT_DEST_POINTER_WINDOW` refers to the window
     * that contains the mouse pointer.
     * \n
     * The special value `XCB_SEND_EVENT_DEST_ITEM_FOCUS` refers to the window which
     * has the keyboard focus.
     * @param event_mask Event_mask for determining which clients should receive the specified event.
     * See \a destination and \a propagate.
     * @param event The event to send to the specified \a destination.
     * @return A cookie
     *
     * Identifies the \a destination window, determines which clients should receive
     * the specified event and ignores any active grabs.
     *
     * The \a event must be one of the core events or an event defined by an extension,
     * so that the X server can correctly byte-swap the contents as necessary. The
     * contents of \a event are otherwise unaltered and unchecked except for the
     * `send_event` field which is forced to 'true'.
     *
     */
    pub unsafe fn xcb_send_event(
        &self,
        c: *mut xcb_connection_t,
        propagate: u8,
        destination: xcb_window_t,
        event_mask: u32,
        event: *const [c_char; 32],
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_send_event)(c, propagate, destination, event_mask, event)
    }

    /// Returns `true` iff the symbol `xcb_send_event` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_send_event(&self) -> bool {
        has_sym!(self, xcb_send_event)
    }

    /**
     * @brief Grab the pointer
     *
     * @param c The connection
     * @param owner_events If 1, the \a grab_window will still get the pointer events. If 0, events are not
     * reported to the \a grab_window.
     * @param grab_window Specifies the window on which the pointer should be grabbed.
     * @param event_mask Specifies which pointer events are reported to the client.
     * \n
     * TODO: which values?
     * @param pointer_mode A bitmask of #xcb_grab_mode_t values.
     * @param pointer_mode \n
     * @param keyboard_mode A bitmask of #xcb_grab_mode_t values.
     * @param keyboard_mode \n
     * @param confine_to Specifies the window to confine the pointer in (the user will not be able to
     * move the pointer out of that window).
     * \n
     * The special value `XCB_NONE` means don't confine the pointer.
     * @param cursor Specifies the cursor that should be displayed or `XCB_NONE` to not change the
     * cursor.
     * @param time The time argument allows you to avoid certain circumstances that come up if
     * applications take a long time to respond or if there are long network delays.
     * Consider a situation where you have two applications, both of which normally
     * grab the pointer when clicked on. If both applications specify the timestamp
     * from the event, the second application may wake up faster and successfully grab
     * the pointer before the first application. The first application then will get
     * an indication that the other application grabbed the pointer before its request
     * was processed.
     * \n
     * The special value `XCB_CURRENT_TIME` will be replaced with the current server
     * time.
     * @return A cookie
     *
     * Actively grabs control of the pointer. Further pointer events are reported only to the grabbing client. Overrides any active pointer grab by this client.
     *
     */
    pub unsafe fn xcb_grab_pointer(
        &self,
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        event_mask: u16,
        pointer_mode: u8,
        keyboard_mode: u8,
        confine_to: xcb_window_t,
        cursor: xcb_cursor_t,
        time: xcb_timestamp_t,
    ) -> xcb_grab_pointer_cookie_t {
        sym!(self, xcb_grab_pointer)(
            c,
            owner_events,
            grab_window,
            event_mask,
            pointer_mode,
            keyboard_mode,
            confine_to,
            cursor,
            time,
        )
    }

    /// Returns `true` iff the symbol `xcb_grab_pointer` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_grab_pointer(&self) -> bool {
        has_sym!(self, xcb_grab_pointer)
    }

    /**
     * @brief Grab the pointer
     *
     * @param c The connection
     * @param owner_events If 1, the \a grab_window will still get the pointer events. If 0, events are not
     * reported to the \a grab_window.
     * @param grab_window Specifies the window on which the pointer should be grabbed.
     * @param event_mask Specifies which pointer events are reported to the client.
     * \n
     * TODO: which values?
     * @param pointer_mode A bitmask of #xcb_grab_mode_t values.
     * @param pointer_mode \n
     * @param keyboard_mode A bitmask of #xcb_grab_mode_t values.
     * @param keyboard_mode \n
     * @param confine_to Specifies the window to confine the pointer in (the user will not be able to
     * move the pointer out of that window).
     * \n
     * The special value `XCB_NONE` means don't confine the pointer.
     * @param cursor Specifies the cursor that should be displayed or `XCB_NONE` to not change the
     * cursor.
     * @param time The time argument allows you to avoid certain circumstances that come up if
     * applications take a long time to respond or if there are long network delays.
     * Consider a situation where you have two applications, both of which normally
     * grab the pointer when clicked on. If both applications specify the timestamp
     * from the event, the second application may wake up faster and successfully grab
     * the pointer before the first application. The first application then will get
     * an indication that the other application grabbed the pointer before its request
     * was processed.
     * \n
     * The special value `XCB_CURRENT_TIME` will be replaced with the current server
     * time.
     * @return A cookie
     *
     * Actively grabs control of the pointer. Further pointer events are reported only to the grabbing client. Overrides any active pointer grab by this client.
     *
     * This form can be used only if the request will cause
     * a reply to be generated. Any returned error will be
     * placed in the event queue.
     */
    pub unsafe fn xcb_grab_pointer_unchecked(
        &self,
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        event_mask: u16,
        pointer_mode: u8,
        keyboard_mode: u8,
        confine_to: xcb_window_t,
        cursor: xcb_cursor_t,
        time: xcb_timestamp_t,
    ) -> xcb_grab_pointer_cookie_t {
        sym!(self, xcb_grab_pointer_unchecked)(
            c,
            owner_events,
            grab_window,
            event_mask,
            pointer_mode,
            keyboard_mode,
            confine_to,
            cursor,
            time,
        )
    }

    /// Returns `true` iff the symbol `xcb_grab_pointer_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_grab_pointer_unchecked(&self) -> bool {
        has_sym!(self, xcb_grab_pointer_unchecked)
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
     * xcb_grab_pointer_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_grab_pointer_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_grab_pointer_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_grab_pointer_reply_t {
        sym!(self, xcb_grab_pointer_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_grab_pointer_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_grab_pointer_reply(&self) -> bool {
        has_sym!(self, xcb_grab_pointer_reply)
    }

    /**
     * @brief release the pointer
     *
     * @param c The connection
     * @param time Timestamp to avoid race conditions when running X over the network.
     * \n
     * The pointer will not be released if \a time is earlier than the
     * last-pointer-grab time or later than the current X server time.
     * @return A cookie
     *
     * Releases the pointer and any queued events if you actively grabbed the pointer
     * before using `xcb_grab_pointer`, `xcb_grab_button` or within a normal button
     * press.
     *
     * EnterNotify and LeaveNotify events are generated.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_ungrab_pointer_checked(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_ungrab_pointer_checked)(c, time)
    }

    /// Returns `true` iff the symbol `xcb_ungrab_pointer_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_ungrab_pointer_checked(&self) -> bool {
        has_sym!(self, xcb_ungrab_pointer_checked)
    }

    /**
     * @brief release the pointer
     *
     * @param c The connection
     * @param time Timestamp to avoid race conditions when running X over the network.
     * \n
     * The pointer will not be released if \a time is earlier than the
     * last-pointer-grab time or later than the current X server time.
     * @return A cookie
     *
     * Releases the pointer and any queued events if you actively grabbed the pointer
     * before using `xcb_grab_pointer`, `xcb_grab_button` or within a normal button
     * press.
     *
     * EnterNotify and LeaveNotify events are generated.
     *
     */
    pub unsafe fn xcb_ungrab_pointer(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_ungrab_pointer)(c, time)
    }

    /// Returns `true` iff the symbol `xcb_ungrab_pointer` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_ungrab_pointer(&self) -> bool {
        has_sym!(self, xcb_ungrab_pointer)
    }

    /**
     * @brief Grab pointer button(s)
     *
     * @param c The connection
     * @param owner_events If 1, the \a grab_window will still get the pointer events. If 0, events are not
     * reported to the \a grab_window.
     * @param grab_window Specifies the window on which the pointer should be grabbed.
     * @param event_mask Specifies which pointer events are reported to the client.
     * \n
     * TODO: which values?
     * @param pointer_mode A bitmask of #xcb_grab_mode_t values.
     * @param pointer_mode \n
     * @param keyboard_mode A bitmask of #xcb_grab_mode_t values.
     * @param keyboard_mode \n
     * @param confine_to Specifies the window to confine the pointer in (the user will not be able to
     * move the pointer out of that window).
     * \n
     * The special value `XCB_NONE` means don't confine the pointer.
     * @param cursor Specifies the cursor that should be displayed or `XCB_NONE` to not change the
     * cursor.
     * @param button A bitmask of #xcb_button_index_t values.
     * @param button \n
     * @param modifiers The modifiers to grab.
     * \n
     * Using the special value `XCB_MOD_MASK_ANY` means grab the pointer with all
     * possible modifier combinations.
     * @return A cookie
     *
     * This request establishes a passive grab. The pointer is actively grabbed as
     * described in GrabPointer, the last-pointer-grab time is set to the time at
     * which the button was pressed (as transmitted in the ButtonPress event), and the
     * ButtonPress event is reported if all of the following conditions are true:
     *
     * The pointer is not grabbed and the specified button is logically pressed when
     * the specified modifier keys are logically down, and no other buttons or
     * modifier keys are logically down.
     *
     * The grab-window contains the pointer.
     *
     * The confine-to window (if any) is viewable.
     *
     * A passive grab on the same button/key combination does not exist on any
     * ancestor of grab-window.
     *
     * The interpretation of the remaining arguments is the same as for GrabPointer.
     * The active grab is terminated automatically when the logical state of the
     * pointer has all buttons released, independent of the logical state of modifier
     * keys. Note that the logical state of a device (as seen by means of the
     * protocol) may lag the physical state if device event processing is frozen. This
     * request overrides all previous passive grabs by the same client on the same
     * button/key combinations on the same window. A modifier of AnyModifier is
     * equivalent to issuing the request for all possible modifier combinations
     * (including the combination of no modifiers). It is not required that all
     * specified modifiers have currently assigned keycodes. A button of AnyButton is
     * equivalent to issuing the request for all possible buttons. Otherwise, it is
     * not required that the button specified currently be assigned to a physical
     * button.
     *
     * An Access error is generated if some other client has already issued a
     * GrabButton request with the same button/key combination on the same window.
     * When using AnyModifier or AnyButton, the request fails completely (no grabs are
     * established), and an Access error is generated if there is a conflicting grab
     * for any combination. The request has no effect on an active grab.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_grab_button_checked(
        &self,
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        event_mask: u16,
        pointer_mode: u8,
        keyboard_mode: u8,
        confine_to: xcb_window_t,
        cursor: xcb_cursor_t,
        button: u8,
        modifiers: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_grab_button_checked)(
            c,
            owner_events,
            grab_window,
            event_mask,
            pointer_mode,
            keyboard_mode,
            confine_to,
            cursor,
            button,
            modifiers,
        )
    }

    /// Returns `true` iff the symbol `xcb_grab_button_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_grab_button_checked(&self) -> bool {
        has_sym!(self, xcb_grab_button_checked)
    }

    /**
     * @brief Grab pointer button(s)
     *
     * @param c The connection
     * @param owner_events If 1, the \a grab_window will still get the pointer events. If 0, events are not
     * reported to the \a grab_window.
     * @param grab_window Specifies the window on which the pointer should be grabbed.
     * @param event_mask Specifies which pointer events are reported to the client.
     * \n
     * TODO: which values?
     * @param pointer_mode A bitmask of #xcb_grab_mode_t values.
     * @param pointer_mode \n
     * @param keyboard_mode A bitmask of #xcb_grab_mode_t values.
     * @param keyboard_mode \n
     * @param confine_to Specifies the window to confine the pointer in (the user will not be able to
     * move the pointer out of that window).
     * \n
     * The special value `XCB_NONE` means don't confine the pointer.
     * @param cursor Specifies the cursor that should be displayed or `XCB_NONE` to not change the
     * cursor.
     * @param button A bitmask of #xcb_button_index_t values.
     * @param button \n
     * @param modifiers The modifiers to grab.
     * \n
     * Using the special value `XCB_MOD_MASK_ANY` means grab the pointer with all
     * possible modifier combinations.
     * @return A cookie
     *
     * This request establishes a passive grab. The pointer is actively grabbed as
     * described in GrabPointer, the last-pointer-grab time is set to the time at
     * which the button was pressed (as transmitted in the ButtonPress event), and the
     * ButtonPress event is reported if all of the following conditions are true:
     *
     * The pointer is not grabbed and the specified button is logically pressed when
     * the specified modifier keys are logically down, and no other buttons or
     * modifier keys are logically down.
     *
     * The grab-window contains the pointer.
     *
     * The confine-to window (if any) is viewable.
     *
     * A passive grab on the same button/key combination does not exist on any
     * ancestor of grab-window.
     *
     * The interpretation of the remaining arguments is the same as for GrabPointer.
     * The active grab is terminated automatically when the logical state of the
     * pointer has all buttons released, independent of the logical state of modifier
     * keys. Note that the logical state of a device (as seen by means of the
     * protocol) may lag the physical state if device event processing is frozen. This
     * request overrides all previous passive grabs by the same client on the same
     * button/key combinations on the same window. A modifier of AnyModifier is
     * equivalent to issuing the request for all possible modifier combinations
     * (including the combination of no modifiers). It is not required that all
     * specified modifiers have currently assigned keycodes. A button of AnyButton is
     * equivalent to issuing the request for all possible buttons. Otherwise, it is
     * not required that the button specified currently be assigned to a physical
     * button.
     *
     * An Access error is generated if some other client has already issued a
     * GrabButton request with the same button/key combination on the same window.
     * When using AnyModifier or AnyButton, the request fails completely (no grabs are
     * established), and an Access error is generated if there is a conflicting grab
     * for any combination. The request has no effect on an active grab.
     *
     */
    pub unsafe fn xcb_grab_button(
        &self,
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        event_mask: u16,
        pointer_mode: u8,
        keyboard_mode: u8,
        confine_to: xcb_window_t,
        cursor: xcb_cursor_t,
        button: u8,
        modifiers: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_grab_button)(
            c,
            owner_events,
            grab_window,
            event_mask,
            pointer_mode,
            keyboard_mode,
            confine_to,
            cursor,
            button,
            modifiers,
        )
    }

    /// Returns `true` iff the symbol `xcb_grab_button` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_grab_button(&self) -> bool {
        has_sym!(self, xcb_grab_button)
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
    pub unsafe fn xcb_ungrab_button_checked(
        &self,
        c: *mut xcb_connection_t,
        button: u8,
        grab_window: xcb_window_t,
        modifiers: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_ungrab_button_checked)(c, button, grab_window, modifiers)
    }

    /// Returns `true` iff the symbol `xcb_ungrab_button_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_ungrab_button_checked(&self) -> bool {
        has_sym!(self, xcb_ungrab_button_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_ungrab_button(
        &self,
        c: *mut xcb_connection_t,
        button: u8,
        grab_window: xcb_window_t,
        modifiers: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_ungrab_button)(c, button, grab_window, modifiers)
    }

    /// Returns `true` iff the symbol `xcb_ungrab_button` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_ungrab_button(&self) -> bool {
        has_sym!(self, xcb_ungrab_button)
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
    pub unsafe fn xcb_change_active_pointer_grab_checked(
        &self,
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
        time: xcb_timestamp_t,
        event_mask: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_active_pointer_grab_checked)(c, cursor, time, event_mask)
    }

    /// Returns `true` iff the symbol `xcb_change_active_pointer_grab_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_active_pointer_grab_checked(&self) -> bool {
        has_sym!(self, xcb_change_active_pointer_grab_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_change_active_pointer_grab(
        &self,
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
        time: xcb_timestamp_t,
        event_mask: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_active_pointer_grab)(c, cursor, time, event_mask)
    }

    /// Returns `true` iff the symbol `xcb_change_active_pointer_grab` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_active_pointer_grab(&self) -> bool {
        has_sym!(self, xcb_change_active_pointer_grab)
    }

    /**
     * @brief Grab the keyboard
     *
     * @param c The connection
     * @param owner_events If 1, the \a grab_window will still get the pointer events. If 0, events are not
     * reported to the \a grab_window.
     * @param grab_window Specifies the window on which the pointer should be grabbed.
     * @param time Timestamp to avoid race conditions when running X over the network.
     * \n
     * The special value `XCB_CURRENT_TIME` will be replaced with the current server
     * time.
     * @param pointer_mode A bitmask of #xcb_grab_mode_t values.
     * @param pointer_mode \n
     * @param keyboard_mode A bitmask of #xcb_grab_mode_t values.
     * @param keyboard_mode \n
     * @return A cookie
     *
     * Actively grabs control of the keyboard and generates FocusIn and FocusOut
     * events. Further key events are reported only to the grabbing client.
     *
     * Any active keyboard grab by this client is overridden. If the keyboard is
     * actively grabbed by some other client, `AlreadyGrabbed` is returned. If
     * \a grab_window is not viewable, `GrabNotViewable` is returned. If the keyboard
     * is frozen by an active grab of another client, `GrabFrozen` is returned. If the
     * specified \a time is earlier than the last-keyboard-grab time or later than the
     * current X server time, `GrabInvalidTime` is returned. Otherwise, the
     * last-keyboard-grab time is set to the specified time.
     *
     */
    pub unsafe fn xcb_grab_keyboard(
        &self,
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        time: xcb_timestamp_t,
        pointer_mode: u8,
        keyboard_mode: u8,
    ) -> xcb_grab_keyboard_cookie_t {
        sym!(self, xcb_grab_keyboard)(
            c,
            owner_events,
            grab_window,
            time,
            pointer_mode,
            keyboard_mode,
        )
    }

    /// Returns `true` iff the symbol `xcb_grab_keyboard` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_grab_keyboard(&self) -> bool {
        has_sym!(self, xcb_grab_keyboard)
    }

    /**
     * @brief Grab the keyboard
     *
     * @param c The connection
     * @param owner_events If 1, the \a grab_window will still get the pointer events. If 0, events are not
     * reported to the \a grab_window.
     * @param grab_window Specifies the window on which the pointer should be grabbed.
     * @param time Timestamp to avoid race conditions when running X over the network.
     * \n
     * The special value `XCB_CURRENT_TIME` will be replaced with the current server
     * time.
     * @param pointer_mode A bitmask of #xcb_grab_mode_t values.
     * @param pointer_mode \n
     * @param keyboard_mode A bitmask of #xcb_grab_mode_t values.
     * @param keyboard_mode \n
     * @return A cookie
     *
     * Actively grabs control of the keyboard and generates FocusIn and FocusOut
     * events. Further key events are reported only to the grabbing client.
     *
     * Any active keyboard grab by this client is overridden. If the keyboard is
     * actively grabbed by some other client, `AlreadyGrabbed` is returned. If
     * \a grab_window is not viewable, `GrabNotViewable` is returned. If the keyboard
     * is frozen by an active grab of another client, `GrabFrozen` is returned. If the
     * specified \a time is earlier than the last-keyboard-grab time or later than the
     * current X server time, `GrabInvalidTime` is returned. Otherwise, the
     * last-keyboard-grab time is set to the specified time.
     *
     * This form can be used only if the request will cause
     * a reply to be generated. Any returned error will be
     * placed in the event queue.
     */
    pub unsafe fn xcb_grab_keyboard_unchecked(
        &self,
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        time: xcb_timestamp_t,
        pointer_mode: u8,
        keyboard_mode: u8,
    ) -> xcb_grab_keyboard_cookie_t {
        sym!(self, xcb_grab_keyboard_unchecked)(
            c,
            owner_events,
            grab_window,
            time,
            pointer_mode,
            keyboard_mode,
        )
    }

    /// Returns `true` iff the symbol `xcb_grab_keyboard_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_grab_keyboard_unchecked(&self) -> bool {
        has_sym!(self, xcb_grab_keyboard_unchecked)
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
     * xcb_grab_keyboard_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_grab_keyboard_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_grab_keyboard_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_grab_keyboard_reply_t {
        sym!(self, xcb_grab_keyboard_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_grab_keyboard_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_grab_keyboard_reply(&self) -> bool {
        has_sym!(self, xcb_grab_keyboard_reply)
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
    pub unsafe fn xcb_ungrab_keyboard_checked(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_ungrab_keyboard_checked)(c, time)
    }

    /// Returns `true` iff the symbol `xcb_ungrab_keyboard_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_ungrab_keyboard_checked(&self) -> bool {
        has_sym!(self, xcb_ungrab_keyboard_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_ungrab_keyboard(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_ungrab_keyboard)(c, time)
    }

    /// Returns `true` iff the symbol `xcb_ungrab_keyboard` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_ungrab_keyboard(&self) -> bool {
        has_sym!(self, xcb_ungrab_keyboard)
    }

    /**
     * @brief Grab keyboard key(s)
     *
     * @param c The connection
     * @param owner_events If 1, the \a grab_window will still get the pointer events. If 0, events are not
     * reported to the \a grab_window.
     * @param grab_window Specifies the window on which the pointer should be grabbed.
     * @param modifiers The modifiers to grab.
     * \n
     * Using the special value `XCB_MOD_MASK_ANY` means grab the pointer with all
     * possible modifier combinations.
     * @param key The keycode of the key to grab.
     * \n
     * The special value `XCB_GRAB_ANY` means grab any key.
     * @param pointer_mode A bitmask of #xcb_grab_mode_t values.
     * @param pointer_mode \n
     * @param keyboard_mode A bitmask of #xcb_grab_mode_t values.
     * @param keyboard_mode \n
     * @return A cookie
     *
     * Establishes a passive grab on the keyboard. In the future, the keyboard is
     * actively grabbed (as for `GrabKeyboard`), the last-keyboard-grab time is set to
     * the time at which the key was pressed (as transmitted in the KeyPress event),
     * and the KeyPress event is reported if all of the following conditions are true:
     *
     * The keyboard is not grabbed and the specified key (which can itself be a
     * modifier key) is logically pressed when the specified modifier keys are
     * logically down, and no other modifier keys are logically down.
     *
     * Either the grab_window is an ancestor of (or is) the focus window, or the
     * grab_window is a descendant of the focus window and contains the pointer.
     *
     * A passive grab on the same key combination does not exist on any ancestor of
     * grab_window.
     *
     * The interpretation of the remaining arguments is as for XGrabKeyboard.  The active grab is terminated
     * automatically when the logical state of the keyboard has the specified key released (independent of the
     * logical state of the modifier keys), at which point a KeyRelease event is reported to the grabbing window.
     *
     * Note that the logical state of a device (as seen by client applications) may lag the physical state if
     * device event processing is frozen.
     *
     * A modifiers argument of AnyModifier is equivalent to issuing the request for all possible modifier combinations (including the combination of no modifiers).  It is not required that all modifiers specified
     * have currently assigned KeyCodes.  A keycode argument of AnyKey is equivalent to issuing the request for
     * all possible KeyCodes.  Otherwise, the specified keycode must be in the range specified by min_keycode
     * and max_keycode in the connection setup, or a BadValue error results.
     *
     * If some other client has issued a XGrabKey with the same key combination on the same window, a BadAccess
     * error results.  When using AnyModifier or AnyKey, the request fails completely, and a BadAccess error
     * results (no grabs are established) if there is a conflicting grab for any combination.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_grab_key_checked(
        &self,
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        modifiers: u16,
        key: xcb_keycode_t,
        pointer_mode: u8,
        keyboard_mode: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_grab_key_checked)(
            c,
            owner_events,
            grab_window,
            modifiers,
            key,
            pointer_mode,
            keyboard_mode,
        )
    }

    /// Returns `true` iff the symbol `xcb_grab_key_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_grab_key_checked(&self) -> bool {
        has_sym!(self, xcb_grab_key_checked)
    }

    /**
     * @brief Grab keyboard key(s)
     *
     * @param c The connection
     * @param owner_events If 1, the \a grab_window will still get the pointer events. If 0, events are not
     * reported to the \a grab_window.
     * @param grab_window Specifies the window on which the pointer should be grabbed.
     * @param modifiers The modifiers to grab.
     * \n
     * Using the special value `XCB_MOD_MASK_ANY` means grab the pointer with all
     * possible modifier combinations.
     * @param key The keycode of the key to grab.
     * \n
     * The special value `XCB_GRAB_ANY` means grab any key.
     * @param pointer_mode A bitmask of #xcb_grab_mode_t values.
     * @param pointer_mode \n
     * @param keyboard_mode A bitmask of #xcb_grab_mode_t values.
     * @param keyboard_mode \n
     * @return A cookie
     *
     * Establishes a passive grab on the keyboard. In the future, the keyboard is
     * actively grabbed (as for `GrabKeyboard`), the last-keyboard-grab time is set to
     * the time at which the key was pressed (as transmitted in the KeyPress event),
     * and the KeyPress event is reported if all of the following conditions are true:
     *
     * The keyboard is not grabbed and the specified key (which can itself be a
     * modifier key) is logically pressed when the specified modifier keys are
     * logically down, and no other modifier keys are logically down.
     *
     * Either the grab_window is an ancestor of (or is) the focus window, or the
     * grab_window is a descendant of the focus window and contains the pointer.
     *
     * A passive grab on the same key combination does not exist on any ancestor of
     * grab_window.
     *
     * The interpretation of the remaining arguments is as for XGrabKeyboard.  The active grab is terminated
     * automatically when the logical state of the keyboard has the specified key released (independent of the
     * logical state of the modifier keys), at which point a KeyRelease event is reported to the grabbing window.
     *
     * Note that the logical state of a device (as seen by client applications) may lag the physical state if
     * device event processing is frozen.
     *
     * A modifiers argument of AnyModifier is equivalent to issuing the request for all possible modifier combinations (including the combination of no modifiers).  It is not required that all modifiers specified
     * have currently assigned KeyCodes.  A keycode argument of AnyKey is equivalent to issuing the request for
     * all possible KeyCodes.  Otherwise, the specified keycode must be in the range specified by min_keycode
     * and max_keycode in the connection setup, or a BadValue error results.
     *
     * If some other client has issued a XGrabKey with the same key combination on the same window, a BadAccess
     * error results.  When using AnyModifier or AnyKey, the request fails completely, and a BadAccess error
     * results (no grabs are established) if there is a conflicting grab for any combination.
     *
     */
    pub unsafe fn xcb_grab_key(
        &self,
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        modifiers: u16,
        key: xcb_keycode_t,
        pointer_mode: u8,
        keyboard_mode: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_grab_key)(
            c,
            owner_events,
            grab_window,
            modifiers,
            key,
            pointer_mode,
            keyboard_mode,
        )
    }

    /// Returns `true` iff the symbol `xcb_grab_key` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_grab_key(&self) -> bool {
        has_sym!(self, xcb_grab_key)
    }

    /**
     * @brief release a key combination
     *
     * @param c The connection
     * @param key The keycode of the specified key combination.
     * \n
     * Using the special value `XCB_GRAB_ANY` means releasing all possible key codes.
     * @param grab_window The window on which the grabbed key combination will be released.
     * @param modifiers The modifiers of the specified key combination.
     * \n
     * Using the special value `XCB_MOD_MASK_ANY` means releasing the key combination
     * with every possible modifier combination.
     * @return A cookie
     *
     * Releases the key combination on \a grab_window if you grabbed it using
     * `xcb_grab_key` before.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_ungrab_key_checked(
        &self,
        c: *mut xcb_connection_t,
        key: xcb_keycode_t,
        grab_window: xcb_window_t,
        modifiers: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_ungrab_key_checked)(c, key, grab_window, modifiers)
    }

    /// Returns `true` iff the symbol `xcb_ungrab_key_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_ungrab_key_checked(&self) -> bool {
        has_sym!(self, xcb_ungrab_key_checked)
    }

    /**
     * @brief release a key combination
     *
     * @param c The connection
     * @param key The keycode of the specified key combination.
     * \n
     * Using the special value `XCB_GRAB_ANY` means releasing all possible key codes.
     * @param grab_window The window on which the grabbed key combination will be released.
     * @param modifiers The modifiers of the specified key combination.
     * \n
     * Using the special value `XCB_MOD_MASK_ANY` means releasing the key combination
     * with every possible modifier combination.
     * @return A cookie
     *
     * Releases the key combination on \a grab_window if you grabbed it using
     * `xcb_grab_key` before.
     *
     */
    pub unsafe fn xcb_ungrab_key(
        &self,
        c: *mut xcb_connection_t,
        key: xcb_keycode_t,
        grab_window: xcb_window_t,
        modifiers: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_ungrab_key)(c, key, grab_window, modifiers)
    }

    /// Returns `true` iff the symbol `xcb_ungrab_key` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_ungrab_key(&self) -> bool {
        has_sym!(self, xcb_ungrab_key)
    }

    /**
     * @brief release queued events
     *
     * @param c The connection
     * @param mode A bitmask of #xcb_allow_t values.
     * @param mode \n
     * @param time Timestamp to avoid race conditions when running X over the network.
     * \n
     * The special value `XCB_CURRENT_TIME` will be replaced with the current server
     * time.
     * @return A cookie
     *
     * Releases queued events if the client has caused a device (pointer/keyboard) to
     * freeze due to grabbing it actively. This request has no effect if \a time is
     * earlier than the last-grab time of the most recent active grab for this client
     * or if \a time is later than the current X server time.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_allow_events_checked(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_allow_events_checked)(c, mode, time)
    }

    /// Returns `true` iff the symbol `xcb_allow_events_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_allow_events_checked(&self) -> bool {
        has_sym!(self, xcb_allow_events_checked)
    }

    /**
     * @brief release queued events
     *
     * @param c The connection
     * @param mode A bitmask of #xcb_allow_t values.
     * @param mode \n
     * @param time Timestamp to avoid race conditions when running X over the network.
     * \n
     * The special value `XCB_CURRENT_TIME` will be replaced with the current server
     * time.
     * @return A cookie
     *
     * Releases queued events if the client has caused a device (pointer/keyboard) to
     * freeze due to grabbing it actively. This request has no effect if \a time is
     * earlier than the last-grab time of the most recent active grab for this client
     * or if \a time is later than the current X server time.
     *
     */
    pub unsafe fn xcb_allow_events(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_allow_events)(c, mode, time)
    }

    /// Returns `true` iff the symbol `xcb_allow_events` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_allow_events(&self) -> bool {
        has_sym!(self, xcb_allow_events)
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
    pub unsafe fn xcb_grab_server_checked(&self, c: *mut xcb_connection_t) -> xcb_void_cookie_t {
        sym!(self, xcb_grab_server_checked)(c)
    }

    /// Returns `true` iff the symbol `xcb_grab_server_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_grab_server_checked(&self) -> bool {
        has_sym!(self, xcb_grab_server_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_grab_server(&self, c: *mut xcb_connection_t) -> xcb_void_cookie_t {
        sym!(self, xcb_grab_server)(c)
    }

    /// Returns `true` iff the symbol `xcb_grab_server` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_grab_server(&self) -> bool {
        has_sym!(self, xcb_grab_server)
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
    pub unsafe fn xcb_ungrab_server_checked(&self, c: *mut xcb_connection_t) -> xcb_void_cookie_t {
        sym!(self, xcb_ungrab_server_checked)(c)
    }

    /// Returns `true` iff the symbol `xcb_ungrab_server_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_ungrab_server_checked(&self) -> bool {
        has_sym!(self, xcb_ungrab_server_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_ungrab_server(&self, c: *mut xcb_connection_t) -> xcb_void_cookie_t {
        sym!(self, xcb_ungrab_server)(c)
    }

    /// Returns `true` iff the symbol `xcb_ungrab_server` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_ungrab_server(&self) -> bool {
        has_sym!(self, xcb_ungrab_server)
    }

    /**
     * @brief get pointer coordinates
     *
     * @param c The connection
     * @param window A window to check if the pointer is on the same screen as \a window (see the
     * `same_screen` field in the reply).
     * @return A cookie
     *
     * Gets the root window the pointer is logically on and the pointer coordinates
     * relative to the root window's origin.
     *
     */
    pub unsafe fn xcb_query_pointer(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_query_pointer_cookie_t {
        sym!(self, xcb_query_pointer)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_query_pointer` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_pointer(&self) -> bool {
        has_sym!(self, xcb_query_pointer)
    }

    /**
     * @brief get pointer coordinates
     *
     * @param c The connection
     * @param window A window to check if the pointer is on the same screen as \a window (see the
     * `same_screen` field in the reply).
     * @return A cookie
     *
     * Gets the root window the pointer is logically on and the pointer coordinates
     * relative to the root window's origin.
     *
     * This form can be used only if the request will cause
     * a reply to be generated. Any returned error will be
     * placed in the event queue.
     */
    pub unsafe fn xcb_query_pointer_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_query_pointer_cookie_t {
        sym!(self, xcb_query_pointer_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_query_pointer_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_pointer_unchecked(&self) -> bool {
        has_sym!(self, xcb_query_pointer_unchecked)
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
     * xcb_query_pointer_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_query_pointer_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_query_pointer_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_query_pointer_reply_t {
        sym!(self, xcb_query_pointer_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_query_pointer_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_pointer_reply(&self) -> bool {
        has_sym!(self, xcb_query_pointer_reply)
    }

    pub unsafe fn xcb_timecoord_next(&self, i: *mut xcb_timecoord_iterator_t) {
        sym!(self, xcb_timecoord_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_timecoord_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_timecoord_next(&self) -> bool {
        has_sym!(self, xcb_timecoord_next)
    }

    pub unsafe fn xcb_timecoord_end(&self, i: xcb_timecoord_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_timecoord_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_timecoord_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_timecoord_end(&self) -> bool {
        has_sym!(self, xcb_timecoord_end)
    }

    pub unsafe fn xcb_get_motion_events_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_get_motion_events_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_get_motion_events_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_motion_events_sizeof(&self) -> bool {
        has_sym!(self, xcb_get_motion_events_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_get_motion_events(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        start: xcb_timestamp_t,
        stop: xcb_timestamp_t,
    ) -> xcb_get_motion_events_cookie_t {
        sym!(self, xcb_get_motion_events)(c, window, start, stop)
    }

    /// Returns `true` iff the symbol `xcb_get_motion_events` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_motion_events(&self) -> bool {
        has_sym!(self, xcb_get_motion_events)
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
    pub unsafe fn xcb_get_motion_events_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        start: xcb_timestamp_t,
        stop: xcb_timestamp_t,
    ) -> xcb_get_motion_events_cookie_t {
        sym!(self, xcb_get_motion_events_unchecked)(c, window, start, stop)
    }

    /// Returns `true` iff the symbol `xcb_get_motion_events_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_motion_events_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_motion_events_unchecked)
    }

    pub unsafe fn xcb_get_motion_events_events(
        &self,
        r: *const xcb_get_motion_events_reply_t,
    ) -> *mut xcb_timecoord_t {
        sym!(self, xcb_get_motion_events_events)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_motion_events_events` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_motion_events_events(&self) -> bool {
        has_sym!(self, xcb_get_motion_events_events)
    }

    pub unsafe fn xcb_get_motion_events_events_length(
        &self,
        r: *const xcb_get_motion_events_reply_t,
    ) -> c_int {
        sym!(self, xcb_get_motion_events_events_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_motion_events_events_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_motion_events_events_length(&self) -> bool {
        has_sym!(self, xcb_get_motion_events_events_length)
    }

    pub unsafe fn xcb_get_motion_events_events_iterator(
        &self,
        r: *const xcb_get_motion_events_reply_t,
    ) -> xcb_timecoord_iterator_t {
        sym!(self, xcb_get_motion_events_events_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_motion_events_events_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_motion_events_events_iterator(&self) -> bool {
        has_sym!(self, xcb_get_motion_events_events_iterator)
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
     * xcb_get_motion_events_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_get_motion_events_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_motion_events_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_get_motion_events_reply_t {
        sym!(self, xcb_get_motion_events_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_motion_events_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_motion_events_reply(&self) -> bool {
        has_sym!(self, xcb_get_motion_events_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_translate_coordinates(
        &self,
        c: *mut xcb_connection_t,
        src_window: xcb_window_t,
        dst_window: xcb_window_t,
        src_x: i16,
        src_y: i16,
    ) -> xcb_translate_coordinates_cookie_t {
        sym!(self, xcb_translate_coordinates)(c, src_window, dst_window, src_x, src_y)
    }

    /// Returns `true` iff the symbol `xcb_translate_coordinates` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_translate_coordinates(&self) -> bool {
        has_sym!(self, xcb_translate_coordinates)
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
    pub unsafe fn xcb_translate_coordinates_unchecked(
        &self,
        c: *mut xcb_connection_t,
        src_window: xcb_window_t,
        dst_window: xcb_window_t,
        src_x: i16,
        src_y: i16,
    ) -> xcb_translate_coordinates_cookie_t {
        sym!(self, xcb_translate_coordinates_unchecked)(c, src_window, dst_window, src_x, src_y)
    }

    /// Returns `true` iff the symbol `xcb_translate_coordinates_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_translate_coordinates_unchecked(&self) -> bool {
        has_sym!(self, xcb_translate_coordinates_unchecked)
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
     * xcb_translate_coordinates_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_translate_coordinates_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_translate_coordinates_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_translate_coordinates_reply_t {
        sym!(self, xcb_translate_coordinates_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_translate_coordinates_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_translate_coordinates_reply(&self) -> bool {
        has_sym!(self, xcb_translate_coordinates_reply)
    }

    /**
     * @brief move mouse pointer
     *
     * @param c The connection
     * @param src_window If \a src_window is not `XCB_NONE` (TODO), the move will only take place if the
     * pointer is inside \a src_window and within the rectangle specified by (\a src_x,
     * \a src_y, \a src_width, \a src_height). The rectangle coordinates are relative to
     * \a src_window.
     * @param dst_window If \a dst_window is not `XCB_NONE` (TODO), the pointer will be moved to the
     * offsets (\a dst_x, \a dst_y) relative to \a dst_window. If \a dst_window is
     * `XCB_NONE` (TODO), the pointer will be moved by the offsets (\a dst_x, \a dst_y)
     * relative to the current position of the pointer.
     * @return A cookie
     *
     * Moves the mouse pointer to the specified position.
     *
     * If \a src_window is not `XCB_NONE` (TODO), the move will only take place if the
     * pointer is inside \a src_window and within the rectangle specified by (\a src_x,
     * \a src_y, \a src_width, \a src_height). The rectangle coordinates are relative to
     * \a src_window.
     *
     * If \a dst_window is not `XCB_NONE` (TODO), the pointer will be moved to the
     * offsets (\a dst_x, \a dst_y) relative to \a dst_window. If \a dst_window is
     * `XCB_NONE` (TODO), the pointer will be moved by the offsets (\a dst_x, \a dst_y)
     * relative to the current position of the pointer.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_warp_pointer_checked(
        &self,
        c: *mut xcb_connection_t,
        src_window: xcb_window_t,
        dst_window: xcb_window_t,
        src_x: i16,
        src_y: i16,
        src_width: u16,
        src_height: u16,
        dst_x: i16,
        dst_y: i16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_warp_pointer_checked)(
            c, src_window, dst_window, src_x, src_y, src_width, src_height, dst_x, dst_y,
        )
    }

    /// Returns `true` iff the symbol `xcb_warp_pointer_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_warp_pointer_checked(&self) -> bool {
        has_sym!(self, xcb_warp_pointer_checked)
    }

    /**
     * @brief move mouse pointer
     *
     * @param c The connection
     * @param src_window If \a src_window is not `XCB_NONE` (TODO), the move will only take place if the
     * pointer is inside \a src_window and within the rectangle specified by (\a src_x,
     * \a src_y, \a src_width, \a src_height). The rectangle coordinates are relative to
     * \a src_window.
     * @param dst_window If \a dst_window is not `XCB_NONE` (TODO), the pointer will be moved to the
     * offsets (\a dst_x, \a dst_y) relative to \a dst_window. If \a dst_window is
     * `XCB_NONE` (TODO), the pointer will be moved by the offsets (\a dst_x, \a dst_y)
     * relative to the current position of the pointer.
     * @return A cookie
     *
     * Moves the mouse pointer to the specified position.
     *
     * If \a src_window is not `XCB_NONE` (TODO), the move will only take place if the
     * pointer is inside \a src_window and within the rectangle specified by (\a src_x,
     * \a src_y, \a src_width, \a src_height). The rectangle coordinates are relative to
     * \a src_window.
     *
     * If \a dst_window is not `XCB_NONE` (TODO), the pointer will be moved to the
     * offsets (\a dst_x, \a dst_y) relative to \a dst_window. If \a dst_window is
     * `XCB_NONE` (TODO), the pointer will be moved by the offsets (\a dst_x, \a dst_y)
     * relative to the current position of the pointer.
     *
     */
    pub unsafe fn xcb_warp_pointer(
        &self,
        c: *mut xcb_connection_t,
        src_window: xcb_window_t,
        dst_window: xcb_window_t,
        src_x: i16,
        src_y: i16,
        src_width: u16,
        src_height: u16,
        dst_x: i16,
        dst_y: i16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_warp_pointer)(
            c, src_window, dst_window, src_x, src_y, src_width, src_height, dst_x, dst_y,
        )
    }

    /// Returns `true` iff the symbol `xcb_warp_pointer` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_warp_pointer(&self) -> bool {
        has_sym!(self, xcb_warp_pointer)
    }

    /**
     * @brief Sets input focus
     *
     * @param c The connection
     * @param revert_to A bitmask of #xcb_input_focus_t values.
     * @param revert_to Specifies what happens when the \a focus window becomes unviewable (if \a focus
     * is neither `XCB_NONE` nor `XCB_POINTER_ROOT`).
     * @param focus The window to focus. All keyboard events will be reported to this window. The
     * window must be viewable (TODO), or a `xcb_match_error_t` occurs (TODO).
     * \n
     * If \a focus is `XCB_NONE` (TODO), all keyboard events are
     * discarded until a new focus window is set.
     * \n
     * If \a focus is `XCB_POINTER_ROOT` (TODO), focus is on the root window of the
     * screen on which the pointer is on currently.
     * @param time Timestamp to avoid race conditions when running X over the network.
     * \n
     * The special value `XCB_CURRENT_TIME` will be replaced with the current server
     * time.
     * @return A cookie
     *
     * Changes the input focus and the last-focus-change time. If the specified \a time
     * is earlier than the current last-focus-change time, the request is ignored (to
     * avoid race conditions when running X over the network).
     *
     * A FocusIn and FocusOut event is generated when focus is changed.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_set_input_focus_checked(
        &self,
        c: *mut xcb_connection_t,
        revert_to: u8,
        focus: xcb_window_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_input_focus_checked)(c, revert_to, focus, time)
    }

    /// Returns `true` iff the symbol `xcb_set_input_focus_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_input_focus_checked(&self) -> bool {
        has_sym!(self, xcb_set_input_focus_checked)
    }

    /**
     * @brief Sets input focus
     *
     * @param c The connection
     * @param revert_to A bitmask of #xcb_input_focus_t values.
     * @param revert_to Specifies what happens when the \a focus window becomes unviewable (if \a focus
     * is neither `XCB_NONE` nor `XCB_POINTER_ROOT`).
     * @param focus The window to focus. All keyboard events will be reported to this window. The
     * window must be viewable (TODO), or a `xcb_match_error_t` occurs (TODO).
     * \n
     * If \a focus is `XCB_NONE` (TODO), all keyboard events are
     * discarded until a new focus window is set.
     * \n
     * If \a focus is `XCB_POINTER_ROOT` (TODO), focus is on the root window of the
     * screen on which the pointer is on currently.
     * @param time Timestamp to avoid race conditions when running X over the network.
     * \n
     * The special value `XCB_CURRENT_TIME` will be replaced with the current server
     * time.
     * @return A cookie
     *
     * Changes the input focus and the last-focus-change time. If the specified \a time
     * is earlier than the current last-focus-change time, the request is ignored (to
     * avoid race conditions when running X over the network).
     *
     * A FocusIn and FocusOut event is generated when focus is changed.
     *
     */
    pub unsafe fn xcb_set_input_focus(
        &self,
        c: *mut xcb_connection_t,
        revert_to: u8,
        focus: xcb_window_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_input_focus)(c, revert_to, focus, time)
    }

    /// Returns `true` iff the symbol `xcb_set_input_focus` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_input_focus(&self) -> bool {
        has_sym!(self, xcb_set_input_focus)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_get_input_focus(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_input_focus_cookie_t {
        sym!(self, xcb_get_input_focus)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_input_focus` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_input_focus(&self) -> bool {
        has_sym!(self, xcb_get_input_focus)
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
    pub unsafe fn xcb_get_input_focus_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_input_focus_cookie_t {
        sym!(self, xcb_get_input_focus_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_input_focus_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_input_focus_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_input_focus_unchecked)
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
     * xcb_get_input_focus_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_get_input_focus_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_input_focus_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_get_input_focus_reply_t {
        sym!(self, xcb_get_input_focus_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_input_focus_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_input_focus_reply(&self) -> bool {
        has_sym!(self, xcb_get_input_focus_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_query_keymap(&self, c: *mut xcb_connection_t) -> xcb_query_keymap_cookie_t {
        sym!(self, xcb_query_keymap)(c)
    }

    /// Returns `true` iff the symbol `xcb_query_keymap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_keymap(&self) -> bool {
        has_sym!(self, xcb_query_keymap)
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
    pub unsafe fn xcb_query_keymap_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_query_keymap_cookie_t {
        sym!(self, xcb_query_keymap_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_query_keymap_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_keymap_unchecked(&self) -> bool {
        has_sym!(self, xcb_query_keymap_unchecked)
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
     * xcb_query_keymap_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_query_keymap_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_query_keymap_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_query_keymap_reply_t {
        sym!(self, xcb_query_keymap_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_query_keymap_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_keymap_reply(&self) -> bool {
        has_sym!(self, xcb_query_keymap_reply)
    }

    pub unsafe fn xcb_open_font_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_open_font_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_open_font_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_open_font_sizeof(&self) -> bool {
        has_sym!(self, xcb_open_font_sizeof)
    }

    /**
     * @brief opens a font
     *
     * @param c The connection
     * @param fid The ID with which you will refer to the font, created by `xcb_generate_id`.
     * @param name_len Length (in bytes) of \a name.
     * @param name A pattern describing an X core font.
     * @return A cookie
     *
     * Opens any X core font matching the given \a name (for example "-misc-fixed-*").
     *
     * Note that X core fonts are deprecated (but still supported) in favor of
     * client-side rendering using Xft.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_open_font_checked(
        &self,
        c: *mut xcb_connection_t,
        fid: xcb_font_t,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_open_font_checked)(c, fid, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_open_font_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_open_font_checked(&self) -> bool {
        has_sym!(self, xcb_open_font_checked)
    }

    /**
     * @brief opens a font
     *
     * @param c The connection
     * @param fid The ID with which you will refer to the font, created by `xcb_generate_id`.
     * @param name_len Length (in bytes) of \a name.
     * @param name A pattern describing an X core font.
     * @return A cookie
     *
     * Opens any X core font matching the given \a name (for example "-misc-fixed-*").
     *
     * Note that X core fonts are deprecated (but still supported) in favor of
     * client-side rendering using Xft.
     *
     */
    pub unsafe fn xcb_open_font(
        &self,
        c: *mut xcb_connection_t,
        fid: xcb_font_t,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_open_font)(c, fid, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_open_font` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_open_font(&self) -> bool {
        has_sym!(self, xcb_open_font)
    }

    pub unsafe fn xcb_open_font_name(&self, r: *const xcb_open_font_request_t) -> *mut c_char {
        sym!(self, xcb_open_font_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_open_font_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_open_font_name(&self) -> bool {
        has_sym!(self, xcb_open_font_name)
    }

    pub unsafe fn xcb_open_font_name_length(&self, r: *const xcb_open_font_request_t) -> c_int {
        sym!(self, xcb_open_font_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_open_font_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_open_font_name_length(&self) -> bool {
        has_sym!(self, xcb_open_font_name_length)
    }

    pub unsafe fn xcb_open_font_name_end(
        &self,
        r: *const xcb_open_font_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_open_font_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_open_font_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_open_font_name_end(&self) -> bool {
        has_sym!(self, xcb_open_font_name_end)
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
    pub unsafe fn xcb_close_font_checked(
        &self,
        c: *mut xcb_connection_t,
        font: xcb_font_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_close_font_checked)(c, font)
    }

    /// Returns `true` iff the symbol `xcb_close_font_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_close_font_checked(&self) -> bool {
        has_sym!(self, xcb_close_font_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_close_font(
        &self,
        c: *mut xcb_connection_t,
        font: xcb_font_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_close_font)(c, font)
    }

    /// Returns `true` iff the symbol `xcb_close_font` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_close_font(&self) -> bool {
        has_sym!(self, xcb_close_font)
    }

    pub unsafe fn xcb_fontprop_next(&self, i: *mut xcb_fontprop_iterator_t) {
        sym!(self, xcb_fontprop_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_fontprop_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_fontprop_next(&self) -> bool {
        has_sym!(self, xcb_fontprop_next)
    }

    pub unsafe fn xcb_fontprop_end(&self, i: xcb_fontprop_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_fontprop_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_fontprop_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_fontprop_end(&self) -> bool {
        has_sym!(self, xcb_fontprop_end)
    }

    pub unsafe fn xcb_charinfo_next(&self, i: *mut xcb_charinfo_iterator_t) {
        sym!(self, xcb_charinfo_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_charinfo_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_charinfo_next(&self) -> bool {
        has_sym!(self, xcb_charinfo_next)
    }

    pub unsafe fn xcb_charinfo_end(&self, i: xcb_charinfo_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_charinfo_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_charinfo_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_charinfo_end(&self) -> bool {
        has_sym!(self, xcb_charinfo_end)
    }

    pub unsafe fn xcb_query_font_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_query_font_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_query_font_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_font_sizeof(&self) -> bool {
        has_sym!(self, xcb_query_font_sizeof)
    }

    /**
     * @brief query font metrics
     *
     * @param c The connection
     * @param font The fontable (Font or Graphics Context) to query.
     * @return A cookie
     *
     * Queries information associated with the font.
     *
     */
    pub unsafe fn xcb_query_font(
        &self,
        c: *mut xcb_connection_t,
        font: xcb_fontable_t,
    ) -> xcb_query_font_cookie_t {
        sym!(self, xcb_query_font)(c, font)
    }

    /// Returns `true` iff the symbol `xcb_query_font` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_font(&self) -> bool {
        has_sym!(self, xcb_query_font)
    }

    /**
     * @brief query font metrics
     *
     * @param c The connection
     * @param font The fontable (Font or Graphics Context) to query.
     * @return A cookie
     *
     * Queries information associated with the font.
     *
     * This form can be used only if the request will cause
     * a reply to be generated. Any returned error will be
     * placed in the event queue.
     */
    pub unsafe fn xcb_query_font_unchecked(
        &self,
        c: *mut xcb_connection_t,
        font: xcb_fontable_t,
    ) -> xcb_query_font_cookie_t {
        sym!(self, xcb_query_font_unchecked)(c, font)
    }

    /// Returns `true` iff the symbol `xcb_query_font_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_font_unchecked(&self) -> bool {
        has_sym!(self, xcb_query_font_unchecked)
    }

    pub unsafe fn xcb_query_font_properties(
        &self,
        r: *const xcb_query_font_reply_t,
    ) -> *mut xcb_fontprop_t {
        sym!(self, xcb_query_font_properties)(r)
    }

    /// Returns `true` iff the symbol `xcb_query_font_properties` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_font_properties(&self) -> bool {
        has_sym!(self, xcb_query_font_properties)
    }

    pub unsafe fn xcb_query_font_properties_length(
        &self,
        r: *const xcb_query_font_reply_t,
    ) -> c_int {
        sym!(self, xcb_query_font_properties_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_query_font_properties_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_font_properties_length(&self) -> bool {
        has_sym!(self, xcb_query_font_properties_length)
    }

    pub unsafe fn xcb_query_font_properties_iterator(
        &self,
        r: *const xcb_query_font_reply_t,
    ) -> xcb_fontprop_iterator_t {
        sym!(self, xcb_query_font_properties_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_query_font_properties_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_font_properties_iterator(&self) -> bool {
        has_sym!(self, xcb_query_font_properties_iterator)
    }

    pub unsafe fn xcb_query_font_char_infos(
        &self,
        r: *const xcb_query_font_reply_t,
    ) -> *mut xcb_charinfo_t {
        sym!(self, xcb_query_font_char_infos)(r)
    }

    /// Returns `true` iff the symbol `xcb_query_font_char_infos` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_font_char_infos(&self) -> bool {
        has_sym!(self, xcb_query_font_char_infos)
    }

    pub unsafe fn xcb_query_font_char_infos_length(
        &self,
        r: *const xcb_query_font_reply_t,
    ) -> c_int {
        sym!(self, xcb_query_font_char_infos_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_query_font_char_infos_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_font_char_infos_length(&self) -> bool {
        has_sym!(self, xcb_query_font_char_infos_length)
    }

    pub unsafe fn xcb_query_font_char_infos_iterator(
        &self,
        r: *const xcb_query_font_reply_t,
    ) -> xcb_charinfo_iterator_t {
        sym!(self, xcb_query_font_char_infos_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_query_font_char_infos_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_font_char_infos_iterator(&self) -> bool {
        has_sym!(self, xcb_query_font_char_infos_iterator)
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
     * xcb_query_font_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_query_font_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_query_font_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_query_font_reply_t {
        sym!(self, xcb_query_font_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_query_font_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_font_reply(&self) -> bool {
        has_sym!(self, xcb_query_font_reply)
    }

    pub unsafe fn xcb_query_text_extents_sizeof(
        &self,
        _buffer: *const c_void,
        string_len: u32,
    ) -> c_int {
        sym!(self, xcb_query_text_extents_sizeof)(_buffer, string_len)
    }

    /// Returns `true` iff the symbol `xcb_query_text_extents_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_text_extents_sizeof(&self) -> bool {
        has_sym!(self, xcb_query_text_extents_sizeof)
    }

    /**
     * @brief get text extents
     *
     * @param c The connection
     * @param font The \a font to calculate text extents in. You can also pass a graphics context.
     * @param string_len The number of characters in \a string.
     * @param string The text to get text extents for.
     * @return A cookie
     *
     * Query text extents from the X11 server. This request returns the bounding box
     * of the specified 16-bit character string in the specified \a font or the font
     * contained in the specified graphics context.
     *
     * `font_ascent` is set to the maximum of the ascent metrics of all characters in
     * the string. `font_descent` is set to the maximum of the descent metrics.
     * `overall_width` is set to the sum of the character-width metrics of all
     * characters in the string. For each character in the string, let W be the sum of
     * the character-width metrics of all characters preceding it in the string. Let L
     * be the left-side-bearing metric of the character plus W. Let R be the
     * right-side-bearing metric of the character plus W. The lbearing member is set
     * to the minimum L of all characters in the string. The rbearing member is set to
     * the maximum R.
     *
     * For fonts defined with linear indexing rather than 2-byte matrix indexing, each
     * `xcb_char2b_t` structure is interpreted as a 16-bit number with byte1 as the
     * most significant byte. If the font has no defined default character, undefined
     * characters in the string are taken to have all zero metrics.
     *
     * Characters with all zero metrics are ignored. If the font has no defined
     * default_char, the undefined characters in the string are also ignored.
     *
     */
    pub unsafe fn xcb_query_text_extents(
        &self,
        c: *mut xcb_connection_t,
        font: xcb_fontable_t,
        string_len: u32,
        string: *const xcb_char2b_t,
    ) -> xcb_query_text_extents_cookie_t {
        sym!(self, xcb_query_text_extents)(c, font, string_len, string)
    }

    /// Returns `true` iff the symbol `xcb_query_text_extents` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_text_extents(&self) -> bool {
        has_sym!(self, xcb_query_text_extents)
    }

    /**
     * @brief get text extents
     *
     * @param c The connection
     * @param font The \a font to calculate text extents in. You can also pass a graphics context.
     * @param string_len The number of characters in \a string.
     * @param string The text to get text extents for.
     * @return A cookie
     *
     * Query text extents from the X11 server. This request returns the bounding box
     * of the specified 16-bit character string in the specified \a font or the font
     * contained in the specified graphics context.
     *
     * `font_ascent` is set to the maximum of the ascent metrics of all characters in
     * the string. `font_descent` is set to the maximum of the descent metrics.
     * `overall_width` is set to the sum of the character-width metrics of all
     * characters in the string. For each character in the string, let W be the sum of
     * the character-width metrics of all characters preceding it in the string. Let L
     * be the left-side-bearing metric of the character plus W. Let R be the
     * right-side-bearing metric of the character plus W. The lbearing member is set
     * to the minimum L of all characters in the string. The rbearing member is set to
     * the maximum R.
     *
     * For fonts defined with linear indexing rather than 2-byte matrix indexing, each
     * `xcb_char2b_t` structure is interpreted as a 16-bit number with byte1 as the
     * most significant byte. If the font has no defined default character, undefined
     * characters in the string are taken to have all zero metrics.
     *
     * Characters with all zero metrics are ignored. If the font has no defined
     * default_char, the undefined characters in the string are also ignored.
     *
     * This form can be used only if the request will cause
     * a reply to be generated. Any returned error will be
     * placed in the event queue.
     */
    pub unsafe fn xcb_query_text_extents_unchecked(
        &self,
        c: *mut xcb_connection_t,
        font: xcb_fontable_t,
        string_len: u32,
        string: *const xcb_char2b_t,
    ) -> xcb_query_text_extents_cookie_t {
        sym!(self, xcb_query_text_extents_unchecked)(c, font, string_len, string)
    }

    /// Returns `true` iff the symbol `xcb_query_text_extents_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_text_extents_unchecked(&self) -> bool {
        has_sym!(self, xcb_query_text_extents_unchecked)
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
     * xcb_query_text_extents_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_query_text_extents_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_query_text_extents_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_query_text_extents_reply_t {
        sym!(self, xcb_query_text_extents_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_query_text_extents_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_text_extents_reply(&self) -> bool {
        has_sym!(self, xcb_query_text_extents_reply)
    }

    pub unsafe fn xcb_str_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_str_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_str_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_str_sizeof(&self) -> bool {
        has_sym!(self, xcb_str_sizeof)
    }

    pub unsafe fn xcb_str_name(&self, r: *const xcb_str_t) -> *mut c_char {
        sym!(self, xcb_str_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_str_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_str_name(&self) -> bool {
        has_sym!(self, xcb_str_name)
    }

    pub unsafe fn xcb_str_name_length(&self, r: *const xcb_str_t) -> c_int {
        sym!(self, xcb_str_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_str_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_str_name_length(&self) -> bool {
        has_sym!(self, xcb_str_name_length)
    }

    pub unsafe fn xcb_str_name_end(&self, r: *const xcb_str_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_str_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_str_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_str_name_end(&self) -> bool {
        has_sym!(self, xcb_str_name_end)
    }

    pub unsafe fn xcb_str_next(&self, i: *mut xcb_str_iterator_t) {
        sym!(self, xcb_str_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_str_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_str_next(&self) -> bool {
        has_sym!(self, xcb_str_next)
    }

    pub unsafe fn xcb_str_end(&self, i: xcb_str_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_str_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_str_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_str_end(&self) -> bool {
        has_sym!(self, xcb_str_end)
    }

    pub unsafe fn xcb_list_fonts_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_list_fonts_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_sizeof(&self) -> bool {
        has_sym!(self, xcb_list_fonts_sizeof)
    }

    /**
     * @brief get matching font names
     *
     * @param c The connection
     * @param max_names The maximum number of fonts to be returned.
     * @param pattern_len The length (in bytes) of \a pattern.
     * @param pattern A font pattern, for example "-misc-fixed-*".
     * \n
     * The asterisk (*) is a wildcard for any number of characters. The question mark
     * (?) is a wildcard for a single character. Use of uppercase or lowercase does
     * not matter.
     * @return A cookie
     *
     * Gets a list of available font names which match the given \a pattern.
     *
     */
    pub unsafe fn xcb_list_fonts(
        &self,
        c: *mut xcb_connection_t,
        max_names: u16,
        pattern_len: u16,
        pattern: *const c_char,
    ) -> xcb_list_fonts_cookie_t {
        sym!(self, xcb_list_fonts)(c, max_names, pattern_len, pattern)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts(&self) -> bool {
        has_sym!(self, xcb_list_fonts)
    }

    /**
     * @brief get matching font names
     *
     * @param c The connection
     * @param max_names The maximum number of fonts to be returned.
     * @param pattern_len The length (in bytes) of \a pattern.
     * @param pattern A font pattern, for example "-misc-fixed-*".
     * \n
     * The asterisk (*) is a wildcard for any number of characters. The question mark
     * (?) is a wildcard for a single character. Use of uppercase or lowercase does
     * not matter.
     * @return A cookie
     *
     * Gets a list of available font names which match the given \a pattern.
     *
     * This form can be used only if the request will cause
     * a reply to be generated. Any returned error will be
     * placed in the event queue.
     */
    pub unsafe fn xcb_list_fonts_unchecked(
        &self,
        c: *mut xcb_connection_t,
        max_names: u16,
        pattern_len: u16,
        pattern: *const c_char,
    ) -> xcb_list_fonts_cookie_t {
        sym!(self, xcb_list_fonts_unchecked)(c, max_names, pattern_len, pattern)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_unchecked(&self) -> bool {
        has_sym!(self, xcb_list_fonts_unchecked)
    }

    pub unsafe fn xcb_list_fonts_names_length(&self, r: *const xcb_list_fonts_reply_t) -> c_int {
        sym!(self, xcb_list_fonts_names_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_names_length(&self) -> bool {
        has_sym!(self, xcb_list_fonts_names_length)
    }

    pub unsafe fn xcb_list_fonts_names_iterator(
        &self,
        r: *const xcb_list_fonts_reply_t,
    ) -> xcb_str_iterator_t {
        sym!(self, xcb_list_fonts_names_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_names_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_names_iterator(&self) -> bool {
        has_sym!(self, xcb_list_fonts_names_iterator)
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
     * xcb_list_fonts_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_list_fonts_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_list_fonts_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_list_fonts_reply_t {
        sym!(self, xcb_list_fonts_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_reply(&self) -> bool {
        has_sym!(self, xcb_list_fonts_reply)
    }

    pub unsafe fn xcb_list_fonts_with_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_list_fonts_with_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_with_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_with_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_list_fonts_with_info_sizeof)
    }

    /**
     * @brief get matching font names and information
     *
     * @param c The connection
     * @param max_names The maximum number of fonts to be returned.
     * @param pattern_len The length (in bytes) of \a pattern.
     * @param pattern A font pattern, for example "-misc-fixed-*".
     * \n
     * The asterisk (*) is a wildcard for any number of characters. The question mark
     * (?) is a wildcard for a single character. Use of uppercase or lowercase does
     * not matter.
     * @return A cookie
     *
     * Gets a list of available font names which match the given \a pattern.
     *
     */
    pub unsafe fn xcb_list_fonts_with_info(
        &self,
        c: *mut xcb_connection_t,
        max_names: u16,
        pattern_len: u16,
        pattern: *const c_char,
    ) -> xcb_list_fonts_with_info_cookie_t {
        sym!(self, xcb_list_fonts_with_info)(c, max_names, pattern_len, pattern)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_with_info` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_with_info(&self) -> bool {
        has_sym!(self, xcb_list_fonts_with_info)
    }

    /**
     * @brief get matching font names and information
     *
     * @param c The connection
     * @param max_names The maximum number of fonts to be returned.
     * @param pattern_len The length (in bytes) of \a pattern.
     * @param pattern A font pattern, for example "-misc-fixed-*".
     * \n
     * The asterisk (*) is a wildcard for any number of characters. The question mark
     * (?) is a wildcard for a single character. Use of uppercase or lowercase does
     * not matter.
     * @return A cookie
     *
     * Gets a list of available font names which match the given \a pattern.
     *
     * This form can be used only if the request will cause
     * a reply to be generated. Any returned error will be
     * placed in the event queue.
     */
    pub unsafe fn xcb_list_fonts_with_info_unchecked(
        &self,
        c: *mut xcb_connection_t,
        max_names: u16,
        pattern_len: u16,
        pattern: *const c_char,
    ) -> xcb_list_fonts_with_info_cookie_t {
        sym!(self, xcb_list_fonts_with_info_unchecked)(c, max_names, pattern_len, pattern)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_with_info_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_with_info_unchecked(&self) -> bool {
        has_sym!(self, xcb_list_fonts_with_info_unchecked)
    }

    pub unsafe fn xcb_list_fonts_with_info_properties(
        &self,
        r: *const xcb_list_fonts_with_info_reply_t,
    ) -> *mut xcb_fontprop_t {
        sym!(self, xcb_list_fonts_with_info_properties)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_with_info_properties` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_with_info_properties(&self) -> bool {
        has_sym!(self, xcb_list_fonts_with_info_properties)
    }

    pub unsafe fn xcb_list_fonts_with_info_properties_length(
        &self,
        r: *const xcb_list_fonts_with_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_list_fonts_with_info_properties_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_with_info_properties_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_with_info_properties_length(&self) -> bool {
        has_sym!(self, xcb_list_fonts_with_info_properties_length)
    }

    pub unsafe fn xcb_list_fonts_with_info_properties_iterator(
        &self,
        r: *const xcb_list_fonts_with_info_reply_t,
    ) -> xcb_fontprop_iterator_t {
        sym!(self, xcb_list_fonts_with_info_properties_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_with_info_properties_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_with_info_properties_iterator(&self) -> bool {
        has_sym!(self, xcb_list_fonts_with_info_properties_iterator)
    }

    pub unsafe fn xcb_list_fonts_with_info_name(
        &self,
        r: *const xcb_list_fonts_with_info_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_list_fonts_with_info_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_with_info_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_with_info_name(&self) -> bool {
        has_sym!(self, xcb_list_fonts_with_info_name)
    }

    pub unsafe fn xcb_list_fonts_with_info_name_length(
        &self,
        r: *const xcb_list_fonts_with_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_list_fonts_with_info_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_with_info_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_with_info_name_length(&self) -> bool {
        has_sym!(self, xcb_list_fonts_with_info_name_length)
    }

    pub unsafe fn xcb_list_fonts_with_info_name_end(
        &self,
        r: *const xcb_list_fonts_with_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_list_fonts_with_info_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_with_info_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_with_info_name_end(&self) -> bool {
        has_sym!(self, xcb_list_fonts_with_info_name_end)
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
     * xcb_list_fonts_with_info_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_list_fonts_with_info_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_list_fonts_with_info_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_list_fonts_with_info_reply_t {
        sym!(self, xcb_list_fonts_with_info_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_with_info_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_with_info_reply(&self) -> bool {
        has_sym!(self, xcb_list_fonts_with_info_reply)
    }

    pub unsafe fn xcb_set_font_path_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_set_font_path_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_set_font_path_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_font_path_sizeof(&self) -> bool {
        has_sym!(self, xcb_set_font_path_sizeof)
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
    pub unsafe fn xcb_set_font_path_checked(
        &self,
        c: *mut xcb_connection_t,
        font_qty: u16,
        font: *const xcb_str_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_font_path_checked)(c, font_qty, font)
    }

    /// Returns `true` iff the symbol `xcb_set_font_path_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_font_path_checked(&self) -> bool {
        has_sym!(self, xcb_set_font_path_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_set_font_path(
        &self,
        c: *mut xcb_connection_t,
        font_qty: u16,
        font: *const xcb_str_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_font_path)(c, font_qty, font)
    }

    /// Returns `true` iff the symbol `xcb_set_font_path` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_font_path(&self) -> bool {
        has_sym!(self, xcb_set_font_path)
    }

    pub unsafe fn xcb_set_font_path_font_length(
        &self,
        r: *const xcb_set_font_path_request_t,
    ) -> c_int {
        sym!(self, xcb_set_font_path_font_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_set_font_path_font_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_font_path_font_length(&self) -> bool {
        has_sym!(self, xcb_set_font_path_font_length)
    }

    pub unsafe fn xcb_set_font_path_font_iterator(
        &self,
        r: *const xcb_set_font_path_request_t,
    ) -> xcb_str_iterator_t {
        sym!(self, xcb_set_font_path_font_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_set_font_path_font_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_font_path_font_iterator(&self) -> bool {
        has_sym!(self, xcb_set_font_path_font_iterator)
    }

    pub unsafe fn xcb_get_font_path_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_get_font_path_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_get_font_path_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_font_path_sizeof(&self) -> bool {
        has_sym!(self, xcb_get_font_path_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_get_font_path(&self, c: *mut xcb_connection_t) -> xcb_get_font_path_cookie_t {
        sym!(self, xcb_get_font_path)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_font_path` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_font_path(&self) -> bool {
        has_sym!(self, xcb_get_font_path)
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
    pub unsafe fn xcb_get_font_path_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_font_path_cookie_t {
        sym!(self, xcb_get_font_path_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_font_path_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_font_path_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_font_path_unchecked)
    }

    pub unsafe fn xcb_get_font_path_path_length(
        &self,
        r: *const xcb_get_font_path_reply_t,
    ) -> c_int {
        sym!(self, xcb_get_font_path_path_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_font_path_path_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_font_path_path_length(&self) -> bool {
        has_sym!(self, xcb_get_font_path_path_length)
    }

    pub unsafe fn xcb_get_font_path_path_iterator(
        &self,
        r: *const xcb_get_font_path_reply_t,
    ) -> xcb_str_iterator_t {
        sym!(self, xcb_get_font_path_path_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_font_path_path_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_font_path_path_iterator(&self) -> bool {
        has_sym!(self, xcb_get_font_path_path_iterator)
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
     * xcb_get_font_path_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_get_font_path_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_font_path_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_get_font_path_reply_t {
        sym!(self, xcb_get_font_path_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_font_path_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_font_path_reply(&self) -> bool {
        has_sym!(self, xcb_get_font_path_reply)
    }

    /**
     * @brief Creates a pixmap
     *
     * @param c The connection
     * @param depth TODO
     * @param pid The ID with which you will refer to the new pixmap, created by
     * `xcb_generate_id`.
     * @param drawable Drawable to get the screen from.
     * @param width The width of the new pixmap.
     * @param height The height of the new pixmap.
     * @return A cookie
     *
     * Creates a pixmap. The pixmap can only be used on the same screen as \a drawable
     * is on and only with drawables of the same \a depth.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_create_pixmap_checked(
        &self,
        c: *mut xcb_connection_t,
        depth: u8,
        pid: xcb_pixmap_t,
        drawable: xcb_drawable_t,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_pixmap_checked)(c, depth, pid, drawable, width, height)
    }

    /// Returns `true` iff the symbol `xcb_create_pixmap_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_pixmap_checked(&self) -> bool {
        has_sym!(self, xcb_create_pixmap_checked)
    }

    /**
     * @brief Creates a pixmap
     *
     * @param c The connection
     * @param depth TODO
     * @param pid The ID with which you will refer to the new pixmap, created by
     * `xcb_generate_id`.
     * @param drawable Drawable to get the screen from.
     * @param width The width of the new pixmap.
     * @param height The height of the new pixmap.
     * @return A cookie
     *
     * Creates a pixmap. The pixmap can only be used on the same screen as \a drawable
     * is on and only with drawables of the same \a depth.
     *
     */
    pub unsafe fn xcb_create_pixmap(
        &self,
        c: *mut xcb_connection_t,
        depth: u8,
        pid: xcb_pixmap_t,
        drawable: xcb_drawable_t,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_pixmap)(c, depth, pid, drawable, width, height)
    }

    /// Returns `true` iff the symbol `xcb_create_pixmap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_pixmap(&self) -> bool {
        has_sym!(self, xcb_create_pixmap)
    }

    /**
     * @brief Destroys a pixmap
     *
     * @param c The connection
     * @param pixmap The pixmap to destroy.
     * @return A cookie
     *
     * Deletes the association between the pixmap ID and the pixmap. The pixmap
     * storage will be freed when there are no more references to it.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_free_pixmap_checked(
        &self,
        c: *mut xcb_connection_t,
        pixmap: xcb_pixmap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_free_pixmap_checked)(c, pixmap)
    }

    /// Returns `true` iff the symbol `xcb_free_pixmap_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_pixmap_checked(&self) -> bool {
        has_sym!(self, xcb_free_pixmap_checked)
    }

    /**
     * @brief Destroys a pixmap
     *
     * @param c The connection
     * @param pixmap The pixmap to destroy.
     * @return A cookie
     *
     * Deletes the association between the pixmap ID and the pixmap. The pixmap
     * storage will be freed when there are no more references to it.
     *
     */
    pub unsafe fn xcb_free_pixmap(
        &self,
        c: *mut xcb_connection_t,
        pixmap: xcb_pixmap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_free_pixmap)(c, pixmap)
    }

    /// Returns `true` iff the symbol `xcb_free_pixmap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_pixmap(&self) -> bool {
        has_sym!(self, xcb_free_pixmap)
    }

    pub unsafe fn xcb_create_gc_value_list_serialize(
        &self,
        _buffer: *mut *mut c_void,
        value_mask: u32,
        _aux: *const xcb_create_gc_value_list_t,
    ) -> c_int {
        sym!(self, xcb_create_gc_value_list_serialize)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_create_gc_value_list_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_gc_value_list_serialize(&self) -> bool {
        has_sym!(self, xcb_create_gc_value_list_serialize)
    }

    pub unsafe fn xcb_create_gc_value_list_unpack(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
        _aux: *mut xcb_create_gc_value_list_t,
    ) -> c_int {
        sym!(self, xcb_create_gc_value_list_unpack)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_create_gc_value_list_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_gc_value_list_unpack(&self) -> bool {
        has_sym!(self, xcb_create_gc_value_list_unpack)
    }

    pub unsafe fn xcb_create_gc_value_list_sizeof(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
    ) -> c_int {
        sym!(self, xcb_create_gc_value_list_sizeof)(_buffer, value_mask)
    }

    /// Returns `true` iff the symbol `xcb_create_gc_value_list_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_gc_value_list_sizeof(&self) -> bool {
        has_sym!(self, xcb_create_gc_value_list_sizeof)
    }

    pub unsafe fn xcb_create_gc_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_create_gc_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_create_gc_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_gc_sizeof(&self) -> bool {
        has_sym!(self, xcb_create_gc_sizeof)
    }

    /**
     * @brief Creates a graphics context
     *
     * @param c The connection
     * @param cid The ID with which you will refer to the graphics context, created by
     * `xcb_generate_id`.
     * @param drawable Drawable to get the root/depth from.
     * @return A cookie
     *
     * Creates a graphics context. The graphics context can be used with any drawable
     * that has the same root and depth as the specified drawable.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_create_gc_checked(
        &self,
        c: *mut xcb_connection_t,
        cid: xcb_gcontext_t,
        drawable: xcb_drawable_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_gc_checked)(c, cid, drawable, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_create_gc_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_gc_checked(&self) -> bool {
        has_sym!(self, xcb_create_gc_checked)
    }

    /**
     * @brief Creates a graphics context
     *
     * @param c The connection
     * @param cid The ID with which you will refer to the graphics context, created by
     * `xcb_generate_id`.
     * @param drawable Drawable to get the root/depth from.
     * @return A cookie
     *
     * Creates a graphics context. The graphics context can be used with any drawable
     * that has the same root and depth as the specified drawable.
     *
     */
    pub unsafe fn xcb_create_gc(
        &self,
        c: *mut xcb_connection_t,
        cid: xcb_gcontext_t,
        drawable: xcb_drawable_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_gc)(c, cid, drawable, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_create_gc` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_gc(&self) -> bool {
        has_sym!(self, xcb_create_gc)
    }

    /**
     * @brief Creates a graphics context
     *
     * @param c The connection
     * @param cid The ID with which you will refer to the graphics context, created by
     * `xcb_generate_id`.
     * @param drawable Drawable to get the root/depth from.
     * @return A cookie
     *
     * Creates a graphics context. The graphics context can be used with any drawable
     * that has the same root and depth as the specified drawable.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_create_gc_aux_checked(
        &self,
        c: *mut xcb_connection_t,
        cid: xcb_gcontext_t,
        drawable: xcb_drawable_t,
        value_mask: u32,
        value_list: *const xcb_create_gc_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_gc_aux_checked)(c, cid, drawable, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_create_gc_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_gc_aux_checked(&self) -> bool {
        has_sym!(self, xcb_create_gc_aux_checked)
    }

    /**
     * @brief Creates a graphics context
     *
     * @param c The connection
     * @param cid The ID with which you will refer to the graphics context, created by
     * `xcb_generate_id`.
     * @param drawable Drawable to get the root/depth from.
     * @return A cookie
     *
     * Creates a graphics context. The graphics context can be used with any drawable
     * that has the same root and depth as the specified drawable.
     *
     */
    pub unsafe fn xcb_create_gc_aux(
        &self,
        c: *mut xcb_connection_t,
        cid: xcb_gcontext_t,
        drawable: xcb_drawable_t,
        value_mask: u32,
        value_list: *const xcb_create_gc_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_gc_aux)(c, cid, drawable, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_create_gc_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_gc_aux(&self) -> bool {
        has_sym!(self, xcb_create_gc_aux)
    }

    pub unsafe fn xcb_create_gc_value_list(
        &self,
        r: *const xcb_create_gc_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_create_gc_value_list)(r)
    }

    /// Returns `true` iff the symbol `xcb_create_gc_value_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_gc_value_list(&self) -> bool {
        has_sym!(self, xcb_create_gc_value_list)
    }

    pub unsafe fn xcb_change_gc_value_list_serialize(
        &self,
        _buffer: *mut *mut c_void,
        value_mask: u32,
        _aux: *const xcb_change_gc_value_list_t,
    ) -> c_int {
        sym!(self, xcb_change_gc_value_list_serialize)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_change_gc_value_list_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_gc_value_list_serialize(&self) -> bool {
        has_sym!(self, xcb_change_gc_value_list_serialize)
    }

    pub unsafe fn xcb_change_gc_value_list_unpack(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
        _aux: *mut xcb_change_gc_value_list_t,
    ) -> c_int {
        sym!(self, xcb_change_gc_value_list_unpack)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_change_gc_value_list_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_gc_value_list_unpack(&self) -> bool {
        has_sym!(self, xcb_change_gc_value_list_unpack)
    }

    pub unsafe fn xcb_change_gc_value_list_sizeof(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
    ) -> c_int {
        sym!(self, xcb_change_gc_value_list_sizeof)(_buffer, value_mask)
    }

    /// Returns `true` iff the symbol `xcb_change_gc_value_list_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_gc_value_list_sizeof(&self) -> bool {
        has_sym!(self, xcb_change_gc_value_list_sizeof)
    }

    pub unsafe fn xcb_change_gc_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_change_gc_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_change_gc_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_gc_sizeof(&self) -> bool {
        has_sym!(self, xcb_change_gc_sizeof)
    }

    /**
     * @brief change graphics context components
     *
     * @param c The connection
     * @param gc The graphics context to change.
     * @param value_mask A bitmask of #xcb_gc_t values.
     * @param value_mask \n
     * @param value_list Values for each of the components specified in the bitmask \a value_mask. The
     * order has to correspond to the order of possible \a value_mask bits. See the
     * example.
     * @return A cookie
     *
     * Changes the components specified by \a value_mask for the specified graphics context.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_change_gc_checked(
        &self,
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_gc_checked)(c, gc, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_change_gc_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_gc_checked(&self) -> bool {
        has_sym!(self, xcb_change_gc_checked)
    }

    /**
     * @brief change graphics context components
     *
     * @param c The connection
     * @param gc The graphics context to change.
     * @param value_mask A bitmask of #xcb_gc_t values.
     * @param value_mask \n
     * @param value_list Values for each of the components specified in the bitmask \a value_mask. The
     * order has to correspond to the order of possible \a value_mask bits. See the
     * example.
     * @return A cookie
     *
     * Changes the components specified by \a value_mask for the specified graphics context.
     *
     */
    pub unsafe fn xcb_change_gc(
        &self,
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_gc)(c, gc, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_change_gc` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_gc(&self) -> bool {
        has_sym!(self, xcb_change_gc)
    }

    /**
     * @brief change graphics context components
     *
     * @param c The connection
     * @param gc The graphics context to change.
     * @param value_mask A bitmask of #xcb_gc_t values.
     * @param value_mask \n
     * @param value_list Values for each of the components specified in the bitmask \a value_mask. The
     * order has to correspond to the order of possible \a value_mask bits. See the
     * example.
     * @return A cookie
     *
     * Changes the components specified by \a value_mask for the specified graphics context.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_change_gc_aux_checked(
        &self,
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        value_mask: u32,
        value_list: *const xcb_change_gc_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_gc_aux_checked)(c, gc, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_change_gc_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_gc_aux_checked(&self) -> bool {
        has_sym!(self, xcb_change_gc_aux_checked)
    }

    /**
     * @brief change graphics context components
     *
     * @param c The connection
     * @param gc The graphics context to change.
     * @param value_mask A bitmask of #xcb_gc_t values.
     * @param value_mask \n
     * @param value_list Values for each of the components specified in the bitmask \a value_mask. The
     * order has to correspond to the order of possible \a value_mask bits. See the
     * example.
     * @return A cookie
     *
     * Changes the components specified by \a value_mask for the specified graphics context.
     *
     */
    pub unsafe fn xcb_change_gc_aux(
        &self,
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        value_mask: u32,
        value_list: *const xcb_change_gc_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_gc_aux)(c, gc, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_change_gc_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_gc_aux(&self) -> bool {
        has_sym!(self, xcb_change_gc_aux)
    }

    pub unsafe fn xcb_change_gc_value_list(
        &self,
        r: *const xcb_change_gc_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_change_gc_value_list)(r)
    }

    /// Returns `true` iff the symbol `xcb_change_gc_value_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_gc_value_list(&self) -> bool {
        has_sym!(self, xcb_change_gc_value_list)
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
    pub unsafe fn xcb_copy_gc_checked(
        &self,
        c: *mut xcb_connection_t,
        src_gc: xcb_gcontext_t,
        dst_gc: xcb_gcontext_t,
        value_mask: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_copy_gc_checked)(c, src_gc, dst_gc, value_mask)
    }

    /// Returns `true` iff the symbol `xcb_copy_gc_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_copy_gc_checked(&self) -> bool {
        has_sym!(self, xcb_copy_gc_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_copy_gc(
        &self,
        c: *mut xcb_connection_t,
        src_gc: xcb_gcontext_t,
        dst_gc: xcb_gcontext_t,
        value_mask: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_copy_gc)(c, src_gc, dst_gc, value_mask)
    }

    /// Returns `true` iff the symbol `xcb_copy_gc` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_copy_gc(&self) -> bool {
        has_sym!(self, xcb_copy_gc)
    }

    pub unsafe fn xcb_set_dashes_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_set_dashes_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_set_dashes_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_dashes_sizeof(&self) -> bool {
        has_sym!(self, xcb_set_dashes_sizeof)
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
    pub unsafe fn xcb_set_dashes_checked(
        &self,
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        dash_offset: u16,
        dashes_len: u16,
        dashes: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_dashes_checked)(c, gc, dash_offset, dashes_len, dashes)
    }

    /// Returns `true` iff the symbol `xcb_set_dashes_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_dashes_checked(&self) -> bool {
        has_sym!(self, xcb_set_dashes_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_set_dashes(
        &self,
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        dash_offset: u16,
        dashes_len: u16,
        dashes: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_dashes)(c, gc, dash_offset, dashes_len, dashes)
    }

    /// Returns `true` iff the symbol `xcb_set_dashes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_dashes(&self) -> bool {
        has_sym!(self, xcb_set_dashes)
    }

    pub unsafe fn xcb_set_dashes_dashes(&self, r: *const xcb_set_dashes_request_t) -> *mut u8 {
        sym!(self, xcb_set_dashes_dashes)(r)
    }

    /// Returns `true` iff the symbol `xcb_set_dashes_dashes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_dashes_dashes(&self) -> bool {
        has_sym!(self, xcb_set_dashes_dashes)
    }

    pub unsafe fn xcb_set_dashes_dashes_length(&self, r: *const xcb_set_dashes_request_t) -> c_int {
        sym!(self, xcb_set_dashes_dashes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_set_dashes_dashes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_dashes_dashes_length(&self) -> bool {
        has_sym!(self, xcb_set_dashes_dashes_length)
    }

    pub unsafe fn xcb_set_dashes_dashes_end(
        &self,
        r: *const xcb_set_dashes_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_set_dashes_dashes_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_set_dashes_dashes_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_dashes_dashes_end(&self) -> bool {
        has_sym!(self, xcb_set_dashes_dashes_end)
    }

    pub unsafe fn xcb_set_clip_rectangles_sizeof(
        &self,
        _buffer: *const c_void,
        rectangles_len: u32,
    ) -> c_int {
        sym!(self, xcb_set_clip_rectangles_sizeof)(_buffer, rectangles_len)
    }

    /// Returns `true` iff the symbol `xcb_set_clip_rectangles_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_clip_rectangles_sizeof(&self) -> bool {
        has_sym!(self, xcb_set_clip_rectangles_sizeof)
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
    pub unsafe fn xcb_set_clip_rectangles_checked(
        &self,
        c: *mut xcb_connection_t,
        ordering: u8,
        gc: xcb_gcontext_t,
        clip_x_origin: i16,
        clip_y_origin: i16,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_clip_rectangles_checked)(
            c,
            ordering,
            gc,
            clip_x_origin,
            clip_y_origin,
            rectangles_len,
            rectangles,
        )
    }

    /// Returns `true` iff the symbol `xcb_set_clip_rectangles_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_clip_rectangles_checked(&self) -> bool {
        has_sym!(self, xcb_set_clip_rectangles_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_set_clip_rectangles(
        &self,
        c: *mut xcb_connection_t,
        ordering: u8,
        gc: xcb_gcontext_t,
        clip_x_origin: i16,
        clip_y_origin: i16,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_clip_rectangles)(
            c,
            ordering,
            gc,
            clip_x_origin,
            clip_y_origin,
            rectangles_len,
            rectangles,
        )
    }

    /// Returns `true` iff the symbol `xcb_set_clip_rectangles` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_clip_rectangles(&self) -> bool {
        has_sym!(self, xcb_set_clip_rectangles)
    }

    pub unsafe fn xcb_set_clip_rectangles_rectangles(
        &self,
        r: *const xcb_set_clip_rectangles_request_t,
    ) -> *mut xcb_rectangle_t {
        sym!(self, xcb_set_clip_rectangles_rectangles)(r)
    }

    /// Returns `true` iff the symbol `xcb_set_clip_rectangles_rectangles` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_clip_rectangles_rectangles(&self) -> bool {
        has_sym!(self, xcb_set_clip_rectangles_rectangles)
    }

    pub unsafe fn xcb_set_clip_rectangles_rectangles_length(
        &self,
        r: *const xcb_set_clip_rectangles_request_t,
    ) -> c_int {
        sym!(self, xcb_set_clip_rectangles_rectangles_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_set_clip_rectangles_rectangles_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_clip_rectangles_rectangles_length(&self) -> bool {
        has_sym!(self, xcb_set_clip_rectangles_rectangles_length)
    }

    pub unsafe fn xcb_set_clip_rectangles_rectangles_iterator(
        &self,
        r: *const xcb_set_clip_rectangles_request_t,
    ) -> xcb_rectangle_iterator_t {
        sym!(self, xcb_set_clip_rectangles_rectangles_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_set_clip_rectangles_rectangles_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_clip_rectangles_rectangles_iterator(&self) -> bool {
        has_sym!(self, xcb_set_clip_rectangles_rectangles_iterator)
    }

    /**
     * @brief Destroys a graphics context
     *
     * @param c The connection
     * @param gc The graphics context to destroy.
     * @return A cookie
     *
     * Destroys the specified \a gc and all associated storage.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_free_gc_checked(
        &self,
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_free_gc_checked)(c, gc)
    }

    /// Returns `true` iff the symbol `xcb_free_gc_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_gc_checked(&self) -> bool {
        has_sym!(self, xcb_free_gc_checked)
    }

    /**
     * @brief Destroys a graphics context
     *
     * @param c The connection
     * @param gc The graphics context to destroy.
     * @return A cookie
     *
     * Destroys the specified \a gc and all associated storage.
     *
     */
    pub unsafe fn xcb_free_gc(
        &self,
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_free_gc)(c, gc)
    }

    /// Returns `true` iff the symbol `xcb_free_gc` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_gc(&self) -> bool {
        has_sym!(self, xcb_free_gc)
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
    pub unsafe fn xcb_clear_area_checked(
        &self,
        c: *mut xcb_connection_t,
        exposures: u8,
        window: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_clear_area_checked)(c, exposures, window, x, y, width, height)
    }

    /// Returns `true` iff the symbol `xcb_clear_area_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_clear_area_checked(&self) -> bool {
        has_sym!(self, xcb_clear_area_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_clear_area(
        &self,
        c: *mut xcb_connection_t,
        exposures: u8,
        window: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_clear_area)(c, exposures, window, x, y, width, height)
    }

    /// Returns `true` iff the symbol `xcb_clear_area` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_clear_area(&self) -> bool {
        has_sym!(self, xcb_clear_area)
    }

    /**
     * @brief copy areas
     *
     * @param c The connection
     * @param src_drawable The source drawable (Window or Pixmap).
     * @param dst_drawable The destination drawable (Window or Pixmap).
     * @param gc The graphics context to use.
     * @param src_x The source X coordinate.
     * @param src_y The source Y coordinate.
     * @param dst_x The destination X coordinate.
     * @param dst_y The destination Y coordinate.
     * @param width The width of the area to copy (in pixels).
     * @param height The height of the area to copy (in pixels).
     * @return A cookie
     *
     * Copies the specified rectangle from \a src_drawable to \a dst_drawable.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_copy_area_checked(
        &self,
        c: *mut xcb_connection_t,
        src_drawable: xcb_drawable_t,
        dst_drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        src_x: i16,
        src_y: i16,
        dst_x: i16,
        dst_y: i16,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_copy_area_checked)(
            c,
            src_drawable,
            dst_drawable,
            gc,
            src_x,
            src_y,
            dst_x,
            dst_y,
            width,
            height,
        )
    }

    /// Returns `true` iff the symbol `xcb_copy_area_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_copy_area_checked(&self) -> bool {
        has_sym!(self, xcb_copy_area_checked)
    }

    /**
     * @brief copy areas
     *
     * @param c The connection
     * @param src_drawable The source drawable (Window or Pixmap).
     * @param dst_drawable The destination drawable (Window or Pixmap).
     * @param gc The graphics context to use.
     * @param src_x The source X coordinate.
     * @param src_y The source Y coordinate.
     * @param dst_x The destination X coordinate.
     * @param dst_y The destination Y coordinate.
     * @param width The width of the area to copy (in pixels).
     * @param height The height of the area to copy (in pixels).
     * @return A cookie
     *
     * Copies the specified rectangle from \a src_drawable to \a dst_drawable.
     *
     */
    pub unsafe fn xcb_copy_area(
        &self,
        c: *mut xcb_connection_t,
        src_drawable: xcb_drawable_t,
        dst_drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        src_x: i16,
        src_y: i16,
        dst_x: i16,
        dst_y: i16,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_copy_area)(
            c,
            src_drawable,
            dst_drawable,
            gc,
            src_x,
            src_y,
            dst_x,
            dst_y,
            width,
            height,
        )
    }

    /// Returns `true` iff the symbol `xcb_copy_area` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_copy_area(&self) -> bool {
        has_sym!(self, xcb_copy_area)
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
    pub unsafe fn xcb_copy_plane_checked(
        &self,
        c: *mut xcb_connection_t,
        src_drawable: xcb_drawable_t,
        dst_drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        src_x: i16,
        src_y: i16,
        dst_x: i16,
        dst_y: i16,
        width: u16,
        height: u16,
        bit_plane: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_copy_plane_checked)(
            c,
            src_drawable,
            dst_drawable,
            gc,
            src_x,
            src_y,
            dst_x,
            dst_y,
            width,
            height,
            bit_plane,
        )
    }

    /// Returns `true` iff the symbol `xcb_copy_plane_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_copy_plane_checked(&self) -> bool {
        has_sym!(self, xcb_copy_plane_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_copy_plane(
        &self,
        c: *mut xcb_connection_t,
        src_drawable: xcb_drawable_t,
        dst_drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        src_x: i16,
        src_y: i16,
        dst_x: i16,
        dst_y: i16,
        width: u16,
        height: u16,
        bit_plane: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_copy_plane)(
            c,
            src_drawable,
            dst_drawable,
            gc,
            src_x,
            src_y,
            dst_x,
            dst_y,
            width,
            height,
            bit_plane,
        )
    }

    /// Returns `true` iff the symbol `xcb_copy_plane` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_copy_plane(&self) -> bool {
        has_sym!(self, xcb_copy_plane)
    }

    pub unsafe fn xcb_poly_point_sizeof(&self, _buffer: *const c_void, points_len: u32) -> c_int {
        sym!(self, xcb_poly_point_sizeof)(_buffer, points_len)
    }

    /// Returns `true` iff the symbol `xcb_poly_point_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_point_sizeof(&self) -> bool {
        has_sym!(self, xcb_poly_point_sizeof)
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
    pub unsafe fn xcb_poly_point_checked(
        &self,
        c: *mut xcb_connection_t,
        coordinate_mode: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        points_len: u32,
        points: *const xcb_point_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_point_checked)(c, coordinate_mode, drawable, gc, points_len, points)
    }

    /// Returns `true` iff the symbol `xcb_poly_point_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_point_checked(&self) -> bool {
        has_sym!(self, xcb_poly_point_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_poly_point(
        &self,
        c: *mut xcb_connection_t,
        coordinate_mode: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        points_len: u32,
        points: *const xcb_point_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_point)(c, coordinate_mode, drawable, gc, points_len, points)
    }

    /// Returns `true` iff the symbol `xcb_poly_point` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_point(&self) -> bool {
        has_sym!(self, xcb_poly_point)
    }

    pub unsafe fn xcb_poly_point_points(
        &self,
        r: *const xcb_poly_point_request_t,
    ) -> *mut xcb_point_t {
        sym!(self, xcb_poly_point_points)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_point_points` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_point_points(&self) -> bool {
        has_sym!(self, xcb_poly_point_points)
    }

    pub unsafe fn xcb_poly_point_points_length(&self, r: *const xcb_poly_point_request_t) -> c_int {
        sym!(self, xcb_poly_point_points_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_point_points_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_point_points_length(&self) -> bool {
        has_sym!(self, xcb_poly_point_points_length)
    }

    pub unsafe fn xcb_poly_point_points_iterator(
        &self,
        r: *const xcb_poly_point_request_t,
    ) -> xcb_point_iterator_t {
        sym!(self, xcb_poly_point_points_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_point_points_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_point_points_iterator(&self) -> bool {
        has_sym!(self, xcb_poly_point_points_iterator)
    }

    pub unsafe fn xcb_poly_line_sizeof(&self, _buffer: *const c_void, points_len: u32) -> c_int {
        sym!(self, xcb_poly_line_sizeof)(_buffer, points_len)
    }

    /// Returns `true` iff the symbol `xcb_poly_line_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_line_sizeof(&self) -> bool {
        has_sym!(self, xcb_poly_line_sizeof)
    }

    /**
     * @brief draw lines
     *
     * @param c The connection
     * @param coordinate_mode A bitmask of #xcb_coord_mode_t values.
     * @param coordinate_mode \n
     * @param drawable The drawable to draw the line(s) on.
     * @param gc The graphics context to use.
     * @param points_len The number of `xcb_point_t` structures in \a points.
     * @param points An array of points.
     * @return A cookie
     *
     * Draws \a points_len-1 lines between each pair of points (point[i], point[i+1])
     * in the \a points array. The lines are drawn in the order listed in the array.
     * They join correctly at all intermediate points, and if the first and last
     * points coincide, the first and last lines also join correctly. For any given
     * line, a pixel is not drawn more than once. If thin (zero line-width) lines
     * intersect, the intersecting pixels are drawn multiple times. If wide lines
     * intersect, the intersecting pixels are drawn only once, as though the entire
     * request were a single, filled shape.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_poly_line_checked(
        &self,
        c: *mut xcb_connection_t,
        coordinate_mode: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        points_len: u32,
        points: *const xcb_point_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_line_checked)(c, coordinate_mode, drawable, gc, points_len, points)
    }

    /// Returns `true` iff the symbol `xcb_poly_line_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_line_checked(&self) -> bool {
        has_sym!(self, xcb_poly_line_checked)
    }

    /**
     * @brief draw lines
     *
     * @param c The connection
     * @param coordinate_mode A bitmask of #xcb_coord_mode_t values.
     * @param coordinate_mode \n
     * @param drawable The drawable to draw the line(s) on.
     * @param gc The graphics context to use.
     * @param points_len The number of `xcb_point_t` structures in \a points.
     * @param points An array of points.
     * @return A cookie
     *
     * Draws \a points_len-1 lines between each pair of points (point[i], point[i+1])
     * in the \a points array. The lines are drawn in the order listed in the array.
     * They join correctly at all intermediate points, and if the first and last
     * points coincide, the first and last lines also join correctly. For any given
     * line, a pixel is not drawn more than once. If thin (zero line-width) lines
     * intersect, the intersecting pixels are drawn multiple times. If wide lines
     * intersect, the intersecting pixels are drawn only once, as though the entire
     * request were a single, filled shape.
     *
     */
    pub unsafe fn xcb_poly_line(
        &self,
        c: *mut xcb_connection_t,
        coordinate_mode: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        points_len: u32,
        points: *const xcb_point_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_line)(c, coordinate_mode, drawable, gc, points_len, points)
    }

    /// Returns `true` iff the symbol `xcb_poly_line` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_line(&self) -> bool {
        has_sym!(self, xcb_poly_line)
    }

    pub unsafe fn xcb_poly_line_points(
        &self,
        r: *const xcb_poly_line_request_t,
    ) -> *mut xcb_point_t {
        sym!(self, xcb_poly_line_points)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_line_points` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_line_points(&self) -> bool {
        has_sym!(self, xcb_poly_line_points)
    }

    pub unsafe fn xcb_poly_line_points_length(&self, r: *const xcb_poly_line_request_t) -> c_int {
        sym!(self, xcb_poly_line_points_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_line_points_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_line_points_length(&self) -> bool {
        has_sym!(self, xcb_poly_line_points_length)
    }

    pub unsafe fn xcb_poly_line_points_iterator(
        &self,
        r: *const xcb_poly_line_request_t,
    ) -> xcb_point_iterator_t {
        sym!(self, xcb_poly_line_points_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_line_points_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_line_points_iterator(&self) -> bool {
        has_sym!(self, xcb_poly_line_points_iterator)
    }

    pub unsafe fn xcb_segment_next(&self, i: *mut xcb_segment_iterator_t) {
        sym!(self, xcb_segment_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_segment_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_segment_next(&self) -> bool {
        has_sym!(self, xcb_segment_next)
    }

    pub unsafe fn xcb_segment_end(&self, i: xcb_segment_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_segment_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_segment_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_segment_end(&self) -> bool {
        has_sym!(self, xcb_segment_end)
    }

    pub unsafe fn xcb_poly_segment_sizeof(
        &self,
        _buffer: *const c_void,
        segments_len: u32,
    ) -> c_int {
        sym!(self, xcb_poly_segment_sizeof)(_buffer, segments_len)
    }

    /// Returns `true` iff the symbol `xcb_poly_segment_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_segment_sizeof(&self) -> bool {
        has_sym!(self, xcb_poly_segment_sizeof)
    }

    /**
     * @brief draw lines
     *
     * @param c The connection
     * @param drawable A drawable (Window or Pixmap) to draw on.
     * @param gc The graphics context to use.
     * \n
     * TODO: document which attributes of a gc are used
     * @param segments_len The number of `xcb_segment_t` structures in \a segments.
     * @param segments An array of `xcb_segment_t` structures.
     * @return A cookie
     *
     * Draws multiple, unconnected lines. For each segment, a line is drawn between
     * (x1, y1) and (x2, y2). The lines are drawn in the order listed in the array of
     * `xcb_segment_t` structures and does not perform joining at coincident
     * endpoints. For any given line, a pixel is not drawn more than once. If lines
     * intersect, the intersecting pixels are drawn multiple times.
     *
     * TODO: include the xcb_segment_t data structure
     *
     * TODO: an example
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_poly_segment_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        segments_len: u32,
        segments: *const xcb_segment_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_segment_checked)(c, drawable, gc, segments_len, segments)
    }

    /// Returns `true` iff the symbol `xcb_poly_segment_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_segment_checked(&self) -> bool {
        has_sym!(self, xcb_poly_segment_checked)
    }

    /**
     * @brief draw lines
     *
     * @param c The connection
     * @param drawable A drawable (Window or Pixmap) to draw on.
     * @param gc The graphics context to use.
     * \n
     * TODO: document which attributes of a gc are used
     * @param segments_len The number of `xcb_segment_t` structures in \a segments.
     * @param segments An array of `xcb_segment_t` structures.
     * @return A cookie
     *
     * Draws multiple, unconnected lines. For each segment, a line is drawn between
     * (x1, y1) and (x2, y2). The lines are drawn in the order listed in the array of
     * `xcb_segment_t` structures and does not perform joining at coincident
     * endpoints. For any given line, a pixel is not drawn more than once. If lines
     * intersect, the intersecting pixels are drawn multiple times.
     *
     * TODO: include the xcb_segment_t data structure
     *
     * TODO: an example
     *
     */
    pub unsafe fn xcb_poly_segment(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        segments_len: u32,
        segments: *const xcb_segment_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_segment)(c, drawable, gc, segments_len, segments)
    }

    /// Returns `true` iff the symbol `xcb_poly_segment` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_segment(&self) -> bool {
        has_sym!(self, xcb_poly_segment)
    }

    pub unsafe fn xcb_poly_segment_segments(
        &self,
        r: *const xcb_poly_segment_request_t,
    ) -> *mut xcb_segment_t {
        sym!(self, xcb_poly_segment_segments)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_segment_segments` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_segment_segments(&self) -> bool {
        has_sym!(self, xcb_poly_segment_segments)
    }

    pub unsafe fn xcb_poly_segment_segments_length(
        &self,
        r: *const xcb_poly_segment_request_t,
    ) -> c_int {
        sym!(self, xcb_poly_segment_segments_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_segment_segments_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_segment_segments_length(&self) -> bool {
        has_sym!(self, xcb_poly_segment_segments_length)
    }

    pub unsafe fn xcb_poly_segment_segments_iterator(
        &self,
        r: *const xcb_poly_segment_request_t,
    ) -> xcb_segment_iterator_t {
        sym!(self, xcb_poly_segment_segments_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_segment_segments_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_segment_segments_iterator(&self) -> bool {
        has_sym!(self, xcb_poly_segment_segments_iterator)
    }

    pub unsafe fn xcb_poly_rectangle_sizeof(
        &self,
        _buffer: *const c_void,
        rectangles_len: u32,
    ) -> c_int {
        sym!(self, xcb_poly_rectangle_sizeof)(_buffer, rectangles_len)
    }

    /// Returns `true` iff the symbol `xcb_poly_rectangle_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_rectangle_sizeof(&self) -> bool {
        has_sym!(self, xcb_poly_rectangle_sizeof)
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
    pub unsafe fn xcb_poly_rectangle_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_rectangle_checked)(c, drawable, gc, rectangles_len, rectangles)
    }

    /// Returns `true` iff the symbol `xcb_poly_rectangle_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_rectangle_checked(&self) -> bool {
        has_sym!(self, xcb_poly_rectangle_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_poly_rectangle(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_rectangle)(c, drawable, gc, rectangles_len, rectangles)
    }

    /// Returns `true` iff the symbol `xcb_poly_rectangle` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_rectangle(&self) -> bool {
        has_sym!(self, xcb_poly_rectangle)
    }

    pub unsafe fn xcb_poly_rectangle_rectangles(
        &self,
        r: *const xcb_poly_rectangle_request_t,
    ) -> *mut xcb_rectangle_t {
        sym!(self, xcb_poly_rectangle_rectangles)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_rectangle_rectangles` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_rectangle_rectangles(&self) -> bool {
        has_sym!(self, xcb_poly_rectangle_rectangles)
    }

    pub unsafe fn xcb_poly_rectangle_rectangles_length(
        &self,
        r: *const xcb_poly_rectangle_request_t,
    ) -> c_int {
        sym!(self, xcb_poly_rectangle_rectangles_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_rectangle_rectangles_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_rectangle_rectangles_length(&self) -> bool {
        has_sym!(self, xcb_poly_rectangle_rectangles_length)
    }

    pub unsafe fn xcb_poly_rectangle_rectangles_iterator(
        &self,
        r: *const xcb_poly_rectangle_request_t,
    ) -> xcb_rectangle_iterator_t {
        sym!(self, xcb_poly_rectangle_rectangles_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_rectangle_rectangles_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_rectangle_rectangles_iterator(&self) -> bool {
        has_sym!(self, xcb_poly_rectangle_rectangles_iterator)
    }

    pub unsafe fn xcb_poly_arc_sizeof(&self, _buffer: *const c_void, arcs_len: u32) -> c_int {
        sym!(self, xcb_poly_arc_sizeof)(_buffer, arcs_len)
    }

    /// Returns `true` iff the symbol `xcb_poly_arc_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_arc_sizeof(&self) -> bool {
        has_sym!(self, xcb_poly_arc_sizeof)
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
    pub unsafe fn xcb_poly_arc_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        arcs_len: u32,
        arcs: *const xcb_arc_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_arc_checked)(c, drawable, gc, arcs_len, arcs)
    }

    /// Returns `true` iff the symbol `xcb_poly_arc_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_arc_checked(&self) -> bool {
        has_sym!(self, xcb_poly_arc_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_poly_arc(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        arcs_len: u32,
        arcs: *const xcb_arc_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_arc)(c, drawable, gc, arcs_len, arcs)
    }

    /// Returns `true` iff the symbol `xcb_poly_arc` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_arc(&self) -> bool {
        has_sym!(self, xcb_poly_arc)
    }

    pub unsafe fn xcb_poly_arc_arcs(&self, r: *const xcb_poly_arc_request_t) -> *mut xcb_arc_t {
        sym!(self, xcb_poly_arc_arcs)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_arc_arcs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_arc_arcs(&self) -> bool {
        has_sym!(self, xcb_poly_arc_arcs)
    }

    pub unsafe fn xcb_poly_arc_arcs_length(&self, r: *const xcb_poly_arc_request_t) -> c_int {
        sym!(self, xcb_poly_arc_arcs_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_arc_arcs_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_arc_arcs_length(&self) -> bool {
        has_sym!(self, xcb_poly_arc_arcs_length)
    }

    pub unsafe fn xcb_poly_arc_arcs_iterator(
        &self,
        r: *const xcb_poly_arc_request_t,
    ) -> xcb_arc_iterator_t {
        sym!(self, xcb_poly_arc_arcs_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_arc_arcs_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_arc_arcs_iterator(&self) -> bool {
        has_sym!(self, xcb_poly_arc_arcs_iterator)
    }

    pub unsafe fn xcb_fill_poly_sizeof(&self, _buffer: *const c_void, points_len: u32) -> c_int {
        sym!(self, xcb_fill_poly_sizeof)(_buffer, points_len)
    }

    /// Returns `true` iff the symbol `xcb_fill_poly_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_fill_poly_sizeof(&self) -> bool {
        has_sym!(self, xcb_fill_poly_sizeof)
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
    pub unsafe fn xcb_fill_poly_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        shape: u8,
        coordinate_mode: u8,
        points_len: u32,
        points: *const xcb_point_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_fill_poly_checked)(
            c,
            drawable,
            gc,
            shape,
            coordinate_mode,
            points_len,
            points,
        )
    }

    /// Returns `true` iff the symbol `xcb_fill_poly_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_fill_poly_checked(&self) -> bool {
        has_sym!(self, xcb_fill_poly_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_fill_poly(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        shape: u8,
        coordinate_mode: u8,
        points_len: u32,
        points: *const xcb_point_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_fill_poly)(c, drawable, gc, shape, coordinate_mode, points_len, points)
    }

    /// Returns `true` iff the symbol `xcb_fill_poly` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_fill_poly(&self) -> bool {
        has_sym!(self, xcb_fill_poly)
    }

    pub unsafe fn xcb_fill_poly_points(
        &self,
        r: *const xcb_fill_poly_request_t,
    ) -> *mut xcb_point_t {
        sym!(self, xcb_fill_poly_points)(r)
    }

    /// Returns `true` iff the symbol `xcb_fill_poly_points` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_fill_poly_points(&self) -> bool {
        has_sym!(self, xcb_fill_poly_points)
    }

    pub unsafe fn xcb_fill_poly_points_length(&self, r: *const xcb_fill_poly_request_t) -> c_int {
        sym!(self, xcb_fill_poly_points_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_fill_poly_points_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_fill_poly_points_length(&self) -> bool {
        has_sym!(self, xcb_fill_poly_points_length)
    }

    pub unsafe fn xcb_fill_poly_points_iterator(
        &self,
        r: *const xcb_fill_poly_request_t,
    ) -> xcb_point_iterator_t {
        sym!(self, xcb_fill_poly_points_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_fill_poly_points_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_fill_poly_points_iterator(&self) -> bool {
        has_sym!(self, xcb_fill_poly_points_iterator)
    }

    pub unsafe fn xcb_poly_fill_rectangle_sizeof(
        &self,
        _buffer: *const c_void,
        rectangles_len: u32,
    ) -> c_int {
        sym!(self, xcb_poly_fill_rectangle_sizeof)(_buffer, rectangles_len)
    }

    /// Returns `true` iff the symbol `xcb_poly_fill_rectangle_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_fill_rectangle_sizeof(&self) -> bool {
        has_sym!(self, xcb_poly_fill_rectangle_sizeof)
    }

    /**
     * @brief Fills rectangles
     *
     * @param c The connection
     * @param drawable The drawable (Window or Pixmap) to draw on.
     * @param gc The graphics context to use.
     * \n
     * The following graphics context components are used: function, plane-mask,
     * fill-style, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
     * \n
     * The following graphics context mode-dependent components are used:
     * foreground, background, tile, stipple, tile-stipple-x-origin, and
     * tile-stipple-y-origin.
     * @param rectangles_len The number of `xcb_rectangle_t` structures in \a rectangles.
     * @param rectangles The rectangles to fill.
     * @return A cookie
     *
     * Fills the specified rectangle(s) in the order listed in the array. For any
     * given rectangle, each pixel is not drawn more than once. If rectangles
     * intersect, the intersecting pixels are drawn multiple times.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_poly_fill_rectangle_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_fill_rectangle_checked)(c, drawable, gc, rectangles_len, rectangles)
    }

    /// Returns `true` iff the symbol `xcb_poly_fill_rectangle_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_fill_rectangle_checked(&self) -> bool {
        has_sym!(self, xcb_poly_fill_rectangle_checked)
    }

    /**
     * @brief Fills rectangles
     *
     * @param c The connection
     * @param drawable The drawable (Window or Pixmap) to draw on.
     * @param gc The graphics context to use.
     * \n
     * The following graphics context components are used: function, plane-mask,
     * fill-style, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
     * \n
     * The following graphics context mode-dependent components are used:
     * foreground, background, tile, stipple, tile-stipple-x-origin, and
     * tile-stipple-y-origin.
     * @param rectangles_len The number of `xcb_rectangle_t` structures in \a rectangles.
     * @param rectangles The rectangles to fill.
     * @return A cookie
     *
     * Fills the specified rectangle(s) in the order listed in the array. For any
     * given rectangle, each pixel is not drawn more than once. If rectangles
     * intersect, the intersecting pixels are drawn multiple times.
     *
     */
    pub unsafe fn xcb_poly_fill_rectangle(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_fill_rectangle)(c, drawable, gc, rectangles_len, rectangles)
    }

    /// Returns `true` iff the symbol `xcb_poly_fill_rectangle` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_fill_rectangle(&self) -> bool {
        has_sym!(self, xcb_poly_fill_rectangle)
    }

    pub unsafe fn xcb_poly_fill_rectangle_rectangles(
        &self,
        r: *const xcb_poly_fill_rectangle_request_t,
    ) -> *mut xcb_rectangle_t {
        sym!(self, xcb_poly_fill_rectangle_rectangles)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_fill_rectangle_rectangles` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_fill_rectangle_rectangles(&self) -> bool {
        has_sym!(self, xcb_poly_fill_rectangle_rectangles)
    }

    pub unsafe fn xcb_poly_fill_rectangle_rectangles_length(
        &self,
        r: *const xcb_poly_fill_rectangle_request_t,
    ) -> c_int {
        sym!(self, xcb_poly_fill_rectangle_rectangles_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_fill_rectangle_rectangles_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_fill_rectangle_rectangles_length(&self) -> bool {
        has_sym!(self, xcb_poly_fill_rectangle_rectangles_length)
    }

    pub unsafe fn xcb_poly_fill_rectangle_rectangles_iterator(
        &self,
        r: *const xcb_poly_fill_rectangle_request_t,
    ) -> xcb_rectangle_iterator_t {
        sym!(self, xcb_poly_fill_rectangle_rectangles_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_fill_rectangle_rectangles_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_fill_rectangle_rectangles_iterator(&self) -> bool {
        has_sym!(self, xcb_poly_fill_rectangle_rectangles_iterator)
    }

    pub unsafe fn xcb_poly_fill_arc_sizeof(&self, _buffer: *const c_void, arcs_len: u32) -> c_int {
        sym!(self, xcb_poly_fill_arc_sizeof)(_buffer, arcs_len)
    }

    /// Returns `true` iff the symbol `xcb_poly_fill_arc_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_fill_arc_sizeof(&self) -> bool {
        has_sym!(self, xcb_poly_fill_arc_sizeof)
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
    pub unsafe fn xcb_poly_fill_arc_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        arcs_len: u32,
        arcs: *const xcb_arc_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_fill_arc_checked)(c, drawable, gc, arcs_len, arcs)
    }

    /// Returns `true` iff the symbol `xcb_poly_fill_arc_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_fill_arc_checked(&self) -> bool {
        has_sym!(self, xcb_poly_fill_arc_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_poly_fill_arc(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        arcs_len: u32,
        arcs: *const xcb_arc_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_fill_arc)(c, drawable, gc, arcs_len, arcs)
    }

    /// Returns `true` iff the symbol `xcb_poly_fill_arc` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_fill_arc(&self) -> bool {
        has_sym!(self, xcb_poly_fill_arc)
    }

    pub unsafe fn xcb_poly_fill_arc_arcs(
        &self,
        r: *const xcb_poly_fill_arc_request_t,
    ) -> *mut xcb_arc_t {
        sym!(self, xcb_poly_fill_arc_arcs)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_fill_arc_arcs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_fill_arc_arcs(&self) -> bool {
        has_sym!(self, xcb_poly_fill_arc_arcs)
    }

    pub unsafe fn xcb_poly_fill_arc_arcs_length(
        &self,
        r: *const xcb_poly_fill_arc_request_t,
    ) -> c_int {
        sym!(self, xcb_poly_fill_arc_arcs_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_fill_arc_arcs_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_fill_arc_arcs_length(&self) -> bool {
        has_sym!(self, xcb_poly_fill_arc_arcs_length)
    }

    pub unsafe fn xcb_poly_fill_arc_arcs_iterator(
        &self,
        r: *const xcb_poly_fill_arc_request_t,
    ) -> xcb_arc_iterator_t {
        sym!(self, xcb_poly_fill_arc_arcs_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_fill_arc_arcs_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_fill_arc_arcs_iterator(&self) -> bool {
        has_sym!(self, xcb_poly_fill_arc_arcs_iterator)
    }

    pub unsafe fn xcb_put_image_sizeof(&self, _buffer: *const c_void, data_len: u32) -> c_int {
        sym!(self, xcb_put_image_sizeof)(_buffer, data_len)
    }

    /// Returns `true` iff the symbol `xcb_put_image_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_put_image_sizeof(&self) -> bool {
        has_sym!(self, xcb_put_image_sizeof)
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
    pub unsafe fn xcb_put_image_checked(
        &self,
        c: *mut xcb_connection_t,
        format: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        width: u16,
        height: u16,
        dst_x: i16,
        dst_y: i16,
        left_pad: u8,
        depth: u8,
        data_len: u32,
        data: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_put_image_checked)(
            c, format, drawable, gc, width, height, dst_x, dst_y, left_pad, depth, data_len, data,
        )
    }

    /// Returns `true` iff the symbol `xcb_put_image_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_put_image_checked(&self) -> bool {
        has_sym!(self, xcb_put_image_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_put_image(
        &self,
        c: *mut xcb_connection_t,
        format: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        width: u16,
        height: u16,
        dst_x: i16,
        dst_y: i16,
        left_pad: u8,
        depth: u8,
        data_len: u32,
        data: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_put_image)(
            c, format, drawable, gc, width, height, dst_x, dst_y, left_pad, depth, data_len, data,
        )
    }

    /// Returns `true` iff the symbol `xcb_put_image` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_put_image(&self) -> bool {
        has_sym!(self, xcb_put_image)
    }

    pub unsafe fn xcb_put_image_data(&self, r: *const xcb_put_image_request_t) -> *mut u8 {
        sym!(self, xcb_put_image_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_put_image_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_put_image_data(&self) -> bool {
        has_sym!(self, xcb_put_image_data)
    }

    pub unsafe fn xcb_put_image_data_length(&self, r: *const xcb_put_image_request_t) -> c_int {
        sym!(self, xcb_put_image_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_put_image_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_put_image_data_length(&self) -> bool {
        has_sym!(self, xcb_put_image_data_length)
    }

    pub unsafe fn xcb_put_image_data_end(
        &self,
        r: *const xcb_put_image_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_put_image_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_put_image_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_put_image_data_end(&self) -> bool {
        has_sym!(self, xcb_put_image_data_end)
    }

    pub unsafe fn xcb_get_image_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_get_image_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_get_image_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_image_sizeof(&self) -> bool {
        has_sym!(self, xcb_get_image_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_get_image(
        &self,
        c: *mut xcb_connection_t,
        format: u8,
        drawable: xcb_drawable_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        plane_mask: u32,
    ) -> xcb_get_image_cookie_t {
        sym!(self, xcb_get_image)(c, format, drawable, x, y, width, height, plane_mask)
    }

    /// Returns `true` iff the symbol `xcb_get_image` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_image(&self) -> bool {
        has_sym!(self, xcb_get_image)
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
    pub unsafe fn xcb_get_image_unchecked(
        &self,
        c: *mut xcb_connection_t,
        format: u8,
        drawable: xcb_drawable_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        plane_mask: u32,
    ) -> xcb_get_image_cookie_t {
        sym!(self, xcb_get_image_unchecked)(c, format, drawable, x, y, width, height, plane_mask)
    }

    /// Returns `true` iff the symbol `xcb_get_image_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_image_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_image_unchecked)
    }

    pub unsafe fn xcb_get_image_data(&self, r: *const xcb_get_image_reply_t) -> *mut u8 {
        sym!(self, xcb_get_image_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_image_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_image_data(&self) -> bool {
        has_sym!(self, xcb_get_image_data)
    }

    pub unsafe fn xcb_get_image_data_length(&self, r: *const xcb_get_image_reply_t) -> c_int {
        sym!(self, xcb_get_image_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_image_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_image_data_length(&self) -> bool {
        has_sym!(self, xcb_get_image_data_length)
    }

    pub unsafe fn xcb_get_image_data_end(
        &self,
        r: *const xcb_get_image_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_get_image_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_image_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_image_data_end(&self) -> bool {
        has_sym!(self, xcb_get_image_data_end)
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
     * xcb_get_image_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_get_image_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_image_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_get_image_reply_t {
        sym!(self, xcb_get_image_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_image_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_image_reply(&self) -> bool {
        has_sym!(self, xcb_get_image_reply)
    }

    pub unsafe fn xcb_poly_text_8_sizeof(&self, _buffer: *const c_void, items_len: u32) -> c_int {
        sym!(self, xcb_poly_text_8_sizeof)(_buffer, items_len)
    }

    /// Returns `true` iff the symbol `xcb_poly_text_8_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_text_8_sizeof(&self) -> bool {
        has_sym!(self, xcb_poly_text_8_sizeof)
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
    pub unsafe fn xcb_poly_text_8_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: i16,
        y: i16,
        items_len: u32,
        items: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_text_8_checked)(c, drawable, gc, x, y, items_len, items)
    }

    /// Returns `true` iff the symbol `xcb_poly_text_8_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_text_8_checked(&self) -> bool {
        has_sym!(self, xcb_poly_text_8_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_poly_text_8(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: i16,
        y: i16,
        items_len: u32,
        items: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_text_8)(c, drawable, gc, x, y, items_len, items)
    }

    /// Returns `true` iff the symbol `xcb_poly_text_8` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_text_8(&self) -> bool {
        has_sym!(self, xcb_poly_text_8)
    }

    pub unsafe fn xcb_poly_text_8_items(&self, r: *const xcb_poly_text_8_request_t) -> *mut u8 {
        sym!(self, xcb_poly_text_8_items)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_text_8_items` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_text_8_items(&self) -> bool {
        has_sym!(self, xcb_poly_text_8_items)
    }

    pub unsafe fn xcb_poly_text_8_items_length(
        &self,
        r: *const xcb_poly_text_8_request_t,
    ) -> c_int {
        sym!(self, xcb_poly_text_8_items_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_text_8_items_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_text_8_items_length(&self) -> bool {
        has_sym!(self, xcb_poly_text_8_items_length)
    }

    pub unsafe fn xcb_poly_text_8_items_end(
        &self,
        r: *const xcb_poly_text_8_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_poly_text_8_items_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_text_8_items_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_text_8_items_end(&self) -> bool {
        has_sym!(self, xcb_poly_text_8_items_end)
    }

    pub unsafe fn xcb_poly_text_16_sizeof(&self, _buffer: *const c_void, items_len: u32) -> c_int {
        sym!(self, xcb_poly_text_16_sizeof)(_buffer, items_len)
    }

    /// Returns `true` iff the symbol `xcb_poly_text_16_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_text_16_sizeof(&self) -> bool {
        has_sym!(self, xcb_poly_text_16_sizeof)
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
    pub unsafe fn xcb_poly_text_16_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: i16,
        y: i16,
        items_len: u32,
        items: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_text_16_checked)(c, drawable, gc, x, y, items_len, items)
    }

    /// Returns `true` iff the symbol `xcb_poly_text_16_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_text_16_checked(&self) -> bool {
        has_sym!(self, xcb_poly_text_16_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_poly_text_16(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: i16,
        y: i16,
        items_len: u32,
        items: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_text_16)(c, drawable, gc, x, y, items_len, items)
    }

    /// Returns `true` iff the symbol `xcb_poly_text_16` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_text_16(&self) -> bool {
        has_sym!(self, xcb_poly_text_16)
    }

    pub unsafe fn xcb_poly_text_16_items(&self, r: *const xcb_poly_text_16_request_t) -> *mut u8 {
        sym!(self, xcb_poly_text_16_items)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_text_16_items` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_text_16_items(&self) -> bool {
        has_sym!(self, xcb_poly_text_16_items)
    }

    pub unsafe fn xcb_poly_text_16_items_length(
        &self,
        r: *const xcb_poly_text_16_request_t,
    ) -> c_int {
        sym!(self, xcb_poly_text_16_items_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_text_16_items_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_text_16_items_length(&self) -> bool {
        has_sym!(self, xcb_poly_text_16_items_length)
    }

    pub unsafe fn xcb_poly_text_16_items_end(
        &self,
        r: *const xcb_poly_text_16_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_poly_text_16_items_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_text_16_items_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_text_16_items_end(&self) -> bool {
        has_sym!(self, xcb_poly_text_16_items_end)
    }

    pub unsafe fn xcb_image_text_8_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_image_text_8_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_image_text_8_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_image_text_8_sizeof(&self) -> bool {
        has_sym!(self, xcb_image_text_8_sizeof)
    }

    /**
     * @brief Draws text
     *
     * @param c The connection
     * @param string_len The length of the \a string. Note that this parameter limited by 255 due to
     * using 8 bits!
     * @param drawable The drawable (Window or Pixmap) to draw text on.
     * @param gc The graphics context to use.
     * \n
     * The following graphics context components are used: plane-mask, foreground,
     * background, font, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
     * @param x The x coordinate of the first character, relative to the origin of \a drawable.
     * @param y The y coordinate of the first character, relative to the origin of \a drawable.
     * @param string The string to draw. Only the first 255 characters are relevant due to the data
     * type of \a string_len.
     * @return A cookie
     *
     * Fills the destination rectangle with the background pixel from \a gc, then
     * paints the text with the foreground pixel from \a gc. The upper-left corner of
     * the filled rectangle is at [x, y - font-ascent]. The width is overall-width,
     * the height is font-ascent + font-descent. The overall-width, font-ascent and
     * font-descent are as returned by `xcb_query_text_extents` (TODO).
     *
     * Note that using X core fonts is deprecated (but still supported) in favor of
     * client-side rendering using Xft.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_image_text_8_checked(
        &self,
        c: *mut xcb_connection_t,
        string_len: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: i16,
        y: i16,
        string: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_image_text_8_checked)(c, string_len, drawable, gc, x, y, string)
    }

    /// Returns `true` iff the symbol `xcb_image_text_8_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_image_text_8_checked(&self) -> bool {
        has_sym!(self, xcb_image_text_8_checked)
    }

    /**
     * @brief Draws text
     *
     * @param c The connection
     * @param string_len The length of the \a string. Note that this parameter limited by 255 due to
     * using 8 bits!
     * @param drawable The drawable (Window or Pixmap) to draw text on.
     * @param gc The graphics context to use.
     * \n
     * The following graphics context components are used: plane-mask, foreground,
     * background, font, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
     * @param x The x coordinate of the first character, relative to the origin of \a drawable.
     * @param y The y coordinate of the first character, relative to the origin of \a drawable.
     * @param string The string to draw. Only the first 255 characters are relevant due to the data
     * type of \a string_len.
     * @return A cookie
     *
     * Fills the destination rectangle with the background pixel from \a gc, then
     * paints the text with the foreground pixel from \a gc. The upper-left corner of
     * the filled rectangle is at [x, y - font-ascent]. The width is overall-width,
     * the height is font-ascent + font-descent. The overall-width, font-ascent and
     * font-descent are as returned by `xcb_query_text_extents` (TODO).
     *
     * Note that using X core fonts is deprecated (but still supported) in favor of
     * client-side rendering using Xft.
     *
     */
    pub unsafe fn xcb_image_text_8(
        &self,
        c: *mut xcb_connection_t,
        string_len: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: i16,
        y: i16,
        string: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_image_text_8)(c, string_len, drawable, gc, x, y, string)
    }

    /// Returns `true` iff the symbol `xcb_image_text_8` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_image_text_8(&self) -> bool {
        has_sym!(self, xcb_image_text_8)
    }

    pub unsafe fn xcb_image_text_8_string(
        &self,
        r: *const xcb_image_text_8_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_image_text_8_string)(r)
    }

    /// Returns `true` iff the symbol `xcb_image_text_8_string` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_image_text_8_string(&self) -> bool {
        has_sym!(self, xcb_image_text_8_string)
    }

    pub unsafe fn xcb_image_text_8_string_length(
        &self,
        r: *const xcb_image_text_8_request_t,
    ) -> c_int {
        sym!(self, xcb_image_text_8_string_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_image_text_8_string_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_image_text_8_string_length(&self) -> bool {
        has_sym!(self, xcb_image_text_8_string_length)
    }

    pub unsafe fn xcb_image_text_8_string_end(
        &self,
        r: *const xcb_image_text_8_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_image_text_8_string_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_image_text_8_string_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_image_text_8_string_end(&self) -> bool {
        has_sym!(self, xcb_image_text_8_string_end)
    }

    pub unsafe fn xcb_image_text_16_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_image_text_16_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_image_text_16_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_image_text_16_sizeof(&self) -> bool {
        has_sym!(self, xcb_image_text_16_sizeof)
    }

    /**
     * @brief Draws text
     *
     * @param c The connection
     * @param string_len The length of the \a string in characters. Note that this parameter limited by
     * 255 due to using 8 bits!
     * @param drawable The drawable (Window or Pixmap) to draw text on.
     * @param gc The graphics context to use.
     * \n
     * The following graphics context components are used: plane-mask, foreground,
     * background, font, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
     * @param x The x coordinate of the first character, relative to the origin of \a drawable.
     * @param y The y coordinate of the first character, relative to the origin of \a drawable.
     * @param string The string to draw. Only the first 255 characters are relevant due to the data
     * type of \a string_len. Every character uses 2 bytes (hence the 16 in this
     * request's name).
     * @return A cookie
     *
     * Fills the destination rectangle with the background pixel from \a gc, then
     * paints the text with the foreground pixel from \a gc. The upper-left corner of
     * the filled rectangle is at [x, y - font-ascent]. The width is overall-width,
     * the height is font-ascent + font-descent. The overall-width, font-ascent and
     * font-descent are as returned by `xcb_query_text_extents` (TODO).
     *
     * Note that using X core fonts is deprecated (but still supported) in favor of
     * client-side rendering using Xft.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_image_text_16_checked(
        &self,
        c: *mut xcb_connection_t,
        string_len: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: i16,
        y: i16,
        string: *const xcb_char2b_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_image_text_16_checked)(c, string_len, drawable, gc, x, y, string)
    }

    /// Returns `true` iff the symbol `xcb_image_text_16_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_image_text_16_checked(&self) -> bool {
        has_sym!(self, xcb_image_text_16_checked)
    }

    /**
     * @brief Draws text
     *
     * @param c The connection
     * @param string_len The length of the \a string in characters. Note that this parameter limited by
     * 255 due to using 8 bits!
     * @param drawable The drawable (Window or Pixmap) to draw text on.
     * @param gc The graphics context to use.
     * \n
     * The following graphics context components are used: plane-mask, foreground,
     * background, font, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
     * @param x The x coordinate of the first character, relative to the origin of \a drawable.
     * @param y The y coordinate of the first character, relative to the origin of \a drawable.
     * @param string The string to draw. Only the first 255 characters are relevant due to the data
     * type of \a string_len. Every character uses 2 bytes (hence the 16 in this
     * request's name).
     * @return A cookie
     *
     * Fills the destination rectangle with the background pixel from \a gc, then
     * paints the text with the foreground pixel from \a gc. The upper-left corner of
     * the filled rectangle is at [x, y - font-ascent]. The width is overall-width,
     * the height is font-ascent + font-descent. The overall-width, font-ascent and
     * font-descent are as returned by `xcb_query_text_extents` (TODO).
     *
     * Note that using X core fonts is deprecated (but still supported) in favor of
     * client-side rendering using Xft.
     *
     */
    pub unsafe fn xcb_image_text_16(
        &self,
        c: *mut xcb_connection_t,
        string_len: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: i16,
        y: i16,
        string: *const xcb_char2b_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_image_text_16)(c, string_len, drawable, gc, x, y, string)
    }

    /// Returns `true` iff the symbol `xcb_image_text_16` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_image_text_16(&self) -> bool {
        has_sym!(self, xcb_image_text_16)
    }

    pub unsafe fn xcb_image_text_16_string(
        &self,
        r: *const xcb_image_text_16_request_t,
    ) -> *mut xcb_char2b_t {
        sym!(self, xcb_image_text_16_string)(r)
    }

    /// Returns `true` iff the symbol `xcb_image_text_16_string` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_image_text_16_string(&self) -> bool {
        has_sym!(self, xcb_image_text_16_string)
    }

    pub unsafe fn xcb_image_text_16_string_length(
        &self,
        r: *const xcb_image_text_16_request_t,
    ) -> c_int {
        sym!(self, xcb_image_text_16_string_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_image_text_16_string_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_image_text_16_string_length(&self) -> bool {
        has_sym!(self, xcb_image_text_16_string_length)
    }

    pub unsafe fn xcb_image_text_16_string_iterator(
        &self,
        r: *const xcb_image_text_16_request_t,
    ) -> xcb_char2b_iterator_t {
        sym!(self, xcb_image_text_16_string_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_image_text_16_string_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_image_text_16_string_iterator(&self) -> bool {
        has_sym!(self, xcb_image_text_16_string_iterator)
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
    pub unsafe fn xcb_create_colormap_checked(
        &self,
        c: *mut xcb_connection_t,
        alloc: u8,
        mid: xcb_colormap_t,
        window: xcb_window_t,
        visual: xcb_visualid_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_colormap_checked)(c, alloc, mid, window, visual)
    }

    /// Returns `true` iff the symbol `xcb_create_colormap_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_colormap_checked(&self) -> bool {
        has_sym!(self, xcb_create_colormap_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_create_colormap(
        &self,
        c: *mut xcb_connection_t,
        alloc: u8,
        mid: xcb_colormap_t,
        window: xcb_window_t,
        visual: xcb_visualid_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_colormap)(c, alloc, mid, window, visual)
    }

    /// Returns `true` iff the symbol `xcb_create_colormap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_colormap(&self) -> bool {
        has_sym!(self, xcb_create_colormap)
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
    pub unsafe fn xcb_free_colormap_checked(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_free_colormap_checked)(c, cmap)
    }

    /// Returns `true` iff the symbol `xcb_free_colormap_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_colormap_checked(&self) -> bool {
        has_sym!(self, xcb_free_colormap_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_free_colormap(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_free_colormap)(c, cmap)
    }

    /// Returns `true` iff the symbol `xcb_free_colormap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_colormap(&self) -> bool {
        has_sym!(self, xcb_free_colormap)
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
    pub unsafe fn xcb_copy_colormap_and_free_checked(
        &self,
        c: *mut xcb_connection_t,
        mid: xcb_colormap_t,
        src_cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_copy_colormap_and_free_checked)(c, mid, src_cmap)
    }

    /// Returns `true` iff the symbol `xcb_copy_colormap_and_free_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_copy_colormap_and_free_checked(&self) -> bool {
        has_sym!(self, xcb_copy_colormap_and_free_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_copy_colormap_and_free(
        &self,
        c: *mut xcb_connection_t,
        mid: xcb_colormap_t,
        src_cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_copy_colormap_and_free)(c, mid, src_cmap)
    }

    /// Returns `true` iff the symbol `xcb_copy_colormap_and_free` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_copy_colormap_and_free(&self) -> bool {
        has_sym!(self, xcb_copy_colormap_and_free)
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
    pub unsafe fn xcb_install_colormap_checked(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_install_colormap_checked)(c, cmap)
    }

    /// Returns `true` iff the symbol `xcb_install_colormap_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_install_colormap_checked(&self) -> bool {
        has_sym!(self, xcb_install_colormap_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_install_colormap(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_install_colormap)(c, cmap)
    }

    /// Returns `true` iff the symbol `xcb_install_colormap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_install_colormap(&self) -> bool {
        has_sym!(self, xcb_install_colormap)
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
    pub unsafe fn xcb_uninstall_colormap_checked(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_uninstall_colormap_checked)(c, cmap)
    }

    /// Returns `true` iff the symbol `xcb_uninstall_colormap_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_uninstall_colormap_checked(&self) -> bool {
        has_sym!(self, xcb_uninstall_colormap_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_uninstall_colormap(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_uninstall_colormap)(c, cmap)
    }

    /// Returns `true` iff the symbol `xcb_uninstall_colormap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_uninstall_colormap(&self) -> bool {
        has_sym!(self, xcb_uninstall_colormap)
    }

    pub unsafe fn xcb_list_installed_colormaps_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_list_installed_colormaps_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_list_installed_colormaps_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_installed_colormaps_sizeof(&self) -> bool {
        has_sym!(self, xcb_list_installed_colormaps_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_list_installed_colormaps(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_list_installed_colormaps_cookie_t {
        sym!(self, xcb_list_installed_colormaps)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_list_installed_colormaps` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_installed_colormaps(&self) -> bool {
        has_sym!(self, xcb_list_installed_colormaps)
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
    pub unsafe fn xcb_list_installed_colormaps_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_list_installed_colormaps_cookie_t {
        sym!(self, xcb_list_installed_colormaps_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_list_installed_colormaps_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_installed_colormaps_unchecked(&self) -> bool {
        has_sym!(self, xcb_list_installed_colormaps_unchecked)
    }

    pub unsafe fn xcb_list_installed_colormaps_cmaps(
        &self,
        r: *const xcb_list_installed_colormaps_reply_t,
    ) -> *mut xcb_colormap_t {
        sym!(self, xcb_list_installed_colormaps_cmaps)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_installed_colormaps_cmaps` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_installed_colormaps_cmaps(&self) -> bool {
        has_sym!(self, xcb_list_installed_colormaps_cmaps)
    }

    pub unsafe fn xcb_list_installed_colormaps_cmaps_length(
        &self,
        r: *const xcb_list_installed_colormaps_reply_t,
    ) -> c_int {
        sym!(self, xcb_list_installed_colormaps_cmaps_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_installed_colormaps_cmaps_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_installed_colormaps_cmaps_length(&self) -> bool {
        has_sym!(self, xcb_list_installed_colormaps_cmaps_length)
    }

    pub unsafe fn xcb_list_installed_colormaps_cmaps_end(
        &self,
        r: *const xcb_list_installed_colormaps_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_list_installed_colormaps_cmaps_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_installed_colormaps_cmaps_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_installed_colormaps_cmaps_end(&self) -> bool {
        has_sym!(self, xcb_list_installed_colormaps_cmaps_end)
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
     * xcb_list_installed_colormaps_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_list_installed_colormaps_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_list_installed_colormaps_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_list_installed_colormaps_reply_t {
        sym!(self, xcb_list_installed_colormaps_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_list_installed_colormaps_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_installed_colormaps_reply(&self) -> bool {
        has_sym!(self, xcb_list_installed_colormaps_reply)
    }

    /**
     * @brief Allocate a color
     *
     * @param c The connection
     * @param cmap TODO
     * @param red The red value of your color.
     * @param green The green value of your color.
     * @param blue The blue value of your color.
     * @return A cookie
     *
     * Allocates a read-only colormap entry corresponding to the closest RGB value
     * supported by the hardware. If you are using TrueColor, you can take a shortcut
     * and directly calculate the color pixel value to avoid the round trip. But, for
     * example, on 16-bit color setups (VNC), you can easily get the closest supported
     * RGB value to the RGB value you are specifying.
     *
     */
    pub unsafe fn xcb_alloc_color(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        red: u16,
        green: u16,
        blue: u16,
    ) -> xcb_alloc_color_cookie_t {
        sym!(self, xcb_alloc_color)(c, cmap, red, green, blue)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color(&self) -> bool {
        has_sym!(self, xcb_alloc_color)
    }

    /**
     * @brief Allocate a color
     *
     * @param c The connection
     * @param cmap TODO
     * @param red The red value of your color.
     * @param green The green value of your color.
     * @param blue The blue value of your color.
     * @return A cookie
     *
     * Allocates a read-only colormap entry corresponding to the closest RGB value
     * supported by the hardware. If you are using TrueColor, you can take a shortcut
     * and directly calculate the color pixel value to avoid the round trip. But, for
     * example, on 16-bit color setups (VNC), you can easily get the closest supported
     * RGB value to the RGB value you are specifying.
     *
     * This form can be used only if the request will cause
     * a reply to be generated. Any returned error will be
     * placed in the event queue.
     */
    pub unsafe fn xcb_alloc_color_unchecked(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        red: u16,
        green: u16,
        blue: u16,
    ) -> xcb_alloc_color_cookie_t {
        sym!(self, xcb_alloc_color_unchecked)(c, cmap, red, green, blue)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_unchecked(&self) -> bool {
        has_sym!(self, xcb_alloc_color_unchecked)
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
     * xcb_alloc_color_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_alloc_color_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_alloc_color_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_alloc_color_reply_t {
        sym!(self, xcb_alloc_color_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_reply(&self) -> bool {
        has_sym!(self, xcb_alloc_color_reply)
    }

    pub unsafe fn xcb_alloc_named_color_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_alloc_named_color_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_alloc_named_color_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_named_color_sizeof(&self) -> bool {
        has_sym!(self, xcb_alloc_named_color_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_alloc_named_color(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_alloc_named_color_cookie_t {
        sym!(self, xcb_alloc_named_color)(c, cmap, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_alloc_named_color` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_named_color(&self) -> bool {
        has_sym!(self, xcb_alloc_named_color)
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
    pub unsafe fn xcb_alloc_named_color_unchecked(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_alloc_named_color_cookie_t {
        sym!(self, xcb_alloc_named_color_unchecked)(c, cmap, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_alloc_named_color_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_named_color_unchecked(&self) -> bool {
        has_sym!(self, xcb_alloc_named_color_unchecked)
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
     * xcb_alloc_named_color_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_alloc_named_color_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_alloc_named_color_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_alloc_named_color_reply_t {
        sym!(self, xcb_alloc_named_color_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_alloc_named_color_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_named_color_reply(&self) -> bool {
        has_sym!(self, xcb_alloc_named_color_reply)
    }

    pub unsafe fn xcb_alloc_color_cells_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_alloc_color_cells_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_cells_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_cells_sizeof(&self) -> bool {
        has_sym!(self, xcb_alloc_color_cells_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_alloc_color_cells(
        &self,
        c: *mut xcb_connection_t,
        contiguous: u8,
        cmap: xcb_colormap_t,
        colors: u16,
        planes: u16,
    ) -> xcb_alloc_color_cells_cookie_t {
        sym!(self, xcb_alloc_color_cells)(c, contiguous, cmap, colors, planes)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_cells` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_cells(&self) -> bool {
        has_sym!(self, xcb_alloc_color_cells)
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
    pub unsafe fn xcb_alloc_color_cells_unchecked(
        &self,
        c: *mut xcb_connection_t,
        contiguous: u8,
        cmap: xcb_colormap_t,
        colors: u16,
        planes: u16,
    ) -> xcb_alloc_color_cells_cookie_t {
        sym!(self, xcb_alloc_color_cells_unchecked)(c, contiguous, cmap, colors, planes)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_cells_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_cells_unchecked(&self) -> bool {
        has_sym!(self, xcb_alloc_color_cells_unchecked)
    }

    pub unsafe fn xcb_alloc_color_cells_pixels(
        &self,
        r: *const xcb_alloc_color_cells_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_alloc_color_cells_pixels)(r)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_cells_pixels` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_cells_pixels(&self) -> bool {
        has_sym!(self, xcb_alloc_color_cells_pixels)
    }

    pub unsafe fn xcb_alloc_color_cells_pixels_length(
        &self,
        r: *const xcb_alloc_color_cells_reply_t,
    ) -> c_int {
        sym!(self, xcb_alloc_color_cells_pixels_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_cells_pixels_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_cells_pixels_length(&self) -> bool {
        has_sym!(self, xcb_alloc_color_cells_pixels_length)
    }

    pub unsafe fn xcb_alloc_color_cells_pixels_end(
        &self,
        r: *const xcb_alloc_color_cells_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_alloc_color_cells_pixels_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_cells_pixels_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_cells_pixels_end(&self) -> bool {
        has_sym!(self, xcb_alloc_color_cells_pixels_end)
    }

    pub unsafe fn xcb_alloc_color_cells_masks(
        &self,
        r: *const xcb_alloc_color_cells_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_alloc_color_cells_masks)(r)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_cells_masks` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_cells_masks(&self) -> bool {
        has_sym!(self, xcb_alloc_color_cells_masks)
    }

    pub unsafe fn xcb_alloc_color_cells_masks_length(
        &self,
        r: *const xcb_alloc_color_cells_reply_t,
    ) -> c_int {
        sym!(self, xcb_alloc_color_cells_masks_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_cells_masks_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_cells_masks_length(&self) -> bool {
        has_sym!(self, xcb_alloc_color_cells_masks_length)
    }

    pub unsafe fn xcb_alloc_color_cells_masks_end(
        &self,
        r: *const xcb_alloc_color_cells_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_alloc_color_cells_masks_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_cells_masks_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_cells_masks_end(&self) -> bool {
        has_sym!(self, xcb_alloc_color_cells_masks_end)
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
     * xcb_alloc_color_cells_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_alloc_color_cells_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_alloc_color_cells_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_alloc_color_cells_reply_t {
        sym!(self, xcb_alloc_color_cells_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_cells_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_cells_reply(&self) -> bool {
        has_sym!(self, xcb_alloc_color_cells_reply)
    }

    pub unsafe fn xcb_alloc_color_planes_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_alloc_color_planes_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_planes_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_planes_sizeof(&self) -> bool {
        has_sym!(self, xcb_alloc_color_planes_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_alloc_color_planes(
        &self,
        c: *mut xcb_connection_t,
        contiguous: u8,
        cmap: xcb_colormap_t,
        colors: u16,
        reds: u16,
        greens: u16,
        blues: u16,
    ) -> xcb_alloc_color_planes_cookie_t {
        sym!(self, xcb_alloc_color_planes)(c, contiguous, cmap, colors, reds, greens, blues)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_planes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_planes(&self) -> bool {
        has_sym!(self, xcb_alloc_color_planes)
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
    pub unsafe fn xcb_alloc_color_planes_unchecked(
        &self,
        c: *mut xcb_connection_t,
        contiguous: u8,
        cmap: xcb_colormap_t,
        colors: u16,
        reds: u16,
        greens: u16,
        blues: u16,
    ) -> xcb_alloc_color_planes_cookie_t {
        sym!(self, xcb_alloc_color_planes_unchecked)(
            c, contiguous, cmap, colors, reds, greens, blues,
        )
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_planes_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_planes_unchecked(&self) -> bool {
        has_sym!(self, xcb_alloc_color_planes_unchecked)
    }

    pub unsafe fn xcb_alloc_color_planes_pixels(
        &self,
        r: *const xcb_alloc_color_planes_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_alloc_color_planes_pixels)(r)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_planes_pixels` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_planes_pixels(&self) -> bool {
        has_sym!(self, xcb_alloc_color_planes_pixels)
    }

    pub unsafe fn xcb_alloc_color_planes_pixels_length(
        &self,
        r: *const xcb_alloc_color_planes_reply_t,
    ) -> c_int {
        sym!(self, xcb_alloc_color_planes_pixels_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_planes_pixels_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_planes_pixels_length(&self) -> bool {
        has_sym!(self, xcb_alloc_color_planes_pixels_length)
    }

    pub unsafe fn xcb_alloc_color_planes_pixels_end(
        &self,
        r: *const xcb_alloc_color_planes_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_alloc_color_planes_pixels_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_planes_pixels_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_planes_pixels_end(&self) -> bool {
        has_sym!(self, xcb_alloc_color_planes_pixels_end)
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
     * xcb_alloc_color_planes_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_alloc_color_planes_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_alloc_color_planes_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_alloc_color_planes_reply_t {
        sym!(self, xcb_alloc_color_planes_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_planes_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_planes_reply(&self) -> bool {
        has_sym!(self, xcb_alloc_color_planes_reply)
    }

    pub unsafe fn xcb_free_colors_sizeof(&self, _buffer: *const c_void, pixels_len: u32) -> c_int {
        sym!(self, xcb_free_colors_sizeof)(_buffer, pixels_len)
    }

    /// Returns `true` iff the symbol `xcb_free_colors_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_colors_sizeof(&self) -> bool {
        has_sym!(self, xcb_free_colors_sizeof)
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
    pub unsafe fn xcb_free_colors_checked(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        plane_mask: u32,
        pixels_len: u32,
        pixels: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_free_colors_checked)(c, cmap, plane_mask, pixels_len, pixels)
    }

    /// Returns `true` iff the symbol `xcb_free_colors_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_colors_checked(&self) -> bool {
        has_sym!(self, xcb_free_colors_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_free_colors(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        plane_mask: u32,
        pixels_len: u32,
        pixels: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_free_colors)(c, cmap, plane_mask, pixels_len, pixels)
    }

    /// Returns `true` iff the symbol `xcb_free_colors` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_colors(&self) -> bool {
        has_sym!(self, xcb_free_colors)
    }

    pub unsafe fn xcb_free_colors_pixels(&self, r: *const xcb_free_colors_request_t) -> *mut u32 {
        sym!(self, xcb_free_colors_pixels)(r)
    }

    /// Returns `true` iff the symbol `xcb_free_colors_pixels` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_colors_pixels(&self) -> bool {
        has_sym!(self, xcb_free_colors_pixels)
    }

    pub unsafe fn xcb_free_colors_pixels_length(
        &self,
        r: *const xcb_free_colors_request_t,
    ) -> c_int {
        sym!(self, xcb_free_colors_pixels_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_free_colors_pixels_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_colors_pixels_length(&self) -> bool {
        has_sym!(self, xcb_free_colors_pixels_length)
    }

    pub unsafe fn xcb_free_colors_pixels_end(
        &self,
        r: *const xcb_free_colors_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_free_colors_pixels_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_free_colors_pixels_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_colors_pixels_end(&self) -> bool {
        has_sym!(self, xcb_free_colors_pixels_end)
    }

    pub unsafe fn xcb_coloritem_next(&self, i: *mut xcb_coloritem_iterator_t) {
        sym!(self, xcb_coloritem_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_coloritem_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_coloritem_next(&self) -> bool {
        has_sym!(self, xcb_coloritem_next)
    }

    pub unsafe fn xcb_coloritem_end(&self, i: xcb_coloritem_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_coloritem_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_coloritem_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_coloritem_end(&self) -> bool {
        has_sym!(self, xcb_coloritem_end)
    }

    pub unsafe fn xcb_store_colors_sizeof(&self, _buffer: *const c_void, items_len: u32) -> c_int {
        sym!(self, xcb_store_colors_sizeof)(_buffer, items_len)
    }

    /// Returns `true` iff the symbol `xcb_store_colors_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_store_colors_sizeof(&self) -> bool {
        has_sym!(self, xcb_store_colors_sizeof)
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
    pub unsafe fn xcb_store_colors_checked(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        items_len: u32,
        items: *const xcb_coloritem_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_store_colors_checked)(c, cmap, items_len, items)
    }

    /// Returns `true` iff the symbol `xcb_store_colors_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_store_colors_checked(&self) -> bool {
        has_sym!(self, xcb_store_colors_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_store_colors(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        items_len: u32,
        items: *const xcb_coloritem_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_store_colors)(c, cmap, items_len, items)
    }

    /// Returns `true` iff the symbol `xcb_store_colors` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_store_colors(&self) -> bool {
        has_sym!(self, xcb_store_colors)
    }

    pub unsafe fn xcb_store_colors_items(
        &self,
        r: *const xcb_store_colors_request_t,
    ) -> *mut xcb_coloritem_t {
        sym!(self, xcb_store_colors_items)(r)
    }

    /// Returns `true` iff the symbol `xcb_store_colors_items` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_store_colors_items(&self) -> bool {
        has_sym!(self, xcb_store_colors_items)
    }

    pub unsafe fn xcb_store_colors_items_length(
        &self,
        r: *const xcb_store_colors_request_t,
    ) -> c_int {
        sym!(self, xcb_store_colors_items_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_store_colors_items_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_store_colors_items_length(&self) -> bool {
        has_sym!(self, xcb_store_colors_items_length)
    }

    pub unsafe fn xcb_store_colors_items_iterator(
        &self,
        r: *const xcb_store_colors_request_t,
    ) -> xcb_coloritem_iterator_t {
        sym!(self, xcb_store_colors_items_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_store_colors_items_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_store_colors_items_iterator(&self) -> bool {
        has_sym!(self, xcb_store_colors_items_iterator)
    }

    pub unsafe fn xcb_store_named_color_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_store_named_color_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_store_named_color_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_store_named_color_sizeof(&self) -> bool {
        has_sym!(self, xcb_store_named_color_sizeof)
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
    pub unsafe fn xcb_store_named_color_checked(
        &self,
        c: *mut xcb_connection_t,
        flags: u8,
        cmap: xcb_colormap_t,
        pixel: u32,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_store_named_color_checked)(c, flags, cmap, pixel, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_store_named_color_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_store_named_color_checked(&self) -> bool {
        has_sym!(self, xcb_store_named_color_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_store_named_color(
        &self,
        c: *mut xcb_connection_t,
        flags: u8,
        cmap: xcb_colormap_t,
        pixel: u32,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_store_named_color)(c, flags, cmap, pixel, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_store_named_color` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_store_named_color(&self) -> bool {
        has_sym!(self, xcb_store_named_color)
    }

    pub unsafe fn xcb_store_named_color_name(
        &self,
        r: *const xcb_store_named_color_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_store_named_color_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_store_named_color_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_store_named_color_name(&self) -> bool {
        has_sym!(self, xcb_store_named_color_name)
    }

    pub unsafe fn xcb_store_named_color_name_length(
        &self,
        r: *const xcb_store_named_color_request_t,
    ) -> c_int {
        sym!(self, xcb_store_named_color_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_store_named_color_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_store_named_color_name_length(&self) -> bool {
        has_sym!(self, xcb_store_named_color_name_length)
    }

    pub unsafe fn xcb_store_named_color_name_end(
        &self,
        r: *const xcb_store_named_color_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_store_named_color_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_store_named_color_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_store_named_color_name_end(&self) -> bool {
        has_sym!(self, xcb_store_named_color_name_end)
    }

    pub unsafe fn xcb_rgb_next(&self, i: *mut xcb_rgb_iterator_t) {
        sym!(self, xcb_rgb_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_rgb_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_rgb_next(&self) -> bool {
        has_sym!(self, xcb_rgb_next)
    }

    pub unsafe fn xcb_rgb_end(&self, i: xcb_rgb_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_rgb_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_rgb_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_rgb_end(&self) -> bool {
        has_sym!(self, xcb_rgb_end)
    }

    pub unsafe fn xcb_query_colors_sizeof(&self, _buffer: *const c_void, pixels_len: u32) -> c_int {
        sym!(self, xcb_query_colors_sizeof)(_buffer, pixels_len)
    }

    /// Returns `true` iff the symbol `xcb_query_colors_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_colors_sizeof(&self) -> bool {
        has_sym!(self, xcb_query_colors_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_query_colors(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        pixels_len: u32,
        pixels: *const u32,
    ) -> xcb_query_colors_cookie_t {
        sym!(self, xcb_query_colors)(c, cmap, pixels_len, pixels)
    }

    /// Returns `true` iff the symbol `xcb_query_colors` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_colors(&self) -> bool {
        has_sym!(self, xcb_query_colors)
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
    pub unsafe fn xcb_query_colors_unchecked(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        pixels_len: u32,
        pixels: *const u32,
    ) -> xcb_query_colors_cookie_t {
        sym!(self, xcb_query_colors_unchecked)(c, cmap, pixels_len, pixels)
    }

    /// Returns `true` iff the symbol `xcb_query_colors_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_colors_unchecked(&self) -> bool {
        has_sym!(self, xcb_query_colors_unchecked)
    }

    pub unsafe fn xcb_query_colors_colors(
        &self,
        r: *const xcb_query_colors_reply_t,
    ) -> *mut xcb_rgb_t {
        sym!(self, xcb_query_colors_colors)(r)
    }

    /// Returns `true` iff the symbol `xcb_query_colors_colors` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_colors_colors(&self) -> bool {
        has_sym!(self, xcb_query_colors_colors)
    }

    pub unsafe fn xcb_query_colors_colors_length(
        &self,
        r: *const xcb_query_colors_reply_t,
    ) -> c_int {
        sym!(self, xcb_query_colors_colors_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_query_colors_colors_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_colors_colors_length(&self) -> bool {
        has_sym!(self, xcb_query_colors_colors_length)
    }

    pub unsafe fn xcb_query_colors_colors_iterator(
        &self,
        r: *const xcb_query_colors_reply_t,
    ) -> xcb_rgb_iterator_t {
        sym!(self, xcb_query_colors_colors_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_query_colors_colors_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_colors_colors_iterator(&self) -> bool {
        has_sym!(self, xcb_query_colors_colors_iterator)
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
     * xcb_query_colors_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_query_colors_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_query_colors_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_query_colors_reply_t {
        sym!(self, xcb_query_colors_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_query_colors_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_colors_reply(&self) -> bool {
        has_sym!(self, xcb_query_colors_reply)
    }

    pub unsafe fn xcb_lookup_color_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_lookup_color_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_lookup_color_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_lookup_color_sizeof(&self) -> bool {
        has_sym!(self, xcb_lookup_color_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_lookup_color(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_lookup_color_cookie_t {
        sym!(self, xcb_lookup_color)(c, cmap, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_lookup_color` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_lookup_color(&self) -> bool {
        has_sym!(self, xcb_lookup_color)
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
    pub unsafe fn xcb_lookup_color_unchecked(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_lookup_color_cookie_t {
        sym!(self, xcb_lookup_color_unchecked)(c, cmap, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_lookup_color_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_lookup_color_unchecked(&self) -> bool {
        has_sym!(self, xcb_lookup_color_unchecked)
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
     * xcb_lookup_color_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_lookup_color_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_lookup_color_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_lookup_color_reply_t {
        sym!(self, xcb_lookup_color_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_lookup_color_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_lookup_color_reply(&self) -> bool {
        has_sym!(self, xcb_lookup_color_reply)
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
    pub unsafe fn xcb_create_cursor_checked(
        &self,
        c: *mut xcb_connection_t,
        cid: xcb_cursor_t,
        source: xcb_pixmap_t,
        mask: xcb_pixmap_t,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
        x: u16,
        y: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_cursor_checked)(
            c, cid, source, mask, fore_red, fore_green, fore_blue, back_red, back_green, back_blue,
            x, y,
        )
    }

    /// Returns `true` iff the symbol `xcb_create_cursor_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_cursor_checked(&self) -> bool {
        has_sym!(self, xcb_create_cursor_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_create_cursor(
        &self,
        c: *mut xcb_connection_t,
        cid: xcb_cursor_t,
        source: xcb_pixmap_t,
        mask: xcb_pixmap_t,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
        x: u16,
        y: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_cursor)(
            c, cid, source, mask, fore_red, fore_green, fore_blue, back_red, back_green, back_blue,
            x, y,
        )
    }

    /// Returns `true` iff the symbol `xcb_create_cursor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_cursor(&self) -> bool {
        has_sym!(self, xcb_create_cursor)
    }

    /**
     * @brief create cursor
     *
     * @param c The connection
     * @param cid The ID with which you will refer to the cursor, created by `xcb_generate_id`.
     * @param source_font In which font to look for the cursor glyph.
     * @param mask_font In which font to look for the mask glyph.
     * @param source_char The glyph of \a source_font to use.
     * @param mask_char The glyph of \a mask_font to use as a mask: Pixels which are set to 1 define
     * which source pixels are displayed. All pixels which are set to 0 are not
     * displayed.
     * @param fore_red The red value of the foreground color.
     * @param fore_green The green value of the foreground color.
     * @param fore_blue The blue value of the foreground color.
     * @param back_red The red value of the background color.
     * @param back_green The green value of the background color.
     * @param back_blue The blue value of the background color.
     * @return A cookie
     *
     * Creates a cursor from a font glyph. X provides a set of standard cursor shapes
     * in a special font named cursor. Applications are encouraged to use this
     * interface for their cursors because the font can be customized for the
     * individual display type.
     *
     * All pixels which are set to 1 in the source will use the foreground color (as
     * specified by \a fore_red, \a fore_green and \a fore_blue). All pixels set to 0
     * will use the background color (as specified by \a back_red, \a back_green and
     * \a back_blue).
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_create_glyph_cursor_checked(
        &self,
        c: *mut xcb_connection_t,
        cid: xcb_cursor_t,
        source_font: xcb_font_t,
        mask_font: xcb_font_t,
        source_char: u16,
        mask_char: u16,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_glyph_cursor_checked)(
            c,
            cid,
            source_font,
            mask_font,
            source_char,
            mask_char,
            fore_red,
            fore_green,
            fore_blue,
            back_red,
            back_green,
            back_blue,
        )
    }

    /// Returns `true` iff the symbol `xcb_create_glyph_cursor_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_glyph_cursor_checked(&self) -> bool {
        has_sym!(self, xcb_create_glyph_cursor_checked)
    }

    /**
     * @brief create cursor
     *
     * @param c The connection
     * @param cid The ID with which you will refer to the cursor, created by `xcb_generate_id`.
     * @param source_font In which font to look for the cursor glyph.
     * @param mask_font In which font to look for the mask glyph.
     * @param source_char The glyph of \a source_font to use.
     * @param mask_char The glyph of \a mask_font to use as a mask: Pixels which are set to 1 define
     * which source pixels are displayed. All pixels which are set to 0 are not
     * displayed.
     * @param fore_red The red value of the foreground color.
     * @param fore_green The green value of the foreground color.
     * @param fore_blue The blue value of the foreground color.
     * @param back_red The red value of the background color.
     * @param back_green The green value of the background color.
     * @param back_blue The blue value of the background color.
     * @return A cookie
     *
     * Creates a cursor from a font glyph. X provides a set of standard cursor shapes
     * in a special font named cursor. Applications are encouraged to use this
     * interface for their cursors because the font can be customized for the
     * individual display type.
     *
     * All pixels which are set to 1 in the source will use the foreground color (as
     * specified by \a fore_red, \a fore_green and \a fore_blue). All pixels set to 0
     * will use the background color (as specified by \a back_red, \a back_green and
     * \a back_blue).
     *
     */
    pub unsafe fn xcb_create_glyph_cursor(
        &self,
        c: *mut xcb_connection_t,
        cid: xcb_cursor_t,
        source_font: xcb_font_t,
        mask_font: xcb_font_t,
        source_char: u16,
        mask_char: u16,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_glyph_cursor)(
            c,
            cid,
            source_font,
            mask_font,
            source_char,
            mask_char,
            fore_red,
            fore_green,
            fore_blue,
            back_red,
            back_green,
            back_blue,
        )
    }

    /// Returns `true` iff the symbol `xcb_create_glyph_cursor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_glyph_cursor(&self) -> bool {
        has_sym!(self, xcb_create_glyph_cursor)
    }

    /**
     * @brief Deletes a cursor
     *
     * @param c The connection
     * @param cursor The cursor to destroy.
     * @return A cookie
     *
     * Deletes the association between the cursor resource ID and the specified
     * cursor. The cursor is freed when no other resource references it.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_free_cursor_checked(
        &self,
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_free_cursor_checked)(c, cursor)
    }

    /// Returns `true` iff the symbol `xcb_free_cursor_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_cursor_checked(&self) -> bool {
        has_sym!(self, xcb_free_cursor_checked)
    }

    /**
     * @brief Deletes a cursor
     *
     * @param c The connection
     * @param cursor The cursor to destroy.
     * @return A cookie
     *
     * Deletes the association between the cursor resource ID and the specified
     * cursor. The cursor is freed when no other resource references it.
     *
     */
    pub unsafe fn xcb_free_cursor(
        &self,
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_free_cursor)(c, cursor)
    }

    /// Returns `true` iff the symbol `xcb_free_cursor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_cursor(&self) -> bool {
        has_sym!(self, xcb_free_cursor)
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
    pub unsafe fn xcb_recolor_cursor_checked(
        &self,
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_recolor_cursor_checked)(
            c, cursor, fore_red, fore_green, fore_blue, back_red, back_green, back_blue,
        )
    }

    /// Returns `true` iff the symbol `xcb_recolor_cursor_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_recolor_cursor_checked(&self) -> bool {
        has_sym!(self, xcb_recolor_cursor_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_recolor_cursor(
        &self,
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_recolor_cursor)(
            c, cursor, fore_red, fore_green, fore_blue, back_red, back_green, back_blue,
        )
    }

    /// Returns `true` iff the symbol `xcb_recolor_cursor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_recolor_cursor(&self) -> bool {
        has_sym!(self, xcb_recolor_cursor)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_query_best_size(
        &self,
        c: *mut xcb_connection_t,
        class: u8,
        drawable: xcb_drawable_t,
        width: u16,
        height: u16,
    ) -> xcb_query_best_size_cookie_t {
        sym!(self, xcb_query_best_size)(c, class, drawable, width, height)
    }

    /// Returns `true` iff the symbol `xcb_query_best_size` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_best_size(&self) -> bool {
        has_sym!(self, xcb_query_best_size)
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
    pub unsafe fn xcb_query_best_size_unchecked(
        &self,
        c: *mut xcb_connection_t,
        class: u8,
        drawable: xcb_drawable_t,
        width: u16,
        height: u16,
    ) -> xcb_query_best_size_cookie_t {
        sym!(self, xcb_query_best_size_unchecked)(c, class, drawable, width, height)
    }

    /// Returns `true` iff the symbol `xcb_query_best_size_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_best_size_unchecked(&self) -> bool {
        has_sym!(self, xcb_query_best_size_unchecked)
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
     * xcb_query_best_size_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_query_best_size_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_query_best_size_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_query_best_size_reply_t {
        sym!(self, xcb_query_best_size_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_query_best_size_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_best_size_reply(&self) -> bool {
        has_sym!(self, xcb_query_best_size_reply)
    }

    pub unsafe fn xcb_query_extension_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_query_extension_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_query_extension_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_extension_sizeof(&self) -> bool {
        has_sym!(self, xcb_query_extension_sizeof)
    }

    /**
     * @brief check if extension is present
     *
     * @param c The connection
     * @param name_len The length of \a name in bytes.
     * @param name The name of the extension to query, for example "RANDR". This is case
     * sensitive!
     * @return A cookie
     *
     * Determines if the specified extension is present on this X11 server.
     *
     * Every extension has a unique `major_opcode` to identify requests, the minor
     * opcodes and request formats are extension-specific. If the extension provides
     * events and errors, the `first_event` and `first_error` fields in the reply are
     * set accordingly.
     *
     * There should rarely be a need to use this request directly, XCB provides the
     * `xcb_get_extension_data` function instead.
     *
     */
    pub unsafe fn xcb_query_extension(
        &self,
        c: *mut xcb_connection_t,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_query_extension_cookie_t {
        sym!(self, xcb_query_extension)(c, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_query_extension` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_extension(&self) -> bool {
        has_sym!(self, xcb_query_extension)
    }

    /**
     * @brief check if extension is present
     *
     * @param c The connection
     * @param name_len The length of \a name in bytes.
     * @param name The name of the extension to query, for example "RANDR". This is case
     * sensitive!
     * @return A cookie
     *
     * Determines if the specified extension is present on this X11 server.
     *
     * Every extension has a unique `major_opcode` to identify requests, the minor
     * opcodes and request formats are extension-specific. If the extension provides
     * events and errors, the `first_event` and `first_error` fields in the reply are
     * set accordingly.
     *
     * There should rarely be a need to use this request directly, XCB provides the
     * `xcb_get_extension_data` function instead.
     *
     * This form can be used only if the request will cause
     * a reply to be generated. Any returned error will be
     * placed in the event queue.
     */
    pub unsafe fn xcb_query_extension_unchecked(
        &self,
        c: *mut xcb_connection_t,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_query_extension_cookie_t {
        sym!(self, xcb_query_extension_unchecked)(c, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_query_extension_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_extension_unchecked(&self) -> bool {
        has_sym!(self, xcb_query_extension_unchecked)
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
     * xcb_query_extension_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_query_extension_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_query_extension_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_query_extension_reply_t {
        sym!(self, xcb_query_extension_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_query_extension_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_extension_reply(&self) -> bool {
        has_sym!(self, xcb_query_extension_reply)
    }

    pub unsafe fn xcb_list_extensions_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_list_extensions_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_list_extensions_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_extensions_sizeof(&self) -> bool {
        has_sym!(self, xcb_list_extensions_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_list_extensions(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_list_extensions_cookie_t {
        sym!(self, xcb_list_extensions)(c)
    }

    /// Returns `true` iff the symbol `xcb_list_extensions` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_extensions(&self) -> bool {
        has_sym!(self, xcb_list_extensions)
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
    pub unsafe fn xcb_list_extensions_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_list_extensions_cookie_t {
        sym!(self, xcb_list_extensions_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_list_extensions_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_extensions_unchecked(&self) -> bool {
        has_sym!(self, xcb_list_extensions_unchecked)
    }

    pub unsafe fn xcb_list_extensions_names_length(
        &self,
        r: *const xcb_list_extensions_reply_t,
    ) -> c_int {
        sym!(self, xcb_list_extensions_names_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_extensions_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_extensions_names_length(&self) -> bool {
        has_sym!(self, xcb_list_extensions_names_length)
    }

    pub unsafe fn xcb_list_extensions_names_iterator(
        &self,
        r: *const xcb_list_extensions_reply_t,
    ) -> xcb_str_iterator_t {
        sym!(self, xcb_list_extensions_names_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_extensions_names_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_extensions_names_iterator(&self) -> bool {
        has_sym!(self, xcb_list_extensions_names_iterator)
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
     * xcb_list_extensions_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_list_extensions_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_list_extensions_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_list_extensions_reply_t {
        sym!(self, xcb_list_extensions_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_list_extensions_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_extensions_reply(&self) -> bool {
        has_sym!(self, xcb_list_extensions_reply)
    }

    pub unsafe fn xcb_change_keyboard_mapping_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_change_keyboard_mapping_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_mapping_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_mapping_sizeof(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_mapping_sizeof)
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
    pub unsafe fn xcb_change_keyboard_mapping_checked(
        &self,
        c: *mut xcb_connection_t,
        keycode_count: u8,
        first_keycode: xcb_keycode_t,
        keysyms_per_keycode: u8,
        keysyms: *const xcb_keysym_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_keyboard_mapping_checked)(
            c,
            keycode_count,
            first_keycode,
            keysyms_per_keycode,
            keysyms,
        )
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_mapping_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_mapping_checked(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_mapping_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_change_keyboard_mapping(
        &self,
        c: *mut xcb_connection_t,
        keycode_count: u8,
        first_keycode: xcb_keycode_t,
        keysyms_per_keycode: u8,
        keysyms: *const xcb_keysym_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_keyboard_mapping)(
            c,
            keycode_count,
            first_keycode,
            keysyms_per_keycode,
            keysyms,
        )
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_mapping` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_mapping(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_mapping)
    }

    pub unsafe fn xcb_change_keyboard_mapping_keysyms(
        &self,
        r: *const xcb_change_keyboard_mapping_request_t,
    ) -> *mut xcb_keysym_t {
        sym!(self, xcb_change_keyboard_mapping_keysyms)(r)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_mapping_keysyms` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_mapping_keysyms(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_mapping_keysyms)
    }

    pub unsafe fn xcb_change_keyboard_mapping_keysyms_length(
        &self,
        r: *const xcb_change_keyboard_mapping_request_t,
    ) -> c_int {
        sym!(self, xcb_change_keyboard_mapping_keysyms_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_mapping_keysyms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_mapping_keysyms_length(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_mapping_keysyms_length)
    }

    pub unsafe fn xcb_change_keyboard_mapping_keysyms_end(
        &self,
        r: *const xcb_change_keyboard_mapping_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_change_keyboard_mapping_keysyms_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_mapping_keysyms_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_mapping_keysyms_end(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_mapping_keysyms_end)
    }

    pub unsafe fn xcb_get_keyboard_mapping_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_get_keyboard_mapping_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_get_keyboard_mapping_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_keyboard_mapping_sizeof(&self) -> bool {
        has_sym!(self, xcb_get_keyboard_mapping_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_get_keyboard_mapping(
        &self,
        c: *mut xcb_connection_t,
        first_keycode: xcb_keycode_t,
        count: u8,
    ) -> xcb_get_keyboard_mapping_cookie_t {
        sym!(self, xcb_get_keyboard_mapping)(c, first_keycode, count)
    }

    /// Returns `true` iff the symbol `xcb_get_keyboard_mapping` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_keyboard_mapping(&self) -> bool {
        has_sym!(self, xcb_get_keyboard_mapping)
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
    pub unsafe fn xcb_get_keyboard_mapping_unchecked(
        &self,
        c: *mut xcb_connection_t,
        first_keycode: xcb_keycode_t,
        count: u8,
    ) -> xcb_get_keyboard_mapping_cookie_t {
        sym!(self, xcb_get_keyboard_mapping_unchecked)(c, first_keycode, count)
    }

    /// Returns `true` iff the symbol `xcb_get_keyboard_mapping_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_keyboard_mapping_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_keyboard_mapping_unchecked)
    }

    pub unsafe fn xcb_get_keyboard_mapping_keysyms(
        &self,
        r: *const xcb_get_keyboard_mapping_reply_t,
    ) -> *mut xcb_keysym_t {
        sym!(self, xcb_get_keyboard_mapping_keysyms)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_keyboard_mapping_keysyms` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_keyboard_mapping_keysyms(&self) -> bool {
        has_sym!(self, xcb_get_keyboard_mapping_keysyms)
    }

    pub unsafe fn xcb_get_keyboard_mapping_keysyms_length(
        &self,
        r: *const xcb_get_keyboard_mapping_reply_t,
    ) -> c_int {
        sym!(self, xcb_get_keyboard_mapping_keysyms_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_keyboard_mapping_keysyms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_keyboard_mapping_keysyms_length(&self) -> bool {
        has_sym!(self, xcb_get_keyboard_mapping_keysyms_length)
    }

    pub unsafe fn xcb_get_keyboard_mapping_keysyms_end(
        &self,
        r: *const xcb_get_keyboard_mapping_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_get_keyboard_mapping_keysyms_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_keyboard_mapping_keysyms_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_keyboard_mapping_keysyms_end(&self) -> bool {
        has_sym!(self, xcb_get_keyboard_mapping_keysyms_end)
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
     * xcb_get_keyboard_mapping_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_get_keyboard_mapping_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_keyboard_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_get_keyboard_mapping_reply_t {
        sym!(self, xcb_get_keyboard_mapping_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_keyboard_mapping_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_keyboard_mapping_reply(&self) -> bool {
        has_sym!(self, xcb_get_keyboard_mapping_reply)
    }

    pub unsafe fn xcb_change_keyboard_control_value_list_serialize(
        &self,
        _buffer: *mut *mut c_void,
        value_mask: u32,
        _aux: *const xcb_change_keyboard_control_value_list_t,
    ) -> c_int {
        sym!(self, xcb_change_keyboard_control_value_list_serialize)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_control_value_list_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_control_value_list_serialize(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_control_value_list_serialize)
    }

    pub unsafe fn xcb_change_keyboard_control_value_list_unpack(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
        _aux: *mut xcb_change_keyboard_control_value_list_t,
    ) -> c_int {
        sym!(self, xcb_change_keyboard_control_value_list_unpack)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_control_value_list_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_control_value_list_unpack(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_control_value_list_unpack)
    }

    pub unsafe fn xcb_change_keyboard_control_value_list_sizeof(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
    ) -> c_int {
        sym!(self, xcb_change_keyboard_control_value_list_sizeof)(_buffer, value_mask)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_control_value_list_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_control_value_list_sizeof(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_control_value_list_sizeof)
    }

    pub unsafe fn xcb_change_keyboard_control_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_change_keyboard_control_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_control_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_control_sizeof(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_control_sizeof)
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
    pub unsafe fn xcb_change_keyboard_control_checked(
        &self,
        c: *mut xcb_connection_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_keyboard_control_checked)(c, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_control_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_control_checked(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_control_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_change_keyboard_control(
        &self,
        c: *mut xcb_connection_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_keyboard_control)(c, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_control` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_control(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_control)
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
    pub unsafe fn xcb_change_keyboard_control_aux_checked(
        &self,
        c: *mut xcb_connection_t,
        value_mask: u32,
        value_list: *const xcb_change_keyboard_control_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_keyboard_control_aux_checked)(c, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_control_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_control_aux_checked(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_control_aux_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_change_keyboard_control_aux(
        &self,
        c: *mut xcb_connection_t,
        value_mask: u32,
        value_list: *const xcb_change_keyboard_control_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_keyboard_control_aux)(c, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_control_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_control_aux(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_control_aux)
    }

    pub unsafe fn xcb_change_keyboard_control_value_list(
        &self,
        r: *const xcb_change_keyboard_control_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_change_keyboard_control_value_list)(r)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_control_value_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_control_value_list(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_control_value_list)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_get_keyboard_control(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_keyboard_control_cookie_t {
        sym!(self, xcb_get_keyboard_control)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_keyboard_control` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_keyboard_control(&self) -> bool {
        has_sym!(self, xcb_get_keyboard_control)
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
    pub unsafe fn xcb_get_keyboard_control_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_keyboard_control_cookie_t {
        sym!(self, xcb_get_keyboard_control_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_keyboard_control_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_keyboard_control_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_keyboard_control_unchecked)
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
     * xcb_get_keyboard_control_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_get_keyboard_control_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_keyboard_control_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_get_keyboard_control_reply_t {
        sym!(self, xcb_get_keyboard_control_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_keyboard_control_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_keyboard_control_reply(&self) -> bool {
        has_sym!(self, xcb_get_keyboard_control_reply)
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
    pub unsafe fn xcb_bell_checked(
        &self,
        c: *mut xcb_connection_t,
        percent: i8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_bell_checked)(c, percent)
    }

    /// Returns `true` iff the symbol `xcb_bell_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_bell_checked(&self) -> bool {
        has_sym!(self, xcb_bell_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_bell(&self, c: *mut xcb_connection_t, percent: i8) -> xcb_void_cookie_t {
        sym!(self, xcb_bell)(c, percent)
    }

    /// Returns `true` iff the symbol `xcb_bell` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_bell(&self) -> bool {
        has_sym!(self, xcb_bell)
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
    pub unsafe fn xcb_change_pointer_control_checked(
        &self,
        c: *mut xcb_connection_t,
        acceleration_numerator: i16,
        acceleration_denominator: i16,
        threshold: i16,
        do_acceleration: u8,
        do_threshold: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_pointer_control_checked)(
            c,
            acceleration_numerator,
            acceleration_denominator,
            threshold,
            do_acceleration,
            do_threshold,
        )
    }

    /// Returns `true` iff the symbol `xcb_change_pointer_control_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_pointer_control_checked(&self) -> bool {
        has_sym!(self, xcb_change_pointer_control_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_change_pointer_control(
        &self,
        c: *mut xcb_connection_t,
        acceleration_numerator: i16,
        acceleration_denominator: i16,
        threshold: i16,
        do_acceleration: u8,
        do_threshold: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_pointer_control)(
            c,
            acceleration_numerator,
            acceleration_denominator,
            threshold,
            do_acceleration,
            do_threshold,
        )
    }

    /// Returns `true` iff the symbol `xcb_change_pointer_control` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_pointer_control(&self) -> bool {
        has_sym!(self, xcb_change_pointer_control)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_get_pointer_control(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_pointer_control_cookie_t {
        sym!(self, xcb_get_pointer_control)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_pointer_control` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_pointer_control(&self) -> bool {
        has_sym!(self, xcb_get_pointer_control)
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
    pub unsafe fn xcb_get_pointer_control_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_pointer_control_cookie_t {
        sym!(self, xcb_get_pointer_control_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_pointer_control_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_pointer_control_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_pointer_control_unchecked)
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
     * xcb_get_pointer_control_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_get_pointer_control_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_pointer_control_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_get_pointer_control_reply_t {
        sym!(self, xcb_get_pointer_control_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_pointer_control_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_pointer_control_reply(&self) -> bool {
        has_sym!(self, xcb_get_pointer_control_reply)
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
    pub unsafe fn xcb_set_screen_saver_checked(
        &self,
        c: *mut xcb_connection_t,
        timeout: i16,
        interval: i16,
        prefer_blanking: u8,
        allow_exposures: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_screen_saver_checked)(
            c,
            timeout,
            interval,
            prefer_blanking,
            allow_exposures,
        )
    }

    /// Returns `true` iff the symbol `xcb_set_screen_saver_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_screen_saver_checked(&self) -> bool {
        has_sym!(self, xcb_set_screen_saver_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_set_screen_saver(
        &self,
        c: *mut xcb_connection_t,
        timeout: i16,
        interval: i16,
        prefer_blanking: u8,
        allow_exposures: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_screen_saver)(c, timeout, interval, prefer_blanking, allow_exposures)
    }

    /// Returns `true` iff the symbol `xcb_set_screen_saver` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_screen_saver(&self) -> bool {
        has_sym!(self, xcb_set_screen_saver)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_get_screen_saver(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_screen_saver_cookie_t {
        sym!(self, xcb_get_screen_saver)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_screen_saver` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_screen_saver(&self) -> bool {
        has_sym!(self, xcb_get_screen_saver)
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
    pub unsafe fn xcb_get_screen_saver_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_screen_saver_cookie_t {
        sym!(self, xcb_get_screen_saver_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_screen_saver_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_screen_saver_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_screen_saver_unchecked)
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
     * xcb_get_screen_saver_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_get_screen_saver_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_screen_saver_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_get_screen_saver_reply_t {
        sym!(self, xcb_get_screen_saver_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_screen_saver_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_screen_saver_reply(&self) -> bool {
        has_sym!(self, xcb_get_screen_saver_reply)
    }

    pub unsafe fn xcb_change_hosts_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_change_hosts_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_change_hosts_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_hosts_sizeof(&self) -> bool {
        has_sym!(self, xcb_change_hosts_sizeof)
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
    pub unsafe fn xcb_change_hosts_checked(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
        family: u8,
        address_len: u16,
        address: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_hosts_checked)(c, mode, family, address_len, address)
    }

    /// Returns `true` iff the symbol `xcb_change_hosts_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_hosts_checked(&self) -> bool {
        has_sym!(self, xcb_change_hosts_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_change_hosts(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
        family: u8,
        address_len: u16,
        address: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_hosts)(c, mode, family, address_len, address)
    }

    /// Returns `true` iff the symbol `xcb_change_hosts` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_hosts(&self) -> bool {
        has_sym!(self, xcb_change_hosts)
    }

    pub unsafe fn xcb_change_hosts_address(&self, r: *const xcb_change_hosts_request_t) -> *mut u8 {
        sym!(self, xcb_change_hosts_address)(r)
    }

    /// Returns `true` iff the symbol `xcb_change_hosts_address` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_hosts_address(&self) -> bool {
        has_sym!(self, xcb_change_hosts_address)
    }

    pub unsafe fn xcb_change_hosts_address_length(
        &self,
        r: *const xcb_change_hosts_request_t,
    ) -> c_int {
        sym!(self, xcb_change_hosts_address_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_change_hosts_address_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_hosts_address_length(&self) -> bool {
        has_sym!(self, xcb_change_hosts_address_length)
    }

    pub unsafe fn xcb_change_hosts_address_end(
        &self,
        r: *const xcb_change_hosts_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_change_hosts_address_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_change_hosts_address_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_hosts_address_end(&self) -> bool {
        has_sym!(self, xcb_change_hosts_address_end)
    }

    pub unsafe fn xcb_host_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_host_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_host_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_host_sizeof(&self) -> bool {
        has_sym!(self, xcb_host_sizeof)
    }

    pub unsafe fn xcb_host_address(&self, r: *const xcb_host_t) -> *mut u8 {
        sym!(self, xcb_host_address)(r)
    }

    /// Returns `true` iff the symbol `xcb_host_address` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_host_address(&self) -> bool {
        has_sym!(self, xcb_host_address)
    }

    pub unsafe fn xcb_host_address_length(&self, r: *const xcb_host_t) -> c_int {
        sym!(self, xcb_host_address_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_host_address_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_host_address_length(&self) -> bool {
        has_sym!(self, xcb_host_address_length)
    }

    pub unsafe fn xcb_host_address_end(&self, r: *const xcb_host_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_host_address_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_host_address_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_host_address_end(&self) -> bool {
        has_sym!(self, xcb_host_address_end)
    }

    pub unsafe fn xcb_host_next(&self, i: *mut xcb_host_iterator_t) {
        sym!(self, xcb_host_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_host_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_host_next(&self) -> bool {
        has_sym!(self, xcb_host_next)
    }

    pub unsafe fn xcb_host_end(&self, i: xcb_host_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_host_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_host_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_host_end(&self) -> bool {
        has_sym!(self, xcb_host_end)
    }

    pub unsafe fn xcb_list_hosts_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_list_hosts_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_list_hosts_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_hosts_sizeof(&self) -> bool {
        has_sym!(self, xcb_list_hosts_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_list_hosts(&self, c: *mut xcb_connection_t) -> xcb_list_hosts_cookie_t {
        sym!(self, xcb_list_hosts)(c)
    }

    /// Returns `true` iff the symbol `xcb_list_hosts` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_hosts(&self) -> bool {
        has_sym!(self, xcb_list_hosts)
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
    pub unsafe fn xcb_list_hosts_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_list_hosts_cookie_t {
        sym!(self, xcb_list_hosts_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_list_hosts_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_hosts_unchecked(&self) -> bool {
        has_sym!(self, xcb_list_hosts_unchecked)
    }

    pub unsafe fn xcb_list_hosts_hosts_length(&self, r: *const xcb_list_hosts_reply_t) -> c_int {
        sym!(self, xcb_list_hosts_hosts_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_hosts_hosts_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_hosts_hosts_length(&self) -> bool {
        has_sym!(self, xcb_list_hosts_hosts_length)
    }

    pub unsafe fn xcb_list_hosts_hosts_iterator(
        &self,
        r: *const xcb_list_hosts_reply_t,
    ) -> xcb_host_iterator_t {
        sym!(self, xcb_list_hosts_hosts_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_hosts_hosts_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_hosts_hosts_iterator(&self) -> bool {
        has_sym!(self, xcb_list_hosts_hosts_iterator)
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
     * xcb_list_hosts_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_list_hosts_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_list_hosts_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_list_hosts_reply_t {
        sym!(self, xcb_list_hosts_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_list_hosts_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_hosts_reply(&self) -> bool {
        has_sym!(self, xcb_list_hosts_reply)
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
    pub unsafe fn xcb_set_access_control_checked(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_access_control_checked)(c, mode)
    }

    /// Returns `true` iff the symbol `xcb_set_access_control_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_access_control_checked(&self) -> bool {
        has_sym!(self, xcb_set_access_control_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_set_access_control(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_access_control)(c, mode)
    }

    /// Returns `true` iff the symbol `xcb_set_access_control` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_access_control(&self) -> bool {
        has_sym!(self, xcb_set_access_control)
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
    pub unsafe fn xcb_set_close_down_mode_checked(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_close_down_mode_checked)(c, mode)
    }

    /// Returns `true` iff the symbol `xcb_set_close_down_mode_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_close_down_mode_checked(&self) -> bool {
        has_sym!(self, xcb_set_close_down_mode_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_set_close_down_mode(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_close_down_mode)(c, mode)
    }

    /// Returns `true` iff the symbol `xcb_set_close_down_mode` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_close_down_mode(&self) -> bool {
        has_sym!(self, xcb_set_close_down_mode)
    }

    /**
     * @brief kills a client
     *
     * @param c The connection
     * @param resource Any resource belonging to the client (for example a Window), used to identify
     * the client connection.
     * \n
     * The special value of `XCB_KILL_ALL_TEMPORARY`, the resources of all clients
     * that have terminated in `RetainTemporary` (TODO) are destroyed.
     * @return A cookie
     *
     * Forces a close down of the client that created the specified \a resource.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_kill_client_checked(
        &self,
        c: *mut xcb_connection_t,
        resource: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_kill_client_checked)(c, resource)
    }

    /// Returns `true` iff the symbol `xcb_kill_client_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_kill_client_checked(&self) -> bool {
        has_sym!(self, xcb_kill_client_checked)
    }

    /**
     * @brief kills a client
     *
     * @param c The connection
     * @param resource Any resource belonging to the client (for example a Window), used to identify
     * the client connection.
     * \n
     * The special value of `XCB_KILL_ALL_TEMPORARY`, the resources of all clients
     * that have terminated in `RetainTemporary` (TODO) are destroyed.
     * @return A cookie
     *
     * Forces a close down of the client that created the specified \a resource.
     *
     */
    pub unsafe fn xcb_kill_client(
        &self,
        c: *mut xcb_connection_t,
        resource: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_kill_client)(c, resource)
    }

    /// Returns `true` iff the symbol `xcb_kill_client` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_kill_client(&self) -> bool {
        has_sym!(self, xcb_kill_client)
    }

    pub unsafe fn xcb_rotate_properties_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_rotate_properties_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_rotate_properties_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_rotate_properties_sizeof(&self) -> bool {
        has_sym!(self, xcb_rotate_properties_sizeof)
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
    pub unsafe fn xcb_rotate_properties_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        atoms_len: u16,
        delta: i16,
        atoms: *const xcb_atom_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_rotate_properties_checked)(c, window, atoms_len, delta, atoms)
    }

    /// Returns `true` iff the symbol `xcb_rotate_properties_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_rotate_properties_checked(&self) -> bool {
        has_sym!(self, xcb_rotate_properties_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_rotate_properties(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        atoms_len: u16,
        delta: i16,
        atoms: *const xcb_atom_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_rotate_properties)(c, window, atoms_len, delta, atoms)
    }

    /// Returns `true` iff the symbol `xcb_rotate_properties` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_rotate_properties(&self) -> bool {
        has_sym!(self, xcb_rotate_properties)
    }

    pub unsafe fn xcb_rotate_properties_atoms(
        &self,
        r: *const xcb_rotate_properties_request_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_rotate_properties_atoms)(r)
    }

    /// Returns `true` iff the symbol `xcb_rotate_properties_atoms` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_rotate_properties_atoms(&self) -> bool {
        has_sym!(self, xcb_rotate_properties_atoms)
    }

    pub unsafe fn xcb_rotate_properties_atoms_length(
        &self,
        r: *const xcb_rotate_properties_request_t,
    ) -> c_int {
        sym!(self, xcb_rotate_properties_atoms_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_rotate_properties_atoms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_rotate_properties_atoms_length(&self) -> bool {
        has_sym!(self, xcb_rotate_properties_atoms_length)
    }

    pub unsafe fn xcb_rotate_properties_atoms_end(
        &self,
        r: *const xcb_rotate_properties_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_rotate_properties_atoms_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_rotate_properties_atoms_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_rotate_properties_atoms_end(&self) -> bool {
        has_sym!(self, xcb_rotate_properties_atoms_end)
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
    pub unsafe fn xcb_force_screen_saver_checked(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_force_screen_saver_checked)(c, mode)
    }

    /// Returns `true` iff the symbol `xcb_force_screen_saver_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_force_screen_saver_checked(&self) -> bool {
        has_sym!(self, xcb_force_screen_saver_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_force_screen_saver(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_force_screen_saver)(c, mode)
    }

    /// Returns `true` iff the symbol `xcb_force_screen_saver` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_force_screen_saver(&self) -> bool {
        has_sym!(self, xcb_force_screen_saver)
    }

    pub unsafe fn xcb_set_pointer_mapping_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_set_pointer_mapping_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_set_pointer_mapping_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_pointer_mapping_sizeof(&self) -> bool {
        has_sym!(self, xcb_set_pointer_mapping_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_set_pointer_mapping(
        &self,
        c: *mut xcb_connection_t,
        map_len: u8,
        map: *const u8,
    ) -> xcb_set_pointer_mapping_cookie_t {
        sym!(self, xcb_set_pointer_mapping)(c, map_len, map)
    }

    /// Returns `true` iff the symbol `xcb_set_pointer_mapping` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_pointer_mapping(&self) -> bool {
        has_sym!(self, xcb_set_pointer_mapping)
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
    pub unsafe fn xcb_set_pointer_mapping_unchecked(
        &self,
        c: *mut xcb_connection_t,
        map_len: u8,
        map: *const u8,
    ) -> xcb_set_pointer_mapping_cookie_t {
        sym!(self, xcb_set_pointer_mapping_unchecked)(c, map_len, map)
    }

    /// Returns `true` iff the symbol `xcb_set_pointer_mapping_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_pointer_mapping_unchecked(&self) -> bool {
        has_sym!(self, xcb_set_pointer_mapping_unchecked)
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
     * xcb_set_pointer_mapping_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_set_pointer_mapping_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_set_pointer_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_set_pointer_mapping_reply_t {
        sym!(self, xcb_set_pointer_mapping_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_set_pointer_mapping_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_pointer_mapping_reply(&self) -> bool {
        has_sym!(self, xcb_set_pointer_mapping_reply)
    }

    pub unsafe fn xcb_get_pointer_mapping_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_get_pointer_mapping_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_get_pointer_mapping_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_pointer_mapping_sizeof(&self) -> bool {
        has_sym!(self, xcb_get_pointer_mapping_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_get_pointer_mapping(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_pointer_mapping_cookie_t {
        sym!(self, xcb_get_pointer_mapping)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_pointer_mapping` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_pointer_mapping(&self) -> bool {
        has_sym!(self, xcb_get_pointer_mapping)
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
    pub unsafe fn xcb_get_pointer_mapping_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_pointer_mapping_cookie_t {
        sym!(self, xcb_get_pointer_mapping_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_pointer_mapping_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_pointer_mapping_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_pointer_mapping_unchecked)
    }

    pub unsafe fn xcb_get_pointer_mapping_map(
        &self,
        r: *const xcb_get_pointer_mapping_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_get_pointer_mapping_map)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_pointer_mapping_map` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_pointer_mapping_map(&self) -> bool {
        has_sym!(self, xcb_get_pointer_mapping_map)
    }

    pub unsafe fn xcb_get_pointer_mapping_map_length(
        &self,
        r: *const xcb_get_pointer_mapping_reply_t,
    ) -> c_int {
        sym!(self, xcb_get_pointer_mapping_map_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_pointer_mapping_map_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_pointer_mapping_map_length(&self) -> bool {
        has_sym!(self, xcb_get_pointer_mapping_map_length)
    }

    pub unsafe fn xcb_get_pointer_mapping_map_end(
        &self,
        r: *const xcb_get_pointer_mapping_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_get_pointer_mapping_map_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_pointer_mapping_map_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_pointer_mapping_map_end(&self) -> bool {
        has_sym!(self, xcb_get_pointer_mapping_map_end)
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
     * xcb_get_pointer_mapping_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_get_pointer_mapping_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_pointer_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_get_pointer_mapping_reply_t {
        sym!(self, xcb_get_pointer_mapping_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_pointer_mapping_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_pointer_mapping_reply(&self) -> bool {
        has_sym!(self, xcb_get_pointer_mapping_reply)
    }

    pub unsafe fn xcb_set_modifier_mapping_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_set_modifier_mapping_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_set_modifier_mapping_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_modifier_mapping_sizeof(&self) -> bool {
        has_sym!(self, xcb_set_modifier_mapping_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_set_modifier_mapping(
        &self,
        c: *mut xcb_connection_t,
        keycodes_per_modifier: u8,
        keycodes: *const xcb_keycode_t,
    ) -> xcb_set_modifier_mapping_cookie_t {
        sym!(self, xcb_set_modifier_mapping)(c, keycodes_per_modifier, keycodes)
    }

    /// Returns `true` iff the symbol `xcb_set_modifier_mapping` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_modifier_mapping(&self) -> bool {
        has_sym!(self, xcb_set_modifier_mapping)
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
    pub unsafe fn xcb_set_modifier_mapping_unchecked(
        &self,
        c: *mut xcb_connection_t,
        keycodes_per_modifier: u8,
        keycodes: *const xcb_keycode_t,
    ) -> xcb_set_modifier_mapping_cookie_t {
        sym!(self, xcb_set_modifier_mapping_unchecked)(c, keycodes_per_modifier, keycodes)
    }

    /// Returns `true` iff the symbol `xcb_set_modifier_mapping_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_modifier_mapping_unchecked(&self) -> bool {
        has_sym!(self, xcb_set_modifier_mapping_unchecked)
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
     * xcb_set_modifier_mapping_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_set_modifier_mapping_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_set_modifier_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_set_modifier_mapping_reply_t {
        sym!(self, xcb_set_modifier_mapping_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_set_modifier_mapping_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_modifier_mapping_reply(&self) -> bool {
        has_sym!(self, xcb_set_modifier_mapping_reply)
    }

    pub unsafe fn xcb_get_modifier_mapping_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_get_modifier_mapping_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_get_modifier_mapping_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_modifier_mapping_sizeof(&self) -> bool {
        has_sym!(self, xcb_get_modifier_mapping_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_get_modifier_mapping(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_modifier_mapping_cookie_t {
        sym!(self, xcb_get_modifier_mapping)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_modifier_mapping` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_modifier_mapping(&self) -> bool {
        has_sym!(self, xcb_get_modifier_mapping)
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
    pub unsafe fn xcb_get_modifier_mapping_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_modifier_mapping_cookie_t {
        sym!(self, xcb_get_modifier_mapping_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_modifier_mapping_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_modifier_mapping_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_modifier_mapping_unchecked)
    }

    pub unsafe fn xcb_get_modifier_mapping_keycodes(
        &self,
        r: *const xcb_get_modifier_mapping_reply_t,
    ) -> *mut xcb_keycode_t {
        sym!(self, xcb_get_modifier_mapping_keycodes)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_modifier_mapping_keycodes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_modifier_mapping_keycodes(&self) -> bool {
        has_sym!(self, xcb_get_modifier_mapping_keycodes)
    }

    pub unsafe fn xcb_get_modifier_mapping_keycodes_length(
        &self,
        r: *const xcb_get_modifier_mapping_reply_t,
    ) -> c_int {
        sym!(self, xcb_get_modifier_mapping_keycodes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_modifier_mapping_keycodes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_modifier_mapping_keycodes_length(&self) -> bool {
        has_sym!(self, xcb_get_modifier_mapping_keycodes_length)
    }

    pub unsafe fn xcb_get_modifier_mapping_keycodes_end(
        &self,
        r: *const xcb_get_modifier_mapping_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_get_modifier_mapping_keycodes_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_modifier_mapping_keycodes_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_modifier_mapping_keycodes_end(&self) -> bool {
        has_sym!(self, xcb_get_modifier_mapping_keycodes_end)
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
     * xcb_get_modifier_mapping_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_get_modifier_mapping_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_modifier_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_get_modifier_mapping_reply_t {
        sym!(self, xcb_get_modifier_mapping_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_modifier_mapping_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_modifier_mapping_reply(&self) -> bool {
        has_sym!(self, xcb_get_modifier_mapping_reply)
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
    pub unsafe fn xcb_no_operation_checked(&self, c: *mut xcb_connection_t) -> xcb_void_cookie_t {
        sym!(self, xcb_no_operation_checked)(c)
    }

    /// Returns `true` iff the symbol `xcb_no_operation_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_no_operation_checked(&self) -> bool {
        has_sym!(self, xcb_no_operation_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_no_operation(&self, c: *mut xcb_connection_t) -> xcb_void_cookie_t {
        sym!(self, xcb_no_operation)(c)
    }

    /// Returns `true` iff the symbol `xcb_no_operation` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_no_operation(&self) -> bool {
        has_sym!(self, xcb_no_operation)
    }
}

#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::Xcb::load().unwrap() };
        assert!(lib.has_xcb_char2b_next());
        assert!(lib.has_xcb_char2b_end());
        assert!(lib.has_xcb_window_next());
        assert!(lib.has_xcb_window_end());
        assert!(lib.has_xcb_pixmap_next());
        assert!(lib.has_xcb_pixmap_end());
        assert!(lib.has_xcb_cursor_next());
        assert!(lib.has_xcb_cursor_end());
        assert!(lib.has_xcb_font_next());
        assert!(lib.has_xcb_font_end());
        assert!(lib.has_xcb_gcontext_next());
        assert!(lib.has_xcb_gcontext_end());
        assert!(lib.has_xcb_colormap_next());
        assert!(lib.has_xcb_colormap_end());
        assert!(lib.has_xcb_atom_next());
        assert!(lib.has_xcb_atom_end());
        assert!(lib.has_xcb_drawable_next());
        assert!(lib.has_xcb_drawable_end());
        assert!(lib.has_xcb_fontable_next());
        assert!(lib.has_xcb_fontable_end());
        assert!(lib.has_xcb_bool32_next());
        assert!(lib.has_xcb_bool32_end());
        assert!(lib.has_xcb_visualid_next());
        assert!(lib.has_xcb_visualid_end());
        assert!(lib.has_xcb_timestamp_next());
        assert!(lib.has_xcb_timestamp_end());
        assert!(lib.has_xcb_keysym_next());
        assert!(lib.has_xcb_keysym_end());
        assert!(lib.has_xcb_keycode_next());
        assert!(lib.has_xcb_keycode_end());
        assert!(lib.has_xcb_keycode32_next());
        assert!(lib.has_xcb_keycode32_end());
        assert!(lib.has_xcb_button_next());
        assert!(lib.has_xcb_button_end());
        assert!(lib.has_xcb_point_next());
        assert!(lib.has_xcb_point_end());
        assert!(lib.has_xcb_rectangle_next());
        assert!(lib.has_xcb_rectangle_end());
        assert!(lib.has_xcb_arc_next());
        assert!(lib.has_xcb_arc_end());
        assert!(lib.has_xcb_format_next());
        assert!(lib.has_xcb_format_end());
        assert!(lib.has_xcb_visualtype_next());
        assert!(lib.has_xcb_visualtype_end());
        assert!(lib.has_xcb_depth_sizeof());
        assert!(lib.has_xcb_depth_visuals());
        assert!(lib.has_xcb_depth_visuals_length());
        assert!(lib.has_xcb_depth_visuals_iterator());
        assert!(lib.has_xcb_depth_next());
        assert!(lib.has_xcb_depth_end());
        assert!(lib.has_xcb_screen_sizeof());
        assert!(lib.has_xcb_screen_allowed_depths_length());
        assert!(lib.has_xcb_screen_allowed_depths_iterator());
        assert!(lib.has_xcb_screen_next());
        assert!(lib.has_xcb_screen_end());
        assert!(lib.has_xcb_setup_request_sizeof());
        assert!(lib.has_xcb_setup_request_authorization_protocol_name());
        assert!(lib.has_xcb_setup_request_authorization_protocol_name_length());
        assert!(lib.has_xcb_setup_request_authorization_protocol_name_end());
        assert!(lib.has_xcb_setup_request_authorization_protocol_data());
        assert!(lib.has_xcb_setup_request_authorization_protocol_data_length());
        assert!(lib.has_xcb_setup_request_authorization_protocol_data_end());
        assert!(lib.has_xcb_setup_request_next());
        assert!(lib.has_xcb_setup_request_end());
        assert!(lib.has_xcb_setup_failed_sizeof());
        assert!(lib.has_xcb_setup_failed_reason());
        assert!(lib.has_xcb_setup_failed_reason_length());
        assert!(lib.has_xcb_setup_failed_reason_end());
        assert!(lib.has_xcb_setup_failed_next());
        assert!(lib.has_xcb_setup_failed_end());
        assert!(lib.has_xcb_setup_authenticate_sizeof());
        assert!(lib.has_xcb_setup_authenticate_reason());
        assert!(lib.has_xcb_setup_authenticate_reason_length());
        assert!(lib.has_xcb_setup_authenticate_reason_end());
        assert!(lib.has_xcb_setup_authenticate_next());
        assert!(lib.has_xcb_setup_authenticate_end());
        assert!(lib.has_xcb_setup_sizeof());
        assert!(lib.has_xcb_setup_vendor());
        assert!(lib.has_xcb_setup_vendor_length());
        assert!(lib.has_xcb_setup_vendor_end());
        assert!(lib.has_xcb_setup_pixmap_formats());
        assert!(lib.has_xcb_setup_pixmap_formats_length());
        assert!(lib.has_xcb_setup_pixmap_formats_iterator());
        assert!(lib.has_xcb_setup_roots_length());
        assert!(lib.has_xcb_setup_roots_iterator());
        assert!(lib.has_xcb_setup_next());
        assert!(lib.has_xcb_setup_end());
        assert!(lib.has_xcb_client_message_data_next());
        assert!(lib.has_xcb_client_message_data_end());
        assert!(lib.has_xcb_create_window_value_list_serialize());
        assert!(lib.has_xcb_create_window_value_list_unpack());
        assert!(lib.has_xcb_create_window_value_list_sizeof());
        assert!(lib.has_xcb_create_window_sizeof());
        assert!(lib.has_xcb_create_window_checked());
        assert!(lib.has_xcb_create_window());
        assert!(lib.has_xcb_create_window_aux_checked());
        assert!(lib.has_xcb_create_window_aux());
        assert!(lib.has_xcb_create_window_value_list());
        assert!(lib.has_xcb_change_window_attributes_value_list_serialize());
        assert!(lib.has_xcb_change_window_attributes_value_list_unpack());
        assert!(lib.has_xcb_change_window_attributes_value_list_sizeof());
        assert!(lib.has_xcb_change_window_attributes_sizeof());
        assert!(lib.has_xcb_change_window_attributes_checked());
        assert!(lib.has_xcb_change_window_attributes());
        assert!(lib.has_xcb_change_window_attributes_aux_checked());
        assert!(lib.has_xcb_change_window_attributes_aux());
        assert!(lib.has_xcb_change_window_attributes_value_list());
        assert!(lib.has_xcb_get_window_attributes());
        assert!(lib.has_xcb_get_window_attributes_unchecked());
        assert!(lib.has_xcb_get_window_attributes_reply());
        assert!(lib.has_xcb_destroy_window_checked());
        assert!(lib.has_xcb_destroy_window());
        assert!(lib.has_xcb_destroy_subwindows_checked());
        assert!(lib.has_xcb_destroy_subwindows());
        assert!(lib.has_xcb_change_save_set_checked());
        assert!(lib.has_xcb_change_save_set());
        assert!(lib.has_xcb_reparent_window_checked());
        assert!(lib.has_xcb_reparent_window());
        assert!(lib.has_xcb_map_window_checked());
        assert!(lib.has_xcb_map_window());
        assert!(lib.has_xcb_map_subwindows_checked());
        assert!(lib.has_xcb_map_subwindows());
        assert!(lib.has_xcb_unmap_window_checked());
        assert!(lib.has_xcb_unmap_window());
        assert!(lib.has_xcb_unmap_subwindows_checked());
        assert!(lib.has_xcb_unmap_subwindows());
        assert!(lib.has_xcb_configure_window_value_list_serialize());
        assert!(lib.has_xcb_configure_window_value_list_unpack());
        assert!(lib.has_xcb_configure_window_value_list_sizeof());
        assert!(lib.has_xcb_configure_window_sizeof());
        assert!(lib.has_xcb_configure_window_checked());
        assert!(lib.has_xcb_configure_window());
        assert!(lib.has_xcb_configure_window_aux_checked());
        assert!(lib.has_xcb_configure_window_aux());
        assert!(lib.has_xcb_configure_window_value_list());
        assert!(lib.has_xcb_circulate_window_checked());
        assert!(lib.has_xcb_circulate_window());
        assert!(lib.has_xcb_get_geometry());
        assert!(lib.has_xcb_get_geometry_unchecked());
        assert!(lib.has_xcb_get_geometry_reply());
        assert!(lib.has_xcb_query_tree_sizeof());
        assert!(lib.has_xcb_query_tree());
        assert!(lib.has_xcb_query_tree_unchecked());
        assert!(lib.has_xcb_query_tree_children());
        assert!(lib.has_xcb_query_tree_children_length());
        assert!(lib.has_xcb_query_tree_children_end());
        assert!(lib.has_xcb_query_tree_reply());
        assert!(lib.has_xcb_intern_atom_sizeof());
        assert!(lib.has_xcb_intern_atom());
        assert!(lib.has_xcb_intern_atom_unchecked());
        assert!(lib.has_xcb_intern_atom_reply());
        assert!(lib.has_xcb_get_atom_name_sizeof());
        assert!(lib.has_xcb_get_atom_name());
        assert!(lib.has_xcb_get_atom_name_unchecked());
        assert!(lib.has_xcb_get_atom_name_name());
        assert!(lib.has_xcb_get_atom_name_name_length());
        assert!(lib.has_xcb_get_atom_name_name_end());
        assert!(lib.has_xcb_get_atom_name_reply());
        assert!(lib.has_xcb_change_property_sizeof());
        assert!(lib.has_xcb_change_property_checked());
        assert!(lib.has_xcb_change_property());
        assert!(lib.has_xcb_change_property_data());
        assert!(lib.has_xcb_change_property_data_length());
        assert!(lib.has_xcb_change_property_data_end());
        assert!(lib.has_xcb_delete_property_checked());
        assert!(lib.has_xcb_delete_property());
        assert!(lib.has_xcb_get_property_sizeof());
        assert!(lib.has_xcb_get_property());
        assert!(lib.has_xcb_get_property_unchecked());
        assert!(lib.has_xcb_get_property_value());
        assert!(lib.has_xcb_get_property_value_length());
        assert!(lib.has_xcb_get_property_value_end());
        assert!(lib.has_xcb_get_property_reply());
        assert!(lib.has_xcb_list_properties_sizeof());
        assert!(lib.has_xcb_list_properties());
        assert!(lib.has_xcb_list_properties_unchecked());
        assert!(lib.has_xcb_list_properties_atoms());
        assert!(lib.has_xcb_list_properties_atoms_length());
        assert!(lib.has_xcb_list_properties_atoms_end());
        assert!(lib.has_xcb_list_properties_reply());
        assert!(lib.has_xcb_set_selection_owner_checked());
        assert!(lib.has_xcb_set_selection_owner());
        assert!(lib.has_xcb_get_selection_owner());
        assert!(lib.has_xcb_get_selection_owner_unchecked());
        assert!(lib.has_xcb_get_selection_owner_reply());
        assert!(lib.has_xcb_convert_selection_checked());
        assert!(lib.has_xcb_convert_selection());
        assert!(lib.has_xcb_send_event_checked());
        assert!(lib.has_xcb_send_event());
        assert!(lib.has_xcb_grab_pointer());
        assert!(lib.has_xcb_grab_pointer_unchecked());
        assert!(lib.has_xcb_grab_pointer_reply());
        assert!(lib.has_xcb_ungrab_pointer_checked());
        assert!(lib.has_xcb_ungrab_pointer());
        assert!(lib.has_xcb_grab_button_checked());
        assert!(lib.has_xcb_grab_button());
        assert!(lib.has_xcb_ungrab_button_checked());
        assert!(lib.has_xcb_ungrab_button());
        assert!(lib.has_xcb_change_active_pointer_grab_checked());
        assert!(lib.has_xcb_change_active_pointer_grab());
        assert!(lib.has_xcb_grab_keyboard());
        assert!(lib.has_xcb_grab_keyboard_unchecked());
        assert!(lib.has_xcb_grab_keyboard_reply());
        assert!(lib.has_xcb_ungrab_keyboard_checked());
        assert!(lib.has_xcb_ungrab_keyboard());
        assert!(lib.has_xcb_grab_key_checked());
        assert!(lib.has_xcb_grab_key());
        assert!(lib.has_xcb_ungrab_key_checked());
        assert!(lib.has_xcb_ungrab_key());
        assert!(lib.has_xcb_allow_events_checked());
        assert!(lib.has_xcb_allow_events());
        assert!(lib.has_xcb_grab_server_checked());
        assert!(lib.has_xcb_grab_server());
        assert!(lib.has_xcb_ungrab_server_checked());
        assert!(lib.has_xcb_ungrab_server());
        assert!(lib.has_xcb_query_pointer());
        assert!(lib.has_xcb_query_pointer_unchecked());
        assert!(lib.has_xcb_query_pointer_reply());
        assert!(lib.has_xcb_timecoord_next());
        assert!(lib.has_xcb_timecoord_end());
        assert!(lib.has_xcb_get_motion_events_sizeof());
        assert!(lib.has_xcb_get_motion_events());
        assert!(lib.has_xcb_get_motion_events_unchecked());
        assert!(lib.has_xcb_get_motion_events_events());
        assert!(lib.has_xcb_get_motion_events_events_length());
        assert!(lib.has_xcb_get_motion_events_events_iterator());
        assert!(lib.has_xcb_get_motion_events_reply());
        assert!(lib.has_xcb_translate_coordinates());
        assert!(lib.has_xcb_translate_coordinates_unchecked());
        assert!(lib.has_xcb_translate_coordinates_reply());
        assert!(lib.has_xcb_warp_pointer_checked());
        assert!(lib.has_xcb_warp_pointer());
        assert!(lib.has_xcb_set_input_focus_checked());
        assert!(lib.has_xcb_set_input_focus());
        assert!(lib.has_xcb_get_input_focus());
        assert!(lib.has_xcb_get_input_focus_unchecked());
        assert!(lib.has_xcb_get_input_focus_reply());
        assert!(lib.has_xcb_query_keymap());
        assert!(lib.has_xcb_query_keymap_unchecked());
        assert!(lib.has_xcb_query_keymap_reply());
        assert!(lib.has_xcb_open_font_sizeof());
        assert!(lib.has_xcb_open_font_checked());
        assert!(lib.has_xcb_open_font());
        assert!(lib.has_xcb_open_font_name());
        assert!(lib.has_xcb_open_font_name_length());
        assert!(lib.has_xcb_open_font_name_end());
        assert!(lib.has_xcb_close_font_checked());
        assert!(lib.has_xcb_close_font());
        assert!(lib.has_xcb_fontprop_next());
        assert!(lib.has_xcb_fontprop_end());
        assert!(lib.has_xcb_charinfo_next());
        assert!(lib.has_xcb_charinfo_end());
        assert!(lib.has_xcb_query_font_sizeof());
        assert!(lib.has_xcb_query_font());
        assert!(lib.has_xcb_query_font_unchecked());
        assert!(lib.has_xcb_query_font_properties());
        assert!(lib.has_xcb_query_font_properties_length());
        assert!(lib.has_xcb_query_font_properties_iterator());
        assert!(lib.has_xcb_query_font_char_infos());
        assert!(lib.has_xcb_query_font_char_infos_length());
        assert!(lib.has_xcb_query_font_char_infos_iterator());
        assert!(lib.has_xcb_query_font_reply());
        assert!(lib.has_xcb_query_text_extents_sizeof());
        assert!(lib.has_xcb_query_text_extents());
        assert!(lib.has_xcb_query_text_extents_unchecked());
        assert!(lib.has_xcb_query_text_extents_reply());
        assert!(lib.has_xcb_str_sizeof());
        assert!(lib.has_xcb_str_name());
        assert!(lib.has_xcb_str_name_length());
        assert!(lib.has_xcb_str_name_end());
        assert!(lib.has_xcb_str_next());
        assert!(lib.has_xcb_str_end());
        assert!(lib.has_xcb_list_fonts_sizeof());
        assert!(lib.has_xcb_list_fonts());
        assert!(lib.has_xcb_list_fonts_unchecked());
        assert!(lib.has_xcb_list_fonts_names_length());
        assert!(lib.has_xcb_list_fonts_names_iterator());
        assert!(lib.has_xcb_list_fonts_reply());
        assert!(lib.has_xcb_list_fonts_with_info_sizeof());
        assert!(lib.has_xcb_list_fonts_with_info());
        assert!(lib.has_xcb_list_fonts_with_info_unchecked());
        assert!(lib.has_xcb_list_fonts_with_info_properties());
        assert!(lib.has_xcb_list_fonts_with_info_properties_length());
        assert!(lib.has_xcb_list_fonts_with_info_properties_iterator());
        assert!(lib.has_xcb_list_fonts_with_info_name());
        assert!(lib.has_xcb_list_fonts_with_info_name_length());
        assert!(lib.has_xcb_list_fonts_with_info_name_end());
        assert!(lib.has_xcb_list_fonts_with_info_reply());
        assert!(lib.has_xcb_set_font_path_sizeof());
        assert!(lib.has_xcb_set_font_path_checked());
        assert!(lib.has_xcb_set_font_path());
        assert!(lib.has_xcb_set_font_path_font_length());
        assert!(lib.has_xcb_set_font_path_font_iterator());
        assert!(lib.has_xcb_get_font_path_sizeof());
        assert!(lib.has_xcb_get_font_path());
        assert!(lib.has_xcb_get_font_path_unchecked());
        assert!(lib.has_xcb_get_font_path_path_length());
        assert!(lib.has_xcb_get_font_path_path_iterator());
        assert!(lib.has_xcb_get_font_path_reply());
        assert!(lib.has_xcb_create_pixmap_checked());
        assert!(lib.has_xcb_create_pixmap());
        assert!(lib.has_xcb_free_pixmap_checked());
        assert!(lib.has_xcb_free_pixmap());
        assert!(lib.has_xcb_create_gc_value_list_serialize());
        assert!(lib.has_xcb_create_gc_value_list_unpack());
        assert!(lib.has_xcb_create_gc_value_list_sizeof());
        assert!(lib.has_xcb_create_gc_sizeof());
        assert!(lib.has_xcb_create_gc_checked());
        assert!(lib.has_xcb_create_gc());
        assert!(lib.has_xcb_create_gc_aux_checked());
        assert!(lib.has_xcb_create_gc_aux());
        assert!(lib.has_xcb_create_gc_value_list());
        assert!(lib.has_xcb_change_gc_value_list_serialize());
        assert!(lib.has_xcb_change_gc_value_list_unpack());
        assert!(lib.has_xcb_change_gc_value_list_sizeof());
        assert!(lib.has_xcb_change_gc_sizeof());
        assert!(lib.has_xcb_change_gc_checked());
        assert!(lib.has_xcb_change_gc());
        assert!(lib.has_xcb_change_gc_aux_checked());
        assert!(lib.has_xcb_change_gc_aux());
        assert!(lib.has_xcb_change_gc_value_list());
        assert!(lib.has_xcb_copy_gc_checked());
        assert!(lib.has_xcb_copy_gc());
        assert!(lib.has_xcb_set_dashes_sizeof());
        assert!(lib.has_xcb_set_dashes_checked());
        assert!(lib.has_xcb_set_dashes());
        assert!(lib.has_xcb_set_dashes_dashes());
        assert!(lib.has_xcb_set_dashes_dashes_length());
        assert!(lib.has_xcb_set_dashes_dashes_end());
        assert!(lib.has_xcb_set_clip_rectangles_sizeof());
        assert!(lib.has_xcb_set_clip_rectangles_checked());
        assert!(lib.has_xcb_set_clip_rectangles());
        assert!(lib.has_xcb_set_clip_rectangles_rectangles());
        assert!(lib.has_xcb_set_clip_rectangles_rectangles_length());
        assert!(lib.has_xcb_set_clip_rectangles_rectangles_iterator());
        assert!(lib.has_xcb_free_gc_checked());
        assert!(lib.has_xcb_free_gc());
        assert!(lib.has_xcb_clear_area_checked());
        assert!(lib.has_xcb_clear_area());
        assert!(lib.has_xcb_copy_area_checked());
        assert!(lib.has_xcb_copy_area());
        assert!(lib.has_xcb_copy_plane_checked());
        assert!(lib.has_xcb_copy_plane());
        assert!(lib.has_xcb_poly_point_sizeof());
        assert!(lib.has_xcb_poly_point_checked());
        assert!(lib.has_xcb_poly_point());
        assert!(lib.has_xcb_poly_point_points());
        assert!(lib.has_xcb_poly_point_points_length());
        assert!(lib.has_xcb_poly_point_points_iterator());
        assert!(lib.has_xcb_poly_line_sizeof());
        assert!(lib.has_xcb_poly_line_checked());
        assert!(lib.has_xcb_poly_line());
        assert!(lib.has_xcb_poly_line_points());
        assert!(lib.has_xcb_poly_line_points_length());
        assert!(lib.has_xcb_poly_line_points_iterator());
        assert!(lib.has_xcb_segment_next());
        assert!(lib.has_xcb_segment_end());
        assert!(lib.has_xcb_poly_segment_sizeof());
        assert!(lib.has_xcb_poly_segment_checked());
        assert!(lib.has_xcb_poly_segment());
        assert!(lib.has_xcb_poly_segment_segments());
        assert!(lib.has_xcb_poly_segment_segments_length());
        assert!(lib.has_xcb_poly_segment_segments_iterator());
        assert!(lib.has_xcb_poly_rectangle_sizeof());
        assert!(lib.has_xcb_poly_rectangle_checked());
        assert!(lib.has_xcb_poly_rectangle());
        assert!(lib.has_xcb_poly_rectangle_rectangles());
        assert!(lib.has_xcb_poly_rectangle_rectangles_length());
        assert!(lib.has_xcb_poly_rectangle_rectangles_iterator());
        assert!(lib.has_xcb_poly_arc_sizeof());
        assert!(lib.has_xcb_poly_arc_checked());
        assert!(lib.has_xcb_poly_arc());
        assert!(lib.has_xcb_poly_arc_arcs());
        assert!(lib.has_xcb_poly_arc_arcs_length());
        assert!(lib.has_xcb_poly_arc_arcs_iterator());
        assert!(lib.has_xcb_fill_poly_sizeof());
        assert!(lib.has_xcb_fill_poly_checked());
        assert!(lib.has_xcb_fill_poly());
        assert!(lib.has_xcb_fill_poly_points());
        assert!(lib.has_xcb_fill_poly_points_length());
        assert!(lib.has_xcb_fill_poly_points_iterator());
        assert!(lib.has_xcb_poly_fill_rectangle_sizeof());
        assert!(lib.has_xcb_poly_fill_rectangle_checked());
        assert!(lib.has_xcb_poly_fill_rectangle());
        assert!(lib.has_xcb_poly_fill_rectangle_rectangles());
        assert!(lib.has_xcb_poly_fill_rectangle_rectangles_length());
        assert!(lib.has_xcb_poly_fill_rectangle_rectangles_iterator());
        assert!(lib.has_xcb_poly_fill_arc_sizeof());
        assert!(lib.has_xcb_poly_fill_arc_checked());
        assert!(lib.has_xcb_poly_fill_arc());
        assert!(lib.has_xcb_poly_fill_arc_arcs());
        assert!(lib.has_xcb_poly_fill_arc_arcs_length());
        assert!(lib.has_xcb_poly_fill_arc_arcs_iterator());
        assert!(lib.has_xcb_put_image_sizeof());
        assert!(lib.has_xcb_put_image_checked());
        assert!(lib.has_xcb_put_image());
        assert!(lib.has_xcb_put_image_data());
        assert!(lib.has_xcb_put_image_data_length());
        assert!(lib.has_xcb_put_image_data_end());
        assert!(lib.has_xcb_get_image_sizeof());
        assert!(lib.has_xcb_get_image());
        assert!(lib.has_xcb_get_image_unchecked());
        assert!(lib.has_xcb_get_image_data());
        assert!(lib.has_xcb_get_image_data_length());
        assert!(lib.has_xcb_get_image_data_end());
        assert!(lib.has_xcb_get_image_reply());
        assert!(lib.has_xcb_poly_text_8_sizeof());
        assert!(lib.has_xcb_poly_text_8_checked());
        assert!(lib.has_xcb_poly_text_8());
        assert!(lib.has_xcb_poly_text_8_items());
        assert!(lib.has_xcb_poly_text_8_items_length());
        assert!(lib.has_xcb_poly_text_8_items_end());
        assert!(lib.has_xcb_poly_text_16_sizeof());
        assert!(lib.has_xcb_poly_text_16_checked());
        assert!(lib.has_xcb_poly_text_16());
        assert!(lib.has_xcb_poly_text_16_items());
        assert!(lib.has_xcb_poly_text_16_items_length());
        assert!(lib.has_xcb_poly_text_16_items_end());
        assert!(lib.has_xcb_image_text_8_sizeof());
        assert!(lib.has_xcb_image_text_8_checked());
        assert!(lib.has_xcb_image_text_8());
        assert!(lib.has_xcb_image_text_8_string());
        assert!(lib.has_xcb_image_text_8_string_length());
        assert!(lib.has_xcb_image_text_8_string_end());
        assert!(lib.has_xcb_image_text_16_sizeof());
        assert!(lib.has_xcb_image_text_16_checked());
        assert!(lib.has_xcb_image_text_16());
        assert!(lib.has_xcb_image_text_16_string());
        assert!(lib.has_xcb_image_text_16_string_length());
        assert!(lib.has_xcb_image_text_16_string_iterator());
        assert!(lib.has_xcb_create_colormap_checked());
        assert!(lib.has_xcb_create_colormap());
        assert!(lib.has_xcb_free_colormap_checked());
        assert!(lib.has_xcb_free_colormap());
        assert!(lib.has_xcb_copy_colormap_and_free_checked());
        assert!(lib.has_xcb_copy_colormap_and_free());
        assert!(lib.has_xcb_install_colormap_checked());
        assert!(lib.has_xcb_install_colormap());
        assert!(lib.has_xcb_uninstall_colormap_checked());
        assert!(lib.has_xcb_uninstall_colormap());
        assert!(lib.has_xcb_list_installed_colormaps_sizeof());
        assert!(lib.has_xcb_list_installed_colormaps());
        assert!(lib.has_xcb_list_installed_colormaps_unchecked());
        assert!(lib.has_xcb_list_installed_colormaps_cmaps());
        assert!(lib.has_xcb_list_installed_colormaps_cmaps_length());
        assert!(lib.has_xcb_list_installed_colormaps_cmaps_end());
        assert!(lib.has_xcb_list_installed_colormaps_reply());
        assert!(lib.has_xcb_alloc_color());
        assert!(lib.has_xcb_alloc_color_unchecked());
        assert!(lib.has_xcb_alloc_color_reply());
        assert!(lib.has_xcb_alloc_named_color_sizeof());
        assert!(lib.has_xcb_alloc_named_color());
        assert!(lib.has_xcb_alloc_named_color_unchecked());
        assert!(lib.has_xcb_alloc_named_color_reply());
        assert!(lib.has_xcb_alloc_color_cells_sizeof());
        assert!(lib.has_xcb_alloc_color_cells());
        assert!(lib.has_xcb_alloc_color_cells_unchecked());
        assert!(lib.has_xcb_alloc_color_cells_pixels());
        assert!(lib.has_xcb_alloc_color_cells_pixels_length());
        assert!(lib.has_xcb_alloc_color_cells_pixels_end());
        assert!(lib.has_xcb_alloc_color_cells_masks());
        assert!(lib.has_xcb_alloc_color_cells_masks_length());
        assert!(lib.has_xcb_alloc_color_cells_masks_end());
        assert!(lib.has_xcb_alloc_color_cells_reply());
        assert!(lib.has_xcb_alloc_color_planes_sizeof());
        assert!(lib.has_xcb_alloc_color_planes());
        assert!(lib.has_xcb_alloc_color_planes_unchecked());
        assert!(lib.has_xcb_alloc_color_planes_pixels());
        assert!(lib.has_xcb_alloc_color_planes_pixels_length());
        assert!(lib.has_xcb_alloc_color_planes_pixels_end());
        assert!(lib.has_xcb_alloc_color_planes_reply());
        assert!(lib.has_xcb_free_colors_sizeof());
        assert!(lib.has_xcb_free_colors_checked());
        assert!(lib.has_xcb_free_colors());
        assert!(lib.has_xcb_free_colors_pixels());
        assert!(lib.has_xcb_free_colors_pixels_length());
        assert!(lib.has_xcb_free_colors_pixels_end());
        assert!(lib.has_xcb_coloritem_next());
        assert!(lib.has_xcb_coloritem_end());
        assert!(lib.has_xcb_store_colors_sizeof());
        assert!(lib.has_xcb_store_colors_checked());
        assert!(lib.has_xcb_store_colors());
        assert!(lib.has_xcb_store_colors_items());
        assert!(lib.has_xcb_store_colors_items_length());
        assert!(lib.has_xcb_store_colors_items_iterator());
        assert!(lib.has_xcb_store_named_color_sizeof());
        assert!(lib.has_xcb_store_named_color_checked());
        assert!(lib.has_xcb_store_named_color());
        assert!(lib.has_xcb_store_named_color_name());
        assert!(lib.has_xcb_store_named_color_name_length());
        assert!(lib.has_xcb_store_named_color_name_end());
        assert!(lib.has_xcb_rgb_next());
        assert!(lib.has_xcb_rgb_end());
        assert!(lib.has_xcb_query_colors_sizeof());
        assert!(lib.has_xcb_query_colors());
        assert!(lib.has_xcb_query_colors_unchecked());
        assert!(lib.has_xcb_query_colors_colors());
        assert!(lib.has_xcb_query_colors_colors_length());
        assert!(lib.has_xcb_query_colors_colors_iterator());
        assert!(lib.has_xcb_query_colors_reply());
        assert!(lib.has_xcb_lookup_color_sizeof());
        assert!(lib.has_xcb_lookup_color());
        assert!(lib.has_xcb_lookup_color_unchecked());
        assert!(lib.has_xcb_lookup_color_reply());
        assert!(lib.has_xcb_create_cursor_checked());
        assert!(lib.has_xcb_create_cursor());
        assert!(lib.has_xcb_create_glyph_cursor_checked());
        assert!(lib.has_xcb_create_glyph_cursor());
        assert!(lib.has_xcb_free_cursor_checked());
        assert!(lib.has_xcb_free_cursor());
        assert!(lib.has_xcb_recolor_cursor_checked());
        assert!(lib.has_xcb_recolor_cursor());
        assert!(lib.has_xcb_query_best_size());
        assert!(lib.has_xcb_query_best_size_unchecked());
        assert!(lib.has_xcb_query_best_size_reply());
        assert!(lib.has_xcb_query_extension_sizeof());
        assert!(lib.has_xcb_query_extension());
        assert!(lib.has_xcb_query_extension_unchecked());
        assert!(lib.has_xcb_query_extension_reply());
        assert!(lib.has_xcb_list_extensions_sizeof());
        assert!(lib.has_xcb_list_extensions());
        assert!(lib.has_xcb_list_extensions_unchecked());
        assert!(lib.has_xcb_list_extensions_names_length());
        assert!(lib.has_xcb_list_extensions_names_iterator());
        assert!(lib.has_xcb_list_extensions_reply());
        assert!(lib.has_xcb_change_keyboard_mapping_sizeof());
        assert!(lib.has_xcb_change_keyboard_mapping_checked());
        assert!(lib.has_xcb_change_keyboard_mapping());
        assert!(lib.has_xcb_change_keyboard_mapping_keysyms());
        assert!(lib.has_xcb_change_keyboard_mapping_keysyms_length());
        assert!(lib.has_xcb_change_keyboard_mapping_keysyms_end());
        assert!(lib.has_xcb_get_keyboard_mapping_sizeof());
        assert!(lib.has_xcb_get_keyboard_mapping());
        assert!(lib.has_xcb_get_keyboard_mapping_unchecked());
        assert!(lib.has_xcb_get_keyboard_mapping_keysyms());
        assert!(lib.has_xcb_get_keyboard_mapping_keysyms_length());
        assert!(lib.has_xcb_get_keyboard_mapping_keysyms_end());
        assert!(lib.has_xcb_get_keyboard_mapping_reply());
        assert!(lib.has_xcb_change_keyboard_control_value_list_serialize());
        assert!(lib.has_xcb_change_keyboard_control_value_list_unpack());
        assert!(lib.has_xcb_change_keyboard_control_value_list_sizeof());
        assert!(lib.has_xcb_change_keyboard_control_sizeof());
        assert!(lib.has_xcb_change_keyboard_control_checked());
        assert!(lib.has_xcb_change_keyboard_control());
        assert!(lib.has_xcb_change_keyboard_control_aux_checked());
        assert!(lib.has_xcb_change_keyboard_control_aux());
        assert!(lib.has_xcb_change_keyboard_control_value_list());
        assert!(lib.has_xcb_get_keyboard_control());
        assert!(lib.has_xcb_get_keyboard_control_unchecked());
        assert!(lib.has_xcb_get_keyboard_control_reply());
        assert!(lib.has_xcb_bell_checked());
        assert!(lib.has_xcb_bell());
        assert!(lib.has_xcb_change_pointer_control_checked());
        assert!(lib.has_xcb_change_pointer_control());
        assert!(lib.has_xcb_get_pointer_control());
        assert!(lib.has_xcb_get_pointer_control_unchecked());
        assert!(lib.has_xcb_get_pointer_control_reply());
        assert!(lib.has_xcb_set_screen_saver_checked());
        assert!(lib.has_xcb_set_screen_saver());
        assert!(lib.has_xcb_get_screen_saver());
        assert!(lib.has_xcb_get_screen_saver_unchecked());
        assert!(lib.has_xcb_get_screen_saver_reply());
        assert!(lib.has_xcb_change_hosts_sizeof());
        assert!(lib.has_xcb_change_hosts_checked());
        assert!(lib.has_xcb_change_hosts());
        assert!(lib.has_xcb_change_hosts_address());
        assert!(lib.has_xcb_change_hosts_address_length());
        assert!(lib.has_xcb_change_hosts_address_end());
        assert!(lib.has_xcb_host_sizeof());
        assert!(lib.has_xcb_host_address());
        assert!(lib.has_xcb_host_address_length());
        assert!(lib.has_xcb_host_address_end());
        assert!(lib.has_xcb_host_next());
        assert!(lib.has_xcb_host_end());
        assert!(lib.has_xcb_list_hosts_sizeof());
        assert!(lib.has_xcb_list_hosts());
        assert!(lib.has_xcb_list_hosts_unchecked());
        assert!(lib.has_xcb_list_hosts_hosts_length());
        assert!(lib.has_xcb_list_hosts_hosts_iterator());
        assert!(lib.has_xcb_list_hosts_reply());
        assert!(lib.has_xcb_set_access_control_checked());
        assert!(lib.has_xcb_set_access_control());
        assert!(lib.has_xcb_set_close_down_mode_checked());
        assert!(lib.has_xcb_set_close_down_mode());
        assert!(lib.has_xcb_kill_client_checked());
        assert!(lib.has_xcb_kill_client());
        assert!(lib.has_xcb_rotate_properties_sizeof());
        assert!(lib.has_xcb_rotate_properties_checked());
        assert!(lib.has_xcb_rotate_properties());
        assert!(lib.has_xcb_rotate_properties_atoms());
        assert!(lib.has_xcb_rotate_properties_atoms_length());
        assert!(lib.has_xcb_rotate_properties_atoms_end());
        assert!(lib.has_xcb_force_screen_saver_checked());
        assert!(lib.has_xcb_force_screen_saver());
        assert!(lib.has_xcb_set_pointer_mapping_sizeof());
        assert!(lib.has_xcb_set_pointer_mapping());
        assert!(lib.has_xcb_set_pointer_mapping_unchecked());
        assert!(lib.has_xcb_set_pointer_mapping_reply());
        assert!(lib.has_xcb_get_pointer_mapping_sizeof());
        assert!(lib.has_xcb_get_pointer_mapping());
        assert!(lib.has_xcb_get_pointer_mapping_unchecked());
        assert!(lib.has_xcb_get_pointer_mapping_map());
        assert!(lib.has_xcb_get_pointer_mapping_map_length());
        assert!(lib.has_xcb_get_pointer_mapping_map_end());
        assert!(lib.has_xcb_get_pointer_mapping_reply());
        assert!(lib.has_xcb_set_modifier_mapping_sizeof());
        assert!(lib.has_xcb_set_modifier_mapping());
        assert!(lib.has_xcb_set_modifier_mapping_unchecked());
        assert!(lib.has_xcb_set_modifier_mapping_reply());
        assert!(lib.has_xcb_get_modifier_mapping_sizeof());
        assert!(lib.has_xcb_get_modifier_mapping());
        assert!(lib.has_xcb_get_modifier_mapping_unchecked());
        assert!(lib.has_xcb_get_modifier_mapping_keycodes());
        assert!(lib.has_xcb_get_modifier_mapping_keycodes_length());
        assert!(lib.has_xcb_get_modifier_mapping_keycodes_end());
        assert!(lib.has_xcb_get_modifier_mapping_reply());
        assert!(lib.has_xcb_no_operation_checked());
        assert!(lib.has_xcb_no_operation());
    }
}
