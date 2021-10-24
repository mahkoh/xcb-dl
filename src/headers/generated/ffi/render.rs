use crate::*;
use std::os::raw::*;

pub const XCB_RENDER_MAJOR_VERSION: u32 = 0;
pub const XCB_RENDER_MINOR_VERSION: u32 = 11;

pub type xcb_render_pict_type_t = u32;
pub const XCB_RENDER_PICT_TYPE_INDEXED: xcb_render_pict_type_t = 0x00;
pub const XCB_RENDER_PICT_TYPE_DIRECT: xcb_render_pict_type_t = 0x01;

pub type xcb_render_picture_enum_t = u32;
pub const XCB_RENDER_PICTURE_NONE: xcb_render_picture_enum_t = 0x00;

pub type xcb_render_pict_op_t = u32;
pub const XCB_RENDER_PICT_OP_CLEAR: xcb_render_pict_op_t = 0x00;
pub const XCB_RENDER_PICT_OP_SRC: xcb_render_pict_op_t = 0x01;
pub const XCB_RENDER_PICT_OP_DST: xcb_render_pict_op_t = 0x02;
pub const XCB_RENDER_PICT_OP_OVER: xcb_render_pict_op_t = 0x03;
pub const XCB_RENDER_PICT_OP_OVER_REVERSE: xcb_render_pict_op_t = 0x04;
pub const XCB_RENDER_PICT_OP_IN: xcb_render_pict_op_t = 0x05;
pub const XCB_RENDER_PICT_OP_IN_REVERSE: xcb_render_pict_op_t = 0x06;
pub const XCB_RENDER_PICT_OP_OUT: xcb_render_pict_op_t = 0x07;
pub const XCB_RENDER_PICT_OP_OUT_REVERSE: xcb_render_pict_op_t = 0x08;
pub const XCB_RENDER_PICT_OP_ATOP: xcb_render_pict_op_t = 0x09;
pub const XCB_RENDER_PICT_OP_ATOP_REVERSE: xcb_render_pict_op_t = 0x0a;
pub const XCB_RENDER_PICT_OP_XOR: xcb_render_pict_op_t = 0x0b;
pub const XCB_RENDER_PICT_OP_ADD: xcb_render_pict_op_t = 0x0c;
pub const XCB_RENDER_PICT_OP_SATURATE: xcb_render_pict_op_t = 0x0d;
pub const XCB_RENDER_PICT_OP_DISJOINT_CLEAR: xcb_render_pict_op_t = 0x10;
pub const XCB_RENDER_PICT_OP_DISJOINT_SRC: xcb_render_pict_op_t = 0x11;
pub const XCB_RENDER_PICT_OP_DISJOINT_DST: xcb_render_pict_op_t = 0x12;
pub const XCB_RENDER_PICT_OP_DISJOINT_OVER: xcb_render_pict_op_t = 0x13;
pub const XCB_RENDER_PICT_OP_DISJOINT_OVER_REVERSE: xcb_render_pict_op_t = 0x14;
pub const XCB_RENDER_PICT_OP_DISJOINT_IN: xcb_render_pict_op_t = 0x15;
pub const XCB_RENDER_PICT_OP_DISJOINT_IN_REVERSE: xcb_render_pict_op_t = 0x16;
pub const XCB_RENDER_PICT_OP_DISJOINT_OUT: xcb_render_pict_op_t = 0x17;
pub const XCB_RENDER_PICT_OP_DISJOINT_OUT_REVERSE: xcb_render_pict_op_t = 0x18;
pub const XCB_RENDER_PICT_OP_DISJOINT_ATOP: xcb_render_pict_op_t = 0x19;
pub const XCB_RENDER_PICT_OP_DISJOINT_ATOP_REVERSE: xcb_render_pict_op_t = 0x1a;
pub const XCB_RENDER_PICT_OP_DISJOINT_XOR: xcb_render_pict_op_t = 0x1b;
pub const XCB_RENDER_PICT_OP_CONJOINT_CLEAR: xcb_render_pict_op_t = 0x20;
pub const XCB_RENDER_PICT_OP_CONJOINT_SRC: xcb_render_pict_op_t = 0x21;
pub const XCB_RENDER_PICT_OP_CONJOINT_DST: xcb_render_pict_op_t = 0x22;
pub const XCB_RENDER_PICT_OP_CONJOINT_OVER: xcb_render_pict_op_t = 0x23;
pub const XCB_RENDER_PICT_OP_CONJOINT_OVER_REVERSE: xcb_render_pict_op_t = 0x24;
pub const XCB_RENDER_PICT_OP_CONJOINT_IN: xcb_render_pict_op_t = 0x25;
pub const XCB_RENDER_PICT_OP_CONJOINT_IN_REVERSE: xcb_render_pict_op_t = 0x26;
pub const XCB_RENDER_PICT_OP_CONJOINT_OUT: xcb_render_pict_op_t = 0x27;
pub const XCB_RENDER_PICT_OP_CONJOINT_OUT_REVERSE: xcb_render_pict_op_t = 0x28;
pub const XCB_RENDER_PICT_OP_CONJOINT_ATOP: xcb_render_pict_op_t = 0x29;
pub const XCB_RENDER_PICT_OP_CONJOINT_ATOP_REVERSE: xcb_render_pict_op_t = 0x2a;
pub const XCB_RENDER_PICT_OP_CONJOINT_XOR: xcb_render_pict_op_t = 0x2b;
pub const XCB_RENDER_PICT_OP_MULTIPLY: xcb_render_pict_op_t = 0x30;
pub const XCB_RENDER_PICT_OP_SCREEN: xcb_render_pict_op_t = 0x31;
pub const XCB_RENDER_PICT_OP_OVERLAY: xcb_render_pict_op_t = 0x32;
pub const XCB_RENDER_PICT_OP_DARKEN: xcb_render_pict_op_t = 0x33;
pub const XCB_RENDER_PICT_OP_LIGHTEN: xcb_render_pict_op_t = 0x34;
pub const XCB_RENDER_PICT_OP_COLOR_DODGE: xcb_render_pict_op_t = 0x35;
pub const XCB_RENDER_PICT_OP_COLOR_BURN: xcb_render_pict_op_t = 0x36;
pub const XCB_RENDER_PICT_OP_HARD_LIGHT: xcb_render_pict_op_t = 0x37;
pub const XCB_RENDER_PICT_OP_SOFT_LIGHT: xcb_render_pict_op_t = 0x38;
pub const XCB_RENDER_PICT_OP_DIFFERENCE: xcb_render_pict_op_t = 0x39;
pub const XCB_RENDER_PICT_OP_EXCLUSION: xcb_render_pict_op_t = 0x3a;
pub const XCB_RENDER_PICT_OP_HSL_HUE: xcb_render_pict_op_t = 0x3b;
pub const XCB_RENDER_PICT_OP_HSL_SATURATION: xcb_render_pict_op_t = 0x3c;
pub const XCB_RENDER_PICT_OP_HSL_COLOR: xcb_render_pict_op_t = 0x3d;
pub const XCB_RENDER_PICT_OP_HSL_LUMINOSITY: xcb_render_pict_op_t = 0x3e;

pub type xcb_render_poly_edge_t = u32;
pub const XCB_RENDER_POLY_EDGE_SHARP: xcb_render_poly_edge_t = 0x00;
pub const XCB_RENDER_POLY_EDGE_SMOOTH: xcb_render_poly_edge_t = 0x01;

pub type xcb_render_poly_mode_t = u32;
pub const XCB_RENDER_POLY_MODE_PRECISE: xcb_render_poly_mode_t = 0x00;
pub const XCB_RENDER_POLY_MODE_IMPRECISE: xcb_render_poly_mode_t = 0x01;

pub type xcb_render_cp_t = u32;
pub const XCB_RENDER_CP_REPEAT: xcb_render_cp_t = 0x01;
pub const XCB_RENDER_CP_ALPHA_MAP: xcb_render_cp_t = 0x02;
pub const XCB_RENDER_CP_ALPHA_X_ORIGIN: xcb_render_cp_t = 0x04;
pub const XCB_RENDER_CP_ALPHA_Y_ORIGIN: xcb_render_cp_t = 0x08;
pub const XCB_RENDER_CP_CLIP_X_ORIGIN: xcb_render_cp_t = 0x10;
pub const XCB_RENDER_CP_CLIP_Y_ORIGIN: xcb_render_cp_t = 0x20;
pub const XCB_RENDER_CP_CLIP_MASK: xcb_render_cp_t = 0x40;
pub const XCB_RENDER_CP_GRAPHICS_EXPOSURE: xcb_render_cp_t = 0x80;
pub const XCB_RENDER_CP_SUBWINDOW_MODE: xcb_render_cp_t = 0x100;
pub const XCB_RENDER_CP_POLY_EDGE: xcb_render_cp_t = 0x200;
pub const XCB_RENDER_CP_POLY_MODE: xcb_render_cp_t = 0x400;
pub const XCB_RENDER_CP_DITHER: xcb_render_cp_t = 0x800;
pub const XCB_RENDER_CP_COMPONENT_ALPHA: xcb_render_cp_t = 0x1000;

pub type xcb_render_sub_pixel_t = u32;
pub const XCB_RENDER_SUB_PIXEL_UNKNOWN: xcb_render_sub_pixel_t = 0x00;
pub const XCB_RENDER_SUB_PIXEL_HORIZONTAL_RGB: xcb_render_sub_pixel_t = 0x01;
pub const XCB_RENDER_SUB_PIXEL_HORIZONTAL_BGR: xcb_render_sub_pixel_t = 0x02;
pub const XCB_RENDER_SUB_PIXEL_VERTICAL_RGB: xcb_render_sub_pixel_t = 0x03;
pub const XCB_RENDER_SUB_PIXEL_VERTICAL_BGR: xcb_render_sub_pixel_t = 0x04;
pub const XCB_RENDER_SUB_PIXEL_NONE: xcb_render_sub_pixel_t = 0x05;

pub type xcb_render_repeat_t = u32;
pub const XCB_RENDER_REPEAT_NONE: xcb_render_repeat_t = 0x00;
pub const XCB_RENDER_REPEAT_NORMAL: xcb_render_repeat_t = 0x01;
pub const XCB_RENDER_REPEAT_PAD: xcb_render_repeat_t = 0x02;
pub const XCB_RENDER_REPEAT_REFLECT: xcb_render_repeat_t = 0x03;

pub type xcb_render_glyph_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_glyph_iterator_t {
    pub data: *mut xcb_render_glyph_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_render_glyphset_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_glyphset_iterator_t {
    pub data: *mut xcb_render_glyphset_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_render_picture_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_picture_iterator_t {
    pub data: *mut xcb_render_picture_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_render_pictformat_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pictformat_iterator_t {
    pub data: *mut xcb_render_pictformat_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_render_fixed_t = i32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_fixed_iterator_t {
    pub data: *mut xcb_render_fixed_t,
    pub rem: c_int,
    pub index: c_int,
}

pub const XCB_RENDER_PICT_FORMAT: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pict_format_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

pub const XCB_RENDER_PICTURE: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_picture_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

pub const XCB_RENDER_PICT_OP: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pict_op_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

pub const XCB_RENDER_GLYPH_SET: u8 = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_glyph_set_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

pub const XCB_RENDER_GLYPH: u8 = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_glyph_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_directformat_t {
    pub red_shift: u16,
    pub red_mask: u16,
    pub green_shift: u16,
    pub green_mask: u16,
    pub blue_shift: u16,
    pub blue_mask: u16,
    pub alpha_shift: u16,
    pub alpha_mask: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_directformat_iterator_t {
    pub data: *mut xcb_render_directformat_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pictforminfo_t {
    pub id: xcb_render_pictformat_t,
    pub type_: u8,
    pub depth: u8,
    pub pad0: [u8; 2],
    pub direct: xcb_render_directformat_t,
    pub colormap: xcb_colormap_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pictforminfo_iterator_t {
    pub data: *mut xcb_render_pictforminfo_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pictvisual_t {
    pub visual: xcb_visualid_t,
    pub format: xcb_render_pictformat_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pictvisual_iterator_t {
    pub data: *mut xcb_render_pictvisual_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pictdepth_t {
    pub depth: u8,
    pub pad0: u8,
    pub num_visuals: u16,
    pub pad1: [u8; 4],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pictdepth_iterator_t {
    pub data: *mut xcb_render_pictdepth_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pictscreen_t {
    pub num_depths: u32,
    pub fallback: xcb_render_pictformat_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pictscreen_iterator_t {
    pub data: *mut xcb_render_pictscreen_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_indexvalue_t {
    pub pixel: u32,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub alpha: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_indexvalue_iterator_t {
    pub data: *mut xcb_render_indexvalue_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_color_t {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub alpha: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_color_iterator_t {
    pub data: *mut xcb_render_color_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pointfix_t {
    pub x: xcb_render_fixed_t,
    pub y: xcb_render_fixed_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pointfix_iterator_t {
    pub data: *mut xcb_render_pointfix_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_linefix_t {
    pub p1: xcb_render_pointfix_t,
    pub p2: xcb_render_pointfix_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_linefix_iterator_t {
    pub data: *mut xcb_render_linefix_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_triangle_t {
    pub p1: xcb_render_pointfix_t,
    pub p2: xcb_render_pointfix_t,
    pub p3: xcb_render_pointfix_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_triangle_iterator_t {
    pub data: *mut xcb_render_triangle_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_trapezoid_t {
    pub top: xcb_render_fixed_t,
    pub bottom: xcb_render_fixed_t,
    pub left: xcb_render_linefix_t,
    pub right: xcb_render_linefix_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_trapezoid_iterator_t {
    pub data: *mut xcb_render_trapezoid_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_glyphinfo_t {
    pub width: u16,
    pub height: u16,
    pub x: i16,
    pub y: i16,
    pub x_off: i16,
    pub y_off: i16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_glyphinfo_iterator_t {
    pub data: *mut xcb_render_glyphinfo_t,
    pub rem: c_int,
    pub index: c_int,
}

pub const XCB_RENDER_QUERY_VERSION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client_major_version: u32,
    pub client_minor_version: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_query_version_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
    pub pad1: [u8; 16],
}

pub const XCB_RENDER_QUERY_PICT_FORMATS: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_query_pict_formats_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_query_pict_formats_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_query_pict_formats_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_formats: u32,
    pub num_screens: u32,
    pub num_depths: u32,
    pub num_visuals: u32,
    pub num_subpixel: u32,
    pub pad1: [u8; 4],
}

pub const XCB_RENDER_QUERY_PICT_INDEX_VALUES: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_query_pict_index_values_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub format: xcb_render_pictformat_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_query_pict_index_values_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_query_pict_index_values_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_values: u32,
    pub pad1: [u8; 20],
}

pub const XCB_RENDER_CREATE_PICTURE: u8 = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_create_picture_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub pid: xcb_render_picture_t,
    pub drawable: xcb_drawable_t,
    pub format: xcb_render_pictformat_t,
    pub value_mask: u32,
}

pub const XCB_RENDER_CHANGE_PICTURE: u8 = 5;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_change_picture_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub picture: xcb_render_picture_t,
    pub value_mask: u32,
}

pub const XCB_RENDER_SET_PICTURE_CLIP_RECTANGLES: u8 = 6;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_set_picture_clip_rectangles_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub picture: xcb_render_picture_t,
    pub clip_x_origin: i16,
    pub clip_y_origin: i16,
}

pub const XCB_RENDER_FREE_PICTURE: u8 = 7;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_free_picture_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub picture: xcb_render_picture_t,
}

pub const XCB_RENDER_COMPOSITE: u8 = 8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_composite_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub op: u8,
    pub pad0: [u8; 3],
    pub src: xcb_render_picture_t,
    pub mask: xcb_render_picture_t,
    pub dst: xcb_render_picture_t,
    pub src_x: i16,
    pub src_y: i16,
    pub mask_x: i16,
    pub mask_y: i16,
    pub dst_x: i16,
    pub dst_y: i16,
    pub width: u16,
    pub height: u16,
}

pub const XCB_RENDER_TRAPEZOIDS: u8 = 10;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_trapezoids_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub op: u8,
    pub pad0: [u8; 3],
    pub src: xcb_render_picture_t,
    pub dst: xcb_render_picture_t,
    pub mask_format: xcb_render_pictformat_t,
    pub src_x: i16,
    pub src_y: i16,
}

pub const XCB_RENDER_TRIANGLES: u8 = 11;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_triangles_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub op: u8,
    pub pad0: [u8; 3],
    pub src: xcb_render_picture_t,
    pub dst: xcb_render_picture_t,
    pub mask_format: xcb_render_pictformat_t,
    pub src_x: i16,
    pub src_y: i16,
}

pub const XCB_RENDER_TRI_STRIP: u8 = 12;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_tri_strip_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub op: u8,
    pub pad0: [u8; 3],
    pub src: xcb_render_picture_t,
    pub dst: xcb_render_picture_t,
    pub mask_format: xcb_render_pictformat_t,
    pub src_x: i16,
    pub src_y: i16,
}

pub const XCB_RENDER_TRI_FAN: u8 = 13;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_tri_fan_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub op: u8,
    pub pad0: [u8; 3],
    pub src: xcb_render_picture_t,
    pub dst: xcb_render_picture_t,
    pub mask_format: xcb_render_pictformat_t,
    pub src_x: i16,
    pub src_y: i16,
}

pub const XCB_RENDER_CREATE_GLYPH_SET: u8 = 17;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_create_glyph_set_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub gsid: xcb_render_glyphset_t,
    pub format: xcb_render_pictformat_t,
}

pub const XCB_RENDER_REFERENCE_GLYPH_SET: u8 = 18;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_reference_glyph_set_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub gsid: xcb_render_glyphset_t,
    pub existing: xcb_render_glyphset_t,
}

pub const XCB_RENDER_FREE_GLYPH_SET: u8 = 19;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_free_glyph_set_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub glyphset: xcb_render_glyphset_t,
}

pub const XCB_RENDER_ADD_GLYPHS: u8 = 20;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_add_glyphs_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub glyphset: xcb_render_glyphset_t,
    pub glyphs_len: u32,
}

pub const XCB_RENDER_FREE_GLYPHS: u8 = 22;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_free_glyphs_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub glyphset: xcb_render_glyphset_t,
}

pub const XCB_RENDER_COMPOSITE_GLYPHS_8: u8 = 23;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_composite_glyphs_8_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub op: u8,
    pub pad0: [u8; 3],
    pub src: xcb_render_picture_t,
    pub dst: xcb_render_picture_t,
    pub mask_format: xcb_render_pictformat_t,
    pub glyphset: xcb_render_glyphset_t,
    pub src_x: i16,
    pub src_y: i16,
}

pub const XCB_RENDER_COMPOSITE_GLYPHS_16: u8 = 24;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_composite_glyphs_16_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub op: u8,
    pub pad0: [u8; 3],
    pub src: xcb_render_picture_t,
    pub dst: xcb_render_picture_t,
    pub mask_format: xcb_render_pictformat_t,
    pub glyphset: xcb_render_glyphset_t,
    pub src_x: i16,
    pub src_y: i16,
}

pub const XCB_RENDER_COMPOSITE_GLYPHS_32: u8 = 25;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_composite_glyphs_32_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub op: u8,
    pub pad0: [u8; 3],
    pub src: xcb_render_picture_t,
    pub dst: xcb_render_picture_t,
    pub mask_format: xcb_render_pictformat_t,
    pub glyphset: xcb_render_glyphset_t,
    pub src_x: i16,
    pub src_y: i16,
}

pub const XCB_RENDER_FILL_RECTANGLES: u8 = 26;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_fill_rectangles_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub op: u8,
    pub pad0: [u8; 3],
    pub dst: xcb_render_picture_t,
    pub color: xcb_render_color_t,
}

pub const XCB_RENDER_CREATE_CURSOR: u8 = 27;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_create_cursor_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub cid: xcb_cursor_t,
    pub source: xcb_render_picture_t,
    pub x: u16,
    pub y: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_transform_t {
    pub matrix11: xcb_render_fixed_t,
    pub matrix12: xcb_render_fixed_t,
    pub matrix13: xcb_render_fixed_t,
    pub matrix21: xcb_render_fixed_t,
    pub matrix22: xcb_render_fixed_t,
    pub matrix23: xcb_render_fixed_t,
    pub matrix31: xcb_render_fixed_t,
    pub matrix32: xcb_render_fixed_t,
    pub matrix33: xcb_render_fixed_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_transform_iterator_t {
    pub data: *mut xcb_render_transform_t,
    pub rem: c_int,
    pub index: c_int,
}

pub const XCB_RENDER_SET_PICTURE_TRANSFORM: u8 = 28;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_set_picture_transform_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub picture: xcb_render_picture_t,
    pub transform: xcb_render_transform_t,
}

pub const XCB_RENDER_QUERY_FILTERS: u8 = 29;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_query_filters_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_query_filters_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_query_filters_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_aliases: u32,
    pub num_filters: u32,
    pub pad1: [u8; 16],
}

pub const XCB_RENDER_SET_PICTURE_FILTER: u8 = 30;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_set_picture_filter_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub picture: xcb_render_picture_t,
    pub filter_len: u16,
    pub pad0: [u8; 2],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_animcursorelt_t {
    pub cursor: xcb_cursor_t,
    pub delay: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_animcursorelt_iterator_t {
    pub data: *mut xcb_render_animcursorelt_t,
    pub rem: c_int,
    pub index: c_int,
}

pub const XCB_RENDER_CREATE_ANIM_CURSOR: u8 = 31;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_create_anim_cursor_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub cid: xcb_cursor_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_spanfix_t {
    pub l: xcb_render_fixed_t,
    pub r: xcb_render_fixed_t,
    pub y: xcb_render_fixed_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_spanfix_iterator_t {
    pub data: *mut xcb_render_spanfix_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_trap_t {
    pub top: xcb_render_spanfix_t,
    pub bot: xcb_render_spanfix_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_trap_iterator_t {
    pub data: *mut xcb_render_trap_t,
    pub rem: c_int,
    pub index: c_int,
}

pub const XCB_RENDER_ADD_TRAPS: u8 = 32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_add_traps_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub picture: xcb_render_picture_t,
    pub x_off: i16,
    pub y_off: i16,
}

pub const XCB_RENDER_CREATE_SOLID_FILL: u8 = 33;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_create_solid_fill_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub picture: xcb_render_picture_t,
    pub color: xcb_render_color_t,
}

pub const XCB_RENDER_CREATE_LINEAR_GRADIENT: u8 = 34;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_create_linear_gradient_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub picture: xcb_render_picture_t,
    pub p1: xcb_render_pointfix_t,
    pub p2: xcb_render_pointfix_t,
    pub num_stops: u32,
}

pub const XCB_RENDER_CREATE_RADIAL_GRADIENT: u8 = 35;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_create_radial_gradient_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub picture: xcb_render_picture_t,
    pub inner: xcb_render_pointfix_t,
    pub outer: xcb_render_pointfix_t,
    pub inner_radius: xcb_render_fixed_t,
    pub outer_radius: xcb_render_fixed_t,
    pub num_stops: u32,
}

pub const XCB_RENDER_CREATE_CONICAL_GRADIENT: u8 = 36;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_create_conical_gradient_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub picture: xcb_render_picture_t,
    pub center: xcb_render_pointfix_t,
    pub angle: xcb_render_fixed_t,
    pub num_stops: u32,
}

impl XcbRender {
    #[inline]
    pub fn xcb_render_id(&self) -> *mut xcb_extension_t {
        call!(self, xcb_render_id)
    }

    #[inline]
    pub unsafe fn xcb_render_glyph_next(&self, i: *mut xcb_render_glyph_iterator_t) {
        call!(self, xcb_render_glyph_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_render_glyph_end(
        &self,
        i: *mut xcb_render_glyph_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_render_glyph_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_render_glyphset_next(&self, i: *mut xcb_render_glyphset_iterator_t) {
        call!(self, xcb_render_glyphset_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_render_glyphset_end(
        &self,
        i: *mut xcb_render_glyphset_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_render_glyphset_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_render_picture_next(&self, i: *mut xcb_render_picture_iterator_t) {
        call!(self, xcb_render_picture_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_render_picture_end(
        &self,
        i: *mut xcb_render_picture_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_render_picture_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_render_pictformat_next(&self, i: *mut xcb_render_pictformat_iterator_t) {
        call!(self, xcb_render_pictformat_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_render_pictformat_end(
        &self,
        i: *mut xcb_render_pictformat_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_render_pictformat_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_render_fixed_next(&self, i: *mut xcb_render_fixed_iterator_t) {
        call!(self, xcb_render_fixed_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_render_fixed_end(
        &self,
        i: *mut xcb_render_fixed_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_render_fixed_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_render_directformat_next(&self, i: *mut xcb_render_directformat_iterator_t) {
        call!(self, xcb_render_directformat_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_render_directformat_end(
        &self,
        i: *mut xcb_render_directformat_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_render_directformat_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_render_pictforminfo_next(&self, i: *mut xcb_render_pictforminfo_iterator_t) {
        call!(self, xcb_render_pictforminfo_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_render_pictforminfo_end(
        &self,
        i: *mut xcb_render_pictforminfo_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_render_pictforminfo_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_render_pictvisual_next(&self, i: *mut xcb_render_pictvisual_iterator_t) {
        call!(self, xcb_render_pictvisual_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_render_pictvisual_end(
        &self,
        i: *mut xcb_render_pictvisual_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_render_pictvisual_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_render_pictdepth_visuals(
        &self,
        R: *const xcb_render_pictdepth_t,
    ) -> *mut xcb_render_pictvisual_t {
        call!(self, xcb_render_pictdepth_visuals)(R)
    }

    #[inline]
    pub unsafe fn xcb_render_pictdepth_visuals_length(
        &self,
        R: *const xcb_render_pictdepth_t,
    ) -> c_int {
        call!(self, xcb_render_pictdepth_visuals_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_render_pictdepth_visuals_iterator(
        &self,
        R: *const xcb_render_pictdepth_t,
    ) -> xcb_render_pictvisual_iterator_t {
        call!(self, xcb_render_pictdepth_visuals_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_render_pictdepth_next(&self, i: *mut xcb_render_pictdepth_iterator_t) {
        call!(self, xcb_render_pictdepth_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_render_pictdepth_end(
        &self,
        i: *mut xcb_render_pictdepth_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_render_pictdepth_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_render_pictscreen_depths_length(
        &self,
        R: *const xcb_render_pictscreen_t,
    ) -> c_int {
        call!(self, xcb_render_pictscreen_depths_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_render_pictscreen_depths_iterator(
        &self,
        R: *const xcb_render_pictscreen_t,
    ) -> xcb_render_pictdepth_iterator_t {
        call!(self, xcb_render_pictscreen_depths_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_render_pictscreen_next(&self, i: *mut xcb_render_pictscreen_iterator_t) {
        call!(self, xcb_render_pictscreen_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_render_pictscreen_end(
        &self,
        i: *mut xcb_render_pictscreen_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_render_pictscreen_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_render_indexvalue_next(&self, i: *mut xcb_render_indexvalue_iterator_t) {
        call!(self, xcb_render_indexvalue_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_render_indexvalue_end(
        &self,
        i: *mut xcb_render_indexvalue_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_render_indexvalue_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_render_color_next(&self, i: *mut xcb_render_color_iterator_t) {
        call!(self, xcb_render_color_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_render_color_end(
        &self,
        i: *mut xcb_render_color_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_render_color_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_render_pointfix_next(&self, i: *mut xcb_render_pointfix_iterator_t) {
        call!(self, xcb_render_pointfix_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_render_pointfix_end(
        &self,
        i: *mut xcb_render_pointfix_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_render_pointfix_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_render_linefix_next(&self, i: *mut xcb_render_linefix_iterator_t) {
        call!(self, xcb_render_linefix_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_render_linefix_end(
        &self,
        i: *mut xcb_render_linefix_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_render_linefix_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_render_triangle_next(&self, i: *mut xcb_render_triangle_iterator_t) {
        call!(self, xcb_render_triangle_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_render_triangle_end(
        &self,
        i: *mut xcb_render_triangle_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_render_triangle_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_render_trapezoid_next(&self, i: *mut xcb_render_trapezoid_iterator_t) {
        call!(self, xcb_render_trapezoid_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_render_trapezoid_end(
        &self,
        i: *mut xcb_render_trapezoid_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_render_trapezoid_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_render_glyphinfo_next(&self, i: *mut xcb_render_glyphinfo_iterator_t) {
        call!(self, xcb_render_glyphinfo_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_render_glyphinfo_end(
        &self,
        i: *mut xcb_render_glyphinfo_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_render_glyphinfo_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_render_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_render_query_version_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_render_query_version_reply_t {
        call!(self, xcb_render_query_version_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_render_query_version(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u32,
        client_minor_version: u32,
    ) -> xcb_render_query_version_cookie_t {
        call!(self, xcb_render_query_version)(c, client_major_version, client_minor_version)
    }

    #[inline]
    pub unsafe fn xcb_render_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u32,
        client_minor_version: u32,
    ) -> xcb_render_query_version_cookie_t {
        call!(self, xcb_render_query_version_unchecked)(
            c,
            client_major_version,
            client_minor_version,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_query_pict_formats_formats(
        &self,
        R: *const xcb_render_query_pict_formats_reply_t,
    ) -> *mut xcb_render_pictforminfo_t {
        call!(self, xcb_render_query_pict_formats_formats)(R)
    }

    #[inline]
    pub unsafe fn xcb_render_query_pict_formats_formats_length(
        &self,
        R: *const xcb_render_query_pict_formats_reply_t,
    ) -> c_int {
        call!(self, xcb_render_query_pict_formats_formats_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_render_query_pict_formats_formats_iterator(
        &self,
        R: *const xcb_render_query_pict_formats_reply_t,
    ) -> xcb_render_pictforminfo_iterator_t {
        call!(self, xcb_render_query_pict_formats_formats_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_render_query_pict_formats_screens_length(
        &self,
        R: *const xcb_render_query_pict_formats_reply_t,
    ) -> c_int {
        call!(self, xcb_render_query_pict_formats_screens_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_render_query_pict_formats_screens_iterator(
        &self,
        R: *const xcb_render_query_pict_formats_reply_t,
    ) -> xcb_render_pictscreen_iterator_t {
        call!(self, xcb_render_query_pict_formats_screens_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_render_query_pict_formats_subpixels(
        &self,
        R: *const xcb_render_query_pict_formats_reply_t,
    ) -> *mut u32 {
        call!(self, xcb_render_query_pict_formats_subpixels)(R)
    }

    #[inline]
    pub unsafe fn xcb_render_query_pict_formats_subpixels_length(
        &self,
        R: *const xcb_render_query_pict_formats_reply_t,
    ) -> c_int {
        call!(self, xcb_render_query_pict_formats_subpixels_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_render_query_pict_formats_subpixels_end(
        &self,
        R: *const xcb_render_query_pict_formats_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_render_query_pict_formats_subpixels_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_render_query_pict_formats_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_render_query_pict_formats_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_render_query_pict_formats_reply_t {
        call!(self, xcb_render_query_pict_formats_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_render_query_pict_formats(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_render_query_pict_formats_cookie_t {
        call!(self, xcb_render_query_pict_formats)(c)
    }

    #[inline]
    pub unsafe fn xcb_render_query_pict_formats_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_render_query_pict_formats_cookie_t {
        call!(self, xcb_render_query_pict_formats_unchecked)(c)
    }

    #[inline]
    pub unsafe fn xcb_render_query_pict_index_values_values(
        &self,
        R: *const xcb_render_query_pict_index_values_reply_t,
    ) -> *mut xcb_render_indexvalue_t {
        call!(self, xcb_render_query_pict_index_values_values)(R)
    }

    #[inline]
    pub unsafe fn xcb_render_query_pict_index_values_values_length(
        &self,
        R: *const xcb_render_query_pict_index_values_reply_t,
    ) -> c_int {
        call!(self, xcb_render_query_pict_index_values_values_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_render_query_pict_index_values_values_iterator(
        &self,
        R: *const xcb_render_query_pict_index_values_reply_t,
    ) -> xcb_render_indexvalue_iterator_t {
        call!(self, xcb_render_query_pict_index_values_values_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_render_query_pict_index_values_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_render_query_pict_index_values_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_render_query_pict_index_values_reply_t {
        call!(self, xcb_render_query_pict_index_values_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_render_query_pict_index_values(
        &self,
        c: *mut xcb_connection_t,
        format: xcb_render_pictformat_t,
    ) -> xcb_render_query_pict_index_values_cookie_t {
        call!(self, xcb_render_query_pict_index_values)(c, format)
    }

    #[inline]
    pub unsafe fn xcb_render_query_pict_index_values_unchecked(
        &self,
        c: *mut xcb_connection_t,
        format: xcb_render_pictformat_t,
    ) -> xcb_render_query_pict_index_values_cookie_t {
        call!(self, xcb_render_query_pict_index_values_unchecked)(c, format)
    }

    #[inline]
    pub unsafe fn xcb_render_create_picture(
        &self,
        c: *mut xcb_connection_t,
        pid: xcb_render_picture_t,
        drawable: xcb_drawable_t,
        format: xcb_render_pictformat_t,
        value_mask: u32,
        value_list: *const u32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_create_picture)(c, pid, drawable, format, value_mask, value_list)
    }

    #[inline]
    pub unsafe fn xcb_render_create_picture_checked(
        &self,
        c: *mut xcb_connection_t,
        pid: xcb_render_picture_t,
        drawable: xcb_drawable_t,
        format: xcb_render_pictformat_t,
        value_mask: u32,
        value_list: *const u32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_create_picture_checked)(
            c, pid, drawable, format, value_mask, value_list,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_change_picture(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        value_mask: u32,
        value_list: *const u32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_change_picture)(c, picture, value_mask, value_list)
    }

    #[inline]
    pub unsafe fn xcb_render_change_picture_checked(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        value_mask: u32,
        value_list: *const u32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_change_picture_checked)(c, picture, value_mask, value_list)
    }

    #[inline]
    pub unsafe fn xcb_render_set_picture_clip_rectangles(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        clip_x_origin: i16,
        clip_y_origin: i16,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_set_picture_clip_rectangles)(
            c,
            picture,
            clip_x_origin,
            clip_y_origin,
            rectangles_len,
            rectangles,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_set_picture_clip_rectangles_checked(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        clip_x_origin: i16,
        clip_y_origin: i16,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_set_picture_clip_rectangles_checked)(
            c,
            picture,
            clip_x_origin,
            clip_y_origin,
            rectangles_len,
            rectangles,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_free_picture(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_free_picture)(c, picture)
    }

    #[inline]
    pub unsafe fn xcb_render_free_picture_checked(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_free_picture_checked)(c, picture)
    }

    #[inline]
    pub unsafe fn xcb_render_composite(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        mask: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        src_x: i16,
        src_y: i16,
        mask_x: i16,
        mask_y: i16,
        dst_x: i16,
        dst_y: i16,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_composite)(
            c, op, src, mask, dst, src_x, src_y, mask_x, mask_y, dst_x, dst_y, width, height,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_composite_checked(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        mask: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        src_x: i16,
        src_y: i16,
        mask_x: i16,
        mask_y: i16,
        dst_x: i16,
        dst_y: i16,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_composite_checked)(
            c, op, src, mask, dst, src_x, src_y, mask_x, mask_y, dst_x, dst_y, width, height,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_trapezoids(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        src_x: i16,
        src_y: i16,
        traps_len: u32,
        traps: *const xcb_render_trapezoid_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_trapezoids)(
            c,
            op,
            src,
            dst,
            mask_format,
            src_x,
            src_y,
            traps_len,
            traps,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_trapezoids_checked(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        src_x: i16,
        src_y: i16,
        traps_len: u32,
        traps: *const xcb_render_trapezoid_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_trapezoids_checked)(
            c,
            op,
            src,
            dst,
            mask_format,
            src_x,
            src_y,
            traps_len,
            traps,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_triangles(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        src_x: i16,
        src_y: i16,
        triangles_len: u32,
        triangles: *const xcb_render_triangle_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_triangles)(
            c,
            op,
            src,
            dst,
            mask_format,
            src_x,
            src_y,
            triangles_len,
            triangles,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_triangles_checked(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        src_x: i16,
        src_y: i16,
        triangles_len: u32,
        triangles: *const xcb_render_triangle_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_triangles_checked)(
            c,
            op,
            src,
            dst,
            mask_format,
            src_x,
            src_y,
            triangles_len,
            triangles,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_tri_strip(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        src_x: i16,
        src_y: i16,
        points_len: u32,
        points: *const xcb_render_pointfix_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_tri_strip)(
            c,
            op,
            src,
            dst,
            mask_format,
            src_x,
            src_y,
            points_len,
            points,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_tri_strip_checked(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        src_x: i16,
        src_y: i16,
        points_len: u32,
        points: *const xcb_render_pointfix_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_tri_strip_checked)(
            c,
            op,
            src,
            dst,
            mask_format,
            src_x,
            src_y,
            points_len,
            points,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_tri_fan(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        src_x: i16,
        src_y: i16,
        points_len: u32,
        points: *const xcb_render_pointfix_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_tri_fan)(
            c,
            op,
            src,
            dst,
            mask_format,
            src_x,
            src_y,
            points_len,
            points,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_tri_fan_checked(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        src_x: i16,
        src_y: i16,
        points_len: u32,
        points: *const xcb_render_pointfix_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_tri_fan_checked)(
            c,
            op,
            src,
            dst,
            mask_format,
            src_x,
            src_y,
            points_len,
            points,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_create_glyph_set(
        &self,
        c: *mut xcb_connection_t,
        gsid: xcb_render_glyphset_t,
        format: xcb_render_pictformat_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_create_glyph_set)(c, gsid, format)
    }

    #[inline]
    pub unsafe fn xcb_render_create_glyph_set_checked(
        &self,
        c: *mut xcb_connection_t,
        gsid: xcb_render_glyphset_t,
        format: xcb_render_pictformat_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_create_glyph_set_checked)(c, gsid, format)
    }

    #[inline]
    pub unsafe fn xcb_render_reference_glyph_set(
        &self,
        c: *mut xcb_connection_t,
        gsid: xcb_render_glyphset_t,
        existing: xcb_render_glyphset_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_reference_glyph_set)(c, gsid, existing)
    }

    #[inline]
    pub unsafe fn xcb_render_reference_glyph_set_checked(
        &self,
        c: *mut xcb_connection_t,
        gsid: xcb_render_glyphset_t,
        existing: xcb_render_glyphset_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_reference_glyph_set_checked)(c, gsid, existing)
    }

    #[inline]
    pub unsafe fn xcb_render_free_glyph_set(
        &self,
        c: *mut xcb_connection_t,
        glyphset: xcb_render_glyphset_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_free_glyph_set)(c, glyphset)
    }

    #[inline]
    pub unsafe fn xcb_render_free_glyph_set_checked(
        &self,
        c: *mut xcb_connection_t,
        glyphset: xcb_render_glyphset_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_free_glyph_set_checked)(c, glyphset)
    }

    #[inline]
    pub unsafe fn xcb_render_add_glyphs(
        &self,
        c: *mut xcb_connection_t,
        glyphset: xcb_render_glyphset_t,
        glyphs_len: u32,
        glyphids: *const u32,
        glyphs: *const xcb_render_glyphinfo_t,
        data_len: u32,
        data: *const u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_add_glyphs)(
            c, glyphset, glyphs_len, glyphids, glyphs, data_len, data,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_add_glyphs_checked(
        &self,
        c: *mut xcb_connection_t,
        glyphset: xcb_render_glyphset_t,
        glyphs_len: u32,
        glyphids: *const u32,
        glyphs: *const xcb_render_glyphinfo_t,
        data_len: u32,
        data: *const u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_add_glyphs_checked)(
            c, glyphset, glyphs_len, glyphids, glyphs, data_len, data,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_free_glyphs(
        &self,
        c: *mut xcb_connection_t,
        glyphset: xcb_render_glyphset_t,
        glyphs_len: u32,
        glyphs: *const xcb_render_glyph_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_free_glyphs)(c, glyphset, glyphs_len, glyphs)
    }

    #[inline]
    pub unsafe fn xcb_render_free_glyphs_checked(
        &self,
        c: *mut xcb_connection_t,
        glyphset: xcb_render_glyphset_t,
        glyphs_len: u32,
        glyphs: *const xcb_render_glyph_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_free_glyphs_checked)(c, glyphset, glyphs_len, glyphs)
    }

    #[inline]
    pub unsafe fn xcb_render_composite_glyphs_8(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        glyphset: xcb_render_glyphset_t,
        src_x: i16,
        src_y: i16,
        glyphcmds_len: u32,
        glyphcmds: *const u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_composite_glyphs_8)(
            c,
            op,
            src,
            dst,
            mask_format,
            glyphset,
            src_x,
            src_y,
            glyphcmds_len,
            glyphcmds,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_composite_glyphs_8_checked(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        glyphset: xcb_render_glyphset_t,
        src_x: i16,
        src_y: i16,
        glyphcmds_len: u32,
        glyphcmds: *const u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_composite_glyphs_8_checked)(
            c,
            op,
            src,
            dst,
            mask_format,
            glyphset,
            src_x,
            src_y,
            glyphcmds_len,
            glyphcmds,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_composite_glyphs_16(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        glyphset: xcb_render_glyphset_t,
        src_x: i16,
        src_y: i16,
        glyphcmds_len: u32,
        glyphcmds: *const u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_composite_glyphs_16)(
            c,
            op,
            src,
            dst,
            mask_format,
            glyphset,
            src_x,
            src_y,
            glyphcmds_len,
            glyphcmds,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_composite_glyphs_16_checked(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        glyphset: xcb_render_glyphset_t,
        src_x: i16,
        src_y: i16,
        glyphcmds_len: u32,
        glyphcmds: *const u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_composite_glyphs_16_checked)(
            c,
            op,
            src,
            dst,
            mask_format,
            glyphset,
            src_x,
            src_y,
            glyphcmds_len,
            glyphcmds,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_composite_glyphs_32(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        glyphset: xcb_render_glyphset_t,
        src_x: i16,
        src_y: i16,
        glyphcmds_len: u32,
        glyphcmds: *const u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_composite_glyphs_32)(
            c,
            op,
            src,
            dst,
            mask_format,
            glyphset,
            src_x,
            src_y,
            glyphcmds_len,
            glyphcmds,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_composite_glyphs_32_checked(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        glyphset: xcb_render_glyphset_t,
        src_x: i16,
        src_y: i16,
        glyphcmds_len: u32,
        glyphcmds: *const u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_composite_glyphs_32_checked)(
            c,
            op,
            src,
            dst,
            mask_format,
            glyphset,
            src_x,
            src_y,
            glyphcmds_len,
            glyphcmds,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_fill_rectangles(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        dst: xcb_render_picture_t,
        color: xcb_render_color_t,
        rects_len: u32,
        rects: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_fill_rectangles)(c, op, dst, color, rects_len, rects)
    }

    #[inline]
    pub unsafe fn xcb_render_fill_rectangles_checked(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        dst: xcb_render_picture_t,
        color: xcb_render_color_t,
        rects_len: u32,
        rects: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_fill_rectangles_checked)(c, op, dst, color, rects_len, rects)
    }

    #[inline]
    pub unsafe fn xcb_render_create_cursor(
        &self,
        c: *mut xcb_connection_t,
        cid: xcb_cursor_t,
        source: xcb_render_picture_t,
        x: u16,
        y: u16,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_create_cursor)(c, cid, source, x, y)
    }

    #[inline]
    pub unsafe fn xcb_render_create_cursor_checked(
        &self,
        c: *mut xcb_connection_t,
        cid: xcb_cursor_t,
        source: xcb_render_picture_t,
        x: u16,
        y: u16,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_create_cursor_checked)(c, cid, source, x, y)
    }

    #[inline]
    pub unsafe fn xcb_render_transform_next(&self, i: *mut xcb_render_transform_iterator_t) {
        call!(self, xcb_render_transform_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_render_transform_end(
        &self,
        i: *mut xcb_render_transform_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_render_transform_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_render_set_picture_transform(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        transform: xcb_render_transform_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_set_picture_transform)(c, picture, transform)
    }

    #[inline]
    pub unsafe fn xcb_render_set_picture_transform_checked(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        transform: xcb_render_transform_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_set_picture_transform_checked)(c, picture, transform)
    }

    #[inline]
    pub unsafe fn xcb_render_query_filters_aliases(
        &self,
        R: *const xcb_render_query_filters_reply_t,
    ) -> *mut u16 {
        call!(self, xcb_render_query_filters_aliases)(R)
    }

    #[inline]
    pub unsafe fn xcb_render_query_filters_aliases_length(
        &self,
        R: *const xcb_render_query_filters_reply_t,
    ) -> c_int {
        call!(self, xcb_render_query_filters_aliases_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_render_query_filters_aliases_end(
        &self,
        R: *const xcb_render_query_filters_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_render_query_filters_aliases_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_render_query_filters_filters_length(
        &self,
        R: *const xcb_render_query_filters_reply_t,
    ) -> c_int {
        call!(self, xcb_render_query_filters_filters_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_render_query_filters_filters_iterator(
        &self,
        R: *const xcb_render_query_filters_reply_t,
    ) -> xcb_str_iterator_t {
        call!(self, xcb_render_query_filters_filters_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_render_query_filters_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_render_query_filters_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_render_query_filters_reply_t {
        call!(self, xcb_render_query_filters_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_render_query_filters(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_render_query_filters_cookie_t {
        call!(self, xcb_render_query_filters)(c, drawable)
    }

    #[inline]
    pub unsafe fn xcb_render_query_filters_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_render_query_filters_cookie_t {
        call!(self, xcb_render_query_filters_unchecked)(c, drawable)
    }

    #[inline]
    pub unsafe fn xcb_render_set_picture_filter(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        filter_len: u16,
        filter: *const c_char,
        values_len: u32,
        values: *const xcb_render_fixed_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_set_picture_filter)(
            c, picture, filter_len, filter, values_len, values,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_set_picture_filter_checked(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        filter_len: u16,
        filter: *const c_char,
        values_len: u32,
        values: *const xcb_render_fixed_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_set_picture_filter_checked)(
            c, picture, filter_len, filter, values_len, values,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_animcursorelt_next(
        &self,
        i: *mut xcb_render_animcursorelt_iterator_t,
    ) {
        call!(self, xcb_render_animcursorelt_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_render_animcursorelt_end(
        &self,
        i: *mut xcb_render_animcursorelt_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_render_animcursorelt_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_render_create_anim_cursor(
        &self,
        c: *mut xcb_connection_t,
        cid: xcb_cursor_t,
        cursors_len: u32,
        cursors: *const xcb_render_animcursorelt_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_create_anim_cursor)(c, cid, cursors_len, cursors)
    }

    #[inline]
    pub unsafe fn xcb_render_create_anim_cursor_checked(
        &self,
        c: *mut xcb_connection_t,
        cid: xcb_cursor_t,
        cursors_len: u32,
        cursors: *const xcb_render_animcursorelt_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_create_anim_cursor_checked)(c, cid, cursors_len, cursors)
    }

    #[inline]
    pub unsafe fn xcb_render_spanfix_next(&self, i: *mut xcb_render_spanfix_iterator_t) {
        call!(self, xcb_render_spanfix_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_render_spanfix_end(
        &self,
        i: *mut xcb_render_spanfix_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_render_spanfix_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_render_trap_next(&self, i: *mut xcb_render_trap_iterator_t) {
        call!(self, xcb_render_trap_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_render_trap_end(
        &self,
        i: *mut xcb_render_trap_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_render_trap_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_render_add_traps(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        x_off: i16,
        y_off: i16,
        traps_len: u32,
        traps: *const xcb_render_trap_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_add_traps)(c, picture, x_off, y_off, traps_len, traps)
    }

    #[inline]
    pub unsafe fn xcb_render_add_traps_checked(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        x_off: i16,
        y_off: i16,
        traps_len: u32,
        traps: *const xcb_render_trap_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_add_traps_checked)(c, picture, x_off, y_off, traps_len, traps)
    }

    #[inline]
    pub unsafe fn xcb_render_create_solid_fill(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        color: xcb_render_color_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_create_solid_fill)(c, picture, color)
    }

    #[inline]
    pub unsafe fn xcb_render_create_solid_fill_checked(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        color: xcb_render_color_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_create_solid_fill_checked)(c, picture, color)
    }

    #[inline]
    pub unsafe fn xcb_render_create_linear_gradient(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        p1: xcb_render_pointfix_t,
        p2: xcb_render_pointfix_t,
        num_stops: u32,
        stops: *const xcb_render_fixed_t,
        colors: *const xcb_render_color_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_create_linear_gradient)(c, picture, p1, p2, num_stops, stops, colors)
    }

    #[inline]
    pub unsafe fn xcb_render_create_linear_gradient_checked(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        p1: xcb_render_pointfix_t,
        p2: xcb_render_pointfix_t,
        num_stops: u32,
        stops: *const xcb_render_fixed_t,
        colors: *const xcb_render_color_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_create_linear_gradient_checked)(
            c, picture, p1, p2, num_stops, stops, colors,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_create_radial_gradient(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        inner: xcb_render_pointfix_t,
        outer: xcb_render_pointfix_t,
        inner_radius: xcb_render_fixed_t,
        outer_radius: xcb_render_fixed_t,
        num_stops: u32,
        stops: *const xcb_render_fixed_t,
        colors: *const xcb_render_color_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_create_radial_gradient)(
            c,
            picture,
            inner,
            outer,
            inner_radius,
            outer_radius,
            num_stops,
            stops,
            colors,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_create_radial_gradient_checked(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        inner: xcb_render_pointfix_t,
        outer: xcb_render_pointfix_t,
        inner_radius: xcb_render_fixed_t,
        outer_radius: xcb_render_fixed_t,
        num_stops: u32,
        stops: *const xcb_render_fixed_t,
        colors: *const xcb_render_color_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_create_radial_gradient_checked)(
            c,
            picture,
            inner,
            outer,
            inner_radius,
            outer_radius,
            num_stops,
            stops,
            colors,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_create_conical_gradient(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        center: xcb_render_pointfix_t,
        angle: xcb_render_fixed_t,
        num_stops: u32,
        stops: *const xcb_render_fixed_t,
        colors: *const xcb_render_color_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_create_conical_gradient)(
            c, picture, center, angle, num_stops, stops, colors,
        )
    }

    #[inline]
    pub unsafe fn xcb_render_create_conical_gradient_checked(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        center: xcb_render_pointfix_t,
        angle: xcb_render_fixed_t,
        num_stops: u32,
        stops: *const xcb_render_fixed_t,
        colors: *const xcb_render_color_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_render_create_conical_gradient_checked)(
            c, picture, center, angle, num_stops, stops, colors,
        )
    }
}

pub struct XcbRender {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_render_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_render_glyph_next: LazySymbol<unsafe fn(i: *mut xcb_render_glyph_iterator_t)>,
    pub(crate) xcb_render_glyph_end:
        LazySymbol<unsafe fn(i: *mut xcb_render_glyph_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_render_glyphset_next:
        LazySymbol<unsafe fn(i: *mut xcb_render_glyphset_iterator_t)>,
    pub(crate) xcb_render_glyphset_end:
        LazySymbol<unsafe fn(i: *mut xcb_render_glyphset_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_render_picture_next:
        LazySymbol<unsafe fn(i: *mut xcb_render_picture_iterator_t)>,
    pub(crate) xcb_render_picture_end:
        LazySymbol<unsafe fn(i: *mut xcb_render_picture_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_render_pictformat_next:
        LazySymbol<unsafe fn(i: *mut xcb_render_pictformat_iterator_t)>,
    pub(crate) xcb_render_pictformat_end:
        LazySymbol<unsafe fn(i: *mut xcb_render_pictformat_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_render_fixed_next: LazySymbol<unsafe fn(i: *mut xcb_render_fixed_iterator_t)>,
    pub(crate) xcb_render_fixed_end:
        LazySymbol<unsafe fn(i: *mut xcb_render_fixed_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_render_directformat_next:
        LazySymbol<unsafe fn(i: *mut xcb_render_directformat_iterator_t)>,
    pub(crate) xcb_render_directformat_end:
        LazySymbol<unsafe fn(i: *mut xcb_render_directformat_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_render_pictforminfo_next:
        LazySymbol<unsafe fn(i: *mut xcb_render_pictforminfo_iterator_t)>,
    pub(crate) xcb_render_pictforminfo_end:
        LazySymbol<unsafe fn(i: *mut xcb_render_pictforminfo_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_render_pictvisual_next:
        LazySymbol<unsafe fn(i: *mut xcb_render_pictvisual_iterator_t)>,
    pub(crate) xcb_render_pictvisual_end:
        LazySymbol<unsafe fn(i: *mut xcb_render_pictvisual_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_render_pictdepth_visuals:
        LazySymbol<unsafe fn(R: *const xcb_render_pictdepth_t) -> *mut xcb_render_pictvisual_t>,
    pub(crate) xcb_render_pictdepth_visuals_length:
        LazySymbol<unsafe fn(R: *const xcb_render_pictdepth_t) -> c_int>,
    pub(crate) xcb_render_pictdepth_visuals_iterator:
        LazySymbol<unsafe fn(R: *const xcb_render_pictdepth_t) -> xcb_render_pictvisual_iterator_t>,
    pub(crate) xcb_render_pictdepth_next:
        LazySymbol<unsafe fn(i: *mut xcb_render_pictdepth_iterator_t)>,
    pub(crate) xcb_render_pictdepth_end:
        LazySymbol<unsafe fn(i: *mut xcb_render_pictdepth_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_render_pictscreen_depths_length:
        LazySymbol<unsafe fn(R: *const xcb_render_pictscreen_t) -> c_int>,
    pub(crate) xcb_render_pictscreen_depths_iterator:
        LazySymbol<unsafe fn(R: *const xcb_render_pictscreen_t) -> xcb_render_pictdepth_iterator_t>,
    pub(crate) xcb_render_pictscreen_next:
        LazySymbol<unsafe fn(i: *mut xcb_render_pictscreen_iterator_t)>,
    pub(crate) xcb_render_pictscreen_end:
        LazySymbol<unsafe fn(i: *mut xcb_render_pictscreen_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_render_indexvalue_next:
        LazySymbol<unsafe fn(i: *mut xcb_render_indexvalue_iterator_t)>,
    pub(crate) xcb_render_indexvalue_end:
        LazySymbol<unsafe fn(i: *mut xcb_render_indexvalue_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_render_color_next: LazySymbol<unsafe fn(i: *mut xcb_render_color_iterator_t)>,
    pub(crate) xcb_render_color_end:
        LazySymbol<unsafe fn(i: *mut xcb_render_color_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_render_pointfix_next:
        LazySymbol<unsafe fn(i: *mut xcb_render_pointfix_iterator_t)>,
    pub(crate) xcb_render_pointfix_end:
        LazySymbol<unsafe fn(i: *mut xcb_render_pointfix_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_render_linefix_next:
        LazySymbol<unsafe fn(i: *mut xcb_render_linefix_iterator_t)>,
    pub(crate) xcb_render_linefix_end:
        LazySymbol<unsafe fn(i: *mut xcb_render_linefix_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_render_triangle_next:
        LazySymbol<unsafe fn(i: *mut xcb_render_triangle_iterator_t)>,
    pub(crate) xcb_render_triangle_end:
        LazySymbol<unsafe fn(i: *mut xcb_render_triangle_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_render_trapezoid_next:
        LazySymbol<unsafe fn(i: *mut xcb_render_trapezoid_iterator_t)>,
    pub(crate) xcb_render_trapezoid_end:
        LazySymbol<unsafe fn(i: *mut xcb_render_trapezoid_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_render_glyphinfo_next:
        LazySymbol<unsafe fn(i: *mut xcb_render_glyphinfo_iterator_t)>,
    pub(crate) xcb_render_glyphinfo_end:
        LazySymbol<unsafe fn(i: *mut xcb_render_glyphinfo_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_render_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_render_query_version_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_render_query_version_reply_t,
    >,
    pub(crate) xcb_render_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u32,
            client_minor_version: u32,
        ) -> xcb_render_query_version_cookie_t,
    >,
    pub(crate) xcb_render_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u32,
            client_minor_version: u32,
        ) -> xcb_render_query_version_cookie_t,
    >,
    pub(crate) xcb_render_query_pict_formats_formats: LazySymbol<
        unsafe fn(
            R: *const xcb_render_query_pict_formats_reply_t,
        ) -> *mut xcb_render_pictforminfo_t,
    >,
    pub(crate) xcb_render_query_pict_formats_formats_length:
        LazySymbol<unsafe fn(R: *const xcb_render_query_pict_formats_reply_t) -> c_int>,
    pub(crate) xcb_render_query_pict_formats_formats_iterator: LazySymbol<
        unsafe fn(
            R: *const xcb_render_query_pict_formats_reply_t,
        ) -> xcb_render_pictforminfo_iterator_t,
    >,
    pub(crate) xcb_render_query_pict_formats_screens_length:
        LazySymbol<unsafe fn(R: *const xcb_render_query_pict_formats_reply_t) -> c_int>,
    pub(crate) xcb_render_query_pict_formats_screens_iterator: LazySymbol<
        unsafe fn(
            R: *const xcb_render_query_pict_formats_reply_t,
        ) -> xcb_render_pictscreen_iterator_t,
    >,
    pub(crate) xcb_render_query_pict_formats_subpixels:
        LazySymbol<unsafe fn(R: *const xcb_render_query_pict_formats_reply_t) -> *mut u32>,
    pub(crate) xcb_render_query_pict_formats_subpixels_length:
        LazySymbol<unsafe fn(R: *const xcb_render_query_pict_formats_reply_t) -> c_int>,
    pub(crate) xcb_render_query_pict_formats_subpixels_end: LazySymbol<
        unsafe fn(R: *const xcb_render_query_pict_formats_reply_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_render_query_pict_formats_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_render_query_pict_formats_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_render_query_pict_formats_reply_t,
    >,
    pub(crate) xcb_render_query_pict_formats:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_render_query_pict_formats_cookie_t>,
    pub(crate) xcb_render_query_pict_formats_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_render_query_pict_formats_cookie_t>,
    pub(crate) xcb_render_query_pict_index_values_values: LazySymbol<
        unsafe fn(
            R: *const xcb_render_query_pict_index_values_reply_t,
        ) -> *mut xcb_render_indexvalue_t,
    >,
    pub(crate) xcb_render_query_pict_index_values_values_length:
        LazySymbol<unsafe fn(R: *const xcb_render_query_pict_index_values_reply_t) -> c_int>,
    pub(crate) xcb_render_query_pict_index_values_values_iterator: LazySymbol<
        unsafe fn(
            R: *const xcb_render_query_pict_index_values_reply_t,
        ) -> xcb_render_indexvalue_iterator_t,
    >,
    pub(crate) xcb_render_query_pict_index_values_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_render_query_pict_index_values_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_render_query_pict_index_values_reply_t,
    >,
    pub(crate) xcb_render_query_pict_index_values: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            format: xcb_render_pictformat_t,
        ) -> xcb_render_query_pict_index_values_cookie_t,
    >,
    pub(crate) xcb_render_query_pict_index_values_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            format: xcb_render_pictformat_t,
        ) -> xcb_render_query_pict_index_values_cookie_t,
    >,
    pub(crate) xcb_render_create_picture: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            pid: xcb_render_picture_t,
            drawable: xcb_drawable_t,
            format: xcb_render_pictformat_t,
            value_mask: u32,
            value_list: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_create_picture_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            pid: xcb_render_picture_t,
            drawable: xcb_drawable_t,
            format: xcb_render_pictformat_t,
            value_mask: u32,
            value_list: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_change_picture: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            value_mask: u32,
            value_list: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_change_picture_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            value_mask: u32,
            value_list: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_set_picture_clip_rectangles: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            clip_x_origin: i16,
            clip_y_origin: i16,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_set_picture_clip_rectangles_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            clip_x_origin: i16,
            clip_y_origin: i16,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_free_picture: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, picture: xcb_render_picture_t) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_free_picture_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, picture: xcb_render_picture_t) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_composite: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            mask: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            src_x: i16,
            src_y: i16,
            mask_x: i16,
            mask_y: i16,
            dst_x: i16,
            dst_y: i16,
            width: u16,
            height: u16,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_composite_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            mask: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            src_x: i16,
            src_y: i16,
            mask_x: i16,
            mask_y: i16,
            dst_x: i16,
            dst_y: i16,
            width: u16,
            height: u16,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_trapezoids: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            src_x: i16,
            src_y: i16,
            traps_len: u32,
            traps: *const xcb_render_trapezoid_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_trapezoids_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            src_x: i16,
            src_y: i16,
            traps_len: u32,
            traps: *const xcb_render_trapezoid_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_triangles: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            src_x: i16,
            src_y: i16,
            triangles_len: u32,
            triangles: *const xcb_render_triangle_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_triangles_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            src_x: i16,
            src_y: i16,
            triangles_len: u32,
            triangles: *const xcb_render_triangle_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_tri_strip: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            src_x: i16,
            src_y: i16,
            points_len: u32,
            points: *const xcb_render_pointfix_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_tri_strip_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            src_x: i16,
            src_y: i16,
            points_len: u32,
            points: *const xcb_render_pointfix_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_tri_fan: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            src_x: i16,
            src_y: i16,
            points_len: u32,
            points: *const xcb_render_pointfix_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_tri_fan_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            src_x: i16,
            src_y: i16,
            points_len: u32,
            points: *const xcb_render_pointfix_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_create_glyph_set: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gsid: xcb_render_glyphset_t,
            format: xcb_render_pictformat_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_create_glyph_set_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gsid: xcb_render_glyphset_t,
            format: xcb_render_pictformat_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_reference_glyph_set: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gsid: xcb_render_glyphset_t,
            existing: xcb_render_glyphset_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_reference_glyph_set_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gsid: xcb_render_glyphset_t,
            existing: xcb_render_glyphset_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_free_glyph_set: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, glyphset: xcb_render_glyphset_t) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_free_glyph_set_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, glyphset: xcb_render_glyphset_t) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_add_glyphs: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            glyphset: xcb_render_glyphset_t,
            glyphs_len: u32,
            glyphids: *const u32,
            glyphs: *const xcb_render_glyphinfo_t,
            data_len: u32,
            data: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_add_glyphs_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            glyphset: xcb_render_glyphset_t,
            glyphs_len: u32,
            glyphids: *const u32,
            glyphs: *const xcb_render_glyphinfo_t,
            data_len: u32,
            data: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_free_glyphs: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            glyphset: xcb_render_glyphset_t,
            glyphs_len: u32,
            glyphs: *const xcb_render_glyph_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_free_glyphs_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            glyphset: xcb_render_glyphset_t,
            glyphs_len: u32,
            glyphs: *const xcb_render_glyph_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_composite_glyphs_8: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            glyphset: xcb_render_glyphset_t,
            src_x: i16,
            src_y: i16,
            glyphcmds_len: u32,
            glyphcmds: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_composite_glyphs_8_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            glyphset: xcb_render_glyphset_t,
            src_x: i16,
            src_y: i16,
            glyphcmds_len: u32,
            glyphcmds: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_composite_glyphs_16: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            glyphset: xcb_render_glyphset_t,
            src_x: i16,
            src_y: i16,
            glyphcmds_len: u32,
            glyphcmds: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_composite_glyphs_16_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            glyphset: xcb_render_glyphset_t,
            src_x: i16,
            src_y: i16,
            glyphcmds_len: u32,
            glyphcmds: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_composite_glyphs_32: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            glyphset: xcb_render_glyphset_t,
            src_x: i16,
            src_y: i16,
            glyphcmds_len: u32,
            glyphcmds: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_composite_glyphs_32_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            glyphset: xcb_render_glyphset_t,
            src_x: i16,
            src_y: i16,
            glyphcmds_len: u32,
            glyphcmds: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_fill_rectangles: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            dst: xcb_render_picture_t,
            color: xcb_render_color_t,
            rects_len: u32,
            rects: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_fill_rectangles_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            dst: xcb_render_picture_t,
            color: xcb_render_color_t,
            rects_len: u32,
            rects: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_create_cursor: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cid: xcb_cursor_t,
            source: xcb_render_picture_t,
            x: u16,
            y: u16,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_create_cursor_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cid: xcb_cursor_t,
            source: xcb_render_picture_t,
            x: u16,
            y: u16,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_transform_next:
        LazySymbol<unsafe fn(i: *mut xcb_render_transform_iterator_t)>,
    pub(crate) xcb_render_transform_end:
        LazySymbol<unsafe fn(i: *mut xcb_render_transform_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_render_set_picture_transform: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            transform: xcb_render_transform_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_set_picture_transform_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            transform: xcb_render_transform_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_query_filters_aliases:
        LazySymbol<unsafe fn(R: *const xcb_render_query_filters_reply_t) -> *mut u16>,
    pub(crate) xcb_render_query_filters_aliases_length:
        LazySymbol<unsafe fn(R: *const xcb_render_query_filters_reply_t) -> c_int>,
    pub(crate) xcb_render_query_filters_aliases_end:
        LazySymbol<unsafe fn(R: *const xcb_render_query_filters_reply_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_render_query_filters_filters_length:
        LazySymbol<unsafe fn(R: *const xcb_render_query_filters_reply_t) -> c_int>,
    pub(crate) xcb_render_query_filters_filters_iterator:
        LazySymbol<unsafe fn(R: *const xcb_render_query_filters_reply_t) -> xcb_str_iterator_t>,
    pub(crate) xcb_render_query_filters_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_render_query_filters_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_render_query_filters_reply_t,
    >,
    pub(crate) xcb_render_query_filters: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
        ) -> xcb_render_query_filters_cookie_t,
    >,
    pub(crate) xcb_render_query_filters_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
        ) -> xcb_render_query_filters_cookie_t,
    >,
    pub(crate) xcb_render_set_picture_filter: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            filter_len: u16,
            filter: *const c_char,
            values_len: u32,
            values: *const xcb_render_fixed_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_set_picture_filter_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            filter_len: u16,
            filter: *const c_char,
            values_len: u32,
            values: *const xcb_render_fixed_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_animcursorelt_next:
        LazySymbol<unsafe fn(i: *mut xcb_render_animcursorelt_iterator_t)>,
    pub(crate) xcb_render_animcursorelt_end: LazySymbol<
        unsafe fn(i: *mut xcb_render_animcursorelt_iterator_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_render_create_anim_cursor: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cid: xcb_cursor_t,
            cursors_len: u32,
            cursors: *const xcb_render_animcursorelt_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_create_anim_cursor_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cid: xcb_cursor_t,
            cursors_len: u32,
            cursors: *const xcb_render_animcursorelt_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_spanfix_next:
        LazySymbol<unsafe fn(i: *mut xcb_render_spanfix_iterator_t)>,
    pub(crate) xcb_render_spanfix_end:
        LazySymbol<unsafe fn(i: *mut xcb_render_spanfix_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_render_trap_next: LazySymbol<unsafe fn(i: *mut xcb_render_trap_iterator_t)>,
    pub(crate) xcb_render_trap_end:
        LazySymbol<unsafe fn(i: *mut xcb_render_trap_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_render_add_traps: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            x_off: i16,
            y_off: i16,
            traps_len: u32,
            traps: *const xcb_render_trap_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_add_traps_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            x_off: i16,
            y_off: i16,
            traps_len: u32,
            traps: *const xcb_render_trap_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_create_solid_fill: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            color: xcb_render_color_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_create_solid_fill_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            color: xcb_render_color_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_create_linear_gradient: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            p1: xcb_render_pointfix_t,
            p2: xcb_render_pointfix_t,
            num_stops: u32,
            stops: *const xcb_render_fixed_t,
            colors: *const xcb_render_color_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_create_linear_gradient_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            p1: xcb_render_pointfix_t,
            p2: xcb_render_pointfix_t,
            num_stops: u32,
            stops: *const xcb_render_fixed_t,
            colors: *const xcb_render_color_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_create_radial_gradient: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            inner: xcb_render_pointfix_t,
            outer: xcb_render_pointfix_t,
            inner_radius: xcb_render_fixed_t,
            outer_radius: xcb_render_fixed_t,
            num_stops: u32,
            stops: *const xcb_render_fixed_t,
            colors: *const xcb_render_color_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_create_radial_gradient_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            inner: xcb_render_pointfix_t,
            outer: xcb_render_pointfix_t,
            inner_radius: xcb_render_fixed_t,
            outer_radius: xcb_render_fixed_t,
            num_stops: u32,
            stops: *const xcb_render_fixed_t,
            colors: *const xcb_render_color_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_create_conical_gradient: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            center: xcb_render_pointfix_t,
            angle: xcb_render_fixed_t,
            num_stops: u32,
            stops: *const xcb_render_fixed_t,
            colors: *const xcb_render_color_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_render_create_conical_gradient_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            center: xcb_render_pointfix_t,
            angle: xcb_render_fixed_t,
            num_stops: u32,
            stops: *const xcb_render_fixed_t,
            colors: *const xcb_render_color_t,
        ) -> xcb_void_cookie_t,
    >,
}
