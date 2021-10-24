use crate::ffi::*;
use crate::*;
use std::os::raw::*;

pub const XCB_RANDR_MAJOR_VERSION: u32 = 1;
pub const XCB_RANDR_MINOR_VERSION: u32 = 4;

pub type xcb_randr_mode_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_mode_iterator_t {
    pub data: *mut xcb_randr_mode_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_randr_crtc_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_crtc_iterator_t {
    pub data: *mut xcb_randr_crtc_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_randr_output_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_output_iterator_t {
    pub data: *mut xcb_randr_output_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_randr_provider_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_provider_iterator_t {
    pub data: *mut xcb_randr_provider_t,
    pub rem: c_int,
    pub index: c_int,
}

pub const XCB_RANDR_BAD_OUTPUT: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_bad_output_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

pub const XCB_RANDR_BAD_CRTC: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_bad_crtc_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

pub const XCB_RANDR_BAD_MODE: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_bad_mode_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

pub const XCB_RANDR_BAD_PROVIDER: u8 = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_bad_provider_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

pub type xcb_randr_rotation_t = u32;
pub const XCB_RANDR_ROTATION_ROTATE_0: xcb_randr_rotation_t = 0x01;
pub const XCB_RANDR_ROTATION_ROTATE_90: xcb_randr_rotation_t = 0x02;
pub const XCB_RANDR_ROTATION_ROTATE_180: xcb_randr_rotation_t = 0x04;
pub const XCB_RANDR_ROTATION_ROTATE_270: xcb_randr_rotation_t = 0x08;
pub const XCB_RANDR_ROTATION_REFLECT_X: xcb_randr_rotation_t = 0x10;
pub const XCB_RANDR_ROTATION_REFLECT_Y: xcb_randr_rotation_t = 0x20;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_screen_size_t {
    pub width: u16,
    pub height: u16,
    pub mwidth: u16,
    pub mheight: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_screen_size_iterator_t {
    pub data: *mut xcb_randr_screen_size_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_refresh_rates_t {
    pub n_rates: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_refresh_rates_iterator_t {
    pub data: *mut xcb_randr_refresh_rates_t,
    pub rem: c_int,
    pub index: c_int,
}

pub const XCB_RANDR_QUERY_VERSION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub major_version: u32,
    pub minor_version: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_query_version_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
    pub pad1: [u8; 16],
}

pub type xcb_randr_set_config_t = u32;
pub const XCB_RANDR_SET_CONFIG_SUCCESS: xcb_randr_set_config_t = 0x00;
pub const XCB_RANDR_SET_CONFIG_INVALID_CONFIG_TIME: xcb_randr_set_config_t = 0x01;
pub const XCB_RANDR_SET_CONFIG_INVALID_TIME: xcb_randr_set_config_t = 0x02;
pub const XCB_RANDR_SET_CONFIG_FAILED: xcb_randr_set_config_t = 0x03;

pub const XCB_RANDR_SET_SCREEN_CONFIG: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_screen_config_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub timestamp: xcb_timestamp_t,
    pub config_timestamp: xcb_timestamp_t,
    pub size_id: u16,
    pub rotation: u16,
    pub rate: u16,
    pub pad0: [u8; 2],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_screen_config_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_screen_config_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
    pub new_timestamp: xcb_timestamp_t,
    pub config_timestamp: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub subpixel_order: u16,
    pub pad0: [u8; 10],
}

pub type xcb_randr_notify_mask_t = u32;
pub const XCB_RANDR_NOTIFY_MASK_SCREEN_CHANGE: xcb_randr_notify_mask_t = 0x01;
pub const XCB_RANDR_NOTIFY_MASK_CRTC_CHANGE: xcb_randr_notify_mask_t = 0x02;
pub const XCB_RANDR_NOTIFY_MASK_OUTPUT_CHANGE: xcb_randr_notify_mask_t = 0x04;
pub const XCB_RANDR_NOTIFY_MASK_OUTPUT_PROPERTY: xcb_randr_notify_mask_t = 0x08;
pub const XCB_RANDR_NOTIFY_MASK_PROVIDER_CHANGE: xcb_randr_notify_mask_t = 0x10;
pub const XCB_RANDR_NOTIFY_MASK_PROVIDER_PROPERTY: xcb_randr_notify_mask_t = 0x20;
pub const XCB_RANDR_NOTIFY_MASK_RESOURCE_CHANGE: xcb_randr_notify_mask_t = 0x40;

pub const XCB_RANDR_SELECT_INPUT: u8 = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_select_input_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub enable: u16,
    pub pad0: [u8; 2],
}

pub const XCB_RANDR_GET_SCREEN_INFO: u8 = 5;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_screen_info_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_screen_info_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_screen_info_reply_t {
    pub response_type: u8,
    pub rotations: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: xcb_window_t,
    pub timestamp: xcb_timestamp_t,
    pub config_timestamp: xcb_timestamp_t,
    pub n_sizes: u16,
    pub size_id: u16,
    pub rotation: u16,
    pub rate: u16,
    pub n_info: u16,
    pub pad0: [u8; 2],
}

pub const XCB_RANDR_GET_SCREEN_SIZE_RANGE: u8 = 6;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_screen_size_range_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_screen_size_range_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_screen_size_range_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub min_width: u16,
    pub min_height: u16,
    pub max_width: u16,
    pub max_height: u16,
    pub pad1: [u8; 16],
}

pub const XCB_RANDR_SET_SCREEN_SIZE: u8 = 7;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_screen_size_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub width: u16,
    pub height: u16,
    pub mm_width: u32,
    pub mm_height: u32,
}

pub type xcb_randr_mode_flag_t = u32;
pub const XCB_RANDR_MODE_FLAG_HSYNC_POSITIVE: xcb_randr_mode_flag_t = 0x01;
pub const XCB_RANDR_MODE_FLAG_HSYNC_NEGATIVE: xcb_randr_mode_flag_t = 0x02;
pub const XCB_RANDR_MODE_FLAG_VSYNC_POSITIVE: xcb_randr_mode_flag_t = 0x04;
pub const XCB_RANDR_MODE_FLAG_VSYNC_NEGATIVE: xcb_randr_mode_flag_t = 0x08;
pub const XCB_RANDR_MODE_FLAG_INTERLACE: xcb_randr_mode_flag_t = 0x10;
pub const XCB_RANDR_MODE_FLAG_DOUBLE_SCAN: xcb_randr_mode_flag_t = 0x20;
pub const XCB_RANDR_MODE_FLAG_CSYNC: xcb_randr_mode_flag_t = 0x40;
pub const XCB_RANDR_MODE_FLAG_CSYNC_POSITIVE: xcb_randr_mode_flag_t = 0x80;
pub const XCB_RANDR_MODE_FLAG_CSYNC_NEGATIVE: xcb_randr_mode_flag_t = 0x100;
pub const XCB_RANDR_MODE_FLAG_HSKEW_PRESENT: xcb_randr_mode_flag_t = 0x200;
pub const XCB_RANDR_MODE_FLAG_BCAST: xcb_randr_mode_flag_t = 0x400;
pub const XCB_RANDR_MODE_FLAG_PIXEL_MULTIPLEX: xcb_randr_mode_flag_t = 0x800;
pub const XCB_RANDR_MODE_FLAG_DOUBLE_CLOCK: xcb_randr_mode_flag_t = 0x1000;
pub const XCB_RANDR_MODE_FLAG_HALVE_CLOCK: xcb_randr_mode_flag_t = 0x2000;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_mode_info_t {
    pub id: u32,
    pub width: u16,
    pub height: u16,
    pub dot_clock: u32,
    pub hsync_start: u16,
    pub hsync_end: u16,
    pub htotal: u16,
    pub hskew: u16,
    pub vsync_start: u16,
    pub vsync_end: u16,
    pub vtotal: u16,
    pub name_len: u16,
    pub mode_flags: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_mode_info_iterator_t {
    pub data: *mut xcb_randr_mode_info_t,
    pub rem: c_int,
    pub index: c_int,
}

pub const XCB_RANDR_GET_SCREEN_RESOURCES: u8 = 8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_screen_resources_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_screen_resources_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_screen_resources_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xcb_timestamp_t,
    pub config_timestamp: xcb_timestamp_t,
    pub num_crtcs: u16,
    pub num_outputs: u16,
    pub num_modes: u16,
    pub names_len: u16,
    pub pad1: [u8; 8],
}

pub type xcb_randr_connection_t = u32;
pub const XCB_RANDR_CONNECTION_CONNECTED: xcb_randr_connection_t = 0x00;
pub const XCB_RANDR_CONNECTION_DISCONNECTED: xcb_randr_connection_t = 0x01;
pub const XCB_RANDR_CONNECTION_UNKNOWN: xcb_randr_connection_t = 0x02;

pub const XCB_RANDR_GET_OUTPUT_INFO: u8 = 9;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_output_info_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub output: xcb_randr_output_t,
    pub config_timestamp: xcb_timestamp_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_output_info_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_output_info_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xcb_timestamp_t,
    pub crtc: xcb_randr_crtc_t,
    pub mm_width: u32,
    pub mm_height: u32,
    pub connection: u8,
    pub subpixel_order: u8,
    pub num_crtcs: u16,
    pub num_modes: u16,
    pub num_preferred: u16,
    pub num_clones: u16,
    pub name_len: u16,
}

pub const XCB_RANDR_LIST_OUTPUT_PROPERTIES: u8 = 10;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_list_output_properties_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub output: xcb_randr_output_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_list_output_properties_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_list_output_properties_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_atoms: u16,
    pub pad1: [u8; 22],
}

pub const XCB_RANDR_QUERY_OUTPUT_PROPERTY: u8 = 11;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_query_output_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub output: xcb_randr_output_t,
    pub property: xcb_atom_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_query_output_property_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_query_output_property_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pending: u8,
    pub range: u8,
    pub immutable: u8,
    pub pad1: [u8; 21],
}

pub const XCB_RANDR_CONFIGURE_OUTPUT_PROPERTY: u8 = 12;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_configure_output_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub output: xcb_randr_output_t,
    pub property: xcb_atom_t,
    pub pending: u8,
    pub range: u8,
    pub pad0: [u8; 2],
}

pub const XCB_RANDR_CHANGE_OUTPUT_PROPERTY: u8 = 13;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_change_output_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub output: xcb_randr_output_t,
    pub property: xcb_atom_t,
    pub type_: xcb_atom_t,
    pub format: u8,
    pub mode: u8,
    pub pad0: [u8; 2],
    pub num_units: u32,
}

pub const XCB_RANDR_DELETE_OUTPUT_PROPERTY: u8 = 14;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_delete_output_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub output: xcb_randr_output_t,
    pub property: xcb_atom_t,
}

pub const XCB_RANDR_GET_OUTPUT_PROPERTY: u8 = 15;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_output_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub output: xcb_randr_output_t,
    pub property: xcb_atom_t,
    pub type_: xcb_atom_t,
    pub long_offset: u32,
    pub long_length: u32,
    pub delete: u8,
    pub pending: u8,
    pub pad0: [u8; 2],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_output_property_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_output_property_reply_t {
    pub response_type: u8,
    pub format: u8,
    pub sequence: u16,
    pub length: u32,
    pub type_: xcb_atom_t,
    pub bytes_after: u32,
    pub num_items: u32,
    pub pad0: [u8; 12],
}

pub const XCB_RANDR_CREATE_MODE: u8 = 16;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_create_mode_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub mode_info: xcb_randr_mode_info_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_create_mode_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_create_mode_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub mode: xcb_randr_mode_t,
    pub pad1: [u8; 20],
}

pub const XCB_RANDR_DESTROY_MODE: u8 = 17;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_destroy_mode_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub mode: xcb_randr_mode_t,
}

pub const XCB_RANDR_ADD_OUTPUT_MODE: u8 = 18;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_add_output_mode_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub output: xcb_randr_output_t,
    pub mode: xcb_randr_mode_t,
}

pub const XCB_RANDR_DELETE_OUTPUT_MODE: u8 = 19;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_delete_output_mode_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub output: xcb_randr_output_t,
    pub mode: xcb_randr_mode_t,
}

pub const XCB_RANDR_GET_CRTC_INFO: u8 = 20;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_crtc_info_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub crtc: xcb_randr_crtc_t,
    pub config_timestamp: xcb_timestamp_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_crtc_info_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_crtc_info_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xcb_timestamp_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub mode: xcb_randr_mode_t,
    pub rotation: u16,
    pub rotations: u16,
    pub num_outputs: u16,
    pub num_possible_outputs: u16,
}

pub const XCB_RANDR_SET_CRTC_CONFIG: u8 = 21;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_crtc_config_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub crtc: xcb_randr_crtc_t,
    pub timestamp: xcb_timestamp_t,
    pub config_timestamp: xcb_timestamp_t,
    pub x: i16,
    pub y: i16,
    pub mode: xcb_randr_mode_t,
    pub rotation: u16,
    pub pad0: [u8; 2],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_crtc_config_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_crtc_config_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xcb_timestamp_t,
    pub pad0: [u8; 20],
}

pub const XCB_RANDR_GET_CRTC_GAMMA_SIZE: u8 = 22;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_crtc_gamma_size_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub crtc: xcb_randr_crtc_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_crtc_gamma_size_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_crtc_gamma_size_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub size: u16,
    pub pad1: [u8; 22],
}

pub const XCB_RANDR_GET_CRTC_GAMMA: u8 = 23;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_crtc_gamma_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub crtc: xcb_randr_crtc_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_crtc_gamma_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_crtc_gamma_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub size: u16,
    pub pad1: [u8; 22],
}

pub const XCB_RANDR_SET_CRTC_GAMMA: u8 = 24;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_crtc_gamma_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub crtc: xcb_randr_crtc_t,
    pub size: u16,
    pub pad0: [u8; 2],
}

pub const XCB_RANDR_GET_SCREEN_RESOURCES_CURRENT: u8 = 25;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_screen_resources_current_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_screen_resources_current_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_screen_resources_current_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xcb_timestamp_t,
    pub config_timestamp: xcb_timestamp_t,
    pub num_crtcs: u16,
    pub num_outputs: u16,
    pub num_modes: u16,
    pub names_len: u16,
    pub pad1: [u8; 8],
}

pub type xcb_randr_transform_t = u32;
pub const XCB_RANDR_TRANSFORM_UNIT: xcb_randr_transform_t = 0x01;
pub const XCB_RANDR_TRANSFORM_SCALE_UP: xcb_randr_transform_t = 0x02;
pub const XCB_RANDR_TRANSFORM_SCALE_DOWN: xcb_randr_transform_t = 0x04;
pub const XCB_RANDR_TRANSFORM_PROJECTIVE: xcb_randr_transform_t = 0x08;

pub const XCB_RANDR_SET_CRTC_TRANSFORM: u8 = 26;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_crtc_transform_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub crtc: xcb_randr_crtc_t,
    pub transform: xcb_render_transform_t,
    pub filter_len: u16,
    pub pad0: [u8; 2],
}

pub const XCB_RANDR_GET_CRTC_TRANSFORM: u8 = 27;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_crtc_transform_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub crtc: xcb_randr_crtc_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_crtc_transform_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_crtc_transform_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pending_transform: xcb_render_transform_t,
    pub has_transforms: u8,
    pub pad1: [u8; 3],
    pub current_transform: xcb_render_transform_t,
    pub pad2: [u8; 4],
    pub pending_len: u16,
    pub pending_nparams: u16,
    pub current_len: u16,
    pub current_nparams: u16,
}

pub const XCB_RANDR_GET_PANNING: u8 = 28;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_panning_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub crtc: xcb_randr_crtc_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_panning_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_panning_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xcb_timestamp_t,
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16,
    pub track_left: u16,
    pub track_top: u16,
    pub track_width: u16,
    pub track_height: u16,
    pub border_left: i16,
    pub border_top: i16,
    pub border_right: i16,
    pub border_bottom: i16,
}

pub const XCB_RANDR_SET_PANNING: u8 = 29;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_panning_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub crtc: xcb_randr_crtc_t,
    pub timestamp: xcb_timestamp_t,
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16,
    pub track_left: u16,
    pub track_top: u16,
    pub track_width: u16,
    pub track_height: u16,
    pub border_left: i16,
    pub border_top: i16,
    pub border_right: i16,
    pub border_bottom: i16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_panning_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_panning_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xcb_timestamp_t,
}

pub const XCB_RANDR_SET_OUTPUT_PRIMARY: u8 = 30;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_output_primary_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub output: xcb_randr_output_t,
}

pub const XCB_RANDR_GET_OUTPUT_PRIMARY: u8 = 31;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_output_primary_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_output_primary_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_output_primary_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub output: xcb_randr_output_t,
}

pub const XCB_RANDR_GET_PROVIDERS: u8 = 32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_providers_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_providers_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_providers_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xcb_timestamp_t,
    pub num_providers: u16,
    pub pad1: [u8; 18],
}

pub type xcb_randr_provider_capability_t = u32;
pub const XCB_RANDR_PROVIDER_CAPABILITY_SOURCE_OUTPUT: xcb_randr_provider_capability_t = 0x01;
pub const XCB_RANDR_PROVIDER_CAPABILITY_SINK_OUTPUT: xcb_randr_provider_capability_t = 0x02;
pub const XCB_RANDR_PROVIDER_CAPABILITY_SOURCE_OFFLOAD: xcb_randr_provider_capability_t = 0x04;
pub const XCB_RANDR_PROVIDER_CAPABILITY_SINK_OFFLOAD: xcb_randr_provider_capability_t = 0x08;

pub const XCB_RANDR_GET_PROVIDER_INFO: u8 = 33;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_provider_info_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub provider: xcb_randr_provider_t,
    pub config_timestamp: xcb_timestamp_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_provider_info_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_provider_info_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
    pub timestamp: xcb_timestamp_t,
    pub capabilities: u32,
    pub num_crtcs: u16,
    pub num_outputs: u16,
    pub num_associated_providers: u16,
    pub name_len: u16,
    pub pad0: [u8; 8],
}

pub const XCB_RANDR_SET_PROVIDER_OFFLOAD_SINK: u8 = 34;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_provider_offload_sink_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub provider: xcb_randr_provider_t,
    pub sink_provider: xcb_randr_provider_t,
    pub config_timestamp: xcb_timestamp_t,
}

pub const XCB_RANDR_SET_PROVIDER_OUTPUT_SOURCE: u8 = 35;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_set_provider_output_source_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub provider: xcb_randr_provider_t,
    pub source_provider: xcb_randr_provider_t,
    pub config_timestamp: xcb_timestamp_t,
}

pub const XCB_RANDR_LIST_PROVIDER_PROPERTIES: u8 = 36;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_list_provider_properties_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub provider: xcb_randr_provider_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_list_provider_properties_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_list_provider_properties_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_atoms: u16,
    pub pad1: [u8; 22],
}

pub const XCB_RANDR_QUERY_PROVIDER_PROPERTY: u8 = 37;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_query_provider_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub provider: xcb_randr_provider_t,
    pub property: xcb_atom_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_query_provider_property_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_query_provider_property_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pending: u8,
    pub range: u8,
    pub immutable: u8,
    pub pad1: [u8; 21],
}

pub const XCB_RANDR_CONFIGURE_PROVIDER_PROPERTY: u8 = 38;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_configure_provider_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub provider: xcb_randr_provider_t,
    pub property: xcb_atom_t,
    pub pending: u8,
    pub range: u8,
    pub pad0: [u8; 2],
}

pub const XCB_RANDR_CHANGE_PROVIDER_PROPERTY: u8 = 39;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_change_provider_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub provider: xcb_randr_provider_t,
    pub property: xcb_atom_t,
    pub type_: xcb_atom_t,
    pub format: u8,
    pub mode: u8,
    pub pad0: [u8; 2],
    pub num_items: u32,
}

pub const XCB_RANDR_DELETE_PROVIDER_PROPERTY: u8 = 40;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_delete_provider_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub provider: xcb_randr_provider_t,
    pub property: xcb_atom_t,
}

pub const XCB_RANDR_GET_PROVIDER_PROPERTY: u8 = 41;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_provider_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub provider: xcb_randr_provider_t,
    pub property: xcb_atom_t,
    pub type_: xcb_atom_t,
    pub long_offset: u32,
    pub long_length: u32,
    pub delete: u8,
    pub pending: u8,
    pub pad0: [u8; 2],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_provider_property_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_get_provider_property_reply_t {
    pub response_type: u8,
    pub format: u8,
    pub sequence: u16,
    pub length: u32,
    pub type_: xcb_atom_t,
    pub bytes_after: u32,
    pub num_items: u32,
    pub pad0: [u8; 12],
}

pub const XCB_RANDR_SCREEN_CHANGE_NOTIFY: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_screen_change_notify_event_t {
    pub response_type: u8,
    pub rotation: u8,
    pub sequence: u16,
    pub timestamp: xcb_timestamp_t,
    pub config_timestamp: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub request_window: xcb_window_t,
    pub size_id: u16,
    pub subpixel_order: u16,
    pub width: u16,
    pub height: u16,
    pub mwidth: u16,
    pub mheight: u16,
}

pub type xcb_randr_notify_t = u32;
pub const XCB_RANDR_NOTIFY_CRTC_CHANGE: xcb_randr_notify_t = 0x00;
pub const XCB_RANDR_NOTIFY_OUTPUT_CHANGE: xcb_randr_notify_t = 0x01;
pub const XCB_RANDR_NOTIFY_OUTPUT_PROPERTY: xcb_randr_notify_t = 0x02;
pub const XCB_RANDR_NOTIFY_PROVIDER_CHANGE: xcb_randr_notify_t = 0x03;
pub const XCB_RANDR_NOTIFY_PROVIDER_PROPERTY: xcb_randr_notify_t = 0x04;
pub const XCB_RANDR_NOTIFY_RESOURCE_CHANGE: xcb_randr_notify_t = 0x05;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_crtc_change_t {
    pub timestamp: xcb_timestamp_t,
    pub window: xcb_window_t,
    pub crtc: xcb_randr_crtc_t,
    pub mode: xcb_randr_mode_t,
    pub rotation: u16,
    pub pad0: [u8; 2],
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_crtc_change_iterator_t {
    pub data: *mut xcb_randr_crtc_change_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_output_change_t {
    pub timestamp: xcb_timestamp_t,
    pub config_timestamp: xcb_timestamp_t,
    pub window: xcb_window_t,
    pub output: xcb_randr_output_t,
    pub crtc: xcb_randr_crtc_t,
    pub mode: xcb_randr_mode_t,
    pub rotation: u16,
    pub connection: u8,
    pub subpixel_order: u8,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_output_change_iterator_t {
    pub data: *mut xcb_randr_output_change_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_output_property_t {
    pub window: xcb_window_t,
    pub output: xcb_randr_output_t,
    pub atom: xcb_atom_t,
    pub timestamp: xcb_timestamp_t,
    pub status: u8,
    pub pad0: [u8; 11],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_output_property_iterator_t {
    pub data: *mut xcb_randr_output_property_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_provider_change_t {
    pub timestamp: xcb_timestamp_t,
    pub window: xcb_window_t,
    pub provider: xcb_randr_provider_t,
    pub pad0: [u8; 16],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_provider_change_iterator_t {
    pub data: *mut xcb_randr_provider_change_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_provider_property_t {
    pub window: xcb_window_t,
    pub provider: xcb_randr_provider_t,
    pub atom: xcb_atom_t,
    pub timestamp: xcb_timestamp_t,
    pub state: u8,
    pub pad0: [u8; 11],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_provider_property_iterator_t {
    pub data: *mut xcb_randr_provider_property_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_resource_change_t {
    pub timestamp: xcb_timestamp_t,
    pub window: xcb_window_t,
    pub pad0: [u8; 20],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_resource_change_iterator_t {
    pub data: *mut xcb_randr_resource_change_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union xcb_randr_notify_data_t {
    pub cc: xcb_randr_crtc_change_t,
    pub oc: xcb_randr_output_change_t,
    pub op: xcb_randr_output_property_t,
    pub pc: xcb_randr_provider_change_t,
    pub pp: xcb_randr_provider_property_t,
    pub rc: xcb_randr_resource_change_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_notify_data_iterator_t {
    pub data: *mut xcb_randr_notify_data_t,
    pub rem: c_int,
    pub index: c_int,
}

pub const XCB_RANDR_NOTIFY: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_randr_notify_event_t {
    pub response_type: u8,
    pub sub_code: u8,
    pub sequence: u16,
    pub u: xcb_randr_notify_data_t,
}

impl XcbRandr {
    #[inline]
    pub unsafe fn xcb_randr_id(&self) -> *mut xcb_extension_t {
        sym!(self, xcb_randr_id)
    }

    #[inline]
    pub unsafe fn xcb_randr_mode_next(&self, i: *mut xcb_randr_mode_iterator_t) {
        sym!(self, xcb_randr_mode_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_randr_mode_end(
        &self,
        i: *mut xcb_randr_mode_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_mode_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_randr_crtc_next(&self, i: *mut xcb_randr_crtc_iterator_t) {
        sym!(self, xcb_randr_crtc_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_randr_crtc_end(
        &self,
        i: *mut xcb_randr_crtc_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_crtc_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_randr_output_next(&self, i: *mut xcb_randr_output_iterator_t) {
        sym!(self, xcb_randr_output_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_randr_output_end(
        &self,
        i: *mut xcb_randr_output_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_output_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_randr_provider_next(&self, i: *mut xcb_randr_provider_iterator_t) {
        sym!(self, xcb_randr_provider_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_randr_provider_end(
        &self,
        i: *mut xcb_randr_provider_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_provider_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_randr_screen_size_next(&self, i: *mut xcb_randr_screen_size_iterator_t) {
        sym!(self, xcb_randr_screen_size_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_randr_screen_size_end(
        &self,
        i: *mut xcb_randr_screen_size_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_screen_size_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_randr_refresh_rates_rates(
        &self,
        R: *const xcb_randr_refresh_rates_t,
    ) -> *mut u16 {
        sym!(self, xcb_randr_refresh_rates_rates)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_refresh_rates_rates_length(
        &self,
        R: *const xcb_randr_refresh_rates_t,
    ) -> c_int {
        sym!(self, xcb_randr_refresh_rates_rates_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_refresh_rates_rates_end(
        &self,
        R: *const xcb_randr_refresh_rates_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_refresh_rates_rates_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_refresh_rates_next(&self, i: *mut xcb_randr_refresh_rates_iterator_t) {
        sym!(self, xcb_randr_refresh_rates_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_randr_refresh_rates_end(
        &self,
        i: *mut xcb_randr_refresh_rates_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_refresh_rates_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_randr_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_query_version_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_query_version_reply_t {
        sym!(self, xcb_randr_query_version_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_randr_query_version(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
    ) -> xcb_randr_query_version_cookie_t {
        sym!(self, xcb_randr_query_version)(c, major_version, minor_version)
    }

    #[inline]
    pub unsafe fn xcb_randr_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
    ) -> xcb_randr_query_version_cookie_t {
        sym!(self, xcb_randr_query_version_unchecked)(c, major_version, minor_version)
    }

    #[inline]
    pub unsafe fn xcb_randr_set_screen_config_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_set_screen_config_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_set_screen_config_reply_t {
        sym!(self, xcb_randr_set_screen_config_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_randr_set_screen_config(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        timestamp: xcb_timestamp_t,
        config_timestamp: xcb_timestamp_t,
        size_id: u16,
        rotation: u16,
        rate: u16,
    ) -> xcb_randr_set_screen_config_cookie_t {
        sym!(self, xcb_randr_set_screen_config)(
            c,
            window,
            timestamp,
            config_timestamp,
            size_id,
            rotation,
            rate,
        )
    }

    #[inline]
    pub unsafe fn xcb_randr_set_screen_config_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        timestamp: xcb_timestamp_t,
        config_timestamp: xcb_timestamp_t,
        size_id: u16,
        rotation: u16,
        rate: u16,
    ) -> xcb_randr_set_screen_config_cookie_t {
        sym!(self, xcb_randr_set_screen_config_unchecked)(
            c,
            window,
            timestamp,
            config_timestamp,
            size_id,
            rotation,
            rate,
        )
    }

    #[inline]
    pub unsafe fn xcb_randr_select_input(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        enable: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_select_input)(c, window, enable)
    }

    #[inline]
    pub unsafe fn xcb_randr_select_input_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        enable: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_select_input_checked)(c, window, enable)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_info_sizes(
        &self,
        R: *const xcb_randr_get_screen_info_reply_t,
    ) -> *mut xcb_randr_screen_size_t {
        sym!(self, xcb_randr_get_screen_info_sizes)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_info_sizes_length(
        &self,
        R: *const xcb_randr_get_screen_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_screen_info_sizes_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_info_sizes_iterator(
        &self,
        R: *const xcb_randr_get_screen_info_reply_t,
    ) -> xcb_randr_screen_size_iterator_t {
        sym!(self, xcb_randr_get_screen_info_sizes_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_info_rates_length(
        &self,
        R: *const xcb_randr_get_screen_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_screen_info_rates_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_info_rates_iterator(
        &self,
        R: *const xcb_randr_get_screen_info_reply_t,
    ) -> xcb_randr_refresh_rates_iterator_t {
        sym!(self, xcb_randr_get_screen_info_rates_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_info_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_screen_info_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_screen_info_reply_t {
        sym!(self, xcb_randr_get_screen_info_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_info(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_screen_info_cookie_t {
        sym!(self, xcb_randr_get_screen_info)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_info_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_screen_info_cookie_t {
        sym!(self, xcb_randr_get_screen_info_unchecked)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_size_range_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_screen_size_range_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_screen_size_range_reply_t {
        sym!(self, xcb_randr_get_screen_size_range_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_size_range(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_screen_size_range_cookie_t {
        sym!(self, xcb_randr_get_screen_size_range)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_size_range_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_screen_size_range_cookie_t {
        sym!(self, xcb_randr_get_screen_size_range_unchecked)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_randr_set_screen_size(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        width: u16,
        height: u16,
        mm_width: u32,
        mm_height: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_screen_size)(c, window, width, height, mm_width, mm_height)
    }

    #[inline]
    pub unsafe fn xcb_randr_set_screen_size_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        width: u16,
        height: u16,
        mm_width: u32,
        mm_height: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_screen_size_checked)(c, window, width, height, mm_width, mm_height)
    }

    #[inline]
    pub unsafe fn xcb_randr_mode_info_next(&self, i: *mut xcb_randr_mode_info_iterator_t) {
        sym!(self, xcb_randr_mode_info_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_randr_mode_info_end(
        &self,
        i: *mut xcb_randr_mode_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_mode_info_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_crtcs(
        &self,
        R: *const xcb_randr_get_screen_resources_reply_t,
    ) -> *mut xcb_randr_crtc_t {
        sym!(self, xcb_randr_get_screen_resources_crtcs)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_crtcs_length(
        &self,
        R: *const xcb_randr_get_screen_resources_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_screen_resources_crtcs_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_crtcs_end(
        &self,
        R: *const xcb_randr_get_screen_resources_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_screen_resources_crtcs_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_outputs(
        &self,
        R: *const xcb_randr_get_screen_resources_reply_t,
    ) -> *mut xcb_randr_output_t {
        sym!(self, xcb_randr_get_screen_resources_outputs)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_outputs_length(
        &self,
        R: *const xcb_randr_get_screen_resources_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_screen_resources_outputs_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_outputs_end(
        &self,
        R: *const xcb_randr_get_screen_resources_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_screen_resources_outputs_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_modes(
        &self,
        R: *const xcb_randr_get_screen_resources_reply_t,
    ) -> *mut xcb_randr_mode_info_t {
        sym!(self, xcb_randr_get_screen_resources_modes)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_modes_length(
        &self,
        R: *const xcb_randr_get_screen_resources_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_screen_resources_modes_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_modes_iterator(
        &self,
        R: *const xcb_randr_get_screen_resources_reply_t,
    ) -> xcb_randr_mode_info_iterator_t {
        sym!(self, xcb_randr_get_screen_resources_modes_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_names(
        &self,
        R: *const xcb_randr_get_screen_resources_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_randr_get_screen_resources_names)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_names_length(
        &self,
        R: *const xcb_randr_get_screen_resources_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_screen_resources_names_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_names_end(
        &self,
        R: *const xcb_randr_get_screen_resources_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_screen_resources_names_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_screen_resources_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_screen_resources_reply_t {
        sym!(self, xcb_randr_get_screen_resources_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_screen_resources_cookie_t {
        sym!(self, xcb_randr_get_screen_resources)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_screen_resources_cookie_t {
        sym!(self, xcb_randr_get_screen_resources_unchecked)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_output_info_crtcs(
        &self,
        R: *const xcb_randr_get_output_info_reply_t,
    ) -> *mut xcb_randr_crtc_t {
        sym!(self, xcb_randr_get_output_info_crtcs)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_output_info_crtcs_length(
        &self,
        R: *const xcb_randr_get_output_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_output_info_crtcs_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_output_info_crtcs_end(
        &self,
        R: *const xcb_randr_get_output_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_output_info_crtcs_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_output_info_modes(
        &self,
        R: *const xcb_randr_get_output_info_reply_t,
    ) -> *mut xcb_randr_mode_t {
        sym!(self, xcb_randr_get_output_info_modes)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_output_info_modes_length(
        &self,
        R: *const xcb_randr_get_output_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_output_info_modes_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_output_info_modes_end(
        &self,
        R: *const xcb_randr_get_output_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_output_info_modes_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_output_info_clones(
        &self,
        R: *const xcb_randr_get_output_info_reply_t,
    ) -> *mut xcb_randr_output_t {
        sym!(self, xcb_randr_get_output_info_clones)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_output_info_clones_length(
        &self,
        R: *const xcb_randr_get_output_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_output_info_clones_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_output_info_clones_end(
        &self,
        R: *const xcb_randr_get_output_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_output_info_clones_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_output_info_name(
        &self,
        R: *const xcb_randr_get_output_info_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_randr_get_output_info_name)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_output_info_name_length(
        &self,
        R: *const xcb_randr_get_output_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_output_info_name_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_output_info_name_end(
        &self,
        R: *const xcb_randr_get_output_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_output_info_name_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_output_info_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_output_info_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_output_info_reply_t {
        sym!(self, xcb_randr_get_output_info_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_output_info(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_randr_get_output_info_cookie_t {
        sym!(self, xcb_randr_get_output_info)(c, output, config_timestamp)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_output_info_unchecked(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_randr_get_output_info_cookie_t {
        sym!(self, xcb_randr_get_output_info_unchecked)(c, output, config_timestamp)
    }

    #[inline]
    pub unsafe fn xcb_randr_list_output_properties_atoms(
        &self,
        R: *const xcb_randr_list_output_properties_reply_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_randr_list_output_properties_atoms)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_list_output_properties_atoms_length(
        &self,
        R: *const xcb_randr_list_output_properties_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_list_output_properties_atoms_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_list_output_properties_atoms_end(
        &self,
        R: *const xcb_randr_list_output_properties_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_list_output_properties_atoms_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_list_output_properties_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_list_output_properties_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_list_output_properties_reply_t {
        sym!(self, xcb_randr_list_output_properties_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_randr_list_output_properties(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
    ) -> xcb_randr_list_output_properties_cookie_t {
        sym!(self, xcb_randr_list_output_properties)(c, output)
    }

    #[inline]
    pub unsafe fn xcb_randr_list_output_properties_unchecked(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
    ) -> xcb_randr_list_output_properties_cookie_t {
        sym!(self, xcb_randr_list_output_properties_unchecked)(c, output)
    }

    #[inline]
    pub unsafe fn xcb_randr_query_output_property_valid_values(
        &self,
        R: *const xcb_randr_query_output_property_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_randr_query_output_property_valid_values)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_query_output_property_valid_values_length(
        &self,
        R: *const xcb_randr_query_output_property_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_query_output_property_valid_values_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_query_output_property_valid_values_end(
        &self,
        R: *const xcb_randr_query_output_property_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_query_output_property_valid_values_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_query_output_property_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_query_output_property_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_query_output_property_reply_t {
        sym!(self, xcb_randr_query_output_property_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_randr_query_output_property(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        property: xcb_atom_t,
    ) -> xcb_randr_query_output_property_cookie_t {
        sym!(self, xcb_randr_query_output_property)(c, output, property)
    }

    #[inline]
    pub unsafe fn xcb_randr_query_output_property_unchecked(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        property: xcb_atom_t,
    ) -> xcb_randr_query_output_property_cookie_t {
        sym!(self, xcb_randr_query_output_property_unchecked)(c, output, property)
    }

    #[inline]
    pub unsafe fn xcb_randr_configure_output_property(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        property: xcb_atom_t,
        pending: u8,
        range: u8,
        values_len: u32,
        values: *const i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_configure_output_property)(
            c, output, property, pending, range, values_len, values,
        )
    }

    #[inline]
    pub unsafe fn xcb_randr_configure_output_property_checked(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        property: xcb_atom_t,
        pending: u8,
        range: u8,
        values_len: u32,
        values: *const i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_configure_output_property_checked)(
            c, output, property, pending, range, values_len, values,
        )
    }

    #[inline]
    pub unsafe fn xcb_randr_change_output_property(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        format: u8,
        mode: u8,
        num_units: u32,
        data: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_change_output_property)(
            c, output, property, type_, format, mode, num_units, data,
        )
    }

    #[inline]
    pub unsafe fn xcb_randr_change_output_property_checked(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        format: u8,
        mode: u8,
        num_units: u32,
        data: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_change_output_property_checked)(
            c, output, property, type_, format, mode, num_units, data,
        )
    }

    #[inline]
    pub unsafe fn xcb_randr_delete_output_property(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        property: xcb_atom_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_delete_output_property)(c, output, property)
    }

    #[inline]
    pub unsafe fn xcb_randr_delete_output_property_checked(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        property: xcb_atom_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_delete_output_property_checked)(c, output, property)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_output_property_data(
        &self,
        R: *const xcb_randr_get_output_property_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_randr_get_output_property_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_output_property_data_length(
        &self,
        R: *const xcb_randr_get_output_property_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_output_property_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_output_property_data_end(
        &self,
        R: *const xcb_randr_get_output_property_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_output_property_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_output_property_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_output_property_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_output_property_reply_t {
        sym!(self, xcb_randr_get_output_property_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_output_property(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        long_offset: u32,
        long_length: u32,
        delete: u8,
        pending: u8,
    ) -> xcb_randr_get_output_property_cookie_t {
        sym!(self, xcb_randr_get_output_property)(
            c,
            output,
            property,
            type_,
            long_offset,
            long_length,
            delete,
            pending,
        )
    }

    #[inline]
    pub unsafe fn xcb_randr_get_output_property_unchecked(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        long_offset: u32,
        long_length: u32,
        delete: u8,
        pending: u8,
    ) -> xcb_randr_get_output_property_cookie_t {
        sym!(self, xcb_randr_get_output_property_unchecked)(
            c,
            output,
            property,
            type_,
            long_offset,
            long_length,
            delete,
            pending,
        )
    }

    #[inline]
    pub unsafe fn xcb_randr_create_mode_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_create_mode_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_create_mode_reply_t {
        sym!(self, xcb_randr_create_mode_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_randr_create_mode(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        mode_info: xcb_randr_mode_info_t,
        name_len: u32,
        name: *const c_char,
    ) -> xcb_randr_create_mode_cookie_t {
        sym!(self, xcb_randr_create_mode)(c, window, mode_info, name_len, name)
    }

    #[inline]
    pub unsafe fn xcb_randr_create_mode_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        mode_info: xcb_randr_mode_info_t,
        name_len: u32,
        name: *const c_char,
    ) -> xcb_randr_create_mode_cookie_t {
        sym!(self, xcb_randr_create_mode_unchecked)(c, window, mode_info, name_len, name)
    }

    #[inline]
    pub unsafe fn xcb_randr_destroy_mode(
        &self,
        c: *mut xcb_connection_t,
        mode: xcb_randr_mode_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_destroy_mode)(c, mode)
    }

    #[inline]
    pub unsafe fn xcb_randr_destroy_mode_checked(
        &self,
        c: *mut xcb_connection_t,
        mode: xcb_randr_mode_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_destroy_mode_checked)(c, mode)
    }

    #[inline]
    pub unsafe fn xcb_randr_add_output_mode(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        mode: xcb_randr_mode_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_add_output_mode)(c, output, mode)
    }

    #[inline]
    pub unsafe fn xcb_randr_add_output_mode_checked(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        mode: xcb_randr_mode_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_add_output_mode_checked)(c, output, mode)
    }

    #[inline]
    pub unsafe fn xcb_randr_delete_output_mode(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        mode: xcb_randr_mode_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_delete_output_mode)(c, output, mode)
    }

    #[inline]
    pub unsafe fn xcb_randr_delete_output_mode_checked(
        &self,
        c: *mut xcb_connection_t,
        output: xcb_randr_output_t,
        mode: xcb_randr_mode_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_delete_output_mode_checked)(c, output, mode)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_info_outputs(
        &self,
        R: *const xcb_randr_get_crtc_info_reply_t,
    ) -> *mut xcb_randr_output_t {
        sym!(self, xcb_randr_get_crtc_info_outputs)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_info_outputs_length(
        &self,
        R: *const xcb_randr_get_crtc_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_crtc_info_outputs_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_info_outputs_end(
        &self,
        R: *const xcb_randr_get_crtc_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_crtc_info_outputs_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_info_possible(
        &self,
        R: *const xcb_randr_get_crtc_info_reply_t,
    ) -> *mut xcb_randr_output_t {
        sym!(self, xcb_randr_get_crtc_info_possible)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_info_possible_length(
        &self,
        R: *const xcb_randr_get_crtc_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_crtc_info_possible_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_info_possible_end(
        &self,
        R: *const xcb_randr_get_crtc_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_crtc_info_possible_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_info_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_crtc_info_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_crtc_info_reply_t {
        sym!(self, xcb_randr_get_crtc_info_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_info(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_randr_get_crtc_info_cookie_t {
        sym!(self, xcb_randr_get_crtc_info)(c, crtc, config_timestamp)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_info_unchecked(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_randr_get_crtc_info_cookie_t {
        sym!(self, xcb_randr_get_crtc_info_unchecked)(c, crtc, config_timestamp)
    }

    #[inline]
    pub unsafe fn xcb_randr_set_crtc_config_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_set_crtc_config_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_set_crtc_config_reply_t {
        sym!(self, xcb_randr_set_crtc_config_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_randr_set_crtc_config(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
        timestamp: xcb_timestamp_t,
        config_timestamp: xcb_timestamp_t,
        x: i16,
        y: i16,
        mode: xcb_randr_mode_t,
        rotation: u16,
        outputs_len: u32,
        outputs: *const xcb_randr_output_t,
    ) -> xcb_randr_set_crtc_config_cookie_t {
        sym!(self, xcb_randr_set_crtc_config)(
            c,
            crtc,
            timestamp,
            config_timestamp,
            x,
            y,
            mode,
            rotation,
            outputs_len,
            outputs,
        )
    }

    #[inline]
    pub unsafe fn xcb_randr_set_crtc_config_unchecked(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
        timestamp: xcb_timestamp_t,
        config_timestamp: xcb_timestamp_t,
        x: i16,
        y: i16,
        mode: xcb_randr_mode_t,
        rotation: u16,
        outputs_len: u32,
        outputs: *const xcb_randr_output_t,
    ) -> xcb_randr_set_crtc_config_cookie_t {
        sym!(self, xcb_randr_set_crtc_config_unchecked)(
            c,
            crtc,
            timestamp,
            config_timestamp,
            x,
            y,
            mode,
            rotation,
            outputs_len,
            outputs,
        )
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_size_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_crtc_gamma_size_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_crtc_gamma_size_reply_t {
        sym!(self, xcb_randr_get_crtc_gamma_size_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_size(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
    ) -> xcb_randr_get_crtc_gamma_size_cookie_t {
        sym!(self, xcb_randr_get_crtc_gamma_size)(c, crtc)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_size_unchecked(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
    ) -> xcb_randr_get_crtc_gamma_size_cookie_t {
        sym!(self, xcb_randr_get_crtc_gamma_size_unchecked)(c, crtc)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_red(
        &self,
        R: *const xcb_randr_get_crtc_gamma_reply_t,
    ) -> *mut u16 {
        sym!(self, xcb_randr_get_crtc_gamma_red)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_red_length(
        &self,
        R: *const xcb_randr_get_crtc_gamma_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_crtc_gamma_red_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_red_end(
        &self,
        R: *const xcb_randr_get_crtc_gamma_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_crtc_gamma_red_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_green(
        &self,
        R: *const xcb_randr_get_crtc_gamma_reply_t,
    ) -> *mut u16 {
        sym!(self, xcb_randr_get_crtc_gamma_green)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_green_length(
        &self,
        R: *const xcb_randr_get_crtc_gamma_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_crtc_gamma_green_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_green_end(
        &self,
        R: *const xcb_randr_get_crtc_gamma_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_crtc_gamma_green_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_blue(
        &self,
        R: *const xcb_randr_get_crtc_gamma_reply_t,
    ) -> *mut u16 {
        sym!(self, xcb_randr_get_crtc_gamma_blue)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_blue_length(
        &self,
        R: *const xcb_randr_get_crtc_gamma_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_crtc_gamma_blue_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_blue_end(
        &self,
        R: *const xcb_randr_get_crtc_gamma_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_crtc_gamma_blue_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_crtc_gamma_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_crtc_gamma_reply_t {
        sym!(self, xcb_randr_get_crtc_gamma_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
    ) -> xcb_randr_get_crtc_gamma_cookie_t {
        sym!(self, xcb_randr_get_crtc_gamma)(c, crtc)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_gamma_unchecked(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
    ) -> xcb_randr_get_crtc_gamma_cookie_t {
        sym!(self, xcb_randr_get_crtc_gamma_unchecked)(c, crtc)
    }

    #[inline]
    pub unsafe fn xcb_randr_set_crtc_gamma(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
        size: u16,
        red: *const u16,
        green: *const u16,
        blue: *const u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_crtc_gamma)(c, crtc, size, red, green, blue)
    }

    #[inline]
    pub unsafe fn xcb_randr_set_crtc_gamma_checked(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
        size: u16,
        red: *const u16,
        green: *const u16,
        blue: *const u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_crtc_gamma_checked)(c, crtc, size, red, green, blue)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_crtcs(
        &self,
        R: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> *mut xcb_randr_crtc_t {
        sym!(self, xcb_randr_get_screen_resources_current_crtcs)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_crtcs_length(
        &self,
        R: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_screen_resources_current_crtcs_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_crtcs_end(
        &self,
        R: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_screen_resources_current_crtcs_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_outputs(
        &self,
        R: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> *mut xcb_randr_output_t {
        sym!(self, xcb_randr_get_screen_resources_current_outputs)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_outputs_length(
        &self,
        R: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_screen_resources_current_outputs_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_outputs_end(
        &self,
        R: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_screen_resources_current_outputs_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_modes(
        &self,
        R: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> *mut xcb_randr_mode_info_t {
        sym!(self, xcb_randr_get_screen_resources_current_modes)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_modes_length(
        &self,
        R: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_screen_resources_current_modes_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_modes_iterator(
        &self,
        R: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> xcb_randr_mode_info_iterator_t {
        sym!(self, xcb_randr_get_screen_resources_current_modes_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_names(
        &self,
        R: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_randr_get_screen_resources_current_names)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_names_length(
        &self,
        R: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_screen_resources_current_names_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_names_end(
        &self,
        R: *const xcb_randr_get_screen_resources_current_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_screen_resources_current_names_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_screen_resources_current_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_screen_resources_current_reply_t {
        sym!(self, xcb_randr_get_screen_resources_current_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_screen_resources_current_cookie_t {
        sym!(self, xcb_randr_get_screen_resources_current)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_screen_resources_current_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_screen_resources_current_cookie_t {
        sym!(self, xcb_randr_get_screen_resources_current_unchecked)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_randr_set_crtc_transform(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
        transform: xcb_render_transform_t,
        filter_len: u16,
        filter_name: *const c_char,
        filter_params_len: u32,
        filter_params: *const xcb_render_fixed_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_crtc_transform)(
            c,
            crtc,
            transform,
            filter_len,
            filter_name,
            filter_params_len,
            filter_params,
        )
    }

    #[inline]
    pub unsafe fn xcb_randr_set_crtc_transform_checked(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
        transform: xcb_render_transform_t,
        filter_len: u16,
        filter_name: *const c_char,
        filter_params_len: u32,
        filter_params: *const xcb_render_fixed_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_crtc_transform_checked)(
            c,
            crtc,
            transform,
            filter_len,
            filter_name,
            filter_params_len,
            filter_params,
        )
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_pending_filter_name(
        &self,
        R: *const xcb_randr_get_crtc_transform_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_randr_get_crtc_transform_pending_filter_name)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_pending_filter_name_length(
        &self,
        R: *const xcb_randr_get_crtc_transform_reply_t,
    ) -> c_int {
        sym!(
            self,
            xcb_randr_get_crtc_transform_pending_filter_name_length
        )(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_pending_filter_name_end(
        &self,
        R: *const xcb_randr_get_crtc_transform_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_crtc_transform_pending_filter_name_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_pending_params(
        &self,
        R: *const xcb_randr_get_crtc_transform_reply_t,
    ) -> *mut xcb_render_fixed_t {
        sym!(self, xcb_randr_get_crtc_transform_pending_params)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_pending_params_length(
        &self,
        R: *const xcb_randr_get_crtc_transform_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_crtc_transform_pending_params_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_pending_params_end(
        &self,
        R: *const xcb_randr_get_crtc_transform_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_crtc_transform_pending_params_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_current_filter_name(
        &self,
        R: *const xcb_randr_get_crtc_transform_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_randr_get_crtc_transform_current_filter_name)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_current_filter_name_length(
        &self,
        R: *const xcb_randr_get_crtc_transform_reply_t,
    ) -> c_int {
        sym!(
            self,
            xcb_randr_get_crtc_transform_current_filter_name_length
        )(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_current_filter_name_end(
        &self,
        R: *const xcb_randr_get_crtc_transform_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_crtc_transform_current_filter_name_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_current_params(
        &self,
        R: *const xcb_randr_get_crtc_transform_reply_t,
    ) -> *mut xcb_render_fixed_t {
        sym!(self, xcb_randr_get_crtc_transform_current_params)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_current_params_length(
        &self,
        R: *const xcb_randr_get_crtc_transform_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_crtc_transform_current_params_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_current_params_end(
        &self,
        R: *const xcb_randr_get_crtc_transform_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_crtc_transform_current_params_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_crtc_transform_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_crtc_transform_reply_t {
        sym!(self, xcb_randr_get_crtc_transform_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
    ) -> xcb_randr_get_crtc_transform_cookie_t {
        sym!(self, xcb_randr_get_crtc_transform)(c, crtc)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_crtc_transform_unchecked(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
    ) -> xcb_randr_get_crtc_transform_cookie_t {
        sym!(self, xcb_randr_get_crtc_transform_unchecked)(c, crtc)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_panning_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_panning_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_panning_reply_t {
        sym!(self, xcb_randr_get_panning_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_panning(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
    ) -> xcb_randr_get_panning_cookie_t {
        sym!(self, xcb_randr_get_panning)(c, crtc)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_panning_unchecked(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
    ) -> xcb_randr_get_panning_cookie_t {
        sym!(self, xcb_randr_get_panning_unchecked)(c, crtc)
    }

    #[inline]
    pub unsafe fn xcb_randr_set_panning_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_set_panning_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_set_panning_reply_t {
        sym!(self, xcb_randr_set_panning_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_randr_set_panning(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
        timestamp: xcb_timestamp_t,
        left: u16,
        top: u16,
        width: u16,
        height: u16,
        track_left: u16,
        track_top: u16,
        track_width: u16,
        track_height: u16,
        border_left: i16,
        border_top: i16,
        border_right: i16,
        border_bottom: i16,
    ) -> xcb_randr_set_panning_cookie_t {
        sym!(self, xcb_randr_set_panning)(
            c,
            crtc,
            timestamp,
            left,
            top,
            width,
            height,
            track_left,
            track_top,
            track_width,
            track_height,
            border_left,
            border_top,
            border_right,
            border_bottom,
        )
    }

    #[inline]
    pub unsafe fn xcb_randr_set_panning_unchecked(
        &self,
        c: *mut xcb_connection_t,
        crtc: xcb_randr_crtc_t,
        timestamp: xcb_timestamp_t,
        left: u16,
        top: u16,
        width: u16,
        height: u16,
        track_left: u16,
        track_top: u16,
        track_width: u16,
        track_height: u16,
        border_left: i16,
        border_top: i16,
        border_right: i16,
        border_bottom: i16,
    ) -> xcb_randr_set_panning_cookie_t {
        sym!(self, xcb_randr_set_panning_unchecked)(
            c,
            crtc,
            timestamp,
            left,
            top,
            width,
            height,
            track_left,
            track_top,
            track_width,
            track_height,
            border_left,
            border_top,
            border_right,
            border_bottom,
        )
    }

    #[inline]
    pub unsafe fn xcb_randr_set_output_primary(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        output: xcb_randr_output_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_output_primary)(c, window, output)
    }

    #[inline]
    pub unsafe fn xcb_randr_set_output_primary_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        output: xcb_randr_output_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_output_primary_checked)(c, window, output)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_output_primary_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_output_primary_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_output_primary_reply_t {
        sym!(self, xcb_randr_get_output_primary_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_output_primary(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_output_primary_cookie_t {
        sym!(self, xcb_randr_get_output_primary)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_output_primary_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_output_primary_cookie_t {
        sym!(self, xcb_randr_get_output_primary_unchecked)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_providers_providers(
        &self,
        R: *const xcb_randr_get_providers_reply_t,
    ) -> *mut xcb_randr_provider_t {
        sym!(self, xcb_randr_get_providers_providers)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_providers_providers_length(
        &self,
        R: *const xcb_randr_get_providers_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_providers_providers_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_providers_providers_end(
        &self,
        R: *const xcb_randr_get_providers_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_providers_providers_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_providers_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_providers_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_providers_reply_t {
        sym!(self, xcb_randr_get_providers_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_providers(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_providers_cookie_t {
        sym!(self, xcb_randr_get_providers)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_providers_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_randr_get_providers_cookie_t {
        sym!(self, xcb_randr_get_providers_unchecked)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_crtcs(
        &self,
        R: *const xcb_randr_get_provider_info_reply_t,
    ) -> *mut xcb_randr_crtc_t {
        sym!(self, xcb_randr_get_provider_info_crtcs)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_crtcs_length(
        &self,
        R: *const xcb_randr_get_provider_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_provider_info_crtcs_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_crtcs_end(
        &self,
        R: *const xcb_randr_get_provider_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_provider_info_crtcs_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_outputs(
        &self,
        R: *const xcb_randr_get_provider_info_reply_t,
    ) -> *mut xcb_randr_output_t {
        sym!(self, xcb_randr_get_provider_info_outputs)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_outputs_length(
        &self,
        R: *const xcb_randr_get_provider_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_provider_info_outputs_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_outputs_end(
        &self,
        R: *const xcb_randr_get_provider_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_provider_info_outputs_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_associated_providers(
        &self,
        R: *const xcb_randr_get_provider_info_reply_t,
    ) -> *mut xcb_randr_provider_t {
        sym!(self, xcb_randr_get_provider_info_associated_providers)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_associated_providers_length(
        &self,
        R: *const xcb_randr_get_provider_info_reply_t,
    ) -> c_int {
        sym!(
            self,
            xcb_randr_get_provider_info_associated_providers_length
        )(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_associated_providers_end(
        &self,
        R: *const xcb_randr_get_provider_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_provider_info_associated_providers_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_associated_capability(
        &self,
        R: *const xcb_randr_get_provider_info_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_randr_get_provider_info_associated_capability)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_associated_capability_length(
        &self,
        R: *const xcb_randr_get_provider_info_reply_t,
    ) -> c_int {
        sym!(
            self,
            xcb_randr_get_provider_info_associated_capability_length
        )(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_associated_capability_end(
        &self,
        R: *const xcb_randr_get_provider_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_provider_info_associated_capability_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_name(
        &self,
        R: *const xcb_randr_get_provider_info_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_randr_get_provider_info_name)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_name_length(
        &self,
        R: *const xcb_randr_get_provider_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_provider_info_name_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_name_end(
        &self,
        R: *const xcb_randr_get_provider_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_provider_info_name_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_provider_info_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_provider_info_reply_t {
        sym!(self, xcb_randr_get_provider_info_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_provider_info(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_randr_get_provider_info_cookie_t {
        sym!(self, xcb_randr_get_provider_info)(c, provider, config_timestamp)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_provider_info_unchecked(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_randr_get_provider_info_cookie_t {
        sym!(self, xcb_randr_get_provider_info_unchecked)(c, provider, config_timestamp)
    }

    #[inline]
    pub unsafe fn xcb_randr_set_provider_offload_sink(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        sink_provider: xcb_randr_provider_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_provider_offload_sink)(
            c,
            provider,
            sink_provider,
            config_timestamp,
        )
    }

    #[inline]
    pub unsafe fn xcb_randr_set_provider_offload_sink_checked(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        sink_provider: xcb_randr_provider_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_provider_offload_sink_checked)(
            c,
            provider,
            sink_provider,
            config_timestamp,
        )
    }

    #[inline]
    pub unsafe fn xcb_randr_set_provider_output_source(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        source_provider: xcb_randr_provider_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_provider_output_source)(
            c,
            provider,
            source_provider,
            config_timestamp,
        )
    }

    #[inline]
    pub unsafe fn xcb_randr_set_provider_output_source_checked(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        source_provider: xcb_randr_provider_t,
        config_timestamp: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_set_provider_output_source_checked)(
            c,
            provider,
            source_provider,
            config_timestamp,
        )
    }

    #[inline]
    pub unsafe fn xcb_randr_list_provider_properties_atoms(
        &self,
        R: *const xcb_randr_list_provider_properties_reply_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_randr_list_provider_properties_atoms)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_list_provider_properties_atoms_length(
        &self,
        R: *const xcb_randr_list_provider_properties_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_list_provider_properties_atoms_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_list_provider_properties_atoms_end(
        &self,
        R: *const xcb_randr_list_provider_properties_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_list_provider_properties_atoms_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_list_provider_properties_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_list_provider_properties_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_list_provider_properties_reply_t {
        sym!(self, xcb_randr_list_provider_properties_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_randr_list_provider_properties(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
    ) -> xcb_randr_list_provider_properties_cookie_t {
        sym!(self, xcb_randr_list_provider_properties)(c, provider)
    }

    #[inline]
    pub unsafe fn xcb_randr_list_provider_properties_unchecked(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
    ) -> xcb_randr_list_provider_properties_cookie_t {
        sym!(self, xcb_randr_list_provider_properties_unchecked)(c, provider)
    }

    #[inline]
    pub unsafe fn xcb_randr_query_provider_property_valid_values(
        &self,
        R: *const xcb_randr_query_provider_property_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_randr_query_provider_property_valid_values)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_query_provider_property_valid_values_length(
        &self,
        R: *const xcb_randr_query_provider_property_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_query_provider_property_valid_values_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_query_provider_property_valid_values_end(
        &self,
        R: *const xcb_randr_query_provider_property_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_query_provider_property_valid_values_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_query_provider_property_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_query_provider_property_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_query_provider_property_reply_t {
        sym!(self, xcb_randr_query_provider_property_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_randr_query_provider_property(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        property: xcb_atom_t,
    ) -> xcb_randr_query_provider_property_cookie_t {
        sym!(self, xcb_randr_query_provider_property)(c, provider, property)
    }

    #[inline]
    pub unsafe fn xcb_randr_query_provider_property_unchecked(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        property: xcb_atom_t,
    ) -> xcb_randr_query_provider_property_cookie_t {
        sym!(self, xcb_randr_query_provider_property_unchecked)(c, provider, property)
    }

    #[inline]
    pub unsafe fn xcb_randr_configure_provider_property(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        property: xcb_atom_t,
        pending: u8,
        range: u8,
        values_len: u32,
        values: *const i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_configure_provider_property)(
            c, provider, property, pending, range, values_len, values,
        )
    }

    #[inline]
    pub unsafe fn xcb_randr_configure_provider_property_checked(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        property: xcb_atom_t,
        pending: u8,
        range: u8,
        values_len: u32,
        values: *const i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_configure_provider_property_checked)(
            c, provider, property, pending, range, values_len, values,
        )
    }

    #[inline]
    pub unsafe fn xcb_randr_change_provider_property(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        format: u8,
        mode: u8,
        num_items: u32,
        data: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_change_provider_property)(
            c, provider, property, type_, format, mode, num_items, data,
        )
    }

    #[inline]
    pub unsafe fn xcb_randr_change_provider_property_checked(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        format: u8,
        mode: u8,
        num_items: u32,
        data: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_change_provider_property_checked)(
            c, provider, property, type_, format, mode, num_items, data,
        )
    }

    #[inline]
    pub unsafe fn xcb_randr_delete_provider_property(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        property: xcb_atom_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_delete_provider_property)(c, provider, property)
    }

    #[inline]
    pub unsafe fn xcb_randr_delete_provider_property_checked(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        property: xcb_atom_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_randr_delete_provider_property_checked)(c, provider, property)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_provider_property_data(
        &self,
        R: *const xcb_randr_get_provider_property_reply_t,
    ) -> *mut c_void {
        sym!(self, xcb_randr_get_provider_property_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_provider_property_data_length(
        &self,
        R: *const xcb_randr_get_provider_property_reply_t,
    ) -> c_int {
        sym!(self, xcb_randr_get_provider_property_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_provider_property_data_end(
        &self,
        R: *const xcb_randr_get_provider_property_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_get_provider_property_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_provider_property_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_randr_get_provider_property_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_randr_get_provider_property_reply_t {
        sym!(self, xcb_randr_get_provider_property_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_randr_get_provider_property(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        long_offset: u32,
        long_length: u32,
        delete: u8,
        pending: u8,
    ) -> xcb_randr_get_provider_property_cookie_t {
        sym!(self, xcb_randr_get_provider_property)(
            c,
            provider,
            property,
            type_,
            long_offset,
            long_length,
            delete,
            pending,
        )
    }

    #[inline]
    pub unsafe fn xcb_randr_get_provider_property_unchecked(
        &self,
        c: *mut xcb_connection_t,
        provider: xcb_randr_provider_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        long_offset: u32,
        long_length: u32,
        delete: u8,
        pending: u8,
    ) -> xcb_randr_get_provider_property_cookie_t {
        sym!(self, xcb_randr_get_provider_property_unchecked)(
            c,
            provider,
            property,
            type_,
            long_offset,
            long_length,
            delete,
            pending,
        )
    }

    #[inline]
    pub unsafe fn xcb_randr_crtc_change_next(&self, i: *mut xcb_randr_crtc_change_iterator_t) {
        sym!(self, xcb_randr_crtc_change_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_randr_crtc_change_end(
        &self,
        i: *mut xcb_randr_crtc_change_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_crtc_change_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_randr_output_change_next(&self, i: *mut xcb_randr_output_change_iterator_t) {
        sym!(self, xcb_randr_output_change_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_randr_output_change_end(
        &self,
        i: *mut xcb_randr_output_change_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_output_change_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_randr_output_property_next(
        &self,
        i: *mut xcb_randr_output_property_iterator_t,
    ) {
        sym!(self, xcb_randr_output_property_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_randr_output_property_end(
        &self,
        i: *mut xcb_randr_output_property_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_output_property_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_randr_provider_change_next(
        &self,
        i: *mut xcb_randr_provider_change_iterator_t,
    ) {
        sym!(self, xcb_randr_provider_change_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_randr_provider_change_end(
        &self,
        i: *mut xcb_randr_provider_change_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_provider_change_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_randr_provider_property_next(
        &self,
        i: *mut xcb_randr_provider_property_iterator_t,
    ) {
        sym!(self, xcb_randr_provider_property_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_randr_provider_property_end(
        &self,
        i: *mut xcb_randr_provider_property_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_provider_property_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_randr_resource_change_next(
        &self,
        i: *mut xcb_randr_resource_change_iterator_t,
    ) {
        sym!(self, xcb_randr_resource_change_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_randr_resource_change_end(
        &self,
        i: *mut xcb_randr_resource_change_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_resource_change_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_randr_notify_data_next(&self, i: *mut xcb_randr_notify_data_iterator_t) {
        sym!(self, xcb_randr_notify_data_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_randr_notify_data_end(
        &self,
        i: *mut xcb_randr_notify_data_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_randr_notify_data_end)(i)
    }
}
