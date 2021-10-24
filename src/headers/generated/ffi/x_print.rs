use crate::*;
use std::os::raw::*;

pub const XCB_X_PRINT_MAJOR_VERSION: u32 = 1;
pub const XCB_X_PRINT_MINOR_VERSION: u32 = 0;

pub type xcb_x_print_string8_t = c_char;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_string8_iterator_t {
    pub data: *mut xcb_x_print_string8_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_printer_t {
    pub name_len: u32,
    pub desc_len: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_printer_iterator_t {
    pub data: *mut xcb_x_print_printer_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_x_print_pcontext_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_pcontext_iterator_t {
    pub data: *mut xcb_x_print_pcontext_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_x_print_get_doc_t = u32;
pub const XCB_X_PRINT_GET_DOC_FINISHED: xcb_x_print_get_doc_t = 0x00;
pub const XCB_X_PRINT_GET_DOC_SECOND_CONSUMER: xcb_x_print_get_doc_t = 0x01;

pub type xcb_x_print_ev_mask_t = u32;
pub const XCB_X_PRINT_EV_MASK_NO_EVENT_MASK: xcb_x_print_ev_mask_t = 0x00;
pub const XCB_X_PRINT_EV_MASK_PRINT_MASK: xcb_x_print_ev_mask_t = 0x01;
pub const XCB_X_PRINT_EV_MASK_ATTRIBUTE_MASK: xcb_x_print_ev_mask_t = 0x02;

pub type xcb_x_print_detail_t = u32;
pub const XCB_X_PRINT_DETAIL_START_JOB_NOTIFY: xcb_x_print_detail_t = 0x01;
pub const XCB_X_PRINT_DETAIL_END_JOB_NOTIFY: xcb_x_print_detail_t = 0x02;
pub const XCB_X_PRINT_DETAIL_START_DOC_NOTIFY: xcb_x_print_detail_t = 0x03;
pub const XCB_X_PRINT_DETAIL_END_DOC_NOTIFY: xcb_x_print_detail_t = 0x04;
pub const XCB_X_PRINT_DETAIL_START_PAGE_NOTIFY: xcb_x_print_detail_t = 0x05;
pub const XCB_X_PRINT_DETAIL_END_PAGE_NOTIFY: xcb_x_print_detail_t = 0x06;

pub type xcb_x_print_attr_t = u32;
pub const XCB_X_PRINT_ATTR_JOB_ATTR: xcb_x_print_attr_t = 0x01;
pub const XCB_X_PRINT_ATTR_DOC_ATTR: xcb_x_print_attr_t = 0x02;
pub const XCB_X_PRINT_ATTR_PAGE_ATTR: xcb_x_print_attr_t = 0x03;
pub const XCB_X_PRINT_ATTR_PRINTER_ATTR: xcb_x_print_attr_t = 0x04;
pub const XCB_X_PRINT_ATTR_SERVER_ATTR: xcb_x_print_attr_t = 0x05;
pub const XCB_X_PRINT_ATTR_MEDIUM_ATTR: xcb_x_print_attr_t = 0x06;
pub const XCB_X_PRINT_ATTR_SPOOLER_ATTR: xcb_x_print_attr_t = 0x07;

pub const XCB_X_PRINT_PRINT_QUERY_VERSION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_query_version_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_X_PRINT_PRINT_GET_PRINTER_LIST: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_printer_list_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub printer_name_len: u32,
    pub locale_len: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_printer_list_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_X_PRINT_PRINT_REHASH_PRINTER_LIST: u8 = 20;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_rehash_printer_list_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

pub const XCB_X_PRINT_CREATE_CONTEXT: u8 = 2;

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

pub const XCB_X_PRINT_PRINT_SET_CONTEXT: u8 = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_set_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: u32,
}

pub const XCB_X_PRINT_PRINT_GET_CONTEXT: u8 = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_context_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_X_PRINT_PRINT_DESTROY_CONTEXT: u8 = 5;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_destroy_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: u32,
}

pub const XCB_X_PRINT_PRINT_GET_SCREEN_OF_CONTEXT: u8 = 6;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_screen_of_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_screen_of_context_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_X_PRINT_PRINT_START_JOB: u8 = 7;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_start_job_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub output_mode: u8,
}

pub const XCB_X_PRINT_PRINT_END_JOB: u8 = 8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_end_job_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub cancel: u8,
}

pub const XCB_X_PRINT_PRINT_START_DOC: u8 = 9;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_start_doc_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub driver_mode: u8,
}

pub const XCB_X_PRINT_PRINT_END_DOC: u8 = 10;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_end_doc_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub cancel: u8,
}

pub const XCB_X_PRINT_PRINT_PUT_DOCUMENT_DATA: u8 = 11;

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

pub const XCB_X_PRINT_PRINT_GET_DOCUMENT_DATA: u8 = 12;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_document_data_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_x_print_pcontext_t,
    pub max_bytes: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_document_data_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_X_PRINT_PRINT_START_PAGE: u8 = 13;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_start_page_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

pub const XCB_X_PRINT_PRINT_END_PAGE: u8 = 14;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_end_page_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub cancel: u8,
    pub pad0: [u8; 3],
}

pub const XCB_X_PRINT_PRINT_SELECT_INPUT: u8 = 15;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_select_input_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_x_print_pcontext_t,
    pub event_mask: u32,
}

pub const XCB_X_PRINT_PRINT_INPUT_SELECTED: u8 = 16;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_input_selected_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_x_print_pcontext_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_input_selected_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_X_PRINT_PRINT_GET_ATTRIBUTES: u8 = 17;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_attributes_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_X_PRINT_PRINT_GET_ONE_ATTRIBUTES: u8 = 19;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_one_attributes_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_X_PRINT_PRINT_SET_ATTRIBUTES: u8 = 18;

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

pub const XCB_X_PRINT_PRINT_GET_PAGE_DIMENSIONS: u8 = 21;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_page_dimensions_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_x_print_pcontext_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_page_dimensions_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_X_PRINT_PRINT_QUERY_SCREENS: u8 = 22;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_query_screens_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_query_screens_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_X_PRINT_PRINT_SET_IMAGE_RESOLUTION: u8 = 23;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_set_image_resolution_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_x_print_pcontext_t,
    pub image_resolution: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_set_image_resolution_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_X_PRINT_PRINT_GET_IMAGE_RESOLUTION: u8 = 24;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_image_resolution_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context: xcb_x_print_pcontext_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_print_get_image_resolution_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_X_PRINT_NOTIFY: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_notify_event_t {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub context: xcb_x_print_pcontext_t,
    pub cancel: u8,
}

pub const XCB_X_PRINT_ATTRIBUT_NOTIFY: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_attribut_notify_event_t {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub context: xcb_x_print_pcontext_t,
}

pub const XCB_X_PRINT_BAD_CONTEXT: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_bad_context_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

pub const XCB_X_PRINT_BAD_SEQUENCE: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_x_print_bad_sequence_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl XcbXprint {
    #[inline]
    pub fn xcb_x_print_id(&self) -> *mut xcb_extension_t {
        call!(self, xcb_x_print_id)
    }

    #[inline]
    pub unsafe fn xcb_x_print_string8_next(&self, i: *mut xcb_x_print_string8_iterator_t) {
        call!(self, xcb_x_print_string8_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_x_print_string8_end(
        &self,
        i: *mut xcb_x_print_string8_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_x_print_string8_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_x_print_printer_name(
        &self,
        R: *const xcb_x_print_printer_t,
    ) -> *mut xcb_x_print_string8_t {
        call!(self, xcb_x_print_printer_name)(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_printer_name_length(&self, R: *const xcb_x_print_printer_t) -> c_int {
        call!(self, xcb_x_print_printer_name_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_printer_name_end(
        &self,
        R: *const xcb_x_print_printer_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_x_print_printer_name_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_printer_description(
        &self,
        R: *const xcb_x_print_printer_t,
    ) -> *mut xcb_x_print_string8_t {
        call!(self, xcb_x_print_printer_description)(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_printer_description_length(
        &self,
        R: *const xcb_x_print_printer_t,
    ) -> c_int {
        call!(self, xcb_x_print_printer_description_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_printer_description_end(
        &self,
        R: *const xcb_x_print_printer_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_x_print_printer_description_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_printer_next(&self, i: *mut xcb_x_print_printer_iterator_t) {
        call!(self, xcb_x_print_printer_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_x_print_printer_end(
        &self,
        i: *mut xcb_x_print_printer_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_x_print_printer_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_x_print_pcontext_next(&self, i: *mut xcb_x_print_pcontext_iterator_t) {
        call!(self, xcb_x_print_pcontext_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_x_print_pcontext_end(
        &self,
        i: *mut xcb_x_print_pcontext_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_x_print_pcontext_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_x_print_print_query_version_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_x_print_print_query_version_reply_t {
        call!(self, xcb_x_print_print_query_version_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_query_version(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_x_print_print_query_version_cookie_t {
        call!(self, xcb_x_print_print_query_version)(c)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_x_print_print_query_version_cookie_t {
        call!(self, xcb_x_print_print_query_version_unchecked)(c)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_printer_list_printers_length(
        &self,
        R: *const xcb_x_print_print_get_printer_list_reply_t,
    ) -> c_int {
        call!(self, xcb_x_print_print_get_printer_list_printers_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_printer_list_printers_iterator(
        &self,
        R: *const xcb_x_print_print_get_printer_list_reply_t,
    ) -> xcb_x_print_printer_iterator_t {
        call!(self, xcb_x_print_print_get_printer_list_printers_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_printer_list_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_x_print_print_get_printer_list_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_x_print_print_get_printer_list_reply_t {
        call!(self, xcb_x_print_print_get_printer_list_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_printer_list(
        &self,
        c: *mut xcb_connection_t,
        printer_name_len: u32,
        locale_len: u32,
        printer_name: *const xcb_x_print_string8_t,
        locale: *const xcb_x_print_string8_t,
    ) -> xcb_x_print_print_get_printer_list_cookie_t {
        call!(self, xcb_x_print_print_get_printer_list)(
            c,
            printer_name_len,
            locale_len,
            printer_name,
            locale,
        )
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_printer_list_unchecked(
        &self,
        c: *mut xcb_connection_t,
        printer_name_len: u32,
        locale_len: u32,
        printer_name: *const xcb_x_print_string8_t,
        locale: *const xcb_x_print_string8_t,
    ) -> xcb_x_print_print_get_printer_list_cookie_t {
        call!(self, xcb_x_print_print_get_printer_list_unchecked)(
            c,
            printer_name_len,
            locale_len,
            printer_name,
            locale,
        )
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_rehash_printer_list(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_x_print_print_rehash_printer_list)(c)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_rehash_printer_list_checked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_x_print_print_rehash_printer_list_checked)(c)
    }

    #[inline]
    pub unsafe fn xcb_x_print_create_context(
        &self,
        c: *mut xcb_connection_t,
        context_id: u32,
        printer_name_len: u32,
        locale_len: u32,
        printer_name: *const xcb_x_print_string8_t,
        locale: *const xcb_x_print_string8_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_x_print_create_context)(
            c,
            context_id,
            printer_name_len,
            locale_len,
            printer_name,
            locale,
        )
    }

    #[inline]
    pub unsafe fn xcb_x_print_create_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context_id: u32,
        printer_name_len: u32,
        locale_len: u32,
        printer_name: *const xcb_x_print_string8_t,
        locale: *const xcb_x_print_string8_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_x_print_create_context_checked)(
            c,
            context_id,
            printer_name_len,
            locale_len,
            printer_name,
            locale,
        )
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_set_context(
        &self,
        c: *mut xcb_connection_t,
        context: u32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_x_print_print_set_context)(c, context)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_set_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context: u32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_x_print_print_set_context_checked)(c, context)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_x_print_print_get_context_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_x_print_print_get_context_reply_t {
        call!(self, xcb_x_print_print_get_context_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_context(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_x_print_print_get_context_cookie_t {
        call!(self, xcb_x_print_print_get_context)(c)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_x_print_print_get_context_cookie_t {
        call!(self, xcb_x_print_print_get_context_unchecked)(c)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_destroy_context(
        &self,
        c: *mut xcb_connection_t,
        context: u32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_x_print_print_destroy_context)(c, context)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_destroy_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context: u32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_x_print_print_destroy_context_checked)(c, context)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_screen_of_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_x_print_print_get_screen_of_context_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_x_print_print_get_screen_of_context_reply_t {
        call!(self, xcb_x_print_print_get_screen_of_context_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_screen_of_context(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_x_print_print_get_screen_of_context_cookie_t {
        call!(self, xcb_x_print_print_get_screen_of_context)(c)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_screen_of_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_x_print_print_get_screen_of_context_cookie_t {
        call!(self, xcb_x_print_print_get_screen_of_context_unchecked)(c)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_start_job(
        &self,
        c: *mut xcb_connection_t,
        output_mode: u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_x_print_print_start_job)(c, output_mode)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_start_job_checked(
        &self,
        c: *mut xcb_connection_t,
        output_mode: u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_x_print_print_start_job_checked)(c, output_mode)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_end_job(
        &self,
        c: *mut xcb_connection_t,
        cancel: u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_x_print_print_end_job)(c, cancel)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_end_job_checked(
        &self,
        c: *mut xcb_connection_t,
        cancel: u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_x_print_print_end_job_checked)(c, cancel)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_start_doc(
        &self,
        c: *mut xcb_connection_t,
        driver_mode: u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_x_print_print_start_doc)(c, driver_mode)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_start_doc_checked(
        &self,
        c: *mut xcb_connection_t,
        driver_mode: u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_x_print_print_start_doc_checked)(c, driver_mode)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_end_doc(
        &self,
        c: *mut xcb_connection_t,
        cancel: u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_x_print_print_end_doc)(c, cancel)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_end_doc_checked(
        &self,
        c: *mut xcb_connection_t,
        cancel: u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_x_print_print_end_doc_checked)(c, cancel)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_put_document_data(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        len_data: u32,
        len_fmt: u16,
        len_options: u16,
        data: *const u8,
        doc_format_len: u32,
        doc_format: *const xcb_x_print_string8_t,
        options_len: u32,
        options: *const xcb_x_print_string8_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_x_print_print_put_document_data)(
            c,
            drawable,
            len_data,
            len_fmt,
            len_options,
            data,
            doc_format_len,
            doc_format,
            options_len,
            options,
        )
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_put_document_data_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        len_data: u32,
        len_fmt: u16,
        len_options: u16,
        data: *const u8,
        doc_format_len: u32,
        doc_format: *const xcb_x_print_string8_t,
        options_len: u32,
        options: *const xcb_x_print_string8_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_x_print_print_put_document_data_checked)(
            c,
            drawable,
            len_data,
            len_fmt,
            len_options,
            data,
            doc_format_len,
            doc_format,
            options_len,
            options,
        )
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_document_data_data(
        &self,
        R: *const xcb_x_print_print_get_document_data_reply_t,
    ) -> *mut u8 {
        call!(self, xcb_x_print_print_get_document_data_data)(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_document_data_data_length(
        &self,
        R: *const xcb_x_print_print_get_document_data_reply_t,
    ) -> c_int {
        call!(self, xcb_x_print_print_get_document_data_data_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_document_data_data_end(
        &self,
        R: *const xcb_x_print_print_get_document_data_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_x_print_print_get_document_data_data_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_document_data_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_x_print_print_get_document_data_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_x_print_print_get_document_data_reply_t {
        call!(self, xcb_x_print_print_get_document_data_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_document_data(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
        max_bytes: u32,
    ) -> xcb_x_print_print_get_document_data_cookie_t {
        call!(self, xcb_x_print_print_get_document_data)(c, context, max_bytes)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_document_data_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
        max_bytes: u32,
    ) -> xcb_x_print_print_get_document_data_cookie_t {
        call!(self, xcb_x_print_print_get_document_data_unchecked)(c, context, max_bytes)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_start_page(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_x_print_print_start_page)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_start_page_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_x_print_print_start_page_checked)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_end_page(
        &self,
        c: *mut xcb_connection_t,
        cancel: u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_x_print_print_end_page)(c, cancel)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_end_page_checked(
        &self,
        c: *mut xcb_connection_t,
        cancel: u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_x_print_print_end_page_checked)(c, cancel)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_select_input(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
        event_mask: u32,
        event_list: *const u32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_x_print_print_select_input)(c, context, event_mask, event_list)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_select_input_checked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
        event_mask: u32,
        event_list: *const u32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_x_print_print_select_input_checked)(c, context, event_mask, event_list)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_input_selected_event_list(
        &self,
        R: *const xcb_x_print_print_input_selected_reply_t,
    ) -> *mut u32 {
        call!(self, xcb_x_print_print_input_selected_event_list)(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_input_selected_event_list_length(
        &self,
        R: *const xcb_x_print_print_input_selected_reply_t,
    ) -> c_int {
        call!(self, xcb_x_print_print_input_selected_event_list_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_input_selected_event_list_end(
        &self,
        R: *const xcb_x_print_print_input_selected_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_x_print_print_input_selected_event_list_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_input_selected_all_events_list(
        &self,
        R: *const xcb_x_print_print_input_selected_reply_t,
    ) -> *mut u32 {
        call!(self, xcb_x_print_print_input_selected_all_events_list)(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_input_selected_all_events_list_length(
        &self,
        R: *const xcb_x_print_print_input_selected_reply_t,
    ) -> c_int {
        call!(
            self,
            xcb_x_print_print_input_selected_all_events_list_length
        )(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_input_selected_all_events_list_end(
        &self,
        R: *const xcb_x_print_print_input_selected_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_x_print_print_input_selected_all_events_list_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_input_selected_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_x_print_print_input_selected_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_x_print_print_input_selected_reply_t {
        call!(self, xcb_x_print_print_input_selected_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_input_selected(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
    ) -> xcb_x_print_print_input_selected_cookie_t {
        call!(self, xcb_x_print_print_input_selected)(c, context)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_input_selected_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
    ) -> xcb_x_print_print_input_selected_cookie_t {
        call!(self, xcb_x_print_print_input_selected_unchecked)(c, context)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_attributes_attributes(
        &self,
        R: *const xcb_x_print_print_get_attributes_reply_t,
    ) -> *mut xcb_x_print_string8_t {
        call!(self, xcb_x_print_print_get_attributes_attributes)(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_attributes_attributes_length(
        &self,
        R: *const xcb_x_print_print_get_attributes_reply_t,
    ) -> c_int {
        call!(self, xcb_x_print_print_get_attributes_attributes_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_attributes_attributes_end(
        &self,
        R: *const xcb_x_print_print_get_attributes_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_x_print_print_get_attributes_attributes_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_attributes_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_x_print_print_get_attributes_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_x_print_print_get_attributes_reply_t {
        call!(self, xcb_x_print_print_get_attributes_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_attributes(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
        pool: u8,
    ) -> xcb_x_print_print_get_attributes_cookie_t {
        call!(self, xcb_x_print_print_get_attributes)(c, context, pool)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_attributes_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
        pool: u8,
    ) -> xcb_x_print_print_get_attributes_cookie_t {
        call!(self, xcb_x_print_print_get_attributes_unchecked)(c, context, pool)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_one_attributes_value(
        &self,
        R: *const xcb_x_print_print_get_one_attributes_reply_t,
    ) -> *mut xcb_x_print_string8_t {
        call!(self, xcb_x_print_print_get_one_attributes_value)(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_one_attributes_value_length(
        &self,
        R: *const xcb_x_print_print_get_one_attributes_reply_t,
    ) -> c_int {
        call!(self, xcb_x_print_print_get_one_attributes_value_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_one_attributes_value_end(
        &self,
        R: *const xcb_x_print_print_get_one_attributes_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_x_print_print_get_one_attributes_value_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_one_attributes_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_x_print_print_get_one_attributes_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_x_print_print_get_one_attributes_reply_t {
        call!(self, xcb_x_print_print_get_one_attributes_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_one_attributes(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
        name_len: u32,
        pool: u8,
        name: *const xcb_x_print_string8_t,
    ) -> xcb_x_print_print_get_one_attributes_cookie_t {
        call!(self, xcb_x_print_print_get_one_attributes)(c, context, name_len, pool, name)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_one_attributes_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
        name_len: u32,
        pool: u8,
        name: *const xcb_x_print_string8_t,
    ) -> xcb_x_print_print_get_one_attributes_cookie_t {
        call!(self, xcb_x_print_print_get_one_attributes_unchecked)(
            c, context, name_len, pool, name,
        )
    }

    #[inline]
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
        call!(self, xcb_x_print_print_set_attributes)(
            c,
            context,
            string_len,
            pool,
            rule,
            attributes_len,
            attributes,
        )
    }

    #[inline]
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
        call!(self, xcb_x_print_print_set_attributes_checked)(
            c,
            context,
            string_len,
            pool,
            rule,
            attributes_len,
            attributes,
        )
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_page_dimensions_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_x_print_print_get_page_dimensions_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_x_print_print_get_page_dimensions_reply_t {
        call!(self, xcb_x_print_print_get_page_dimensions_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_page_dimensions(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
    ) -> xcb_x_print_print_get_page_dimensions_cookie_t {
        call!(self, xcb_x_print_print_get_page_dimensions)(c, context)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_page_dimensions_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
    ) -> xcb_x_print_print_get_page_dimensions_cookie_t {
        call!(self, xcb_x_print_print_get_page_dimensions_unchecked)(c, context)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_query_screens_roots(
        &self,
        R: *const xcb_x_print_print_query_screens_reply_t,
    ) -> *mut xcb_window_t {
        call!(self, xcb_x_print_print_query_screens_roots)(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_query_screens_roots_length(
        &self,
        R: *const xcb_x_print_print_query_screens_reply_t,
    ) -> c_int {
        call!(self, xcb_x_print_print_query_screens_roots_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_query_screens_roots_end(
        &self,
        R: *const xcb_x_print_print_query_screens_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_x_print_print_query_screens_roots_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_query_screens_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_x_print_print_query_screens_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_x_print_print_query_screens_reply_t {
        call!(self, xcb_x_print_print_query_screens_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_query_screens(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_x_print_print_query_screens_cookie_t {
        call!(self, xcb_x_print_print_query_screens)(c)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_query_screens_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_x_print_print_query_screens_cookie_t {
        call!(self, xcb_x_print_print_query_screens_unchecked)(c)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_set_image_resolution_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_x_print_print_set_image_resolution_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_x_print_print_set_image_resolution_reply_t {
        call!(self, xcb_x_print_print_set_image_resolution_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_set_image_resolution(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
        image_resolution: u16,
    ) -> xcb_x_print_print_set_image_resolution_cookie_t {
        call!(self, xcb_x_print_print_set_image_resolution)(c, context, image_resolution)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_set_image_resolution_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
        image_resolution: u16,
    ) -> xcb_x_print_print_set_image_resolution_cookie_t {
        call!(self, xcb_x_print_print_set_image_resolution_unchecked)(c, context, image_resolution)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_image_resolution_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_x_print_print_get_image_resolution_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_x_print_print_get_image_resolution_reply_t {
        call!(self, xcb_x_print_print_get_image_resolution_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_image_resolution(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
    ) -> xcb_x_print_print_get_image_resolution_cookie_t {
        call!(self, xcb_x_print_print_get_image_resolution)(c, context)
    }

    #[inline]
    pub unsafe fn xcb_x_print_print_get_image_resolution_unchecked(
        &self,
        c: *mut xcb_connection_t,
        context: xcb_x_print_pcontext_t,
    ) -> xcb_x_print_print_get_image_resolution_cookie_t {
        call!(self, xcb_x_print_print_get_image_resolution_unchecked)(c, context)
    }
}

pub struct XcbXprint {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_x_print_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_x_print_string8_next:
        LazySymbol<unsafe fn(i: *mut xcb_x_print_string8_iterator_t)>,
    pub(crate) xcb_x_print_string8_end:
        LazySymbol<unsafe fn(i: *mut xcb_x_print_string8_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_x_print_printer_name:
        LazySymbol<unsafe fn(R: *const xcb_x_print_printer_t) -> *mut xcb_x_print_string8_t>,
    pub(crate) xcb_x_print_printer_name_length:
        LazySymbol<unsafe fn(R: *const xcb_x_print_printer_t) -> c_int>,
    pub(crate) xcb_x_print_printer_name_end:
        LazySymbol<unsafe fn(R: *const xcb_x_print_printer_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_x_print_printer_description:
        LazySymbol<unsafe fn(R: *const xcb_x_print_printer_t) -> *mut xcb_x_print_string8_t>,
    pub(crate) xcb_x_print_printer_description_length:
        LazySymbol<unsafe fn(R: *const xcb_x_print_printer_t) -> c_int>,
    pub(crate) xcb_x_print_printer_description_end:
        LazySymbol<unsafe fn(R: *const xcb_x_print_printer_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_x_print_printer_next:
        LazySymbol<unsafe fn(i: *mut xcb_x_print_printer_iterator_t)>,
    pub(crate) xcb_x_print_printer_end:
        LazySymbol<unsafe fn(i: *mut xcb_x_print_printer_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_x_print_pcontext_next:
        LazySymbol<unsafe fn(i: *mut xcb_x_print_pcontext_iterator_t)>,
    pub(crate) xcb_x_print_pcontext_end:
        LazySymbol<unsafe fn(i: *mut xcb_x_print_pcontext_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_x_print_print_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_x_print_print_query_version_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_x_print_print_query_version_reply_t,
    >,
    pub(crate) xcb_x_print_print_query_version:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_x_print_print_query_version_cookie_t>,
    pub(crate) xcb_x_print_print_query_version_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_x_print_print_query_version_cookie_t>,
    pub(crate) xcb_x_print_print_get_printer_list_printers_length:
        LazySymbol<unsafe fn(R: *const xcb_x_print_print_get_printer_list_reply_t) -> c_int>,
    pub(crate) xcb_x_print_print_get_printer_list_printers_iterator: LazySymbol<
        unsafe fn(
            R: *const xcb_x_print_print_get_printer_list_reply_t,
        ) -> xcb_x_print_printer_iterator_t,
    >,
    pub(crate) xcb_x_print_print_get_printer_list_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_x_print_print_get_printer_list_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_x_print_print_get_printer_list_reply_t,
    >,
    pub(crate) xcb_x_print_print_get_printer_list: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            printer_name_len: u32,
            locale_len: u32,
            printer_name: *const xcb_x_print_string8_t,
            locale: *const xcb_x_print_string8_t,
        ) -> xcb_x_print_print_get_printer_list_cookie_t,
    >,
    pub(crate) xcb_x_print_print_get_printer_list_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            printer_name_len: u32,
            locale_len: u32,
            printer_name: *const xcb_x_print_string8_t,
            locale: *const xcb_x_print_string8_t,
        ) -> xcb_x_print_print_get_printer_list_cookie_t,
    >,
    pub(crate) xcb_x_print_print_rehash_printer_list:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_rehash_printer_list_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_create_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_id: u32,
            printer_name_len: u32,
            locale_len: u32,
            printer_name: *const xcb_x_print_string8_t,
            locale: *const xcb_x_print_string8_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_x_print_create_context_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_id: u32,
            printer_name_len: u32,
            locale_len: u32,
            printer_name: *const xcb_x_print_string8_t,
            locale: *const xcb_x_print_string8_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_x_print_print_set_context:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: u32) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_set_context_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: u32) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_get_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_x_print_print_get_context_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_x_print_print_get_context_reply_t,
    >,
    pub(crate) xcb_x_print_print_get_context:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_x_print_print_get_context_cookie_t>,
    pub(crate) xcb_x_print_print_get_context_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_x_print_print_get_context_cookie_t>,
    pub(crate) xcb_x_print_print_destroy_context:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: u32) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_destroy_context_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: u32) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_get_screen_of_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_x_print_print_get_screen_of_context_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_x_print_print_get_screen_of_context_reply_t,
    >,
    pub(crate) xcb_x_print_print_get_screen_of_context: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_x_print_print_get_screen_of_context_cookie_t,
    >,
    pub(crate) xcb_x_print_print_get_screen_of_context_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_x_print_print_get_screen_of_context_cookie_t,
    >,
    pub(crate) xcb_x_print_print_start_job:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, output_mode: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_start_job_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, output_mode: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_end_job:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cancel: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_end_job_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cancel: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_start_doc:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, driver_mode: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_start_doc_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, driver_mode: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_end_doc:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cancel: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_end_doc_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cancel: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_put_document_data: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            len_data: u32,
            len_fmt: u16,
            len_options: u16,
            data: *const u8,
            doc_format_len: u32,
            doc_format: *const xcb_x_print_string8_t,
            options_len: u32,
            options: *const xcb_x_print_string8_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_x_print_print_put_document_data_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            len_data: u32,
            len_fmt: u16,
            len_options: u16,
            data: *const u8,
            doc_format_len: u32,
            doc_format: *const xcb_x_print_string8_t,
            options_len: u32,
            options: *const xcb_x_print_string8_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_x_print_print_get_document_data_data:
        LazySymbol<unsafe fn(R: *const xcb_x_print_print_get_document_data_reply_t) -> *mut u8>,
    pub(crate) xcb_x_print_print_get_document_data_data_length:
        LazySymbol<unsafe fn(R: *const xcb_x_print_print_get_document_data_reply_t) -> c_int>,
    pub(crate) xcb_x_print_print_get_document_data_data_end: LazySymbol<
        unsafe fn(R: *const xcb_x_print_print_get_document_data_reply_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_x_print_print_get_document_data_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_x_print_print_get_document_data_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_x_print_print_get_document_data_reply_t,
    >,
    pub(crate) xcb_x_print_print_get_document_data: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
            max_bytes: u32,
        ) -> xcb_x_print_print_get_document_data_cookie_t,
    >,
    pub(crate) xcb_x_print_print_get_document_data_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
            max_bytes: u32,
        ) -> xcb_x_print_print_get_document_data_cookie_t,
    >,
    pub(crate) xcb_x_print_print_start_page:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_start_page_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_end_page:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cancel: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_end_page_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cancel: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_select_input: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
            event_mask: u32,
            event_list: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_x_print_print_select_input_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
            event_mask: u32,
            event_list: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_x_print_print_input_selected_event_list:
        LazySymbol<unsafe fn(R: *const xcb_x_print_print_input_selected_reply_t) -> *mut u32>,
    pub(crate) xcb_x_print_print_input_selected_event_list_length:
        LazySymbol<unsafe fn(R: *const xcb_x_print_print_input_selected_reply_t) -> c_int>,
    pub(crate) xcb_x_print_print_input_selected_event_list_end: LazySymbol<
        unsafe fn(R: *const xcb_x_print_print_input_selected_reply_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_x_print_print_input_selected_all_events_list:
        LazySymbol<unsafe fn(R: *const xcb_x_print_print_input_selected_reply_t) -> *mut u32>,
    pub(crate) xcb_x_print_print_input_selected_all_events_list_length:
        LazySymbol<unsafe fn(R: *const xcb_x_print_print_input_selected_reply_t) -> c_int>,
    pub(crate) xcb_x_print_print_input_selected_all_events_list_end: LazySymbol<
        unsafe fn(R: *const xcb_x_print_print_input_selected_reply_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_x_print_print_input_selected_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_x_print_print_input_selected_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_x_print_print_input_selected_reply_t,
    >,
    pub(crate) xcb_x_print_print_input_selected: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
        ) -> xcb_x_print_print_input_selected_cookie_t,
    >,
    pub(crate) xcb_x_print_print_input_selected_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
        ) -> xcb_x_print_print_input_selected_cookie_t,
    >,
    pub(crate) xcb_x_print_print_get_attributes_attributes: LazySymbol<
        unsafe fn(R: *const xcb_x_print_print_get_attributes_reply_t) -> *mut xcb_x_print_string8_t,
    >,
    pub(crate) xcb_x_print_print_get_attributes_attributes_length:
        LazySymbol<unsafe fn(R: *const xcb_x_print_print_get_attributes_reply_t) -> c_int>,
    pub(crate) xcb_x_print_print_get_attributes_attributes_end: LazySymbol<
        unsafe fn(R: *const xcb_x_print_print_get_attributes_reply_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_x_print_print_get_attributes_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_x_print_print_get_attributes_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_x_print_print_get_attributes_reply_t,
    >,
    pub(crate) xcb_x_print_print_get_attributes: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
            pool: u8,
        ) -> xcb_x_print_print_get_attributes_cookie_t,
    >,
    pub(crate) xcb_x_print_print_get_attributes_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
            pool: u8,
        ) -> xcb_x_print_print_get_attributes_cookie_t,
    >,
    pub(crate) xcb_x_print_print_get_one_attributes_value: LazySymbol<
        unsafe fn(
            R: *const xcb_x_print_print_get_one_attributes_reply_t,
        ) -> *mut xcb_x_print_string8_t,
    >,
    pub(crate) xcb_x_print_print_get_one_attributes_value_length:
        LazySymbol<unsafe fn(R: *const xcb_x_print_print_get_one_attributes_reply_t) -> c_int>,
    pub(crate) xcb_x_print_print_get_one_attributes_value_end: LazySymbol<
        unsafe fn(R: *const xcb_x_print_print_get_one_attributes_reply_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_x_print_print_get_one_attributes_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_x_print_print_get_one_attributes_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_x_print_print_get_one_attributes_reply_t,
    >,
    pub(crate) xcb_x_print_print_get_one_attributes: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
            name_len: u32,
            pool: u8,
            name: *const xcb_x_print_string8_t,
        ) -> xcb_x_print_print_get_one_attributes_cookie_t,
    >,
    pub(crate) xcb_x_print_print_get_one_attributes_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
            name_len: u32,
            pool: u8,
            name: *const xcb_x_print_string8_t,
        ) -> xcb_x_print_print_get_one_attributes_cookie_t,
    >,
    pub(crate) xcb_x_print_print_set_attributes: LazySymbol<
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
    pub(crate) xcb_x_print_print_set_attributes_checked: LazySymbol<
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
    pub(crate) xcb_x_print_print_get_page_dimensions_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_x_print_print_get_page_dimensions_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_x_print_print_get_page_dimensions_reply_t,
    >,
    pub(crate) xcb_x_print_print_get_page_dimensions: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
        ) -> xcb_x_print_print_get_page_dimensions_cookie_t,
    >,
    pub(crate) xcb_x_print_print_get_page_dimensions_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
        ) -> xcb_x_print_print_get_page_dimensions_cookie_t,
    >,
    pub(crate) xcb_x_print_print_query_screens_roots: LazySymbol<
        unsafe fn(R: *const xcb_x_print_print_query_screens_reply_t) -> *mut xcb_window_t,
    >,
    pub(crate) xcb_x_print_print_query_screens_roots_length:
        LazySymbol<unsafe fn(R: *const xcb_x_print_print_query_screens_reply_t) -> c_int>,
    pub(crate) xcb_x_print_print_query_screens_roots_end: LazySymbol<
        unsafe fn(R: *const xcb_x_print_print_query_screens_reply_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_x_print_print_query_screens_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_x_print_print_query_screens_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_x_print_print_query_screens_reply_t,
    >,
    pub(crate) xcb_x_print_print_query_screens:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_x_print_print_query_screens_cookie_t>,
    pub(crate) xcb_x_print_print_query_screens_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_x_print_print_query_screens_cookie_t>,
    pub(crate) xcb_x_print_print_set_image_resolution_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_x_print_print_set_image_resolution_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_x_print_print_set_image_resolution_reply_t,
    >,
    pub(crate) xcb_x_print_print_set_image_resolution: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
            image_resolution: u16,
        ) -> xcb_x_print_print_set_image_resolution_cookie_t,
    >,
    pub(crate) xcb_x_print_print_set_image_resolution_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
            image_resolution: u16,
        ) -> xcb_x_print_print_set_image_resolution_cookie_t,
    >,
    pub(crate) xcb_x_print_print_get_image_resolution_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_x_print_print_get_image_resolution_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_x_print_print_get_image_resolution_reply_t,
    >,
    pub(crate) xcb_x_print_print_get_image_resolution: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
        ) -> xcb_x_print_print_get_image_resolution_cookie_t,
    >,
    pub(crate) xcb_x_print_print_get_image_resolution_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context: xcb_x_print_pcontext_t,
        ) -> xcb_x_print_print_get_image_resolution_cookie_t,
    >,
}
