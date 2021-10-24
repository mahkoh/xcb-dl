use crate::ffi::*;
use crate::*;
use std::os::raw::*;

pub const XCB_GLX_MAJOR_VERSION: u32 = 1;
pub const XCB_GLX_MINOR_VERSION: u32 = 4;

pub type xcb_glx_pixmap_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_pixmap_iterator_t {
    pub data: *mut xcb_glx_pixmap_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_glx_context_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_context_iterator_t {
    pub data: *mut xcb_glx_context_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_glx_pbuffer_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_pbuffer_iterator_t {
    pub data: *mut xcb_glx_pbuffer_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_glx_window_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_window_iterator_t {
    pub data: *mut xcb_glx_window_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_glx_fbconfig_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_fbconfig_iterator_t {
    pub data: *mut xcb_glx_fbconfig_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_glx_drawable_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_drawable_iterator_t {
    pub data: *mut xcb_glx_drawable_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_glx_float32_t = f32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_float32_iterator_t {
    pub data: *mut xcb_glx_float32_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_glx_float64_t = f64;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_float64_iterator_t {
    pub data: *mut xcb_glx_float64_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_glx_bool32_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_bool32_iterator_t {
    pub data: *mut xcb_glx_bool32_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_glx_context_tag_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_context_tag_iterator_t {
    pub data: *mut xcb_glx_context_tag_t,
    pub rem: c_int,
    pub index: c_int,
}

pub const XCB_GLX_GENERIC: i8 = -1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_generic_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
    pub pad0: [u8; 21],
}

pub const XCB_GLX_BAD_CONTEXT: u8 = 0;

pub type xcb_glx_bad_context_error_t = xcb_glx_generic_error_t;

pub const XCB_GLX_BAD_CONTEXT_STATE: u8 = 1;

pub type xcb_glx_bad_context_state_error_t = xcb_glx_generic_error_t;

pub const XCB_GLX_BAD_DRAWABLE: u8 = 2;

pub type xcb_glx_bad_drawable_error_t = xcb_glx_generic_error_t;

pub const XCB_GLX_BAD_PIXMAP: u8 = 3;

pub type xcb_glx_bad_pixmap_error_t = xcb_glx_generic_error_t;

pub const XCB_GLX_BAD_CONTEXT_TAG: u8 = 4;

pub type xcb_glx_bad_context_tag_error_t = xcb_glx_generic_error_t;

pub const XCB_GLX_BAD_CURRENT_WINDOW: u8 = 5;

pub type xcb_glx_bad_current_window_error_t = xcb_glx_generic_error_t;

pub const XCB_GLX_BAD_RENDER_REQUEST: u8 = 6;

pub type xcb_glx_bad_render_request_error_t = xcb_glx_generic_error_t;

pub const XCB_GLX_BAD_LARGE_REQUEST: u8 = 7;

pub type xcb_glx_bad_large_request_error_t = xcb_glx_generic_error_t;

pub const XCB_GLX_UNSUPPORTED_PRIVATE_REQUEST: u8 = 8;

pub type xcb_glx_unsupported_private_request_error_t = xcb_glx_generic_error_t;

pub const XCB_GLX_BAD_FB_CONFIG: u8 = 9;

pub type xcb_glx_bad_fb_config_error_t = xcb_glx_generic_error_t;

pub const XCB_GLX_BAD_PBUFFER: u8 = 10;

pub type xcb_glx_bad_pbuffer_error_t = xcb_glx_generic_error_t;

pub const XCB_GLX_BAD_CURRENT_DRAWABLE: u8 = 11;

pub type xcb_glx_bad_current_drawable_error_t = xcb_glx_generic_error_t;

pub const XCB_GLX_BAD_WINDOW: u8 = 12;

pub type xcb_glx_bad_window_error_t = xcb_glx_generic_error_t;

pub const XCB_GLX_GLX_BAD_PROFILE_ARB: u8 = 13;

pub type xcb_glx_glx_bad_profile_arb_error_t = xcb_glx_generic_error_t;

pub const XCB_GLX_PBUFFER_CLOBBER: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_pbuffer_clobber_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event_type: u16,
    pub draw_type: u16,
    pub drawable: xcb_glx_drawable_t,
    pub b_mask: u32,
    pub aux_buffer: u16,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub count: u16,
    pub pad1: [u8; 4],
}

pub const XCB_GLX_BUFFER_SWAP_COMPLETE: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_buffer_swap_complete_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event_type: u16,
    pub pad1: [u8; 2],
    pub drawable: xcb_glx_drawable_t,
    pub ust_hi: u32,
    pub ust_lo: u32,
    pub msc_hi: u32,
    pub msc_lo: u32,
    pub sbc: u32,
}

pub type xcb_glx_pbcet_t = u32;
pub const XCB_GLX_PBCET_DAMAGED: xcb_glx_pbcet_t = 0x8017;
pub const XCB_GLX_PBCET_SAVED: xcb_glx_pbcet_t = 0x8018;

pub type xcb_glx_pbcdt_t = u32;
pub const XCB_GLX_PBCDT_WINDOW: xcb_glx_pbcdt_t = 0x8019;
pub const XCB_GLX_PBCDT_PBUFFER: xcb_glx_pbcdt_t = 0x801a;

pub const XCB_GLX_RENDER: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_render_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
}

pub const XCB_GLX_RENDER_LARGE: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_render_large_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub request_num: u16,
    pub request_total: u16,
    pub data_len: u32,
}

pub const XCB_GLX_CREATE_CONTEXT: u8 = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_create_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_glx_context_t,
    pub visual: xcb_visualid_t,
    pub screen: u32,
    pub share_list: xcb_glx_context_t,
    pub is_direct: u8,
    pub pad0: [u8; 3],
}

pub const XCB_GLX_DESTROY_CONTEXT: u8 = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_destroy_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_glx_context_t,
}

pub const XCB_GLX_MAKE_CURRENT: u8 = 5;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_make_current_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_glx_drawable_t,
    pub context: xcb_glx_context_t,
    pub old_context_tag: xcb_glx_context_tag_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_make_current_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_make_current_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_tag: xcb_glx_context_tag_t,
    pub pad1: [u8; 20],
}

pub const XCB_GLX_IS_DIRECT: u8 = 6;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_is_direct_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_glx_context_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_is_direct_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_is_direct_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub is_direct: u8,
    pub pad1: [u8; 23],
}

pub const XCB_GLX_QUERY_VERSION: u8 = 7;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub major_version: u32,
    pub minor_version: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_query_version_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
    pub pad1: [u8; 16],
}

pub const XCB_GLX_WAIT_GL: u8 = 8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_wait_gl_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
}

pub const XCB_GLX_WAIT_X: u8 = 9;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_wait_x_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
}

pub const XCB_GLX_COPY_CONTEXT: u8 = 10;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_copy_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub src: xcb_glx_context_t,
    pub dest: xcb_glx_context_t,
    pub mask: u32,
    pub src_context_tag: xcb_glx_context_tag_t,
}

pub type xcb_glx_gc_t = u32;
pub const XCB_GLX_GC_GL_CURRENT_BIT: xcb_glx_gc_t = 0x01;
pub const XCB_GLX_GC_GL_POINT_BIT: xcb_glx_gc_t = 0x02;
pub const XCB_GLX_GC_GL_LINE_BIT: xcb_glx_gc_t = 0x04;
pub const XCB_GLX_GC_GL_POLYGON_BIT: xcb_glx_gc_t = 0x08;
pub const XCB_GLX_GC_GL_POLYGON_STIPPLE_BIT: xcb_glx_gc_t = 0x10;
pub const XCB_GLX_GC_GL_PIXEL_MODE_BIT: xcb_glx_gc_t = 0x20;
pub const XCB_GLX_GC_GL_LIGHTING_BIT: xcb_glx_gc_t = 0x40;
pub const XCB_GLX_GC_GL_FOG_BIT: xcb_glx_gc_t = 0x80;
pub const XCB_GLX_GC_GL_DEPTH_BUFFER_BIT: xcb_glx_gc_t = 0x100;
pub const XCB_GLX_GC_GL_ACCUM_BUFFER_BIT: xcb_glx_gc_t = 0x200;
pub const XCB_GLX_GC_GL_STENCIL_BUFFER_BIT: xcb_glx_gc_t = 0x400;
pub const XCB_GLX_GC_GL_VIEWPORT_BIT: xcb_glx_gc_t = 0x800;
pub const XCB_GLX_GC_GL_TRANSFORM_BIT: xcb_glx_gc_t = 0x1000;
pub const XCB_GLX_GC_GL_ENABLE_BIT: xcb_glx_gc_t = 0x2000;
pub const XCB_GLX_GC_GL_COLOR_BUFFER_BIT: xcb_glx_gc_t = 0x4000;
pub const XCB_GLX_GC_GL_HINT_BIT: xcb_glx_gc_t = 0x8000;
pub const XCB_GLX_GC_GL_EVAL_BIT: xcb_glx_gc_t = 0x10000;
pub const XCB_GLX_GC_GL_LIST_BIT: xcb_glx_gc_t = 0x20000;
pub const XCB_GLX_GC_GL_TEXTURE_BIT: xcb_glx_gc_t = 0x40000;
pub const XCB_GLX_GC_GL_SCISSOR_BIT: xcb_glx_gc_t = 0x80000;
pub const XCB_GLX_GC_GL_ALL_ATTRIB_BITS: xcb_glx_gc_t = 0xffffff;

pub const XCB_GLX_SWAP_BUFFERS: u8 = 11;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_swap_buffers_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub drawable: xcb_glx_drawable_t,
}

pub const XCB_GLX_USE_X_FONT: u8 = 12;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_use_x_font_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub font: xcb_font_t,
    pub first: u32,
    pub count: u32,
    pub list_base: u32,
}

pub const XCB_GLX_CREATE_GLX_PIXMAP: u8 = 13;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_create_glx_pixmap_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
    pub visual: xcb_visualid_t,
    pub pixmap: xcb_pixmap_t,
    pub glx_pixmap: xcb_glx_pixmap_t,
}

pub const XCB_GLX_GET_VISUAL_CONFIGS: u8 = 14;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_visual_configs_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_visual_configs_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_visual_configs_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_visuals: u32,
    pub num_properties: u32,
    pub pad1: [u8; 16],
}

pub const XCB_GLX_DESTROY_GLX_PIXMAP: u8 = 15;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_destroy_glx_pixmap_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub glx_pixmap: xcb_glx_pixmap_t,
}

pub const XCB_GLX_VENDOR_PRIVATE: u8 = 16;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_vendor_private_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub vendor_code: u32,
    pub context_tag: xcb_glx_context_tag_t,
}

pub const XCB_GLX_VENDOR_PRIVATE_WITH_REPLY: u8 = 17;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_vendor_private_with_reply_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub vendor_code: u32,
    pub context_tag: xcb_glx_context_tag_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_vendor_private_with_reply_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_vendor_private_with_reply_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub retval: u32,
    pub data1: [u8; 24],
}

pub const XCB_GLX_QUERY_EXTENSIONS_STRING: u8 = 18;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_query_extensions_string_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_query_extensions_string_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_query_extensions_string_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub pad2: [u8; 16],
}

pub const XCB_GLX_QUERY_SERVER_STRING: u8 = 19;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_query_server_string_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
    pub name: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_query_server_string_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_query_server_string_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub str_len: u32,
    pub pad2: [u8; 16],
}

pub const XCB_GLX_CLIENT_INFO: u8 = 20;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_client_info_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub major_version: u32,
    pub minor_version: u32,
    pub str_len: u32,
}

pub const XCB_GLX_GET_FB_CONFIGS: u8 = 21;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_fb_configs_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_fb_configs_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_fb_configs_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_fb_configs: u32,
    pub num_properties: u32,
    pub pad1: [u8; 16],
}

pub const XCB_GLX_CREATE_PIXMAP: u8 = 22;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_create_pixmap_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
    pub fbconfig: xcb_glx_fbconfig_t,
    pub pixmap: xcb_pixmap_t,
    pub glx_pixmap: xcb_glx_pixmap_t,
    pub num_attribs: u32,
}

pub const XCB_GLX_DESTROY_PIXMAP: u8 = 23;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_destroy_pixmap_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub glx_pixmap: xcb_glx_pixmap_t,
}

pub const XCB_GLX_CREATE_NEW_CONTEXT: u8 = 24;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_create_new_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_glx_context_t,
    pub fbconfig: xcb_glx_fbconfig_t,
    pub screen: u32,
    pub render_type: u32,
    pub share_list: xcb_glx_context_t,
    pub is_direct: u8,
    pub pad0: [u8; 3],
}

pub const XCB_GLX_QUERY_CONTEXT: u8 = 25;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_query_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_glx_context_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_query_context_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_query_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_attribs: u32,
    pub pad1: [u8; 20],
}

pub const XCB_GLX_MAKE_CONTEXT_CURRENT: u8 = 26;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_make_context_current_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub old_context_tag: xcb_glx_context_tag_t,
    pub drawable: xcb_glx_drawable_t,
    pub read_drawable: xcb_glx_drawable_t,
    pub context: xcb_glx_context_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_make_context_current_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_make_context_current_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_tag: xcb_glx_context_tag_t,
    pub pad1: [u8; 20],
}

pub const XCB_GLX_CREATE_PBUFFER: u8 = 27;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_create_pbuffer_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
    pub fbconfig: xcb_glx_fbconfig_t,
    pub pbuffer: xcb_glx_pbuffer_t,
    pub num_attribs: u32,
}

pub const XCB_GLX_DESTROY_PBUFFER: u8 = 28;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_destroy_pbuffer_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub pbuffer: xcb_glx_pbuffer_t,
}

pub const XCB_GLX_GET_DRAWABLE_ATTRIBUTES: u8 = 29;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_drawable_attributes_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_glx_drawable_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_drawable_attributes_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_drawable_attributes_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_attribs: u32,
    pub pad1: [u8; 20],
}

pub const XCB_GLX_CHANGE_DRAWABLE_ATTRIBUTES: u8 = 30;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_change_drawable_attributes_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_glx_drawable_t,
    pub num_attribs: u32,
}

pub const XCB_GLX_CREATE_WINDOW: u8 = 31;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_create_window_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
    pub fbconfig: xcb_glx_fbconfig_t,
    pub window: xcb_window_t,
    pub glx_window: xcb_glx_window_t,
    pub num_attribs: u32,
}

pub const XCB_GLX_DELETE_WINDOW: u8 = 32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_delete_window_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub glxwindow: xcb_glx_window_t,
}

pub const XCB_GLX_SET_CLIENT_INFO_ARB: u8 = 33;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_set_client_info_arb_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub major_version: u32,
    pub minor_version: u32,
    pub num_versions: u32,
    pub gl_str_len: u32,
    pub glx_str_len: u32,
}

pub const XCB_GLX_CREATE_CONTEXT_ATTRIBS_ARB: u8 = 34;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_create_context_attribs_arb_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_glx_context_t,
    pub fbconfig: xcb_glx_fbconfig_t,
    pub screen: u32,
    pub share_list: xcb_glx_context_t,
    pub is_direct: u8,
    pub pad0: [u8; 3],
    pub num_attribs: u32,
}

pub const XCB_GLX_SET_CLIENT_INFO_2ARB: u8 = 35;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_set_client_info_2arb_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub major_version: u32,
    pub minor_version: u32,
    pub num_versions: u32,
    pub gl_str_len: u32,
    pub glx_str_len: u32,
}

pub const XCB_GLX_NEW_LIST: u8 = 101;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_new_list_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub list: u32,
    pub mode: u32,
}

pub const XCB_GLX_END_LIST: u8 = 102;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_end_list_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
}

pub const XCB_GLX_DELETE_LISTS: u8 = 103;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_delete_lists_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub list: u32,
    pub range: i32,
}

pub const XCB_GLX_GEN_LISTS: u8 = 104;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_gen_lists_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub range: i32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_gen_lists_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_gen_lists_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: u32,
}

pub const XCB_GLX_FEEDBACK_BUFFER: u8 = 105;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_feedback_buffer_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub size: i32,
    pub type_: i32,
}

pub const XCB_GLX_SELECT_BUFFER: u8 = 106;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_select_buffer_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub size: i32,
}

pub const XCB_GLX_RENDER_MODE: u8 = 107;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_render_mode_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub mode: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_render_mode_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_render_mode_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: u32,
    pub n: u32,
    pub new_mode: u32,
    pub pad1: [u8; 12],
}

pub type xcb_glx_rm_t = u32;
pub const XCB_GLX_RM_GL_RENDER: xcb_glx_rm_t = 0x1c00;
pub const XCB_GLX_RM_GL_FEEDBACK: xcb_glx_rm_t = 0x1c01;
pub const XCB_GLX_RM_GL_SELECT: xcb_glx_rm_t = 0x1c02;

pub const XCB_GLX_FINISH: u8 = 108;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_finish_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_finish_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_finish_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
}

pub const XCB_GLX_PIXEL_STOREF: u8 = 109;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_pixel_storef_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub pname: u32,
    pub datum: xcb_glx_float32_t,
}

pub const XCB_GLX_PIXEL_STOREI: u8 = 110;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_pixel_storei_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub pname: u32,
    pub datum: i32,
}

pub const XCB_GLX_READ_PIXELS: u8 = 111;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_read_pixels_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub format: u32,
    pub type_: u32,
    pub swap_bytes: u8,
    pub lsb_first: u8,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_read_pixels_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_read_pixels_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 24],
}

pub const XCB_GLX_GET_BOOLEANV: u8 = 112;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_booleanv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub pname: i32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_booleanv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_booleanv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: u8,
    pub pad2: [u8; 15],
}

pub const XCB_GLX_GET_CLIP_PLANE: u8 = 113;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_clip_plane_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub plane: i32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_clip_plane_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_clip_plane_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 24],
}

pub const XCB_GLX_GET_DOUBLEV: u8 = 114;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_doublev_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_doublev_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_doublev_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: xcb_glx_float64_t,
    pub pad2: [u8; 8],
}

pub const XCB_GLX_GET_ERROR: u8 = 115;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_error_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_error_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_error_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub error: i32,
}

pub const XCB_GLX_GET_FLOATV: u8 = 116;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_floatv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_floatv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_floatv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: xcb_glx_float32_t,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_INTEGERV: u8 = 117;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_integerv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_integerv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_integerv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: i32,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_LIGHTFV: u8 = 118;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_lightfv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub light: u32,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_lightfv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_lightfv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: xcb_glx_float32_t,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_LIGHTIV: u8 = 119;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_lightiv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub light: u32,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_lightiv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_lightiv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: i32,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_MAPDV: u8 = 120;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_mapdv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub target: u32,
    pub query: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_mapdv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_mapdv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: xcb_glx_float64_t,
    pub pad2: [u8; 8],
}

pub const XCB_GLX_GET_MAPFV: u8 = 121;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_mapfv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub target: u32,
    pub query: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_mapfv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_mapfv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: xcb_glx_float32_t,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_MAPIV: u8 = 122;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_mapiv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub target: u32,
    pub query: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_mapiv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_mapiv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: i32,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_MATERIALFV: u8 = 123;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_materialfv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub face: u32,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_materialfv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_materialfv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: xcb_glx_float32_t,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_MATERIALIV: u8 = 124;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_materialiv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub face: u32,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_materialiv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_materialiv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: i32,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_PIXEL_MAPFV: u8 = 125;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_pixel_mapfv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub map: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_pixel_mapfv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_pixel_mapfv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: xcb_glx_float32_t,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_PIXEL_MAPUIV: u8 = 126;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_pixel_mapuiv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub map: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_pixel_mapuiv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_pixel_mapuiv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: u32,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_PIXEL_MAPUSV: u8 = 127;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_pixel_mapusv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub map: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_pixel_mapusv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_pixel_mapusv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: u16,
    pub pad2: [u8; 16],
}

pub const XCB_GLX_GET_POLYGON_STIPPLE: u8 = 128;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_polygon_stipple_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub lsb_first: u8,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_polygon_stipple_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_polygon_stipple_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 24],
}

pub const XCB_GLX_GET_STRING: u8 = 129;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_string_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub name: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_string_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_string_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub pad2: [u8; 16],
}

pub const XCB_GLX_GET_TEX_ENVFV: u8 = 130;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_envfv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub target: u32,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_envfv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_envfv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: xcb_glx_float32_t,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_TEX_ENVIV: u8 = 131;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_enviv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub target: u32,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_enviv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_enviv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: i32,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_TEX_GENDV: u8 = 132;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_gendv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub coord: u32,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_gendv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_gendv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: xcb_glx_float64_t,
    pub pad2: [u8; 8],
}

pub const XCB_GLX_GET_TEX_GENFV: u8 = 133;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_genfv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub coord: u32,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_genfv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_genfv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: xcb_glx_float32_t,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_TEX_GENIV: u8 = 134;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_geniv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub coord: u32,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_geniv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_geniv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: i32,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_TEX_IMAGE: u8 = 135;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_image_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub target: u32,
    pub level: i32,
    pub format: u32,
    pub type_: u32,
    pub swap_bytes: u8,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_image_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_image_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 8],
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub pad2: [u8; 4],
}

pub const XCB_GLX_GET_TEX_PARAMETERFV: u8 = 136;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_parameterfv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub target: u32,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_parameterfv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_parameterfv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: xcb_glx_float32_t,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_TEX_PARAMETERIV: u8 = 137;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_parameteriv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub target: u32,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_parameteriv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_parameteriv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: i32,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_TEX_LEVEL_PARAMETERFV: u8 = 138;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_level_parameterfv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub target: u32,
    pub level: i32,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_level_parameterfv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_level_parameterfv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: xcb_glx_float32_t,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_TEX_LEVEL_PARAMETERIV: u8 = 139;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_level_parameteriv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub target: u32,
    pub level: i32,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_level_parameteriv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_level_parameteriv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: i32,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_IS_ENABLED: u8 = 140;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_is_enabled_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub capability: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_is_enabled_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_is_enabled_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: xcb_glx_bool32_t,
}

pub const XCB_GLX_IS_LIST: u8 = 141;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_is_list_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub list: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_is_list_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_is_list_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: xcb_glx_bool32_t,
}

pub const XCB_GLX_FLUSH: u8 = 142;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_flush_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
}

pub const XCB_GLX_ARE_TEXTURES_RESIDENT: u8 = 143;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_are_textures_resident_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub n: i32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_are_textures_resident_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_are_textures_resident_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: xcb_glx_bool32_t,
    pub pad1: [u8; 20],
}

pub const XCB_GLX_DELETE_TEXTURES: u8 = 144;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_delete_textures_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub n: i32,
}

pub const XCB_GLX_GEN_TEXTURES: u8 = 145;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_gen_textures_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub n: i32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_gen_textures_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_gen_textures_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 24],
}

pub const XCB_GLX_IS_TEXTURE: u8 = 146;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_is_texture_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub texture: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_is_texture_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_is_texture_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: xcb_glx_bool32_t,
}

pub const XCB_GLX_GET_COLOR_TABLE: u8 = 147;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_color_table_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub target: u32,
    pub format: u32,
    pub type_: u32,
    pub swap_bytes: u8,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_color_table_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_color_table_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 8],
    pub width: i32,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_COLOR_TABLE_PARAMETERFV: u8 = 148;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_color_table_parameterfv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub target: u32,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_color_table_parameterfv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_color_table_parameterfv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: xcb_glx_float32_t,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_COLOR_TABLE_PARAMETERIV: u8 = 149;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_color_table_parameteriv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub target: u32,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_color_table_parameteriv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_color_table_parameteriv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: i32,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_CONVOLUTION_FILTER: u8 = 150;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_convolution_filter_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub target: u32,
    pub format: u32,
    pub type_: u32,
    pub swap_bytes: u8,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_convolution_filter_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_convolution_filter_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 8],
    pub width: i32,
    pub height: i32,
    pub pad2: [u8; 8],
}

pub const XCB_GLX_GET_CONVOLUTION_PARAMETERFV: u8 = 151;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_convolution_parameterfv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub target: u32,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_convolution_parameterfv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_convolution_parameterfv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: xcb_glx_float32_t,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_CONVOLUTION_PARAMETERIV: u8 = 152;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_convolution_parameteriv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub target: u32,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_convolution_parameteriv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_convolution_parameteriv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: i32,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_SEPARABLE_FILTER: u8 = 153;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_separable_filter_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub target: u32,
    pub format: u32,
    pub type_: u32,
    pub swap_bytes: u8,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_separable_filter_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_separable_filter_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 8],
    pub row_w: i32,
    pub col_h: i32,
    pub pad2: [u8; 8],
}

pub const XCB_GLX_GET_HISTOGRAM: u8 = 154;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_histogram_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub target: u32,
    pub format: u32,
    pub type_: u32,
    pub swap_bytes: u8,
    pub reset: u8,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_histogram_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_histogram_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 8],
    pub width: i32,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_HISTOGRAM_PARAMETERFV: u8 = 155;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_histogram_parameterfv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub target: u32,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_histogram_parameterfv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_histogram_parameterfv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: xcb_glx_float32_t,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_HISTOGRAM_PARAMETERIV: u8 = 156;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_histogram_parameteriv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub target: u32,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_histogram_parameteriv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_histogram_parameteriv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: i32,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_MINMAX: u8 = 157;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_minmax_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub target: u32,
    pub format: u32,
    pub type_: u32,
    pub swap_bytes: u8,
    pub reset: u8,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_minmax_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_minmax_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 24],
}

pub const XCB_GLX_GET_MINMAX_PARAMETERFV: u8 = 158;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_minmax_parameterfv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub target: u32,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_minmax_parameterfv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_minmax_parameterfv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: xcb_glx_float32_t,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_MINMAX_PARAMETERIV: u8 = 159;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_minmax_parameteriv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub target: u32,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_minmax_parameteriv_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_minmax_parameteriv_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: i32,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_COMPRESSED_TEX_IMAGE_ARB: u8 = 160;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_compressed_tex_image_arb_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub target: u32,
    pub level: i32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_compressed_tex_image_arb_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_compressed_tex_image_arb_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 8],
    pub size: i32,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_DELETE_QUERIES_ARB: u8 = 161;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_delete_queries_arb_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub n: i32,
}

pub const XCB_GLX_GEN_QUERIES_ARB: u8 = 162;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_gen_queries_arb_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub n: i32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_gen_queries_arb_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_gen_queries_arb_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 24],
}

pub const XCB_GLX_IS_QUERY_ARB: u8 = 163;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_is_query_arb_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub id: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_is_query_arb_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_is_query_arb_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub ret_val: xcb_glx_bool32_t,
}

pub const XCB_GLX_GET_QUERYIV_ARB: u8 = 164;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_queryiv_arb_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub target: u32,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_queryiv_arb_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_queryiv_arb_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: i32,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_QUERY_OBJECTIV_ARB: u8 = 165;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_query_objectiv_arb_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub id: u32,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_query_objectiv_arb_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_query_objectiv_arb_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: i32,
    pub pad2: [u8; 12],
}

pub const XCB_GLX_GET_QUERY_OBJECTUIV_ARB: u8 = 166;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_query_objectuiv_arb_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub id: u32,
    pub pname: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_query_objectuiv_arb_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_query_objectuiv_arb_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 4],
    pub n: u32,
    pub datum: u32,
    pub pad2: [u8; 12],
}

impl XcbGlx {
    #[inline]
    pub unsafe fn xcb_glx_id(&self) -> *mut xcb_extension_t {
        sym!(self, xcb_glx_id)
    }

    #[inline]
    pub unsafe fn xcb_glx_pixmap_next(&self, i: *mut xcb_glx_pixmap_iterator_t) {
        sym!(self, xcb_glx_pixmap_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_glx_pixmap_end(
        &self,
        i: *mut xcb_glx_pixmap_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_pixmap_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_glx_context_next(&self, i: *mut xcb_glx_context_iterator_t) {
        sym!(self, xcb_glx_context_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_glx_context_end(
        &self,
        i: *mut xcb_glx_context_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_context_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_glx_pbuffer_next(&self, i: *mut xcb_glx_pbuffer_iterator_t) {
        sym!(self, xcb_glx_pbuffer_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_glx_pbuffer_end(
        &self,
        i: *mut xcb_glx_pbuffer_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_pbuffer_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_glx_window_next(&self, i: *mut xcb_glx_window_iterator_t) {
        sym!(self, xcb_glx_window_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_glx_window_end(
        &self,
        i: *mut xcb_glx_window_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_window_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_glx_fbconfig_next(&self, i: *mut xcb_glx_fbconfig_iterator_t) {
        sym!(self, xcb_glx_fbconfig_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_glx_fbconfig_end(
        &self,
        i: *mut xcb_glx_fbconfig_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_fbconfig_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_glx_drawable_next(&self, i: *mut xcb_glx_drawable_iterator_t) {
        sym!(self, xcb_glx_drawable_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_glx_drawable_end(
        &self,
        i: *mut xcb_glx_drawable_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_drawable_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_glx_float32_next(&self, i: *mut xcb_glx_float32_iterator_t) {
        sym!(self, xcb_glx_float32_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_glx_float32_end(
        &self,
        i: *mut xcb_glx_float32_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_float32_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_glx_float64_next(&self, i: *mut xcb_glx_float64_iterator_t) {
        sym!(self, xcb_glx_float64_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_glx_float64_end(
        &self,
        i: *mut xcb_glx_float64_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_float64_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_glx_bool32_next(&self, i: *mut xcb_glx_bool32_iterator_t) {
        sym!(self, xcb_glx_bool32_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_glx_bool32_end(
        &self,
        i: *mut xcb_glx_bool32_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_bool32_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_glx_context_tag_next(&self, i: *mut xcb_glx_context_tag_iterator_t) {
        sym!(self, xcb_glx_context_tag_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_glx_context_tag_end(
        &self,
        i: *mut xcb_glx_context_tag_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_context_tag_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_glx_render(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        data_len: u32,
        data: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_render)(c, context_tag, data_len, data)
    }

    #[inline]
    pub unsafe fn xcb_glx_render_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        data_len: u32,
        data: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_render_checked)(c, context_tag, data_len, data)
    }

    #[inline]
    pub unsafe fn xcb_glx_render_large(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        request_num: u16,
        request_total: u16,
        data_len: u32,
        data: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_render_large)(c, context_tag, request_num, request_total, data_len, data)
    }

    #[inline]
    pub unsafe fn xcb_glx_render_large_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        request_num: u16,
        request_total: u16,
        data_len: u32,
        data: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_render_large_checked)(
            c,
            context_tag,
            request_num,
            request_total,
            data_len,
            data,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_create_context(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_glx_context_t,
        visual: xcb_visualid_t,
        screen: u32,
        share_list: xcb_glx_context_t,
        is_direct: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_create_context)(c, context, visual, screen, share_list, is_direct)
    }

    #[inline]
    pub unsafe fn xcb_glx_create_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_glx_context_t,
        visual: xcb_visualid_t,
        screen: u32,
        share_list: xcb_glx_context_t,
        is_direct: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_create_context_checked)(
            c, context, visual, screen, share_list, is_direct,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_destroy_context(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_glx_context_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_destroy_context)(c, context)
    }

    #[inline]
    pub unsafe fn xcb_glx_destroy_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_glx_context_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_destroy_context_checked)(c, context)
    }

    #[inline]
    pub unsafe fn xcb_glx_make_current_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_make_current_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_make_current_reply_t {
        sym!(self, xcb_glx_make_current_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_make_current(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_glx_drawable_t,
        context: xcb_glx_context_t,
        old_context_tag: xcb_glx_context_tag_t,
    ) -> xcb_glx_make_current_cookie_t {
        sym!(self, xcb_glx_make_current)(c, drawable, context, old_context_tag)
    }

    #[inline]
    pub unsafe fn xcb_glx_make_current_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_glx_drawable_t,
        context: xcb_glx_context_t,
        old_context_tag: xcb_glx_context_tag_t,
    ) -> xcb_glx_make_current_cookie_t {
        sym!(self, xcb_glx_make_current_unchecked)(c, drawable, context, old_context_tag)
    }

    #[inline]
    pub unsafe fn xcb_glx_is_direct_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_is_direct_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_is_direct_reply_t {
        sym!(self, xcb_glx_is_direct_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_is_direct(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_glx_context_t,
    ) -> xcb_glx_is_direct_cookie_t {
        sym!(self, xcb_glx_is_direct)(c, context)
    }

    #[inline]
    pub unsafe fn xcb_glx_is_direct_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_glx_context_t,
    ) -> xcb_glx_is_direct_cookie_t {
        sym!(self, xcb_glx_is_direct_unchecked)(c, context)
    }

    #[inline]
    pub unsafe fn xcb_glx_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_query_version_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_query_version_reply_t {
        sym!(self, xcb_glx_query_version_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_query_version(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
    ) -> xcb_glx_query_version_cookie_t {
        sym!(self, xcb_glx_query_version)(c, major_version, minor_version)
    }

    #[inline]
    pub unsafe fn xcb_glx_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
    ) -> xcb_glx_query_version_cookie_t {
        sym!(self, xcb_glx_query_version_unchecked)(c, major_version, minor_version)
    }

    #[inline]
    pub unsafe fn xcb_glx_wait_gl(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_wait_gl)(c, context_tag)
    }

    #[inline]
    pub unsafe fn xcb_glx_wait_gl_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_wait_gl_checked)(c, context_tag)
    }

    #[inline]
    pub unsafe fn xcb_glx_wait_x(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_wait_x)(c, context_tag)
    }

    #[inline]
    pub unsafe fn xcb_glx_wait_x_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_wait_x_checked)(c, context_tag)
    }

    #[inline]
    pub unsafe fn xcb_glx_copy_context(
        &self,
        c: *mut xcb_connection_t,
        src: xcb_glx_context_t,
        dest: xcb_glx_context_t,
        mask: u32,
        src_context_tag: xcb_glx_context_tag_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_copy_context)(c, src, dest, mask, src_context_tag)
    }

    #[inline]
    pub unsafe fn xcb_glx_copy_context_checked(
        &self,
        c: *mut xcb_connection_t,
        src: xcb_glx_context_t,
        dest: xcb_glx_context_t,
        mask: u32,
        src_context_tag: xcb_glx_context_tag_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_copy_context_checked)(c, src, dest, mask, src_context_tag)
    }

    #[inline]
    pub unsafe fn xcb_glx_swap_buffers(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        drawable: xcb_glx_drawable_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_swap_buffers)(c, context_tag, drawable)
    }

    #[inline]
    pub unsafe fn xcb_glx_swap_buffers_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        drawable: xcb_glx_drawable_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_swap_buffers_checked)(c, context_tag, drawable)
    }

    #[inline]
    pub unsafe fn xcb_glx_use_x_font(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        font: xcb_font_t,
        first: u32,
        count: u32,
        list_base: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_use_x_font)(c, context_tag, font, first, count, list_base)
    }

    #[inline]
    pub unsafe fn xcb_glx_use_x_font_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        font: xcb_font_t,
        first: u32,
        count: u32,
        list_base: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_use_x_font_checked)(c, context_tag, font, first, count, list_base)
    }

    #[inline]
    pub unsafe fn xcb_glx_create_glx_pixmap(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        visual: xcb_visualid_t,
        pixmap: xcb_pixmap_t,
        glx_pixmap: xcb_glx_pixmap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_create_glx_pixmap)(c, screen, visual, pixmap, glx_pixmap)
    }

    #[inline]
    pub unsafe fn xcb_glx_create_glx_pixmap_checked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        visual: xcb_visualid_t,
        pixmap: xcb_pixmap_t,
        glx_pixmap: xcb_glx_pixmap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_create_glx_pixmap_checked)(c, screen, visual, pixmap, glx_pixmap)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_visual_configs_property_list(
        &self,
        R: *const xcb_glx_get_visual_configs_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_get_visual_configs_property_list)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_visual_configs_property_list_length(
        &self,
        R: *const xcb_glx_get_visual_configs_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_visual_configs_property_list_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_visual_configs_property_list_end(
        &self,
        R: *const xcb_glx_get_visual_configs_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_visual_configs_property_list_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_visual_configs_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_visual_configs_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_visual_configs_reply_t {
        sym!(self, xcb_glx_get_visual_configs_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_visual_configs(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_glx_get_visual_configs_cookie_t {
        sym!(self, xcb_glx_get_visual_configs)(c, screen)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_visual_configs_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_glx_get_visual_configs_cookie_t {
        sym!(self, xcb_glx_get_visual_configs_unchecked)(c, screen)
    }

    #[inline]
    pub unsafe fn xcb_glx_destroy_glx_pixmap(
        &self,
        c: *mut xcb_connection_t,
        glx_pixmap: xcb_glx_pixmap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_destroy_glx_pixmap)(c, glx_pixmap)
    }

    #[inline]
    pub unsafe fn xcb_glx_destroy_glx_pixmap_checked(
        &self,
        c: *mut xcb_connection_t,
        glx_pixmap: xcb_glx_pixmap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_destroy_glx_pixmap_checked)(c, glx_pixmap)
    }

    #[inline]
    pub unsafe fn xcb_glx_vendor_private(
        &self,
        c: *mut xcb_connection_t,
        vendor_code: u32,
        context_tag: xcb_glx_context_tag_t,
        data_len: u32,
        data: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_vendor_private)(c, vendor_code, context_tag, data_len, data)
    }

    #[inline]
    pub unsafe fn xcb_glx_vendor_private_checked(
        &self,
        c: *mut xcb_connection_t,
        vendor_code: u32,
        context_tag: xcb_glx_context_tag_t,
        data_len: u32,
        data: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_vendor_private_checked)(c, vendor_code, context_tag, data_len, data)
    }

    #[inline]
    pub unsafe fn xcb_glx_vendor_private_with_reply_data_2(
        &self,
        R: *const xcb_glx_vendor_private_with_reply_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_glx_vendor_private_with_reply_data_2)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_vendor_private_with_reply_data_2_length(
        &self,
        R: *const xcb_glx_vendor_private_with_reply_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_vendor_private_with_reply_data_2_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_vendor_private_with_reply_data_2_end(
        &self,
        R: *const xcb_glx_vendor_private_with_reply_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_vendor_private_with_reply_data_2_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_vendor_private_with_reply_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_vendor_private_with_reply_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_vendor_private_with_reply_reply_t {
        sym!(self, xcb_glx_vendor_private_with_reply_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_vendor_private_with_reply(
        &self,
        c: *mut xcb_connection_t,
        vendor_code: u32,
        context_tag: xcb_glx_context_tag_t,
        data_len: u32,
        data: *const u8,
    ) -> xcb_glx_vendor_private_with_reply_cookie_t {
        sym!(self, xcb_glx_vendor_private_with_reply)(c, vendor_code, context_tag, data_len, data)
    }

    #[inline]
    pub unsafe fn xcb_glx_vendor_private_with_reply_unchecked(
        &self,
        c: *mut xcb_connection_t,
        vendor_code: u32,
        context_tag: xcb_glx_context_tag_t,
        data_len: u32,
        data: *const u8,
    ) -> xcb_glx_vendor_private_with_reply_cookie_t {
        sym!(self, xcb_glx_vendor_private_with_reply_unchecked)(
            c,
            vendor_code,
            context_tag,
            data_len,
            data,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_query_extensions_string_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_query_extensions_string_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_query_extensions_string_reply_t {
        sym!(self, xcb_glx_query_extensions_string_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_query_extensions_string(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_glx_query_extensions_string_cookie_t {
        sym!(self, xcb_glx_query_extensions_string)(c, screen)
    }

    #[inline]
    pub unsafe fn xcb_glx_query_extensions_string_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_glx_query_extensions_string_cookie_t {
        sym!(self, xcb_glx_query_extensions_string_unchecked)(c, screen)
    }

    #[inline]
    pub unsafe fn xcb_glx_query_server_string_string(
        &self,
        R: *const xcb_glx_query_server_string_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_glx_query_server_string_string)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_query_server_string_string_length(
        &self,
        R: *const xcb_glx_query_server_string_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_query_server_string_string_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_query_server_string_string_end(
        &self,
        R: *const xcb_glx_query_server_string_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_query_server_string_string_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_query_server_string_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_query_server_string_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_query_server_string_reply_t {
        sym!(self, xcb_glx_query_server_string_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_query_server_string(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        name: u32,
    ) -> xcb_glx_query_server_string_cookie_t {
        sym!(self, xcb_glx_query_server_string)(c, screen, name)
    }

    #[inline]
    pub unsafe fn xcb_glx_query_server_string_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        name: u32,
    ) -> xcb_glx_query_server_string_cookie_t {
        sym!(self, xcb_glx_query_server_string_unchecked)(c, screen, name)
    }

    #[inline]
    pub unsafe fn xcb_glx_client_info(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
        str_len: u32,
        string: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_client_info)(c, major_version, minor_version, str_len, string)
    }

    #[inline]
    pub unsafe fn xcb_glx_client_info_checked(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
        str_len: u32,
        string: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_client_info_checked)(c, major_version, minor_version, str_len, string)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_fb_configs_property_list(
        &self,
        R: *const xcb_glx_get_fb_configs_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_get_fb_configs_property_list)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_fb_configs_property_list_length(
        &self,
        R: *const xcb_glx_get_fb_configs_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_fb_configs_property_list_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_fb_configs_property_list_end(
        &self,
        R: *const xcb_glx_get_fb_configs_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_fb_configs_property_list_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_fb_configs_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_fb_configs_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_fb_configs_reply_t {
        sym!(self, xcb_glx_get_fb_configs_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_fb_configs(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_glx_get_fb_configs_cookie_t {
        sym!(self, xcb_glx_get_fb_configs)(c, screen)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_fb_configs_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_glx_get_fb_configs_cookie_t {
        sym!(self, xcb_glx_get_fb_configs_unchecked)(c, screen)
    }

    #[inline]
    pub unsafe fn xcb_glx_create_pixmap(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        fbconfig: xcb_glx_fbconfig_t,
        pixmap: xcb_pixmap_t,
        glx_pixmap: xcb_glx_pixmap_t,
        num_attribs: u32,
        attribs: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_create_pixmap)(
            c,
            screen,
            fbconfig,
            pixmap,
            glx_pixmap,
            num_attribs,
            attribs,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_create_pixmap_checked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        fbconfig: xcb_glx_fbconfig_t,
        pixmap: xcb_pixmap_t,
        glx_pixmap: xcb_glx_pixmap_t,
        num_attribs: u32,
        attribs: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_create_pixmap_checked)(
            c,
            screen,
            fbconfig,
            pixmap,
            glx_pixmap,
            num_attribs,
            attribs,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_destroy_pixmap(
        &self,
        c: *mut xcb_connection_t,
        glx_pixmap: xcb_glx_pixmap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_destroy_pixmap)(c, glx_pixmap)
    }

    #[inline]
    pub unsafe fn xcb_glx_destroy_pixmap_checked(
        &self,
        c: *mut xcb_connection_t,
        glx_pixmap: xcb_glx_pixmap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_destroy_pixmap_checked)(c, glx_pixmap)
    }

    #[inline]
    pub unsafe fn xcb_glx_create_new_context(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_glx_context_t,
        fbconfig: xcb_glx_fbconfig_t,
        screen: u32,
        render_type: u32,
        share_list: xcb_glx_context_t,
        is_direct: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_create_new_context)(
            c,
            context,
            fbconfig,
            screen,
            render_type,
            share_list,
            is_direct,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_create_new_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_glx_context_t,
        fbconfig: xcb_glx_fbconfig_t,
        screen: u32,
        render_type: u32,
        share_list: xcb_glx_context_t,
        is_direct: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_create_new_context_checked)(
            c,
            context,
            fbconfig,
            screen,
            render_type,
            share_list,
            is_direct,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_query_context_attribs(
        &self,
        R: *const xcb_glx_query_context_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_query_context_attribs)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_query_context_attribs_length(
        &self,
        R: *const xcb_glx_query_context_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_query_context_attribs_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_query_context_attribs_end(
        &self,
        R: *const xcb_glx_query_context_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_query_context_attribs_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_query_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_query_context_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_query_context_reply_t {
        sym!(self, xcb_glx_query_context_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_query_context(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_glx_context_t,
    ) -> xcb_glx_query_context_cookie_t {
        sym!(self, xcb_glx_query_context)(c, context)
    }

    #[inline]
    pub unsafe fn xcb_glx_query_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_glx_context_t,
    ) -> xcb_glx_query_context_cookie_t {
        sym!(self, xcb_glx_query_context_unchecked)(c, context)
    }

    #[inline]
    pub unsafe fn xcb_glx_make_context_current_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_make_context_current_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_make_context_current_reply_t {
        sym!(self, xcb_glx_make_context_current_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_make_context_current(
        &self,
        c: *mut xcb_connection_t,
        old_context_tag: xcb_glx_context_tag_t,
        drawable: xcb_glx_drawable_t,
        read_drawable: xcb_glx_drawable_t,
        context: xcb_glx_context_t,
    ) -> xcb_glx_make_context_current_cookie_t {
        sym!(self, xcb_glx_make_context_current)(
            c,
            old_context_tag,
            drawable,
            read_drawable,
            context,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_make_context_current_unchecked(
        &self,
        c: *mut xcb_connection_t,
        old_context_tag: xcb_glx_context_tag_t,
        drawable: xcb_glx_drawable_t,
        read_drawable: xcb_glx_drawable_t,
        context: xcb_glx_context_t,
    ) -> xcb_glx_make_context_current_cookie_t {
        sym!(self, xcb_glx_make_context_current_unchecked)(
            c,
            old_context_tag,
            drawable,
            read_drawable,
            context,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_create_pbuffer(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        fbconfig: xcb_glx_fbconfig_t,
        pbuffer: xcb_glx_pbuffer_t,
        num_attribs: u32,
        attribs: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_create_pbuffer)(c, screen, fbconfig, pbuffer, num_attribs, attribs)
    }

    #[inline]
    pub unsafe fn xcb_glx_create_pbuffer_checked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        fbconfig: xcb_glx_fbconfig_t,
        pbuffer: xcb_glx_pbuffer_t,
        num_attribs: u32,
        attribs: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_create_pbuffer_checked)(
            c,
            screen,
            fbconfig,
            pbuffer,
            num_attribs,
            attribs,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_destroy_pbuffer(
        &self,
        c: *mut xcb_connection_t,
        pbuffer: xcb_glx_pbuffer_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_destroy_pbuffer)(c, pbuffer)
    }

    #[inline]
    pub unsafe fn xcb_glx_destroy_pbuffer_checked(
        &self,
        c: *mut xcb_connection_t,
        pbuffer: xcb_glx_pbuffer_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_destroy_pbuffer_checked)(c, pbuffer)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_drawable_attributes_attribs(
        &self,
        R: *const xcb_glx_get_drawable_attributes_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_get_drawable_attributes_attribs)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_drawable_attributes_attribs_length(
        &self,
        R: *const xcb_glx_get_drawable_attributes_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_drawable_attributes_attribs_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_drawable_attributes_attribs_end(
        &self,
        R: *const xcb_glx_get_drawable_attributes_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_drawable_attributes_attribs_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_drawable_attributes_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_drawable_attributes_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_drawable_attributes_reply_t {
        sym!(self, xcb_glx_get_drawable_attributes_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_drawable_attributes(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_glx_drawable_t,
    ) -> xcb_glx_get_drawable_attributes_cookie_t {
        sym!(self, xcb_glx_get_drawable_attributes)(c, drawable)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_drawable_attributes_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_glx_drawable_t,
    ) -> xcb_glx_get_drawable_attributes_cookie_t {
        sym!(self, xcb_glx_get_drawable_attributes_unchecked)(c, drawable)
    }

    #[inline]
    pub unsafe fn xcb_glx_change_drawable_attributes(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_glx_drawable_t,
        num_attribs: u32,
        attribs: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_change_drawable_attributes)(c, drawable, num_attribs, attribs)
    }

    #[inline]
    pub unsafe fn xcb_glx_change_drawable_attributes_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_glx_drawable_t,
        num_attribs: u32,
        attribs: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_change_drawable_attributes_checked)(c, drawable, num_attribs, attribs)
    }

    #[inline]
    pub unsafe fn xcb_glx_create_window(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        fbconfig: xcb_glx_fbconfig_t,
        window: xcb_window_t,
        glx_window: xcb_glx_window_t,
        num_attribs: u32,
        attribs: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_create_window)(
            c,
            screen,
            fbconfig,
            window,
            glx_window,
            num_attribs,
            attribs,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_create_window_checked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        fbconfig: xcb_glx_fbconfig_t,
        window: xcb_window_t,
        glx_window: xcb_glx_window_t,
        num_attribs: u32,
        attribs: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_create_window_checked)(
            c,
            screen,
            fbconfig,
            window,
            glx_window,
            num_attribs,
            attribs,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_delete_window(
        &self,
        c: *mut xcb_connection_t,
        glxwindow: xcb_glx_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_delete_window)(c, glxwindow)
    }

    #[inline]
    pub unsafe fn xcb_glx_delete_window_checked(
        &self,
        c: *mut xcb_connection_t,
        glxwindow: xcb_glx_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_delete_window_checked)(c, glxwindow)
    }

    #[inline]
    pub unsafe fn xcb_glx_set_client_info_arb(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
        num_versions: u32,
        gl_str_len: u32,
        glx_str_len: u32,
        gl_versions: *const u32,
        gl_extension_string: *const c_char,
        glx_extension_string: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_set_client_info_arb)(
            c,
            major_version,
            minor_version,
            num_versions,
            gl_str_len,
            glx_str_len,
            gl_versions,
            gl_extension_string,
            glx_extension_string,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_set_client_info_arb_checked(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
        num_versions: u32,
        gl_str_len: u32,
        glx_str_len: u32,
        gl_versions: *const u32,
        gl_extension_string: *const c_char,
        glx_extension_string: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_set_client_info_arb_checked)(
            c,
            major_version,
            minor_version,
            num_versions,
            gl_str_len,
            glx_str_len,
            gl_versions,
            gl_extension_string,
            glx_extension_string,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_create_context_attribs_arb(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_glx_context_t,
        fbconfig: xcb_glx_fbconfig_t,
        screen: u32,
        share_list: xcb_glx_context_t,
        is_direct: u8,
        num_attribs: u32,
        attribs: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_create_context_attribs_arb)(
            c,
            context,
            fbconfig,
            screen,
            share_list,
            is_direct,
            num_attribs,
            attribs,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_create_context_attribs_arb_checked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_glx_context_t,
        fbconfig: xcb_glx_fbconfig_t,
        screen: u32,
        share_list: xcb_glx_context_t,
        is_direct: u8,
        num_attribs: u32,
        attribs: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_create_context_attribs_arb_checked)(
            c,
            context,
            fbconfig,
            screen,
            share_list,
            is_direct,
            num_attribs,
            attribs,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_set_client_info_2arb(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
        num_versions: u32,
        gl_str_len: u32,
        glx_str_len: u32,
        gl_versions: *const u32,
        gl_extension_string: *const c_char,
        glx_extension_string: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_set_client_info_2arb)(
            c,
            major_version,
            minor_version,
            num_versions,
            gl_str_len,
            glx_str_len,
            gl_versions,
            gl_extension_string,
            glx_extension_string,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_set_client_info_2arb_checked(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
        num_versions: u32,
        gl_str_len: u32,
        glx_str_len: u32,
        gl_versions: *const u32,
        gl_extension_string: *const c_char,
        glx_extension_string: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_set_client_info_2arb_checked)(
            c,
            major_version,
            minor_version,
            num_versions,
            gl_str_len,
            glx_str_len,
            gl_versions,
            gl_extension_string,
            glx_extension_string,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_new_list(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        list: u32,
        mode: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_new_list)(c, context_tag, list, mode)
    }

    #[inline]
    pub unsafe fn xcb_glx_new_list_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        list: u32,
        mode: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_new_list_checked)(c, context_tag, list, mode)
    }

    #[inline]
    pub unsafe fn xcb_glx_end_list(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_end_list)(c, context_tag)
    }

    #[inline]
    pub unsafe fn xcb_glx_end_list_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_end_list_checked)(c, context_tag)
    }

    #[inline]
    pub unsafe fn xcb_glx_delete_lists(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        list: u32,
        range: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_delete_lists)(c, context_tag, list, range)
    }

    #[inline]
    pub unsafe fn xcb_glx_delete_lists_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        list: u32,
        range: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_delete_lists_checked)(c, context_tag, list, range)
    }

    #[inline]
    pub unsafe fn xcb_glx_gen_lists_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_gen_lists_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_gen_lists_reply_t {
        sym!(self, xcb_glx_gen_lists_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_gen_lists(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        range: i32,
    ) -> xcb_glx_gen_lists_cookie_t {
        sym!(self, xcb_glx_gen_lists)(c, context_tag, range)
    }

    #[inline]
    pub unsafe fn xcb_glx_gen_lists_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        range: i32,
    ) -> xcb_glx_gen_lists_cookie_t {
        sym!(self, xcb_glx_gen_lists_unchecked)(c, context_tag, range)
    }

    #[inline]
    pub unsafe fn xcb_glx_feedback_buffer(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        size: i32,
        type_: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_feedback_buffer)(c, context_tag, size, type_)
    }

    #[inline]
    pub unsafe fn xcb_glx_feedback_buffer_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        size: i32,
        type_: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_feedback_buffer_checked)(c, context_tag, size, type_)
    }

    #[inline]
    pub unsafe fn xcb_glx_select_buffer(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        size: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_select_buffer)(c, context_tag, size)
    }

    #[inline]
    pub unsafe fn xcb_glx_select_buffer_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        size: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_select_buffer_checked)(c, context_tag, size)
    }

    #[inline]
    pub unsafe fn xcb_glx_render_mode_data(
        &self,
        R: *const xcb_glx_render_mode_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_render_mode_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_render_mode_data_length(
        &self,
        R: *const xcb_glx_render_mode_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_render_mode_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_render_mode_data_end(
        &self,
        R: *const xcb_glx_render_mode_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_render_mode_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_render_mode_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_render_mode_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_render_mode_reply_t {
        sym!(self, xcb_glx_render_mode_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_render_mode(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        mode: u32,
    ) -> xcb_glx_render_mode_cookie_t {
        sym!(self, xcb_glx_render_mode)(c, context_tag, mode)
    }

    #[inline]
    pub unsafe fn xcb_glx_render_mode_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        mode: u32,
    ) -> xcb_glx_render_mode_cookie_t {
        sym!(self, xcb_glx_render_mode_unchecked)(c, context_tag, mode)
    }

    #[inline]
    pub unsafe fn xcb_glx_finish_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_finish_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_finish_reply_t {
        sym!(self, xcb_glx_finish_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_finish(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
    ) -> xcb_glx_finish_cookie_t {
        sym!(self, xcb_glx_finish)(c, context_tag)
    }

    #[inline]
    pub unsafe fn xcb_glx_finish_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
    ) -> xcb_glx_finish_cookie_t {
        sym!(self, xcb_glx_finish_unchecked)(c, context_tag)
    }

    #[inline]
    pub unsafe fn xcb_glx_pixel_storef(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        pname: u32,
        datum: xcb_glx_float32_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_pixel_storef)(c, context_tag, pname, datum)
    }

    #[inline]
    pub unsafe fn xcb_glx_pixel_storef_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        pname: u32,
        datum: xcb_glx_float32_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_pixel_storef_checked)(c, context_tag, pname, datum)
    }

    #[inline]
    pub unsafe fn xcb_glx_pixel_storei(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        pname: u32,
        datum: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_pixel_storei)(c, context_tag, pname, datum)
    }

    #[inline]
    pub unsafe fn xcb_glx_pixel_storei_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        pname: u32,
        datum: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_pixel_storei_checked)(c, context_tag, pname, datum)
    }

    #[inline]
    pub unsafe fn xcb_glx_read_pixels_data(
        &self,
        R: *const xcb_glx_read_pixels_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_glx_read_pixels_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_read_pixels_data_length(
        &self,
        R: *const xcb_glx_read_pixels_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_read_pixels_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_read_pixels_data_end(
        &self,
        R: *const xcb_glx_read_pixels_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_read_pixels_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_read_pixels_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_read_pixels_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_read_pixels_reply_t {
        sym!(self, xcb_glx_read_pixels_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_read_pixels(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        swap_bytes: u8,
        lsb_first: u8,
    ) -> xcb_glx_read_pixels_cookie_t {
        sym!(self, xcb_glx_read_pixels)(
            c,
            context_tag,
            x,
            y,
            width,
            height,
            format,
            type_,
            swap_bytes,
            lsb_first,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_read_pixels_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        swap_bytes: u8,
        lsb_first: u8,
    ) -> xcb_glx_read_pixels_cookie_t {
        sym!(self, xcb_glx_read_pixels_unchecked)(
            c,
            context_tag,
            x,
            y,
            width,
            height,
            format,
            type_,
            swap_bytes,
            lsb_first,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_get_booleanv_data(
        &self,
        R: *const xcb_glx_get_booleanv_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_glx_get_booleanv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_booleanv_data_length(
        &self,
        R: *const xcb_glx_get_booleanv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_booleanv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_booleanv_data_end(
        &self,
        R: *const xcb_glx_get_booleanv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_booleanv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_booleanv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_booleanv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_booleanv_reply_t {
        sym!(self, xcb_glx_get_booleanv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_booleanv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        pname: i32,
    ) -> xcb_glx_get_booleanv_cookie_t {
        sym!(self, xcb_glx_get_booleanv)(c, context_tag, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_booleanv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        pname: i32,
    ) -> xcb_glx_get_booleanv_cookie_t {
        sym!(self, xcb_glx_get_booleanv_unchecked)(c, context_tag, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_clip_plane_data(
        &self,
        R: *const xcb_glx_get_clip_plane_reply_t,
    ) -> *mut xcb_glx_float64_t {
        sym!(self, xcb_glx_get_clip_plane_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_clip_plane_data_length(
        &self,
        R: *const xcb_glx_get_clip_plane_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_clip_plane_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_clip_plane_data_end(
        &self,
        R: *const xcb_glx_get_clip_plane_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_clip_plane_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_clip_plane_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_clip_plane_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_clip_plane_reply_t {
        sym!(self, xcb_glx_get_clip_plane_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_clip_plane(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        plane: i32,
    ) -> xcb_glx_get_clip_plane_cookie_t {
        sym!(self, xcb_glx_get_clip_plane)(c, context_tag, plane)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_clip_plane_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        plane: i32,
    ) -> xcb_glx_get_clip_plane_cookie_t {
        sym!(self, xcb_glx_get_clip_plane_unchecked)(c, context_tag, plane)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_doublev_data(
        &self,
        R: *const xcb_glx_get_doublev_reply_t,
    ) -> *mut xcb_glx_float64_t {
        sym!(self, xcb_glx_get_doublev_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_doublev_data_length(
        &self,
        R: *const xcb_glx_get_doublev_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_doublev_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_doublev_data_end(
        &self,
        R: *const xcb_glx_get_doublev_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_doublev_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_doublev_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_doublev_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_doublev_reply_t {
        sym!(self, xcb_glx_get_doublev_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_doublev(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        pname: u32,
    ) -> xcb_glx_get_doublev_cookie_t {
        sym!(self, xcb_glx_get_doublev)(c, context_tag, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_doublev_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        pname: u32,
    ) -> xcb_glx_get_doublev_cookie_t {
        sym!(self, xcb_glx_get_doublev_unchecked)(c, context_tag, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_error_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_error_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_error_reply_t {
        sym!(self, xcb_glx_get_error_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_error(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
    ) -> xcb_glx_get_error_cookie_t {
        sym!(self, xcb_glx_get_error)(c, context_tag)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_error_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
    ) -> xcb_glx_get_error_cookie_t {
        sym!(self, xcb_glx_get_error_unchecked)(c, context_tag)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_floatv_data(
        &self,
        R: *const xcb_glx_get_floatv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_floatv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_floatv_data_length(
        &self,
        R: *const xcb_glx_get_floatv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_floatv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_floatv_data_end(
        &self,
        R: *const xcb_glx_get_floatv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_floatv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_floatv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_floatv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_floatv_reply_t {
        sym!(self, xcb_glx_get_floatv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_floatv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        pname: u32,
    ) -> xcb_glx_get_floatv_cookie_t {
        sym!(self, xcb_glx_get_floatv)(c, context_tag, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_floatv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        pname: u32,
    ) -> xcb_glx_get_floatv_cookie_t {
        sym!(self, xcb_glx_get_floatv_unchecked)(c, context_tag, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_integerv_data(
        &self,
        R: *const xcb_glx_get_integerv_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_integerv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_integerv_data_length(
        &self,
        R: *const xcb_glx_get_integerv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_integerv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_integerv_data_end(
        &self,
        R: *const xcb_glx_get_integerv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_integerv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_integerv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_integerv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_integerv_reply_t {
        sym!(self, xcb_glx_get_integerv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_integerv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        pname: u32,
    ) -> xcb_glx_get_integerv_cookie_t {
        sym!(self, xcb_glx_get_integerv)(c, context_tag, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_integerv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        pname: u32,
    ) -> xcb_glx_get_integerv_cookie_t {
        sym!(self, xcb_glx_get_integerv_unchecked)(c, context_tag, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_lightfv_data(
        &self,
        R: *const xcb_glx_get_lightfv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_lightfv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_lightfv_data_length(
        &self,
        R: *const xcb_glx_get_lightfv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_lightfv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_lightfv_data_end(
        &self,
        R: *const xcb_glx_get_lightfv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_lightfv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_lightfv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_lightfv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_lightfv_reply_t {
        sym!(self, xcb_glx_get_lightfv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_lightfv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        light: u32,
        pname: u32,
    ) -> xcb_glx_get_lightfv_cookie_t {
        sym!(self, xcb_glx_get_lightfv)(c, context_tag, light, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_lightfv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        light: u32,
        pname: u32,
    ) -> xcb_glx_get_lightfv_cookie_t {
        sym!(self, xcb_glx_get_lightfv_unchecked)(c, context_tag, light, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_lightiv_data(
        &self,
        R: *const xcb_glx_get_lightiv_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_lightiv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_lightiv_data_length(
        &self,
        R: *const xcb_glx_get_lightiv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_lightiv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_lightiv_data_end(
        &self,
        R: *const xcb_glx_get_lightiv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_lightiv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_lightiv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_lightiv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_lightiv_reply_t {
        sym!(self, xcb_glx_get_lightiv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_lightiv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        light: u32,
        pname: u32,
    ) -> xcb_glx_get_lightiv_cookie_t {
        sym!(self, xcb_glx_get_lightiv)(c, context_tag, light, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_lightiv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        light: u32,
        pname: u32,
    ) -> xcb_glx_get_lightiv_cookie_t {
        sym!(self, xcb_glx_get_lightiv_unchecked)(c, context_tag, light, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_mapdv_data(
        &self,
        R: *const xcb_glx_get_mapdv_reply_t,
    ) -> *mut xcb_glx_float64_t {
        sym!(self, xcb_glx_get_mapdv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_mapdv_data_length(
        &self,
        R: *const xcb_glx_get_mapdv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_mapdv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_mapdv_data_end(
        &self,
        R: *const xcb_glx_get_mapdv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_mapdv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_mapdv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_mapdv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_mapdv_reply_t {
        sym!(self, xcb_glx_get_mapdv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_mapdv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        query: u32,
    ) -> xcb_glx_get_mapdv_cookie_t {
        sym!(self, xcb_glx_get_mapdv)(c, context_tag, target, query)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_mapdv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        query: u32,
    ) -> xcb_glx_get_mapdv_cookie_t {
        sym!(self, xcb_glx_get_mapdv_unchecked)(c, context_tag, target, query)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_mapfv_data(
        &self,
        R: *const xcb_glx_get_mapfv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_mapfv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_mapfv_data_length(
        &self,
        R: *const xcb_glx_get_mapfv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_mapfv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_mapfv_data_end(
        &self,
        R: *const xcb_glx_get_mapfv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_mapfv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_mapfv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_mapfv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_mapfv_reply_t {
        sym!(self, xcb_glx_get_mapfv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_mapfv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        query: u32,
    ) -> xcb_glx_get_mapfv_cookie_t {
        sym!(self, xcb_glx_get_mapfv)(c, context_tag, target, query)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_mapfv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        query: u32,
    ) -> xcb_glx_get_mapfv_cookie_t {
        sym!(self, xcb_glx_get_mapfv_unchecked)(c, context_tag, target, query)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_mapiv_data(&self, R: *const xcb_glx_get_mapiv_reply_t) -> *mut i32 {
        sym!(self, xcb_glx_get_mapiv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_mapiv_data_length(
        &self,
        R: *const xcb_glx_get_mapiv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_mapiv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_mapiv_data_end(
        &self,
        R: *const xcb_glx_get_mapiv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_mapiv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_mapiv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_mapiv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_mapiv_reply_t {
        sym!(self, xcb_glx_get_mapiv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_mapiv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        query: u32,
    ) -> xcb_glx_get_mapiv_cookie_t {
        sym!(self, xcb_glx_get_mapiv)(c, context_tag, target, query)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_mapiv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        query: u32,
    ) -> xcb_glx_get_mapiv_cookie_t {
        sym!(self, xcb_glx_get_mapiv_unchecked)(c, context_tag, target, query)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_materialfv_data(
        &self,
        R: *const xcb_glx_get_materialfv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_materialfv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_materialfv_data_length(
        &self,
        R: *const xcb_glx_get_materialfv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_materialfv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_materialfv_data_end(
        &self,
        R: *const xcb_glx_get_materialfv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_materialfv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_materialfv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_materialfv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_materialfv_reply_t {
        sym!(self, xcb_glx_get_materialfv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_materialfv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        face: u32,
        pname: u32,
    ) -> xcb_glx_get_materialfv_cookie_t {
        sym!(self, xcb_glx_get_materialfv)(c, context_tag, face, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_materialfv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        face: u32,
        pname: u32,
    ) -> xcb_glx_get_materialfv_cookie_t {
        sym!(self, xcb_glx_get_materialfv_unchecked)(c, context_tag, face, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_materialiv_data(
        &self,
        R: *const xcb_glx_get_materialiv_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_materialiv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_materialiv_data_length(
        &self,
        R: *const xcb_glx_get_materialiv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_materialiv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_materialiv_data_end(
        &self,
        R: *const xcb_glx_get_materialiv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_materialiv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_materialiv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_materialiv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_materialiv_reply_t {
        sym!(self, xcb_glx_get_materialiv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_materialiv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        face: u32,
        pname: u32,
    ) -> xcb_glx_get_materialiv_cookie_t {
        sym!(self, xcb_glx_get_materialiv)(c, context_tag, face, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_materialiv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        face: u32,
        pname: u32,
    ) -> xcb_glx_get_materialiv_cookie_t {
        sym!(self, xcb_glx_get_materialiv_unchecked)(c, context_tag, face, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_pixel_mapfv_data(
        &self,
        R: *const xcb_glx_get_pixel_mapfv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_pixel_mapfv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_pixel_mapfv_data_length(
        &self,
        R: *const xcb_glx_get_pixel_mapfv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_pixel_mapfv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_pixel_mapfv_data_end(
        &self,
        R: *const xcb_glx_get_pixel_mapfv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_pixel_mapfv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_pixel_mapfv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_pixel_mapfv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_pixel_mapfv_reply_t {
        sym!(self, xcb_glx_get_pixel_mapfv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_pixel_mapfv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        map: u32,
    ) -> xcb_glx_get_pixel_mapfv_cookie_t {
        sym!(self, xcb_glx_get_pixel_mapfv)(c, context_tag, map)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_pixel_mapfv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        map: u32,
    ) -> xcb_glx_get_pixel_mapfv_cookie_t {
        sym!(self, xcb_glx_get_pixel_mapfv_unchecked)(c, context_tag, map)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_pixel_mapuiv_data(
        &self,
        R: *const xcb_glx_get_pixel_mapuiv_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_get_pixel_mapuiv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_pixel_mapuiv_data_length(
        &self,
        R: *const xcb_glx_get_pixel_mapuiv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_pixel_mapuiv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_pixel_mapuiv_data_end(
        &self,
        R: *const xcb_glx_get_pixel_mapuiv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_pixel_mapuiv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_pixel_mapuiv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_pixel_mapuiv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_pixel_mapuiv_reply_t {
        sym!(self, xcb_glx_get_pixel_mapuiv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_pixel_mapuiv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        map: u32,
    ) -> xcb_glx_get_pixel_mapuiv_cookie_t {
        sym!(self, xcb_glx_get_pixel_mapuiv)(c, context_tag, map)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_pixel_mapuiv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        map: u32,
    ) -> xcb_glx_get_pixel_mapuiv_cookie_t {
        sym!(self, xcb_glx_get_pixel_mapuiv_unchecked)(c, context_tag, map)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_pixel_mapusv_data(
        &self,
        R: *const xcb_glx_get_pixel_mapusv_reply_t,
    ) -> *mut u16 {
        sym!(self, xcb_glx_get_pixel_mapusv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_pixel_mapusv_data_length(
        &self,
        R: *const xcb_glx_get_pixel_mapusv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_pixel_mapusv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_pixel_mapusv_data_end(
        &self,
        R: *const xcb_glx_get_pixel_mapusv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_pixel_mapusv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_pixel_mapusv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_pixel_mapusv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_pixel_mapusv_reply_t {
        sym!(self, xcb_glx_get_pixel_mapusv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_pixel_mapusv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        map: u32,
    ) -> xcb_glx_get_pixel_mapusv_cookie_t {
        sym!(self, xcb_glx_get_pixel_mapusv)(c, context_tag, map)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_pixel_mapusv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        map: u32,
    ) -> xcb_glx_get_pixel_mapusv_cookie_t {
        sym!(self, xcb_glx_get_pixel_mapusv_unchecked)(c, context_tag, map)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_polygon_stipple_data(
        &self,
        R: *const xcb_glx_get_polygon_stipple_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_glx_get_polygon_stipple_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_polygon_stipple_data_length(
        &self,
        R: *const xcb_glx_get_polygon_stipple_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_polygon_stipple_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_polygon_stipple_data_end(
        &self,
        R: *const xcb_glx_get_polygon_stipple_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_polygon_stipple_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_polygon_stipple_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_polygon_stipple_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_polygon_stipple_reply_t {
        sym!(self, xcb_glx_get_polygon_stipple_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_polygon_stipple(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        lsb_first: u8,
    ) -> xcb_glx_get_polygon_stipple_cookie_t {
        sym!(self, xcb_glx_get_polygon_stipple)(c, context_tag, lsb_first)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_polygon_stipple_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        lsb_first: u8,
    ) -> xcb_glx_get_polygon_stipple_cookie_t {
        sym!(self, xcb_glx_get_polygon_stipple_unchecked)(c, context_tag, lsb_first)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_string_string(
        &self,
        R: *const xcb_glx_get_string_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_glx_get_string_string)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_string_string_length(
        &self,
        R: *const xcb_glx_get_string_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_string_string_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_string_string_end(
        &self,
        R: *const xcb_glx_get_string_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_string_string_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_string_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_string_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_string_reply_t {
        sym!(self, xcb_glx_get_string_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_string(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        name: u32,
    ) -> xcb_glx_get_string_cookie_t {
        sym!(self, xcb_glx_get_string)(c, context_tag, name)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_string_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        name: u32,
    ) -> xcb_glx_get_string_cookie_t {
        sym!(self, xcb_glx_get_string_unchecked)(c, context_tag, name)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_envfv_data(
        &self,
        R: *const xcb_glx_get_tex_envfv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_tex_envfv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_envfv_data_length(
        &self,
        R: *const xcb_glx_get_tex_envfv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_tex_envfv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_envfv_data_end(
        &self,
        R: *const xcb_glx_get_tex_envfv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_tex_envfv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_envfv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_tex_envfv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_tex_envfv_reply_t {
        sym!(self, xcb_glx_get_tex_envfv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_envfv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_envfv_cookie_t {
        sym!(self, xcb_glx_get_tex_envfv)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_envfv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_envfv_cookie_t {
        sym!(self, xcb_glx_get_tex_envfv_unchecked)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_enviv_data(
        &self,
        R: *const xcb_glx_get_tex_enviv_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_tex_enviv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_enviv_data_length(
        &self,
        R: *const xcb_glx_get_tex_enviv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_tex_enviv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_enviv_data_end(
        &self,
        R: *const xcb_glx_get_tex_enviv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_tex_enviv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_enviv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_tex_enviv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_tex_enviv_reply_t {
        sym!(self, xcb_glx_get_tex_enviv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_enviv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_enviv_cookie_t {
        sym!(self, xcb_glx_get_tex_enviv)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_enviv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_enviv_cookie_t {
        sym!(self, xcb_glx_get_tex_enviv_unchecked)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_gendv_data(
        &self,
        R: *const xcb_glx_get_tex_gendv_reply_t,
    ) -> *mut xcb_glx_float64_t {
        sym!(self, xcb_glx_get_tex_gendv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_gendv_data_length(
        &self,
        R: *const xcb_glx_get_tex_gendv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_tex_gendv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_gendv_data_end(
        &self,
        R: *const xcb_glx_get_tex_gendv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_tex_gendv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_gendv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_tex_gendv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_tex_gendv_reply_t {
        sym!(self, xcb_glx_get_tex_gendv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_gendv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        coord: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_gendv_cookie_t {
        sym!(self, xcb_glx_get_tex_gendv)(c, context_tag, coord, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_gendv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        coord: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_gendv_cookie_t {
        sym!(self, xcb_glx_get_tex_gendv_unchecked)(c, context_tag, coord, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_genfv_data(
        &self,
        R: *const xcb_glx_get_tex_genfv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_tex_genfv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_genfv_data_length(
        &self,
        R: *const xcb_glx_get_tex_genfv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_tex_genfv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_genfv_data_end(
        &self,
        R: *const xcb_glx_get_tex_genfv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_tex_genfv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_genfv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_tex_genfv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_tex_genfv_reply_t {
        sym!(self, xcb_glx_get_tex_genfv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_genfv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        coord: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_genfv_cookie_t {
        sym!(self, xcb_glx_get_tex_genfv)(c, context_tag, coord, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_genfv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        coord: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_genfv_cookie_t {
        sym!(self, xcb_glx_get_tex_genfv_unchecked)(c, context_tag, coord, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_geniv_data(
        &self,
        R: *const xcb_glx_get_tex_geniv_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_tex_geniv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_geniv_data_length(
        &self,
        R: *const xcb_glx_get_tex_geniv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_tex_geniv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_geniv_data_end(
        &self,
        R: *const xcb_glx_get_tex_geniv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_tex_geniv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_geniv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_tex_geniv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_tex_geniv_reply_t {
        sym!(self, xcb_glx_get_tex_geniv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_geniv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        coord: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_geniv_cookie_t {
        sym!(self, xcb_glx_get_tex_geniv)(c, context_tag, coord, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_geniv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        coord: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_geniv_cookie_t {
        sym!(self, xcb_glx_get_tex_geniv_unchecked)(c, context_tag, coord, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_image_data(
        &self,
        R: *const xcb_glx_get_tex_image_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_glx_get_tex_image_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_image_data_length(
        &self,
        R: *const xcb_glx_get_tex_image_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_tex_image_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_image_data_end(
        &self,
        R: *const xcb_glx_get_tex_image_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_tex_image_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_image_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_tex_image_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_tex_image_reply_t {
        sym!(self, xcb_glx_get_tex_image_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_image(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        level: i32,
        format: u32,
        type_: u32,
        swap_bytes: u8,
    ) -> xcb_glx_get_tex_image_cookie_t {
        sym!(self, xcb_glx_get_tex_image)(c, context_tag, target, level, format, type_, swap_bytes)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_image_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        level: i32,
        format: u32,
        type_: u32,
        swap_bytes: u8,
    ) -> xcb_glx_get_tex_image_cookie_t {
        sym!(self, xcb_glx_get_tex_image_unchecked)(
            c,
            context_tag,
            target,
            level,
            format,
            type_,
            swap_bytes,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_parameterfv_data(
        &self,
        R: *const xcb_glx_get_tex_parameterfv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_tex_parameterfv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_parameterfv_data_length(
        &self,
        R: *const xcb_glx_get_tex_parameterfv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_tex_parameterfv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_parameterfv_data_end(
        &self,
        R: *const xcb_glx_get_tex_parameterfv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_tex_parameterfv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_parameterfv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_tex_parameterfv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_tex_parameterfv_reply_t {
        sym!(self, xcb_glx_get_tex_parameterfv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_parameterfv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_parameterfv_cookie_t {
        sym!(self, xcb_glx_get_tex_parameterfv)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_parameterfv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_parameterfv_cookie_t {
        sym!(self, xcb_glx_get_tex_parameterfv_unchecked)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_parameteriv_data(
        &self,
        R: *const xcb_glx_get_tex_parameteriv_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_tex_parameteriv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_parameteriv_data_length(
        &self,
        R: *const xcb_glx_get_tex_parameteriv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_tex_parameteriv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_parameteriv_data_end(
        &self,
        R: *const xcb_glx_get_tex_parameteriv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_tex_parameteriv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_parameteriv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_tex_parameteriv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_tex_parameteriv_reply_t {
        sym!(self, xcb_glx_get_tex_parameteriv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_parameteriv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_parameteriv_cookie_t {
        sym!(self, xcb_glx_get_tex_parameteriv)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_parameteriv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_parameteriv_cookie_t {
        sym!(self, xcb_glx_get_tex_parameteriv_unchecked)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_level_parameterfv_data(
        &self,
        R: *const xcb_glx_get_tex_level_parameterfv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_tex_level_parameterfv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_level_parameterfv_data_length(
        &self,
        R: *const xcb_glx_get_tex_level_parameterfv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_tex_level_parameterfv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_level_parameterfv_data_end(
        &self,
        R: *const xcb_glx_get_tex_level_parameterfv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_tex_level_parameterfv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_level_parameterfv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_tex_level_parameterfv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_tex_level_parameterfv_reply_t {
        sym!(self, xcb_glx_get_tex_level_parameterfv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_level_parameterfv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        level: i32,
        pname: u32,
    ) -> xcb_glx_get_tex_level_parameterfv_cookie_t {
        sym!(self, xcb_glx_get_tex_level_parameterfv)(c, context_tag, target, level, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_level_parameterfv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        level: i32,
        pname: u32,
    ) -> xcb_glx_get_tex_level_parameterfv_cookie_t {
        sym!(self, xcb_glx_get_tex_level_parameterfv_unchecked)(
            c,
            context_tag,
            target,
            level,
            pname,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_level_parameteriv_data(
        &self,
        R: *const xcb_glx_get_tex_level_parameteriv_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_tex_level_parameteriv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_level_parameteriv_data_length(
        &self,
        R: *const xcb_glx_get_tex_level_parameteriv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_tex_level_parameteriv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_level_parameteriv_data_end(
        &self,
        R: *const xcb_glx_get_tex_level_parameteriv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_tex_level_parameteriv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_level_parameteriv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_tex_level_parameteriv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_tex_level_parameteriv_reply_t {
        sym!(self, xcb_glx_get_tex_level_parameteriv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_level_parameteriv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        level: i32,
        pname: u32,
    ) -> xcb_glx_get_tex_level_parameteriv_cookie_t {
        sym!(self, xcb_glx_get_tex_level_parameteriv)(c, context_tag, target, level, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_tex_level_parameteriv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        level: i32,
        pname: u32,
    ) -> xcb_glx_get_tex_level_parameteriv_cookie_t {
        sym!(self, xcb_glx_get_tex_level_parameteriv_unchecked)(
            c,
            context_tag,
            target,
            level,
            pname,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_is_enabled_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_is_enabled_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_is_enabled_reply_t {
        sym!(self, xcb_glx_is_enabled_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_is_enabled(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        capability: u32,
    ) -> xcb_glx_is_enabled_cookie_t {
        sym!(self, xcb_glx_is_enabled)(c, context_tag, capability)
    }

    #[inline]
    pub unsafe fn xcb_glx_is_enabled_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        capability: u32,
    ) -> xcb_glx_is_enabled_cookie_t {
        sym!(self, xcb_glx_is_enabled_unchecked)(c, context_tag, capability)
    }

    #[inline]
    pub unsafe fn xcb_glx_is_list_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_is_list_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_is_list_reply_t {
        sym!(self, xcb_glx_is_list_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_is_list(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        list: u32,
    ) -> xcb_glx_is_list_cookie_t {
        sym!(self, xcb_glx_is_list)(c, context_tag, list)
    }

    #[inline]
    pub unsafe fn xcb_glx_is_list_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        list: u32,
    ) -> xcb_glx_is_list_cookie_t {
        sym!(self, xcb_glx_is_list_unchecked)(c, context_tag, list)
    }

    #[inline]
    pub unsafe fn xcb_glx_flush(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_flush)(c, context_tag)
    }

    #[inline]
    pub unsafe fn xcb_glx_flush_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_flush_checked)(c, context_tag)
    }

    #[inline]
    pub unsafe fn xcb_glx_are_textures_resident_data(
        &self,
        R: *const xcb_glx_are_textures_resident_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_glx_are_textures_resident_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_are_textures_resident_data_length(
        &self,
        R: *const xcb_glx_are_textures_resident_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_are_textures_resident_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_are_textures_resident_data_end(
        &self,
        R: *const xcb_glx_are_textures_resident_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_are_textures_resident_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_are_textures_resident_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_are_textures_resident_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_are_textures_resident_reply_t {
        sym!(self, xcb_glx_are_textures_resident_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_are_textures_resident(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        n: i32,
        textures: *const u32,
    ) -> xcb_glx_are_textures_resident_cookie_t {
        sym!(self, xcb_glx_are_textures_resident)(c, context_tag, n, textures)
    }

    #[inline]
    pub unsafe fn xcb_glx_are_textures_resident_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        n: i32,
        textures: *const u32,
    ) -> xcb_glx_are_textures_resident_cookie_t {
        sym!(self, xcb_glx_are_textures_resident_unchecked)(c, context_tag, n, textures)
    }

    #[inline]
    pub unsafe fn xcb_glx_delete_textures(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        n: i32,
        textures: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_delete_textures)(c, context_tag, n, textures)
    }

    #[inline]
    pub unsafe fn xcb_glx_delete_textures_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        n: i32,
        textures: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_delete_textures_checked)(c, context_tag, n, textures)
    }

    #[inline]
    pub unsafe fn xcb_glx_gen_textures_data(
        &self,
        R: *const xcb_glx_gen_textures_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_gen_textures_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_gen_textures_data_length(
        &self,
        R: *const xcb_glx_gen_textures_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_gen_textures_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_gen_textures_data_end(
        &self,
        R: *const xcb_glx_gen_textures_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_gen_textures_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_gen_textures_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_gen_textures_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_gen_textures_reply_t {
        sym!(self, xcb_glx_gen_textures_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_gen_textures(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        n: i32,
    ) -> xcb_glx_gen_textures_cookie_t {
        sym!(self, xcb_glx_gen_textures)(c, context_tag, n)
    }

    #[inline]
    pub unsafe fn xcb_glx_gen_textures_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        n: i32,
    ) -> xcb_glx_gen_textures_cookie_t {
        sym!(self, xcb_glx_gen_textures_unchecked)(c, context_tag, n)
    }

    #[inline]
    pub unsafe fn xcb_glx_is_texture_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_is_texture_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_is_texture_reply_t {
        sym!(self, xcb_glx_is_texture_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_is_texture(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        texture: u32,
    ) -> xcb_glx_is_texture_cookie_t {
        sym!(self, xcb_glx_is_texture)(c, context_tag, texture)
    }

    #[inline]
    pub unsafe fn xcb_glx_is_texture_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        texture: u32,
    ) -> xcb_glx_is_texture_cookie_t {
        sym!(self, xcb_glx_is_texture_unchecked)(c, context_tag, texture)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_color_table_data(
        &self,
        R: *const xcb_glx_get_color_table_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_glx_get_color_table_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_color_table_data_length(
        &self,
        R: *const xcb_glx_get_color_table_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_color_table_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_color_table_data_end(
        &self,
        R: *const xcb_glx_get_color_table_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_color_table_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_color_table_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_color_table_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_color_table_reply_t {
        sym!(self, xcb_glx_get_color_table_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_color_table(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        format: u32,
        type_: u32,
        swap_bytes: u8,
    ) -> xcb_glx_get_color_table_cookie_t {
        sym!(self, xcb_glx_get_color_table)(c, context_tag, target, format, type_, swap_bytes)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_color_table_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        format: u32,
        type_: u32,
        swap_bytes: u8,
    ) -> xcb_glx_get_color_table_cookie_t {
        sym!(self, xcb_glx_get_color_table_unchecked)(
            c,
            context_tag,
            target,
            format,
            type_,
            swap_bytes,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_get_color_table_parameterfv_data(
        &self,
        R: *const xcb_glx_get_color_table_parameterfv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_color_table_parameterfv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_color_table_parameterfv_data_length(
        &self,
        R: *const xcb_glx_get_color_table_parameterfv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_color_table_parameterfv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_color_table_parameterfv_data_end(
        &self,
        R: *const xcb_glx_get_color_table_parameterfv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_color_table_parameterfv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_color_table_parameterfv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_color_table_parameterfv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_color_table_parameterfv_reply_t {
        sym!(self, xcb_glx_get_color_table_parameterfv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_color_table_parameterfv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_color_table_parameterfv_cookie_t {
        sym!(self, xcb_glx_get_color_table_parameterfv)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_color_table_parameterfv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_color_table_parameterfv_cookie_t {
        sym!(self, xcb_glx_get_color_table_parameterfv_unchecked)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_color_table_parameteriv_data(
        &self,
        R: *const xcb_glx_get_color_table_parameteriv_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_color_table_parameteriv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_color_table_parameteriv_data_length(
        &self,
        R: *const xcb_glx_get_color_table_parameteriv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_color_table_parameteriv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_color_table_parameteriv_data_end(
        &self,
        R: *const xcb_glx_get_color_table_parameteriv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_color_table_parameteriv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_color_table_parameteriv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_color_table_parameteriv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_color_table_parameteriv_reply_t {
        sym!(self, xcb_glx_get_color_table_parameteriv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_color_table_parameteriv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_color_table_parameteriv_cookie_t {
        sym!(self, xcb_glx_get_color_table_parameteriv)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_color_table_parameteriv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_color_table_parameteriv_cookie_t {
        sym!(self, xcb_glx_get_color_table_parameteriv_unchecked)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_convolution_filter_data(
        &self,
        R: *const xcb_glx_get_convolution_filter_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_glx_get_convolution_filter_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_convolution_filter_data_length(
        &self,
        R: *const xcb_glx_get_convolution_filter_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_convolution_filter_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_convolution_filter_data_end(
        &self,
        R: *const xcb_glx_get_convolution_filter_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_convolution_filter_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_convolution_filter_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_convolution_filter_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_convolution_filter_reply_t {
        sym!(self, xcb_glx_get_convolution_filter_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_convolution_filter(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        format: u32,
        type_: u32,
        swap_bytes: u8,
    ) -> xcb_glx_get_convolution_filter_cookie_t {
        sym!(self, xcb_glx_get_convolution_filter)(
            c,
            context_tag,
            target,
            format,
            type_,
            swap_bytes,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_get_convolution_filter_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        format: u32,
        type_: u32,
        swap_bytes: u8,
    ) -> xcb_glx_get_convolution_filter_cookie_t {
        sym!(self, xcb_glx_get_convolution_filter_unchecked)(
            c,
            context_tag,
            target,
            format,
            type_,
            swap_bytes,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_get_convolution_parameterfv_data(
        &self,
        R: *const xcb_glx_get_convolution_parameterfv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_convolution_parameterfv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_convolution_parameterfv_data_length(
        &self,
        R: *const xcb_glx_get_convolution_parameterfv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_convolution_parameterfv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_convolution_parameterfv_data_end(
        &self,
        R: *const xcb_glx_get_convolution_parameterfv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_convolution_parameterfv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_convolution_parameterfv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_convolution_parameterfv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_convolution_parameterfv_reply_t {
        sym!(self, xcb_glx_get_convolution_parameterfv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_convolution_parameterfv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_convolution_parameterfv_cookie_t {
        sym!(self, xcb_glx_get_convolution_parameterfv)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_convolution_parameterfv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_convolution_parameterfv_cookie_t {
        sym!(self, xcb_glx_get_convolution_parameterfv_unchecked)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_convolution_parameteriv_data(
        &self,
        R: *const xcb_glx_get_convolution_parameteriv_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_convolution_parameteriv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_convolution_parameteriv_data_length(
        &self,
        R: *const xcb_glx_get_convolution_parameteriv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_convolution_parameteriv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_convolution_parameteriv_data_end(
        &self,
        R: *const xcb_glx_get_convolution_parameteriv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_convolution_parameteriv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_convolution_parameteriv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_convolution_parameteriv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_convolution_parameteriv_reply_t {
        sym!(self, xcb_glx_get_convolution_parameteriv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_convolution_parameteriv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_convolution_parameteriv_cookie_t {
        sym!(self, xcb_glx_get_convolution_parameteriv)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_convolution_parameteriv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_convolution_parameteriv_cookie_t {
        sym!(self, xcb_glx_get_convolution_parameteriv_unchecked)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_separable_filter_rows_and_cols(
        &self,
        R: *const xcb_glx_get_separable_filter_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_glx_get_separable_filter_rows_and_cols)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_separable_filter_rows_and_cols_length(
        &self,
        R: *const xcb_glx_get_separable_filter_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_separable_filter_rows_and_cols_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_separable_filter_rows_and_cols_end(
        &self,
        R: *const xcb_glx_get_separable_filter_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_separable_filter_rows_and_cols_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_separable_filter_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_separable_filter_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_separable_filter_reply_t {
        sym!(self, xcb_glx_get_separable_filter_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_separable_filter(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        format: u32,
        type_: u32,
        swap_bytes: u8,
    ) -> xcb_glx_get_separable_filter_cookie_t {
        sym!(self, xcb_glx_get_separable_filter)(c, context_tag, target, format, type_, swap_bytes)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_separable_filter_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        format: u32,
        type_: u32,
        swap_bytes: u8,
    ) -> xcb_glx_get_separable_filter_cookie_t {
        sym!(self, xcb_glx_get_separable_filter_unchecked)(
            c,
            context_tag,
            target,
            format,
            type_,
            swap_bytes,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_get_histogram_data(
        &self,
        R: *const xcb_glx_get_histogram_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_glx_get_histogram_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_histogram_data_length(
        &self,
        R: *const xcb_glx_get_histogram_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_histogram_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_histogram_data_end(
        &self,
        R: *const xcb_glx_get_histogram_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_histogram_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_histogram_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_histogram_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_histogram_reply_t {
        sym!(self, xcb_glx_get_histogram_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_histogram(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        format: u32,
        type_: u32,
        swap_bytes: u8,
        reset: u8,
    ) -> xcb_glx_get_histogram_cookie_t {
        sym!(self, xcb_glx_get_histogram)(c, context_tag, target, format, type_, swap_bytes, reset)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_histogram_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        format: u32,
        type_: u32,
        swap_bytes: u8,
        reset: u8,
    ) -> xcb_glx_get_histogram_cookie_t {
        sym!(self, xcb_glx_get_histogram_unchecked)(
            c,
            context_tag,
            target,
            format,
            type_,
            swap_bytes,
            reset,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_get_histogram_parameterfv_data(
        &self,
        R: *const xcb_glx_get_histogram_parameterfv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_histogram_parameterfv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_histogram_parameterfv_data_length(
        &self,
        R: *const xcb_glx_get_histogram_parameterfv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_histogram_parameterfv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_histogram_parameterfv_data_end(
        &self,
        R: *const xcb_glx_get_histogram_parameterfv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_histogram_parameterfv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_histogram_parameterfv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_histogram_parameterfv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_histogram_parameterfv_reply_t {
        sym!(self, xcb_glx_get_histogram_parameterfv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_histogram_parameterfv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_histogram_parameterfv_cookie_t {
        sym!(self, xcb_glx_get_histogram_parameterfv)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_histogram_parameterfv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_histogram_parameterfv_cookie_t {
        sym!(self, xcb_glx_get_histogram_parameterfv_unchecked)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_histogram_parameteriv_data(
        &self,
        R: *const xcb_glx_get_histogram_parameteriv_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_histogram_parameteriv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_histogram_parameteriv_data_length(
        &self,
        R: *const xcb_glx_get_histogram_parameteriv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_histogram_parameteriv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_histogram_parameteriv_data_end(
        &self,
        R: *const xcb_glx_get_histogram_parameteriv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_histogram_parameteriv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_histogram_parameteriv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_histogram_parameteriv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_histogram_parameteriv_reply_t {
        sym!(self, xcb_glx_get_histogram_parameteriv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_histogram_parameteriv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_histogram_parameteriv_cookie_t {
        sym!(self, xcb_glx_get_histogram_parameteriv)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_histogram_parameteriv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_histogram_parameteriv_cookie_t {
        sym!(self, xcb_glx_get_histogram_parameteriv_unchecked)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_minmax_data(&self, R: *const xcb_glx_get_minmax_reply_t) -> *mut u8 {
        sym!(self, xcb_glx_get_minmax_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_minmax_data_length(
        &self,
        R: *const xcb_glx_get_minmax_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_minmax_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_minmax_data_end(
        &self,
        R: *const xcb_glx_get_minmax_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_minmax_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_minmax_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_minmax_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_minmax_reply_t {
        sym!(self, xcb_glx_get_minmax_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_minmax(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        format: u32,
        type_: u32,
        swap_bytes: u8,
        reset: u8,
    ) -> xcb_glx_get_minmax_cookie_t {
        sym!(self, xcb_glx_get_minmax)(c, context_tag, target, format, type_, swap_bytes, reset)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_minmax_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        format: u32,
        type_: u32,
        swap_bytes: u8,
        reset: u8,
    ) -> xcb_glx_get_minmax_cookie_t {
        sym!(self, xcb_glx_get_minmax_unchecked)(
            c,
            context_tag,
            target,
            format,
            type_,
            swap_bytes,
            reset,
        )
    }

    #[inline]
    pub unsafe fn xcb_glx_get_minmax_parameterfv_data(
        &self,
        R: *const xcb_glx_get_minmax_parameterfv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_minmax_parameterfv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_minmax_parameterfv_data_length(
        &self,
        R: *const xcb_glx_get_minmax_parameterfv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_minmax_parameterfv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_minmax_parameterfv_data_end(
        &self,
        R: *const xcb_glx_get_minmax_parameterfv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_minmax_parameterfv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_minmax_parameterfv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_minmax_parameterfv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_minmax_parameterfv_reply_t {
        sym!(self, xcb_glx_get_minmax_parameterfv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_minmax_parameterfv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_minmax_parameterfv_cookie_t {
        sym!(self, xcb_glx_get_minmax_parameterfv)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_minmax_parameterfv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_minmax_parameterfv_cookie_t {
        sym!(self, xcb_glx_get_minmax_parameterfv_unchecked)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_minmax_parameteriv_data(
        &self,
        R: *const xcb_glx_get_minmax_parameteriv_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_minmax_parameteriv_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_minmax_parameteriv_data_length(
        &self,
        R: *const xcb_glx_get_minmax_parameteriv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_minmax_parameteriv_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_minmax_parameteriv_data_end(
        &self,
        R: *const xcb_glx_get_minmax_parameteriv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_minmax_parameteriv_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_minmax_parameteriv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_minmax_parameteriv_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_minmax_parameteriv_reply_t {
        sym!(self, xcb_glx_get_minmax_parameteriv_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_minmax_parameteriv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_minmax_parameteriv_cookie_t {
        sym!(self, xcb_glx_get_minmax_parameteriv)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_minmax_parameteriv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_minmax_parameteriv_cookie_t {
        sym!(self, xcb_glx_get_minmax_parameteriv_unchecked)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_compressed_tex_image_arb_data(
        &self,
        R: *const xcb_glx_get_compressed_tex_image_arb_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_glx_get_compressed_tex_image_arb_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_compressed_tex_image_arb_data_length(
        &self,
        R: *const xcb_glx_get_compressed_tex_image_arb_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_compressed_tex_image_arb_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_compressed_tex_image_arb_data_end(
        &self,
        R: *const xcb_glx_get_compressed_tex_image_arb_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_compressed_tex_image_arb_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_compressed_tex_image_arb_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_compressed_tex_image_arb_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_compressed_tex_image_arb_reply_t {
        sym!(self, xcb_glx_get_compressed_tex_image_arb_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_compressed_tex_image_arb(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        level: i32,
    ) -> xcb_glx_get_compressed_tex_image_arb_cookie_t {
        sym!(self, xcb_glx_get_compressed_tex_image_arb)(c, context_tag, target, level)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_compressed_tex_image_arb_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        level: i32,
    ) -> xcb_glx_get_compressed_tex_image_arb_cookie_t {
        sym!(self, xcb_glx_get_compressed_tex_image_arb_unchecked)(c, context_tag, target, level)
    }

    #[inline]
    pub unsafe fn xcb_glx_delete_queries_arb(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        n: i32,
        ids: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_delete_queries_arb)(c, context_tag, n, ids)
    }

    #[inline]
    pub unsafe fn xcb_glx_delete_queries_arb_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        n: i32,
        ids: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_delete_queries_arb_checked)(c, context_tag, n, ids)
    }

    #[inline]
    pub unsafe fn xcb_glx_gen_queries_arb_data(
        &self,
        R: *const xcb_glx_gen_queries_arb_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_gen_queries_arb_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_gen_queries_arb_data_length(
        &self,
        R: *const xcb_glx_gen_queries_arb_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_gen_queries_arb_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_gen_queries_arb_data_end(
        &self,
        R: *const xcb_glx_gen_queries_arb_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_gen_queries_arb_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_gen_queries_arb_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_gen_queries_arb_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_gen_queries_arb_reply_t {
        sym!(self, xcb_glx_gen_queries_arb_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_gen_queries_arb(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        n: i32,
    ) -> xcb_glx_gen_queries_arb_cookie_t {
        sym!(self, xcb_glx_gen_queries_arb)(c, context_tag, n)
    }

    #[inline]
    pub unsafe fn xcb_glx_gen_queries_arb_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        n: i32,
    ) -> xcb_glx_gen_queries_arb_cookie_t {
        sym!(self, xcb_glx_gen_queries_arb_unchecked)(c, context_tag, n)
    }

    #[inline]
    pub unsafe fn xcb_glx_is_query_arb_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_is_query_arb_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_is_query_arb_reply_t {
        sym!(self, xcb_glx_is_query_arb_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_is_query_arb(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        id: u32,
    ) -> xcb_glx_is_query_arb_cookie_t {
        sym!(self, xcb_glx_is_query_arb)(c, context_tag, id)
    }

    #[inline]
    pub unsafe fn xcb_glx_is_query_arb_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        id: u32,
    ) -> xcb_glx_is_query_arb_cookie_t {
        sym!(self, xcb_glx_is_query_arb_unchecked)(c, context_tag, id)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_queryiv_arb_data(
        &self,
        R: *const xcb_glx_get_queryiv_arb_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_queryiv_arb_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_queryiv_arb_data_length(
        &self,
        R: *const xcb_glx_get_queryiv_arb_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_queryiv_arb_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_queryiv_arb_data_end(
        &self,
        R: *const xcb_glx_get_queryiv_arb_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_queryiv_arb_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_queryiv_arb_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_queryiv_arb_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_queryiv_arb_reply_t {
        sym!(self, xcb_glx_get_queryiv_arb_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_queryiv_arb(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_queryiv_arb_cookie_t {
        sym!(self, xcb_glx_get_queryiv_arb)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_queryiv_arb_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_queryiv_arb_cookie_t {
        sym!(self, xcb_glx_get_queryiv_arb_unchecked)(c, context_tag, target, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_query_objectiv_arb_data(
        &self,
        R: *const xcb_glx_get_query_objectiv_arb_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_query_objectiv_arb_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_query_objectiv_arb_data_length(
        &self,
        R: *const xcb_glx_get_query_objectiv_arb_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_query_objectiv_arb_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_query_objectiv_arb_data_end(
        &self,
        R: *const xcb_glx_get_query_objectiv_arb_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_query_objectiv_arb_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_query_objectiv_arb_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_query_objectiv_arb_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_query_objectiv_arb_reply_t {
        sym!(self, xcb_glx_get_query_objectiv_arb_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_query_objectiv_arb(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        id: u32,
        pname: u32,
    ) -> xcb_glx_get_query_objectiv_arb_cookie_t {
        sym!(self, xcb_glx_get_query_objectiv_arb)(c, context_tag, id, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_query_objectiv_arb_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        id: u32,
        pname: u32,
    ) -> xcb_glx_get_query_objectiv_arb_cookie_t {
        sym!(self, xcb_glx_get_query_objectiv_arb_unchecked)(c, context_tag, id, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_query_objectuiv_arb_data(
        &self,
        R: *const xcb_glx_get_query_objectuiv_arb_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_get_query_objectuiv_arb_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_query_objectuiv_arb_data_length(
        &self,
        R: *const xcb_glx_get_query_objectuiv_arb_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_query_objectuiv_arb_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_query_objectuiv_arb_data_end(
        &self,
        R: *const xcb_glx_get_query_objectuiv_arb_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_query_objectuiv_arb_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_query_objectuiv_arb_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_query_objectuiv_arb_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_glx_get_query_objectuiv_arb_reply_t {
        sym!(self, xcb_glx_get_query_objectuiv_arb_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_query_objectuiv_arb(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        id: u32,
        pname: u32,
    ) -> xcb_glx_get_query_objectuiv_arb_cookie_t {
        sym!(self, xcb_glx_get_query_objectuiv_arb)(c, context_tag, id, pname)
    }

    #[inline]
    pub unsafe fn xcb_glx_get_query_objectuiv_arb_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        id: u32,
        pname: u32,
    ) -> xcb_glx_get_query_objectuiv_arb_cookie_t {
        sym!(self, xcb_glx_get_query_objectuiv_arb_unchecked)(c, context_tag, id, pname)
    }
}
