use crate::ffi::*;
use crate::*;
use std::os::raw::*;

pub const XCB_XV_MAJOR_VERSION: u32 = 2;
pub const XCB_XV_MINOR_VERSION: u32 = 2;

pub type xcb_xv_port_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_port_iterator_t {
    pub data: *mut xcb_xv_port_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_xv_encoding_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_encoding_iterator_t {
    pub data: *mut xcb_xv_encoding_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_xv_type_t = u32;
pub const XCB_XV_TYPE_INPUT_MASK: xcb_xv_type_t = 0x01;
pub const XCB_XV_TYPE_OUTPUT_MASK: xcb_xv_type_t = 0x02;
pub const XCB_XV_TYPE_VIDEO_MASK: xcb_xv_type_t = 0x04;
pub const XCB_XV_TYPE_STILL_MASK: xcb_xv_type_t = 0x08;
pub const XCB_XV_TYPE_IMAGE_MASK: xcb_xv_type_t = 0x10;

pub type xcb_xv_image_format_info_type_t = u32;
pub const XCB_XV_IMAGE_FORMAT_INFO_TYPE_RGB: xcb_xv_image_format_info_type_t = 0x00;
pub const XCB_XV_IMAGE_FORMAT_INFO_TYPE_YUV: xcb_xv_image_format_info_type_t = 0x01;

pub type xcb_xv_image_format_info_format_t = u32;
pub const XCB_XV_IMAGE_FORMAT_INFO_FORMAT_PACKED: xcb_xv_image_format_info_format_t = 0x00;
pub const XCB_XV_IMAGE_FORMAT_INFO_FORMAT_PLANAR: xcb_xv_image_format_info_format_t = 0x01;

pub type xcb_xv_attribute_flag_t = u32;
pub const XCB_XV_ATTRIBUTE_FLAG_GETTABLE: xcb_xv_attribute_flag_t = 0x01;
pub const XCB_XV_ATTRIBUTE_FLAG_SETTABLE: xcb_xv_attribute_flag_t = 0x02;

pub type xcb_xv_video_notify_reason_t = u32;
pub const XCB_XV_VIDEO_NOTIFY_REASON_STARTED: xcb_xv_video_notify_reason_t = 0x00;
pub const XCB_XV_VIDEO_NOTIFY_REASON_STOPPED: xcb_xv_video_notify_reason_t = 0x01;
pub const XCB_XV_VIDEO_NOTIFY_REASON_BUSY: xcb_xv_video_notify_reason_t = 0x02;
pub const XCB_XV_VIDEO_NOTIFY_REASON_PREEMPTED: xcb_xv_video_notify_reason_t = 0x03;
pub const XCB_XV_VIDEO_NOTIFY_REASON_HARD_ERROR: xcb_xv_video_notify_reason_t = 0x04;

pub type xcb_xv_scanline_order_t = u32;
pub const XCB_XV_SCANLINE_ORDER_TOP_TO_BOTTOM: xcb_xv_scanline_order_t = 0x00;
pub const XCB_XV_SCANLINE_ORDER_BOTTOM_TO_TOP: xcb_xv_scanline_order_t = 0x01;

pub type xcb_xv_grab_port_status_t = u32;
pub const XCB_XV_GRAB_PORT_STATUS_SUCCESS: xcb_xv_grab_port_status_t = 0x00;
pub const XCB_XV_GRAB_PORT_STATUS_BAD_EXTENSION: xcb_xv_grab_port_status_t = 0x01;
pub const XCB_XV_GRAB_PORT_STATUS_ALREADY_GRABBED: xcb_xv_grab_port_status_t = 0x02;
pub const XCB_XV_GRAB_PORT_STATUS_INVALID_TIME: xcb_xv_grab_port_status_t = 0x03;
pub const XCB_XV_GRAB_PORT_STATUS_BAD_REPLY: xcb_xv_grab_port_status_t = 0x04;
pub const XCB_XV_GRAB_PORT_STATUS_BAD_ALLOC: xcb_xv_grab_port_status_t = 0x05;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_rational_t {
    pub numerator: i32,
    pub denominator: i32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_rational_iterator_t {
    pub data: *mut xcb_xv_rational_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_format_t {
    pub visual: xcb_visualid_t,
    pub depth: u8,
    pub pad0: [u8; 3],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_format_iterator_t {
    pub data: *mut xcb_xv_format_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_adaptor_info_t {
    pub base_id: xcb_xv_port_t,
    pub name_size: u16,
    pub num_ports: u16,
    pub num_formats: u16,
    pub type_: u8,
    pub pad0: u8,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_adaptor_info_iterator_t {
    pub data: *mut xcb_xv_adaptor_info_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_encoding_info_t {
    pub encoding: xcb_xv_encoding_t,
    pub name_size: u16,
    pub width: u16,
    pub height: u16,
    pub pad0: [u8; 2],
    pub rate: xcb_xv_rational_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_encoding_info_iterator_t {
    pub data: *mut xcb_xv_encoding_info_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_image_t {
    pub id: u32,
    pub width: u16,
    pub height: u16,
    pub data_size: u32,
    pub num_planes: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_image_iterator_t {
    pub data: *mut xcb_xv_image_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_attribute_info_t {
    pub flags: u32,
    pub min: i32,
    pub max: i32,
    pub size: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_attribute_info_iterator_t {
    pub data: *mut xcb_xv_attribute_info_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_image_format_info_t {
    pub id: u32,
    pub type_: u8,
    pub byte_order: u8,
    pub pad0: [u8; 2],
    pub guid: [u8; 16],
    pub bpp: u8,
    pub num_planes: u8,
    pub pad1: [u8; 2],
    pub depth: u8,
    pub pad2: [u8; 3],
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub format: u8,
    pub pad3: [u8; 3],
    pub y_sample_bits: u32,
    pub u_sample_bits: u32,
    pub v_sample_bits: u32,
    pub vhorz_y_period: u32,
    pub vhorz_u_period: u32,
    pub vhorz_v_period: u32,
    pub vvert_y_period: u32,
    pub vvert_u_period: u32,
    pub vvert_v_period: u32,
    pub vcomp_order: [u8; 32],
    pub vscanline_order: u8,
    pub pad4: [u8; 11],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_image_format_info_iterator_t {
    pub data: *mut xcb_xv_image_format_info_t,
    pub rem: c_int,
    pub index: c_int,
}

pub const XCB_XV_BAD_PORT: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_bad_port_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

pub const XCB_XV_BAD_ENCODING: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_bad_encoding_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

pub const XCB_XV_BAD_CONTROL: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_bad_control_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

pub const XCB_XV_VIDEO_NOTIFY: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_video_notify_event_t {
    pub response_type: u8,
    pub reason: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub drawable: xcb_drawable_t,
    pub port: xcb_xv_port_t,
}

pub const XCB_XV_PORT_NOTIFY: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_port_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub port: xcb_xv_port_t,
    pub attribute: xcb_atom_t,
    pub value: i32,
}

pub const XCB_XV_QUERY_EXTENSION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_extension_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_extension_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_extension_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major: u16,
    pub minor: u16,
}

pub const XCB_XV_QUERY_ADAPTORS: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_adaptors_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_adaptors_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_adaptors_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_adaptors: u16,
    pub pad1: [u8; 22],
}

pub const XCB_XV_QUERY_ENCODINGS: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_encodings_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_encodings_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_encodings_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_encodings: u16,
    pub pad1: [u8; 22],
}

pub const XCB_XV_GRAB_PORT: u8 = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_grab_port_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub time: xcb_timestamp_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_grab_port_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_grab_port_reply_t {
    pub response_type: u8,
    pub result: u8,
    pub sequence: u16,
    pub length: u32,
}

pub const XCB_XV_UNGRAB_PORT: u8 = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_ungrab_port_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub time: xcb_timestamp_t,
}

pub const XCB_XV_PUT_VIDEO: u8 = 5;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_put_video_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub vid_x: i16,
    pub vid_y: i16,
    pub vid_w: u16,
    pub vid_h: u16,
    pub drw_x: i16,
    pub drw_y: i16,
    pub drw_w: u16,
    pub drw_h: u16,
}

pub const XCB_XV_PUT_STILL: u8 = 6;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_put_still_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub vid_x: i16,
    pub vid_y: i16,
    pub vid_w: u16,
    pub vid_h: u16,
    pub drw_x: i16,
    pub drw_y: i16,
    pub drw_w: u16,
    pub drw_h: u16,
}

pub const XCB_XV_GET_VIDEO: u8 = 7;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_get_video_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub vid_x: i16,
    pub vid_y: i16,
    pub vid_w: u16,
    pub vid_h: u16,
    pub drw_x: i16,
    pub drw_y: i16,
    pub drw_w: u16,
    pub drw_h: u16,
}

pub const XCB_XV_GET_STILL: u8 = 8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_get_still_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub vid_x: i16,
    pub vid_y: i16,
    pub vid_w: u16,
    pub vid_h: u16,
    pub drw_x: i16,
    pub drw_y: i16,
    pub drw_w: u16,
    pub drw_h: u16,
}

pub const XCB_XV_STOP_VIDEO: u8 = 9;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_stop_video_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub drawable: xcb_drawable_t,
}

pub const XCB_XV_SELECT_VIDEO_NOTIFY: u8 = 10;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_select_video_notify_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub onoff: u8,
    pub pad0: [u8; 3],
}

pub const XCB_XV_SELECT_PORT_NOTIFY: u8 = 11;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_select_port_notify_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub onoff: u8,
    pub pad0: [u8; 3],
}

pub const XCB_XV_QUERY_BEST_SIZE: u8 = 12;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_best_size_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub vid_w: u16,
    pub vid_h: u16,
    pub drw_w: u16,
    pub drw_h: u16,
    pub motion: u8,
    pub pad0: [u8; 3],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_best_size_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_best_size_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub actual_width: u16,
    pub actual_height: u16,
}

pub const XCB_XV_SET_PORT_ATTRIBUTE: u8 = 13;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_set_port_attribute_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub attribute: xcb_atom_t,
    pub value: i32,
}

pub const XCB_XV_GET_PORT_ATTRIBUTE: u8 = 14;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_get_port_attribute_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub attribute: xcb_atom_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_get_port_attribute_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_get_port_attribute_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub value: i32,
}

pub const XCB_XV_QUERY_PORT_ATTRIBUTES: u8 = 15;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_port_attributes_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_port_attributes_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_port_attributes_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_attributes: u32,
    pub text_size: u32,
    pub pad1: [u8; 16],
}

pub const XCB_XV_LIST_IMAGE_FORMATS: u8 = 16;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_list_image_formats_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_list_image_formats_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_list_image_formats_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_formats: u32,
    pub pad1: [u8; 20],
}

pub const XCB_XV_QUERY_IMAGE_ATTRIBUTES: u8 = 17;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_image_attributes_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub id: u32,
    pub width: u16,
    pub height: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_image_attributes_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_query_image_attributes_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_planes: u32,
    pub data_size: u32,
    pub width: u16,
    pub height: u16,
    pub pad1: [u8; 12],
}

pub const XCB_XV_PUT_IMAGE: u8 = 18;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_put_image_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub id: u32,
    pub src_x: i16,
    pub src_y: i16,
    pub src_w: u16,
    pub src_h: u16,
    pub drw_x: i16,
    pub drw_y: i16,
    pub drw_w: u16,
    pub drw_h: u16,
    pub width: u16,
    pub height: u16,
}

pub const XCB_XV_SHM_PUT_IMAGE: u8 = 19;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xv_shm_put_image_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub port: xcb_xv_port_t,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub shmseg: xcb_shm_seg_t,
    pub id: u32,
    pub offset: u32,
    pub src_x: i16,
    pub src_y: i16,
    pub src_w: u16,
    pub src_h: u16,
    pub drw_x: i16,
    pub drw_y: i16,
    pub drw_w: u16,
    pub drw_h: u16,
    pub width: u16,
    pub height: u16,
    pub send_event: u8,
    pub pad0: [u8; 3],
}

impl XcbXv {
    #[inline]
    pub unsafe fn xcb_xv_id(&self) -> *mut xcb_extension_t {
        sym!(self, xcb_xv_id)
    }

    #[inline]
    pub unsafe fn xcb_xv_port_next(&self, i: *mut xcb_xv_port_iterator_t) {
        sym!(self, xcb_xv_port_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_xv_port_end(&self, i: *mut xcb_xv_port_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_port_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_xv_encoding_next(&self, i: *mut xcb_xv_encoding_iterator_t) {
        sym!(self, xcb_xv_encoding_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_xv_encoding_end(
        &self,
        i: *mut xcb_xv_encoding_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_encoding_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_xv_rational_next(&self, i: *mut xcb_xv_rational_iterator_t) {
        sym!(self, xcb_xv_rational_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_xv_rational_end(
        &self,
        i: *mut xcb_xv_rational_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_rational_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_xv_format_next(&self, i: *mut xcb_xv_format_iterator_t) {
        sym!(self, xcb_xv_format_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_xv_format_end(
        &self,
        i: *mut xcb_xv_format_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_format_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_xv_adaptor_info_name(&self, R: *const xcb_xv_adaptor_info_t) -> *mut c_char {
        sym!(self, xcb_xv_adaptor_info_name)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_adaptor_info_name_length(&self, R: *const xcb_xv_adaptor_info_t) -> c_int {
        sym!(self, xcb_xv_adaptor_info_name_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_adaptor_info_name_end(
        &self,
        R: *const xcb_xv_adaptor_info_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_adaptor_info_name_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_adaptor_info_formats(
        &self,
        R: *const xcb_xv_adaptor_info_t,
    ) -> *mut xcb_xv_format_t {
        sym!(self, xcb_xv_adaptor_info_formats)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_adaptor_info_formats_length(
        &self,
        R: *const xcb_xv_adaptor_info_t,
    ) -> c_int {
        sym!(self, xcb_xv_adaptor_info_formats_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_adaptor_info_formats_iterator(
        &self,
        R: *const xcb_xv_adaptor_info_t,
    ) -> xcb_xv_format_iterator_t {
        sym!(self, xcb_xv_adaptor_info_formats_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_adaptor_info_next(&self, i: *mut xcb_xv_adaptor_info_iterator_t) {
        sym!(self, xcb_xv_adaptor_info_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_xv_adaptor_info_end(
        &self,
        i: *mut xcb_xv_adaptor_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_adaptor_info_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_xv_encoding_info_name(
        &self,
        R: *const xcb_xv_encoding_info_t,
    ) -> *mut c_char {
        sym!(self, xcb_xv_encoding_info_name)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_encoding_info_name_length(
        &self,
        R: *const xcb_xv_encoding_info_t,
    ) -> c_int {
        sym!(self, xcb_xv_encoding_info_name_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_encoding_info_name_end(
        &self,
        R: *const xcb_xv_encoding_info_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_encoding_info_name_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_encoding_info_next(&self, i: *mut xcb_xv_encoding_info_iterator_t) {
        sym!(self, xcb_xv_encoding_info_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_xv_encoding_info_end(
        &self,
        i: *mut xcb_xv_encoding_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_encoding_info_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_xv_image_pitches(&self, R: *const xcb_xv_image_t) -> *mut u32 {
        sym!(self, xcb_xv_image_pitches)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_image_pitches_length(&self, R: *const xcb_xv_image_t) -> c_int {
        sym!(self, xcb_xv_image_pitches_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_image_pitches_end(
        &self,
        R: *const xcb_xv_image_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_image_pitches_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_image_offsets(&self, R: *const xcb_xv_image_t) -> *mut u32 {
        sym!(self, xcb_xv_image_offsets)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_image_offsets_length(&self, R: *const xcb_xv_image_t) -> c_int {
        sym!(self, xcb_xv_image_offsets_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_image_offsets_end(
        &self,
        R: *const xcb_xv_image_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_image_offsets_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_image_data(&self, R: *const xcb_xv_image_t) -> *mut u8 {
        sym!(self, xcb_xv_image_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_image_data_length(&self, R: *const xcb_xv_image_t) -> c_int {
        sym!(self, xcb_xv_image_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_image_data_end(&self, R: *const xcb_xv_image_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_image_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_image_next(&self, i: *mut xcb_xv_image_iterator_t) {
        sym!(self, xcb_xv_image_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_xv_image_end(
        &self,
        i: *mut xcb_xv_image_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_image_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_xv_attribute_info_name(
        &self,
        R: *const xcb_xv_attribute_info_t,
    ) -> *mut c_char {
        sym!(self, xcb_xv_attribute_info_name)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_attribute_info_name_length(
        &self,
        R: *const xcb_xv_attribute_info_t,
    ) -> c_int {
        sym!(self, xcb_xv_attribute_info_name_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_attribute_info_name_end(
        &self,
        R: *const xcb_xv_attribute_info_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_attribute_info_name_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_attribute_info_next(&self, i: *mut xcb_xv_attribute_info_iterator_t) {
        sym!(self, xcb_xv_attribute_info_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_xv_attribute_info_end(
        &self,
        i: *mut xcb_xv_attribute_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_attribute_info_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_xv_image_format_info_next(
        &self,
        i: *mut xcb_xv_image_format_info_iterator_t,
    ) {
        sym!(self, xcb_xv_image_format_info_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_xv_image_format_info_end(
        &self,
        i: *mut xcb_xv_image_format_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_image_format_info_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_extension_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xv_query_extension_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xv_query_extension_reply_t {
        sym!(self, xcb_xv_query_extension_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_extension(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xv_query_extension_cookie_t {
        sym!(self, xcb_xv_query_extension)(c)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_extension_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xv_query_extension_cookie_t {
        sym!(self, xcb_xv_query_extension_unchecked)(c)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_adaptors_info_length(
        &self,
        R: *const xcb_xv_query_adaptors_reply_t,
    ) -> c_int {
        sym!(self, xcb_xv_query_adaptors_info_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_adaptors_info_iterator(
        &self,
        R: *const xcb_xv_query_adaptors_reply_t,
    ) -> xcb_xv_adaptor_info_iterator_t {
        sym!(self, xcb_xv_query_adaptors_info_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_adaptors_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xv_query_adaptors_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xv_query_adaptors_reply_t {
        sym!(self, xcb_xv_query_adaptors_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_adaptors(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_xv_query_adaptors_cookie_t {
        sym!(self, xcb_xv_query_adaptors)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_adaptors_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_xv_query_adaptors_cookie_t {
        sym!(self, xcb_xv_query_adaptors_unchecked)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_encodings_info_length(
        &self,
        R: *const xcb_xv_query_encodings_reply_t,
    ) -> c_int {
        sym!(self, xcb_xv_query_encodings_info_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_encodings_info_iterator(
        &self,
        R: *const xcb_xv_query_encodings_reply_t,
    ) -> xcb_xv_encoding_info_iterator_t {
        sym!(self, xcb_xv_query_encodings_info_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_encodings_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xv_query_encodings_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xv_query_encodings_reply_t {
        sym!(self, xcb_xv_query_encodings_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_encodings(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
    ) -> xcb_xv_query_encodings_cookie_t {
        sym!(self, xcb_xv_query_encodings)(c, port)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_encodings_unchecked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
    ) -> xcb_xv_query_encodings_cookie_t {
        sym!(self, xcb_xv_query_encodings_unchecked)(c, port)
    }

    #[inline]
    pub unsafe fn xcb_xv_grab_port_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xv_grab_port_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xv_grab_port_reply_t {
        sym!(self, xcb_xv_grab_port_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xv_grab_port(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        time: xcb_timestamp_t,
    ) -> xcb_xv_grab_port_cookie_t {
        sym!(self, xcb_xv_grab_port)(c, port, time)
    }

    #[inline]
    pub unsafe fn xcb_xv_grab_port_unchecked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        time: xcb_timestamp_t,
    ) -> xcb_xv_grab_port_cookie_t {
        sym!(self, xcb_xv_grab_port_unchecked)(c, port, time)
    }

    #[inline]
    pub unsafe fn xcb_xv_ungrab_port(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_ungrab_port)(c, port, time)
    }

    #[inline]
    pub unsafe fn xcb_xv_ungrab_port_checked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_ungrab_port_checked)(c, port, time)
    }

    #[inline]
    pub unsafe fn xcb_xv_put_video(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        vid_x: i16,
        vid_y: i16,
        vid_w: u16,
        vid_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_put_video)(
            c, port, drawable, gc, vid_x, vid_y, vid_w, vid_h, drw_x, drw_y, drw_w, drw_h,
        )
    }

    #[inline]
    pub unsafe fn xcb_xv_put_video_checked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        vid_x: i16,
        vid_y: i16,
        vid_w: u16,
        vid_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_put_video_checked)(
            c, port, drawable, gc, vid_x, vid_y, vid_w, vid_h, drw_x, drw_y, drw_w, drw_h,
        )
    }

    #[inline]
    pub unsafe fn xcb_xv_put_still(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        vid_x: i16,
        vid_y: i16,
        vid_w: u16,
        vid_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_put_still)(
            c, port, drawable, gc, vid_x, vid_y, vid_w, vid_h, drw_x, drw_y, drw_w, drw_h,
        )
    }

    #[inline]
    pub unsafe fn xcb_xv_put_still_checked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        vid_x: i16,
        vid_y: i16,
        vid_w: u16,
        vid_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_put_still_checked)(
            c, port, drawable, gc, vid_x, vid_y, vid_w, vid_h, drw_x, drw_y, drw_w, drw_h,
        )
    }

    #[inline]
    pub unsafe fn xcb_xv_get_video(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        vid_x: i16,
        vid_y: i16,
        vid_w: u16,
        vid_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_get_video)(
            c, port, drawable, gc, vid_x, vid_y, vid_w, vid_h, drw_x, drw_y, drw_w, drw_h,
        )
    }

    #[inline]
    pub unsafe fn xcb_xv_get_video_checked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        vid_x: i16,
        vid_y: i16,
        vid_w: u16,
        vid_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_get_video_checked)(
            c, port, drawable, gc, vid_x, vid_y, vid_w, vid_h, drw_x, drw_y, drw_w, drw_h,
        )
    }

    #[inline]
    pub unsafe fn xcb_xv_get_still(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        vid_x: i16,
        vid_y: i16,
        vid_w: u16,
        vid_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_get_still)(
            c, port, drawable, gc, vid_x, vid_y, vid_w, vid_h, drw_x, drw_y, drw_w, drw_h,
        )
    }

    #[inline]
    pub unsafe fn xcb_xv_get_still_checked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        vid_x: i16,
        vid_y: i16,
        vid_w: u16,
        vid_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_get_still_checked)(
            c, port, drawable, gc, vid_x, vid_y, vid_w, vid_h, drw_x, drw_y, drw_w, drw_h,
        )
    }

    #[inline]
    pub unsafe fn xcb_xv_stop_video(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_stop_video)(c, port, drawable)
    }

    #[inline]
    pub unsafe fn xcb_xv_stop_video_checked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_stop_video_checked)(c, port, drawable)
    }

    #[inline]
    pub unsafe fn xcb_xv_select_video_notify(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        onoff: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_select_video_notify)(c, drawable, onoff)
    }

    #[inline]
    pub unsafe fn xcb_xv_select_video_notify_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        onoff: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_select_video_notify_checked)(c, drawable, onoff)
    }

    #[inline]
    pub unsafe fn xcb_xv_select_port_notify(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        onoff: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_select_port_notify)(c, port, onoff)
    }

    #[inline]
    pub unsafe fn xcb_xv_select_port_notify_checked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        onoff: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_select_port_notify_checked)(c, port, onoff)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_best_size_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xv_query_best_size_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xv_query_best_size_reply_t {
        sym!(self, xcb_xv_query_best_size_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_best_size(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        vid_w: u16,
        vid_h: u16,
        drw_w: u16,
        drw_h: u16,
        motion: u8,
    ) -> xcb_xv_query_best_size_cookie_t {
        sym!(self, xcb_xv_query_best_size)(c, port, vid_w, vid_h, drw_w, drw_h, motion)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_best_size_unchecked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        vid_w: u16,
        vid_h: u16,
        drw_w: u16,
        drw_h: u16,
        motion: u8,
    ) -> xcb_xv_query_best_size_cookie_t {
        sym!(self, xcb_xv_query_best_size_unchecked)(c, port, vid_w, vid_h, drw_w, drw_h, motion)
    }

    #[inline]
    pub unsafe fn xcb_xv_set_port_attribute(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        attribute: xcb_atom_t,
        value: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_set_port_attribute)(c, port, attribute, value)
    }

    #[inline]
    pub unsafe fn xcb_xv_set_port_attribute_checked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        attribute: xcb_atom_t,
        value: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_set_port_attribute_checked)(c, port, attribute, value)
    }

    #[inline]
    pub unsafe fn xcb_xv_get_port_attribute_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xv_get_port_attribute_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xv_get_port_attribute_reply_t {
        sym!(self, xcb_xv_get_port_attribute_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xv_get_port_attribute(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        attribute: xcb_atom_t,
    ) -> xcb_xv_get_port_attribute_cookie_t {
        sym!(self, xcb_xv_get_port_attribute)(c, port, attribute)
    }

    #[inline]
    pub unsafe fn xcb_xv_get_port_attribute_unchecked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        attribute: xcb_atom_t,
    ) -> xcb_xv_get_port_attribute_cookie_t {
        sym!(self, xcb_xv_get_port_attribute_unchecked)(c, port, attribute)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_port_attributes_attributes_length(
        &self,
        R: *const xcb_xv_query_port_attributes_reply_t,
    ) -> c_int {
        sym!(self, xcb_xv_query_port_attributes_attributes_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_port_attributes_attributes_iterator(
        &self,
        R: *const xcb_xv_query_port_attributes_reply_t,
    ) -> xcb_xv_attribute_info_iterator_t {
        sym!(self, xcb_xv_query_port_attributes_attributes_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_port_attributes_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xv_query_port_attributes_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xv_query_port_attributes_reply_t {
        sym!(self, xcb_xv_query_port_attributes_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_port_attributes(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
    ) -> xcb_xv_query_port_attributes_cookie_t {
        sym!(self, xcb_xv_query_port_attributes)(c, port)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_port_attributes_unchecked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
    ) -> xcb_xv_query_port_attributes_cookie_t {
        sym!(self, xcb_xv_query_port_attributes_unchecked)(c, port)
    }

    #[inline]
    pub unsafe fn xcb_xv_list_image_formats_format(
        &self,
        R: *const xcb_xv_list_image_formats_reply_t,
    ) -> *mut xcb_xv_image_format_info_t {
        sym!(self, xcb_xv_list_image_formats_format)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_list_image_formats_format_length(
        &self,
        R: *const xcb_xv_list_image_formats_reply_t,
    ) -> c_int {
        sym!(self, xcb_xv_list_image_formats_format_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_list_image_formats_format_iterator(
        &self,
        R: *const xcb_xv_list_image_formats_reply_t,
    ) -> xcb_xv_image_format_info_iterator_t {
        sym!(self, xcb_xv_list_image_formats_format_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_list_image_formats_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xv_list_image_formats_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xv_list_image_formats_reply_t {
        sym!(self, xcb_xv_list_image_formats_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xv_list_image_formats(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
    ) -> xcb_xv_list_image_formats_cookie_t {
        sym!(self, xcb_xv_list_image_formats)(c, port)
    }

    #[inline]
    pub unsafe fn xcb_xv_list_image_formats_unchecked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
    ) -> xcb_xv_list_image_formats_cookie_t {
        sym!(self, xcb_xv_list_image_formats_unchecked)(c, port)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_image_attributes_pitches(
        &self,
        R: *const xcb_xv_query_image_attributes_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_xv_query_image_attributes_pitches)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_image_attributes_pitches_length(
        &self,
        R: *const xcb_xv_query_image_attributes_reply_t,
    ) -> c_int {
        sym!(self, xcb_xv_query_image_attributes_pitches_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_image_attributes_pitches_end(
        &self,
        R: *const xcb_xv_query_image_attributes_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_query_image_attributes_pitches_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_image_attributes_offsets(
        &self,
        R: *const xcb_xv_query_image_attributes_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_xv_query_image_attributes_offsets)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_image_attributes_offsets_length(
        &self,
        R: *const xcb_xv_query_image_attributes_reply_t,
    ) -> c_int {
        sym!(self, xcb_xv_query_image_attributes_offsets_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_image_attributes_offsets_end(
        &self,
        R: *const xcb_xv_query_image_attributes_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xv_query_image_attributes_offsets_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_image_attributes_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xv_query_image_attributes_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xv_query_image_attributes_reply_t {
        sym!(self, xcb_xv_query_image_attributes_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_image_attributes(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        id: u32,
        width: u16,
        height: u16,
    ) -> xcb_xv_query_image_attributes_cookie_t {
        sym!(self, xcb_xv_query_image_attributes)(c, port, id, width, height)
    }

    #[inline]
    pub unsafe fn xcb_xv_query_image_attributes_unchecked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        id: u32,
        width: u16,
        height: u16,
    ) -> xcb_xv_query_image_attributes_cookie_t {
        sym!(self, xcb_xv_query_image_attributes_unchecked)(c, port, id, width, height)
    }

    #[inline]
    pub unsafe fn xcb_xv_put_image(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        id: u32,
        src_x: i16,
        src_y: i16,
        src_w: u16,
        src_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
        width: u16,
        height: u16,
        data_len: u32,
        data: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_put_image)(
            c, port, drawable, gc, id, src_x, src_y, src_w, src_h, drw_x, drw_y, drw_w, drw_h,
            width, height, data_len, data,
        )
    }

    #[inline]
    pub unsafe fn xcb_xv_put_image_checked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        id: u32,
        src_x: i16,
        src_y: i16,
        src_w: u16,
        src_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
        width: u16,
        height: u16,
        data_len: u32,
        data: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_put_image_checked)(
            c, port, drawable, gc, id, src_x, src_y, src_w, src_h, drw_x, drw_y, drw_w, drw_h,
            width, height, data_len, data,
        )
    }

    #[inline]
    pub unsafe fn xcb_xv_shm_put_image(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        shmseg: xcb_shm_seg_t,
        id: u32,
        offset: u32,
        src_x: i16,
        src_y: i16,
        src_w: u16,
        src_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
        width: u16,
        height: u16,
        send_event: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_shm_put_image)(
            c, port, drawable, gc, shmseg, id, offset, src_x, src_y, src_w, src_h, drw_x, drw_y,
            drw_w, drw_h, width, height, send_event,
        )
    }

    #[inline]
    pub unsafe fn xcb_xv_shm_put_image_checked(
        &self,
        c: *mut xcb_connection_t,
        port: xcb_xv_port_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        shmseg: xcb_shm_seg_t,
        id: u32,
        offset: u32,
        src_x: i16,
        src_y: i16,
        src_w: u16,
        src_h: u16,
        drw_x: i16,
        drw_y: i16,
        drw_w: u16,
        drw_h: u16,
        width: u16,
        height: u16,
        send_event: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xv_shm_put_image_checked)(
            c, port, drawable, gc, shmseg, id, offset, src_x, src_y, src_w, src_h, drw_x, drw_y,
            drw_w, drw_h, width, height, send_event,
        )
    }
}
