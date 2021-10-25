// This file was generated using generate.py. Do not edit.

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

pub type xcb_x_print_string8_t = c_char;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_string8_iterator_t {
    pub data: *mut xcb_x_print_string8_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_x_print_string8_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_printer_t {
    pub name_len: u32,
    pub desc_len: u32,
}

impl Default for xcb_x_print_printer_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_printer_iterator_t {
    pub data: *mut xcb_x_print_printer_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_x_print_printer_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_x_print_pcontext_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_pcontext_iterator_t {
    pub data: *mut xcb_x_print_pcontext_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_x_print_pcontext_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_x_print_get_doc_t = u32;
pub const XCB_X_PRINT_GET_DOC_FINISHED: xcb_x_print_get_doc_t = 0;
pub const XCB_X_PRINT_GET_DOC_SECOND_CONSUMER: xcb_x_print_get_doc_t = 1;

pub type xcb_x_print_ev_mask_t = u32;
pub const XCB_X_PRINT_EV_MASK_NO_EVENT_MASK: xcb_x_print_ev_mask_t = 0;
pub const XCB_X_PRINT_EV_MASK_PRINT_MASK: xcb_x_print_ev_mask_t = 1;
pub const XCB_X_PRINT_EV_MASK_ATTRIBUTE_MASK: xcb_x_print_ev_mask_t = 2;

pub type xcb_x_print_detail_t = u32;
pub const XCB_X_PRINT_DETAIL_START_JOB_NOTIFY: xcb_x_print_detail_t = 1;
pub const XCB_X_PRINT_DETAIL_END_JOB_NOTIFY: xcb_x_print_detail_t = 2;
pub const XCB_X_PRINT_DETAIL_START_DOC_NOTIFY: xcb_x_print_detail_t = 3;
pub const XCB_X_PRINT_DETAIL_END_DOC_NOTIFY: xcb_x_print_detail_t = 4;
pub const XCB_X_PRINT_DETAIL_START_PAGE_NOTIFY: xcb_x_print_detail_t = 5;
pub const XCB_X_PRINT_DETAIL_END_PAGE_NOTIFY: xcb_x_print_detail_t = 6;

pub type xcb_x_print_attr_t = u32;
pub const XCB_X_PRINT_ATTR_JOB_ATTR: xcb_x_print_attr_t = 1;
pub const XCB_X_PRINT_ATTR_DOC_ATTR: xcb_x_print_attr_t = 2;
pub const XCB_X_PRINT_ATTR_PAGE_ATTR: xcb_x_print_attr_t = 3;
pub const XCB_X_PRINT_ATTR_PRINTER_ATTR: xcb_x_print_attr_t = 4;
pub const XCB_X_PRINT_ATTR_SERVER_ATTR: xcb_x_print_attr_t = 5;
pub const XCB_X_PRINT_ATTR_MEDIUM_ATTR: xcb_x_print_attr_t = 6;
pub const XCB_X_PRINT_ATTR_SPOOLER_ATTR: xcb_x_print_attr_t = 7;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_query_version_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_x_print_print_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_print_query_version.
pub const XCB_X_PRINT_PRINT_QUERY_VERSION: u8 = 0i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_x_print_print_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u16,
    pub minor_version: u16,
}

impl Default for xcb_x_print_print_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_printer_list_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_x_print_print_get_printer_list_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_print_get_printer_list.
pub const XCB_X_PRINT_PRINT_GET_PRINTER_LIST: u8 = 1i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_printer_list_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub printer_name_len: u32,
    pub locale_len: u32,
}

impl Default for xcb_x_print_print_get_printer_list_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_printer_list_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub list_count: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_x_print_print_get_printer_list_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_print_rehash_printer_list.
pub const XCB_X_PRINT_PRINT_REHASH_PRINTER_LIST: u8 = 20i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_rehash_printer_list_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_x_print_print_rehash_printer_list_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_create_context.
pub const XCB_X_PRINT_CREATE_CONTEXT: u8 = 2i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_create_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_id: u32,
    pub printer_name_len: u32,
    pub locale_len: u32,
}

impl Default for xcb_x_print_create_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_print_set_context.
pub const XCB_X_PRINT_PRINT_SET_CONTEXT: u8 = 3i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_set_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: u32,
}

impl Default for xcb_x_print_print_set_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_context_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_x_print_print_get_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_print_get_context.
pub const XCB_X_PRINT_PRINT_GET_CONTEXT: u8 = 4i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_x_print_print_get_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: u32,
}

impl Default for xcb_x_print_print_get_context_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_print_destroy_context.
pub const XCB_X_PRINT_PRINT_DESTROY_CONTEXT: u8 = 5i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_destroy_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: u32,
}

impl Default for xcb_x_print_print_destroy_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_screen_of_context_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_x_print_print_get_screen_of_context_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_print_get_screen_of_context.
pub const XCB_X_PRINT_PRINT_GET_SCREEN_OF_CONTEXT: u8 = 6i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_screen_of_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_x_print_print_get_screen_of_context_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_screen_of_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: xcb_window_t,
}

impl Default for xcb_x_print_print_get_screen_of_context_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_print_start_job.
pub const XCB_X_PRINT_PRINT_START_JOB: u8 = 7i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_start_job_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub output_mode: u8,
}

impl Default for xcb_x_print_print_start_job_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_print_end_job.
pub const XCB_X_PRINT_PRINT_END_JOB: u8 = 8i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_end_job_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub cancel: u8,
}

impl Default for xcb_x_print_print_end_job_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_print_start_doc.
pub const XCB_X_PRINT_PRINT_START_DOC: u8 = 9i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_start_doc_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub driver_mode: u8,
}

impl Default for xcb_x_print_print_start_doc_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_print_end_doc.
pub const XCB_X_PRINT_PRINT_END_DOC: u8 = 10i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_end_doc_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub cancel: u8,
}

impl Default for xcb_x_print_print_end_doc_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_print_put_document_data.
pub const XCB_X_PRINT_PRINT_PUT_DOCUMENT_DATA: u8 = 11i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_put_document_data_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub len_data: u32,
    pub len_fmt: u16,
    pub len_options: u16,
}

impl Default for xcb_x_print_print_put_document_data_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_document_data_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_x_print_print_get_document_data_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_print_get_document_data.
pub const XCB_X_PRINT_PRINT_GET_DOCUMENT_DATA: u8 = 12i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_document_data_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_x_print_pcontext_t,
    pub max_bytes: u32,
}

impl Default for xcb_x_print_print_get_document_data_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_document_data_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub status_code: u32,
    pub finished_flag: u32,
    pub data_len: u32,
    pub pad1: [u8; 12],
}

impl Default for xcb_x_print_print_get_document_data_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_print_start_page.
pub const XCB_X_PRINT_PRINT_START_PAGE: u8 = 13i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_start_page_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_x_print_print_start_page_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_print_end_page.
pub const XCB_X_PRINT_PRINT_END_PAGE: u8 = 14i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_end_page_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub cancel: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_x_print_print_end_page_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_print_select_input.
pub const XCB_X_PRINT_PRINT_SELECT_INPUT: u8 = 15i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_select_input_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_x_print_pcontext_t,
    pub event_mask: u32,
}

impl Default for xcb_x_print_print_select_input_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_input_selected_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_x_print_print_input_selected_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_print_input_selected.
pub const XCB_X_PRINT_PRINT_INPUT_SELECTED: u8 = 16i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_input_selected_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_x_print_pcontext_t,
}

impl Default for xcb_x_print_print_input_selected_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_input_selected_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_mask: u32,
    pub all_events_mask: u32,
}

impl Default for xcb_x_print_print_input_selected_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_attributes_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_x_print_print_get_attributes_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_print_get_attributes.
pub const XCB_X_PRINT_PRINT_GET_ATTRIBUTES: u8 = 17i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_attributes_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_x_print_pcontext_t,
    pub pool: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_x_print_print_get_attributes_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_attributes_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub string_len: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_x_print_print_get_attributes_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_one_attributes_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_x_print_print_get_one_attributes_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_print_get_one_attributes.
pub const XCB_X_PRINT_PRINT_GET_ONE_ATTRIBUTES: u8 = 19i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_one_attributes_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_x_print_pcontext_t,
    pub name_len: u32,
    pub pool: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_x_print_print_get_one_attributes_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_one_attributes_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub value_len: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_x_print_print_get_one_attributes_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_print_set_attributes.
pub const XCB_X_PRINT_PRINT_SET_ATTRIBUTES: u8 = 18i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_set_attributes_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_x_print_pcontext_t,
    pub string_len: u32,
    pub pool: u8,
    pub rule: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_x_print_print_set_attributes_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_page_dimensions_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_x_print_print_get_page_dimensions_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_print_get_page_dimensions.
pub const XCB_X_PRINT_PRINT_GET_PAGE_DIMENSIONS: u8 = 21i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_page_dimensions_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_x_print_pcontext_t,
}

impl Default for xcb_x_print_print_get_page_dimensions_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_page_dimensions_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: u16,
    pub height: u16,
    pub offset_x: u16,
    pub offset_y: u16,
    pub reproducible_width: u16,
    pub reproducible_height: u16,
}

impl Default for xcb_x_print_print_get_page_dimensions_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_query_screens_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_x_print_print_query_screens_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_print_query_screens.
pub const XCB_X_PRINT_PRINT_QUERY_SCREENS: u8 = 22i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_query_screens_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_x_print_print_query_screens_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_query_screens_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub list_count: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_x_print_print_query_screens_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_set_image_resolution_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_x_print_print_set_image_resolution_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_print_set_image_resolution.
pub const XCB_X_PRINT_PRINT_SET_IMAGE_RESOLUTION: u8 = 23i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_set_image_resolution_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_x_print_pcontext_t,
    pub image_resolution: u16,
}

impl Default for xcb_x_print_print_set_image_resolution_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_set_image_resolution_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
    pub previous_resolutions: u16,
}

impl Default for xcb_x_print_print_set_image_resolution_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_image_resolution_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_x_print_print_get_image_resolution_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_print_get_image_resolution.
pub const XCB_X_PRINT_PRINT_GET_IMAGE_RESOLUTION: u8 = 24i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_image_resolution_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_x_print_pcontext_t,
}

impl Default for xcb_x_print_print_get_image_resolution_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_image_resolution_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub image_resolution: u16,
}

impl Default for xcb_x_print_print_get_image_resolution_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_notify.
pub const XCB_X_PRINT_NOTIFY: u8 = 0i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_notify_event_t {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub context: xcb_x_print_pcontext_t,
    pub cancel: u8,
}

impl Default for xcb_x_print_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_attribut_notify.
pub const XCB_X_PRINT_ATTRIBUT_NOTIFY: u8 = 1i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_attribut_notify_event_t {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub context: xcb_x_print_pcontext_t,
}

impl Default for xcb_x_print_attribut_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_bad_context.
pub const XCB_X_PRINT_BAD_CONTEXT: u8 = 0i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_bad_context_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_x_print_bad_context_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_x_print_bad_sequence.
pub const XCB_X_PRINT_BAD_SEQUENCE: u8 = 1i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_bad_sequence_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_x_print_bad_sequence_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_xprint")]
pub(crate) struct XcbXprintXprint {
    xcb_x_print_id: LazySymbol<*mut xcb_extension_t>,
    xcb_x_print_string8_next: LazySymbol<unsafe fn(i: *mut xcb_x_print_string8_iterator_t)>,
    xcb_x_print_string8_end:
        LazySymbol<unsafe fn(i: xcb_x_print_string8_iterator_t) -> xcb_generic_iterator_t>,
    xcb_x_print_printer_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            _aux: *const xcb_x_print_printer_t,
            name: *const xcb_x_print_string8_t,
            description: *const xcb_x_print_string8_t,
        ) -> c_int,
    >,
    xcb_x_print_printer_unserialize: LazySymbol<
        unsafe fn(_buffer: *const c_void, _aux: *mut *mut xcb_x_print_printer_t) -> c_int,
    >,
    xcb_x_print_printer_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_x_print_printer_name:
        LazySymbol<unsafe fn(r: *const xcb_x_print_printer_t) -> *mut xcb_x_print_string8_t>,
    xcb_x_print_printer_name_length:
        LazySymbol<unsafe fn(r: *const xcb_x_print_printer_t) -> c_int>,
    xcb_x_print_printer_name_end:
        LazySymbol<unsafe fn(r: *const xcb_x_print_printer_t) -> xcb_generic_iterator_t>,
    xcb_x_print_printer_description:
        LazySymbol<unsafe fn(r: *const xcb_x_print_printer_t) -> *mut xcb_x_print_string8_t>,
    xcb_x_print_printer_description_length:
        LazySymbol<unsafe fn(r: *const xcb_x_print_printer_t) -> c_int>,
    xcb_x_print_printer_description_end:
        LazySymbol<unsafe fn(r: *const xcb_x_print_printer_t) -> xcb_generic_iterator_t>,
    xcb_x_print_printer_next: LazySymbol<unsafe fn(i: *mut xcb_x_print_printer_iterator_t)>,
    xcb_x_print_printer_end:
        LazySymbol<unsafe fn(i: xcb_x_print_printer_iterator_t) -> xcb_generic_iterator_t>,
    xcb_x_print_pcontext_next: LazySymbol<unsafe fn(i: *mut xcb_x_print_pcontext_iterator_t)>,
    xcb_x_print_pcontext_end:
        LazySymbol<unsafe fn(i: xcb_x_print_pcontext_iterator_t) -> xcb_generic_iterator_t>,
    xcb_x_print_print_query_version:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_x_print_print_query_version_cookie_t>,
    xcb_x_print_print_query_version_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_x_print_print_query_version_cookie_t>,
    xcb_x_print_print_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_x_print_print_query_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_x_print_print_query_version_reply_t,
    >,
    xcb_x_print_print_get_printer_list_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_x_print_print_get_printer_list: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            printer_name_len: u32,
            locale_len: u32,
            printer_name: *const xcb_x_print_string8_t,
            locale: *const xcb_x_print_string8_t,
        ) -> xcb_x_print_print_get_printer_list_cookie_t,
    >,
    xcb_x_print_print_get_printer_list_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            printer_name_len: u32,
            locale_len: u32,
            printer_name: *const xcb_x_print_string8_t,
            locale: *const xcb_x_print_string8_t,
        ) -> xcb_x_print_print_get_printer_list_cookie_t,
    >,
    xcb_x_print_print_get_printer_list_printers_length:
        LazySymbol<unsafe fn(r: *const xcb_x_print_print_get_printer_list_reply_t) -> c_int>,
    xcb_x_print_print_get_printer_list_printers_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_x_print_print_get_printer_list_reply_t,
        ) -> xcb_x_print_printer_iterator_t,
    >,
    xcb_x_print_print_get_printer_list_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_x_print_print_get_printer_list_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_x_print_print_get_printer_list_reply_t,
    >,
    xcb_x_print_print_rehash_printer_list_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    xcb_x_print_print_rehash_printer_list:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    xcb_x_print_create_context_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_x_print_create_context_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_id: u32,
            printer_name_len: u32,
            locale_len: u32,
            printer_name: *const xcb_x_print_string8_t,
            locale: *const xcb_x_print_string8_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_x_print_create_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_id: u32,
            printer_name_len: u32,
            locale_len: u32,
            printer_name: *const xcb_x_print_string8_t,
            locale: *const xcb_x_print_string8_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_x_print_create_context_printer_name: LazySymbol<
        unsafe fn(r: *const xcb_x_print_create_context_request_t) -> *mut xcb_x_print_string8_t,
    >,
    xcb_x_print_create_context_printer_name_length:
        LazySymbol<unsafe fn(r: *const xcb_x_print_create_context_request_t) -> c_int>,
    xcb_x_print_create_context_printer_name_end: LazySymbol<
        unsafe fn(r: *const xcb_x_print_create_context_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_x_print_create_context_locale: LazySymbol<
        unsafe fn(r: *const xcb_x_print_create_context_request_t) -> *mut xcb_x_print_string8_t,
    >,
    xcb_x_print_create_context_locale_length:
        LazySymbol<unsafe fn(r: *const xcb_x_print_create_context_request_t) -> c_int>,
    xcb_x_print_create_context_locale_end: LazySymbol<
        unsafe fn(r: *const xcb_x_print_create_context_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_x_print_print_set_context_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: u32) -> xcb_void_cookie_t>,
    xcb_x_print_print_set_context:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: u32) -> xcb_void_cookie_t>,
    xcb_x_print_print_get_context:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_x_print_print_get_context_cookie_t>,
    xcb_x_print_print_get_context_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_x_print_print_get_context_cookie_t>,
    xcb_x_print_print_get_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_x_print_print_get_context_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_x_print_print_get_context_reply_t,
    >,
    xcb_x_print_print_destroy_context_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: u32) -> xcb_void_cookie_t>,
    xcb_x_print_print_destroy_context:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: u32) -> xcb_void_cookie_t>,
    xcb_x_print_print_get_screen_of_context: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_x_print_print_get_screen_of_context_cookie_t,
    >,
    xcb_x_print_print_get_screen_of_context_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_x_print_print_get_screen_of_context_cookie_t,
    >,
    xcb_x_print_print_get_screen_of_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_x_print_print_get_screen_of_context_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_x_print_print_get_screen_of_context_reply_t,
    >,
    xcb_x_print_print_start_job_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, output_mode: u8) -> xcb_void_cookie_t>,
    xcb_x_print_print_start_job:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, output_mode: u8) -> xcb_void_cookie_t>,
    xcb_x_print_print_end_job_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cancel: u8) -> xcb_void_cookie_t>,
    xcb_x_print_print_end_job:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cancel: u8) -> xcb_void_cookie_t>,
    xcb_x_print_print_start_doc_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, driver_mode: u8) -> xcb_void_cookie_t>,
    xcb_x_print_print_start_doc:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, driver_mode: u8) -> xcb_void_cookie_t>,
    xcb_x_print_print_end_doc_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cancel: u8) -> xcb_void_cookie_t>,
    xcb_x_print_print_end_doc:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cancel: u8) -> xcb_void_cookie_t>,
    xcb_x_print_print_put_document_data_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_x_print_print_put_document_data_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            len_data: u32,
            len_fmt: u16,
            len_options: u16,
            data: *const u8,
            doc_format: *const xcb_x_print_string8_t,
            options: *const xcb_x_print_string8_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_x_print_print_put_document_data: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            len_data: u32,
            len_fmt: u16,
            len_options: u16,
            data: *const u8,
            doc_format: *const xcb_x_print_string8_t,
            options: *const xcb_x_print_string8_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_x_print_print_put_document_data_data:
        LazySymbol<unsafe fn(r: *const xcb_x_print_print_put_document_data_request_t) -> *mut u8>,
    xcb_x_print_print_put_document_data_data_length:
        LazySymbol<unsafe fn(r: *const xcb_x_print_print_put_document_data_request_t) -> c_int>,
    xcb_x_print_print_put_document_data_data_end: LazySymbol<
        unsafe fn(
            r: *const xcb_x_print_print_put_document_data_request_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_x_print_print_put_document_data_doc_format: LazySymbol<
        unsafe fn(
            r: *const xcb_x_print_print_put_document_data_request_t,
        ) -> *mut xcb_x_print_string8_t,
    >,
    xcb_x_print_print_put_document_data_doc_format_length:
        LazySymbol<unsafe fn(r: *const xcb_x_print_print_put_document_data_request_t) -> c_int>,
    xcb_x_print_print_put_document_data_doc_format_end: LazySymbol<
        unsafe fn(
            r: *const xcb_x_print_print_put_document_data_request_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_x_print_print_put_document_data_options: LazySymbol<
        unsafe fn(
            r: *const xcb_x_print_print_put_document_data_request_t,
        ) -> *mut xcb_x_print_string8_t,
    >,
    xcb_x_print_print_put_document_data_options_length:
        LazySymbol<unsafe fn(r: *const xcb_x_print_print_put_document_data_request_t) -> c_int>,
    xcb_x_print_print_put_document_data_options_end: LazySymbol<
        unsafe fn(
            r: *const xcb_x_print_print_put_document_data_request_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_x_print_print_get_document_data_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_x_print_print_get_document_data: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
            max_bytes: u32,
        ) -> xcb_x_print_print_get_document_data_cookie_t,
    >,
    xcb_x_print_print_get_document_data_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
            max_bytes: u32,
        ) -> xcb_x_print_print_get_document_data_cookie_t,
    >,
    xcb_x_print_print_get_document_data_data:
        LazySymbol<unsafe fn(r: *const xcb_x_print_print_get_document_data_reply_t) -> *mut u8>,
    xcb_x_print_print_get_document_data_data_length:
        LazySymbol<unsafe fn(r: *const xcb_x_print_print_get_document_data_reply_t) -> c_int>,
    xcb_x_print_print_get_document_data_data_end: LazySymbol<
        unsafe fn(r: *const xcb_x_print_print_get_document_data_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_x_print_print_get_document_data_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_x_print_print_get_document_data_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_x_print_print_get_document_data_reply_t,
    >,
    xcb_x_print_print_start_page_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_x_print_print_start_page:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_x_print_print_end_page_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cancel: u8) -> xcb_void_cookie_t>,
    xcb_x_print_print_end_page:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cancel: u8) -> xcb_void_cookie_t>,
    xcb_x_print_print_select_input_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
            event_mask: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_x_print_print_select_input: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
            event_mask: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_x_print_print_input_selected: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
        ) -> xcb_x_print_print_input_selected_cookie_t,
    >,
    xcb_x_print_print_input_selected_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
        ) -> xcb_x_print_print_input_selected_cookie_t,
    >,
    xcb_x_print_print_input_selected_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_x_print_print_input_selected_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_x_print_print_input_selected_reply_t,
    >,
    xcb_x_print_print_get_attributes_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_x_print_print_get_attributes: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
            pool: u8,
        ) -> xcb_x_print_print_get_attributes_cookie_t,
    >,
    xcb_x_print_print_get_attributes_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
            pool: u8,
        ) -> xcb_x_print_print_get_attributes_cookie_t,
    >,
    xcb_x_print_print_get_attributes_attributes: LazySymbol<
        unsafe fn(r: *const xcb_x_print_print_get_attributes_reply_t) -> *mut xcb_x_print_string8_t,
    >,
    xcb_x_print_print_get_attributes_attributes_length:
        LazySymbol<unsafe fn(r: *const xcb_x_print_print_get_attributes_reply_t) -> c_int>,
    xcb_x_print_print_get_attributes_attributes_end: LazySymbol<
        unsafe fn(r: *const xcb_x_print_print_get_attributes_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_x_print_print_get_attributes_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_x_print_print_get_attributes_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_x_print_print_get_attributes_reply_t,
    >,
    xcb_x_print_print_get_one_attributes_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_x_print_print_get_one_attributes: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
            name_len: u32,
            pool: u8,
            name: *const xcb_x_print_string8_t,
        ) -> xcb_x_print_print_get_one_attributes_cookie_t,
    >,
    xcb_x_print_print_get_one_attributes_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
            name_len: u32,
            pool: u8,
            name: *const xcb_x_print_string8_t,
        ) -> xcb_x_print_print_get_one_attributes_cookie_t,
    >,
    xcb_x_print_print_get_one_attributes_value: LazySymbol<
        unsafe fn(
            r: *const xcb_x_print_print_get_one_attributes_reply_t,
        ) -> *mut xcb_x_print_string8_t,
    >,
    xcb_x_print_print_get_one_attributes_value_length:
        LazySymbol<unsafe fn(r: *const xcb_x_print_print_get_one_attributes_reply_t) -> c_int>,
    xcb_x_print_print_get_one_attributes_value_end: LazySymbol<
        unsafe fn(r: *const xcb_x_print_print_get_one_attributes_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_x_print_print_get_one_attributes_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_x_print_print_get_one_attributes_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_x_print_print_get_one_attributes_reply_t,
    >,
    xcb_x_print_print_set_attributes_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, attributes_len: u32) -> c_int>,
    xcb_x_print_print_set_attributes_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
            string_len: u32,
            pool: u8,
            rule: u8,
            attributes_len: u32,
            attributes: *const xcb_x_print_string8_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_x_print_print_set_attributes: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
            string_len: u32,
            pool: u8,
            rule: u8,
            attributes_len: u32,
            attributes: *const xcb_x_print_string8_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_x_print_print_set_attributes_attributes: LazySymbol<
        unsafe fn(
            r: *const xcb_x_print_print_set_attributes_request_t,
        ) -> *mut xcb_x_print_string8_t,
    >,
    xcb_x_print_print_set_attributes_attributes_length:
        LazySymbol<unsafe fn(r: *const xcb_x_print_print_set_attributes_request_t) -> c_int>,
    xcb_x_print_print_set_attributes_attributes_end: LazySymbol<
        unsafe fn(r: *const xcb_x_print_print_set_attributes_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_x_print_print_get_page_dimensions: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
        ) -> xcb_x_print_print_get_page_dimensions_cookie_t,
    >,
    xcb_x_print_print_get_page_dimensions_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
        ) -> xcb_x_print_print_get_page_dimensions_cookie_t,
    >,
    xcb_x_print_print_get_page_dimensions_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_x_print_print_get_page_dimensions_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_x_print_print_get_page_dimensions_reply_t,
    >,
    xcb_x_print_print_query_screens_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_x_print_print_query_screens:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_x_print_print_query_screens_cookie_t>,
    xcb_x_print_print_query_screens_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_x_print_print_query_screens_cookie_t>,
    xcb_x_print_print_query_screens_roots: LazySymbol<
        unsafe fn(r: *const xcb_x_print_print_query_screens_reply_t) -> *mut xcb_window_t,
    >,
    xcb_x_print_print_query_screens_roots_length:
        LazySymbol<unsafe fn(r: *const xcb_x_print_print_query_screens_reply_t) -> c_int>,
    xcb_x_print_print_query_screens_roots_end: LazySymbol<
        unsafe fn(r: *const xcb_x_print_print_query_screens_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_x_print_print_query_screens_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_x_print_print_query_screens_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_x_print_print_query_screens_reply_t,
    >,
    xcb_x_print_print_set_image_resolution: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
            image_resolution: u16,
        ) -> xcb_x_print_print_set_image_resolution_cookie_t,
    >,
    xcb_x_print_print_set_image_resolution_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
            image_resolution: u16,
        ) -> xcb_x_print_print_set_image_resolution_cookie_t,
    >,
    xcb_x_print_print_set_image_resolution_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_x_print_print_set_image_resolution_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_x_print_print_set_image_resolution_reply_t,
    >,
    xcb_x_print_print_get_image_resolution: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
        ) -> xcb_x_print_print_get_image_resolution_cookie_t,
    >,
    xcb_x_print_print_get_image_resolution_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
        ) -> xcb_x_print_print_get_image_resolution_cookie_t,
    >,
    xcb_x_print_print_get_image_resolution_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_x_print_print_get_image_resolution_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_x_print_print_get_image_resolution_reply_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self
            .xprint
            .$f
            .get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .xprint
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_xprint")]
impl XcbXprint {
    pub fn xcb_x_print_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_x_print_id) }
    }

    /// Returns `true` iff the symbol `xcb_x_print_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_id(&self) -> bool {
        has_sym!(self, xcb_x_print_id)
    }

    pub unsafe fn xcb_x_print_string8_next(&self, i: *mut xcb_x_print_string8_iterator_t) {
        sym!(self, xcb_x_print_string8_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_x_print_string8_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_string8_next(&self) -> bool {
        has_sym!(self, xcb_x_print_string8_next)
    }

    pub unsafe fn xcb_x_print_string8_end(
        &self,
        i: xcb_x_print_string8_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_x_print_string8_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_x_print_string8_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_string8_end(&self) -> bool {
        has_sym!(self, xcb_x_print_string8_end)
    }

    pub unsafe fn xcb_x_print_printer_serialize(
        &self,
        _buffer: *mut *mut c_void,
        _aux: *const xcb_x_print_printer_t,
        name: *const xcb_x_print_string8_t,
        description: *const xcb_x_print_string8_t,
    ) -> c_int {
        sym!(self, xcb_x_print_printer_serialize)(_buffer, _aux, name, description)
    }

    /// Returns `true` iff the symbol `xcb_x_print_printer_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_printer_serialize(&self) -> bool {
        has_sym!(self, xcb_x_print_printer_serialize)
    }

    pub unsafe fn xcb_x_print_printer_unserialize(
        &self,
        _buffer: *const c_void,
        _aux: *mut *mut xcb_x_print_printer_t,
    ) -> c_int {
        sym!(self, xcb_x_print_printer_unserialize)(_buffer, _aux)
    }

    /// Returns `true` iff the symbol `xcb_x_print_printer_unserialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_printer_unserialize(&self) -> bool {
        has_sym!(self, xcb_x_print_printer_unserialize)
    }

    pub unsafe fn xcb_x_print_printer_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_x_print_printer_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_x_print_printer_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_printer_sizeof(&self) -> bool {
        has_sym!(self, xcb_x_print_printer_sizeof)
    }

    pub unsafe fn xcb_x_print_printer_name(
        &self,
        r: *const xcb_x_print_printer_t,
    ) -> *mut xcb_x_print_string8_t {
        sym!(self, xcb_x_print_printer_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_printer_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_printer_name(&self) -> bool {
        has_sym!(self, xcb_x_print_printer_name)
    }

    pub unsafe fn xcb_x_print_printer_name_length(&self, r: *const xcb_x_print_printer_t) -> c_int {
        sym!(self, xcb_x_print_printer_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_printer_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_printer_name_length(&self) -> bool {
        has_sym!(self, xcb_x_print_printer_name_length)
    }

    pub unsafe fn xcb_x_print_printer_name_end(
        &self,
        r: *const xcb_x_print_printer_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_x_print_printer_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_printer_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_printer_name_end(&self) -> bool {
        has_sym!(self, xcb_x_print_printer_name_end)
    }

    pub unsafe fn xcb_x_print_printer_description(
        &self,
        r: *const xcb_x_print_printer_t,
    ) -> *mut xcb_x_print_string8_t {
        sym!(self, xcb_x_print_printer_description)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_printer_description` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_printer_description(&self) -> bool {
        has_sym!(self, xcb_x_print_printer_description)
    }

    pub unsafe fn xcb_x_print_printer_description_length(
        &self,
        r: *const xcb_x_print_printer_t,
    ) -> c_int {
        sym!(self, xcb_x_print_printer_description_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_printer_description_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_printer_description_length(&self) -> bool {
        has_sym!(self, xcb_x_print_printer_description_length)
    }

    pub unsafe fn xcb_x_print_printer_description_end(
        &self,
        r: *const xcb_x_print_printer_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_x_print_printer_description_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_printer_description_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_printer_description_end(&self) -> bool {
        has_sym!(self, xcb_x_print_printer_description_end)
    }

    pub unsafe fn xcb_x_print_printer_next(&self, i: *mut xcb_x_print_printer_iterator_t) {
        sym!(self, xcb_x_print_printer_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_x_print_printer_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_printer_next(&self) -> bool {
        has_sym!(self, xcb_x_print_printer_next)
    }

    pub unsafe fn xcb_x_print_printer_end(
        &self,
        i: xcb_x_print_printer_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_x_print_printer_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_x_print_printer_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_printer_end(&self) -> bool {
        has_sym!(self, xcb_x_print_printer_end)
    }

    pub unsafe fn xcb_x_print_pcontext_next(&self, i: *mut xcb_x_print_pcontext_iterator_t) {
        sym!(self, xcb_x_print_pcontext_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_x_print_pcontext_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_pcontext_next(&self) -> bool {
        has_sym!(self, xcb_x_print_pcontext_next)
    }

    pub unsafe fn xcb_x_print_pcontext_end(
        &self,
        i: xcb_x_print_pcontext_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_x_print_pcontext_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_x_print_pcontext_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_pcontext_end(&self) -> bool {
        has_sym!(self, xcb_x_print_pcontext_end)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_x_print_print_query_version(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_x_print_print_query_version_cookie_t {
        sym!(self, xcb_x_print_print_query_version)(c)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_query_version(&self) -> bool {
        has_sym!(self, xcb_x_print_print_query_version)
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
    pub unsafe fn xcb_x_print_print_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_x_print_print_query_version_cookie_t {
        sym!(self, xcb_x_print_print_query_version_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_query_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_x_print_print_query_version_unchecked)
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
     * xcb_x_print_print_query_version_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_x_print_print_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_x_print_print_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_x_print_print_query_version_reply_t {
        sym!(self, xcb_x_print_print_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_x_print_print_query_version_reply)
    }

    pub unsafe fn xcb_x_print_print_get_printer_list_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_x_print_print_get_printer_list_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_printer_list_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_printer_list_sizeof(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_printer_list_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_x_print_print_get_printer_list(
        &self,
        c: *mut xcb_connection_t,
        printer_name_len: u32,
        locale_len: u32,
        printer_name: *const xcb_x_print_string8_t,
        locale: *const xcb_x_print_string8_t,
    ) -> xcb_x_print_print_get_printer_list_cookie_t {
        sym!(self, xcb_x_print_print_get_printer_list)(
            c,
            printer_name_len,
            locale_len,
            printer_name,
            locale,
        )
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_printer_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_printer_list(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_printer_list)
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
    pub unsafe fn xcb_x_print_print_get_printer_list_unchecked(
        &self,
        c: *mut xcb_connection_t,
        printer_name_len: u32,
        locale_len: u32,
        printer_name: *const xcb_x_print_string8_t,
        locale: *const xcb_x_print_string8_t,
    ) -> xcb_x_print_print_get_printer_list_cookie_t {
        sym!(self, xcb_x_print_print_get_printer_list_unchecked)(
            c,
            printer_name_len,
            locale_len,
            printer_name,
            locale,
        )
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_printer_list_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_printer_list_unchecked(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_printer_list_unchecked)
    }

    pub unsafe fn xcb_x_print_print_get_printer_list_printers_length(
        &self,
        r: *const xcb_x_print_print_get_printer_list_reply_t,
    ) -> c_int {
        sym!(self, xcb_x_print_print_get_printer_list_printers_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_printer_list_printers_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_printer_list_printers_length(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_printer_list_printers_length)
    }

    pub unsafe fn xcb_x_print_print_get_printer_list_printers_iterator(
        &self,
        r: *const xcb_x_print_print_get_printer_list_reply_t,
    ) -> xcb_x_print_printer_iterator_t {
        sym!(self, xcb_x_print_print_get_printer_list_printers_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_printer_list_printers_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_printer_list_printers_iterator(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_printer_list_printers_iterator)
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
     * xcb_x_print_print_get_printer_list_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_x_print_print_get_printer_list_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_x_print_print_get_printer_list_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_x_print_print_get_printer_list_reply_t {
        sym!(self, xcb_x_print_print_get_printer_list_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_printer_list_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_printer_list_reply(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_printer_list_reply)
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
    pub unsafe fn xcb_x_print_print_rehash_printer_list_checked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_print_rehash_printer_list_checked)(c)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_rehash_printer_list_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_rehash_printer_list_checked(&self) -> bool {
        has_sym!(self, xcb_x_print_print_rehash_printer_list_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_x_print_print_rehash_printer_list(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_print_rehash_printer_list)(c)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_rehash_printer_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_rehash_printer_list(&self) -> bool {
        has_sym!(self, xcb_x_print_print_rehash_printer_list)
    }

    pub unsafe fn xcb_x_print_create_context_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_x_print_create_context_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_x_print_create_context_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_create_context_sizeof(&self) -> bool {
        has_sym!(self, xcb_x_print_create_context_sizeof)
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
    pub unsafe fn xcb_x_print_create_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context_id: u32,
        printer_name_len: u32,
        locale_len: u32,
        printer_name: *const xcb_x_print_string8_t,
        locale: *const xcb_x_print_string8_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_create_context_checked)(
            c,
            context_id,
            printer_name_len,
            locale_len,
            printer_name,
            locale,
        )
    }

    /// Returns `true` iff the symbol `xcb_x_print_create_context_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_create_context_checked(&self) -> bool {
        has_sym!(self, xcb_x_print_create_context_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_x_print_create_context(
        &self,
        c: *mut xcb_connection_t,
        context_id: u32,
        printer_name_len: u32,
        locale_len: u32,
        printer_name: *const xcb_x_print_string8_t,
        locale: *const xcb_x_print_string8_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_create_context)(
            c,
            context_id,
            printer_name_len,
            locale_len,
            printer_name,
            locale,
        )
    }

    /// Returns `true` iff the symbol `xcb_x_print_create_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_create_context(&self) -> bool {
        has_sym!(self, xcb_x_print_create_context)
    }

    pub unsafe fn xcb_x_print_create_context_printer_name(
        &self,
        r: *const xcb_x_print_create_context_request_t,
    ) -> *mut xcb_x_print_string8_t {
        sym!(self, xcb_x_print_create_context_printer_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_create_context_printer_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_create_context_printer_name(&self) -> bool {
        has_sym!(self, xcb_x_print_create_context_printer_name)
    }

    pub unsafe fn xcb_x_print_create_context_printer_name_length(
        &self,
        r: *const xcb_x_print_create_context_request_t,
    ) -> c_int {
        sym!(self, xcb_x_print_create_context_printer_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_create_context_printer_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_create_context_printer_name_length(&self) -> bool {
        has_sym!(self, xcb_x_print_create_context_printer_name_length)
    }

    pub unsafe fn xcb_x_print_create_context_printer_name_end(
        &self,
        r: *const xcb_x_print_create_context_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_x_print_create_context_printer_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_create_context_printer_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_create_context_printer_name_end(&self) -> bool {
        has_sym!(self, xcb_x_print_create_context_printer_name_end)
    }

    pub unsafe fn xcb_x_print_create_context_locale(
        &self,
        r: *const xcb_x_print_create_context_request_t,
    ) -> *mut xcb_x_print_string8_t {
        sym!(self, xcb_x_print_create_context_locale)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_create_context_locale` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_create_context_locale(&self) -> bool {
        has_sym!(self, xcb_x_print_create_context_locale)
    }

    pub unsafe fn xcb_x_print_create_context_locale_length(
        &self,
        r: *const xcb_x_print_create_context_request_t,
    ) -> c_int {
        sym!(self, xcb_x_print_create_context_locale_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_create_context_locale_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_create_context_locale_length(&self) -> bool {
        has_sym!(self, xcb_x_print_create_context_locale_length)
    }

    pub unsafe fn xcb_x_print_create_context_locale_end(
        &self,
        r: *const xcb_x_print_create_context_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_x_print_create_context_locale_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_create_context_locale_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_create_context_locale_end(&self) -> bool {
        has_sym!(self, xcb_x_print_create_context_locale_end)
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
    pub unsafe fn xcb_x_print_print_set_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_print_set_context_checked)(c, context)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_set_context_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_set_context_checked(&self) -> bool {
        has_sym!(self, xcb_x_print_print_set_context_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_x_print_print_set_context(
        &self,
        c: *mut xcb_connection_t,
        context: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_print_set_context)(c, context)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_set_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_set_context(&self) -> bool {
        has_sym!(self, xcb_x_print_print_set_context)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_x_print_print_get_context(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_x_print_print_get_context_cookie_t {
        sym!(self, xcb_x_print_print_get_context)(c)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_context(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_context)
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
    pub unsafe fn xcb_x_print_print_get_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_x_print_print_get_context_cookie_t {
        sym!(self, xcb_x_print_print_get_context_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_context_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_context_unchecked(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_context_unchecked)
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
     * xcb_x_print_print_get_context_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_x_print_print_get_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_x_print_print_get_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_x_print_print_get_context_reply_t {
        sym!(self, xcb_x_print_print_get_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_context_reply(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_context_reply)
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
    pub unsafe fn xcb_x_print_print_destroy_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_print_destroy_context_checked)(c, context)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_destroy_context_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_destroy_context_checked(&self) -> bool {
        has_sym!(self, xcb_x_print_print_destroy_context_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_x_print_print_destroy_context(
        &self,
        c: *mut xcb_connection_t,
        context: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_print_destroy_context)(c, context)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_destroy_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_destroy_context(&self) -> bool {
        has_sym!(self, xcb_x_print_print_destroy_context)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_x_print_print_get_screen_of_context(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_x_print_print_get_screen_of_context_cookie_t {
        sym!(self, xcb_x_print_print_get_screen_of_context)(c)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_screen_of_context` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_screen_of_context(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_screen_of_context)
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
    pub unsafe fn xcb_x_print_print_get_screen_of_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_x_print_print_get_screen_of_context_cookie_t {
        sym!(self, xcb_x_print_print_get_screen_of_context_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_screen_of_context_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_screen_of_context_unchecked(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_screen_of_context_unchecked)
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
     * xcb_x_print_print_get_screen_of_context_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_x_print_print_get_screen_of_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_x_print_print_get_screen_of_context_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_x_print_print_get_screen_of_context_reply_t {
        sym!(self, xcb_x_print_print_get_screen_of_context_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_screen_of_context_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_screen_of_context_reply(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_screen_of_context_reply)
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
    pub unsafe fn xcb_x_print_print_start_job_checked(
        &self,
        c: *mut xcb_connection_t,
        output_mode: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_print_start_job_checked)(c, output_mode)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_start_job_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_start_job_checked(&self) -> bool {
        has_sym!(self, xcb_x_print_print_start_job_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_x_print_print_start_job(
        &self,
        c: *mut xcb_connection_t,
        output_mode: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_print_start_job)(c, output_mode)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_start_job` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_start_job(&self) -> bool {
        has_sym!(self, xcb_x_print_print_start_job)
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
    pub unsafe fn xcb_x_print_print_end_job_checked(
        &self,
        c: *mut xcb_connection_t,
        cancel: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_print_end_job_checked)(c, cancel)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_end_job_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_end_job_checked(&self) -> bool {
        has_sym!(self, xcb_x_print_print_end_job_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_x_print_print_end_job(
        &self,
        c: *mut xcb_connection_t,
        cancel: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_print_end_job)(c, cancel)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_end_job` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_end_job(&self) -> bool {
        has_sym!(self, xcb_x_print_print_end_job)
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
    pub unsafe fn xcb_x_print_print_start_doc_checked(
        &self,
        c: *mut xcb_connection_t,
        driver_mode: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_print_start_doc_checked)(c, driver_mode)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_start_doc_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_start_doc_checked(&self) -> bool {
        has_sym!(self, xcb_x_print_print_start_doc_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_x_print_print_start_doc(
        &self,
        c: *mut xcb_connection_t,
        driver_mode: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_print_start_doc)(c, driver_mode)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_start_doc` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_start_doc(&self) -> bool {
        has_sym!(self, xcb_x_print_print_start_doc)
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
    pub unsafe fn xcb_x_print_print_end_doc_checked(
        &self,
        c: *mut xcb_connection_t,
        cancel: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_print_end_doc_checked)(c, cancel)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_end_doc_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_end_doc_checked(&self) -> bool {
        has_sym!(self, xcb_x_print_print_end_doc_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_x_print_print_end_doc(
        &self,
        c: *mut xcb_connection_t,
        cancel: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_print_end_doc)(c, cancel)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_end_doc` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_end_doc(&self) -> bool {
        has_sym!(self, xcb_x_print_print_end_doc)
    }

    pub unsafe fn xcb_x_print_print_put_document_data_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_x_print_print_put_document_data_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_put_document_data_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_put_document_data_sizeof(&self) -> bool {
        has_sym!(self, xcb_x_print_print_put_document_data_sizeof)
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
    pub unsafe fn xcb_x_print_print_put_document_data_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        len_data: u32,
        len_fmt: u16,
        len_options: u16,
        data: *const u8,
        doc_format: *const xcb_x_print_string8_t,
        options: *const xcb_x_print_string8_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_print_put_document_data_checked)(
            c,
            drawable,
            len_data,
            len_fmt,
            len_options,
            data,
            doc_format,
            options,
        )
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_put_document_data_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_put_document_data_checked(&self) -> bool {
        has_sym!(self, xcb_x_print_print_put_document_data_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_x_print_print_put_document_data(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        len_data: u32,
        len_fmt: u16,
        len_options: u16,
        data: *const u8,
        doc_format: *const xcb_x_print_string8_t,
        options: *const xcb_x_print_string8_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_print_put_document_data)(
            c,
            drawable,
            len_data,
            len_fmt,
            len_options,
            data,
            doc_format,
            options,
        )
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_put_document_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_put_document_data(&self) -> bool {
        has_sym!(self, xcb_x_print_print_put_document_data)
    }

    pub unsafe fn xcb_x_print_print_put_document_data_data(
        &self,
        r: *const xcb_x_print_print_put_document_data_request_t,
    ) -> *mut u8 {
        sym!(self, xcb_x_print_print_put_document_data_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_put_document_data_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_put_document_data_data(&self) -> bool {
        has_sym!(self, xcb_x_print_print_put_document_data_data)
    }

    pub unsafe fn xcb_x_print_print_put_document_data_data_length(
        &self,
        r: *const xcb_x_print_print_put_document_data_request_t,
    ) -> c_int {
        sym!(self, xcb_x_print_print_put_document_data_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_put_document_data_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_put_document_data_data_length(&self) -> bool {
        has_sym!(self, xcb_x_print_print_put_document_data_data_length)
    }

    pub unsafe fn xcb_x_print_print_put_document_data_data_end(
        &self,
        r: *const xcb_x_print_print_put_document_data_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_x_print_print_put_document_data_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_put_document_data_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_put_document_data_data_end(&self) -> bool {
        has_sym!(self, xcb_x_print_print_put_document_data_data_end)
    }

    pub unsafe fn xcb_x_print_print_put_document_data_doc_format(
        &self,
        r: *const xcb_x_print_print_put_document_data_request_t,
    ) -> *mut xcb_x_print_string8_t {
        sym!(self, xcb_x_print_print_put_document_data_doc_format)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_put_document_data_doc_format` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_put_document_data_doc_format(&self) -> bool {
        has_sym!(self, xcb_x_print_print_put_document_data_doc_format)
    }

    pub unsafe fn xcb_x_print_print_put_document_data_doc_format_length(
        &self,
        r: *const xcb_x_print_print_put_document_data_request_t,
    ) -> c_int {
        sym!(self, xcb_x_print_print_put_document_data_doc_format_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_put_document_data_doc_format_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_put_document_data_doc_format_length(&self) -> bool {
        has_sym!(self, xcb_x_print_print_put_document_data_doc_format_length)
    }

    pub unsafe fn xcb_x_print_print_put_document_data_doc_format_end(
        &self,
        r: *const xcb_x_print_print_put_document_data_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_x_print_print_put_document_data_doc_format_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_put_document_data_doc_format_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_put_document_data_doc_format_end(&self) -> bool {
        has_sym!(self, xcb_x_print_print_put_document_data_doc_format_end)
    }

    pub unsafe fn xcb_x_print_print_put_document_data_options(
        &self,
        r: *const xcb_x_print_print_put_document_data_request_t,
    ) -> *mut xcb_x_print_string8_t {
        sym!(self, xcb_x_print_print_put_document_data_options)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_put_document_data_options` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_put_document_data_options(&self) -> bool {
        has_sym!(self, xcb_x_print_print_put_document_data_options)
    }

    pub unsafe fn xcb_x_print_print_put_document_data_options_length(
        &self,
        r: *const xcb_x_print_print_put_document_data_request_t,
    ) -> c_int {
        sym!(self, xcb_x_print_print_put_document_data_options_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_put_document_data_options_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_put_document_data_options_length(&self) -> bool {
        has_sym!(self, xcb_x_print_print_put_document_data_options_length)
    }

    pub unsafe fn xcb_x_print_print_put_document_data_options_end(
        &self,
        r: *const xcb_x_print_print_put_document_data_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_x_print_print_put_document_data_options_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_put_document_data_options_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_put_document_data_options_end(&self) -> bool {
        has_sym!(self, xcb_x_print_print_put_document_data_options_end)
    }

    pub unsafe fn xcb_x_print_print_get_document_data_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_x_print_print_get_document_data_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_document_data_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_document_data_sizeof(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_document_data_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_x_print_print_get_document_data(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
        max_bytes: u32,
    ) -> xcb_x_print_print_get_document_data_cookie_t {
        sym!(self, xcb_x_print_print_get_document_data)(c, context, max_bytes)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_document_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_document_data(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_document_data)
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
    pub unsafe fn xcb_x_print_print_get_document_data_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
        max_bytes: u32,
    ) -> xcb_x_print_print_get_document_data_cookie_t {
        sym!(self, xcb_x_print_print_get_document_data_unchecked)(c, context, max_bytes)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_document_data_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_document_data_unchecked(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_document_data_unchecked)
    }

    pub unsafe fn xcb_x_print_print_get_document_data_data(
        &self,
        r: *const xcb_x_print_print_get_document_data_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_x_print_print_get_document_data_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_document_data_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_document_data_data(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_document_data_data)
    }

    pub unsafe fn xcb_x_print_print_get_document_data_data_length(
        &self,
        r: *const xcb_x_print_print_get_document_data_reply_t,
    ) -> c_int {
        sym!(self, xcb_x_print_print_get_document_data_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_document_data_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_document_data_data_length(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_document_data_data_length)
    }

    pub unsafe fn xcb_x_print_print_get_document_data_data_end(
        &self,
        r: *const xcb_x_print_print_get_document_data_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_x_print_print_get_document_data_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_document_data_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_document_data_data_end(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_document_data_data_end)
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
     * xcb_x_print_print_get_document_data_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_x_print_print_get_document_data_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_x_print_print_get_document_data_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_x_print_print_get_document_data_reply_t {
        sym!(self, xcb_x_print_print_get_document_data_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_document_data_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_document_data_reply(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_document_data_reply)
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
    pub unsafe fn xcb_x_print_print_start_page_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_print_start_page_checked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_start_page_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_start_page_checked(&self) -> bool {
        has_sym!(self, xcb_x_print_print_start_page_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_x_print_print_start_page(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_print_start_page)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_start_page` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_start_page(&self) -> bool {
        has_sym!(self, xcb_x_print_print_start_page)
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
    pub unsafe fn xcb_x_print_print_end_page_checked(
        &self,
        c: *mut xcb_connection_t,
        cancel: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_print_end_page_checked)(c, cancel)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_end_page_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_end_page_checked(&self) -> bool {
        has_sym!(self, xcb_x_print_print_end_page_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_x_print_print_end_page(
        &self,
        c: *mut xcb_connection_t,
        cancel: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_print_end_page)(c, cancel)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_end_page` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_end_page(&self) -> bool {
        has_sym!(self, xcb_x_print_print_end_page)
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
    pub unsafe fn xcb_x_print_print_select_input_checked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
        event_mask: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_print_select_input_checked)(c, context, event_mask)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_select_input_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_select_input_checked(&self) -> bool {
        has_sym!(self, xcb_x_print_print_select_input_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_x_print_print_select_input(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
        event_mask: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_print_select_input)(c, context, event_mask)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_select_input` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_select_input(&self) -> bool {
        has_sym!(self, xcb_x_print_print_select_input)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_x_print_print_input_selected(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
    ) -> xcb_x_print_print_input_selected_cookie_t {
        sym!(self, xcb_x_print_print_input_selected)(c, context)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_input_selected` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_input_selected(&self) -> bool {
        has_sym!(self, xcb_x_print_print_input_selected)
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
    pub unsafe fn xcb_x_print_print_input_selected_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
    ) -> xcb_x_print_print_input_selected_cookie_t {
        sym!(self, xcb_x_print_print_input_selected_unchecked)(c, context)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_input_selected_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_input_selected_unchecked(&self) -> bool {
        has_sym!(self, xcb_x_print_print_input_selected_unchecked)
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
     * xcb_x_print_print_input_selected_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_x_print_print_input_selected_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_x_print_print_input_selected_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_x_print_print_input_selected_reply_t {
        sym!(self, xcb_x_print_print_input_selected_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_input_selected_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_input_selected_reply(&self) -> bool {
        has_sym!(self, xcb_x_print_print_input_selected_reply)
    }

    pub unsafe fn xcb_x_print_print_get_attributes_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_x_print_print_get_attributes_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_attributes_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_attributes_sizeof(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_attributes_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_x_print_print_get_attributes(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
        pool: u8,
    ) -> xcb_x_print_print_get_attributes_cookie_t {
        sym!(self, xcb_x_print_print_get_attributes)(c, context, pool)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_attributes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_attributes(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_attributes)
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
    pub unsafe fn xcb_x_print_print_get_attributes_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
        pool: u8,
    ) -> xcb_x_print_print_get_attributes_cookie_t {
        sym!(self, xcb_x_print_print_get_attributes_unchecked)(c, context, pool)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_attributes_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_attributes_unchecked(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_attributes_unchecked)
    }

    pub unsafe fn xcb_x_print_print_get_attributes_attributes(
        &self,
        r: *const xcb_x_print_print_get_attributes_reply_t,
    ) -> *mut xcb_x_print_string8_t {
        sym!(self, xcb_x_print_print_get_attributes_attributes)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_attributes_attributes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_attributes_attributes(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_attributes_attributes)
    }

    pub unsafe fn xcb_x_print_print_get_attributes_attributes_length(
        &self,
        r: *const xcb_x_print_print_get_attributes_reply_t,
    ) -> c_int {
        sym!(self, xcb_x_print_print_get_attributes_attributes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_attributes_attributes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_attributes_attributes_length(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_attributes_attributes_length)
    }

    pub unsafe fn xcb_x_print_print_get_attributes_attributes_end(
        &self,
        r: *const xcb_x_print_print_get_attributes_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_x_print_print_get_attributes_attributes_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_attributes_attributes_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_attributes_attributes_end(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_attributes_attributes_end)
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
     * xcb_x_print_print_get_attributes_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_x_print_print_get_attributes_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_x_print_print_get_attributes_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_x_print_print_get_attributes_reply_t {
        sym!(self, xcb_x_print_print_get_attributes_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_attributes_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_attributes_reply(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_attributes_reply)
    }

    pub unsafe fn xcb_x_print_print_get_one_attributes_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_x_print_print_get_one_attributes_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_one_attributes_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_one_attributes_sizeof(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_one_attributes_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_x_print_print_get_one_attributes(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
        name_len: u32,
        pool: u8,
        name: *const xcb_x_print_string8_t,
    ) -> xcb_x_print_print_get_one_attributes_cookie_t {
        sym!(self, xcb_x_print_print_get_one_attributes)(c, context, name_len, pool, name)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_one_attributes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_one_attributes(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_one_attributes)
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
    pub unsafe fn xcb_x_print_print_get_one_attributes_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
        name_len: u32,
        pool: u8,
        name: *const xcb_x_print_string8_t,
    ) -> xcb_x_print_print_get_one_attributes_cookie_t {
        sym!(self, xcb_x_print_print_get_one_attributes_unchecked)(c, context, name_len, pool, name)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_one_attributes_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_one_attributes_unchecked(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_one_attributes_unchecked)
    }

    pub unsafe fn xcb_x_print_print_get_one_attributes_value(
        &self,
        r: *const xcb_x_print_print_get_one_attributes_reply_t,
    ) -> *mut xcb_x_print_string8_t {
        sym!(self, xcb_x_print_print_get_one_attributes_value)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_one_attributes_value` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_one_attributes_value(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_one_attributes_value)
    }

    pub unsafe fn xcb_x_print_print_get_one_attributes_value_length(
        &self,
        r: *const xcb_x_print_print_get_one_attributes_reply_t,
    ) -> c_int {
        sym!(self, xcb_x_print_print_get_one_attributes_value_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_one_attributes_value_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_one_attributes_value_length(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_one_attributes_value_length)
    }

    pub unsafe fn xcb_x_print_print_get_one_attributes_value_end(
        &self,
        r: *const xcb_x_print_print_get_one_attributes_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_x_print_print_get_one_attributes_value_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_one_attributes_value_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_one_attributes_value_end(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_one_attributes_value_end)
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
     * xcb_x_print_print_get_one_attributes_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_x_print_print_get_one_attributes_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_x_print_print_get_one_attributes_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_x_print_print_get_one_attributes_reply_t {
        sym!(self, xcb_x_print_print_get_one_attributes_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_one_attributes_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_one_attributes_reply(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_one_attributes_reply)
    }

    pub unsafe fn xcb_x_print_print_set_attributes_sizeof(
        &self,
        _buffer: *const c_void,
        attributes_len: u32,
    ) -> c_int {
        sym!(self, xcb_x_print_print_set_attributes_sizeof)(_buffer, attributes_len)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_set_attributes_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_set_attributes_sizeof(&self) -> bool {
        has_sym!(self, xcb_x_print_print_set_attributes_sizeof)
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
    pub unsafe fn xcb_x_print_print_set_attributes_checked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
        string_len: u32,
        pool: u8,
        rule: u8,
        attributes_len: u32,
        attributes: *const xcb_x_print_string8_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_print_set_attributes_checked)(
            c,
            context,
            string_len,
            pool,
            rule,
            attributes_len,
            attributes,
        )
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_set_attributes_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_set_attributes_checked(&self) -> bool {
        has_sym!(self, xcb_x_print_print_set_attributes_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_x_print_print_set_attributes(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
        string_len: u32,
        pool: u8,
        rule: u8,
        attributes_len: u32,
        attributes: *const xcb_x_print_string8_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_x_print_print_set_attributes)(
            c,
            context,
            string_len,
            pool,
            rule,
            attributes_len,
            attributes,
        )
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_set_attributes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_set_attributes(&self) -> bool {
        has_sym!(self, xcb_x_print_print_set_attributes)
    }

    pub unsafe fn xcb_x_print_print_set_attributes_attributes(
        &self,
        r: *const xcb_x_print_print_set_attributes_request_t,
    ) -> *mut xcb_x_print_string8_t {
        sym!(self, xcb_x_print_print_set_attributes_attributes)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_set_attributes_attributes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_set_attributes_attributes(&self) -> bool {
        has_sym!(self, xcb_x_print_print_set_attributes_attributes)
    }

    pub unsafe fn xcb_x_print_print_set_attributes_attributes_length(
        &self,
        r: *const xcb_x_print_print_set_attributes_request_t,
    ) -> c_int {
        sym!(self, xcb_x_print_print_set_attributes_attributes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_set_attributes_attributes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_set_attributes_attributes_length(&self) -> bool {
        has_sym!(self, xcb_x_print_print_set_attributes_attributes_length)
    }

    pub unsafe fn xcb_x_print_print_set_attributes_attributes_end(
        &self,
        r: *const xcb_x_print_print_set_attributes_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_x_print_print_set_attributes_attributes_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_set_attributes_attributes_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_set_attributes_attributes_end(&self) -> bool {
        has_sym!(self, xcb_x_print_print_set_attributes_attributes_end)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_x_print_print_get_page_dimensions(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
    ) -> xcb_x_print_print_get_page_dimensions_cookie_t {
        sym!(self, xcb_x_print_print_get_page_dimensions)(c, context)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_page_dimensions` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_page_dimensions(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_page_dimensions)
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
    pub unsafe fn xcb_x_print_print_get_page_dimensions_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
    ) -> xcb_x_print_print_get_page_dimensions_cookie_t {
        sym!(self, xcb_x_print_print_get_page_dimensions_unchecked)(c, context)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_page_dimensions_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_page_dimensions_unchecked(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_page_dimensions_unchecked)
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
     * xcb_x_print_print_get_page_dimensions_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_x_print_print_get_page_dimensions_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_x_print_print_get_page_dimensions_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_x_print_print_get_page_dimensions_reply_t {
        sym!(self, xcb_x_print_print_get_page_dimensions_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_page_dimensions_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_page_dimensions_reply(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_page_dimensions_reply)
    }

    pub unsafe fn xcb_x_print_print_query_screens_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_x_print_print_query_screens_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_query_screens_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_query_screens_sizeof(&self) -> bool {
        has_sym!(self, xcb_x_print_print_query_screens_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_x_print_print_query_screens(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_x_print_print_query_screens_cookie_t {
        sym!(self, xcb_x_print_print_query_screens)(c)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_query_screens` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_query_screens(&self) -> bool {
        has_sym!(self, xcb_x_print_print_query_screens)
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
    pub unsafe fn xcb_x_print_print_query_screens_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_x_print_print_query_screens_cookie_t {
        sym!(self, xcb_x_print_print_query_screens_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_query_screens_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_query_screens_unchecked(&self) -> bool {
        has_sym!(self, xcb_x_print_print_query_screens_unchecked)
    }

    pub unsafe fn xcb_x_print_print_query_screens_roots(
        &self,
        r: *const xcb_x_print_print_query_screens_reply_t,
    ) -> *mut xcb_window_t {
        sym!(self, xcb_x_print_print_query_screens_roots)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_query_screens_roots` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_query_screens_roots(&self) -> bool {
        has_sym!(self, xcb_x_print_print_query_screens_roots)
    }

    pub unsafe fn xcb_x_print_print_query_screens_roots_length(
        &self,
        r: *const xcb_x_print_print_query_screens_reply_t,
    ) -> c_int {
        sym!(self, xcb_x_print_print_query_screens_roots_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_query_screens_roots_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_query_screens_roots_length(&self) -> bool {
        has_sym!(self, xcb_x_print_print_query_screens_roots_length)
    }

    pub unsafe fn xcb_x_print_print_query_screens_roots_end(
        &self,
        r: *const xcb_x_print_print_query_screens_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_x_print_print_query_screens_roots_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_query_screens_roots_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_query_screens_roots_end(&self) -> bool {
        has_sym!(self, xcb_x_print_print_query_screens_roots_end)
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
     * xcb_x_print_print_query_screens_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_x_print_print_query_screens_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_x_print_print_query_screens_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_x_print_print_query_screens_reply_t {
        sym!(self, xcb_x_print_print_query_screens_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_query_screens_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_query_screens_reply(&self) -> bool {
        has_sym!(self, xcb_x_print_print_query_screens_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_x_print_print_set_image_resolution(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
        image_resolution: u16,
    ) -> xcb_x_print_print_set_image_resolution_cookie_t {
        sym!(self, xcb_x_print_print_set_image_resolution)(c, context, image_resolution)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_set_image_resolution` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_set_image_resolution(&self) -> bool {
        has_sym!(self, xcb_x_print_print_set_image_resolution)
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
    pub unsafe fn xcb_x_print_print_set_image_resolution_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
        image_resolution: u16,
    ) -> xcb_x_print_print_set_image_resolution_cookie_t {
        sym!(self, xcb_x_print_print_set_image_resolution_unchecked)(c, context, image_resolution)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_set_image_resolution_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_set_image_resolution_unchecked(&self) -> bool {
        has_sym!(self, xcb_x_print_print_set_image_resolution_unchecked)
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
     * xcb_x_print_print_set_image_resolution_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_x_print_print_set_image_resolution_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_x_print_print_set_image_resolution_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_x_print_print_set_image_resolution_reply_t {
        sym!(self, xcb_x_print_print_set_image_resolution_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_set_image_resolution_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_set_image_resolution_reply(&self) -> bool {
        has_sym!(self, xcb_x_print_print_set_image_resolution_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_x_print_print_get_image_resolution(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
    ) -> xcb_x_print_print_get_image_resolution_cookie_t {
        sym!(self, xcb_x_print_print_get_image_resolution)(c, context)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_image_resolution` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_image_resolution(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_image_resolution)
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
    pub unsafe fn xcb_x_print_print_get_image_resolution_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
    ) -> xcb_x_print_print_get_image_resolution_cookie_t {
        sym!(self, xcb_x_print_print_get_image_resolution_unchecked)(c, context)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_image_resolution_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_image_resolution_unchecked(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_image_resolution_unchecked)
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
     * xcb_x_print_print_get_image_resolution_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_x_print_print_get_image_resolution_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_x_print_print_get_image_resolution_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_x_print_print_get_image_resolution_reply_t {
        sym!(self, xcb_x_print_print_get_image_resolution_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_x_print_print_get_image_resolution_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_x_print_print_get_image_resolution_reply(&self) -> bool {
        has_sym!(self, xcb_x_print_print_get_image_resolution_reply)
    }
}

#[cfg(feature = "xcb_xprint")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbXprint::load().unwrap() };
        assert!(lib.has_xcb_x_print_id());
        assert!(lib.has_xcb_x_print_string8_next());
        assert!(lib.has_xcb_x_print_string8_end());
        assert!(lib.has_xcb_x_print_printer_serialize());
        assert!(lib.has_xcb_x_print_printer_unserialize());
        assert!(lib.has_xcb_x_print_printer_sizeof());
        assert!(lib.has_xcb_x_print_printer_name());
        assert!(lib.has_xcb_x_print_printer_name_length());
        assert!(lib.has_xcb_x_print_printer_name_end());
        assert!(lib.has_xcb_x_print_printer_description());
        assert!(lib.has_xcb_x_print_printer_description_length());
        assert!(lib.has_xcb_x_print_printer_description_end());
        assert!(lib.has_xcb_x_print_printer_next());
        assert!(lib.has_xcb_x_print_printer_end());
        assert!(lib.has_xcb_x_print_pcontext_next());
        assert!(lib.has_xcb_x_print_pcontext_end());
        assert!(lib.has_xcb_x_print_print_query_version());
        assert!(lib.has_xcb_x_print_print_query_version_unchecked());
        assert!(lib.has_xcb_x_print_print_query_version_reply());
        assert!(lib.has_xcb_x_print_print_get_printer_list_sizeof());
        assert!(lib.has_xcb_x_print_print_get_printer_list());
        assert!(lib.has_xcb_x_print_print_get_printer_list_unchecked());
        assert!(lib.has_xcb_x_print_print_get_printer_list_printers_length());
        assert!(lib.has_xcb_x_print_print_get_printer_list_printers_iterator());
        assert!(lib.has_xcb_x_print_print_get_printer_list_reply());
        assert!(lib.has_xcb_x_print_print_rehash_printer_list_checked());
        assert!(lib.has_xcb_x_print_print_rehash_printer_list());
        assert!(lib.has_xcb_x_print_create_context_sizeof());
        assert!(lib.has_xcb_x_print_create_context_checked());
        assert!(lib.has_xcb_x_print_create_context());
        assert!(lib.has_xcb_x_print_create_context_printer_name());
        assert!(lib.has_xcb_x_print_create_context_printer_name_length());
        assert!(lib.has_xcb_x_print_create_context_printer_name_end());
        assert!(lib.has_xcb_x_print_create_context_locale());
        assert!(lib.has_xcb_x_print_create_context_locale_length());
        assert!(lib.has_xcb_x_print_create_context_locale_end());
        assert!(lib.has_xcb_x_print_print_set_context_checked());
        assert!(lib.has_xcb_x_print_print_set_context());
        assert!(lib.has_xcb_x_print_print_get_context());
        assert!(lib.has_xcb_x_print_print_get_context_unchecked());
        assert!(lib.has_xcb_x_print_print_get_context_reply());
        assert!(lib.has_xcb_x_print_print_destroy_context_checked());
        assert!(lib.has_xcb_x_print_print_destroy_context());
        assert!(lib.has_xcb_x_print_print_get_screen_of_context());
        assert!(lib.has_xcb_x_print_print_get_screen_of_context_unchecked());
        assert!(lib.has_xcb_x_print_print_get_screen_of_context_reply());
        assert!(lib.has_xcb_x_print_print_start_job_checked());
        assert!(lib.has_xcb_x_print_print_start_job());
        assert!(lib.has_xcb_x_print_print_end_job_checked());
        assert!(lib.has_xcb_x_print_print_end_job());
        assert!(lib.has_xcb_x_print_print_start_doc_checked());
        assert!(lib.has_xcb_x_print_print_start_doc());
        assert!(lib.has_xcb_x_print_print_end_doc_checked());
        assert!(lib.has_xcb_x_print_print_end_doc());
        assert!(lib.has_xcb_x_print_print_put_document_data_sizeof());
        assert!(lib.has_xcb_x_print_print_put_document_data_checked());
        assert!(lib.has_xcb_x_print_print_put_document_data());
        assert!(lib.has_xcb_x_print_print_put_document_data_data());
        assert!(lib.has_xcb_x_print_print_put_document_data_data_length());
        assert!(lib.has_xcb_x_print_print_put_document_data_data_end());
        assert!(lib.has_xcb_x_print_print_put_document_data_doc_format());
        assert!(lib.has_xcb_x_print_print_put_document_data_doc_format_length());
        assert!(lib.has_xcb_x_print_print_put_document_data_doc_format_end());
        assert!(lib.has_xcb_x_print_print_put_document_data_options());
        assert!(lib.has_xcb_x_print_print_put_document_data_options_length());
        assert!(lib.has_xcb_x_print_print_put_document_data_options_end());
        assert!(lib.has_xcb_x_print_print_get_document_data_sizeof());
        assert!(lib.has_xcb_x_print_print_get_document_data());
        assert!(lib.has_xcb_x_print_print_get_document_data_unchecked());
        assert!(lib.has_xcb_x_print_print_get_document_data_data());
        assert!(lib.has_xcb_x_print_print_get_document_data_data_length());
        assert!(lib.has_xcb_x_print_print_get_document_data_data_end());
        assert!(lib.has_xcb_x_print_print_get_document_data_reply());
        assert!(lib.has_xcb_x_print_print_start_page_checked());
        assert!(lib.has_xcb_x_print_print_start_page());
        assert!(lib.has_xcb_x_print_print_end_page_checked());
        assert!(lib.has_xcb_x_print_print_end_page());
        assert!(lib.has_xcb_x_print_print_select_input_checked());
        assert!(lib.has_xcb_x_print_print_select_input());
        assert!(lib.has_xcb_x_print_print_input_selected());
        assert!(lib.has_xcb_x_print_print_input_selected_unchecked());
        assert!(lib.has_xcb_x_print_print_input_selected_reply());
        assert!(lib.has_xcb_x_print_print_get_attributes_sizeof());
        assert!(lib.has_xcb_x_print_print_get_attributes());
        assert!(lib.has_xcb_x_print_print_get_attributes_unchecked());
        assert!(lib.has_xcb_x_print_print_get_attributes_attributes());
        assert!(lib.has_xcb_x_print_print_get_attributes_attributes_length());
        assert!(lib.has_xcb_x_print_print_get_attributes_attributes_end());
        assert!(lib.has_xcb_x_print_print_get_attributes_reply());
        assert!(lib.has_xcb_x_print_print_get_one_attributes_sizeof());
        assert!(lib.has_xcb_x_print_print_get_one_attributes());
        assert!(lib.has_xcb_x_print_print_get_one_attributes_unchecked());
        assert!(lib.has_xcb_x_print_print_get_one_attributes_value());
        assert!(lib.has_xcb_x_print_print_get_one_attributes_value_length());
        assert!(lib.has_xcb_x_print_print_get_one_attributes_value_end());
        assert!(lib.has_xcb_x_print_print_get_one_attributes_reply());
        assert!(lib.has_xcb_x_print_print_set_attributes_sizeof());
        assert!(lib.has_xcb_x_print_print_set_attributes_checked());
        assert!(lib.has_xcb_x_print_print_set_attributes());
        assert!(lib.has_xcb_x_print_print_set_attributes_attributes());
        assert!(lib.has_xcb_x_print_print_set_attributes_attributes_length());
        assert!(lib.has_xcb_x_print_print_set_attributes_attributes_end());
        assert!(lib.has_xcb_x_print_print_get_page_dimensions());
        assert!(lib.has_xcb_x_print_print_get_page_dimensions_unchecked());
        assert!(lib.has_xcb_x_print_print_get_page_dimensions_reply());
        assert!(lib.has_xcb_x_print_print_query_screens_sizeof());
        assert!(lib.has_xcb_x_print_print_query_screens());
        assert!(lib.has_xcb_x_print_print_query_screens_unchecked());
        assert!(lib.has_xcb_x_print_print_query_screens_roots());
        assert!(lib.has_xcb_x_print_print_query_screens_roots_length());
        assert!(lib.has_xcb_x_print_print_query_screens_roots_end());
        assert!(lib.has_xcb_x_print_print_query_screens_reply());
        assert!(lib.has_xcb_x_print_print_set_image_resolution());
        assert!(lib.has_xcb_x_print_print_set_image_resolution_unchecked());
        assert!(lib.has_xcb_x_print_print_set_image_resolution_reply());
        assert!(lib.has_xcb_x_print_print_get_image_resolution());
        assert!(lib.has_xcb_x_print_print_get_image_resolution_unchecked());
        assert!(lib.has_xcb_x_print_print_get_image_resolution_reply());
    }
}
