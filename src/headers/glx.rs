// This file was generated using generate.py. Do not edit.

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

pub type xcb_glx_pixmap_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_pixmap_iterator_t {
    pub data: *mut xcb_glx_pixmap_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_glx_pixmap_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_glx_context_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_context_iterator_t {
    pub data: *mut xcb_glx_context_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_glx_context_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_glx_pbuffer_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_pbuffer_iterator_t {
    pub data: *mut xcb_glx_pbuffer_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_glx_pbuffer_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_glx_window_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_window_iterator_t {
    pub data: *mut xcb_glx_window_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_glx_window_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_glx_fbconfig_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_fbconfig_iterator_t {
    pub data: *mut xcb_glx_fbconfig_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_glx_fbconfig_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_glx_drawable_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_drawable_iterator_t {
    pub data: *mut xcb_glx_drawable_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_glx_drawable_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_glx_float32_t = f32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_float32_iterator_t {
    pub data: *mut xcb_glx_float32_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_glx_float32_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_glx_float64_t = f64;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_float64_iterator_t {
    pub data: *mut xcb_glx_float64_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_glx_float64_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_glx_bool32_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_bool32_iterator_t {
    pub data: *mut xcb_glx_bool32_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_glx_bool32_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_glx_context_tag_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_context_tag_iterator_t {
    pub data: *mut xcb_glx_context_tag_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_glx_context_tag_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_generic.
pub const XCB_GLX_GENERIC: u8 = -1i32 as u8;

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

impl Default for xcb_glx_generic_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_bad_context.
pub const XCB_GLX_BAD_CONTEXT: u8 = 0i32 as u8;

pub type xcb_glx_bad_context_error_t = xcb_glx_generic_error_t;

/// Opcode for xcb_glx_bad_context_state.
pub const XCB_GLX_BAD_CONTEXT_STATE: u8 = 1i32 as u8;

pub type xcb_glx_bad_context_state_error_t = xcb_glx_generic_error_t;

/// Opcode for xcb_glx_bad_drawable.
pub const XCB_GLX_BAD_DRAWABLE: u8 = 2i32 as u8;

pub type xcb_glx_bad_drawable_error_t = xcb_glx_generic_error_t;

/// Opcode for xcb_glx_bad_pixmap.
pub const XCB_GLX_BAD_PIXMAP: u8 = 3i32 as u8;

pub type xcb_glx_bad_pixmap_error_t = xcb_glx_generic_error_t;

/// Opcode for xcb_glx_bad_context_tag.
pub const XCB_GLX_BAD_CONTEXT_TAG: u8 = 4i32 as u8;

pub type xcb_glx_bad_context_tag_error_t = xcb_glx_generic_error_t;

/// Opcode for xcb_glx_bad_current_window.
pub const XCB_GLX_BAD_CURRENT_WINDOW: u8 = 5i32 as u8;

pub type xcb_glx_bad_current_window_error_t = xcb_glx_generic_error_t;

/// Opcode for xcb_glx_bad_render_request.
pub const XCB_GLX_BAD_RENDER_REQUEST: u8 = 6i32 as u8;

pub type xcb_glx_bad_render_request_error_t = xcb_glx_generic_error_t;

/// Opcode for xcb_glx_bad_large_request.
pub const XCB_GLX_BAD_LARGE_REQUEST: u8 = 7i32 as u8;

pub type xcb_glx_bad_large_request_error_t = xcb_glx_generic_error_t;

/// Opcode for xcb_glx_unsupported_private_request.
pub const XCB_GLX_UNSUPPORTED_PRIVATE_REQUEST: u8 = 8i32 as u8;

pub type xcb_glx_unsupported_private_request_error_t = xcb_glx_generic_error_t;

/// Opcode for xcb_glx_bad_fb_config.
pub const XCB_GLX_BAD_FB_CONFIG: u8 = 9i32 as u8;

pub type xcb_glx_bad_fb_config_error_t = xcb_glx_generic_error_t;

/// Opcode for xcb_glx_bad_pbuffer.
pub const XCB_GLX_BAD_PBUFFER: u8 = 10i32 as u8;

pub type xcb_glx_bad_pbuffer_error_t = xcb_glx_generic_error_t;

/// Opcode for xcb_glx_bad_current_drawable.
pub const XCB_GLX_BAD_CURRENT_DRAWABLE: u8 = 11i32 as u8;

pub type xcb_glx_bad_current_drawable_error_t = xcb_glx_generic_error_t;

/// Opcode for xcb_glx_bad_window.
pub const XCB_GLX_BAD_WINDOW: u8 = 12i32 as u8;

pub type xcb_glx_bad_window_error_t = xcb_glx_generic_error_t;

/// Opcode for xcb_glx_glx_bad_profile_arb.
pub const XCB_GLX_GLX_BAD_PROFILE_ARB: u8 = 13i32 as u8;

pub type xcb_glx_glx_bad_profile_arb_error_t = xcb_glx_generic_error_t;

/// Opcode for xcb_glx_pbuffer_clobber.
pub const XCB_GLX_PBUFFER_CLOBBER: u8 = 0i32 as u8;

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

impl Default for xcb_glx_pbuffer_clobber_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_buffer_swap_complete.
pub const XCB_GLX_BUFFER_SWAP_COMPLETE: u8 = 1i32 as u8;

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

impl Default for xcb_glx_buffer_swap_complete_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_glx_pbcet_t = u32;
pub const XCB_GLX_PBCET_DAMAGED: xcb_glx_pbcet_t = 32791;
pub const XCB_GLX_PBCET_SAVED: xcb_glx_pbcet_t = 32792;

pub type xcb_glx_pbcdt_t = u32;
pub const XCB_GLX_PBCDT_WINDOW: xcb_glx_pbcdt_t = 32793;
pub const XCB_GLX_PBCDT_PBUFFER: xcb_glx_pbcdt_t = 32794;

/// Opcode for xcb_glx_render.
pub const XCB_GLX_RENDER: u8 = 1i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_render_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
}

impl Default for xcb_glx_render_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_render_large.
pub const XCB_GLX_RENDER_LARGE: u8 = 2i32 as u8;

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

impl Default for xcb_glx_render_large_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_create_context.
pub const XCB_GLX_CREATE_CONTEXT: u8 = 3i32 as u8;

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

impl Default for xcb_glx_create_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_destroy_context.
pub const XCB_GLX_DESTROY_CONTEXT: u8 = 4i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_destroy_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_glx_context_t,
}

impl Default for xcb_glx_destroy_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_make_current_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_make_current_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_make_current.
pub const XCB_GLX_MAKE_CURRENT: u8 = 5i32 as u8;

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

impl Default for xcb_glx_make_current_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_make_current_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_is_direct_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_is_direct_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_is_direct.
pub const XCB_GLX_IS_DIRECT: u8 = 6i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_is_direct_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_glx_context_t,
}

impl Default for xcb_glx_is_direct_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_is_direct_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_query_version_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_query_version.
pub const XCB_GLX_QUERY_VERSION: u8 = 7i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub major_version: u32,
    pub minor_version: u32,
}

impl Default for xcb_glx_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_wait_gl.
pub const XCB_GLX_WAIT_GL: u8 = 8i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_wait_gl_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
}

impl Default for xcb_glx_wait_gl_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_wait_x.
pub const XCB_GLX_WAIT_X: u8 = 9i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_wait_x_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
}

impl Default for xcb_glx_wait_x_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_copy_context.
pub const XCB_GLX_COPY_CONTEXT: u8 = 10i32 as u8;

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

impl Default for xcb_glx_copy_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_glx_gc_t = u32;
pub const XCB_GLX_GC_GL_CURRENT_BIT: xcb_glx_gc_t = 1;
pub const XCB_GLX_GC_GL_POINT_BIT: xcb_glx_gc_t = 2;
pub const XCB_GLX_GC_GL_LINE_BIT: xcb_glx_gc_t = 4;
pub const XCB_GLX_GC_GL_POLYGON_BIT: xcb_glx_gc_t = 8;
pub const XCB_GLX_GC_GL_POLYGON_STIPPLE_BIT: xcb_glx_gc_t = 16;
pub const XCB_GLX_GC_GL_PIXEL_MODE_BIT: xcb_glx_gc_t = 32;
pub const XCB_GLX_GC_GL_LIGHTING_BIT: xcb_glx_gc_t = 64;
pub const XCB_GLX_GC_GL_FOG_BIT: xcb_glx_gc_t = 128;
pub const XCB_GLX_GC_GL_DEPTH_BUFFER_BIT: xcb_glx_gc_t = 256;
pub const XCB_GLX_GC_GL_ACCUM_BUFFER_BIT: xcb_glx_gc_t = 512;
pub const XCB_GLX_GC_GL_STENCIL_BUFFER_BIT: xcb_glx_gc_t = 1024;
pub const XCB_GLX_GC_GL_VIEWPORT_BIT: xcb_glx_gc_t = 2048;
pub const XCB_GLX_GC_GL_TRANSFORM_BIT: xcb_glx_gc_t = 4096;
pub const XCB_GLX_GC_GL_ENABLE_BIT: xcb_glx_gc_t = 8192;
pub const XCB_GLX_GC_GL_COLOR_BUFFER_BIT: xcb_glx_gc_t = 16384;
pub const XCB_GLX_GC_GL_HINT_BIT: xcb_glx_gc_t = 32768;
pub const XCB_GLX_GC_GL_EVAL_BIT: xcb_glx_gc_t = 65536;
pub const XCB_GLX_GC_GL_LIST_BIT: xcb_glx_gc_t = 131072;
pub const XCB_GLX_GC_GL_TEXTURE_BIT: xcb_glx_gc_t = 262144;
pub const XCB_GLX_GC_GL_SCISSOR_BIT: xcb_glx_gc_t = 524288;
pub const XCB_GLX_GC_GL_ALL_ATTRIB_BITS: xcb_glx_gc_t = 16777215;

/// Opcode for xcb_glx_swap_buffers.
pub const XCB_GLX_SWAP_BUFFERS: u8 = 11i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_swap_buffers_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub drawable: xcb_glx_drawable_t,
}

impl Default for xcb_glx_swap_buffers_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_use_x_font.
pub const XCB_GLX_USE_X_FONT: u8 = 12i32 as u8;

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

impl Default for xcb_glx_use_x_font_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_create_glx_pixmap.
pub const XCB_GLX_CREATE_GLX_PIXMAP: u8 = 13i32 as u8;

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

impl Default for xcb_glx_create_glx_pixmap_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_visual_configs_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_visual_configs_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_visual_configs.
pub const XCB_GLX_GET_VISUAL_CONFIGS: u8 = 14i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_visual_configs_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
}

impl Default for xcb_glx_get_visual_configs_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_visual_configs_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_destroy_glx_pixmap.
pub const XCB_GLX_DESTROY_GLX_PIXMAP: u8 = 15i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_destroy_glx_pixmap_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub glx_pixmap: xcb_glx_pixmap_t,
}

impl Default for xcb_glx_destroy_glx_pixmap_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_vendor_private.
pub const XCB_GLX_VENDOR_PRIVATE: u8 = 16i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_vendor_private_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub vendor_code: u32,
    pub context_tag: xcb_glx_context_tag_t,
}

impl Default for xcb_glx_vendor_private_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_vendor_private_with_reply_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_vendor_private_with_reply_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_vendor_private_with_reply.
pub const XCB_GLX_VENDOR_PRIVATE_WITH_REPLY: u8 = 17i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_vendor_private_with_reply_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub vendor_code: u32,
    pub context_tag: xcb_glx_context_tag_t,
}

impl Default for xcb_glx_vendor_private_with_reply_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_vendor_private_with_reply_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_query_extensions_string_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_query_extensions_string_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_query_extensions_string.
pub const XCB_GLX_QUERY_EXTENSIONS_STRING: u8 = 18i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_query_extensions_string_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
}

impl Default for xcb_glx_query_extensions_string_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_query_extensions_string_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_query_server_string_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_query_server_string_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_query_server_string.
pub const XCB_GLX_QUERY_SERVER_STRING: u8 = 19i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_query_server_string_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
    pub name: u32,
}

impl Default for xcb_glx_query_server_string_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_query_server_string_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_client_info.
pub const XCB_GLX_CLIENT_INFO: u8 = 20i32 as u8;

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

impl Default for xcb_glx_client_info_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_fb_configs_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_fb_configs_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_fb_configs.
pub const XCB_GLX_GET_FB_CONFIGS: u8 = 21i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_fb_configs_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
}

impl Default for xcb_glx_get_fb_configs_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_fb_configs_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num__f_b_configs: u32,
    pub num_properties: u32,
    pub pad1: [u8; 16],
}

impl Default for xcb_glx_get_fb_configs_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_create_pixmap.
pub const XCB_GLX_CREATE_PIXMAP: u8 = 22i32 as u8;

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

impl Default for xcb_glx_create_pixmap_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_destroy_pixmap.
pub const XCB_GLX_DESTROY_PIXMAP: u8 = 23i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_destroy_pixmap_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub glx_pixmap: xcb_glx_pixmap_t,
}

impl Default for xcb_glx_destroy_pixmap_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_create_new_context.
pub const XCB_GLX_CREATE_NEW_CONTEXT: u8 = 24i32 as u8;

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

impl Default for xcb_glx_create_new_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_query_context_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_query_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_query_context.
pub const XCB_GLX_QUERY_CONTEXT: u8 = 25i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_query_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_glx_context_t,
}

impl Default for xcb_glx_query_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_query_context_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_make_context_current_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_make_context_current_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_make_context_current.
pub const XCB_GLX_MAKE_CONTEXT_CURRENT: u8 = 26i32 as u8;

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

impl Default for xcb_glx_make_context_current_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_make_context_current_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_create_pbuffer.
pub const XCB_GLX_CREATE_PBUFFER: u8 = 27i32 as u8;

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

impl Default for xcb_glx_create_pbuffer_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_destroy_pbuffer.
pub const XCB_GLX_DESTROY_PBUFFER: u8 = 28i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_destroy_pbuffer_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub pbuffer: xcb_glx_pbuffer_t,
}

impl Default for xcb_glx_destroy_pbuffer_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_drawable_attributes_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_drawable_attributes_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_drawable_attributes.
pub const XCB_GLX_GET_DRAWABLE_ATTRIBUTES: u8 = 29i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_drawable_attributes_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_glx_drawable_t,
}

impl Default for xcb_glx_get_drawable_attributes_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_drawable_attributes_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_change_drawable_attributes.
pub const XCB_GLX_CHANGE_DRAWABLE_ATTRIBUTES: u8 = 30i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_change_drawable_attributes_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_glx_drawable_t,
    pub num_attribs: u32,
}

impl Default for xcb_glx_change_drawable_attributes_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_create_window.
pub const XCB_GLX_CREATE_WINDOW: u8 = 31i32 as u8;

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

impl Default for xcb_glx_create_window_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_delete_window.
pub const XCB_GLX_DELETE_WINDOW: u8 = 32i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_delete_window_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub glxwindow: xcb_glx_window_t,
}

impl Default for xcb_glx_delete_window_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_set_client_info_arb.
pub const XCB_GLX_SET_CLIENT_INFO_ARB: u8 = 33i32 as u8;

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

impl Default for xcb_glx_set_client_info_arb_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_create_context_attribs_arb.
pub const XCB_GLX_CREATE_CONTEXT_ATTRIBS_ARB: u8 = 34i32 as u8;

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

impl Default for xcb_glx_create_context_attribs_arb_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_set_client_info_2arb.
pub const XCB_GLX_SET_CLIENT_INFO_2ARB: u8 = 35i32 as u8;

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

impl Default for xcb_glx_set_client_info_2arb_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_new_list.
pub const XCB_GLX_NEW_LIST: u8 = 101i32 as u8;

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

impl Default for xcb_glx_new_list_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_end_list.
pub const XCB_GLX_END_LIST: u8 = 102i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_end_list_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
}

impl Default for xcb_glx_end_list_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_delete_lists.
pub const XCB_GLX_DELETE_LISTS: u8 = 103i32 as u8;

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

impl Default for xcb_glx_delete_lists_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_gen_lists_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_gen_lists_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_gen_lists.
pub const XCB_GLX_GEN_LISTS: u8 = 104i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_gen_lists_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub range: i32,
}

impl Default for xcb_glx_gen_lists_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_gen_lists_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_feedback_buffer.
pub const XCB_GLX_FEEDBACK_BUFFER: u8 = 105i32 as u8;

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

impl Default for xcb_glx_feedback_buffer_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_select_buffer.
pub const XCB_GLX_SELECT_BUFFER: u8 = 106i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_select_buffer_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub size: i32,
}

impl Default for xcb_glx_select_buffer_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_render_mode_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_render_mode_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_render_mode.
pub const XCB_GLX_RENDER_MODE: u8 = 107i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_render_mode_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub mode: u32,
}

impl Default for xcb_glx_render_mode_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_render_mode_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_glx_rm_t = u32;
pub const XCB_GLX_RM_GL_RENDER: xcb_glx_rm_t = 7168;
pub const XCB_GLX_RM_GL_FEEDBACK: xcb_glx_rm_t = 7169;
pub const XCB_GLX_RM_GL_SELECT: xcb_glx_rm_t = 7170;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_finish_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_finish_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_finish.
pub const XCB_GLX_FINISH: u8 = 108i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_finish_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
}

impl Default for xcb_glx_finish_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_finish_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
}

impl Default for xcb_glx_finish_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_pixel_storef.
pub const XCB_GLX_PIXEL_STOREF: u8 = 109i32 as u8;

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

impl Default for xcb_glx_pixel_storef_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_pixel_storei.
pub const XCB_GLX_PIXEL_STOREI: u8 = 110i32 as u8;

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

impl Default for xcb_glx_pixel_storei_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_read_pixels_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_read_pixels_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_read_pixels.
pub const XCB_GLX_READ_PIXELS: u8 = 111i32 as u8;

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

impl Default for xcb_glx_read_pixels_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_read_pixels_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_booleanv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_booleanv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_booleanv.
pub const XCB_GLX_GET_BOOLEANV: u8 = 112i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_booleanv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub pname: i32,
}

impl Default for xcb_glx_get_booleanv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_booleanv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_clip_plane_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_clip_plane_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_clip_plane.
pub const XCB_GLX_GET_CLIP_PLANE: u8 = 113i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_clip_plane_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub plane: i32,
}

impl Default for xcb_glx_get_clip_plane_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_clip_plane_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_doublev_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_doublev_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_doublev.
pub const XCB_GLX_GET_DOUBLEV: u8 = 114i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_doublev_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub pname: u32,
}

impl Default for xcb_glx_get_doublev_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_doublev_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_error_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_error_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_error.
pub const XCB_GLX_GET_ERROR: u8 = 115i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_error_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
}

impl Default for xcb_glx_get_error_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_error_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_floatv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_floatv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_floatv.
pub const XCB_GLX_GET_FLOATV: u8 = 116i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_floatv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub pname: u32,
}

impl Default for xcb_glx_get_floatv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_floatv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_integerv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_integerv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_integerv.
pub const XCB_GLX_GET_INTEGERV: u8 = 117i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_integerv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub pname: u32,
}

impl Default for xcb_glx_get_integerv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_integerv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_lightfv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_lightfv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_lightfv.
pub const XCB_GLX_GET_LIGHTFV: u8 = 118i32 as u8;

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

impl Default for xcb_glx_get_lightfv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_lightfv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_lightiv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_lightiv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_lightiv.
pub const XCB_GLX_GET_LIGHTIV: u8 = 119i32 as u8;

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

impl Default for xcb_glx_get_lightiv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_lightiv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_mapdv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_mapdv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_mapdv.
pub const XCB_GLX_GET_MAPDV: u8 = 120i32 as u8;

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

impl Default for xcb_glx_get_mapdv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_mapdv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_mapfv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_mapfv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_mapfv.
pub const XCB_GLX_GET_MAPFV: u8 = 121i32 as u8;

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

impl Default for xcb_glx_get_mapfv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_mapfv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_mapiv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_mapiv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_mapiv.
pub const XCB_GLX_GET_MAPIV: u8 = 122i32 as u8;

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

impl Default for xcb_glx_get_mapiv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_mapiv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_materialfv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_materialfv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_materialfv.
pub const XCB_GLX_GET_MATERIALFV: u8 = 123i32 as u8;

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

impl Default for xcb_glx_get_materialfv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_materialfv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_materialiv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_materialiv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_materialiv.
pub const XCB_GLX_GET_MATERIALIV: u8 = 124i32 as u8;

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

impl Default for xcb_glx_get_materialiv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_materialiv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_pixel_mapfv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_pixel_mapfv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_pixel_mapfv.
pub const XCB_GLX_GET_PIXEL_MAPFV: u8 = 125i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_pixel_mapfv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub map: u32,
}

impl Default for xcb_glx_get_pixel_mapfv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_pixel_mapfv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_pixel_mapuiv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_pixel_mapuiv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_pixel_mapuiv.
pub const XCB_GLX_GET_PIXEL_MAPUIV: u8 = 126i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_pixel_mapuiv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub map: u32,
}

impl Default for xcb_glx_get_pixel_mapuiv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_pixel_mapuiv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_pixel_mapusv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_pixel_mapusv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_pixel_mapusv.
pub const XCB_GLX_GET_PIXEL_MAPUSV: u8 = 127i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_pixel_mapusv_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub map: u32,
}

impl Default for xcb_glx_get_pixel_mapusv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_pixel_mapusv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_polygon_stipple_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_polygon_stipple_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_polygon_stipple.
pub const XCB_GLX_GET_POLYGON_STIPPLE: u8 = 128i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_polygon_stipple_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub lsb_first: u8,
}

impl Default for xcb_glx_get_polygon_stipple_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_polygon_stipple_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_string_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_string_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_string.
pub const XCB_GLX_GET_STRING: u8 = 129i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_string_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub name: u32,
}

impl Default for xcb_glx_get_string_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_string_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_envfv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_tex_envfv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_tex_envfv.
pub const XCB_GLX_GET_TEX_ENVFV: u8 = 130i32 as u8;

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

impl Default for xcb_glx_get_tex_envfv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_tex_envfv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_enviv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_tex_enviv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_tex_enviv.
pub const XCB_GLX_GET_TEX_ENVIV: u8 = 131i32 as u8;

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

impl Default for xcb_glx_get_tex_enviv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_tex_enviv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_gendv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_tex_gendv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_tex_gendv.
pub const XCB_GLX_GET_TEX_GENDV: u8 = 132i32 as u8;

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

impl Default for xcb_glx_get_tex_gendv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_tex_gendv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_genfv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_tex_genfv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_tex_genfv.
pub const XCB_GLX_GET_TEX_GENFV: u8 = 133i32 as u8;

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

impl Default for xcb_glx_get_tex_genfv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_tex_genfv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_geniv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_tex_geniv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_tex_geniv.
pub const XCB_GLX_GET_TEX_GENIV: u8 = 134i32 as u8;

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

impl Default for xcb_glx_get_tex_geniv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_tex_geniv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_image_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_tex_image_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_tex_image.
pub const XCB_GLX_GET_TEX_IMAGE: u8 = 135i32 as u8;

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

impl Default for xcb_glx_get_tex_image_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_tex_image_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_parameterfv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_tex_parameterfv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_tex_parameterfv.
pub const XCB_GLX_GET_TEX_PARAMETERFV: u8 = 136i32 as u8;

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

impl Default for xcb_glx_get_tex_parameterfv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_tex_parameterfv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_parameteriv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_tex_parameteriv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_tex_parameteriv.
pub const XCB_GLX_GET_TEX_PARAMETERIV: u8 = 137i32 as u8;

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

impl Default for xcb_glx_get_tex_parameteriv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_tex_parameteriv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_level_parameterfv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_tex_level_parameterfv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_tex_level_parameterfv.
pub const XCB_GLX_GET_TEX_LEVEL_PARAMETERFV: u8 = 138i32 as u8;

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

impl Default for xcb_glx_get_tex_level_parameterfv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_tex_level_parameterfv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_tex_level_parameteriv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_tex_level_parameteriv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_tex_level_parameteriv.
pub const XCB_GLX_GET_TEX_LEVEL_PARAMETERIV: u8 = 139i32 as u8;

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

impl Default for xcb_glx_get_tex_level_parameteriv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_tex_level_parameteriv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_is_enabled_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_is_enabled_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_is_enabled.
pub const XCB_GLX_IS_ENABLED: u8 = 140i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_is_enabled_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub capability: u32,
}

impl Default for xcb_glx_is_enabled_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_is_enabled_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_is_list_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_is_list_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_is_list.
pub const XCB_GLX_IS_LIST: u8 = 141i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_is_list_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub list: u32,
}

impl Default for xcb_glx_is_list_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_is_list_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_flush.
pub const XCB_GLX_FLUSH: u8 = 142i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_flush_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
}

impl Default for xcb_glx_flush_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_are_textures_resident_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_are_textures_resident_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_are_textures_resident.
pub const XCB_GLX_ARE_TEXTURES_RESIDENT: u8 = 143i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_are_textures_resident_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub n: i32,
}

impl Default for xcb_glx_are_textures_resident_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_are_textures_resident_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_delete_textures.
pub const XCB_GLX_DELETE_TEXTURES: u8 = 144i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_delete_textures_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub n: i32,
}

impl Default for xcb_glx_delete_textures_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_gen_textures_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_gen_textures_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_gen_textures.
pub const XCB_GLX_GEN_TEXTURES: u8 = 145i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_gen_textures_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub n: i32,
}

impl Default for xcb_glx_gen_textures_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_gen_textures_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_is_texture_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_is_texture_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_is_texture.
pub const XCB_GLX_IS_TEXTURE: u8 = 146i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_is_texture_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub texture: u32,
}

impl Default for xcb_glx_is_texture_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_is_texture_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_color_table_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_color_table_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_color_table.
pub const XCB_GLX_GET_COLOR_TABLE: u8 = 147i32 as u8;

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

impl Default for xcb_glx_get_color_table_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_color_table_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_color_table_parameterfv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_color_table_parameterfv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_color_table_parameterfv.
pub const XCB_GLX_GET_COLOR_TABLE_PARAMETERFV: u8 = 148i32 as u8;

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

impl Default for xcb_glx_get_color_table_parameterfv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_color_table_parameterfv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_color_table_parameteriv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_color_table_parameteriv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_color_table_parameteriv.
pub const XCB_GLX_GET_COLOR_TABLE_PARAMETERIV: u8 = 149i32 as u8;

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

impl Default for xcb_glx_get_color_table_parameteriv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_color_table_parameteriv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_convolution_filter_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_convolution_filter_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_convolution_filter.
pub const XCB_GLX_GET_CONVOLUTION_FILTER: u8 = 150i32 as u8;

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

impl Default for xcb_glx_get_convolution_filter_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_convolution_filter_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_convolution_parameterfv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_convolution_parameterfv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_convolution_parameterfv.
pub const XCB_GLX_GET_CONVOLUTION_PARAMETERFV: u8 = 151i32 as u8;

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

impl Default for xcb_glx_get_convolution_parameterfv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_convolution_parameterfv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_convolution_parameteriv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_convolution_parameteriv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_convolution_parameteriv.
pub const XCB_GLX_GET_CONVOLUTION_PARAMETERIV: u8 = 152i32 as u8;

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

impl Default for xcb_glx_get_convolution_parameteriv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_convolution_parameteriv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_separable_filter_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_separable_filter_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_separable_filter.
pub const XCB_GLX_GET_SEPARABLE_FILTER: u8 = 153i32 as u8;

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

impl Default for xcb_glx_get_separable_filter_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_separable_filter_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_histogram_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_histogram_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_histogram.
pub const XCB_GLX_GET_HISTOGRAM: u8 = 154i32 as u8;

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

impl Default for xcb_glx_get_histogram_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_histogram_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_histogram_parameterfv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_histogram_parameterfv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_histogram_parameterfv.
pub const XCB_GLX_GET_HISTOGRAM_PARAMETERFV: u8 = 155i32 as u8;

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

impl Default for xcb_glx_get_histogram_parameterfv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_histogram_parameterfv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_histogram_parameteriv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_histogram_parameteriv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_histogram_parameteriv.
pub const XCB_GLX_GET_HISTOGRAM_PARAMETERIV: u8 = 156i32 as u8;

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

impl Default for xcb_glx_get_histogram_parameteriv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_histogram_parameteriv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_minmax_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_minmax_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_minmax.
pub const XCB_GLX_GET_MINMAX: u8 = 157i32 as u8;

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

impl Default for xcb_glx_get_minmax_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_minmax_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_minmax_parameterfv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_minmax_parameterfv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_minmax_parameterfv.
pub const XCB_GLX_GET_MINMAX_PARAMETERFV: u8 = 158i32 as u8;

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

impl Default for xcb_glx_get_minmax_parameterfv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_minmax_parameterfv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_minmax_parameteriv_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_minmax_parameteriv_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_minmax_parameteriv.
pub const XCB_GLX_GET_MINMAX_PARAMETERIV: u8 = 159i32 as u8;

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

impl Default for xcb_glx_get_minmax_parameteriv_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_minmax_parameteriv_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_compressed_tex_image_arb_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_compressed_tex_image_arb_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_compressed_tex_image_arb.
pub const XCB_GLX_GET_COMPRESSED_TEX_IMAGE_ARB: u8 = 160i32 as u8;

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

impl Default for xcb_glx_get_compressed_tex_image_arb_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_compressed_tex_image_arb_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_delete_queries_arb.
pub const XCB_GLX_DELETE_QUERIES_ARB: u8 = 161i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_delete_queries_arb_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub n: i32,
}

impl Default for xcb_glx_delete_queries_arb_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_gen_queries_arb_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_gen_queries_arb_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_gen_queries_arb.
pub const XCB_GLX_GEN_QUERIES_ARB: u8 = 162i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_gen_queries_arb_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub n: i32,
}

impl Default for xcb_glx_gen_queries_arb_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_gen_queries_arb_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_is_query_arb_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_is_query_arb_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_is_query_arb.
pub const XCB_GLX_IS_QUERY_ARB: u8 = 163i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_is_query_arb_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_tag: xcb_glx_context_tag_t,
    pub id: u32,
}

impl Default for xcb_glx_is_query_arb_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_is_query_arb_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_queryiv_arb_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_queryiv_arb_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_queryiv_arb.
pub const XCB_GLX_GET_QUERYIV_ARB: u8 = 164i32 as u8;

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

impl Default for xcb_glx_get_queryiv_arb_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_queryiv_arb_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_query_objectiv_arb_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_query_objectiv_arb_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_query_objectiv_arb.
pub const XCB_GLX_GET_QUERY_OBJECTIV_ARB: u8 = 165i32 as u8;

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

impl Default for xcb_glx_get_query_objectiv_arb_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_query_objectiv_arb_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_glx_get_query_objectuiv_arb_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_glx_get_query_objectuiv_arb_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_glx_get_query_objectuiv_arb.
pub const XCB_GLX_GET_QUERY_OBJECTUIV_ARB: u8 = 166i32 as u8;

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

impl Default for xcb_glx_get_query_objectuiv_arb_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_glx_get_query_objectuiv_arb_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_glx")]
pub(crate) struct XcbGlxGlx {
    xcb_glx_id: LazySymbol<*mut xcb_extension_t>,
    xcb_glx_pixmap_next: LazySymbol<unsafe fn(i: *mut xcb_glx_pixmap_iterator_t)>,
    xcb_glx_pixmap_end:
        LazySymbol<unsafe fn(i: xcb_glx_pixmap_iterator_t) -> xcb_generic_iterator_t>,
    xcb_glx_context_next: LazySymbol<unsafe fn(i: *mut xcb_glx_context_iterator_t)>,
    xcb_glx_context_end:
        LazySymbol<unsafe fn(i: xcb_glx_context_iterator_t) -> xcb_generic_iterator_t>,
    xcb_glx_pbuffer_next: LazySymbol<unsafe fn(i: *mut xcb_glx_pbuffer_iterator_t)>,
    xcb_glx_pbuffer_end:
        LazySymbol<unsafe fn(i: xcb_glx_pbuffer_iterator_t) -> xcb_generic_iterator_t>,
    xcb_glx_window_next: LazySymbol<unsafe fn(i: *mut xcb_glx_window_iterator_t)>,
    xcb_glx_window_end:
        LazySymbol<unsafe fn(i: xcb_glx_window_iterator_t) -> xcb_generic_iterator_t>,
    xcb_glx_fbconfig_next: LazySymbol<unsafe fn(i: *mut xcb_glx_fbconfig_iterator_t)>,
    xcb_glx_fbconfig_end:
        LazySymbol<unsafe fn(i: xcb_glx_fbconfig_iterator_t) -> xcb_generic_iterator_t>,
    xcb_glx_drawable_next: LazySymbol<unsafe fn(i: *mut xcb_glx_drawable_iterator_t)>,
    xcb_glx_drawable_end:
        LazySymbol<unsafe fn(i: xcb_glx_drawable_iterator_t) -> xcb_generic_iterator_t>,
    xcb_glx_float32_next: LazySymbol<unsafe fn(i: *mut xcb_glx_float32_iterator_t)>,
    xcb_glx_float32_end:
        LazySymbol<unsafe fn(i: xcb_glx_float32_iterator_t) -> xcb_generic_iterator_t>,
    xcb_glx_float64_next: LazySymbol<unsafe fn(i: *mut xcb_glx_float64_iterator_t)>,
    xcb_glx_float64_end:
        LazySymbol<unsafe fn(i: xcb_glx_float64_iterator_t) -> xcb_generic_iterator_t>,
    xcb_glx_bool32_next: LazySymbol<unsafe fn(i: *mut xcb_glx_bool32_iterator_t)>,
    xcb_glx_bool32_end:
        LazySymbol<unsafe fn(i: xcb_glx_bool32_iterator_t) -> xcb_generic_iterator_t>,
    xcb_glx_context_tag_next: LazySymbol<unsafe fn(i: *mut xcb_glx_context_tag_iterator_t)>,
    xcb_glx_context_tag_end:
        LazySymbol<unsafe fn(i: xcb_glx_context_tag_iterator_t) -> xcb_generic_iterator_t>,
    xcb_glx_render_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void, data_len: u32) -> c_int>,
    xcb_glx_render_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            data_len: u32,
            data: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_render: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            data_len: u32,
            data: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_render_data: LazySymbol<unsafe fn(r: *const xcb_glx_render_request_t) -> *mut u8>,
    xcb_glx_render_data_length: LazySymbol<unsafe fn(r: *const xcb_glx_render_request_t) -> c_int>,
    xcb_glx_render_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_render_request_t) -> xcb_generic_iterator_t>,
    xcb_glx_render_large_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_render_large_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            request_num: u16,
            request_total: u16,
            data_len: u32,
            data: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_render_large: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            request_num: u16,
            request_total: u16,
            data_len: u32,
            data: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_render_large_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_render_large_request_t) -> *mut u8>,
    xcb_glx_render_large_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_render_large_request_t) -> c_int>,
    xcb_glx_render_large_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_render_large_request_t) -> xcb_generic_iterator_t>,
    xcb_glx_create_context_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_glx_context_t,
            visual: xcb_visualid_t,
            screen: u32,
            share_list: xcb_glx_context_t,
            is_direct: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_create_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_glx_context_t,
            visual: xcb_visualid_t,
            screen: u32,
            share_list: xcb_glx_context_t,
            is_direct: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_destroy_context_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, context: xcb_glx_context_t) -> xcb_void_cookie_t,
    >,
    xcb_glx_destroy_context: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, context: xcb_glx_context_t) -> xcb_void_cookie_t,
    >,
    xcb_glx_make_current: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_glx_drawable_t,
            context: xcb_glx_context_t,
            old_context_tag: xcb_glx_context_tag_t,
        ) -> xcb_glx_make_current_cookie_t,
    >,
    xcb_glx_make_current_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_glx_drawable_t,
            context: xcb_glx_context_t,
            old_context_tag: xcb_glx_context_tag_t,
        ) -> xcb_glx_make_current_cookie_t,
    >,
    xcb_glx_make_current_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_make_current_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_make_current_reply_t,
    >,
    xcb_glx_is_direct: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_glx_context_t,
        ) -> xcb_glx_is_direct_cookie_t,
    >,
    xcb_glx_is_direct_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_glx_context_t,
        ) -> xcb_glx_is_direct_cookie_t,
    >,
    xcb_glx_is_direct_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_is_direct_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_is_direct_reply_t,
    >,
    xcb_glx_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u32,
            minor_version: u32,
        ) -> xcb_glx_query_version_cookie_t,
    >,
    xcb_glx_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u32,
            minor_version: u32,
        ) -> xcb_glx_query_version_cookie_t,
    >,
    xcb_glx_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_query_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_query_version_reply_t,
    >,
    xcb_glx_wait_gl_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_wait_gl: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_wait_x_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_wait_x: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_copy_context_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src: xcb_glx_context_t,
            dest: xcb_glx_context_t,
            mask: u32,
            src_context_tag: xcb_glx_context_tag_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_copy_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src: xcb_glx_context_t,
            dest: xcb_glx_context_t,
            mask: u32,
            src_context_tag: xcb_glx_context_tag_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_swap_buffers_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            drawable: xcb_glx_drawable_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_swap_buffers: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            drawable: xcb_glx_drawable_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_use_x_font_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            font: xcb_font_t,
            first: u32,
            count: u32,
            list_base: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_use_x_font: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            font: xcb_font_t,
            first: u32,
            count: u32,
            list_base: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_create_glx_pixmap_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            visual: xcb_visualid_t,
            pixmap: xcb_pixmap_t,
            glx_pixmap: xcb_glx_pixmap_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_create_glx_pixmap: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            visual: xcb_visualid_t,
            pixmap: xcb_pixmap_t,
            glx_pixmap: xcb_glx_pixmap_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_get_visual_configs_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_visual_configs: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, screen: u32) -> xcb_glx_get_visual_configs_cookie_t,
    >,
    xcb_glx_get_visual_configs_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, screen: u32) -> xcb_glx_get_visual_configs_cookie_t,
    >,
    xcb_glx_get_visual_configs_property_list:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_visual_configs_reply_t) -> *mut u32>,
    xcb_glx_get_visual_configs_property_list_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_visual_configs_reply_t) -> c_int>,
    xcb_glx_get_visual_configs_property_list_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_visual_configs_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_get_visual_configs_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_visual_configs_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_visual_configs_reply_t,
    >,
    xcb_glx_destroy_glx_pixmap_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, glx_pixmap: xcb_glx_pixmap_t) -> xcb_void_cookie_t,
    >,
    xcb_glx_destroy_glx_pixmap: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, glx_pixmap: xcb_glx_pixmap_t) -> xcb_void_cookie_t,
    >,
    xcb_glx_vendor_private_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, data_len: u32) -> c_int>,
    xcb_glx_vendor_private_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            vendor_code: u32,
            context_tag: xcb_glx_context_tag_t,
            data_len: u32,
            data: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_vendor_private: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            vendor_code: u32,
            context_tag: xcb_glx_context_tag_t,
            data_len: u32,
            data: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_vendor_private_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_vendor_private_request_t) -> *mut u8>,
    xcb_glx_vendor_private_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_vendor_private_request_t) -> c_int>,
    xcb_glx_vendor_private_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_vendor_private_request_t) -> xcb_generic_iterator_t>,
    xcb_glx_vendor_private_with_reply_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, data_len: u32) -> c_int>,
    xcb_glx_vendor_private_with_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            vendor_code: u32,
            context_tag: xcb_glx_context_tag_t,
            data_len: u32,
            data: *const u8,
        ) -> xcb_glx_vendor_private_with_reply_cookie_t,
    >,
    xcb_glx_vendor_private_with_reply_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            vendor_code: u32,
            context_tag: xcb_glx_context_tag_t,
            data_len: u32,
            data: *const u8,
        ) -> xcb_glx_vendor_private_with_reply_cookie_t,
    >,
    xcb_glx_vendor_private_with_reply_data_2:
        LazySymbol<unsafe fn(r: *const xcb_glx_vendor_private_with_reply_reply_t) -> *mut u8>,
    xcb_glx_vendor_private_with_reply_data_2_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_vendor_private_with_reply_reply_t) -> c_int>,
    xcb_glx_vendor_private_with_reply_data_2_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_vendor_private_with_reply_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_vendor_private_with_reply_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_vendor_private_with_reply_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_vendor_private_with_reply_reply_t,
    >,
    xcb_glx_query_extensions_string: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
        ) -> xcb_glx_query_extensions_string_cookie_t,
    >,
    xcb_glx_query_extensions_string_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
        ) -> xcb_glx_query_extensions_string_cookie_t,
    >,
    xcb_glx_query_extensions_string_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_query_extensions_string_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_query_extensions_string_reply_t,
    >,
    xcb_glx_query_server_string_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_query_server_string: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            name: u32,
        ) -> xcb_glx_query_server_string_cookie_t,
    >,
    xcb_glx_query_server_string_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            name: u32,
        ) -> xcb_glx_query_server_string_cookie_t,
    >,
    xcb_glx_query_server_string_string:
        LazySymbol<unsafe fn(r: *const xcb_glx_query_server_string_reply_t) -> *mut c_char>,
    xcb_glx_query_server_string_string_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_query_server_string_reply_t) -> c_int>,
    xcb_glx_query_server_string_string_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_query_server_string_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_query_server_string_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_query_server_string_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_query_server_string_reply_t,
    >,
    xcb_glx_client_info_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_client_info_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u32,
            minor_version: u32,
            str_len: u32,
            string: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_client_info: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u32,
            minor_version: u32,
            str_len: u32,
            string: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_client_info_string:
        LazySymbol<unsafe fn(r: *const xcb_glx_client_info_request_t) -> *mut c_char>,
    xcb_glx_client_info_string_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_client_info_request_t) -> c_int>,
    xcb_glx_client_info_string_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_client_info_request_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_fb_configs_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_fb_configs: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, screen: u32) -> xcb_glx_get_fb_configs_cookie_t,
    >,
    xcb_glx_get_fb_configs_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, screen: u32) -> xcb_glx_get_fb_configs_cookie_t,
    >,
    xcb_glx_get_fb_configs_property_list:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_fb_configs_reply_t) -> *mut u32>,
    xcb_glx_get_fb_configs_property_list_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_fb_configs_reply_t) -> c_int>,
    xcb_glx_get_fb_configs_property_list_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_fb_configs_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_fb_configs_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_fb_configs_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_fb_configs_reply_t,
    >,
    xcb_glx_create_pixmap_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_create_pixmap_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            fbconfig: xcb_glx_fbconfig_t,
            pixmap: xcb_pixmap_t,
            glx_pixmap: xcb_glx_pixmap_t,
            num_attribs: u32,
            attribs: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_create_pixmap: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            fbconfig: xcb_glx_fbconfig_t,
            pixmap: xcb_pixmap_t,
            glx_pixmap: xcb_glx_pixmap_t,
            num_attribs: u32,
            attribs: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_create_pixmap_attribs:
        LazySymbol<unsafe fn(r: *const xcb_glx_create_pixmap_request_t) -> *mut u32>,
    xcb_glx_create_pixmap_attribs_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_create_pixmap_request_t) -> c_int>,
    xcb_glx_create_pixmap_attribs_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_create_pixmap_request_t) -> xcb_generic_iterator_t>,
    xcb_glx_destroy_pixmap_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, glx_pixmap: xcb_glx_pixmap_t) -> xcb_void_cookie_t,
    >,
    xcb_glx_destroy_pixmap: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, glx_pixmap: xcb_glx_pixmap_t) -> xcb_void_cookie_t,
    >,
    xcb_glx_create_new_context_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_glx_context_t,
            fbconfig: xcb_glx_fbconfig_t,
            screen: u32,
            render_type: u32,
            share_list: xcb_glx_context_t,
            is_direct: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_create_new_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_glx_context_t,
            fbconfig: xcb_glx_fbconfig_t,
            screen: u32,
            render_type: u32,
            share_list: xcb_glx_context_t,
            is_direct: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_query_context_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_query_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_glx_context_t,
        ) -> xcb_glx_query_context_cookie_t,
    >,
    xcb_glx_query_context_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_glx_context_t,
        ) -> xcb_glx_query_context_cookie_t,
    >,
    xcb_glx_query_context_attribs:
        LazySymbol<unsafe fn(r: *const xcb_glx_query_context_reply_t) -> *mut u32>,
    xcb_glx_query_context_attribs_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_query_context_reply_t) -> c_int>,
    xcb_glx_query_context_attribs_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_query_context_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_query_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_query_context_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_query_context_reply_t,
    >,
    xcb_glx_make_context_current: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            old_context_tag: xcb_glx_context_tag_t,
            drawable: xcb_glx_drawable_t,
            read_drawable: xcb_glx_drawable_t,
            context: xcb_glx_context_t,
        ) -> xcb_glx_make_context_current_cookie_t,
    >,
    xcb_glx_make_context_current_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            old_context_tag: xcb_glx_context_tag_t,
            drawable: xcb_glx_drawable_t,
            read_drawable: xcb_glx_drawable_t,
            context: xcb_glx_context_t,
        ) -> xcb_glx_make_context_current_cookie_t,
    >,
    xcb_glx_make_context_current_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_make_context_current_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_make_context_current_reply_t,
    >,
    xcb_glx_create_pbuffer_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_create_pbuffer_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            fbconfig: xcb_glx_fbconfig_t,
            pbuffer: xcb_glx_pbuffer_t,
            num_attribs: u32,
            attribs: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_create_pbuffer: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            fbconfig: xcb_glx_fbconfig_t,
            pbuffer: xcb_glx_pbuffer_t,
            num_attribs: u32,
            attribs: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_create_pbuffer_attribs:
        LazySymbol<unsafe fn(r: *const xcb_glx_create_pbuffer_request_t) -> *mut u32>,
    xcb_glx_create_pbuffer_attribs_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_create_pbuffer_request_t) -> c_int>,
    xcb_glx_create_pbuffer_attribs_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_create_pbuffer_request_t) -> xcb_generic_iterator_t>,
    xcb_glx_destroy_pbuffer_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, pbuffer: xcb_glx_pbuffer_t) -> xcb_void_cookie_t,
    >,
    xcb_glx_destroy_pbuffer: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, pbuffer: xcb_glx_pbuffer_t) -> xcb_void_cookie_t,
    >,
    xcb_glx_get_drawable_attributes_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_drawable_attributes: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_glx_drawable_t,
        ) -> xcb_glx_get_drawable_attributes_cookie_t,
    >,
    xcb_glx_get_drawable_attributes_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_glx_drawable_t,
        ) -> xcb_glx_get_drawable_attributes_cookie_t,
    >,
    xcb_glx_get_drawable_attributes_attribs:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_drawable_attributes_reply_t) -> *mut u32>,
    xcb_glx_get_drawable_attributes_attribs_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_drawable_attributes_reply_t) -> c_int>,
    xcb_glx_get_drawable_attributes_attribs_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_drawable_attributes_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_get_drawable_attributes_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_drawable_attributes_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_drawable_attributes_reply_t,
    >,
    xcb_glx_change_drawable_attributes_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_change_drawable_attributes_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_glx_drawable_t,
            num_attribs: u32,
            attribs: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_change_drawable_attributes: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_glx_drawable_t,
            num_attribs: u32,
            attribs: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_change_drawable_attributes_attribs:
        LazySymbol<unsafe fn(r: *const xcb_glx_change_drawable_attributes_request_t) -> *mut u32>,
    xcb_glx_change_drawable_attributes_attribs_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_change_drawable_attributes_request_t) -> c_int>,
    xcb_glx_change_drawable_attributes_attribs_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_change_drawable_attributes_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_create_window_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_create_window_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            fbconfig: xcb_glx_fbconfig_t,
            window: xcb_window_t,
            glx_window: xcb_glx_window_t,
            num_attribs: u32,
            attribs: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_create_window: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            fbconfig: xcb_glx_fbconfig_t,
            window: xcb_window_t,
            glx_window: xcb_glx_window_t,
            num_attribs: u32,
            attribs: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_create_window_attribs:
        LazySymbol<unsafe fn(r: *const xcb_glx_create_window_request_t) -> *mut u32>,
    xcb_glx_create_window_attribs_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_create_window_request_t) -> c_int>,
    xcb_glx_create_window_attribs_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_create_window_request_t) -> xcb_generic_iterator_t>,
    xcb_glx_delete_window_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, glxwindow: xcb_glx_window_t) -> xcb_void_cookie_t,
    >,
    xcb_glx_delete_window: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, glxwindow: xcb_glx_window_t) -> xcb_void_cookie_t,
    >,
    xcb_glx_set_client_info_arb_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_set_client_info_arb_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u32,
            minor_version: u32,
            num_versions: u32,
            gl_str_len: u32,
            glx_str_len: u32,
            gl_versions: *const u32,
            gl_extension_string: *const c_char,
            glx_extension_string: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_set_client_info_arb: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u32,
            minor_version: u32,
            num_versions: u32,
            gl_str_len: u32,
            glx_str_len: u32,
            gl_versions: *const u32,
            gl_extension_string: *const c_char,
            glx_extension_string: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_set_client_info_arb_gl_versions:
        LazySymbol<unsafe fn(r: *const xcb_glx_set_client_info_arb_request_t) -> *mut u32>,
    xcb_glx_set_client_info_arb_gl_versions_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_set_client_info_arb_request_t) -> c_int>,
    xcb_glx_set_client_info_arb_gl_versions_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_set_client_info_arb_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_set_client_info_arb_gl_extension_string:
        LazySymbol<unsafe fn(r: *const xcb_glx_set_client_info_arb_request_t) -> *mut c_char>,
    xcb_glx_set_client_info_arb_gl_extension_string_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_set_client_info_arb_request_t) -> c_int>,
    xcb_glx_set_client_info_arb_gl_extension_string_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_set_client_info_arb_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_set_client_info_arb_glx_extension_string:
        LazySymbol<unsafe fn(r: *const xcb_glx_set_client_info_arb_request_t) -> *mut c_char>,
    xcb_glx_set_client_info_arb_glx_extension_string_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_set_client_info_arb_request_t) -> c_int>,
    xcb_glx_set_client_info_arb_glx_extension_string_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_set_client_info_arb_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_create_context_attribs_arb_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_create_context_attribs_arb_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_glx_context_t,
            fbconfig: xcb_glx_fbconfig_t,
            screen: u32,
            share_list: xcb_glx_context_t,
            is_direct: u8,
            num_attribs: u32,
            attribs: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_create_context_attribs_arb: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_glx_context_t,
            fbconfig: xcb_glx_fbconfig_t,
            screen: u32,
            share_list: xcb_glx_context_t,
            is_direct: u8,
            num_attribs: u32,
            attribs: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_create_context_attribs_arb_attribs:
        LazySymbol<unsafe fn(r: *const xcb_glx_create_context_attribs_arb_request_t) -> *mut u32>,
    xcb_glx_create_context_attribs_arb_attribs_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_create_context_attribs_arb_request_t) -> c_int>,
    xcb_glx_create_context_attribs_arb_attribs_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_create_context_attribs_arb_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_set_client_info_2arb_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_set_client_info_2arb_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u32,
            minor_version: u32,
            num_versions: u32,
            gl_str_len: u32,
            glx_str_len: u32,
            gl_versions: *const u32,
            gl_extension_string: *const c_char,
            glx_extension_string: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_set_client_info_2arb: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u32,
            minor_version: u32,
            num_versions: u32,
            gl_str_len: u32,
            glx_str_len: u32,
            gl_versions: *const u32,
            gl_extension_string: *const c_char,
            glx_extension_string: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_set_client_info_2arb_gl_versions:
        LazySymbol<unsafe fn(r: *const xcb_glx_set_client_info_2arb_request_t) -> *mut u32>,
    xcb_glx_set_client_info_2arb_gl_versions_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_set_client_info_2arb_request_t) -> c_int>,
    xcb_glx_set_client_info_2arb_gl_versions_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_set_client_info_2arb_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_set_client_info_2arb_gl_extension_string:
        LazySymbol<unsafe fn(r: *const xcb_glx_set_client_info_2arb_request_t) -> *mut c_char>,
    xcb_glx_set_client_info_2arb_gl_extension_string_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_set_client_info_2arb_request_t) -> c_int>,
    xcb_glx_set_client_info_2arb_gl_extension_string_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_set_client_info_2arb_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_set_client_info_2arb_glx_extension_string:
        LazySymbol<unsafe fn(r: *const xcb_glx_set_client_info_2arb_request_t) -> *mut c_char>,
    xcb_glx_set_client_info_2arb_glx_extension_string_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_set_client_info_2arb_request_t) -> c_int>,
    xcb_glx_set_client_info_2arb_glx_extension_string_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_set_client_info_2arb_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_new_list_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            list: u32,
            mode: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_new_list: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            list: u32,
            mode: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_end_list_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_end_list: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_delete_lists_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            list: u32,
            range: i32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_delete_lists: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            list: u32,
            range: i32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_gen_lists: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            range: i32,
        ) -> xcb_glx_gen_lists_cookie_t,
    >,
    xcb_glx_gen_lists_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            range: i32,
        ) -> xcb_glx_gen_lists_cookie_t,
    >,
    xcb_glx_gen_lists_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_gen_lists_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_gen_lists_reply_t,
    >,
    xcb_glx_feedback_buffer_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            size: i32,
            type_: i32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_feedback_buffer: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            size: i32,
            type_: i32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_select_buffer_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            size: i32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_select_buffer: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            size: i32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_render_mode_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_render_mode: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            mode: u32,
        ) -> xcb_glx_render_mode_cookie_t,
    >,
    xcb_glx_render_mode_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            mode: u32,
        ) -> xcb_glx_render_mode_cookie_t,
    >,
    xcb_glx_render_mode_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_render_mode_reply_t) -> *mut u32>,
    xcb_glx_render_mode_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_render_mode_reply_t) -> c_int>,
    xcb_glx_render_mode_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_render_mode_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_render_mode_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_render_mode_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_render_mode_reply_t,
    >,
    xcb_glx_finish: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
        ) -> xcb_glx_finish_cookie_t,
    >,
    xcb_glx_finish_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
        ) -> xcb_glx_finish_cookie_t,
    >,
    xcb_glx_finish_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_finish_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_finish_reply_t,
    >,
    xcb_glx_pixel_storef_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            pname: u32,
            datum: xcb_glx_float32_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_pixel_storef: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            pname: u32,
            datum: xcb_glx_float32_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_pixel_storei_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            pname: u32,
            datum: i32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_pixel_storei: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            pname: u32,
            datum: i32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_read_pixels_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_read_pixels: LazySymbol<
        unsafe fn(
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
        ) -> xcb_glx_read_pixels_cookie_t,
    >,
    xcb_glx_read_pixels_unchecked: LazySymbol<
        unsafe fn(
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
        ) -> xcb_glx_read_pixels_cookie_t,
    >,
    xcb_glx_read_pixels_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_read_pixels_reply_t) -> *mut u8>,
    xcb_glx_read_pixels_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_read_pixels_reply_t) -> c_int>,
    xcb_glx_read_pixels_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_read_pixels_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_read_pixels_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_read_pixels_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_read_pixels_reply_t,
    >,
    xcb_glx_get_booleanv_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_booleanv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            pname: i32,
        ) -> xcb_glx_get_booleanv_cookie_t,
    >,
    xcb_glx_get_booleanv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            pname: i32,
        ) -> xcb_glx_get_booleanv_cookie_t,
    >,
    xcb_glx_get_booleanv_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_booleanv_reply_t) -> *mut u8>,
    xcb_glx_get_booleanv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_booleanv_reply_t) -> c_int>,
    xcb_glx_get_booleanv_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_booleanv_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_booleanv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_booleanv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_booleanv_reply_t,
    >,
    xcb_glx_get_clip_plane_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_clip_plane: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            plane: i32,
        ) -> xcb_glx_get_clip_plane_cookie_t,
    >,
    xcb_glx_get_clip_plane_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            plane: i32,
        ) -> xcb_glx_get_clip_plane_cookie_t,
    >,
    xcb_glx_get_clip_plane_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_clip_plane_reply_t) -> *mut xcb_glx_float64_t>,
    xcb_glx_get_clip_plane_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_clip_plane_reply_t) -> c_int>,
    xcb_glx_get_clip_plane_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_clip_plane_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_clip_plane_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_clip_plane_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_clip_plane_reply_t,
    >,
    xcb_glx_get_doublev_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_doublev: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            pname: u32,
        ) -> xcb_glx_get_doublev_cookie_t,
    >,
    xcb_glx_get_doublev_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            pname: u32,
        ) -> xcb_glx_get_doublev_cookie_t,
    >,
    xcb_glx_get_doublev_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_doublev_reply_t) -> *mut xcb_glx_float64_t>,
    xcb_glx_get_doublev_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_doublev_reply_t) -> c_int>,
    xcb_glx_get_doublev_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_doublev_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_doublev_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_doublev_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_doublev_reply_t,
    >,
    xcb_glx_get_error: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
        ) -> xcb_glx_get_error_cookie_t,
    >,
    xcb_glx_get_error_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
        ) -> xcb_glx_get_error_cookie_t,
    >,
    xcb_glx_get_error_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_error_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_error_reply_t,
    >,
    xcb_glx_get_floatv_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_floatv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            pname: u32,
        ) -> xcb_glx_get_floatv_cookie_t,
    >,
    xcb_glx_get_floatv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            pname: u32,
        ) -> xcb_glx_get_floatv_cookie_t,
    >,
    xcb_glx_get_floatv_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_floatv_reply_t) -> *mut xcb_glx_float32_t>,
    xcb_glx_get_floatv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_floatv_reply_t) -> c_int>,
    xcb_glx_get_floatv_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_floatv_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_floatv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_floatv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_floatv_reply_t,
    >,
    xcb_glx_get_integerv_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_integerv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            pname: u32,
        ) -> xcb_glx_get_integerv_cookie_t,
    >,
    xcb_glx_get_integerv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            pname: u32,
        ) -> xcb_glx_get_integerv_cookie_t,
    >,
    xcb_glx_get_integerv_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_integerv_reply_t) -> *mut i32>,
    xcb_glx_get_integerv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_integerv_reply_t) -> c_int>,
    xcb_glx_get_integerv_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_integerv_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_integerv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_integerv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_integerv_reply_t,
    >,
    xcb_glx_get_lightfv_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_lightfv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            light: u32,
            pname: u32,
        ) -> xcb_glx_get_lightfv_cookie_t,
    >,
    xcb_glx_get_lightfv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            light: u32,
            pname: u32,
        ) -> xcb_glx_get_lightfv_cookie_t,
    >,
    xcb_glx_get_lightfv_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_lightfv_reply_t) -> *mut xcb_glx_float32_t>,
    xcb_glx_get_lightfv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_lightfv_reply_t) -> c_int>,
    xcb_glx_get_lightfv_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_lightfv_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_lightfv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_lightfv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_lightfv_reply_t,
    >,
    xcb_glx_get_lightiv_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_lightiv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            light: u32,
            pname: u32,
        ) -> xcb_glx_get_lightiv_cookie_t,
    >,
    xcb_glx_get_lightiv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            light: u32,
            pname: u32,
        ) -> xcb_glx_get_lightiv_cookie_t,
    >,
    xcb_glx_get_lightiv_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_lightiv_reply_t) -> *mut i32>,
    xcb_glx_get_lightiv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_lightiv_reply_t) -> c_int>,
    xcb_glx_get_lightiv_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_lightiv_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_lightiv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_lightiv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_lightiv_reply_t,
    >,
    xcb_glx_get_mapdv_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_mapdv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            query: u32,
        ) -> xcb_glx_get_mapdv_cookie_t,
    >,
    xcb_glx_get_mapdv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            query: u32,
        ) -> xcb_glx_get_mapdv_cookie_t,
    >,
    xcb_glx_get_mapdv_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_mapdv_reply_t) -> *mut xcb_glx_float64_t>,
    xcb_glx_get_mapdv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_mapdv_reply_t) -> c_int>,
    xcb_glx_get_mapdv_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_mapdv_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_mapdv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_mapdv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_mapdv_reply_t,
    >,
    xcb_glx_get_mapfv_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_mapfv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            query: u32,
        ) -> xcb_glx_get_mapfv_cookie_t,
    >,
    xcb_glx_get_mapfv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            query: u32,
        ) -> xcb_glx_get_mapfv_cookie_t,
    >,
    xcb_glx_get_mapfv_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_mapfv_reply_t) -> *mut xcb_glx_float32_t>,
    xcb_glx_get_mapfv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_mapfv_reply_t) -> c_int>,
    xcb_glx_get_mapfv_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_mapfv_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_mapfv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_mapfv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_mapfv_reply_t,
    >,
    xcb_glx_get_mapiv_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_mapiv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            query: u32,
        ) -> xcb_glx_get_mapiv_cookie_t,
    >,
    xcb_glx_get_mapiv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            query: u32,
        ) -> xcb_glx_get_mapiv_cookie_t,
    >,
    xcb_glx_get_mapiv_data: LazySymbol<unsafe fn(r: *const xcb_glx_get_mapiv_reply_t) -> *mut i32>,
    xcb_glx_get_mapiv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_mapiv_reply_t) -> c_int>,
    xcb_glx_get_mapiv_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_mapiv_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_mapiv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_mapiv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_mapiv_reply_t,
    >,
    xcb_glx_get_materialfv_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_materialfv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            face: u32,
            pname: u32,
        ) -> xcb_glx_get_materialfv_cookie_t,
    >,
    xcb_glx_get_materialfv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            face: u32,
            pname: u32,
        ) -> xcb_glx_get_materialfv_cookie_t,
    >,
    xcb_glx_get_materialfv_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_materialfv_reply_t) -> *mut xcb_glx_float32_t>,
    xcb_glx_get_materialfv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_materialfv_reply_t) -> c_int>,
    xcb_glx_get_materialfv_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_materialfv_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_materialfv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_materialfv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_materialfv_reply_t,
    >,
    xcb_glx_get_materialiv_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_materialiv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            face: u32,
            pname: u32,
        ) -> xcb_glx_get_materialiv_cookie_t,
    >,
    xcb_glx_get_materialiv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            face: u32,
            pname: u32,
        ) -> xcb_glx_get_materialiv_cookie_t,
    >,
    xcb_glx_get_materialiv_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_materialiv_reply_t) -> *mut i32>,
    xcb_glx_get_materialiv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_materialiv_reply_t) -> c_int>,
    xcb_glx_get_materialiv_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_materialiv_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_materialiv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_materialiv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_materialiv_reply_t,
    >,
    xcb_glx_get_pixel_mapfv_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_pixel_mapfv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            map: u32,
        ) -> xcb_glx_get_pixel_mapfv_cookie_t,
    >,
    xcb_glx_get_pixel_mapfv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            map: u32,
        ) -> xcb_glx_get_pixel_mapfv_cookie_t,
    >,
    xcb_glx_get_pixel_mapfv_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_pixel_mapfv_reply_t) -> *mut xcb_glx_float32_t>,
    xcb_glx_get_pixel_mapfv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_pixel_mapfv_reply_t) -> c_int>,
    xcb_glx_get_pixel_mapfv_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_pixel_mapfv_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_pixel_mapfv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_pixel_mapfv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_pixel_mapfv_reply_t,
    >,
    xcb_glx_get_pixel_mapuiv_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_pixel_mapuiv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            map: u32,
        ) -> xcb_glx_get_pixel_mapuiv_cookie_t,
    >,
    xcb_glx_get_pixel_mapuiv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            map: u32,
        ) -> xcb_glx_get_pixel_mapuiv_cookie_t,
    >,
    xcb_glx_get_pixel_mapuiv_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_pixel_mapuiv_reply_t) -> *mut u32>,
    xcb_glx_get_pixel_mapuiv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_pixel_mapuiv_reply_t) -> c_int>,
    xcb_glx_get_pixel_mapuiv_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_pixel_mapuiv_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_pixel_mapuiv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_pixel_mapuiv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_pixel_mapuiv_reply_t,
    >,
    xcb_glx_get_pixel_mapusv_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_pixel_mapusv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            map: u32,
        ) -> xcb_glx_get_pixel_mapusv_cookie_t,
    >,
    xcb_glx_get_pixel_mapusv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            map: u32,
        ) -> xcb_glx_get_pixel_mapusv_cookie_t,
    >,
    xcb_glx_get_pixel_mapusv_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_pixel_mapusv_reply_t) -> *mut u16>,
    xcb_glx_get_pixel_mapusv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_pixel_mapusv_reply_t) -> c_int>,
    xcb_glx_get_pixel_mapusv_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_pixel_mapusv_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_pixel_mapusv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_pixel_mapusv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_pixel_mapusv_reply_t,
    >,
    xcb_glx_get_polygon_stipple_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_polygon_stipple: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            lsb_first: u8,
        ) -> xcb_glx_get_polygon_stipple_cookie_t,
    >,
    xcb_glx_get_polygon_stipple_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            lsb_first: u8,
        ) -> xcb_glx_get_polygon_stipple_cookie_t,
    >,
    xcb_glx_get_polygon_stipple_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_polygon_stipple_reply_t) -> *mut u8>,
    xcb_glx_get_polygon_stipple_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_polygon_stipple_reply_t) -> c_int>,
    xcb_glx_get_polygon_stipple_data_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_polygon_stipple_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_get_polygon_stipple_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_polygon_stipple_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_polygon_stipple_reply_t,
    >,
    xcb_glx_get_string_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_string: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            name: u32,
        ) -> xcb_glx_get_string_cookie_t,
    >,
    xcb_glx_get_string_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            name: u32,
        ) -> xcb_glx_get_string_cookie_t,
    >,
    xcb_glx_get_string_string:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_string_reply_t) -> *mut c_char>,
    xcb_glx_get_string_string_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_string_reply_t) -> c_int>,
    xcb_glx_get_string_string_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_string_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_string_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_string_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_string_reply_t,
    >,
    xcb_glx_get_tex_envfv_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_tex_envfv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_tex_envfv_cookie_t,
    >,
    xcb_glx_get_tex_envfv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_tex_envfv_cookie_t,
    >,
    xcb_glx_get_tex_envfv_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_tex_envfv_reply_t) -> *mut xcb_glx_float32_t>,
    xcb_glx_get_tex_envfv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_tex_envfv_reply_t) -> c_int>,
    xcb_glx_get_tex_envfv_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_tex_envfv_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_tex_envfv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_tex_envfv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_tex_envfv_reply_t,
    >,
    xcb_glx_get_tex_enviv_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_tex_enviv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_tex_enviv_cookie_t,
    >,
    xcb_glx_get_tex_enviv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_tex_enviv_cookie_t,
    >,
    xcb_glx_get_tex_enviv_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_tex_enviv_reply_t) -> *mut i32>,
    xcb_glx_get_tex_enviv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_tex_enviv_reply_t) -> c_int>,
    xcb_glx_get_tex_enviv_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_tex_enviv_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_tex_enviv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_tex_enviv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_tex_enviv_reply_t,
    >,
    xcb_glx_get_tex_gendv_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_tex_gendv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            coord: u32,
            pname: u32,
        ) -> xcb_glx_get_tex_gendv_cookie_t,
    >,
    xcb_glx_get_tex_gendv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            coord: u32,
            pname: u32,
        ) -> xcb_glx_get_tex_gendv_cookie_t,
    >,
    xcb_glx_get_tex_gendv_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_tex_gendv_reply_t) -> *mut xcb_glx_float64_t>,
    xcb_glx_get_tex_gendv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_tex_gendv_reply_t) -> c_int>,
    xcb_glx_get_tex_gendv_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_tex_gendv_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_tex_gendv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_tex_gendv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_tex_gendv_reply_t,
    >,
    xcb_glx_get_tex_genfv_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_tex_genfv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            coord: u32,
            pname: u32,
        ) -> xcb_glx_get_tex_genfv_cookie_t,
    >,
    xcb_glx_get_tex_genfv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            coord: u32,
            pname: u32,
        ) -> xcb_glx_get_tex_genfv_cookie_t,
    >,
    xcb_glx_get_tex_genfv_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_tex_genfv_reply_t) -> *mut xcb_glx_float32_t>,
    xcb_glx_get_tex_genfv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_tex_genfv_reply_t) -> c_int>,
    xcb_glx_get_tex_genfv_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_tex_genfv_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_tex_genfv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_tex_genfv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_tex_genfv_reply_t,
    >,
    xcb_glx_get_tex_geniv_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_tex_geniv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            coord: u32,
            pname: u32,
        ) -> xcb_glx_get_tex_geniv_cookie_t,
    >,
    xcb_glx_get_tex_geniv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            coord: u32,
            pname: u32,
        ) -> xcb_glx_get_tex_geniv_cookie_t,
    >,
    xcb_glx_get_tex_geniv_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_tex_geniv_reply_t) -> *mut i32>,
    xcb_glx_get_tex_geniv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_tex_geniv_reply_t) -> c_int>,
    xcb_glx_get_tex_geniv_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_tex_geniv_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_tex_geniv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_tex_geniv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_tex_geniv_reply_t,
    >,
    xcb_glx_get_tex_image_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_tex_image: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            level: i32,
            format: u32,
            type_: u32,
            swap_bytes: u8,
        ) -> xcb_glx_get_tex_image_cookie_t,
    >,
    xcb_glx_get_tex_image_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            level: i32,
            format: u32,
            type_: u32,
            swap_bytes: u8,
        ) -> xcb_glx_get_tex_image_cookie_t,
    >,
    xcb_glx_get_tex_image_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_tex_image_reply_t) -> *mut u8>,
    xcb_glx_get_tex_image_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_tex_image_reply_t) -> c_int>,
    xcb_glx_get_tex_image_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_tex_image_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_tex_image_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_tex_image_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_tex_image_reply_t,
    >,
    xcb_glx_get_tex_parameterfv_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_tex_parameterfv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_tex_parameterfv_cookie_t,
    >,
    xcb_glx_get_tex_parameterfv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_tex_parameterfv_cookie_t,
    >,
    xcb_glx_get_tex_parameterfv_data: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_tex_parameterfv_reply_t) -> *mut xcb_glx_float32_t,
    >,
    xcb_glx_get_tex_parameterfv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_tex_parameterfv_reply_t) -> c_int>,
    xcb_glx_get_tex_parameterfv_data_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_tex_parameterfv_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_get_tex_parameterfv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_tex_parameterfv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_tex_parameterfv_reply_t,
    >,
    xcb_glx_get_tex_parameteriv_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_tex_parameteriv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_tex_parameteriv_cookie_t,
    >,
    xcb_glx_get_tex_parameteriv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_tex_parameteriv_cookie_t,
    >,
    xcb_glx_get_tex_parameteriv_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_tex_parameteriv_reply_t) -> *mut i32>,
    xcb_glx_get_tex_parameteriv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_tex_parameteriv_reply_t) -> c_int>,
    xcb_glx_get_tex_parameteriv_data_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_tex_parameteriv_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_get_tex_parameteriv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_tex_parameteriv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_tex_parameteriv_reply_t,
    >,
    xcb_glx_get_tex_level_parameterfv_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_tex_level_parameterfv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            level: i32,
            pname: u32,
        ) -> xcb_glx_get_tex_level_parameterfv_cookie_t,
    >,
    xcb_glx_get_tex_level_parameterfv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            level: i32,
            pname: u32,
        ) -> xcb_glx_get_tex_level_parameterfv_cookie_t,
    >,
    xcb_glx_get_tex_level_parameterfv_data: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_tex_level_parameterfv_reply_t) -> *mut xcb_glx_float32_t,
    >,
    xcb_glx_get_tex_level_parameterfv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_tex_level_parameterfv_reply_t) -> c_int>,
    xcb_glx_get_tex_level_parameterfv_data_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_tex_level_parameterfv_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_get_tex_level_parameterfv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_tex_level_parameterfv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_tex_level_parameterfv_reply_t,
    >,
    xcb_glx_get_tex_level_parameteriv_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_tex_level_parameteriv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            level: i32,
            pname: u32,
        ) -> xcb_glx_get_tex_level_parameteriv_cookie_t,
    >,
    xcb_glx_get_tex_level_parameteriv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            level: i32,
            pname: u32,
        ) -> xcb_glx_get_tex_level_parameteriv_cookie_t,
    >,
    xcb_glx_get_tex_level_parameteriv_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_tex_level_parameteriv_reply_t) -> *mut i32>,
    xcb_glx_get_tex_level_parameteriv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_tex_level_parameteriv_reply_t) -> c_int>,
    xcb_glx_get_tex_level_parameteriv_data_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_tex_level_parameteriv_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_get_tex_level_parameteriv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_tex_level_parameteriv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_tex_level_parameteriv_reply_t,
    >,
    xcb_glx_is_enabled: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            capability: u32,
        ) -> xcb_glx_is_enabled_cookie_t,
    >,
    xcb_glx_is_enabled_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            capability: u32,
        ) -> xcb_glx_is_enabled_cookie_t,
    >,
    xcb_glx_is_enabled_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_is_enabled_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_is_enabled_reply_t,
    >,
    xcb_glx_is_list: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            list: u32,
        ) -> xcb_glx_is_list_cookie_t,
    >,
    xcb_glx_is_list_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            list: u32,
        ) -> xcb_glx_is_list_cookie_t,
    >,
    xcb_glx_is_list_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_is_list_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_is_list_reply_t,
    >,
    xcb_glx_flush_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_flush: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_are_textures_resident_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_are_textures_resident: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            n: i32,
            textures: *const u32,
        ) -> xcb_glx_are_textures_resident_cookie_t,
    >,
    xcb_glx_are_textures_resident_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            n: i32,
            textures: *const u32,
        ) -> xcb_glx_are_textures_resident_cookie_t,
    >,
    xcb_glx_are_textures_resident_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_are_textures_resident_reply_t) -> *mut u8>,
    xcb_glx_are_textures_resident_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_are_textures_resident_reply_t) -> c_int>,
    xcb_glx_are_textures_resident_data_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_are_textures_resident_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_are_textures_resident_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_are_textures_resident_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_are_textures_resident_reply_t,
    >,
    xcb_glx_delete_textures_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_delete_textures_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            n: i32,
            textures: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_delete_textures: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            n: i32,
            textures: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_delete_textures_textures:
        LazySymbol<unsafe fn(r: *const xcb_glx_delete_textures_request_t) -> *mut u32>,
    xcb_glx_delete_textures_textures_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_delete_textures_request_t) -> c_int>,
    xcb_glx_delete_textures_textures_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_delete_textures_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_gen_textures_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_gen_textures: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            n: i32,
        ) -> xcb_glx_gen_textures_cookie_t,
    >,
    xcb_glx_gen_textures_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            n: i32,
        ) -> xcb_glx_gen_textures_cookie_t,
    >,
    xcb_glx_gen_textures_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_gen_textures_reply_t) -> *mut u32>,
    xcb_glx_gen_textures_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_gen_textures_reply_t) -> c_int>,
    xcb_glx_gen_textures_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_gen_textures_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_gen_textures_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_gen_textures_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_gen_textures_reply_t,
    >,
    xcb_glx_is_texture: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            texture: u32,
        ) -> xcb_glx_is_texture_cookie_t,
    >,
    xcb_glx_is_texture_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            texture: u32,
        ) -> xcb_glx_is_texture_cookie_t,
    >,
    xcb_glx_is_texture_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_is_texture_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_is_texture_reply_t,
    >,
    xcb_glx_get_color_table_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_color_table: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            format: u32,
            type_: u32,
            swap_bytes: u8,
        ) -> xcb_glx_get_color_table_cookie_t,
    >,
    xcb_glx_get_color_table_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            format: u32,
            type_: u32,
            swap_bytes: u8,
        ) -> xcb_glx_get_color_table_cookie_t,
    >,
    xcb_glx_get_color_table_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_color_table_reply_t) -> *mut u8>,
    xcb_glx_get_color_table_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_color_table_reply_t) -> c_int>,
    xcb_glx_get_color_table_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_color_table_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_color_table_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_color_table_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_color_table_reply_t,
    >,
    xcb_glx_get_color_table_parameterfv_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_color_table_parameterfv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_color_table_parameterfv_cookie_t,
    >,
    xcb_glx_get_color_table_parameterfv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_color_table_parameterfv_cookie_t,
    >,
    xcb_glx_get_color_table_parameterfv_data: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_color_table_parameterfv_reply_t) -> *mut xcb_glx_float32_t,
    >,
    xcb_glx_get_color_table_parameterfv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_color_table_parameterfv_reply_t) -> c_int>,
    xcb_glx_get_color_table_parameterfv_data_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_color_table_parameterfv_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_get_color_table_parameterfv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_color_table_parameterfv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_color_table_parameterfv_reply_t,
    >,
    xcb_glx_get_color_table_parameteriv_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_color_table_parameteriv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_color_table_parameteriv_cookie_t,
    >,
    xcb_glx_get_color_table_parameteriv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_color_table_parameteriv_cookie_t,
    >,
    xcb_glx_get_color_table_parameteriv_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_color_table_parameteriv_reply_t) -> *mut i32>,
    xcb_glx_get_color_table_parameteriv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_color_table_parameteriv_reply_t) -> c_int>,
    xcb_glx_get_color_table_parameteriv_data_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_color_table_parameteriv_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_get_color_table_parameteriv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_color_table_parameteriv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_color_table_parameteriv_reply_t,
    >,
    xcb_glx_get_convolution_filter_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_convolution_filter: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            format: u32,
            type_: u32,
            swap_bytes: u8,
        ) -> xcb_glx_get_convolution_filter_cookie_t,
    >,
    xcb_glx_get_convolution_filter_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            format: u32,
            type_: u32,
            swap_bytes: u8,
        ) -> xcb_glx_get_convolution_filter_cookie_t,
    >,
    xcb_glx_get_convolution_filter_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_convolution_filter_reply_t) -> *mut u8>,
    xcb_glx_get_convolution_filter_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_convolution_filter_reply_t) -> c_int>,
    xcb_glx_get_convolution_filter_data_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_convolution_filter_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_get_convolution_filter_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_convolution_filter_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_convolution_filter_reply_t,
    >,
    xcb_glx_get_convolution_parameterfv_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_convolution_parameterfv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_convolution_parameterfv_cookie_t,
    >,
    xcb_glx_get_convolution_parameterfv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_convolution_parameterfv_cookie_t,
    >,
    xcb_glx_get_convolution_parameterfv_data: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_convolution_parameterfv_reply_t) -> *mut xcb_glx_float32_t,
    >,
    xcb_glx_get_convolution_parameterfv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_convolution_parameterfv_reply_t) -> c_int>,
    xcb_glx_get_convolution_parameterfv_data_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_convolution_parameterfv_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_get_convolution_parameterfv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_convolution_parameterfv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_convolution_parameterfv_reply_t,
    >,
    xcb_glx_get_convolution_parameteriv_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_convolution_parameteriv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_convolution_parameteriv_cookie_t,
    >,
    xcb_glx_get_convolution_parameteriv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_convolution_parameteriv_cookie_t,
    >,
    xcb_glx_get_convolution_parameteriv_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_convolution_parameteriv_reply_t) -> *mut i32>,
    xcb_glx_get_convolution_parameteriv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_convolution_parameteriv_reply_t) -> c_int>,
    xcb_glx_get_convolution_parameteriv_data_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_convolution_parameteriv_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_get_convolution_parameteriv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_convolution_parameteriv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_convolution_parameteriv_reply_t,
    >,
    xcb_glx_get_separable_filter_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_separable_filter: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            format: u32,
            type_: u32,
            swap_bytes: u8,
        ) -> xcb_glx_get_separable_filter_cookie_t,
    >,
    xcb_glx_get_separable_filter_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            format: u32,
            type_: u32,
            swap_bytes: u8,
        ) -> xcb_glx_get_separable_filter_cookie_t,
    >,
    xcb_glx_get_separable_filter_rows_and_cols:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_separable_filter_reply_t) -> *mut u8>,
    xcb_glx_get_separable_filter_rows_and_cols_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_separable_filter_reply_t) -> c_int>,
    xcb_glx_get_separable_filter_rows_and_cols_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_separable_filter_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_get_separable_filter_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_separable_filter_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_separable_filter_reply_t,
    >,
    xcb_glx_get_histogram_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_histogram: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            format: u32,
            type_: u32,
            swap_bytes: u8,
            reset: u8,
        ) -> xcb_glx_get_histogram_cookie_t,
    >,
    xcb_glx_get_histogram_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            format: u32,
            type_: u32,
            swap_bytes: u8,
            reset: u8,
        ) -> xcb_glx_get_histogram_cookie_t,
    >,
    xcb_glx_get_histogram_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_histogram_reply_t) -> *mut u8>,
    xcb_glx_get_histogram_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_histogram_reply_t) -> c_int>,
    xcb_glx_get_histogram_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_histogram_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_histogram_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_histogram_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_histogram_reply_t,
    >,
    xcb_glx_get_histogram_parameterfv_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_histogram_parameterfv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_histogram_parameterfv_cookie_t,
    >,
    xcb_glx_get_histogram_parameterfv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_histogram_parameterfv_cookie_t,
    >,
    xcb_glx_get_histogram_parameterfv_data: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_histogram_parameterfv_reply_t) -> *mut xcb_glx_float32_t,
    >,
    xcb_glx_get_histogram_parameterfv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_histogram_parameterfv_reply_t) -> c_int>,
    xcb_glx_get_histogram_parameterfv_data_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_histogram_parameterfv_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_get_histogram_parameterfv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_histogram_parameterfv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_histogram_parameterfv_reply_t,
    >,
    xcb_glx_get_histogram_parameteriv_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_histogram_parameteriv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_histogram_parameteriv_cookie_t,
    >,
    xcb_glx_get_histogram_parameteriv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_histogram_parameteriv_cookie_t,
    >,
    xcb_glx_get_histogram_parameteriv_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_histogram_parameteriv_reply_t) -> *mut i32>,
    xcb_glx_get_histogram_parameteriv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_histogram_parameteriv_reply_t) -> c_int>,
    xcb_glx_get_histogram_parameteriv_data_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_histogram_parameteriv_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_get_histogram_parameteriv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_histogram_parameteriv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_histogram_parameteriv_reply_t,
    >,
    xcb_glx_get_minmax_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_minmax: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            format: u32,
            type_: u32,
            swap_bytes: u8,
            reset: u8,
        ) -> xcb_glx_get_minmax_cookie_t,
    >,
    xcb_glx_get_minmax_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            format: u32,
            type_: u32,
            swap_bytes: u8,
            reset: u8,
        ) -> xcb_glx_get_minmax_cookie_t,
    >,
    xcb_glx_get_minmax_data: LazySymbol<unsafe fn(r: *const xcb_glx_get_minmax_reply_t) -> *mut u8>,
    xcb_glx_get_minmax_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_minmax_reply_t) -> c_int>,
    xcb_glx_get_minmax_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_minmax_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_minmax_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_minmax_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_minmax_reply_t,
    >,
    xcb_glx_get_minmax_parameterfv_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_minmax_parameterfv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_minmax_parameterfv_cookie_t,
    >,
    xcb_glx_get_minmax_parameterfv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_minmax_parameterfv_cookie_t,
    >,
    xcb_glx_get_minmax_parameterfv_data: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_minmax_parameterfv_reply_t) -> *mut xcb_glx_float32_t,
    >,
    xcb_glx_get_minmax_parameterfv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_minmax_parameterfv_reply_t) -> c_int>,
    xcb_glx_get_minmax_parameterfv_data_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_minmax_parameterfv_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_get_minmax_parameterfv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_minmax_parameterfv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_minmax_parameterfv_reply_t,
    >,
    xcb_glx_get_minmax_parameteriv_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_minmax_parameteriv: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_minmax_parameteriv_cookie_t,
    >,
    xcb_glx_get_minmax_parameteriv_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_minmax_parameteriv_cookie_t,
    >,
    xcb_glx_get_minmax_parameteriv_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_minmax_parameteriv_reply_t) -> *mut i32>,
    xcb_glx_get_minmax_parameteriv_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_minmax_parameteriv_reply_t) -> c_int>,
    xcb_glx_get_minmax_parameteriv_data_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_minmax_parameteriv_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_get_minmax_parameteriv_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_minmax_parameteriv_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_minmax_parameteriv_reply_t,
    >,
    xcb_glx_get_compressed_tex_image_arb_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_compressed_tex_image_arb: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            level: i32,
        ) -> xcb_glx_get_compressed_tex_image_arb_cookie_t,
    >,
    xcb_glx_get_compressed_tex_image_arb_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            level: i32,
        ) -> xcb_glx_get_compressed_tex_image_arb_cookie_t,
    >,
    xcb_glx_get_compressed_tex_image_arb_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_compressed_tex_image_arb_reply_t) -> *mut u8>,
    xcb_glx_get_compressed_tex_image_arb_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_compressed_tex_image_arb_reply_t) -> c_int>,
    xcb_glx_get_compressed_tex_image_arb_data_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_compressed_tex_image_arb_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_get_compressed_tex_image_arb_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_compressed_tex_image_arb_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_compressed_tex_image_arb_reply_t,
    >,
    xcb_glx_delete_queries_arb_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_delete_queries_arb_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            n: i32,
            ids: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_delete_queries_arb: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            n: i32,
            ids: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_glx_delete_queries_arb_ids:
        LazySymbol<unsafe fn(r: *const xcb_glx_delete_queries_arb_request_t) -> *mut u32>,
    xcb_glx_delete_queries_arb_ids_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_delete_queries_arb_request_t) -> c_int>,
    xcb_glx_delete_queries_arb_ids_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_delete_queries_arb_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_gen_queries_arb_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_gen_queries_arb: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            n: i32,
        ) -> xcb_glx_gen_queries_arb_cookie_t,
    >,
    xcb_glx_gen_queries_arb_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            n: i32,
        ) -> xcb_glx_gen_queries_arb_cookie_t,
    >,
    xcb_glx_gen_queries_arb_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_gen_queries_arb_reply_t) -> *mut u32>,
    xcb_glx_gen_queries_arb_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_gen_queries_arb_reply_t) -> c_int>,
    xcb_glx_gen_queries_arb_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_gen_queries_arb_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_gen_queries_arb_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_gen_queries_arb_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_gen_queries_arb_reply_t,
    >,
    xcb_glx_is_query_arb: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            id: u32,
        ) -> xcb_glx_is_query_arb_cookie_t,
    >,
    xcb_glx_is_query_arb_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            id: u32,
        ) -> xcb_glx_is_query_arb_cookie_t,
    >,
    xcb_glx_is_query_arb_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_is_query_arb_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_is_query_arb_reply_t,
    >,
    xcb_glx_get_queryiv_arb_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_queryiv_arb: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_queryiv_arb_cookie_t,
    >,
    xcb_glx_get_queryiv_arb_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            target: u32,
            pname: u32,
        ) -> xcb_glx_get_queryiv_arb_cookie_t,
    >,
    xcb_glx_get_queryiv_arb_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_queryiv_arb_reply_t) -> *mut i32>,
    xcb_glx_get_queryiv_arb_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_queryiv_arb_reply_t) -> c_int>,
    xcb_glx_get_queryiv_arb_data_end:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_queryiv_arb_reply_t) -> xcb_generic_iterator_t>,
    xcb_glx_get_queryiv_arb_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_queryiv_arb_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_queryiv_arb_reply_t,
    >,
    xcb_glx_get_query_objectiv_arb_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_query_objectiv_arb: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            id: u32,
            pname: u32,
        ) -> xcb_glx_get_query_objectiv_arb_cookie_t,
    >,
    xcb_glx_get_query_objectiv_arb_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            id: u32,
            pname: u32,
        ) -> xcb_glx_get_query_objectiv_arb_cookie_t,
    >,
    xcb_glx_get_query_objectiv_arb_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_query_objectiv_arb_reply_t) -> *mut i32>,
    xcb_glx_get_query_objectiv_arb_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_query_objectiv_arb_reply_t) -> c_int>,
    xcb_glx_get_query_objectiv_arb_data_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_query_objectiv_arb_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_get_query_objectiv_arb_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_query_objectiv_arb_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_query_objectiv_arb_reply_t,
    >,
    xcb_glx_get_query_objectuiv_arb_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_glx_get_query_objectuiv_arb: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            id: u32,
            pname: u32,
        ) -> xcb_glx_get_query_objectuiv_arb_cookie_t,
    >,
    xcb_glx_get_query_objectuiv_arb_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_tag: xcb_glx_context_tag_t,
            id: u32,
            pname: u32,
        ) -> xcb_glx_get_query_objectuiv_arb_cookie_t,
    >,
    xcb_glx_get_query_objectuiv_arb_data:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_query_objectuiv_arb_reply_t) -> *mut u32>,
    xcb_glx_get_query_objectuiv_arb_data_length:
        LazySymbol<unsafe fn(r: *const xcb_glx_get_query_objectuiv_arb_reply_t) -> c_int>,
    xcb_glx_get_query_objectuiv_arb_data_end: LazySymbol<
        unsafe fn(r: *const xcb_glx_get_query_objectuiv_arb_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_glx_get_query_objectuiv_arb_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_glx_get_query_objectuiv_arb_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_glx_get_query_objectuiv_arb_reply_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self.glx.$f.get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .glx
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_glx")]
impl XcbGlx {
    pub fn xcb_glx_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_glx_id) }
    }

    /// Returns `true` iff the symbol `xcb_glx_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_id(&self) -> bool {
        has_sym!(self, xcb_glx_id)
    }

    pub unsafe fn xcb_glx_pixmap_next(&self, i: *mut xcb_glx_pixmap_iterator_t) {
        sym!(self, xcb_glx_pixmap_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_glx_pixmap_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_pixmap_next(&self) -> bool {
        has_sym!(self, xcb_glx_pixmap_next)
    }

    pub unsafe fn xcb_glx_pixmap_end(
        &self,
        i: xcb_glx_pixmap_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_pixmap_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_glx_pixmap_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_pixmap_end(&self) -> bool {
        has_sym!(self, xcb_glx_pixmap_end)
    }

    pub unsafe fn xcb_glx_context_next(&self, i: *mut xcb_glx_context_iterator_t) {
        sym!(self, xcb_glx_context_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_glx_context_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_context_next(&self) -> bool {
        has_sym!(self, xcb_glx_context_next)
    }

    pub unsafe fn xcb_glx_context_end(
        &self,
        i: xcb_glx_context_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_context_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_glx_context_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_context_end(&self) -> bool {
        has_sym!(self, xcb_glx_context_end)
    }

    pub unsafe fn xcb_glx_pbuffer_next(&self, i: *mut xcb_glx_pbuffer_iterator_t) {
        sym!(self, xcb_glx_pbuffer_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_glx_pbuffer_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_pbuffer_next(&self) -> bool {
        has_sym!(self, xcb_glx_pbuffer_next)
    }

    pub unsafe fn xcb_glx_pbuffer_end(
        &self,
        i: xcb_glx_pbuffer_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_pbuffer_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_glx_pbuffer_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_pbuffer_end(&self) -> bool {
        has_sym!(self, xcb_glx_pbuffer_end)
    }

    pub unsafe fn xcb_glx_window_next(&self, i: *mut xcb_glx_window_iterator_t) {
        sym!(self, xcb_glx_window_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_glx_window_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_window_next(&self) -> bool {
        has_sym!(self, xcb_glx_window_next)
    }

    pub unsafe fn xcb_glx_window_end(
        &self,
        i: xcb_glx_window_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_window_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_glx_window_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_window_end(&self) -> bool {
        has_sym!(self, xcb_glx_window_end)
    }

    pub unsafe fn xcb_glx_fbconfig_next(&self, i: *mut xcb_glx_fbconfig_iterator_t) {
        sym!(self, xcb_glx_fbconfig_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_glx_fbconfig_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_fbconfig_next(&self) -> bool {
        has_sym!(self, xcb_glx_fbconfig_next)
    }

    pub unsafe fn xcb_glx_fbconfig_end(
        &self,
        i: xcb_glx_fbconfig_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_fbconfig_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_glx_fbconfig_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_fbconfig_end(&self) -> bool {
        has_sym!(self, xcb_glx_fbconfig_end)
    }

    pub unsafe fn xcb_glx_drawable_next(&self, i: *mut xcb_glx_drawable_iterator_t) {
        sym!(self, xcb_glx_drawable_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_glx_drawable_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_drawable_next(&self) -> bool {
        has_sym!(self, xcb_glx_drawable_next)
    }

    pub unsafe fn xcb_glx_drawable_end(
        &self,
        i: xcb_glx_drawable_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_drawable_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_glx_drawable_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_drawable_end(&self) -> bool {
        has_sym!(self, xcb_glx_drawable_end)
    }

    pub unsafe fn xcb_glx_float32_next(&self, i: *mut xcb_glx_float32_iterator_t) {
        sym!(self, xcb_glx_float32_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_glx_float32_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_float32_next(&self) -> bool {
        has_sym!(self, xcb_glx_float32_next)
    }

    pub unsafe fn xcb_glx_float32_end(
        &self,
        i: xcb_glx_float32_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_float32_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_glx_float32_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_float32_end(&self) -> bool {
        has_sym!(self, xcb_glx_float32_end)
    }

    pub unsafe fn xcb_glx_float64_next(&self, i: *mut xcb_glx_float64_iterator_t) {
        sym!(self, xcb_glx_float64_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_glx_float64_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_float64_next(&self) -> bool {
        has_sym!(self, xcb_glx_float64_next)
    }

    pub unsafe fn xcb_glx_float64_end(
        &self,
        i: xcb_glx_float64_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_float64_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_glx_float64_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_float64_end(&self) -> bool {
        has_sym!(self, xcb_glx_float64_end)
    }

    pub unsafe fn xcb_glx_bool32_next(&self, i: *mut xcb_glx_bool32_iterator_t) {
        sym!(self, xcb_glx_bool32_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_glx_bool32_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_bool32_next(&self) -> bool {
        has_sym!(self, xcb_glx_bool32_next)
    }

    pub unsafe fn xcb_glx_bool32_end(
        &self,
        i: xcb_glx_bool32_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_bool32_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_glx_bool32_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_bool32_end(&self) -> bool {
        has_sym!(self, xcb_glx_bool32_end)
    }

    pub unsafe fn xcb_glx_context_tag_next(&self, i: *mut xcb_glx_context_tag_iterator_t) {
        sym!(self, xcb_glx_context_tag_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_glx_context_tag_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_context_tag_next(&self) -> bool {
        has_sym!(self, xcb_glx_context_tag_next)
    }

    pub unsafe fn xcb_glx_context_tag_end(
        &self,
        i: xcb_glx_context_tag_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_context_tag_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_glx_context_tag_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_context_tag_end(&self) -> bool {
        has_sym!(self, xcb_glx_context_tag_end)
    }

    pub unsafe fn xcb_glx_render_sizeof(&self, _buffer: *const c_void, data_len: u32) -> c_int {
        sym!(self, xcb_glx_render_sizeof)(_buffer, data_len)
    }

    /// Returns `true` iff the symbol `xcb_glx_render_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_render_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_render_sizeof)
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
    pub unsafe fn xcb_glx_render_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        data_len: u32,
        data: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_render_checked)(c, context_tag, data_len, data)
    }

    /// Returns `true` iff the symbol `xcb_glx_render_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_render_checked(&self) -> bool {
        has_sym!(self, xcb_glx_render_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_render(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        data_len: u32,
        data: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_render)(c, context_tag, data_len, data)
    }

    /// Returns `true` iff the symbol `xcb_glx_render` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_render(&self) -> bool {
        has_sym!(self, xcb_glx_render)
    }

    pub unsafe fn xcb_glx_render_data(&self, r: *const xcb_glx_render_request_t) -> *mut u8 {
        sym!(self, xcb_glx_render_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_render_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_render_data(&self) -> bool {
        has_sym!(self, xcb_glx_render_data)
    }

    pub unsafe fn xcb_glx_render_data_length(&self, r: *const xcb_glx_render_request_t) -> c_int {
        sym!(self, xcb_glx_render_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_render_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_render_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_render_data_length)
    }

    pub unsafe fn xcb_glx_render_data_end(
        &self,
        r: *const xcb_glx_render_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_render_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_render_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_render_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_render_data_end)
    }

    pub unsafe fn xcb_glx_render_large_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_render_large_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_render_large_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_render_large_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_render_large_sizeof)
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

    /// Returns `true` iff the symbol `xcb_glx_render_large_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_render_large_checked(&self) -> bool {
        has_sym!(self, xcb_glx_render_large_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_glx_render_large` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_render_large(&self) -> bool {
        has_sym!(self, xcb_glx_render_large)
    }

    pub unsafe fn xcb_glx_render_large_data(
        &self,
        r: *const xcb_glx_render_large_request_t,
    ) -> *mut u8 {
        sym!(self, xcb_glx_render_large_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_render_large_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_render_large_data(&self) -> bool {
        has_sym!(self, xcb_glx_render_large_data)
    }

    pub unsafe fn xcb_glx_render_large_data_length(
        &self,
        r: *const xcb_glx_render_large_request_t,
    ) -> c_int {
        sym!(self, xcb_glx_render_large_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_render_large_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_render_large_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_render_large_data_length)
    }

    pub unsafe fn xcb_glx_render_large_data_end(
        &self,
        r: *const xcb_glx_render_large_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_render_large_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_render_large_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_render_large_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_render_large_data_end)
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

    /// Returns `true` iff the symbol `xcb_glx_create_context_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_context_checked(&self) -> bool {
        has_sym!(self, xcb_glx_create_context_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_glx_create_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_context(&self) -> bool {
        has_sym!(self, xcb_glx_create_context)
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
    pub unsafe fn xcb_glx_destroy_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_glx_context_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_destroy_context_checked)(c, context)
    }

    /// Returns `true` iff the symbol `xcb_glx_destroy_context_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_destroy_context_checked(&self) -> bool {
        has_sym!(self, xcb_glx_destroy_context_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_destroy_context(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_glx_context_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_destroy_context)(c, context)
    }

    /// Returns `true` iff the symbol `xcb_glx_destroy_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_destroy_context(&self) -> bool {
        has_sym!(self, xcb_glx_destroy_context)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_make_current(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_glx_drawable_t,
        context: xcb_glx_context_t,
        old_context_tag: xcb_glx_context_tag_t,
    ) -> xcb_glx_make_current_cookie_t {
        sym!(self, xcb_glx_make_current)(c, drawable, context, old_context_tag)
    }

    /// Returns `true` iff the symbol `xcb_glx_make_current` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_make_current(&self) -> bool {
        has_sym!(self, xcb_glx_make_current)
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
    pub unsafe fn xcb_glx_make_current_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_glx_drawable_t,
        context: xcb_glx_context_t,
        old_context_tag: xcb_glx_context_tag_t,
    ) -> xcb_glx_make_current_cookie_t {
        sym!(self, xcb_glx_make_current_unchecked)(c, drawable, context, old_context_tag)
    }

    /// Returns `true` iff the symbol `xcb_glx_make_current_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_make_current_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_make_current_unchecked)
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
     * xcb_glx_make_current_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_make_current_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_make_current_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_make_current_reply_t {
        sym!(self, xcb_glx_make_current_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_make_current_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_make_current_reply(&self) -> bool {
        has_sym!(self, xcb_glx_make_current_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_is_direct(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_glx_context_t,
    ) -> xcb_glx_is_direct_cookie_t {
        sym!(self, xcb_glx_is_direct)(c, context)
    }

    /// Returns `true` iff the symbol `xcb_glx_is_direct` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_is_direct(&self) -> bool {
        has_sym!(self, xcb_glx_is_direct)
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
    pub unsafe fn xcb_glx_is_direct_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_glx_context_t,
    ) -> xcb_glx_is_direct_cookie_t {
        sym!(self, xcb_glx_is_direct_unchecked)(c, context)
    }

    /// Returns `true` iff the symbol `xcb_glx_is_direct_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_is_direct_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_is_direct_unchecked)
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
     * xcb_glx_is_direct_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_is_direct_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_is_direct_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_is_direct_reply_t {
        sym!(self, xcb_glx_is_direct_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_is_direct_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_is_direct_reply(&self) -> bool {
        has_sym!(self, xcb_glx_is_direct_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_query_version(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
    ) -> xcb_glx_query_version_cookie_t {
        sym!(self, xcb_glx_query_version)(c, major_version, minor_version)
    }

    /// Returns `true` iff the symbol `xcb_glx_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_query_version(&self) -> bool {
        has_sym!(self, xcb_glx_query_version)
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
    pub unsafe fn xcb_glx_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
    ) -> xcb_glx_query_version_cookie_t {
        sym!(self, xcb_glx_query_version_unchecked)(c, major_version, minor_version)
    }

    /// Returns `true` iff the symbol `xcb_glx_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_query_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_query_version_unchecked)
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
     * xcb_glx_query_version_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_query_version_reply_t {
        sym!(self, xcb_glx_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_glx_query_version_reply)
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
    pub unsafe fn xcb_glx_wait_gl_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_wait_gl_checked)(c, context_tag)
    }

    /// Returns `true` iff the symbol `xcb_glx_wait_gl_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_wait_gl_checked(&self) -> bool {
        has_sym!(self, xcb_glx_wait_gl_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_wait_gl(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_wait_gl)(c, context_tag)
    }

    /// Returns `true` iff the symbol `xcb_glx_wait_gl` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_wait_gl(&self) -> bool {
        has_sym!(self, xcb_glx_wait_gl)
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
    pub unsafe fn xcb_glx_wait_x_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_wait_x_checked)(c, context_tag)
    }

    /// Returns `true` iff the symbol `xcb_glx_wait_x_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_wait_x_checked(&self) -> bool {
        has_sym!(self, xcb_glx_wait_x_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_wait_x(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_wait_x)(c, context_tag)
    }

    /// Returns `true` iff the symbol `xcb_glx_wait_x` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_wait_x(&self) -> bool {
        has_sym!(self, xcb_glx_wait_x)
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

    /// Returns `true` iff the symbol `xcb_glx_copy_context_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_copy_context_checked(&self) -> bool {
        has_sym!(self, xcb_glx_copy_context_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_glx_copy_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_copy_context(&self) -> bool {
        has_sym!(self, xcb_glx_copy_context)
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
    pub unsafe fn xcb_glx_swap_buffers_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        drawable: xcb_glx_drawable_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_swap_buffers_checked)(c, context_tag, drawable)
    }

    /// Returns `true` iff the symbol `xcb_glx_swap_buffers_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_swap_buffers_checked(&self) -> bool {
        has_sym!(self, xcb_glx_swap_buffers_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_swap_buffers(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        drawable: xcb_glx_drawable_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_swap_buffers)(c, context_tag, drawable)
    }

    /// Returns `true` iff the symbol `xcb_glx_swap_buffers` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_swap_buffers(&self) -> bool {
        has_sym!(self, xcb_glx_swap_buffers)
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

    /// Returns `true` iff the symbol `xcb_glx_use_x_font_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_use_x_font_checked(&self) -> bool {
        has_sym!(self, xcb_glx_use_x_font_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_glx_use_x_font` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_use_x_font(&self) -> bool {
        has_sym!(self, xcb_glx_use_x_font)
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

    /// Returns `true` iff the symbol `xcb_glx_create_glx_pixmap_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_glx_pixmap_checked(&self) -> bool {
        has_sym!(self, xcb_glx_create_glx_pixmap_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_glx_create_glx_pixmap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_glx_pixmap(&self) -> bool {
        has_sym!(self, xcb_glx_create_glx_pixmap)
    }

    pub unsafe fn xcb_glx_get_visual_configs_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_visual_configs_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_visual_configs_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_visual_configs_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_visual_configs_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_visual_configs(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_glx_get_visual_configs_cookie_t {
        sym!(self, xcb_glx_get_visual_configs)(c, screen)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_visual_configs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_visual_configs(&self) -> bool {
        has_sym!(self, xcb_glx_get_visual_configs)
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
    pub unsafe fn xcb_glx_get_visual_configs_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_glx_get_visual_configs_cookie_t {
        sym!(self, xcb_glx_get_visual_configs_unchecked)(c, screen)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_visual_configs_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_visual_configs_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_visual_configs_unchecked)
    }

    pub unsafe fn xcb_glx_get_visual_configs_property_list(
        &self,
        r: *const xcb_glx_get_visual_configs_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_get_visual_configs_property_list)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_visual_configs_property_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_visual_configs_property_list(&self) -> bool {
        has_sym!(self, xcb_glx_get_visual_configs_property_list)
    }

    pub unsafe fn xcb_glx_get_visual_configs_property_list_length(
        &self,
        r: *const xcb_glx_get_visual_configs_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_visual_configs_property_list_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_visual_configs_property_list_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_visual_configs_property_list_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_visual_configs_property_list_length)
    }

    pub unsafe fn xcb_glx_get_visual_configs_property_list_end(
        &self,
        r: *const xcb_glx_get_visual_configs_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_visual_configs_property_list_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_visual_configs_property_list_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_visual_configs_property_list_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_visual_configs_property_list_end)
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
     * xcb_glx_get_visual_configs_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_visual_configs_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_visual_configs_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_visual_configs_reply_t {
        sym!(self, xcb_glx_get_visual_configs_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_visual_configs_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_visual_configs_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_visual_configs_reply)
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
    pub unsafe fn xcb_glx_destroy_glx_pixmap_checked(
        &self,
        c: *mut xcb_connection_t,
        glx_pixmap: xcb_glx_pixmap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_destroy_glx_pixmap_checked)(c, glx_pixmap)
    }

    /// Returns `true` iff the symbol `xcb_glx_destroy_glx_pixmap_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_destroy_glx_pixmap_checked(&self) -> bool {
        has_sym!(self, xcb_glx_destroy_glx_pixmap_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_destroy_glx_pixmap(
        &self,
        c: *mut xcb_connection_t,
        glx_pixmap: xcb_glx_pixmap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_destroy_glx_pixmap)(c, glx_pixmap)
    }

    /// Returns `true` iff the symbol `xcb_glx_destroy_glx_pixmap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_destroy_glx_pixmap(&self) -> bool {
        has_sym!(self, xcb_glx_destroy_glx_pixmap)
    }

    pub unsafe fn xcb_glx_vendor_private_sizeof(
        &self,
        _buffer: *const c_void,
        data_len: u32,
    ) -> c_int {
        sym!(self, xcb_glx_vendor_private_sizeof)(_buffer, data_len)
    }

    /// Returns `true` iff the symbol `xcb_glx_vendor_private_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_vendor_private_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_vendor_private_sizeof)
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

    /// Returns `true` iff the symbol `xcb_glx_vendor_private_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_vendor_private_checked(&self) -> bool {
        has_sym!(self, xcb_glx_vendor_private_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_glx_vendor_private` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_vendor_private(&self) -> bool {
        has_sym!(self, xcb_glx_vendor_private)
    }

    pub unsafe fn xcb_glx_vendor_private_data(
        &self,
        r: *const xcb_glx_vendor_private_request_t,
    ) -> *mut u8 {
        sym!(self, xcb_glx_vendor_private_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_vendor_private_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_vendor_private_data(&self) -> bool {
        has_sym!(self, xcb_glx_vendor_private_data)
    }

    pub unsafe fn xcb_glx_vendor_private_data_length(
        &self,
        r: *const xcb_glx_vendor_private_request_t,
    ) -> c_int {
        sym!(self, xcb_glx_vendor_private_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_vendor_private_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_vendor_private_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_vendor_private_data_length)
    }

    pub unsafe fn xcb_glx_vendor_private_data_end(
        &self,
        r: *const xcb_glx_vendor_private_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_vendor_private_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_vendor_private_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_vendor_private_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_vendor_private_data_end)
    }

    pub unsafe fn xcb_glx_vendor_private_with_reply_sizeof(
        &self,
        _buffer: *const c_void,
        data_len: u32,
    ) -> c_int {
        sym!(self, xcb_glx_vendor_private_with_reply_sizeof)(_buffer, data_len)
    }

    /// Returns `true` iff the symbol `xcb_glx_vendor_private_with_reply_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_vendor_private_with_reply_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_vendor_private_with_reply_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_glx_vendor_private_with_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_vendor_private_with_reply(&self) -> bool {
        has_sym!(self, xcb_glx_vendor_private_with_reply)
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

    /// Returns `true` iff the symbol `xcb_glx_vendor_private_with_reply_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_vendor_private_with_reply_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_vendor_private_with_reply_unchecked)
    }

    pub unsafe fn xcb_glx_vendor_private_with_reply_data_2(
        &self,
        r: *const xcb_glx_vendor_private_with_reply_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_glx_vendor_private_with_reply_data_2)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_vendor_private_with_reply_data_2` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_vendor_private_with_reply_data_2(&self) -> bool {
        has_sym!(self, xcb_glx_vendor_private_with_reply_data_2)
    }

    pub unsafe fn xcb_glx_vendor_private_with_reply_data_2_length(
        &self,
        r: *const xcb_glx_vendor_private_with_reply_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_vendor_private_with_reply_data_2_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_vendor_private_with_reply_data_2_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_vendor_private_with_reply_data_2_length(&self) -> bool {
        has_sym!(self, xcb_glx_vendor_private_with_reply_data_2_length)
    }

    pub unsafe fn xcb_glx_vendor_private_with_reply_data_2_end(
        &self,
        r: *const xcb_glx_vendor_private_with_reply_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_vendor_private_with_reply_data_2_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_vendor_private_with_reply_data_2_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_vendor_private_with_reply_data_2_end(&self) -> bool {
        has_sym!(self, xcb_glx_vendor_private_with_reply_data_2_end)
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
     * xcb_glx_vendor_private_with_reply_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_vendor_private_with_reply_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_vendor_private_with_reply_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_vendor_private_with_reply_reply_t {
        sym!(self, xcb_glx_vendor_private_with_reply_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_vendor_private_with_reply_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_vendor_private_with_reply_reply(&self) -> bool {
        has_sym!(self, xcb_glx_vendor_private_with_reply_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_query_extensions_string(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_glx_query_extensions_string_cookie_t {
        sym!(self, xcb_glx_query_extensions_string)(c, screen)
    }

    /// Returns `true` iff the symbol `xcb_glx_query_extensions_string` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_query_extensions_string(&self) -> bool {
        has_sym!(self, xcb_glx_query_extensions_string)
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
    pub unsafe fn xcb_glx_query_extensions_string_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_glx_query_extensions_string_cookie_t {
        sym!(self, xcb_glx_query_extensions_string_unchecked)(c, screen)
    }

    /// Returns `true` iff the symbol `xcb_glx_query_extensions_string_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_query_extensions_string_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_query_extensions_string_unchecked)
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
     * xcb_glx_query_extensions_string_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_query_extensions_string_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_query_extensions_string_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_query_extensions_string_reply_t {
        sym!(self, xcb_glx_query_extensions_string_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_query_extensions_string_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_query_extensions_string_reply(&self) -> bool {
        has_sym!(self, xcb_glx_query_extensions_string_reply)
    }

    pub unsafe fn xcb_glx_query_server_string_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_query_server_string_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_query_server_string_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_query_server_string_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_query_server_string_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_query_server_string(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        name: u32,
    ) -> xcb_glx_query_server_string_cookie_t {
        sym!(self, xcb_glx_query_server_string)(c, screen, name)
    }

    /// Returns `true` iff the symbol `xcb_glx_query_server_string` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_query_server_string(&self) -> bool {
        has_sym!(self, xcb_glx_query_server_string)
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
    pub unsafe fn xcb_glx_query_server_string_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        name: u32,
    ) -> xcb_glx_query_server_string_cookie_t {
        sym!(self, xcb_glx_query_server_string_unchecked)(c, screen, name)
    }

    /// Returns `true` iff the symbol `xcb_glx_query_server_string_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_query_server_string_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_query_server_string_unchecked)
    }

    pub unsafe fn xcb_glx_query_server_string_string(
        &self,
        r: *const xcb_glx_query_server_string_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_glx_query_server_string_string)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_query_server_string_string` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_query_server_string_string(&self) -> bool {
        has_sym!(self, xcb_glx_query_server_string_string)
    }

    pub unsafe fn xcb_glx_query_server_string_string_length(
        &self,
        r: *const xcb_glx_query_server_string_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_query_server_string_string_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_query_server_string_string_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_query_server_string_string_length(&self) -> bool {
        has_sym!(self, xcb_glx_query_server_string_string_length)
    }

    pub unsafe fn xcb_glx_query_server_string_string_end(
        &self,
        r: *const xcb_glx_query_server_string_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_query_server_string_string_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_query_server_string_string_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_query_server_string_string_end(&self) -> bool {
        has_sym!(self, xcb_glx_query_server_string_string_end)
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
     * xcb_glx_query_server_string_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_query_server_string_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_query_server_string_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_query_server_string_reply_t {
        sym!(self, xcb_glx_query_server_string_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_query_server_string_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_query_server_string_reply(&self) -> bool {
        has_sym!(self, xcb_glx_query_server_string_reply)
    }

    pub unsafe fn xcb_glx_client_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_client_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_client_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_client_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_client_info_sizeof)
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

    /// Returns `true` iff the symbol `xcb_glx_client_info_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_client_info_checked(&self) -> bool {
        has_sym!(self, xcb_glx_client_info_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_glx_client_info` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_client_info(&self) -> bool {
        has_sym!(self, xcb_glx_client_info)
    }

    pub unsafe fn xcb_glx_client_info_string(
        &self,
        r: *const xcb_glx_client_info_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_glx_client_info_string)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_client_info_string` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_client_info_string(&self) -> bool {
        has_sym!(self, xcb_glx_client_info_string)
    }

    pub unsafe fn xcb_glx_client_info_string_length(
        &self,
        r: *const xcb_glx_client_info_request_t,
    ) -> c_int {
        sym!(self, xcb_glx_client_info_string_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_client_info_string_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_client_info_string_length(&self) -> bool {
        has_sym!(self, xcb_glx_client_info_string_length)
    }

    pub unsafe fn xcb_glx_client_info_string_end(
        &self,
        r: *const xcb_glx_client_info_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_client_info_string_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_client_info_string_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_client_info_string_end(&self) -> bool {
        has_sym!(self, xcb_glx_client_info_string_end)
    }

    pub unsafe fn xcb_glx_get_fb_configs_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_fb_configs_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_fb_configs_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_fb_configs_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_fb_configs_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_fb_configs(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_glx_get_fb_configs_cookie_t {
        sym!(self, xcb_glx_get_fb_configs)(c, screen)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_fb_configs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_fb_configs(&self) -> bool {
        has_sym!(self, xcb_glx_get_fb_configs)
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
    pub unsafe fn xcb_glx_get_fb_configs_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_glx_get_fb_configs_cookie_t {
        sym!(self, xcb_glx_get_fb_configs_unchecked)(c, screen)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_fb_configs_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_fb_configs_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_fb_configs_unchecked)
    }

    pub unsafe fn xcb_glx_get_fb_configs_property_list(
        &self,
        r: *const xcb_glx_get_fb_configs_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_get_fb_configs_property_list)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_fb_configs_property_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_fb_configs_property_list(&self) -> bool {
        has_sym!(self, xcb_glx_get_fb_configs_property_list)
    }

    pub unsafe fn xcb_glx_get_fb_configs_property_list_length(
        &self,
        r: *const xcb_glx_get_fb_configs_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_fb_configs_property_list_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_fb_configs_property_list_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_fb_configs_property_list_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_fb_configs_property_list_length)
    }

    pub unsafe fn xcb_glx_get_fb_configs_property_list_end(
        &self,
        r: *const xcb_glx_get_fb_configs_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_fb_configs_property_list_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_fb_configs_property_list_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_fb_configs_property_list_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_fb_configs_property_list_end)
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
     * xcb_glx_get_fb_configs_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_fb_configs_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_fb_configs_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_fb_configs_reply_t {
        sym!(self, xcb_glx_get_fb_configs_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_fb_configs_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_fb_configs_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_fb_configs_reply)
    }

    pub unsafe fn xcb_glx_create_pixmap_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_create_pixmap_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_create_pixmap_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_pixmap_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_create_pixmap_sizeof)
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

    /// Returns `true` iff the symbol `xcb_glx_create_pixmap_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_pixmap_checked(&self) -> bool {
        has_sym!(self, xcb_glx_create_pixmap_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_glx_create_pixmap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_pixmap(&self) -> bool {
        has_sym!(self, xcb_glx_create_pixmap)
    }

    pub unsafe fn xcb_glx_create_pixmap_attribs(
        &self,
        r: *const xcb_glx_create_pixmap_request_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_create_pixmap_attribs)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_create_pixmap_attribs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_pixmap_attribs(&self) -> bool {
        has_sym!(self, xcb_glx_create_pixmap_attribs)
    }

    pub unsafe fn xcb_glx_create_pixmap_attribs_length(
        &self,
        r: *const xcb_glx_create_pixmap_request_t,
    ) -> c_int {
        sym!(self, xcb_glx_create_pixmap_attribs_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_create_pixmap_attribs_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_pixmap_attribs_length(&self) -> bool {
        has_sym!(self, xcb_glx_create_pixmap_attribs_length)
    }

    pub unsafe fn xcb_glx_create_pixmap_attribs_end(
        &self,
        r: *const xcb_glx_create_pixmap_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_create_pixmap_attribs_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_create_pixmap_attribs_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_pixmap_attribs_end(&self) -> bool {
        has_sym!(self, xcb_glx_create_pixmap_attribs_end)
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
    pub unsafe fn xcb_glx_destroy_pixmap_checked(
        &self,
        c: *mut xcb_connection_t,
        glx_pixmap: xcb_glx_pixmap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_destroy_pixmap_checked)(c, glx_pixmap)
    }

    /// Returns `true` iff the symbol `xcb_glx_destroy_pixmap_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_destroy_pixmap_checked(&self) -> bool {
        has_sym!(self, xcb_glx_destroy_pixmap_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_destroy_pixmap(
        &self,
        c: *mut xcb_connection_t,
        glx_pixmap: xcb_glx_pixmap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_destroy_pixmap)(c, glx_pixmap)
    }

    /// Returns `true` iff the symbol `xcb_glx_destroy_pixmap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_destroy_pixmap(&self) -> bool {
        has_sym!(self, xcb_glx_destroy_pixmap)
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

    /// Returns `true` iff the symbol `xcb_glx_create_new_context_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_new_context_checked(&self) -> bool {
        has_sym!(self, xcb_glx_create_new_context_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_glx_create_new_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_new_context(&self) -> bool {
        has_sym!(self, xcb_glx_create_new_context)
    }

    pub unsafe fn xcb_glx_query_context_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_query_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_query_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_query_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_query_context_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_query_context(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_glx_context_t,
    ) -> xcb_glx_query_context_cookie_t {
        sym!(self, xcb_glx_query_context)(c, context)
    }

    /// Returns `true` iff the symbol `xcb_glx_query_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_query_context(&self) -> bool {
        has_sym!(self, xcb_glx_query_context)
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
    pub unsafe fn xcb_glx_query_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_glx_context_t,
    ) -> xcb_glx_query_context_cookie_t {
        sym!(self, xcb_glx_query_context_unchecked)(c, context)
    }

    /// Returns `true` iff the symbol `xcb_glx_query_context_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_query_context_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_query_context_unchecked)
    }

    pub unsafe fn xcb_glx_query_context_attribs(
        &self,
        r: *const xcb_glx_query_context_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_query_context_attribs)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_query_context_attribs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_query_context_attribs(&self) -> bool {
        has_sym!(self, xcb_glx_query_context_attribs)
    }

    pub unsafe fn xcb_glx_query_context_attribs_length(
        &self,
        r: *const xcb_glx_query_context_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_query_context_attribs_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_query_context_attribs_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_query_context_attribs_length(&self) -> bool {
        has_sym!(self, xcb_glx_query_context_attribs_length)
    }

    pub unsafe fn xcb_glx_query_context_attribs_end(
        &self,
        r: *const xcb_glx_query_context_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_query_context_attribs_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_query_context_attribs_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_query_context_attribs_end(&self) -> bool {
        has_sym!(self, xcb_glx_query_context_attribs_end)
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
     * xcb_glx_query_context_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_query_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_query_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_query_context_reply_t {
        sym!(self, xcb_glx_query_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_query_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_query_context_reply(&self) -> bool {
        has_sym!(self, xcb_glx_query_context_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_glx_make_context_current` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_make_context_current(&self) -> bool {
        has_sym!(self, xcb_glx_make_context_current)
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

    /// Returns `true` iff the symbol `xcb_glx_make_context_current_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_make_context_current_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_make_context_current_unchecked)
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
     * xcb_glx_make_context_current_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_make_context_current_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_make_context_current_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_make_context_current_reply_t {
        sym!(self, xcb_glx_make_context_current_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_make_context_current_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_make_context_current_reply(&self) -> bool {
        has_sym!(self, xcb_glx_make_context_current_reply)
    }

    pub unsafe fn xcb_glx_create_pbuffer_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_create_pbuffer_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_create_pbuffer_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_pbuffer_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_create_pbuffer_sizeof)
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

    /// Returns `true` iff the symbol `xcb_glx_create_pbuffer_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_pbuffer_checked(&self) -> bool {
        has_sym!(self, xcb_glx_create_pbuffer_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_glx_create_pbuffer` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_pbuffer(&self) -> bool {
        has_sym!(self, xcb_glx_create_pbuffer)
    }

    pub unsafe fn xcb_glx_create_pbuffer_attribs(
        &self,
        r: *const xcb_glx_create_pbuffer_request_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_create_pbuffer_attribs)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_create_pbuffer_attribs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_pbuffer_attribs(&self) -> bool {
        has_sym!(self, xcb_glx_create_pbuffer_attribs)
    }

    pub unsafe fn xcb_glx_create_pbuffer_attribs_length(
        &self,
        r: *const xcb_glx_create_pbuffer_request_t,
    ) -> c_int {
        sym!(self, xcb_glx_create_pbuffer_attribs_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_create_pbuffer_attribs_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_pbuffer_attribs_length(&self) -> bool {
        has_sym!(self, xcb_glx_create_pbuffer_attribs_length)
    }

    pub unsafe fn xcb_glx_create_pbuffer_attribs_end(
        &self,
        r: *const xcb_glx_create_pbuffer_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_create_pbuffer_attribs_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_create_pbuffer_attribs_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_pbuffer_attribs_end(&self) -> bool {
        has_sym!(self, xcb_glx_create_pbuffer_attribs_end)
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
    pub unsafe fn xcb_glx_destroy_pbuffer_checked(
        &self,
        c: *mut xcb_connection_t,
        pbuffer: xcb_glx_pbuffer_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_destroy_pbuffer_checked)(c, pbuffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_destroy_pbuffer_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_destroy_pbuffer_checked(&self) -> bool {
        has_sym!(self, xcb_glx_destroy_pbuffer_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_destroy_pbuffer(
        &self,
        c: *mut xcb_connection_t,
        pbuffer: xcb_glx_pbuffer_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_destroy_pbuffer)(c, pbuffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_destroy_pbuffer` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_destroy_pbuffer(&self) -> bool {
        has_sym!(self, xcb_glx_destroy_pbuffer)
    }

    pub unsafe fn xcb_glx_get_drawable_attributes_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_drawable_attributes_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_drawable_attributes_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_drawable_attributes_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_drawable_attributes_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_drawable_attributes(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_glx_drawable_t,
    ) -> xcb_glx_get_drawable_attributes_cookie_t {
        sym!(self, xcb_glx_get_drawable_attributes)(c, drawable)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_drawable_attributes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_drawable_attributes(&self) -> bool {
        has_sym!(self, xcb_glx_get_drawable_attributes)
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
    pub unsafe fn xcb_glx_get_drawable_attributes_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_glx_drawable_t,
    ) -> xcb_glx_get_drawable_attributes_cookie_t {
        sym!(self, xcb_glx_get_drawable_attributes_unchecked)(c, drawable)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_drawable_attributes_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_drawable_attributes_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_drawable_attributes_unchecked)
    }

    pub unsafe fn xcb_glx_get_drawable_attributes_attribs(
        &self,
        r: *const xcb_glx_get_drawable_attributes_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_get_drawable_attributes_attribs)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_drawable_attributes_attribs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_drawable_attributes_attribs(&self) -> bool {
        has_sym!(self, xcb_glx_get_drawable_attributes_attribs)
    }

    pub unsafe fn xcb_glx_get_drawable_attributes_attribs_length(
        &self,
        r: *const xcb_glx_get_drawable_attributes_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_drawable_attributes_attribs_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_drawable_attributes_attribs_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_drawable_attributes_attribs_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_drawable_attributes_attribs_length)
    }

    pub unsafe fn xcb_glx_get_drawable_attributes_attribs_end(
        &self,
        r: *const xcb_glx_get_drawable_attributes_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_drawable_attributes_attribs_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_drawable_attributes_attribs_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_drawable_attributes_attribs_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_drawable_attributes_attribs_end)
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
     * xcb_glx_get_drawable_attributes_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_drawable_attributes_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_drawable_attributes_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_drawable_attributes_reply_t {
        sym!(self, xcb_glx_get_drawable_attributes_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_drawable_attributes_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_drawable_attributes_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_drawable_attributes_reply)
    }

    pub unsafe fn xcb_glx_change_drawable_attributes_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_glx_change_drawable_attributes_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_change_drawable_attributes_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_change_drawable_attributes_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_change_drawable_attributes_sizeof)
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
    pub unsafe fn xcb_glx_change_drawable_attributes_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_glx_drawable_t,
        num_attribs: u32,
        attribs: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_change_drawable_attributes_checked)(c, drawable, num_attribs, attribs)
    }

    /// Returns `true` iff the symbol `xcb_glx_change_drawable_attributes_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_change_drawable_attributes_checked(&self) -> bool {
        has_sym!(self, xcb_glx_change_drawable_attributes_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_change_drawable_attributes(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_glx_drawable_t,
        num_attribs: u32,
        attribs: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_change_drawable_attributes)(c, drawable, num_attribs, attribs)
    }

    /// Returns `true` iff the symbol `xcb_glx_change_drawable_attributes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_change_drawable_attributes(&self) -> bool {
        has_sym!(self, xcb_glx_change_drawable_attributes)
    }

    pub unsafe fn xcb_glx_change_drawable_attributes_attribs(
        &self,
        r: *const xcb_glx_change_drawable_attributes_request_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_change_drawable_attributes_attribs)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_change_drawable_attributes_attribs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_change_drawable_attributes_attribs(&self) -> bool {
        has_sym!(self, xcb_glx_change_drawable_attributes_attribs)
    }

    pub unsafe fn xcb_glx_change_drawable_attributes_attribs_length(
        &self,
        r: *const xcb_glx_change_drawable_attributes_request_t,
    ) -> c_int {
        sym!(self, xcb_glx_change_drawable_attributes_attribs_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_change_drawable_attributes_attribs_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_change_drawable_attributes_attribs_length(&self) -> bool {
        has_sym!(self, xcb_glx_change_drawable_attributes_attribs_length)
    }

    pub unsafe fn xcb_glx_change_drawable_attributes_attribs_end(
        &self,
        r: *const xcb_glx_change_drawable_attributes_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_change_drawable_attributes_attribs_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_change_drawable_attributes_attribs_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_change_drawable_attributes_attribs_end(&self) -> bool {
        has_sym!(self, xcb_glx_change_drawable_attributes_attribs_end)
    }

    pub unsafe fn xcb_glx_create_window_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_create_window_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_create_window_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_window_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_create_window_sizeof)
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

    /// Returns `true` iff the symbol `xcb_glx_create_window_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_window_checked(&self) -> bool {
        has_sym!(self, xcb_glx_create_window_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_glx_create_window` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_window(&self) -> bool {
        has_sym!(self, xcb_glx_create_window)
    }

    pub unsafe fn xcb_glx_create_window_attribs(
        &self,
        r: *const xcb_glx_create_window_request_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_create_window_attribs)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_create_window_attribs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_window_attribs(&self) -> bool {
        has_sym!(self, xcb_glx_create_window_attribs)
    }

    pub unsafe fn xcb_glx_create_window_attribs_length(
        &self,
        r: *const xcb_glx_create_window_request_t,
    ) -> c_int {
        sym!(self, xcb_glx_create_window_attribs_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_create_window_attribs_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_window_attribs_length(&self) -> bool {
        has_sym!(self, xcb_glx_create_window_attribs_length)
    }

    pub unsafe fn xcb_glx_create_window_attribs_end(
        &self,
        r: *const xcb_glx_create_window_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_create_window_attribs_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_create_window_attribs_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_window_attribs_end(&self) -> bool {
        has_sym!(self, xcb_glx_create_window_attribs_end)
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
    pub unsafe fn xcb_glx_delete_window_checked(
        &self,
        c: *mut xcb_connection_t,
        glxwindow: xcb_glx_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_delete_window_checked)(c, glxwindow)
    }

    /// Returns `true` iff the symbol `xcb_glx_delete_window_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_delete_window_checked(&self) -> bool {
        has_sym!(self, xcb_glx_delete_window_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_delete_window(
        &self,
        c: *mut xcb_connection_t,
        glxwindow: xcb_glx_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_delete_window)(c, glxwindow)
    }

    /// Returns `true` iff the symbol `xcb_glx_delete_window` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_delete_window(&self) -> bool {
        has_sym!(self, xcb_glx_delete_window)
    }

    pub unsafe fn xcb_glx_set_client_info_arb_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_set_client_info_arb_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_set_client_info_arb_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_set_client_info_arb_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_set_client_info_arb_sizeof)
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

    /// Returns `true` iff the symbol `xcb_glx_set_client_info_arb_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_set_client_info_arb_checked(&self) -> bool {
        has_sym!(self, xcb_glx_set_client_info_arb_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_glx_set_client_info_arb` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_set_client_info_arb(&self) -> bool {
        has_sym!(self, xcb_glx_set_client_info_arb)
    }

    pub unsafe fn xcb_glx_set_client_info_arb_gl_versions(
        &self,
        r: *const xcb_glx_set_client_info_arb_request_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_set_client_info_arb_gl_versions)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_set_client_info_arb_gl_versions` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_set_client_info_arb_gl_versions(&self) -> bool {
        has_sym!(self, xcb_glx_set_client_info_arb_gl_versions)
    }

    pub unsafe fn xcb_glx_set_client_info_arb_gl_versions_length(
        &self,
        r: *const xcb_glx_set_client_info_arb_request_t,
    ) -> c_int {
        sym!(self, xcb_glx_set_client_info_arb_gl_versions_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_set_client_info_arb_gl_versions_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_set_client_info_arb_gl_versions_length(&self) -> bool {
        has_sym!(self, xcb_glx_set_client_info_arb_gl_versions_length)
    }

    pub unsafe fn xcb_glx_set_client_info_arb_gl_versions_end(
        &self,
        r: *const xcb_glx_set_client_info_arb_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_set_client_info_arb_gl_versions_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_set_client_info_arb_gl_versions_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_set_client_info_arb_gl_versions_end(&self) -> bool {
        has_sym!(self, xcb_glx_set_client_info_arb_gl_versions_end)
    }

    pub unsafe fn xcb_glx_set_client_info_arb_gl_extension_string(
        &self,
        r: *const xcb_glx_set_client_info_arb_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_glx_set_client_info_arb_gl_extension_string)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_set_client_info_arb_gl_extension_string` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_set_client_info_arb_gl_extension_string(&self) -> bool {
        has_sym!(self, xcb_glx_set_client_info_arb_gl_extension_string)
    }

    pub unsafe fn xcb_glx_set_client_info_arb_gl_extension_string_length(
        &self,
        r: *const xcb_glx_set_client_info_arb_request_t,
    ) -> c_int {
        sym!(self, xcb_glx_set_client_info_arb_gl_extension_string_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_set_client_info_arb_gl_extension_string_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_set_client_info_arb_gl_extension_string_length(&self) -> bool {
        has_sym!(self, xcb_glx_set_client_info_arb_gl_extension_string_length)
    }

    pub unsafe fn xcb_glx_set_client_info_arb_gl_extension_string_end(
        &self,
        r: *const xcb_glx_set_client_info_arb_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_set_client_info_arb_gl_extension_string_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_set_client_info_arb_gl_extension_string_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_set_client_info_arb_gl_extension_string_end(&self) -> bool {
        has_sym!(self, xcb_glx_set_client_info_arb_gl_extension_string_end)
    }

    pub unsafe fn xcb_glx_set_client_info_arb_glx_extension_string(
        &self,
        r: *const xcb_glx_set_client_info_arb_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_glx_set_client_info_arb_glx_extension_string)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_set_client_info_arb_glx_extension_string` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_set_client_info_arb_glx_extension_string(&self) -> bool {
        has_sym!(self, xcb_glx_set_client_info_arb_glx_extension_string)
    }

    pub unsafe fn xcb_glx_set_client_info_arb_glx_extension_string_length(
        &self,
        r: *const xcb_glx_set_client_info_arb_request_t,
    ) -> c_int {
        sym!(
            self,
            xcb_glx_set_client_info_arb_glx_extension_string_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_set_client_info_arb_glx_extension_string_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_set_client_info_arb_glx_extension_string_length(&self) -> bool {
        has_sym!(
            self,
            xcb_glx_set_client_info_arb_glx_extension_string_length
        )
    }

    pub unsafe fn xcb_glx_set_client_info_arb_glx_extension_string_end(
        &self,
        r: *const xcb_glx_set_client_info_arb_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_set_client_info_arb_glx_extension_string_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_set_client_info_arb_glx_extension_string_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_set_client_info_arb_glx_extension_string_end(&self) -> bool {
        has_sym!(self, xcb_glx_set_client_info_arb_glx_extension_string_end)
    }

    pub unsafe fn xcb_glx_create_context_attribs_arb_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_glx_create_context_attribs_arb_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_create_context_attribs_arb_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_context_attribs_arb_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_create_context_attribs_arb_sizeof)
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

    /// Returns `true` iff the symbol `xcb_glx_create_context_attribs_arb_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_context_attribs_arb_checked(&self) -> bool {
        has_sym!(self, xcb_glx_create_context_attribs_arb_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_glx_create_context_attribs_arb` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_context_attribs_arb(&self) -> bool {
        has_sym!(self, xcb_glx_create_context_attribs_arb)
    }

    pub unsafe fn xcb_glx_create_context_attribs_arb_attribs(
        &self,
        r: *const xcb_glx_create_context_attribs_arb_request_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_create_context_attribs_arb_attribs)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_create_context_attribs_arb_attribs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_context_attribs_arb_attribs(&self) -> bool {
        has_sym!(self, xcb_glx_create_context_attribs_arb_attribs)
    }

    pub unsafe fn xcb_glx_create_context_attribs_arb_attribs_length(
        &self,
        r: *const xcb_glx_create_context_attribs_arb_request_t,
    ) -> c_int {
        sym!(self, xcb_glx_create_context_attribs_arb_attribs_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_create_context_attribs_arb_attribs_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_context_attribs_arb_attribs_length(&self) -> bool {
        has_sym!(self, xcb_glx_create_context_attribs_arb_attribs_length)
    }

    pub unsafe fn xcb_glx_create_context_attribs_arb_attribs_end(
        &self,
        r: *const xcb_glx_create_context_attribs_arb_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_create_context_attribs_arb_attribs_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_create_context_attribs_arb_attribs_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_create_context_attribs_arb_attribs_end(&self) -> bool {
        has_sym!(self, xcb_glx_create_context_attribs_arb_attribs_end)
    }

    pub unsafe fn xcb_glx_set_client_info_2arb_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_set_client_info_2arb_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_set_client_info_2arb_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_set_client_info_2arb_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_set_client_info_2arb_sizeof)
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

    /// Returns `true` iff the symbol `xcb_glx_set_client_info_2arb_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_set_client_info_2arb_checked(&self) -> bool {
        has_sym!(self, xcb_glx_set_client_info_2arb_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_glx_set_client_info_2arb` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_set_client_info_2arb(&self) -> bool {
        has_sym!(self, xcb_glx_set_client_info_2arb)
    }

    pub unsafe fn xcb_glx_set_client_info_2arb_gl_versions(
        &self,
        r: *const xcb_glx_set_client_info_2arb_request_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_set_client_info_2arb_gl_versions)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_set_client_info_2arb_gl_versions` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_set_client_info_2arb_gl_versions(&self) -> bool {
        has_sym!(self, xcb_glx_set_client_info_2arb_gl_versions)
    }

    pub unsafe fn xcb_glx_set_client_info_2arb_gl_versions_length(
        &self,
        r: *const xcb_glx_set_client_info_2arb_request_t,
    ) -> c_int {
        sym!(self, xcb_glx_set_client_info_2arb_gl_versions_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_set_client_info_2arb_gl_versions_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_set_client_info_2arb_gl_versions_length(&self) -> bool {
        has_sym!(self, xcb_glx_set_client_info_2arb_gl_versions_length)
    }

    pub unsafe fn xcb_glx_set_client_info_2arb_gl_versions_end(
        &self,
        r: *const xcb_glx_set_client_info_2arb_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_set_client_info_2arb_gl_versions_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_set_client_info_2arb_gl_versions_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_set_client_info_2arb_gl_versions_end(&self) -> bool {
        has_sym!(self, xcb_glx_set_client_info_2arb_gl_versions_end)
    }

    pub unsafe fn xcb_glx_set_client_info_2arb_gl_extension_string(
        &self,
        r: *const xcb_glx_set_client_info_2arb_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_glx_set_client_info_2arb_gl_extension_string)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_set_client_info_2arb_gl_extension_string` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_set_client_info_2arb_gl_extension_string(&self) -> bool {
        has_sym!(self, xcb_glx_set_client_info_2arb_gl_extension_string)
    }

    pub unsafe fn xcb_glx_set_client_info_2arb_gl_extension_string_length(
        &self,
        r: *const xcb_glx_set_client_info_2arb_request_t,
    ) -> c_int {
        sym!(
            self,
            xcb_glx_set_client_info_2arb_gl_extension_string_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_set_client_info_2arb_gl_extension_string_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_set_client_info_2arb_gl_extension_string_length(&self) -> bool {
        has_sym!(
            self,
            xcb_glx_set_client_info_2arb_gl_extension_string_length
        )
    }

    pub unsafe fn xcb_glx_set_client_info_2arb_gl_extension_string_end(
        &self,
        r: *const xcb_glx_set_client_info_2arb_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_set_client_info_2arb_gl_extension_string_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_set_client_info_2arb_gl_extension_string_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_set_client_info_2arb_gl_extension_string_end(&self) -> bool {
        has_sym!(self, xcb_glx_set_client_info_2arb_gl_extension_string_end)
    }

    pub unsafe fn xcb_glx_set_client_info_2arb_glx_extension_string(
        &self,
        r: *const xcb_glx_set_client_info_2arb_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_glx_set_client_info_2arb_glx_extension_string)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_set_client_info_2arb_glx_extension_string` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_set_client_info_2arb_glx_extension_string(&self) -> bool {
        has_sym!(self, xcb_glx_set_client_info_2arb_glx_extension_string)
    }

    pub unsafe fn xcb_glx_set_client_info_2arb_glx_extension_string_length(
        &self,
        r: *const xcb_glx_set_client_info_2arb_request_t,
    ) -> c_int {
        sym!(
            self,
            xcb_glx_set_client_info_2arb_glx_extension_string_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_set_client_info_2arb_glx_extension_string_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_set_client_info_2arb_glx_extension_string_length(&self) -> bool {
        has_sym!(
            self,
            xcb_glx_set_client_info_2arb_glx_extension_string_length
        )
    }

    pub unsafe fn xcb_glx_set_client_info_2arb_glx_extension_string_end(
        &self,
        r: *const xcb_glx_set_client_info_2arb_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_set_client_info_2arb_glx_extension_string_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_set_client_info_2arb_glx_extension_string_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_set_client_info_2arb_glx_extension_string_end(&self) -> bool {
        has_sym!(self, xcb_glx_set_client_info_2arb_glx_extension_string_end)
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
    pub unsafe fn xcb_glx_new_list_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        list: u32,
        mode: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_new_list_checked)(c, context_tag, list, mode)
    }

    /// Returns `true` iff the symbol `xcb_glx_new_list_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_new_list_checked(&self) -> bool {
        has_sym!(self, xcb_glx_new_list_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_new_list(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        list: u32,
        mode: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_new_list)(c, context_tag, list, mode)
    }

    /// Returns `true` iff the symbol `xcb_glx_new_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_new_list(&self) -> bool {
        has_sym!(self, xcb_glx_new_list)
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
    pub unsafe fn xcb_glx_end_list_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_end_list_checked)(c, context_tag)
    }

    /// Returns `true` iff the symbol `xcb_glx_end_list_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_end_list_checked(&self) -> bool {
        has_sym!(self, xcb_glx_end_list_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_end_list(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_end_list)(c, context_tag)
    }

    /// Returns `true` iff the symbol `xcb_glx_end_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_end_list(&self) -> bool {
        has_sym!(self, xcb_glx_end_list)
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
    pub unsafe fn xcb_glx_delete_lists_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        list: u32,
        range: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_delete_lists_checked)(c, context_tag, list, range)
    }

    /// Returns `true` iff the symbol `xcb_glx_delete_lists_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_delete_lists_checked(&self) -> bool {
        has_sym!(self, xcb_glx_delete_lists_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_delete_lists(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        list: u32,
        range: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_delete_lists)(c, context_tag, list, range)
    }

    /// Returns `true` iff the symbol `xcb_glx_delete_lists` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_delete_lists(&self) -> bool {
        has_sym!(self, xcb_glx_delete_lists)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_gen_lists(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        range: i32,
    ) -> xcb_glx_gen_lists_cookie_t {
        sym!(self, xcb_glx_gen_lists)(c, context_tag, range)
    }

    /// Returns `true` iff the symbol `xcb_glx_gen_lists` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_gen_lists(&self) -> bool {
        has_sym!(self, xcb_glx_gen_lists)
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
    pub unsafe fn xcb_glx_gen_lists_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        range: i32,
    ) -> xcb_glx_gen_lists_cookie_t {
        sym!(self, xcb_glx_gen_lists_unchecked)(c, context_tag, range)
    }

    /// Returns `true` iff the symbol `xcb_glx_gen_lists_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_gen_lists_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_gen_lists_unchecked)
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
     * xcb_glx_gen_lists_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_gen_lists_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_gen_lists_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_gen_lists_reply_t {
        sym!(self, xcb_glx_gen_lists_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_gen_lists_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_gen_lists_reply(&self) -> bool {
        has_sym!(self, xcb_glx_gen_lists_reply)
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
    pub unsafe fn xcb_glx_feedback_buffer_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        size: i32,
        type_: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_feedback_buffer_checked)(c, context_tag, size, type_)
    }

    /// Returns `true` iff the symbol `xcb_glx_feedback_buffer_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_feedback_buffer_checked(&self) -> bool {
        has_sym!(self, xcb_glx_feedback_buffer_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_feedback_buffer(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        size: i32,
        type_: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_feedback_buffer)(c, context_tag, size, type_)
    }

    /// Returns `true` iff the symbol `xcb_glx_feedback_buffer` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_feedback_buffer(&self) -> bool {
        has_sym!(self, xcb_glx_feedback_buffer)
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
    pub unsafe fn xcb_glx_select_buffer_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        size: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_select_buffer_checked)(c, context_tag, size)
    }

    /// Returns `true` iff the symbol `xcb_glx_select_buffer_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_select_buffer_checked(&self) -> bool {
        has_sym!(self, xcb_glx_select_buffer_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_select_buffer(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        size: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_select_buffer)(c, context_tag, size)
    }

    /// Returns `true` iff the symbol `xcb_glx_select_buffer` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_select_buffer(&self) -> bool {
        has_sym!(self, xcb_glx_select_buffer)
    }

    pub unsafe fn xcb_glx_render_mode_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_render_mode_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_render_mode_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_render_mode_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_render_mode_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_render_mode(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        mode: u32,
    ) -> xcb_glx_render_mode_cookie_t {
        sym!(self, xcb_glx_render_mode)(c, context_tag, mode)
    }

    /// Returns `true` iff the symbol `xcb_glx_render_mode` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_render_mode(&self) -> bool {
        has_sym!(self, xcb_glx_render_mode)
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
    pub unsafe fn xcb_glx_render_mode_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        mode: u32,
    ) -> xcb_glx_render_mode_cookie_t {
        sym!(self, xcb_glx_render_mode_unchecked)(c, context_tag, mode)
    }

    /// Returns `true` iff the symbol `xcb_glx_render_mode_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_render_mode_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_render_mode_unchecked)
    }

    pub unsafe fn xcb_glx_render_mode_data(
        &self,
        r: *const xcb_glx_render_mode_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_render_mode_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_render_mode_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_render_mode_data(&self) -> bool {
        has_sym!(self, xcb_glx_render_mode_data)
    }

    pub unsafe fn xcb_glx_render_mode_data_length(
        &self,
        r: *const xcb_glx_render_mode_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_render_mode_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_render_mode_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_render_mode_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_render_mode_data_length)
    }

    pub unsafe fn xcb_glx_render_mode_data_end(
        &self,
        r: *const xcb_glx_render_mode_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_render_mode_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_render_mode_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_render_mode_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_render_mode_data_end)
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
     * xcb_glx_render_mode_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_render_mode_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_render_mode_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_render_mode_reply_t {
        sym!(self, xcb_glx_render_mode_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_render_mode_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_render_mode_reply(&self) -> bool {
        has_sym!(self, xcb_glx_render_mode_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_finish(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
    ) -> xcb_glx_finish_cookie_t {
        sym!(self, xcb_glx_finish)(c, context_tag)
    }

    /// Returns `true` iff the symbol `xcb_glx_finish` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_finish(&self) -> bool {
        has_sym!(self, xcb_glx_finish)
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
    pub unsafe fn xcb_glx_finish_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
    ) -> xcb_glx_finish_cookie_t {
        sym!(self, xcb_glx_finish_unchecked)(c, context_tag)
    }

    /// Returns `true` iff the symbol `xcb_glx_finish_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_finish_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_finish_unchecked)
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
     * xcb_glx_finish_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_finish_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_finish_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_finish_reply_t {
        sym!(self, xcb_glx_finish_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_finish_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_finish_reply(&self) -> bool {
        has_sym!(self, xcb_glx_finish_reply)
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
    pub unsafe fn xcb_glx_pixel_storef_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        pname: u32,
        datum: xcb_glx_float32_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_pixel_storef_checked)(c, context_tag, pname, datum)
    }

    /// Returns `true` iff the symbol `xcb_glx_pixel_storef_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_pixel_storef_checked(&self) -> bool {
        has_sym!(self, xcb_glx_pixel_storef_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_pixel_storef(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        pname: u32,
        datum: xcb_glx_float32_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_pixel_storef)(c, context_tag, pname, datum)
    }

    /// Returns `true` iff the symbol `xcb_glx_pixel_storef` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_pixel_storef(&self) -> bool {
        has_sym!(self, xcb_glx_pixel_storef)
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
    pub unsafe fn xcb_glx_pixel_storei_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        pname: u32,
        datum: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_pixel_storei_checked)(c, context_tag, pname, datum)
    }

    /// Returns `true` iff the symbol `xcb_glx_pixel_storei_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_pixel_storei_checked(&self) -> bool {
        has_sym!(self, xcb_glx_pixel_storei_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_pixel_storei(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        pname: u32,
        datum: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_pixel_storei)(c, context_tag, pname, datum)
    }

    /// Returns `true` iff the symbol `xcb_glx_pixel_storei` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_pixel_storei(&self) -> bool {
        has_sym!(self, xcb_glx_pixel_storei)
    }

    pub unsafe fn xcb_glx_read_pixels_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_read_pixels_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_read_pixels_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_read_pixels_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_read_pixels_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_glx_read_pixels` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_read_pixels(&self) -> bool {
        has_sym!(self, xcb_glx_read_pixels)
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

    /// Returns `true` iff the symbol `xcb_glx_read_pixels_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_read_pixels_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_read_pixels_unchecked)
    }

    pub unsafe fn xcb_glx_read_pixels_data(
        &self,
        r: *const xcb_glx_read_pixels_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_glx_read_pixels_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_read_pixels_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_read_pixels_data(&self) -> bool {
        has_sym!(self, xcb_glx_read_pixels_data)
    }

    pub unsafe fn xcb_glx_read_pixels_data_length(
        &self,
        r: *const xcb_glx_read_pixels_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_read_pixels_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_read_pixels_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_read_pixels_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_read_pixels_data_length)
    }

    pub unsafe fn xcb_glx_read_pixels_data_end(
        &self,
        r: *const xcb_glx_read_pixels_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_read_pixels_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_read_pixels_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_read_pixels_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_read_pixels_data_end)
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
     * xcb_glx_read_pixels_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_read_pixels_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_read_pixels_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_read_pixels_reply_t {
        sym!(self, xcb_glx_read_pixels_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_read_pixels_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_read_pixels_reply(&self) -> bool {
        has_sym!(self, xcb_glx_read_pixels_reply)
    }

    pub unsafe fn xcb_glx_get_booleanv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_booleanv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_booleanv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_booleanv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_booleanv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_booleanv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        pname: i32,
    ) -> xcb_glx_get_booleanv_cookie_t {
        sym!(self, xcb_glx_get_booleanv)(c, context_tag, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_booleanv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_booleanv(&self) -> bool {
        has_sym!(self, xcb_glx_get_booleanv)
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
    pub unsafe fn xcb_glx_get_booleanv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        pname: i32,
    ) -> xcb_glx_get_booleanv_cookie_t {
        sym!(self, xcb_glx_get_booleanv_unchecked)(c, context_tag, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_booleanv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_booleanv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_booleanv_unchecked)
    }

    pub unsafe fn xcb_glx_get_booleanv_data(
        &self,
        r: *const xcb_glx_get_booleanv_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_glx_get_booleanv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_booleanv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_booleanv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_booleanv_data)
    }

    pub unsafe fn xcb_glx_get_booleanv_data_length(
        &self,
        r: *const xcb_glx_get_booleanv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_booleanv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_booleanv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_booleanv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_booleanv_data_length)
    }

    pub unsafe fn xcb_glx_get_booleanv_data_end(
        &self,
        r: *const xcb_glx_get_booleanv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_booleanv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_booleanv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_booleanv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_booleanv_data_end)
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
     * xcb_glx_get_booleanv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_booleanv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_booleanv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_booleanv_reply_t {
        sym!(self, xcb_glx_get_booleanv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_booleanv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_booleanv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_booleanv_reply)
    }

    pub unsafe fn xcb_glx_get_clip_plane_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_clip_plane_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_clip_plane_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_clip_plane_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_clip_plane_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_clip_plane(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        plane: i32,
    ) -> xcb_glx_get_clip_plane_cookie_t {
        sym!(self, xcb_glx_get_clip_plane)(c, context_tag, plane)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_clip_plane` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_clip_plane(&self) -> bool {
        has_sym!(self, xcb_glx_get_clip_plane)
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
    pub unsafe fn xcb_glx_get_clip_plane_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        plane: i32,
    ) -> xcb_glx_get_clip_plane_cookie_t {
        sym!(self, xcb_glx_get_clip_plane_unchecked)(c, context_tag, plane)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_clip_plane_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_clip_plane_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_clip_plane_unchecked)
    }

    pub unsafe fn xcb_glx_get_clip_plane_data(
        &self,
        r: *const xcb_glx_get_clip_plane_reply_t,
    ) -> *mut xcb_glx_float64_t {
        sym!(self, xcb_glx_get_clip_plane_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_clip_plane_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_clip_plane_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_clip_plane_data)
    }

    pub unsafe fn xcb_glx_get_clip_plane_data_length(
        &self,
        r: *const xcb_glx_get_clip_plane_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_clip_plane_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_clip_plane_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_clip_plane_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_clip_plane_data_length)
    }

    pub unsafe fn xcb_glx_get_clip_plane_data_end(
        &self,
        r: *const xcb_glx_get_clip_plane_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_clip_plane_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_clip_plane_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_clip_plane_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_clip_plane_data_end)
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
     * xcb_glx_get_clip_plane_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_clip_plane_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_clip_plane_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_clip_plane_reply_t {
        sym!(self, xcb_glx_get_clip_plane_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_clip_plane_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_clip_plane_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_clip_plane_reply)
    }

    pub unsafe fn xcb_glx_get_doublev_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_doublev_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_doublev_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_doublev_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_doublev_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_doublev(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        pname: u32,
    ) -> xcb_glx_get_doublev_cookie_t {
        sym!(self, xcb_glx_get_doublev)(c, context_tag, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_doublev` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_doublev(&self) -> bool {
        has_sym!(self, xcb_glx_get_doublev)
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
    pub unsafe fn xcb_glx_get_doublev_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        pname: u32,
    ) -> xcb_glx_get_doublev_cookie_t {
        sym!(self, xcb_glx_get_doublev_unchecked)(c, context_tag, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_doublev_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_doublev_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_doublev_unchecked)
    }

    pub unsafe fn xcb_glx_get_doublev_data(
        &self,
        r: *const xcb_glx_get_doublev_reply_t,
    ) -> *mut xcb_glx_float64_t {
        sym!(self, xcb_glx_get_doublev_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_doublev_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_doublev_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_doublev_data)
    }

    pub unsafe fn xcb_glx_get_doublev_data_length(
        &self,
        r: *const xcb_glx_get_doublev_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_doublev_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_doublev_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_doublev_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_doublev_data_length)
    }

    pub unsafe fn xcb_glx_get_doublev_data_end(
        &self,
        r: *const xcb_glx_get_doublev_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_doublev_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_doublev_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_doublev_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_doublev_data_end)
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
     * xcb_glx_get_doublev_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_doublev_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_doublev_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_doublev_reply_t {
        sym!(self, xcb_glx_get_doublev_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_doublev_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_doublev_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_doublev_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_error(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
    ) -> xcb_glx_get_error_cookie_t {
        sym!(self, xcb_glx_get_error)(c, context_tag)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_error` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_error(&self) -> bool {
        has_sym!(self, xcb_glx_get_error)
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
    pub unsafe fn xcb_glx_get_error_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
    ) -> xcb_glx_get_error_cookie_t {
        sym!(self, xcb_glx_get_error_unchecked)(c, context_tag)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_error_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_error_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_error_unchecked)
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
     * xcb_glx_get_error_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_error_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_error_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_error_reply_t {
        sym!(self, xcb_glx_get_error_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_error_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_error_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_error_reply)
    }

    pub unsafe fn xcb_glx_get_floatv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_floatv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_floatv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_floatv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_floatv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_floatv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        pname: u32,
    ) -> xcb_glx_get_floatv_cookie_t {
        sym!(self, xcb_glx_get_floatv)(c, context_tag, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_floatv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_floatv(&self) -> bool {
        has_sym!(self, xcb_glx_get_floatv)
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
    pub unsafe fn xcb_glx_get_floatv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        pname: u32,
    ) -> xcb_glx_get_floatv_cookie_t {
        sym!(self, xcb_glx_get_floatv_unchecked)(c, context_tag, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_floatv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_floatv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_floatv_unchecked)
    }

    pub unsafe fn xcb_glx_get_floatv_data(
        &self,
        r: *const xcb_glx_get_floatv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_floatv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_floatv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_floatv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_floatv_data)
    }

    pub unsafe fn xcb_glx_get_floatv_data_length(
        &self,
        r: *const xcb_glx_get_floatv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_floatv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_floatv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_floatv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_floatv_data_length)
    }

    pub unsafe fn xcb_glx_get_floatv_data_end(
        &self,
        r: *const xcb_glx_get_floatv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_floatv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_floatv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_floatv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_floatv_data_end)
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
     * xcb_glx_get_floatv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_floatv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_floatv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_floatv_reply_t {
        sym!(self, xcb_glx_get_floatv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_floatv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_floatv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_floatv_reply)
    }

    pub unsafe fn xcb_glx_get_integerv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_integerv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_integerv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_integerv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_integerv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_integerv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        pname: u32,
    ) -> xcb_glx_get_integerv_cookie_t {
        sym!(self, xcb_glx_get_integerv)(c, context_tag, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_integerv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_integerv(&self) -> bool {
        has_sym!(self, xcb_glx_get_integerv)
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
    pub unsafe fn xcb_glx_get_integerv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        pname: u32,
    ) -> xcb_glx_get_integerv_cookie_t {
        sym!(self, xcb_glx_get_integerv_unchecked)(c, context_tag, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_integerv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_integerv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_integerv_unchecked)
    }

    pub unsafe fn xcb_glx_get_integerv_data(
        &self,
        r: *const xcb_glx_get_integerv_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_integerv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_integerv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_integerv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_integerv_data)
    }

    pub unsafe fn xcb_glx_get_integerv_data_length(
        &self,
        r: *const xcb_glx_get_integerv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_integerv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_integerv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_integerv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_integerv_data_length)
    }

    pub unsafe fn xcb_glx_get_integerv_data_end(
        &self,
        r: *const xcb_glx_get_integerv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_integerv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_integerv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_integerv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_integerv_data_end)
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
     * xcb_glx_get_integerv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_integerv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_integerv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_integerv_reply_t {
        sym!(self, xcb_glx_get_integerv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_integerv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_integerv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_integerv_reply)
    }

    pub unsafe fn xcb_glx_get_lightfv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_lightfv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_lightfv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_lightfv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_lightfv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_lightfv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        light: u32,
        pname: u32,
    ) -> xcb_glx_get_lightfv_cookie_t {
        sym!(self, xcb_glx_get_lightfv)(c, context_tag, light, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_lightfv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_lightfv(&self) -> bool {
        has_sym!(self, xcb_glx_get_lightfv)
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
    pub unsafe fn xcb_glx_get_lightfv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        light: u32,
        pname: u32,
    ) -> xcb_glx_get_lightfv_cookie_t {
        sym!(self, xcb_glx_get_lightfv_unchecked)(c, context_tag, light, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_lightfv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_lightfv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_lightfv_unchecked)
    }

    pub unsafe fn xcb_glx_get_lightfv_data(
        &self,
        r: *const xcb_glx_get_lightfv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_lightfv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_lightfv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_lightfv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_lightfv_data)
    }

    pub unsafe fn xcb_glx_get_lightfv_data_length(
        &self,
        r: *const xcb_glx_get_lightfv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_lightfv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_lightfv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_lightfv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_lightfv_data_length)
    }

    pub unsafe fn xcb_glx_get_lightfv_data_end(
        &self,
        r: *const xcb_glx_get_lightfv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_lightfv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_lightfv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_lightfv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_lightfv_data_end)
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
     * xcb_glx_get_lightfv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_lightfv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_lightfv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_lightfv_reply_t {
        sym!(self, xcb_glx_get_lightfv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_lightfv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_lightfv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_lightfv_reply)
    }

    pub unsafe fn xcb_glx_get_lightiv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_lightiv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_lightiv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_lightiv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_lightiv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_lightiv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        light: u32,
        pname: u32,
    ) -> xcb_glx_get_lightiv_cookie_t {
        sym!(self, xcb_glx_get_lightiv)(c, context_tag, light, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_lightiv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_lightiv(&self) -> bool {
        has_sym!(self, xcb_glx_get_lightiv)
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
    pub unsafe fn xcb_glx_get_lightiv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        light: u32,
        pname: u32,
    ) -> xcb_glx_get_lightiv_cookie_t {
        sym!(self, xcb_glx_get_lightiv_unchecked)(c, context_tag, light, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_lightiv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_lightiv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_lightiv_unchecked)
    }

    pub unsafe fn xcb_glx_get_lightiv_data(
        &self,
        r: *const xcb_glx_get_lightiv_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_lightiv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_lightiv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_lightiv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_lightiv_data)
    }

    pub unsafe fn xcb_glx_get_lightiv_data_length(
        &self,
        r: *const xcb_glx_get_lightiv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_lightiv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_lightiv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_lightiv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_lightiv_data_length)
    }

    pub unsafe fn xcb_glx_get_lightiv_data_end(
        &self,
        r: *const xcb_glx_get_lightiv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_lightiv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_lightiv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_lightiv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_lightiv_data_end)
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
     * xcb_glx_get_lightiv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_lightiv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_lightiv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_lightiv_reply_t {
        sym!(self, xcb_glx_get_lightiv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_lightiv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_lightiv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_lightiv_reply)
    }

    pub unsafe fn xcb_glx_get_mapdv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_mapdv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_mapdv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_mapdv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_mapdv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_mapdv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        query: u32,
    ) -> xcb_glx_get_mapdv_cookie_t {
        sym!(self, xcb_glx_get_mapdv)(c, context_tag, target, query)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_mapdv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_mapdv(&self) -> bool {
        has_sym!(self, xcb_glx_get_mapdv)
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
    pub unsafe fn xcb_glx_get_mapdv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        query: u32,
    ) -> xcb_glx_get_mapdv_cookie_t {
        sym!(self, xcb_glx_get_mapdv_unchecked)(c, context_tag, target, query)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_mapdv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_mapdv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_mapdv_unchecked)
    }

    pub unsafe fn xcb_glx_get_mapdv_data(
        &self,
        r: *const xcb_glx_get_mapdv_reply_t,
    ) -> *mut xcb_glx_float64_t {
        sym!(self, xcb_glx_get_mapdv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_mapdv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_mapdv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_mapdv_data)
    }

    pub unsafe fn xcb_glx_get_mapdv_data_length(
        &self,
        r: *const xcb_glx_get_mapdv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_mapdv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_mapdv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_mapdv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_mapdv_data_length)
    }

    pub unsafe fn xcb_glx_get_mapdv_data_end(
        &self,
        r: *const xcb_glx_get_mapdv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_mapdv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_mapdv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_mapdv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_mapdv_data_end)
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
     * xcb_glx_get_mapdv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_mapdv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_mapdv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_mapdv_reply_t {
        sym!(self, xcb_glx_get_mapdv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_mapdv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_mapdv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_mapdv_reply)
    }

    pub unsafe fn xcb_glx_get_mapfv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_mapfv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_mapfv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_mapfv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_mapfv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_mapfv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        query: u32,
    ) -> xcb_glx_get_mapfv_cookie_t {
        sym!(self, xcb_glx_get_mapfv)(c, context_tag, target, query)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_mapfv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_mapfv(&self) -> bool {
        has_sym!(self, xcb_glx_get_mapfv)
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
    pub unsafe fn xcb_glx_get_mapfv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        query: u32,
    ) -> xcb_glx_get_mapfv_cookie_t {
        sym!(self, xcb_glx_get_mapfv_unchecked)(c, context_tag, target, query)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_mapfv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_mapfv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_mapfv_unchecked)
    }

    pub unsafe fn xcb_glx_get_mapfv_data(
        &self,
        r: *const xcb_glx_get_mapfv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_mapfv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_mapfv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_mapfv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_mapfv_data)
    }

    pub unsafe fn xcb_glx_get_mapfv_data_length(
        &self,
        r: *const xcb_glx_get_mapfv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_mapfv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_mapfv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_mapfv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_mapfv_data_length)
    }

    pub unsafe fn xcb_glx_get_mapfv_data_end(
        &self,
        r: *const xcb_glx_get_mapfv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_mapfv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_mapfv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_mapfv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_mapfv_data_end)
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
     * xcb_glx_get_mapfv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_mapfv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_mapfv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_mapfv_reply_t {
        sym!(self, xcb_glx_get_mapfv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_mapfv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_mapfv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_mapfv_reply)
    }

    pub unsafe fn xcb_glx_get_mapiv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_mapiv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_mapiv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_mapiv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_mapiv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_mapiv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        query: u32,
    ) -> xcb_glx_get_mapiv_cookie_t {
        sym!(self, xcb_glx_get_mapiv)(c, context_tag, target, query)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_mapiv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_mapiv(&self) -> bool {
        has_sym!(self, xcb_glx_get_mapiv)
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
    pub unsafe fn xcb_glx_get_mapiv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        query: u32,
    ) -> xcb_glx_get_mapiv_cookie_t {
        sym!(self, xcb_glx_get_mapiv_unchecked)(c, context_tag, target, query)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_mapiv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_mapiv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_mapiv_unchecked)
    }

    pub unsafe fn xcb_glx_get_mapiv_data(&self, r: *const xcb_glx_get_mapiv_reply_t) -> *mut i32 {
        sym!(self, xcb_glx_get_mapiv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_mapiv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_mapiv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_mapiv_data)
    }

    pub unsafe fn xcb_glx_get_mapiv_data_length(
        &self,
        r: *const xcb_glx_get_mapiv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_mapiv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_mapiv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_mapiv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_mapiv_data_length)
    }

    pub unsafe fn xcb_glx_get_mapiv_data_end(
        &self,
        r: *const xcb_glx_get_mapiv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_mapiv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_mapiv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_mapiv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_mapiv_data_end)
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
     * xcb_glx_get_mapiv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_mapiv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_mapiv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_mapiv_reply_t {
        sym!(self, xcb_glx_get_mapiv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_mapiv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_mapiv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_mapiv_reply)
    }

    pub unsafe fn xcb_glx_get_materialfv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_materialfv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_materialfv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_materialfv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_materialfv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_materialfv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        face: u32,
        pname: u32,
    ) -> xcb_glx_get_materialfv_cookie_t {
        sym!(self, xcb_glx_get_materialfv)(c, context_tag, face, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_materialfv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_materialfv(&self) -> bool {
        has_sym!(self, xcb_glx_get_materialfv)
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
    pub unsafe fn xcb_glx_get_materialfv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        face: u32,
        pname: u32,
    ) -> xcb_glx_get_materialfv_cookie_t {
        sym!(self, xcb_glx_get_materialfv_unchecked)(c, context_tag, face, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_materialfv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_materialfv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_materialfv_unchecked)
    }

    pub unsafe fn xcb_glx_get_materialfv_data(
        &self,
        r: *const xcb_glx_get_materialfv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_materialfv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_materialfv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_materialfv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_materialfv_data)
    }

    pub unsafe fn xcb_glx_get_materialfv_data_length(
        &self,
        r: *const xcb_glx_get_materialfv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_materialfv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_materialfv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_materialfv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_materialfv_data_length)
    }

    pub unsafe fn xcb_glx_get_materialfv_data_end(
        &self,
        r: *const xcb_glx_get_materialfv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_materialfv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_materialfv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_materialfv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_materialfv_data_end)
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
     * xcb_glx_get_materialfv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_materialfv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_materialfv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_materialfv_reply_t {
        sym!(self, xcb_glx_get_materialfv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_materialfv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_materialfv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_materialfv_reply)
    }

    pub unsafe fn xcb_glx_get_materialiv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_materialiv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_materialiv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_materialiv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_materialiv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_materialiv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        face: u32,
        pname: u32,
    ) -> xcb_glx_get_materialiv_cookie_t {
        sym!(self, xcb_glx_get_materialiv)(c, context_tag, face, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_materialiv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_materialiv(&self) -> bool {
        has_sym!(self, xcb_glx_get_materialiv)
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
    pub unsafe fn xcb_glx_get_materialiv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        face: u32,
        pname: u32,
    ) -> xcb_glx_get_materialiv_cookie_t {
        sym!(self, xcb_glx_get_materialiv_unchecked)(c, context_tag, face, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_materialiv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_materialiv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_materialiv_unchecked)
    }

    pub unsafe fn xcb_glx_get_materialiv_data(
        &self,
        r: *const xcb_glx_get_materialiv_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_materialiv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_materialiv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_materialiv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_materialiv_data)
    }

    pub unsafe fn xcb_glx_get_materialiv_data_length(
        &self,
        r: *const xcb_glx_get_materialiv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_materialiv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_materialiv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_materialiv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_materialiv_data_length)
    }

    pub unsafe fn xcb_glx_get_materialiv_data_end(
        &self,
        r: *const xcb_glx_get_materialiv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_materialiv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_materialiv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_materialiv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_materialiv_data_end)
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
     * xcb_glx_get_materialiv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_materialiv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_materialiv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_materialiv_reply_t {
        sym!(self, xcb_glx_get_materialiv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_materialiv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_materialiv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_materialiv_reply)
    }

    pub unsafe fn xcb_glx_get_pixel_mapfv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_pixel_mapfv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_pixel_mapfv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_pixel_mapfv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_pixel_mapfv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_pixel_mapfv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        map: u32,
    ) -> xcb_glx_get_pixel_mapfv_cookie_t {
        sym!(self, xcb_glx_get_pixel_mapfv)(c, context_tag, map)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_pixel_mapfv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_pixel_mapfv(&self) -> bool {
        has_sym!(self, xcb_glx_get_pixel_mapfv)
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
    pub unsafe fn xcb_glx_get_pixel_mapfv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        map: u32,
    ) -> xcb_glx_get_pixel_mapfv_cookie_t {
        sym!(self, xcb_glx_get_pixel_mapfv_unchecked)(c, context_tag, map)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_pixel_mapfv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_pixel_mapfv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_pixel_mapfv_unchecked)
    }

    pub unsafe fn xcb_glx_get_pixel_mapfv_data(
        &self,
        r: *const xcb_glx_get_pixel_mapfv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_pixel_mapfv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_pixel_mapfv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_pixel_mapfv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_pixel_mapfv_data)
    }

    pub unsafe fn xcb_glx_get_pixel_mapfv_data_length(
        &self,
        r: *const xcb_glx_get_pixel_mapfv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_pixel_mapfv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_pixel_mapfv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_pixel_mapfv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_pixel_mapfv_data_length)
    }

    pub unsafe fn xcb_glx_get_pixel_mapfv_data_end(
        &self,
        r: *const xcb_glx_get_pixel_mapfv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_pixel_mapfv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_pixel_mapfv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_pixel_mapfv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_pixel_mapfv_data_end)
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
     * xcb_glx_get_pixel_mapfv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_pixel_mapfv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_pixel_mapfv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_pixel_mapfv_reply_t {
        sym!(self, xcb_glx_get_pixel_mapfv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_pixel_mapfv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_pixel_mapfv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_pixel_mapfv_reply)
    }

    pub unsafe fn xcb_glx_get_pixel_mapuiv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_pixel_mapuiv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_pixel_mapuiv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_pixel_mapuiv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_pixel_mapuiv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_pixel_mapuiv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        map: u32,
    ) -> xcb_glx_get_pixel_mapuiv_cookie_t {
        sym!(self, xcb_glx_get_pixel_mapuiv)(c, context_tag, map)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_pixel_mapuiv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_pixel_mapuiv(&self) -> bool {
        has_sym!(self, xcb_glx_get_pixel_mapuiv)
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
    pub unsafe fn xcb_glx_get_pixel_mapuiv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        map: u32,
    ) -> xcb_glx_get_pixel_mapuiv_cookie_t {
        sym!(self, xcb_glx_get_pixel_mapuiv_unchecked)(c, context_tag, map)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_pixel_mapuiv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_pixel_mapuiv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_pixel_mapuiv_unchecked)
    }

    pub unsafe fn xcb_glx_get_pixel_mapuiv_data(
        &self,
        r: *const xcb_glx_get_pixel_mapuiv_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_get_pixel_mapuiv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_pixel_mapuiv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_pixel_mapuiv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_pixel_mapuiv_data)
    }

    pub unsafe fn xcb_glx_get_pixel_mapuiv_data_length(
        &self,
        r: *const xcb_glx_get_pixel_mapuiv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_pixel_mapuiv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_pixel_mapuiv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_pixel_mapuiv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_pixel_mapuiv_data_length)
    }

    pub unsafe fn xcb_glx_get_pixel_mapuiv_data_end(
        &self,
        r: *const xcb_glx_get_pixel_mapuiv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_pixel_mapuiv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_pixel_mapuiv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_pixel_mapuiv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_pixel_mapuiv_data_end)
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
     * xcb_glx_get_pixel_mapuiv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_pixel_mapuiv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_pixel_mapuiv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_pixel_mapuiv_reply_t {
        sym!(self, xcb_glx_get_pixel_mapuiv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_pixel_mapuiv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_pixel_mapuiv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_pixel_mapuiv_reply)
    }

    pub unsafe fn xcb_glx_get_pixel_mapusv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_pixel_mapusv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_pixel_mapusv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_pixel_mapusv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_pixel_mapusv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_pixel_mapusv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        map: u32,
    ) -> xcb_glx_get_pixel_mapusv_cookie_t {
        sym!(self, xcb_glx_get_pixel_mapusv)(c, context_tag, map)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_pixel_mapusv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_pixel_mapusv(&self) -> bool {
        has_sym!(self, xcb_glx_get_pixel_mapusv)
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
    pub unsafe fn xcb_glx_get_pixel_mapusv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        map: u32,
    ) -> xcb_glx_get_pixel_mapusv_cookie_t {
        sym!(self, xcb_glx_get_pixel_mapusv_unchecked)(c, context_tag, map)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_pixel_mapusv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_pixel_mapusv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_pixel_mapusv_unchecked)
    }

    pub unsafe fn xcb_glx_get_pixel_mapusv_data(
        &self,
        r: *const xcb_glx_get_pixel_mapusv_reply_t,
    ) -> *mut u16 {
        sym!(self, xcb_glx_get_pixel_mapusv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_pixel_mapusv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_pixel_mapusv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_pixel_mapusv_data)
    }

    pub unsafe fn xcb_glx_get_pixel_mapusv_data_length(
        &self,
        r: *const xcb_glx_get_pixel_mapusv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_pixel_mapusv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_pixel_mapusv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_pixel_mapusv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_pixel_mapusv_data_length)
    }

    pub unsafe fn xcb_glx_get_pixel_mapusv_data_end(
        &self,
        r: *const xcb_glx_get_pixel_mapusv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_pixel_mapusv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_pixel_mapusv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_pixel_mapusv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_pixel_mapusv_data_end)
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
     * xcb_glx_get_pixel_mapusv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_pixel_mapusv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_pixel_mapusv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_pixel_mapusv_reply_t {
        sym!(self, xcb_glx_get_pixel_mapusv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_pixel_mapusv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_pixel_mapusv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_pixel_mapusv_reply)
    }

    pub unsafe fn xcb_glx_get_polygon_stipple_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_polygon_stipple_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_polygon_stipple_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_polygon_stipple_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_polygon_stipple_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_polygon_stipple(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        lsb_first: u8,
    ) -> xcb_glx_get_polygon_stipple_cookie_t {
        sym!(self, xcb_glx_get_polygon_stipple)(c, context_tag, lsb_first)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_polygon_stipple` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_polygon_stipple(&self) -> bool {
        has_sym!(self, xcb_glx_get_polygon_stipple)
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
    pub unsafe fn xcb_glx_get_polygon_stipple_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        lsb_first: u8,
    ) -> xcb_glx_get_polygon_stipple_cookie_t {
        sym!(self, xcb_glx_get_polygon_stipple_unchecked)(c, context_tag, lsb_first)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_polygon_stipple_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_polygon_stipple_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_polygon_stipple_unchecked)
    }

    pub unsafe fn xcb_glx_get_polygon_stipple_data(
        &self,
        r: *const xcb_glx_get_polygon_stipple_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_glx_get_polygon_stipple_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_polygon_stipple_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_polygon_stipple_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_polygon_stipple_data)
    }

    pub unsafe fn xcb_glx_get_polygon_stipple_data_length(
        &self,
        r: *const xcb_glx_get_polygon_stipple_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_polygon_stipple_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_polygon_stipple_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_polygon_stipple_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_polygon_stipple_data_length)
    }

    pub unsafe fn xcb_glx_get_polygon_stipple_data_end(
        &self,
        r: *const xcb_glx_get_polygon_stipple_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_polygon_stipple_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_polygon_stipple_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_polygon_stipple_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_polygon_stipple_data_end)
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
     * xcb_glx_get_polygon_stipple_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_polygon_stipple_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_polygon_stipple_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_polygon_stipple_reply_t {
        sym!(self, xcb_glx_get_polygon_stipple_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_polygon_stipple_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_polygon_stipple_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_polygon_stipple_reply)
    }

    pub unsafe fn xcb_glx_get_string_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_string_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_string_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_string_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_string_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_string(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        name: u32,
    ) -> xcb_glx_get_string_cookie_t {
        sym!(self, xcb_glx_get_string)(c, context_tag, name)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_string` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_string(&self) -> bool {
        has_sym!(self, xcb_glx_get_string)
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
    pub unsafe fn xcb_glx_get_string_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        name: u32,
    ) -> xcb_glx_get_string_cookie_t {
        sym!(self, xcb_glx_get_string_unchecked)(c, context_tag, name)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_string_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_string_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_string_unchecked)
    }

    pub unsafe fn xcb_glx_get_string_string(
        &self,
        r: *const xcb_glx_get_string_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_glx_get_string_string)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_string_string` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_string_string(&self) -> bool {
        has_sym!(self, xcb_glx_get_string_string)
    }

    pub unsafe fn xcb_glx_get_string_string_length(
        &self,
        r: *const xcb_glx_get_string_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_string_string_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_string_string_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_string_string_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_string_string_length)
    }

    pub unsafe fn xcb_glx_get_string_string_end(
        &self,
        r: *const xcb_glx_get_string_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_string_string_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_string_string_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_string_string_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_string_string_end)
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
     * xcb_glx_get_string_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_string_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_string_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_string_reply_t {
        sym!(self, xcb_glx_get_string_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_string_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_string_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_string_reply)
    }

    pub unsafe fn xcb_glx_get_tex_envfv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_tex_envfv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_envfv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_envfv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_envfv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_tex_envfv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_envfv_cookie_t {
        sym!(self, xcb_glx_get_tex_envfv)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_envfv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_envfv(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_envfv)
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
    pub unsafe fn xcb_glx_get_tex_envfv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_envfv_cookie_t {
        sym!(self, xcb_glx_get_tex_envfv_unchecked)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_envfv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_envfv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_envfv_unchecked)
    }

    pub unsafe fn xcb_glx_get_tex_envfv_data(
        &self,
        r: *const xcb_glx_get_tex_envfv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_tex_envfv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_envfv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_envfv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_envfv_data)
    }

    pub unsafe fn xcb_glx_get_tex_envfv_data_length(
        &self,
        r: *const xcb_glx_get_tex_envfv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_tex_envfv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_envfv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_envfv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_envfv_data_length)
    }

    pub unsafe fn xcb_glx_get_tex_envfv_data_end(
        &self,
        r: *const xcb_glx_get_tex_envfv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_tex_envfv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_envfv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_envfv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_envfv_data_end)
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
     * xcb_glx_get_tex_envfv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_tex_envfv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_tex_envfv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_tex_envfv_reply_t {
        sym!(self, xcb_glx_get_tex_envfv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_envfv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_envfv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_envfv_reply)
    }

    pub unsafe fn xcb_glx_get_tex_enviv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_tex_enviv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_enviv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_enviv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_enviv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_tex_enviv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_enviv_cookie_t {
        sym!(self, xcb_glx_get_tex_enviv)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_enviv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_enviv(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_enviv)
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
    pub unsafe fn xcb_glx_get_tex_enviv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_enviv_cookie_t {
        sym!(self, xcb_glx_get_tex_enviv_unchecked)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_enviv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_enviv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_enviv_unchecked)
    }

    pub unsafe fn xcb_glx_get_tex_enviv_data(
        &self,
        r: *const xcb_glx_get_tex_enviv_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_tex_enviv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_enviv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_enviv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_enviv_data)
    }

    pub unsafe fn xcb_glx_get_tex_enviv_data_length(
        &self,
        r: *const xcb_glx_get_tex_enviv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_tex_enviv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_enviv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_enviv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_enviv_data_length)
    }

    pub unsafe fn xcb_glx_get_tex_enviv_data_end(
        &self,
        r: *const xcb_glx_get_tex_enviv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_tex_enviv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_enviv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_enviv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_enviv_data_end)
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
     * xcb_glx_get_tex_enviv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_tex_enviv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_tex_enviv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_tex_enviv_reply_t {
        sym!(self, xcb_glx_get_tex_enviv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_enviv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_enviv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_enviv_reply)
    }

    pub unsafe fn xcb_glx_get_tex_gendv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_tex_gendv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_gendv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_gendv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_gendv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_tex_gendv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        coord: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_gendv_cookie_t {
        sym!(self, xcb_glx_get_tex_gendv)(c, context_tag, coord, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_gendv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_gendv(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_gendv)
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
    pub unsafe fn xcb_glx_get_tex_gendv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        coord: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_gendv_cookie_t {
        sym!(self, xcb_glx_get_tex_gendv_unchecked)(c, context_tag, coord, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_gendv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_gendv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_gendv_unchecked)
    }

    pub unsafe fn xcb_glx_get_tex_gendv_data(
        &self,
        r: *const xcb_glx_get_tex_gendv_reply_t,
    ) -> *mut xcb_glx_float64_t {
        sym!(self, xcb_glx_get_tex_gendv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_gendv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_gendv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_gendv_data)
    }

    pub unsafe fn xcb_glx_get_tex_gendv_data_length(
        &self,
        r: *const xcb_glx_get_tex_gendv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_tex_gendv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_gendv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_gendv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_gendv_data_length)
    }

    pub unsafe fn xcb_glx_get_tex_gendv_data_end(
        &self,
        r: *const xcb_glx_get_tex_gendv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_tex_gendv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_gendv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_gendv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_gendv_data_end)
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
     * xcb_glx_get_tex_gendv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_tex_gendv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_tex_gendv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_tex_gendv_reply_t {
        sym!(self, xcb_glx_get_tex_gendv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_gendv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_gendv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_gendv_reply)
    }

    pub unsafe fn xcb_glx_get_tex_genfv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_tex_genfv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_genfv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_genfv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_genfv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_tex_genfv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        coord: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_genfv_cookie_t {
        sym!(self, xcb_glx_get_tex_genfv)(c, context_tag, coord, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_genfv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_genfv(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_genfv)
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
    pub unsafe fn xcb_glx_get_tex_genfv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        coord: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_genfv_cookie_t {
        sym!(self, xcb_glx_get_tex_genfv_unchecked)(c, context_tag, coord, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_genfv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_genfv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_genfv_unchecked)
    }

    pub unsafe fn xcb_glx_get_tex_genfv_data(
        &self,
        r: *const xcb_glx_get_tex_genfv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_tex_genfv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_genfv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_genfv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_genfv_data)
    }

    pub unsafe fn xcb_glx_get_tex_genfv_data_length(
        &self,
        r: *const xcb_glx_get_tex_genfv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_tex_genfv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_genfv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_genfv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_genfv_data_length)
    }

    pub unsafe fn xcb_glx_get_tex_genfv_data_end(
        &self,
        r: *const xcb_glx_get_tex_genfv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_tex_genfv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_genfv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_genfv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_genfv_data_end)
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
     * xcb_glx_get_tex_genfv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_tex_genfv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_tex_genfv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_tex_genfv_reply_t {
        sym!(self, xcb_glx_get_tex_genfv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_genfv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_genfv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_genfv_reply)
    }

    pub unsafe fn xcb_glx_get_tex_geniv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_tex_geniv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_geniv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_geniv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_geniv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_tex_geniv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        coord: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_geniv_cookie_t {
        sym!(self, xcb_glx_get_tex_geniv)(c, context_tag, coord, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_geniv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_geniv(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_geniv)
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
    pub unsafe fn xcb_glx_get_tex_geniv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        coord: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_geniv_cookie_t {
        sym!(self, xcb_glx_get_tex_geniv_unchecked)(c, context_tag, coord, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_geniv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_geniv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_geniv_unchecked)
    }

    pub unsafe fn xcb_glx_get_tex_geniv_data(
        &self,
        r: *const xcb_glx_get_tex_geniv_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_tex_geniv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_geniv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_geniv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_geniv_data)
    }

    pub unsafe fn xcb_glx_get_tex_geniv_data_length(
        &self,
        r: *const xcb_glx_get_tex_geniv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_tex_geniv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_geniv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_geniv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_geniv_data_length)
    }

    pub unsafe fn xcb_glx_get_tex_geniv_data_end(
        &self,
        r: *const xcb_glx_get_tex_geniv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_tex_geniv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_geniv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_geniv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_geniv_data_end)
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
     * xcb_glx_get_tex_geniv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_tex_geniv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_tex_geniv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_tex_geniv_reply_t {
        sym!(self, xcb_glx_get_tex_geniv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_geniv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_geniv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_geniv_reply)
    }

    pub unsafe fn xcb_glx_get_tex_image_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_tex_image_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_image_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_image_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_image_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_glx_get_tex_image` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_image(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_image)
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

    /// Returns `true` iff the symbol `xcb_glx_get_tex_image_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_image_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_image_unchecked)
    }

    pub unsafe fn xcb_glx_get_tex_image_data(
        &self,
        r: *const xcb_glx_get_tex_image_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_glx_get_tex_image_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_image_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_image_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_image_data)
    }

    pub unsafe fn xcb_glx_get_tex_image_data_length(
        &self,
        r: *const xcb_glx_get_tex_image_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_tex_image_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_image_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_image_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_image_data_length)
    }

    pub unsafe fn xcb_glx_get_tex_image_data_end(
        &self,
        r: *const xcb_glx_get_tex_image_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_tex_image_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_image_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_image_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_image_data_end)
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
     * xcb_glx_get_tex_image_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_tex_image_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_tex_image_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_tex_image_reply_t {
        sym!(self, xcb_glx_get_tex_image_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_image_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_image_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_image_reply)
    }

    pub unsafe fn xcb_glx_get_tex_parameterfv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_tex_parameterfv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_parameterfv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_parameterfv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_parameterfv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_tex_parameterfv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_parameterfv_cookie_t {
        sym!(self, xcb_glx_get_tex_parameterfv)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_parameterfv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_parameterfv(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_parameterfv)
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
    pub unsafe fn xcb_glx_get_tex_parameterfv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_parameterfv_cookie_t {
        sym!(self, xcb_glx_get_tex_parameterfv_unchecked)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_parameterfv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_parameterfv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_parameterfv_unchecked)
    }

    pub unsafe fn xcb_glx_get_tex_parameterfv_data(
        &self,
        r: *const xcb_glx_get_tex_parameterfv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_tex_parameterfv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_parameterfv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_parameterfv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_parameterfv_data)
    }

    pub unsafe fn xcb_glx_get_tex_parameterfv_data_length(
        &self,
        r: *const xcb_glx_get_tex_parameterfv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_tex_parameterfv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_parameterfv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_parameterfv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_parameterfv_data_length)
    }

    pub unsafe fn xcb_glx_get_tex_parameterfv_data_end(
        &self,
        r: *const xcb_glx_get_tex_parameterfv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_tex_parameterfv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_parameterfv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_parameterfv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_parameterfv_data_end)
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
     * xcb_glx_get_tex_parameterfv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_tex_parameterfv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_tex_parameterfv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_tex_parameterfv_reply_t {
        sym!(self, xcb_glx_get_tex_parameterfv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_parameterfv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_parameterfv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_parameterfv_reply)
    }

    pub unsafe fn xcb_glx_get_tex_parameteriv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_tex_parameteriv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_parameteriv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_parameteriv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_parameteriv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_tex_parameteriv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_parameteriv_cookie_t {
        sym!(self, xcb_glx_get_tex_parameteriv)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_parameteriv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_parameteriv(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_parameteriv)
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
    pub unsafe fn xcb_glx_get_tex_parameteriv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_tex_parameteriv_cookie_t {
        sym!(self, xcb_glx_get_tex_parameteriv_unchecked)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_parameteriv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_parameteriv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_parameteriv_unchecked)
    }

    pub unsafe fn xcb_glx_get_tex_parameteriv_data(
        &self,
        r: *const xcb_glx_get_tex_parameteriv_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_tex_parameteriv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_parameteriv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_parameteriv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_parameteriv_data)
    }

    pub unsafe fn xcb_glx_get_tex_parameteriv_data_length(
        &self,
        r: *const xcb_glx_get_tex_parameteriv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_tex_parameteriv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_parameteriv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_parameteriv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_parameteriv_data_length)
    }

    pub unsafe fn xcb_glx_get_tex_parameteriv_data_end(
        &self,
        r: *const xcb_glx_get_tex_parameteriv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_tex_parameteriv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_parameteriv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_parameteriv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_parameteriv_data_end)
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
     * xcb_glx_get_tex_parameteriv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_tex_parameteriv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_tex_parameteriv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_tex_parameteriv_reply_t {
        sym!(self, xcb_glx_get_tex_parameteriv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_parameteriv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_parameteriv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_parameteriv_reply)
    }

    pub unsafe fn xcb_glx_get_tex_level_parameterfv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_tex_level_parameterfv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_level_parameterfv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_level_parameterfv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_level_parameterfv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_glx_get_tex_level_parameterfv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_level_parameterfv(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_level_parameterfv)
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

    /// Returns `true` iff the symbol `xcb_glx_get_tex_level_parameterfv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_level_parameterfv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_level_parameterfv_unchecked)
    }

    pub unsafe fn xcb_glx_get_tex_level_parameterfv_data(
        &self,
        r: *const xcb_glx_get_tex_level_parameterfv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_tex_level_parameterfv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_level_parameterfv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_level_parameterfv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_level_parameterfv_data)
    }

    pub unsafe fn xcb_glx_get_tex_level_parameterfv_data_length(
        &self,
        r: *const xcb_glx_get_tex_level_parameterfv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_tex_level_parameterfv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_level_parameterfv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_level_parameterfv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_level_parameterfv_data_length)
    }

    pub unsafe fn xcb_glx_get_tex_level_parameterfv_data_end(
        &self,
        r: *const xcb_glx_get_tex_level_parameterfv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_tex_level_parameterfv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_level_parameterfv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_level_parameterfv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_level_parameterfv_data_end)
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
     * xcb_glx_get_tex_level_parameterfv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_tex_level_parameterfv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_tex_level_parameterfv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_tex_level_parameterfv_reply_t {
        sym!(self, xcb_glx_get_tex_level_parameterfv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_level_parameterfv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_level_parameterfv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_level_parameterfv_reply)
    }

    pub unsafe fn xcb_glx_get_tex_level_parameteriv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_tex_level_parameteriv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_level_parameteriv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_level_parameteriv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_level_parameteriv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_glx_get_tex_level_parameteriv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_level_parameteriv(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_level_parameteriv)
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

    /// Returns `true` iff the symbol `xcb_glx_get_tex_level_parameteriv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_level_parameteriv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_level_parameteriv_unchecked)
    }

    pub unsafe fn xcb_glx_get_tex_level_parameteriv_data(
        &self,
        r: *const xcb_glx_get_tex_level_parameteriv_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_tex_level_parameteriv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_level_parameteriv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_level_parameteriv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_level_parameteriv_data)
    }

    pub unsafe fn xcb_glx_get_tex_level_parameteriv_data_length(
        &self,
        r: *const xcb_glx_get_tex_level_parameteriv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_tex_level_parameteriv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_level_parameteriv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_level_parameteriv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_level_parameteriv_data_length)
    }

    pub unsafe fn xcb_glx_get_tex_level_parameteriv_data_end(
        &self,
        r: *const xcb_glx_get_tex_level_parameteriv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_tex_level_parameteriv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_level_parameteriv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_level_parameteriv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_level_parameteriv_data_end)
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
     * xcb_glx_get_tex_level_parameteriv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_tex_level_parameteriv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_tex_level_parameteriv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_tex_level_parameteriv_reply_t {
        sym!(self, xcb_glx_get_tex_level_parameteriv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_tex_level_parameteriv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_tex_level_parameteriv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_tex_level_parameteriv_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_is_enabled(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        capability: u32,
    ) -> xcb_glx_is_enabled_cookie_t {
        sym!(self, xcb_glx_is_enabled)(c, context_tag, capability)
    }

    /// Returns `true` iff the symbol `xcb_glx_is_enabled` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_is_enabled(&self) -> bool {
        has_sym!(self, xcb_glx_is_enabled)
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
    pub unsafe fn xcb_glx_is_enabled_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        capability: u32,
    ) -> xcb_glx_is_enabled_cookie_t {
        sym!(self, xcb_glx_is_enabled_unchecked)(c, context_tag, capability)
    }

    /// Returns `true` iff the symbol `xcb_glx_is_enabled_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_is_enabled_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_is_enabled_unchecked)
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
     * xcb_glx_is_enabled_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_is_enabled_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_is_enabled_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_is_enabled_reply_t {
        sym!(self, xcb_glx_is_enabled_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_is_enabled_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_is_enabled_reply(&self) -> bool {
        has_sym!(self, xcb_glx_is_enabled_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_is_list(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        list: u32,
    ) -> xcb_glx_is_list_cookie_t {
        sym!(self, xcb_glx_is_list)(c, context_tag, list)
    }

    /// Returns `true` iff the symbol `xcb_glx_is_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_is_list(&self) -> bool {
        has_sym!(self, xcb_glx_is_list)
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
    pub unsafe fn xcb_glx_is_list_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        list: u32,
    ) -> xcb_glx_is_list_cookie_t {
        sym!(self, xcb_glx_is_list_unchecked)(c, context_tag, list)
    }

    /// Returns `true` iff the symbol `xcb_glx_is_list_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_is_list_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_is_list_unchecked)
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
     * xcb_glx_is_list_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_is_list_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_is_list_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_is_list_reply_t {
        sym!(self, xcb_glx_is_list_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_is_list_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_is_list_reply(&self) -> bool {
        has_sym!(self, xcb_glx_is_list_reply)
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
    pub unsafe fn xcb_glx_flush_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_flush_checked)(c, context_tag)
    }

    /// Returns `true` iff the symbol `xcb_glx_flush_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_flush_checked(&self) -> bool {
        has_sym!(self, xcb_glx_flush_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_flush(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_flush)(c, context_tag)
    }

    /// Returns `true` iff the symbol `xcb_glx_flush` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_flush(&self) -> bool {
        has_sym!(self, xcb_glx_flush)
    }

    pub unsafe fn xcb_glx_are_textures_resident_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_are_textures_resident_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_are_textures_resident_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_are_textures_resident_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_are_textures_resident_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_are_textures_resident(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        n: i32,
        textures: *const u32,
    ) -> xcb_glx_are_textures_resident_cookie_t {
        sym!(self, xcb_glx_are_textures_resident)(c, context_tag, n, textures)
    }

    /// Returns `true` iff the symbol `xcb_glx_are_textures_resident` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_are_textures_resident(&self) -> bool {
        has_sym!(self, xcb_glx_are_textures_resident)
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
    pub unsafe fn xcb_glx_are_textures_resident_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        n: i32,
        textures: *const u32,
    ) -> xcb_glx_are_textures_resident_cookie_t {
        sym!(self, xcb_glx_are_textures_resident_unchecked)(c, context_tag, n, textures)
    }

    /// Returns `true` iff the symbol `xcb_glx_are_textures_resident_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_are_textures_resident_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_are_textures_resident_unchecked)
    }

    pub unsafe fn xcb_glx_are_textures_resident_data(
        &self,
        r: *const xcb_glx_are_textures_resident_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_glx_are_textures_resident_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_are_textures_resident_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_are_textures_resident_data(&self) -> bool {
        has_sym!(self, xcb_glx_are_textures_resident_data)
    }

    pub unsafe fn xcb_glx_are_textures_resident_data_length(
        &self,
        r: *const xcb_glx_are_textures_resident_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_are_textures_resident_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_are_textures_resident_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_are_textures_resident_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_are_textures_resident_data_length)
    }

    pub unsafe fn xcb_glx_are_textures_resident_data_end(
        &self,
        r: *const xcb_glx_are_textures_resident_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_are_textures_resident_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_are_textures_resident_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_are_textures_resident_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_are_textures_resident_data_end)
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
     * xcb_glx_are_textures_resident_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_are_textures_resident_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_are_textures_resident_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_are_textures_resident_reply_t {
        sym!(self, xcb_glx_are_textures_resident_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_are_textures_resident_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_are_textures_resident_reply(&self) -> bool {
        has_sym!(self, xcb_glx_are_textures_resident_reply)
    }

    pub unsafe fn xcb_glx_delete_textures_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_delete_textures_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_delete_textures_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_delete_textures_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_delete_textures_sizeof)
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
    pub unsafe fn xcb_glx_delete_textures_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        n: i32,
        textures: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_delete_textures_checked)(c, context_tag, n, textures)
    }

    /// Returns `true` iff the symbol `xcb_glx_delete_textures_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_delete_textures_checked(&self) -> bool {
        has_sym!(self, xcb_glx_delete_textures_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_delete_textures(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        n: i32,
        textures: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_delete_textures)(c, context_tag, n, textures)
    }

    /// Returns `true` iff the symbol `xcb_glx_delete_textures` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_delete_textures(&self) -> bool {
        has_sym!(self, xcb_glx_delete_textures)
    }

    pub unsafe fn xcb_glx_delete_textures_textures(
        &self,
        r: *const xcb_glx_delete_textures_request_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_delete_textures_textures)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_delete_textures_textures` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_delete_textures_textures(&self) -> bool {
        has_sym!(self, xcb_glx_delete_textures_textures)
    }

    pub unsafe fn xcb_glx_delete_textures_textures_length(
        &self,
        r: *const xcb_glx_delete_textures_request_t,
    ) -> c_int {
        sym!(self, xcb_glx_delete_textures_textures_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_delete_textures_textures_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_delete_textures_textures_length(&self) -> bool {
        has_sym!(self, xcb_glx_delete_textures_textures_length)
    }

    pub unsafe fn xcb_glx_delete_textures_textures_end(
        &self,
        r: *const xcb_glx_delete_textures_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_delete_textures_textures_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_delete_textures_textures_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_delete_textures_textures_end(&self) -> bool {
        has_sym!(self, xcb_glx_delete_textures_textures_end)
    }

    pub unsafe fn xcb_glx_gen_textures_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_gen_textures_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_gen_textures_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_gen_textures_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_gen_textures_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_gen_textures(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        n: i32,
    ) -> xcb_glx_gen_textures_cookie_t {
        sym!(self, xcb_glx_gen_textures)(c, context_tag, n)
    }

    /// Returns `true` iff the symbol `xcb_glx_gen_textures` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_gen_textures(&self) -> bool {
        has_sym!(self, xcb_glx_gen_textures)
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
    pub unsafe fn xcb_glx_gen_textures_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        n: i32,
    ) -> xcb_glx_gen_textures_cookie_t {
        sym!(self, xcb_glx_gen_textures_unchecked)(c, context_tag, n)
    }

    /// Returns `true` iff the symbol `xcb_glx_gen_textures_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_gen_textures_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_gen_textures_unchecked)
    }

    pub unsafe fn xcb_glx_gen_textures_data(
        &self,
        r: *const xcb_glx_gen_textures_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_gen_textures_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_gen_textures_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_gen_textures_data(&self) -> bool {
        has_sym!(self, xcb_glx_gen_textures_data)
    }

    pub unsafe fn xcb_glx_gen_textures_data_length(
        &self,
        r: *const xcb_glx_gen_textures_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_gen_textures_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_gen_textures_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_gen_textures_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_gen_textures_data_length)
    }

    pub unsafe fn xcb_glx_gen_textures_data_end(
        &self,
        r: *const xcb_glx_gen_textures_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_gen_textures_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_gen_textures_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_gen_textures_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_gen_textures_data_end)
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
     * xcb_glx_gen_textures_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_gen_textures_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_gen_textures_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_gen_textures_reply_t {
        sym!(self, xcb_glx_gen_textures_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_gen_textures_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_gen_textures_reply(&self) -> bool {
        has_sym!(self, xcb_glx_gen_textures_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_is_texture(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        texture: u32,
    ) -> xcb_glx_is_texture_cookie_t {
        sym!(self, xcb_glx_is_texture)(c, context_tag, texture)
    }

    /// Returns `true` iff the symbol `xcb_glx_is_texture` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_is_texture(&self) -> bool {
        has_sym!(self, xcb_glx_is_texture)
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
    pub unsafe fn xcb_glx_is_texture_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        texture: u32,
    ) -> xcb_glx_is_texture_cookie_t {
        sym!(self, xcb_glx_is_texture_unchecked)(c, context_tag, texture)
    }

    /// Returns `true` iff the symbol `xcb_glx_is_texture_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_is_texture_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_is_texture_unchecked)
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
     * xcb_glx_is_texture_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_is_texture_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_is_texture_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_is_texture_reply_t {
        sym!(self, xcb_glx_is_texture_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_is_texture_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_is_texture_reply(&self) -> bool {
        has_sym!(self, xcb_glx_is_texture_reply)
    }

    pub unsafe fn xcb_glx_get_color_table_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_color_table_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_color_table_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_color_table_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_color_table_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_glx_get_color_table` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_color_table(&self) -> bool {
        has_sym!(self, xcb_glx_get_color_table)
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

    /// Returns `true` iff the symbol `xcb_glx_get_color_table_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_color_table_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_color_table_unchecked)
    }

    pub unsafe fn xcb_glx_get_color_table_data(
        &self,
        r: *const xcb_glx_get_color_table_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_glx_get_color_table_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_color_table_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_color_table_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_color_table_data)
    }

    pub unsafe fn xcb_glx_get_color_table_data_length(
        &self,
        r: *const xcb_glx_get_color_table_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_color_table_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_color_table_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_color_table_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_color_table_data_length)
    }

    pub unsafe fn xcb_glx_get_color_table_data_end(
        &self,
        r: *const xcb_glx_get_color_table_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_color_table_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_color_table_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_color_table_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_color_table_data_end)
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
     * xcb_glx_get_color_table_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_color_table_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_color_table_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_color_table_reply_t {
        sym!(self, xcb_glx_get_color_table_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_color_table_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_color_table_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_color_table_reply)
    }

    pub unsafe fn xcb_glx_get_color_table_parameterfv_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_glx_get_color_table_parameterfv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_color_table_parameterfv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_color_table_parameterfv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_color_table_parameterfv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_color_table_parameterfv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_color_table_parameterfv_cookie_t {
        sym!(self, xcb_glx_get_color_table_parameterfv)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_color_table_parameterfv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_color_table_parameterfv(&self) -> bool {
        has_sym!(self, xcb_glx_get_color_table_parameterfv)
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
    pub unsafe fn xcb_glx_get_color_table_parameterfv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_color_table_parameterfv_cookie_t {
        sym!(self, xcb_glx_get_color_table_parameterfv_unchecked)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_color_table_parameterfv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_color_table_parameterfv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_color_table_parameterfv_unchecked)
    }

    pub unsafe fn xcb_glx_get_color_table_parameterfv_data(
        &self,
        r: *const xcb_glx_get_color_table_parameterfv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_color_table_parameterfv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_color_table_parameterfv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_color_table_parameterfv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_color_table_parameterfv_data)
    }

    pub unsafe fn xcb_glx_get_color_table_parameterfv_data_length(
        &self,
        r: *const xcb_glx_get_color_table_parameterfv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_color_table_parameterfv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_color_table_parameterfv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_color_table_parameterfv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_color_table_parameterfv_data_length)
    }

    pub unsafe fn xcb_glx_get_color_table_parameterfv_data_end(
        &self,
        r: *const xcb_glx_get_color_table_parameterfv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_color_table_parameterfv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_color_table_parameterfv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_color_table_parameterfv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_color_table_parameterfv_data_end)
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
     * xcb_glx_get_color_table_parameterfv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_color_table_parameterfv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_color_table_parameterfv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_color_table_parameterfv_reply_t {
        sym!(self, xcb_glx_get_color_table_parameterfv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_color_table_parameterfv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_color_table_parameterfv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_color_table_parameterfv_reply)
    }

    pub unsafe fn xcb_glx_get_color_table_parameteriv_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_glx_get_color_table_parameteriv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_color_table_parameteriv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_color_table_parameteriv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_color_table_parameteriv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_color_table_parameteriv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_color_table_parameteriv_cookie_t {
        sym!(self, xcb_glx_get_color_table_parameteriv)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_color_table_parameteriv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_color_table_parameteriv(&self) -> bool {
        has_sym!(self, xcb_glx_get_color_table_parameteriv)
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
    pub unsafe fn xcb_glx_get_color_table_parameteriv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_color_table_parameteriv_cookie_t {
        sym!(self, xcb_glx_get_color_table_parameteriv_unchecked)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_color_table_parameteriv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_color_table_parameteriv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_color_table_parameteriv_unchecked)
    }

    pub unsafe fn xcb_glx_get_color_table_parameteriv_data(
        &self,
        r: *const xcb_glx_get_color_table_parameteriv_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_color_table_parameteriv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_color_table_parameteriv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_color_table_parameteriv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_color_table_parameteriv_data)
    }

    pub unsafe fn xcb_glx_get_color_table_parameteriv_data_length(
        &self,
        r: *const xcb_glx_get_color_table_parameteriv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_color_table_parameteriv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_color_table_parameteriv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_color_table_parameteriv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_color_table_parameteriv_data_length)
    }

    pub unsafe fn xcb_glx_get_color_table_parameteriv_data_end(
        &self,
        r: *const xcb_glx_get_color_table_parameteriv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_color_table_parameteriv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_color_table_parameteriv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_color_table_parameteriv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_color_table_parameteriv_data_end)
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
     * xcb_glx_get_color_table_parameteriv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_color_table_parameteriv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_color_table_parameteriv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_color_table_parameteriv_reply_t {
        sym!(self, xcb_glx_get_color_table_parameteriv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_color_table_parameteriv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_color_table_parameteriv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_color_table_parameteriv_reply)
    }

    pub unsafe fn xcb_glx_get_convolution_filter_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_convolution_filter_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_convolution_filter_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_convolution_filter_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_convolution_filter_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_glx_get_convolution_filter` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_convolution_filter(&self) -> bool {
        has_sym!(self, xcb_glx_get_convolution_filter)
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

    /// Returns `true` iff the symbol `xcb_glx_get_convolution_filter_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_convolution_filter_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_convolution_filter_unchecked)
    }

    pub unsafe fn xcb_glx_get_convolution_filter_data(
        &self,
        r: *const xcb_glx_get_convolution_filter_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_glx_get_convolution_filter_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_convolution_filter_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_convolution_filter_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_convolution_filter_data)
    }

    pub unsafe fn xcb_glx_get_convolution_filter_data_length(
        &self,
        r: *const xcb_glx_get_convolution_filter_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_convolution_filter_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_convolution_filter_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_convolution_filter_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_convolution_filter_data_length)
    }

    pub unsafe fn xcb_glx_get_convolution_filter_data_end(
        &self,
        r: *const xcb_glx_get_convolution_filter_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_convolution_filter_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_convolution_filter_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_convolution_filter_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_convolution_filter_data_end)
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
     * xcb_glx_get_convolution_filter_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_convolution_filter_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_convolution_filter_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_convolution_filter_reply_t {
        sym!(self, xcb_glx_get_convolution_filter_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_convolution_filter_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_convolution_filter_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_convolution_filter_reply)
    }

    pub unsafe fn xcb_glx_get_convolution_parameterfv_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_glx_get_convolution_parameterfv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_convolution_parameterfv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_convolution_parameterfv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_convolution_parameterfv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_convolution_parameterfv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_convolution_parameterfv_cookie_t {
        sym!(self, xcb_glx_get_convolution_parameterfv)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_convolution_parameterfv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_convolution_parameterfv(&self) -> bool {
        has_sym!(self, xcb_glx_get_convolution_parameterfv)
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
    pub unsafe fn xcb_glx_get_convolution_parameterfv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_convolution_parameterfv_cookie_t {
        sym!(self, xcb_glx_get_convolution_parameterfv_unchecked)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_convolution_parameterfv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_convolution_parameterfv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_convolution_parameterfv_unchecked)
    }

    pub unsafe fn xcb_glx_get_convolution_parameterfv_data(
        &self,
        r: *const xcb_glx_get_convolution_parameterfv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_convolution_parameterfv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_convolution_parameterfv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_convolution_parameterfv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_convolution_parameterfv_data)
    }

    pub unsafe fn xcb_glx_get_convolution_parameterfv_data_length(
        &self,
        r: *const xcb_glx_get_convolution_parameterfv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_convolution_parameterfv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_convolution_parameterfv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_convolution_parameterfv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_convolution_parameterfv_data_length)
    }

    pub unsafe fn xcb_glx_get_convolution_parameterfv_data_end(
        &self,
        r: *const xcb_glx_get_convolution_parameterfv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_convolution_parameterfv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_convolution_parameterfv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_convolution_parameterfv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_convolution_parameterfv_data_end)
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
     * xcb_glx_get_convolution_parameterfv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_convolution_parameterfv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_convolution_parameterfv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_convolution_parameterfv_reply_t {
        sym!(self, xcb_glx_get_convolution_parameterfv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_convolution_parameterfv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_convolution_parameterfv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_convolution_parameterfv_reply)
    }

    pub unsafe fn xcb_glx_get_convolution_parameteriv_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_glx_get_convolution_parameteriv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_convolution_parameteriv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_convolution_parameteriv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_convolution_parameteriv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_convolution_parameteriv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_convolution_parameteriv_cookie_t {
        sym!(self, xcb_glx_get_convolution_parameteriv)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_convolution_parameteriv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_convolution_parameteriv(&self) -> bool {
        has_sym!(self, xcb_glx_get_convolution_parameteriv)
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
    pub unsafe fn xcb_glx_get_convolution_parameteriv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_convolution_parameteriv_cookie_t {
        sym!(self, xcb_glx_get_convolution_parameteriv_unchecked)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_convolution_parameteriv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_convolution_parameteriv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_convolution_parameteriv_unchecked)
    }

    pub unsafe fn xcb_glx_get_convolution_parameteriv_data(
        &self,
        r: *const xcb_glx_get_convolution_parameteriv_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_convolution_parameteriv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_convolution_parameteriv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_convolution_parameteriv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_convolution_parameteriv_data)
    }

    pub unsafe fn xcb_glx_get_convolution_parameteriv_data_length(
        &self,
        r: *const xcb_glx_get_convolution_parameteriv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_convolution_parameteriv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_convolution_parameteriv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_convolution_parameteriv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_convolution_parameteriv_data_length)
    }

    pub unsafe fn xcb_glx_get_convolution_parameteriv_data_end(
        &self,
        r: *const xcb_glx_get_convolution_parameteriv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_convolution_parameteriv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_convolution_parameteriv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_convolution_parameteriv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_convolution_parameteriv_data_end)
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
     * xcb_glx_get_convolution_parameteriv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_convolution_parameteriv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_convolution_parameteriv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_convolution_parameteriv_reply_t {
        sym!(self, xcb_glx_get_convolution_parameteriv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_convolution_parameteriv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_convolution_parameteriv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_convolution_parameteriv_reply)
    }

    pub unsafe fn xcb_glx_get_separable_filter_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_separable_filter_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_separable_filter_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_separable_filter_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_separable_filter_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_glx_get_separable_filter` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_separable_filter(&self) -> bool {
        has_sym!(self, xcb_glx_get_separable_filter)
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

    /// Returns `true` iff the symbol `xcb_glx_get_separable_filter_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_separable_filter_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_separable_filter_unchecked)
    }

    pub unsafe fn xcb_glx_get_separable_filter_rows_and_cols(
        &self,
        r: *const xcb_glx_get_separable_filter_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_glx_get_separable_filter_rows_and_cols)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_separable_filter_rows_and_cols` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_separable_filter_rows_and_cols(&self) -> bool {
        has_sym!(self, xcb_glx_get_separable_filter_rows_and_cols)
    }

    pub unsafe fn xcb_glx_get_separable_filter_rows_and_cols_length(
        &self,
        r: *const xcb_glx_get_separable_filter_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_separable_filter_rows_and_cols_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_separable_filter_rows_and_cols_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_separable_filter_rows_and_cols_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_separable_filter_rows_and_cols_length)
    }

    pub unsafe fn xcb_glx_get_separable_filter_rows_and_cols_end(
        &self,
        r: *const xcb_glx_get_separable_filter_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_separable_filter_rows_and_cols_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_separable_filter_rows_and_cols_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_separable_filter_rows_and_cols_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_separable_filter_rows_and_cols_end)
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
     * xcb_glx_get_separable_filter_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_separable_filter_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_separable_filter_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_separable_filter_reply_t {
        sym!(self, xcb_glx_get_separable_filter_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_separable_filter_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_separable_filter_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_separable_filter_reply)
    }

    pub unsafe fn xcb_glx_get_histogram_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_histogram_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_histogram_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_histogram_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_histogram_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_glx_get_histogram` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_histogram(&self) -> bool {
        has_sym!(self, xcb_glx_get_histogram)
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

    /// Returns `true` iff the symbol `xcb_glx_get_histogram_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_histogram_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_histogram_unchecked)
    }

    pub unsafe fn xcb_glx_get_histogram_data(
        &self,
        r: *const xcb_glx_get_histogram_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_glx_get_histogram_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_histogram_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_histogram_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_histogram_data)
    }

    pub unsafe fn xcb_glx_get_histogram_data_length(
        &self,
        r: *const xcb_glx_get_histogram_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_histogram_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_histogram_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_histogram_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_histogram_data_length)
    }

    pub unsafe fn xcb_glx_get_histogram_data_end(
        &self,
        r: *const xcb_glx_get_histogram_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_histogram_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_histogram_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_histogram_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_histogram_data_end)
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
     * xcb_glx_get_histogram_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_histogram_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_histogram_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_histogram_reply_t {
        sym!(self, xcb_glx_get_histogram_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_histogram_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_histogram_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_histogram_reply)
    }

    pub unsafe fn xcb_glx_get_histogram_parameterfv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_histogram_parameterfv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_histogram_parameterfv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_histogram_parameterfv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_histogram_parameterfv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_histogram_parameterfv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_histogram_parameterfv_cookie_t {
        sym!(self, xcb_glx_get_histogram_parameterfv)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_histogram_parameterfv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_histogram_parameterfv(&self) -> bool {
        has_sym!(self, xcb_glx_get_histogram_parameterfv)
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
    pub unsafe fn xcb_glx_get_histogram_parameterfv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_histogram_parameterfv_cookie_t {
        sym!(self, xcb_glx_get_histogram_parameterfv_unchecked)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_histogram_parameterfv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_histogram_parameterfv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_histogram_parameterfv_unchecked)
    }

    pub unsafe fn xcb_glx_get_histogram_parameterfv_data(
        &self,
        r: *const xcb_glx_get_histogram_parameterfv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_histogram_parameterfv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_histogram_parameterfv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_histogram_parameterfv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_histogram_parameterfv_data)
    }

    pub unsafe fn xcb_glx_get_histogram_parameterfv_data_length(
        &self,
        r: *const xcb_glx_get_histogram_parameterfv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_histogram_parameterfv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_histogram_parameterfv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_histogram_parameterfv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_histogram_parameterfv_data_length)
    }

    pub unsafe fn xcb_glx_get_histogram_parameterfv_data_end(
        &self,
        r: *const xcb_glx_get_histogram_parameterfv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_histogram_parameterfv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_histogram_parameterfv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_histogram_parameterfv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_histogram_parameterfv_data_end)
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
     * xcb_glx_get_histogram_parameterfv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_histogram_parameterfv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_histogram_parameterfv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_histogram_parameterfv_reply_t {
        sym!(self, xcb_glx_get_histogram_parameterfv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_histogram_parameterfv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_histogram_parameterfv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_histogram_parameterfv_reply)
    }

    pub unsafe fn xcb_glx_get_histogram_parameteriv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_histogram_parameteriv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_histogram_parameteriv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_histogram_parameteriv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_histogram_parameteriv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_histogram_parameteriv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_histogram_parameteriv_cookie_t {
        sym!(self, xcb_glx_get_histogram_parameteriv)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_histogram_parameteriv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_histogram_parameteriv(&self) -> bool {
        has_sym!(self, xcb_glx_get_histogram_parameteriv)
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
    pub unsafe fn xcb_glx_get_histogram_parameteriv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_histogram_parameteriv_cookie_t {
        sym!(self, xcb_glx_get_histogram_parameteriv_unchecked)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_histogram_parameteriv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_histogram_parameteriv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_histogram_parameteriv_unchecked)
    }

    pub unsafe fn xcb_glx_get_histogram_parameteriv_data(
        &self,
        r: *const xcb_glx_get_histogram_parameteriv_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_histogram_parameteriv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_histogram_parameteriv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_histogram_parameteriv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_histogram_parameteriv_data)
    }

    pub unsafe fn xcb_glx_get_histogram_parameteriv_data_length(
        &self,
        r: *const xcb_glx_get_histogram_parameteriv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_histogram_parameteriv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_histogram_parameteriv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_histogram_parameteriv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_histogram_parameteriv_data_length)
    }

    pub unsafe fn xcb_glx_get_histogram_parameteriv_data_end(
        &self,
        r: *const xcb_glx_get_histogram_parameteriv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_histogram_parameteriv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_histogram_parameteriv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_histogram_parameteriv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_histogram_parameteriv_data_end)
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
     * xcb_glx_get_histogram_parameteriv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_histogram_parameteriv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_histogram_parameteriv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_histogram_parameteriv_reply_t {
        sym!(self, xcb_glx_get_histogram_parameteriv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_histogram_parameteriv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_histogram_parameteriv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_histogram_parameteriv_reply)
    }

    pub unsafe fn xcb_glx_get_minmax_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_minmax_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_minmax_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_minmax_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_minmax_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /// Returns `true` iff the symbol `xcb_glx_get_minmax` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_minmax(&self) -> bool {
        has_sym!(self, xcb_glx_get_minmax)
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

    /// Returns `true` iff the symbol `xcb_glx_get_minmax_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_minmax_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_minmax_unchecked)
    }

    pub unsafe fn xcb_glx_get_minmax_data(&self, r: *const xcb_glx_get_minmax_reply_t) -> *mut u8 {
        sym!(self, xcb_glx_get_minmax_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_minmax_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_minmax_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_minmax_data)
    }

    pub unsafe fn xcb_glx_get_minmax_data_length(
        &self,
        r: *const xcb_glx_get_minmax_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_minmax_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_minmax_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_minmax_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_minmax_data_length)
    }

    pub unsafe fn xcb_glx_get_minmax_data_end(
        &self,
        r: *const xcb_glx_get_minmax_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_minmax_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_minmax_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_minmax_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_minmax_data_end)
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
     * xcb_glx_get_minmax_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_minmax_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_minmax_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_minmax_reply_t {
        sym!(self, xcb_glx_get_minmax_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_minmax_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_minmax_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_minmax_reply)
    }

    pub unsafe fn xcb_glx_get_minmax_parameterfv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_minmax_parameterfv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_minmax_parameterfv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_minmax_parameterfv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_minmax_parameterfv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_minmax_parameterfv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_minmax_parameterfv_cookie_t {
        sym!(self, xcb_glx_get_minmax_parameterfv)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_minmax_parameterfv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_minmax_parameterfv(&self) -> bool {
        has_sym!(self, xcb_glx_get_minmax_parameterfv)
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
    pub unsafe fn xcb_glx_get_minmax_parameterfv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_minmax_parameterfv_cookie_t {
        sym!(self, xcb_glx_get_minmax_parameterfv_unchecked)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_minmax_parameterfv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_minmax_parameterfv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_minmax_parameterfv_unchecked)
    }

    pub unsafe fn xcb_glx_get_minmax_parameterfv_data(
        &self,
        r: *const xcb_glx_get_minmax_parameterfv_reply_t,
    ) -> *mut xcb_glx_float32_t {
        sym!(self, xcb_glx_get_minmax_parameterfv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_minmax_parameterfv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_minmax_parameterfv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_minmax_parameterfv_data)
    }

    pub unsafe fn xcb_glx_get_minmax_parameterfv_data_length(
        &self,
        r: *const xcb_glx_get_minmax_parameterfv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_minmax_parameterfv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_minmax_parameterfv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_minmax_parameterfv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_minmax_parameterfv_data_length)
    }

    pub unsafe fn xcb_glx_get_minmax_parameterfv_data_end(
        &self,
        r: *const xcb_glx_get_minmax_parameterfv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_minmax_parameterfv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_minmax_parameterfv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_minmax_parameterfv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_minmax_parameterfv_data_end)
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
     * xcb_glx_get_minmax_parameterfv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_minmax_parameterfv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_minmax_parameterfv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_minmax_parameterfv_reply_t {
        sym!(self, xcb_glx_get_minmax_parameterfv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_minmax_parameterfv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_minmax_parameterfv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_minmax_parameterfv_reply)
    }

    pub unsafe fn xcb_glx_get_minmax_parameteriv_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_minmax_parameteriv_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_minmax_parameteriv_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_minmax_parameteriv_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_minmax_parameteriv_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_minmax_parameteriv(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_minmax_parameteriv_cookie_t {
        sym!(self, xcb_glx_get_minmax_parameteriv)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_minmax_parameteriv` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_minmax_parameteriv(&self) -> bool {
        has_sym!(self, xcb_glx_get_minmax_parameteriv)
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
    pub unsafe fn xcb_glx_get_minmax_parameteriv_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_minmax_parameteriv_cookie_t {
        sym!(self, xcb_glx_get_minmax_parameteriv_unchecked)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_minmax_parameteriv_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_minmax_parameteriv_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_minmax_parameteriv_unchecked)
    }

    pub unsafe fn xcb_glx_get_minmax_parameteriv_data(
        &self,
        r: *const xcb_glx_get_minmax_parameteriv_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_minmax_parameteriv_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_minmax_parameteriv_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_minmax_parameteriv_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_minmax_parameteriv_data)
    }

    pub unsafe fn xcb_glx_get_minmax_parameteriv_data_length(
        &self,
        r: *const xcb_glx_get_minmax_parameteriv_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_minmax_parameteriv_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_minmax_parameteriv_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_minmax_parameteriv_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_minmax_parameteriv_data_length)
    }

    pub unsafe fn xcb_glx_get_minmax_parameteriv_data_end(
        &self,
        r: *const xcb_glx_get_minmax_parameteriv_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_minmax_parameteriv_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_minmax_parameteriv_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_minmax_parameteriv_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_minmax_parameteriv_data_end)
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
     * xcb_glx_get_minmax_parameteriv_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_minmax_parameteriv_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_minmax_parameteriv_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_minmax_parameteriv_reply_t {
        sym!(self, xcb_glx_get_minmax_parameteriv_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_minmax_parameteriv_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_minmax_parameteriv_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_minmax_parameteriv_reply)
    }

    pub unsafe fn xcb_glx_get_compressed_tex_image_arb_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_glx_get_compressed_tex_image_arb_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_compressed_tex_image_arb_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_compressed_tex_image_arb_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_compressed_tex_image_arb_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_compressed_tex_image_arb(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        level: i32,
    ) -> xcb_glx_get_compressed_tex_image_arb_cookie_t {
        sym!(self, xcb_glx_get_compressed_tex_image_arb)(c, context_tag, target, level)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_compressed_tex_image_arb` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_compressed_tex_image_arb(&self) -> bool {
        has_sym!(self, xcb_glx_get_compressed_tex_image_arb)
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
    pub unsafe fn xcb_glx_get_compressed_tex_image_arb_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        level: i32,
    ) -> xcb_glx_get_compressed_tex_image_arb_cookie_t {
        sym!(self, xcb_glx_get_compressed_tex_image_arb_unchecked)(c, context_tag, target, level)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_compressed_tex_image_arb_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_compressed_tex_image_arb_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_compressed_tex_image_arb_unchecked)
    }

    pub unsafe fn xcb_glx_get_compressed_tex_image_arb_data(
        &self,
        r: *const xcb_glx_get_compressed_tex_image_arb_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_glx_get_compressed_tex_image_arb_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_compressed_tex_image_arb_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_compressed_tex_image_arb_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_compressed_tex_image_arb_data)
    }

    pub unsafe fn xcb_glx_get_compressed_tex_image_arb_data_length(
        &self,
        r: *const xcb_glx_get_compressed_tex_image_arb_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_compressed_tex_image_arb_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_compressed_tex_image_arb_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_compressed_tex_image_arb_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_compressed_tex_image_arb_data_length)
    }

    pub unsafe fn xcb_glx_get_compressed_tex_image_arb_data_end(
        &self,
        r: *const xcb_glx_get_compressed_tex_image_arb_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_compressed_tex_image_arb_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_compressed_tex_image_arb_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_compressed_tex_image_arb_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_compressed_tex_image_arb_data_end)
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
     * xcb_glx_get_compressed_tex_image_arb_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_compressed_tex_image_arb_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_compressed_tex_image_arb_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_compressed_tex_image_arb_reply_t {
        sym!(self, xcb_glx_get_compressed_tex_image_arb_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_compressed_tex_image_arb_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_compressed_tex_image_arb_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_compressed_tex_image_arb_reply)
    }

    pub unsafe fn xcb_glx_delete_queries_arb_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_delete_queries_arb_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_delete_queries_arb_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_delete_queries_arb_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_delete_queries_arb_sizeof)
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
    pub unsafe fn xcb_glx_delete_queries_arb_checked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        n: i32,
        ids: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_delete_queries_arb_checked)(c, context_tag, n, ids)
    }

    /// Returns `true` iff the symbol `xcb_glx_delete_queries_arb_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_delete_queries_arb_checked(&self) -> bool {
        has_sym!(self, xcb_glx_delete_queries_arb_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_delete_queries_arb(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        n: i32,
        ids: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_glx_delete_queries_arb)(c, context_tag, n, ids)
    }

    /// Returns `true` iff the symbol `xcb_glx_delete_queries_arb` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_delete_queries_arb(&self) -> bool {
        has_sym!(self, xcb_glx_delete_queries_arb)
    }

    pub unsafe fn xcb_glx_delete_queries_arb_ids(
        &self,
        r: *const xcb_glx_delete_queries_arb_request_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_delete_queries_arb_ids)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_delete_queries_arb_ids` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_delete_queries_arb_ids(&self) -> bool {
        has_sym!(self, xcb_glx_delete_queries_arb_ids)
    }

    pub unsafe fn xcb_glx_delete_queries_arb_ids_length(
        &self,
        r: *const xcb_glx_delete_queries_arb_request_t,
    ) -> c_int {
        sym!(self, xcb_glx_delete_queries_arb_ids_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_delete_queries_arb_ids_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_delete_queries_arb_ids_length(&self) -> bool {
        has_sym!(self, xcb_glx_delete_queries_arb_ids_length)
    }

    pub unsafe fn xcb_glx_delete_queries_arb_ids_end(
        &self,
        r: *const xcb_glx_delete_queries_arb_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_delete_queries_arb_ids_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_delete_queries_arb_ids_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_delete_queries_arb_ids_end(&self) -> bool {
        has_sym!(self, xcb_glx_delete_queries_arb_ids_end)
    }

    pub unsafe fn xcb_glx_gen_queries_arb_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_gen_queries_arb_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_gen_queries_arb_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_gen_queries_arb_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_gen_queries_arb_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_gen_queries_arb(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        n: i32,
    ) -> xcb_glx_gen_queries_arb_cookie_t {
        sym!(self, xcb_glx_gen_queries_arb)(c, context_tag, n)
    }

    /// Returns `true` iff the symbol `xcb_glx_gen_queries_arb` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_gen_queries_arb(&self) -> bool {
        has_sym!(self, xcb_glx_gen_queries_arb)
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
    pub unsafe fn xcb_glx_gen_queries_arb_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        n: i32,
    ) -> xcb_glx_gen_queries_arb_cookie_t {
        sym!(self, xcb_glx_gen_queries_arb_unchecked)(c, context_tag, n)
    }

    /// Returns `true` iff the symbol `xcb_glx_gen_queries_arb_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_gen_queries_arb_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_gen_queries_arb_unchecked)
    }

    pub unsafe fn xcb_glx_gen_queries_arb_data(
        &self,
        r: *const xcb_glx_gen_queries_arb_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_gen_queries_arb_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_gen_queries_arb_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_gen_queries_arb_data(&self) -> bool {
        has_sym!(self, xcb_glx_gen_queries_arb_data)
    }

    pub unsafe fn xcb_glx_gen_queries_arb_data_length(
        &self,
        r: *const xcb_glx_gen_queries_arb_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_gen_queries_arb_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_gen_queries_arb_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_gen_queries_arb_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_gen_queries_arb_data_length)
    }

    pub unsafe fn xcb_glx_gen_queries_arb_data_end(
        &self,
        r: *const xcb_glx_gen_queries_arb_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_gen_queries_arb_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_gen_queries_arb_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_gen_queries_arb_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_gen_queries_arb_data_end)
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
     * xcb_glx_gen_queries_arb_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_gen_queries_arb_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_gen_queries_arb_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_gen_queries_arb_reply_t {
        sym!(self, xcb_glx_gen_queries_arb_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_gen_queries_arb_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_gen_queries_arb_reply(&self) -> bool {
        has_sym!(self, xcb_glx_gen_queries_arb_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_is_query_arb(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        id: u32,
    ) -> xcb_glx_is_query_arb_cookie_t {
        sym!(self, xcb_glx_is_query_arb)(c, context_tag, id)
    }

    /// Returns `true` iff the symbol `xcb_glx_is_query_arb` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_is_query_arb(&self) -> bool {
        has_sym!(self, xcb_glx_is_query_arb)
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
    pub unsafe fn xcb_glx_is_query_arb_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        id: u32,
    ) -> xcb_glx_is_query_arb_cookie_t {
        sym!(self, xcb_glx_is_query_arb_unchecked)(c, context_tag, id)
    }

    /// Returns `true` iff the symbol `xcb_glx_is_query_arb_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_is_query_arb_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_is_query_arb_unchecked)
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
     * xcb_glx_is_query_arb_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_is_query_arb_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_is_query_arb_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_is_query_arb_reply_t {
        sym!(self, xcb_glx_is_query_arb_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_is_query_arb_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_is_query_arb_reply(&self) -> bool {
        has_sym!(self, xcb_glx_is_query_arb_reply)
    }

    pub unsafe fn xcb_glx_get_queryiv_arb_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_queryiv_arb_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_queryiv_arb_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_queryiv_arb_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_queryiv_arb_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_queryiv_arb(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_queryiv_arb_cookie_t {
        sym!(self, xcb_glx_get_queryiv_arb)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_queryiv_arb` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_queryiv_arb(&self) -> bool {
        has_sym!(self, xcb_glx_get_queryiv_arb)
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
    pub unsafe fn xcb_glx_get_queryiv_arb_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        target: u32,
        pname: u32,
    ) -> xcb_glx_get_queryiv_arb_cookie_t {
        sym!(self, xcb_glx_get_queryiv_arb_unchecked)(c, context_tag, target, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_queryiv_arb_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_queryiv_arb_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_queryiv_arb_unchecked)
    }

    pub unsafe fn xcb_glx_get_queryiv_arb_data(
        &self,
        r: *const xcb_glx_get_queryiv_arb_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_queryiv_arb_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_queryiv_arb_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_queryiv_arb_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_queryiv_arb_data)
    }

    pub unsafe fn xcb_glx_get_queryiv_arb_data_length(
        &self,
        r: *const xcb_glx_get_queryiv_arb_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_queryiv_arb_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_queryiv_arb_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_queryiv_arb_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_queryiv_arb_data_length)
    }

    pub unsafe fn xcb_glx_get_queryiv_arb_data_end(
        &self,
        r: *const xcb_glx_get_queryiv_arb_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_queryiv_arb_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_queryiv_arb_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_queryiv_arb_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_queryiv_arb_data_end)
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
     * xcb_glx_get_queryiv_arb_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_queryiv_arb_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_queryiv_arb_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_queryiv_arb_reply_t {
        sym!(self, xcb_glx_get_queryiv_arb_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_queryiv_arb_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_queryiv_arb_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_queryiv_arb_reply)
    }

    pub unsafe fn xcb_glx_get_query_objectiv_arb_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_query_objectiv_arb_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_query_objectiv_arb_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_query_objectiv_arb_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_query_objectiv_arb_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_query_objectiv_arb(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        id: u32,
        pname: u32,
    ) -> xcb_glx_get_query_objectiv_arb_cookie_t {
        sym!(self, xcb_glx_get_query_objectiv_arb)(c, context_tag, id, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_query_objectiv_arb` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_query_objectiv_arb(&self) -> bool {
        has_sym!(self, xcb_glx_get_query_objectiv_arb)
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
    pub unsafe fn xcb_glx_get_query_objectiv_arb_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        id: u32,
        pname: u32,
    ) -> xcb_glx_get_query_objectiv_arb_cookie_t {
        sym!(self, xcb_glx_get_query_objectiv_arb_unchecked)(c, context_tag, id, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_query_objectiv_arb_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_query_objectiv_arb_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_query_objectiv_arb_unchecked)
    }

    pub unsafe fn xcb_glx_get_query_objectiv_arb_data(
        &self,
        r: *const xcb_glx_get_query_objectiv_arb_reply_t,
    ) -> *mut i32 {
        sym!(self, xcb_glx_get_query_objectiv_arb_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_query_objectiv_arb_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_query_objectiv_arb_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_query_objectiv_arb_data)
    }

    pub unsafe fn xcb_glx_get_query_objectiv_arb_data_length(
        &self,
        r: *const xcb_glx_get_query_objectiv_arb_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_query_objectiv_arb_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_query_objectiv_arb_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_query_objectiv_arb_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_query_objectiv_arb_data_length)
    }

    pub unsafe fn xcb_glx_get_query_objectiv_arb_data_end(
        &self,
        r: *const xcb_glx_get_query_objectiv_arb_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_query_objectiv_arb_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_query_objectiv_arb_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_query_objectiv_arb_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_query_objectiv_arb_data_end)
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
     * xcb_glx_get_query_objectiv_arb_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_query_objectiv_arb_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_query_objectiv_arb_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_query_objectiv_arb_reply_t {
        sym!(self, xcb_glx_get_query_objectiv_arb_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_query_objectiv_arb_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_query_objectiv_arb_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_query_objectiv_arb_reply)
    }

    pub unsafe fn xcb_glx_get_query_objectuiv_arb_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_glx_get_query_objectuiv_arb_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_query_objectuiv_arb_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_query_objectuiv_arb_sizeof(&self) -> bool {
        has_sym!(self, xcb_glx_get_query_objectuiv_arb_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_glx_get_query_objectuiv_arb(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        id: u32,
        pname: u32,
    ) -> xcb_glx_get_query_objectuiv_arb_cookie_t {
        sym!(self, xcb_glx_get_query_objectuiv_arb)(c, context_tag, id, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_query_objectuiv_arb` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_query_objectuiv_arb(&self) -> bool {
        has_sym!(self, xcb_glx_get_query_objectuiv_arb)
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
    pub unsafe fn xcb_glx_get_query_objectuiv_arb_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context_tag: xcb_glx_context_tag_t,
        id: u32,
        pname: u32,
    ) -> xcb_glx_get_query_objectuiv_arb_cookie_t {
        sym!(self, xcb_glx_get_query_objectuiv_arb_unchecked)(c, context_tag, id, pname)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_query_objectuiv_arb_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_query_objectuiv_arb_unchecked(&self) -> bool {
        has_sym!(self, xcb_glx_get_query_objectuiv_arb_unchecked)
    }

    pub unsafe fn xcb_glx_get_query_objectuiv_arb_data(
        &self,
        r: *const xcb_glx_get_query_objectuiv_arb_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_glx_get_query_objectuiv_arb_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_query_objectuiv_arb_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_query_objectuiv_arb_data(&self) -> bool {
        has_sym!(self, xcb_glx_get_query_objectuiv_arb_data)
    }

    pub unsafe fn xcb_glx_get_query_objectuiv_arb_data_length(
        &self,
        r: *const xcb_glx_get_query_objectuiv_arb_reply_t,
    ) -> c_int {
        sym!(self, xcb_glx_get_query_objectuiv_arb_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_query_objectuiv_arb_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_query_objectuiv_arb_data_length(&self) -> bool {
        has_sym!(self, xcb_glx_get_query_objectuiv_arb_data_length)
    }

    pub unsafe fn xcb_glx_get_query_objectuiv_arb_data_end(
        &self,
        r: *const xcb_glx_get_query_objectuiv_arb_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_glx_get_query_objectuiv_arb_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_query_objectuiv_arb_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_query_objectuiv_arb_data_end(&self) -> bool {
        has_sym!(self, xcb_glx_get_query_objectuiv_arb_data_end)
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
     * xcb_glx_get_query_objectuiv_arb_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_glx_get_query_objectuiv_arb_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_glx_get_query_objectuiv_arb_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_glx_get_query_objectuiv_arb_reply_t {
        sym!(self, xcb_glx_get_query_objectuiv_arb_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_glx_get_query_objectuiv_arb_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_glx_get_query_objectuiv_arb_reply(&self) -> bool {
        has_sym!(self, xcb_glx_get_query_objectuiv_arb_reply)
    }
}

#[cfg(feature = "xcb_glx")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbGlx::load().unwrap() };
        assert!(lib.has_xcb_glx_id());
        assert!(lib.has_xcb_glx_pixmap_next());
        assert!(lib.has_xcb_glx_pixmap_end());
        assert!(lib.has_xcb_glx_context_next());
        assert!(lib.has_xcb_glx_context_end());
        assert!(lib.has_xcb_glx_pbuffer_next());
        assert!(lib.has_xcb_glx_pbuffer_end());
        assert!(lib.has_xcb_glx_window_next());
        assert!(lib.has_xcb_glx_window_end());
        assert!(lib.has_xcb_glx_fbconfig_next());
        assert!(lib.has_xcb_glx_fbconfig_end());
        assert!(lib.has_xcb_glx_drawable_next());
        assert!(lib.has_xcb_glx_drawable_end());
        assert!(lib.has_xcb_glx_float32_next());
        assert!(lib.has_xcb_glx_float32_end());
        assert!(lib.has_xcb_glx_float64_next());
        assert!(lib.has_xcb_glx_float64_end());
        assert!(lib.has_xcb_glx_bool32_next());
        assert!(lib.has_xcb_glx_bool32_end());
        assert!(lib.has_xcb_glx_context_tag_next());
        assert!(lib.has_xcb_glx_context_tag_end());
        assert!(lib.has_xcb_glx_render_sizeof());
        assert!(lib.has_xcb_glx_render_checked());
        assert!(lib.has_xcb_glx_render());
        assert!(lib.has_xcb_glx_render_data());
        assert!(lib.has_xcb_glx_render_data_length());
        assert!(lib.has_xcb_glx_render_data_end());
        assert!(lib.has_xcb_glx_render_large_sizeof());
        assert!(lib.has_xcb_glx_render_large_checked());
        assert!(lib.has_xcb_glx_render_large());
        assert!(lib.has_xcb_glx_render_large_data());
        assert!(lib.has_xcb_glx_render_large_data_length());
        assert!(lib.has_xcb_glx_render_large_data_end());
        assert!(lib.has_xcb_glx_create_context_checked());
        assert!(lib.has_xcb_glx_create_context());
        assert!(lib.has_xcb_glx_destroy_context_checked());
        assert!(lib.has_xcb_glx_destroy_context());
        assert!(lib.has_xcb_glx_make_current());
        assert!(lib.has_xcb_glx_make_current_unchecked());
        assert!(lib.has_xcb_glx_make_current_reply());
        assert!(lib.has_xcb_glx_is_direct());
        assert!(lib.has_xcb_glx_is_direct_unchecked());
        assert!(lib.has_xcb_glx_is_direct_reply());
        assert!(lib.has_xcb_glx_query_version());
        assert!(lib.has_xcb_glx_query_version_unchecked());
        assert!(lib.has_xcb_glx_query_version_reply());
        assert!(lib.has_xcb_glx_wait_gl_checked());
        assert!(lib.has_xcb_glx_wait_gl());
        assert!(lib.has_xcb_glx_wait_x_checked());
        assert!(lib.has_xcb_glx_wait_x());
        assert!(lib.has_xcb_glx_copy_context_checked());
        assert!(lib.has_xcb_glx_copy_context());
        assert!(lib.has_xcb_glx_swap_buffers_checked());
        assert!(lib.has_xcb_glx_swap_buffers());
        assert!(lib.has_xcb_glx_use_x_font_checked());
        assert!(lib.has_xcb_glx_use_x_font());
        assert!(lib.has_xcb_glx_create_glx_pixmap_checked());
        assert!(lib.has_xcb_glx_create_glx_pixmap());
        assert!(lib.has_xcb_glx_get_visual_configs_sizeof());
        assert!(lib.has_xcb_glx_get_visual_configs());
        assert!(lib.has_xcb_glx_get_visual_configs_unchecked());
        assert!(lib.has_xcb_glx_get_visual_configs_property_list());
        assert!(lib.has_xcb_glx_get_visual_configs_property_list_length());
        assert!(lib.has_xcb_glx_get_visual_configs_property_list_end());
        assert!(lib.has_xcb_glx_get_visual_configs_reply());
        assert!(lib.has_xcb_glx_destroy_glx_pixmap_checked());
        assert!(lib.has_xcb_glx_destroy_glx_pixmap());
        assert!(lib.has_xcb_glx_vendor_private_sizeof());
        assert!(lib.has_xcb_glx_vendor_private_checked());
        assert!(lib.has_xcb_glx_vendor_private());
        assert!(lib.has_xcb_glx_vendor_private_data());
        assert!(lib.has_xcb_glx_vendor_private_data_length());
        assert!(lib.has_xcb_glx_vendor_private_data_end());
        assert!(lib.has_xcb_glx_vendor_private_with_reply_sizeof());
        assert!(lib.has_xcb_glx_vendor_private_with_reply());
        assert!(lib.has_xcb_glx_vendor_private_with_reply_unchecked());
        assert!(lib.has_xcb_glx_vendor_private_with_reply_data_2());
        assert!(lib.has_xcb_glx_vendor_private_with_reply_data_2_length());
        assert!(lib.has_xcb_glx_vendor_private_with_reply_data_2_end());
        assert!(lib.has_xcb_glx_vendor_private_with_reply_reply());
        assert!(lib.has_xcb_glx_query_extensions_string());
        assert!(lib.has_xcb_glx_query_extensions_string_unchecked());
        assert!(lib.has_xcb_glx_query_extensions_string_reply());
        assert!(lib.has_xcb_glx_query_server_string_sizeof());
        assert!(lib.has_xcb_glx_query_server_string());
        assert!(lib.has_xcb_glx_query_server_string_unchecked());
        assert!(lib.has_xcb_glx_query_server_string_string());
        assert!(lib.has_xcb_glx_query_server_string_string_length());
        assert!(lib.has_xcb_glx_query_server_string_string_end());
        assert!(lib.has_xcb_glx_query_server_string_reply());
        assert!(lib.has_xcb_glx_client_info_sizeof());
        assert!(lib.has_xcb_glx_client_info_checked());
        assert!(lib.has_xcb_glx_client_info());
        assert!(lib.has_xcb_glx_client_info_string());
        assert!(lib.has_xcb_glx_client_info_string_length());
        assert!(lib.has_xcb_glx_client_info_string_end());
        assert!(lib.has_xcb_glx_get_fb_configs_sizeof());
        assert!(lib.has_xcb_glx_get_fb_configs());
        assert!(lib.has_xcb_glx_get_fb_configs_unchecked());
        assert!(lib.has_xcb_glx_get_fb_configs_property_list());
        assert!(lib.has_xcb_glx_get_fb_configs_property_list_length());
        assert!(lib.has_xcb_glx_get_fb_configs_property_list_end());
        assert!(lib.has_xcb_glx_get_fb_configs_reply());
        assert!(lib.has_xcb_glx_create_pixmap_sizeof());
        assert!(lib.has_xcb_glx_create_pixmap_checked());
        assert!(lib.has_xcb_glx_create_pixmap());
        assert!(lib.has_xcb_glx_create_pixmap_attribs());
        assert!(lib.has_xcb_glx_create_pixmap_attribs_length());
        assert!(lib.has_xcb_glx_create_pixmap_attribs_end());
        assert!(lib.has_xcb_glx_destroy_pixmap_checked());
        assert!(lib.has_xcb_glx_destroy_pixmap());
        assert!(lib.has_xcb_glx_create_new_context_checked());
        assert!(lib.has_xcb_glx_create_new_context());
        assert!(lib.has_xcb_glx_query_context_sizeof());
        assert!(lib.has_xcb_glx_query_context());
        assert!(lib.has_xcb_glx_query_context_unchecked());
        assert!(lib.has_xcb_glx_query_context_attribs());
        assert!(lib.has_xcb_glx_query_context_attribs_length());
        assert!(lib.has_xcb_glx_query_context_attribs_end());
        assert!(lib.has_xcb_glx_query_context_reply());
        assert!(lib.has_xcb_glx_make_context_current());
        assert!(lib.has_xcb_glx_make_context_current_unchecked());
        assert!(lib.has_xcb_glx_make_context_current_reply());
        assert!(lib.has_xcb_glx_create_pbuffer_sizeof());
        assert!(lib.has_xcb_glx_create_pbuffer_checked());
        assert!(lib.has_xcb_glx_create_pbuffer());
        assert!(lib.has_xcb_glx_create_pbuffer_attribs());
        assert!(lib.has_xcb_glx_create_pbuffer_attribs_length());
        assert!(lib.has_xcb_glx_create_pbuffer_attribs_end());
        assert!(lib.has_xcb_glx_destroy_pbuffer_checked());
        assert!(lib.has_xcb_glx_destroy_pbuffer());
        assert!(lib.has_xcb_glx_get_drawable_attributes_sizeof());
        assert!(lib.has_xcb_glx_get_drawable_attributes());
        assert!(lib.has_xcb_glx_get_drawable_attributes_unchecked());
        assert!(lib.has_xcb_glx_get_drawable_attributes_attribs());
        assert!(lib.has_xcb_glx_get_drawable_attributes_attribs_length());
        assert!(lib.has_xcb_glx_get_drawable_attributes_attribs_end());
        assert!(lib.has_xcb_glx_get_drawable_attributes_reply());
        assert!(lib.has_xcb_glx_change_drawable_attributes_sizeof());
        assert!(lib.has_xcb_glx_change_drawable_attributes_checked());
        assert!(lib.has_xcb_glx_change_drawable_attributes());
        assert!(lib.has_xcb_glx_change_drawable_attributes_attribs());
        assert!(lib.has_xcb_glx_change_drawable_attributes_attribs_length());
        assert!(lib.has_xcb_glx_change_drawable_attributes_attribs_end());
        assert!(lib.has_xcb_glx_create_window_sizeof());
        assert!(lib.has_xcb_glx_create_window_checked());
        assert!(lib.has_xcb_glx_create_window());
        assert!(lib.has_xcb_glx_create_window_attribs());
        assert!(lib.has_xcb_glx_create_window_attribs_length());
        assert!(lib.has_xcb_glx_create_window_attribs_end());
        assert!(lib.has_xcb_glx_delete_window_checked());
        assert!(lib.has_xcb_glx_delete_window());
        assert!(lib.has_xcb_glx_set_client_info_arb_sizeof());
        assert!(lib.has_xcb_glx_set_client_info_arb_checked());
        assert!(lib.has_xcb_glx_set_client_info_arb());
        assert!(lib.has_xcb_glx_set_client_info_arb_gl_versions());
        assert!(lib.has_xcb_glx_set_client_info_arb_gl_versions_length());
        assert!(lib.has_xcb_glx_set_client_info_arb_gl_versions_end());
        assert!(lib.has_xcb_glx_set_client_info_arb_gl_extension_string());
        assert!(lib.has_xcb_glx_set_client_info_arb_gl_extension_string_length());
        assert!(lib.has_xcb_glx_set_client_info_arb_gl_extension_string_end());
        assert!(lib.has_xcb_glx_set_client_info_arb_glx_extension_string());
        assert!(lib.has_xcb_glx_set_client_info_arb_glx_extension_string_length());
        assert!(lib.has_xcb_glx_set_client_info_arb_glx_extension_string_end());
        assert!(lib.has_xcb_glx_create_context_attribs_arb_sizeof());
        assert!(lib.has_xcb_glx_create_context_attribs_arb_checked());
        assert!(lib.has_xcb_glx_create_context_attribs_arb());
        assert!(lib.has_xcb_glx_create_context_attribs_arb_attribs());
        assert!(lib.has_xcb_glx_create_context_attribs_arb_attribs_length());
        assert!(lib.has_xcb_glx_create_context_attribs_arb_attribs_end());
        assert!(lib.has_xcb_glx_set_client_info_2arb_sizeof());
        assert!(lib.has_xcb_glx_set_client_info_2arb_checked());
        assert!(lib.has_xcb_glx_set_client_info_2arb());
        assert!(lib.has_xcb_glx_set_client_info_2arb_gl_versions());
        assert!(lib.has_xcb_glx_set_client_info_2arb_gl_versions_length());
        assert!(lib.has_xcb_glx_set_client_info_2arb_gl_versions_end());
        assert!(lib.has_xcb_glx_set_client_info_2arb_gl_extension_string());
        assert!(lib.has_xcb_glx_set_client_info_2arb_gl_extension_string_length());
        assert!(lib.has_xcb_glx_set_client_info_2arb_gl_extension_string_end());
        assert!(lib.has_xcb_glx_set_client_info_2arb_glx_extension_string());
        assert!(lib.has_xcb_glx_set_client_info_2arb_glx_extension_string_length());
        assert!(lib.has_xcb_glx_set_client_info_2arb_glx_extension_string_end());
        assert!(lib.has_xcb_glx_new_list_checked());
        assert!(lib.has_xcb_glx_new_list());
        assert!(lib.has_xcb_glx_end_list_checked());
        assert!(lib.has_xcb_glx_end_list());
        assert!(lib.has_xcb_glx_delete_lists_checked());
        assert!(lib.has_xcb_glx_delete_lists());
        assert!(lib.has_xcb_glx_gen_lists());
        assert!(lib.has_xcb_glx_gen_lists_unchecked());
        assert!(lib.has_xcb_glx_gen_lists_reply());
        assert!(lib.has_xcb_glx_feedback_buffer_checked());
        assert!(lib.has_xcb_glx_feedback_buffer());
        assert!(lib.has_xcb_glx_select_buffer_checked());
        assert!(lib.has_xcb_glx_select_buffer());
        assert!(lib.has_xcb_glx_render_mode_sizeof());
        assert!(lib.has_xcb_glx_render_mode());
        assert!(lib.has_xcb_glx_render_mode_unchecked());
        assert!(lib.has_xcb_glx_render_mode_data());
        assert!(lib.has_xcb_glx_render_mode_data_length());
        assert!(lib.has_xcb_glx_render_mode_data_end());
        assert!(lib.has_xcb_glx_render_mode_reply());
        assert!(lib.has_xcb_glx_finish());
        assert!(lib.has_xcb_glx_finish_unchecked());
        assert!(lib.has_xcb_glx_finish_reply());
        assert!(lib.has_xcb_glx_pixel_storef_checked());
        assert!(lib.has_xcb_glx_pixel_storef());
        assert!(lib.has_xcb_glx_pixel_storei_checked());
        assert!(lib.has_xcb_glx_pixel_storei());
        assert!(lib.has_xcb_glx_read_pixels_sizeof());
        assert!(lib.has_xcb_glx_read_pixels());
        assert!(lib.has_xcb_glx_read_pixels_unchecked());
        assert!(lib.has_xcb_glx_read_pixels_data());
        assert!(lib.has_xcb_glx_read_pixels_data_length());
        assert!(lib.has_xcb_glx_read_pixels_data_end());
        assert!(lib.has_xcb_glx_read_pixels_reply());
        assert!(lib.has_xcb_glx_get_booleanv_sizeof());
        assert!(lib.has_xcb_glx_get_booleanv());
        assert!(lib.has_xcb_glx_get_booleanv_unchecked());
        assert!(lib.has_xcb_glx_get_booleanv_data());
        assert!(lib.has_xcb_glx_get_booleanv_data_length());
        assert!(lib.has_xcb_glx_get_booleanv_data_end());
        assert!(lib.has_xcb_glx_get_booleanv_reply());
        assert!(lib.has_xcb_glx_get_clip_plane_sizeof());
        assert!(lib.has_xcb_glx_get_clip_plane());
        assert!(lib.has_xcb_glx_get_clip_plane_unchecked());
        assert!(lib.has_xcb_glx_get_clip_plane_data());
        assert!(lib.has_xcb_glx_get_clip_plane_data_length());
        assert!(lib.has_xcb_glx_get_clip_plane_data_end());
        assert!(lib.has_xcb_glx_get_clip_plane_reply());
        assert!(lib.has_xcb_glx_get_doublev_sizeof());
        assert!(lib.has_xcb_glx_get_doublev());
        assert!(lib.has_xcb_glx_get_doublev_unchecked());
        assert!(lib.has_xcb_glx_get_doublev_data());
        assert!(lib.has_xcb_glx_get_doublev_data_length());
        assert!(lib.has_xcb_glx_get_doublev_data_end());
        assert!(lib.has_xcb_glx_get_doublev_reply());
        assert!(lib.has_xcb_glx_get_error());
        assert!(lib.has_xcb_glx_get_error_unchecked());
        assert!(lib.has_xcb_glx_get_error_reply());
        assert!(lib.has_xcb_glx_get_floatv_sizeof());
        assert!(lib.has_xcb_glx_get_floatv());
        assert!(lib.has_xcb_glx_get_floatv_unchecked());
        assert!(lib.has_xcb_glx_get_floatv_data());
        assert!(lib.has_xcb_glx_get_floatv_data_length());
        assert!(lib.has_xcb_glx_get_floatv_data_end());
        assert!(lib.has_xcb_glx_get_floatv_reply());
        assert!(lib.has_xcb_glx_get_integerv_sizeof());
        assert!(lib.has_xcb_glx_get_integerv());
        assert!(lib.has_xcb_glx_get_integerv_unchecked());
        assert!(lib.has_xcb_glx_get_integerv_data());
        assert!(lib.has_xcb_glx_get_integerv_data_length());
        assert!(lib.has_xcb_glx_get_integerv_data_end());
        assert!(lib.has_xcb_glx_get_integerv_reply());
        assert!(lib.has_xcb_glx_get_lightfv_sizeof());
        assert!(lib.has_xcb_glx_get_lightfv());
        assert!(lib.has_xcb_glx_get_lightfv_unchecked());
        assert!(lib.has_xcb_glx_get_lightfv_data());
        assert!(lib.has_xcb_glx_get_lightfv_data_length());
        assert!(lib.has_xcb_glx_get_lightfv_data_end());
        assert!(lib.has_xcb_glx_get_lightfv_reply());
        assert!(lib.has_xcb_glx_get_lightiv_sizeof());
        assert!(lib.has_xcb_glx_get_lightiv());
        assert!(lib.has_xcb_glx_get_lightiv_unchecked());
        assert!(lib.has_xcb_glx_get_lightiv_data());
        assert!(lib.has_xcb_glx_get_lightiv_data_length());
        assert!(lib.has_xcb_glx_get_lightiv_data_end());
        assert!(lib.has_xcb_glx_get_lightiv_reply());
        assert!(lib.has_xcb_glx_get_mapdv_sizeof());
        assert!(lib.has_xcb_glx_get_mapdv());
        assert!(lib.has_xcb_glx_get_mapdv_unchecked());
        assert!(lib.has_xcb_glx_get_mapdv_data());
        assert!(lib.has_xcb_glx_get_mapdv_data_length());
        assert!(lib.has_xcb_glx_get_mapdv_data_end());
        assert!(lib.has_xcb_glx_get_mapdv_reply());
        assert!(lib.has_xcb_glx_get_mapfv_sizeof());
        assert!(lib.has_xcb_glx_get_mapfv());
        assert!(lib.has_xcb_glx_get_mapfv_unchecked());
        assert!(lib.has_xcb_glx_get_mapfv_data());
        assert!(lib.has_xcb_glx_get_mapfv_data_length());
        assert!(lib.has_xcb_glx_get_mapfv_data_end());
        assert!(lib.has_xcb_glx_get_mapfv_reply());
        assert!(lib.has_xcb_glx_get_mapiv_sizeof());
        assert!(lib.has_xcb_glx_get_mapiv());
        assert!(lib.has_xcb_glx_get_mapiv_unchecked());
        assert!(lib.has_xcb_glx_get_mapiv_data());
        assert!(lib.has_xcb_glx_get_mapiv_data_length());
        assert!(lib.has_xcb_glx_get_mapiv_data_end());
        assert!(lib.has_xcb_glx_get_mapiv_reply());
        assert!(lib.has_xcb_glx_get_materialfv_sizeof());
        assert!(lib.has_xcb_glx_get_materialfv());
        assert!(lib.has_xcb_glx_get_materialfv_unchecked());
        assert!(lib.has_xcb_glx_get_materialfv_data());
        assert!(lib.has_xcb_glx_get_materialfv_data_length());
        assert!(lib.has_xcb_glx_get_materialfv_data_end());
        assert!(lib.has_xcb_glx_get_materialfv_reply());
        assert!(lib.has_xcb_glx_get_materialiv_sizeof());
        assert!(lib.has_xcb_glx_get_materialiv());
        assert!(lib.has_xcb_glx_get_materialiv_unchecked());
        assert!(lib.has_xcb_glx_get_materialiv_data());
        assert!(lib.has_xcb_glx_get_materialiv_data_length());
        assert!(lib.has_xcb_glx_get_materialiv_data_end());
        assert!(lib.has_xcb_glx_get_materialiv_reply());
        assert!(lib.has_xcb_glx_get_pixel_mapfv_sizeof());
        assert!(lib.has_xcb_glx_get_pixel_mapfv());
        assert!(lib.has_xcb_glx_get_pixel_mapfv_unchecked());
        assert!(lib.has_xcb_glx_get_pixel_mapfv_data());
        assert!(lib.has_xcb_glx_get_pixel_mapfv_data_length());
        assert!(lib.has_xcb_glx_get_pixel_mapfv_data_end());
        assert!(lib.has_xcb_glx_get_pixel_mapfv_reply());
        assert!(lib.has_xcb_glx_get_pixel_mapuiv_sizeof());
        assert!(lib.has_xcb_glx_get_pixel_mapuiv());
        assert!(lib.has_xcb_glx_get_pixel_mapuiv_unchecked());
        assert!(lib.has_xcb_glx_get_pixel_mapuiv_data());
        assert!(lib.has_xcb_glx_get_pixel_mapuiv_data_length());
        assert!(lib.has_xcb_glx_get_pixel_mapuiv_data_end());
        assert!(lib.has_xcb_glx_get_pixel_mapuiv_reply());
        assert!(lib.has_xcb_glx_get_pixel_mapusv_sizeof());
        assert!(lib.has_xcb_glx_get_pixel_mapusv());
        assert!(lib.has_xcb_glx_get_pixel_mapusv_unchecked());
        assert!(lib.has_xcb_glx_get_pixel_mapusv_data());
        assert!(lib.has_xcb_glx_get_pixel_mapusv_data_length());
        assert!(lib.has_xcb_glx_get_pixel_mapusv_data_end());
        assert!(lib.has_xcb_glx_get_pixel_mapusv_reply());
        assert!(lib.has_xcb_glx_get_polygon_stipple_sizeof());
        assert!(lib.has_xcb_glx_get_polygon_stipple());
        assert!(lib.has_xcb_glx_get_polygon_stipple_unchecked());
        assert!(lib.has_xcb_glx_get_polygon_stipple_data());
        assert!(lib.has_xcb_glx_get_polygon_stipple_data_length());
        assert!(lib.has_xcb_glx_get_polygon_stipple_data_end());
        assert!(lib.has_xcb_glx_get_polygon_stipple_reply());
        assert!(lib.has_xcb_glx_get_string_sizeof());
        assert!(lib.has_xcb_glx_get_string());
        assert!(lib.has_xcb_glx_get_string_unchecked());
        assert!(lib.has_xcb_glx_get_string_string());
        assert!(lib.has_xcb_glx_get_string_string_length());
        assert!(lib.has_xcb_glx_get_string_string_end());
        assert!(lib.has_xcb_glx_get_string_reply());
        assert!(lib.has_xcb_glx_get_tex_envfv_sizeof());
        assert!(lib.has_xcb_glx_get_tex_envfv());
        assert!(lib.has_xcb_glx_get_tex_envfv_unchecked());
        assert!(lib.has_xcb_glx_get_tex_envfv_data());
        assert!(lib.has_xcb_glx_get_tex_envfv_data_length());
        assert!(lib.has_xcb_glx_get_tex_envfv_data_end());
        assert!(lib.has_xcb_glx_get_tex_envfv_reply());
        assert!(lib.has_xcb_glx_get_tex_enviv_sizeof());
        assert!(lib.has_xcb_glx_get_tex_enviv());
        assert!(lib.has_xcb_glx_get_tex_enviv_unchecked());
        assert!(lib.has_xcb_glx_get_tex_enviv_data());
        assert!(lib.has_xcb_glx_get_tex_enviv_data_length());
        assert!(lib.has_xcb_glx_get_tex_enviv_data_end());
        assert!(lib.has_xcb_glx_get_tex_enviv_reply());
        assert!(lib.has_xcb_glx_get_tex_gendv_sizeof());
        assert!(lib.has_xcb_glx_get_tex_gendv());
        assert!(lib.has_xcb_glx_get_tex_gendv_unchecked());
        assert!(lib.has_xcb_glx_get_tex_gendv_data());
        assert!(lib.has_xcb_glx_get_tex_gendv_data_length());
        assert!(lib.has_xcb_glx_get_tex_gendv_data_end());
        assert!(lib.has_xcb_glx_get_tex_gendv_reply());
        assert!(lib.has_xcb_glx_get_tex_genfv_sizeof());
        assert!(lib.has_xcb_glx_get_tex_genfv());
        assert!(lib.has_xcb_glx_get_tex_genfv_unchecked());
        assert!(lib.has_xcb_glx_get_tex_genfv_data());
        assert!(lib.has_xcb_glx_get_tex_genfv_data_length());
        assert!(lib.has_xcb_glx_get_tex_genfv_data_end());
        assert!(lib.has_xcb_glx_get_tex_genfv_reply());
        assert!(lib.has_xcb_glx_get_tex_geniv_sizeof());
        assert!(lib.has_xcb_glx_get_tex_geniv());
        assert!(lib.has_xcb_glx_get_tex_geniv_unchecked());
        assert!(lib.has_xcb_glx_get_tex_geniv_data());
        assert!(lib.has_xcb_glx_get_tex_geniv_data_length());
        assert!(lib.has_xcb_glx_get_tex_geniv_data_end());
        assert!(lib.has_xcb_glx_get_tex_geniv_reply());
        assert!(lib.has_xcb_glx_get_tex_image_sizeof());
        assert!(lib.has_xcb_glx_get_tex_image());
        assert!(lib.has_xcb_glx_get_tex_image_unchecked());
        assert!(lib.has_xcb_glx_get_tex_image_data());
        assert!(lib.has_xcb_glx_get_tex_image_data_length());
        assert!(lib.has_xcb_glx_get_tex_image_data_end());
        assert!(lib.has_xcb_glx_get_tex_image_reply());
        assert!(lib.has_xcb_glx_get_tex_parameterfv_sizeof());
        assert!(lib.has_xcb_glx_get_tex_parameterfv());
        assert!(lib.has_xcb_glx_get_tex_parameterfv_unchecked());
        assert!(lib.has_xcb_glx_get_tex_parameterfv_data());
        assert!(lib.has_xcb_glx_get_tex_parameterfv_data_length());
        assert!(lib.has_xcb_glx_get_tex_parameterfv_data_end());
        assert!(lib.has_xcb_glx_get_tex_parameterfv_reply());
        assert!(lib.has_xcb_glx_get_tex_parameteriv_sizeof());
        assert!(lib.has_xcb_glx_get_tex_parameteriv());
        assert!(lib.has_xcb_glx_get_tex_parameteriv_unchecked());
        assert!(lib.has_xcb_glx_get_tex_parameteriv_data());
        assert!(lib.has_xcb_glx_get_tex_parameteriv_data_length());
        assert!(lib.has_xcb_glx_get_tex_parameteriv_data_end());
        assert!(lib.has_xcb_glx_get_tex_parameteriv_reply());
        assert!(lib.has_xcb_glx_get_tex_level_parameterfv_sizeof());
        assert!(lib.has_xcb_glx_get_tex_level_parameterfv());
        assert!(lib.has_xcb_glx_get_tex_level_parameterfv_unchecked());
        assert!(lib.has_xcb_glx_get_tex_level_parameterfv_data());
        assert!(lib.has_xcb_glx_get_tex_level_parameterfv_data_length());
        assert!(lib.has_xcb_glx_get_tex_level_parameterfv_data_end());
        assert!(lib.has_xcb_glx_get_tex_level_parameterfv_reply());
        assert!(lib.has_xcb_glx_get_tex_level_parameteriv_sizeof());
        assert!(lib.has_xcb_glx_get_tex_level_parameteriv());
        assert!(lib.has_xcb_glx_get_tex_level_parameteriv_unchecked());
        assert!(lib.has_xcb_glx_get_tex_level_parameteriv_data());
        assert!(lib.has_xcb_glx_get_tex_level_parameteriv_data_length());
        assert!(lib.has_xcb_glx_get_tex_level_parameteriv_data_end());
        assert!(lib.has_xcb_glx_get_tex_level_parameteriv_reply());
        assert!(lib.has_xcb_glx_is_enabled());
        assert!(lib.has_xcb_glx_is_enabled_unchecked());
        assert!(lib.has_xcb_glx_is_enabled_reply());
        assert!(lib.has_xcb_glx_is_list());
        assert!(lib.has_xcb_glx_is_list_unchecked());
        assert!(lib.has_xcb_glx_is_list_reply());
        assert!(lib.has_xcb_glx_flush_checked());
        assert!(lib.has_xcb_glx_flush());
        assert!(lib.has_xcb_glx_are_textures_resident_sizeof());
        assert!(lib.has_xcb_glx_are_textures_resident());
        assert!(lib.has_xcb_glx_are_textures_resident_unchecked());
        assert!(lib.has_xcb_glx_are_textures_resident_data());
        assert!(lib.has_xcb_glx_are_textures_resident_data_length());
        assert!(lib.has_xcb_glx_are_textures_resident_data_end());
        assert!(lib.has_xcb_glx_are_textures_resident_reply());
        assert!(lib.has_xcb_glx_delete_textures_sizeof());
        assert!(lib.has_xcb_glx_delete_textures_checked());
        assert!(lib.has_xcb_glx_delete_textures());
        assert!(lib.has_xcb_glx_delete_textures_textures());
        assert!(lib.has_xcb_glx_delete_textures_textures_length());
        assert!(lib.has_xcb_glx_delete_textures_textures_end());
        assert!(lib.has_xcb_glx_gen_textures_sizeof());
        assert!(lib.has_xcb_glx_gen_textures());
        assert!(lib.has_xcb_glx_gen_textures_unchecked());
        assert!(lib.has_xcb_glx_gen_textures_data());
        assert!(lib.has_xcb_glx_gen_textures_data_length());
        assert!(lib.has_xcb_glx_gen_textures_data_end());
        assert!(lib.has_xcb_glx_gen_textures_reply());
        assert!(lib.has_xcb_glx_is_texture());
        assert!(lib.has_xcb_glx_is_texture_unchecked());
        assert!(lib.has_xcb_glx_is_texture_reply());
        assert!(lib.has_xcb_glx_get_color_table_sizeof());
        assert!(lib.has_xcb_glx_get_color_table());
        assert!(lib.has_xcb_glx_get_color_table_unchecked());
        assert!(lib.has_xcb_glx_get_color_table_data());
        assert!(lib.has_xcb_glx_get_color_table_data_length());
        assert!(lib.has_xcb_glx_get_color_table_data_end());
        assert!(lib.has_xcb_glx_get_color_table_reply());
        assert!(lib.has_xcb_glx_get_color_table_parameterfv_sizeof());
        assert!(lib.has_xcb_glx_get_color_table_parameterfv());
        assert!(lib.has_xcb_glx_get_color_table_parameterfv_unchecked());
        assert!(lib.has_xcb_glx_get_color_table_parameterfv_data());
        assert!(lib.has_xcb_glx_get_color_table_parameterfv_data_length());
        assert!(lib.has_xcb_glx_get_color_table_parameterfv_data_end());
        assert!(lib.has_xcb_glx_get_color_table_parameterfv_reply());
        assert!(lib.has_xcb_glx_get_color_table_parameteriv_sizeof());
        assert!(lib.has_xcb_glx_get_color_table_parameteriv());
        assert!(lib.has_xcb_glx_get_color_table_parameteriv_unchecked());
        assert!(lib.has_xcb_glx_get_color_table_parameteriv_data());
        assert!(lib.has_xcb_glx_get_color_table_parameteriv_data_length());
        assert!(lib.has_xcb_glx_get_color_table_parameteriv_data_end());
        assert!(lib.has_xcb_glx_get_color_table_parameteriv_reply());
        assert!(lib.has_xcb_glx_get_convolution_filter_sizeof());
        assert!(lib.has_xcb_glx_get_convolution_filter());
        assert!(lib.has_xcb_glx_get_convolution_filter_unchecked());
        assert!(lib.has_xcb_glx_get_convolution_filter_data());
        assert!(lib.has_xcb_glx_get_convolution_filter_data_length());
        assert!(lib.has_xcb_glx_get_convolution_filter_data_end());
        assert!(lib.has_xcb_glx_get_convolution_filter_reply());
        assert!(lib.has_xcb_glx_get_convolution_parameterfv_sizeof());
        assert!(lib.has_xcb_glx_get_convolution_parameterfv());
        assert!(lib.has_xcb_glx_get_convolution_parameterfv_unchecked());
        assert!(lib.has_xcb_glx_get_convolution_parameterfv_data());
        assert!(lib.has_xcb_glx_get_convolution_parameterfv_data_length());
        assert!(lib.has_xcb_glx_get_convolution_parameterfv_data_end());
        assert!(lib.has_xcb_glx_get_convolution_parameterfv_reply());
        assert!(lib.has_xcb_glx_get_convolution_parameteriv_sizeof());
        assert!(lib.has_xcb_glx_get_convolution_parameteriv());
        assert!(lib.has_xcb_glx_get_convolution_parameteriv_unchecked());
        assert!(lib.has_xcb_glx_get_convolution_parameteriv_data());
        assert!(lib.has_xcb_glx_get_convolution_parameteriv_data_length());
        assert!(lib.has_xcb_glx_get_convolution_parameteriv_data_end());
        assert!(lib.has_xcb_glx_get_convolution_parameteriv_reply());
        assert!(lib.has_xcb_glx_get_separable_filter_sizeof());
        assert!(lib.has_xcb_glx_get_separable_filter());
        assert!(lib.has_xcb_glx_get_separable_filter_unchecked());
        assert!(lib.has_xcb_glx_get_separable_filter_rows_and_cols());
        assert!(lib.has_xcb_glx_get_separable_filter_rows_and_cols_length());
        assert!(lib.has_xcb_glx_get_separable_filter_rows_and_cols_end());
        assert!(lib.has_xcb_glx_get_separable_filter_reply());
        assert!(lib.has_xcb_glx_get_histogram_sizeof());
        assert!(lib.has_xcb_glx_get_histogram());
        assert!(lib.has_xcb_glx_get_histogram_unchecked());
        assert!(lib.has_xcb_glx_get_histogram_data());
        assert!(lib.has_xcb_glx_get_histogram_data_length());
        assert!(lib.has_xcb_glx_get_histogram_data_end());
        assert!(lib.has_xcb_glx_get_histogram_reply());
        assert!(lib.has_xcb_glx_get_histogram_parameterfv_sizeof());
        assert!(lib.has_xcb_glx_get_histogram_parameterfv());
        assert!(lib.has_xcb_glx_get_histogram_parameterfv_unchecked());
        assert!(lib.has_xcb_glx_get_histogram_parameterfv_data());
        assert!(lib.has_xcb_glx_get_histogram_parameterfv_data_length());
        assert!(lib.has_xcb_glx_get_histogram_parameterfv_data_end());
        assert!(lib.has_xcb_glx_get_histogram_parameterfv_reply());
        assert!(lib.has_xcb_glx_get_histogram_parameteriv_sizeof());
        assert!(lib.has_xcb_glx_get_histogram_parameteriv());
        assert!(lib.has_xcb_glx_get_histogram_parameteriv_unchecked());
        assert!(lib.has_xcb_glx_get_histogram_parameteriv_data());
        assert!(lib.has_xcb_glx_get_histogram_parameteriv_data_length());
        assert!(lib.has_xcb_glx_get_histogram_parameteriv_data_end());
        assert!(lib.has_xcb_glx_get_histogram_parameteriv_reply());
        assert!(lib.has_xcb_glx_get_minmax_sizeof());
        assert!(lib.has_xcb_glx_get_minmax());
        assert!(lib.has_xcb_glx_get_minmax_unchecked());
        assert!(lib.has_xcb_glx_get_minmax_data());
        assert!(lib.has_xcb_glx_get_minmax_data_length());
        assert!(lib.has_xcb_glx_get_minmax_data_end());
        assert!(lib.has_xcb_glx_get_minmax_reply());
        assert!(lib.has_xcb_glx_get_minmax_parameterfv_sizeof());
        assert!(lib.has_xcb_glx_get_minmax_parameterfv());
        assert!(lib.has_xcb_glx_get_minmax_parameterfv_unchecked());
        assert!(lib.has_xcb_glx_get_minmax_parameterfv_data());
        assert!(lib.has_xcb_glx_get_minmax_parameterfv_data_length());
        assert!(lib.has_xcb_glx_get_minmax_parameterfv_data_end());
        assert!(lib.has_xcb_glx_get_minmax_parameterfv_reply());
        assert!(lib.has_xcb_glx_get_minmax_parameteriv_sizeof());
        assert!(lib.has_xcb_glx_get_minmax_parameteriv());
        assert!(lib.has_xcb_glx_get_minmax_parameteriv_unchecked());
        assert!(lib.has_xcb_glx_get_minmax_parameteriv_data());
        assert!(lib.has_xcb_glx_get_minmax_parameteriv_data_length());
        assert!(lib.has_xcb_glx_get_minmax_parameteriv_data_end());
        assert!(lib.has_xcb_glx_get_minmax_parameteriv_reply());
        assert!(lib.has_xcb_glx_get_compressed_tex_image_arb_sizeof());
        assert!(lib.has_xcb_glx_get_compressed_tex_image_arb());
        assert!(lib.has_xcb_glx_get_compressed_tex_image_arb_unchecked());
        assert!(lib.has_xcb_glx_get_compressed_tex_image_arb_data());
        assert!(lib.has_xcb_glx_get_compressed_tex_image_arb_data_length());
        assert!(lib.has_xcb_glx_get_compressed_tex_image_arb_data_end());
        assert!(lib.has_xcb_glx_get_compressed_tex_image_arb_reply());
        assert!(lib.has_xcb_glx_delete_queries_arb_sizeof());
        assert!(lib.has_xcb_glx_delete_queries_arb_checked());
        assert!(lib.has_xcb_glx_delete_queries_arb());
        assert!(lib.has_xcb_glx_delete_queries_arb_ids());
        assert!(lib.has_xcb_glx_delete_queries_arb_ids_length());
        assert!(lib.has_xcb_glx_delete_queries_arb_ids_end());
        assert!(lib.has_xcb_glx_gen_queries_arb_sizeof());
        assert!(lib.has_xcb_glx_gen_queries_arb());
        assert!(lib.has_xcb_glx_gen_queries_arb_unchecked());
        assert!(lib.has_xcb_glx_gen_queries_arb_data());
        assert!(lib.has_xcb_glx_gen_queries_arb_data_length());
        assert!(lib.has_xcb_glx_gen_queries_arb_data_end());
        assert!(lib.has_xcb_glx_gen_queries_arb_reply());
        assert!(lib.has_xcb_glx_is_query_arb());
        assert!(lib.has_xcb_glx_is_query_arb_unchecked());
        assert!(lib.has_xcb_glx_is_query_arb_reply());
        assert!(lib.has_xcb_glx_get_queryiv_arb_sizeof());
        assert!(lib.has_xcb_glx_get_queryiv_arb());
        assert!(lib.has_xcb_glx_get_queryiv_arb_unchecked());
        assert!(lib.has_xcb_glx_get_queryiv_arb_data());
        assert!(lib.has_xcb_glx_get_queryiv_arb_data_length());
        assert!(lib.has_xcb_glx_get_queryiv_arb_data_end());
        assert!(lib.has_xcb_glx_get_queryiv_arb_reply());
        assert!(lib.has_xcb_glx_get_query_objectiv_arb_sizeof());
        assert!(lib.has_xcb_glx_get_query_objectiv_arb());
        assert!(lib.has_xcb_glx_get_query_objectiv_arb_unchecked());
        assert!(lib.has_xcb_glx_get_query_objectiv_arb_data());
        assert!(lib.has_xcb_glx_get_query_objectiv_arb_data_length());
        assert!(lib.has_xcb_glx_get_query_objectiv_arb_data_end());
        assert!(lib.has_xcb_glx_get_query_objectiv_arb_reply());
        assert!(lib.has_xcb_glx_get_query_objectuiv_arb_sizeof());
        assert!(lib.has_xcb_glx_get_query_objectuiv_arb());
        assert!(lib.has_xcb_glx_get_query_objectuiv_arb_unchecked());
        assert!(lib.has_xcb_glx_get_query_objectuiv_arb_data());
        assert!(lib.has_xcb_glx_get_query_objectuiv_arb_data_length());
        assert!(lib.has_xcb_glx_get_query_objectuiv_arb_data_end());
        assert!(lib.has_xcb_glx_get_query_objectuiv_arb_reply());
    }
}
