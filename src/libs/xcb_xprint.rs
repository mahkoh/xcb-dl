use crate::lazy::*;
use crate::*;
use libloading::{Error, Library};
use std::os::raw::*;

#[rustfmt::skip]
pub struct XcbXprint {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_x_print_create_context: LazySymbol<unsafe fn(c: *mut xcb_connection_t, context_id: u32, printer_name_len: u32, locale_len: u32, printer_name: *const xcb_x_print_string8_t, locale: *const xcb_x_print_string8_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_create_context_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, context_id: u32, printer_name_len: u32, locale_len: u32, printer_name: *const xcb_x_print_string8_t, locale: *const xcb_x_print_string8_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_x_print_pcontext_end: LazySymbol<unsafe fn(i: *mut xcb_x_print_pcontext_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_x_print_pcontext_next: LazySymbol<unsafe fn(i: *mut xcb_x_print_pcontext_iterator_t)>,
    pub(crate) xcb_x_print_print_destroy_context: LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: u32) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_destroy_context_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: u32) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_end_doc: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cancel: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_end_doc_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cancel: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_end_job: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cancel: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_end_job_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cancel: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_end_page: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cancel: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_end_page_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cancel: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_get_attributes: LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: xcb_x_print_pcontext_t, pool: u8) -> xcb_x_print_print_get_attributes_cookie_t>,
    pub(crate) xcb_x_print_print_get_attributes_attributes: LazySymbol<unsafe fn(R: *const xcb_x_print_print_get_attributes_reply_t) -> *mut xcb_x_print_string8_t>,
    pub(crate) xcb_x_print_print_get_attributes_attributes_end: LazySymbol<unsafe fn(R: *const xcb_x_print_print_get_attributes_reply_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_x_print_print_get_attributes_attributes_length: LazySymbol<unsafe fn(R: *const xcb_x_print_print_get_attributes_reply_t) -> c_int>,
    pub(crate) xcb_x_print_print_get_attributes_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_x_print_print_get_attributes_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_x_print_print_get_attributes_reply_t>,
    pub(crate) xcb_x_print_print_get_attributes_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: xcb_x_print_pcontext_t, pool: u8) -> xcb_x_print_print_get_attributes_cookie_t>,
    pub(crate) xcb_x_print_print_get_context: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_x_print_print_get_context_cookie_t>,
    pub(crate) xcb_x_print_print_get_context_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_x_print_print_get_context_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_x_print_print_get_context_reply_t>,
    pub(crate) xcb_x_print_print_get_context_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_x_print_print_get_context_cookie_t>,
    pub(crate) xcb_x_print_print_get_document_data: LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: xcb_x_print_pcontext_t, max_bytes: u32) -> xcb_x_print_print_get_document_data_cookie_t>,
    pub(crate) xcb_x_print_print_get_document_data_data: LazySymbol<unsafe fn(R: *const xcb_x_print_print_get_document_data_reply_t) -> *mut u8>,
    pub(crate) xcb_x_print_print_get_document_data_data_end: LazySymbol<unsafe fn(R: *const xcb_x_print_print_get_document_data_reply_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_x_print_print_get_document_data_data_length: LazySymbol<unsafe fn(R: *const xcb_x_print_print_get_document_data_reply_t) -> c_int>,
    pub(crate) xcb_x_print_print_get_document_data_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_x_print_print_get_document_data_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_x_print_print_get_document_data_reply_t>,
    pub(crate) xcb_x_print_print_get_document_data_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: xcb_x_print_pcontext_t, max_bytes: u32) -> xcb_x_print_print_get_document_data_cookie_t>,
    pub(crate) xcb_x_print_print_get_image_resolution: LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: xcb_x_print_pcontext_t) -> xcb_x_print_print_get_image_resolution_cookie_t>,
    pub(crate) xcb_x_print_print_get_image_resolution_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_x_print_print_get_image_resolution_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_x_print_print_get_image_resolution_reply_t>,
    pub(crate) xcb_x_print_print_get_image_resolution_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: xcb_x_print_pcontext_t) -> xcb_x_print_print_get_image_resolution_cookie_t>,
    pub(crate) xcb_x_print_print_get_one_attributes: LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: xcb_x_print_pcontext_t, name_len: u32, pool: u8, name: *const xcb_x_print_string8_t) -> xcb_x_print_print_get_one_attributes_cookie_t>,
    pub(crate) xcb_x_print_print_get_one_attributes_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_x_print_print_get_one_attributes_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_x_print_print_get_one_attributes_reply_t>,
    pub(crate) xcb_x_print_print_get_one_attributes_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: xcb_x_print_pcontext_t, name_len: u32, pool: u8, name: *const xcb_x_print_string8_t) -> xcb_x_print_print_get_one_attributes_cookie_t>,
    pub(crate) xcb_x_print_print_get_one_attributes_value: LazySymbol<unsafe fn(R: *const xcb_x_print_print_get_one_attributes_reply_t) -> *mut xcb_x_print_string8_t>,
    pub(crate) xcb_x_print_print_get_one_attributes_value_end: LazySymbol<unsafe fn(R: *const xcb_x_print_print_get_one_attributes_reply_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_x_print_print_get_one_attributes_value_length: LazySymbol<unsafe fn(R: *const xcb_x_print_print_get_one_attributes_reply_t) -> c_int>,
    pub(crate) xcb_x_print_print_get_page_dimensions: LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: xcb_x_print_pcontext_t) -> xcb_x_print_print_get_page_dimensions_cookie_t>,
    pub(crate) xcb_x_print_print_get_page_dimensions_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_x_print_print_get_page_dimensions_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_x_print_print_get_page_dimensions_reply_t>,
    pub(crate) xcb_x_print_print_get_page_dimensions_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: xcb_x_print_pcontext_t) -> xcb_x_print_print_get_page_dimensions_cookie_t>,
    pub(crate) xcb_x_print_print_get_printer_list: LazySymbol<unsafe fn(c: *mut xcb_connection_t, printer_name_len: u32, locale_len: u32, printer_name: *const xcb_x_print_string8_t, locale: *const xcb_x_print_string8_t) -> xcb_x_print_print_get_printer_list_cookie_t>,
    pub(crate) xcb_x_print_print_get_printer_list_printers_iterator: LazySymbol<unsafe fn(R: *const xcb_x_print_print_get_printer_list_reply_t) -> xcb_x_print_printer_iterator_t>,
    pub(crate) xcb_x_print_print_get_printer_list_printers_length: LazySymbol<unsafe fn(R: *const xcb_x_print_print_get_printer_list_reply_t) -> c_int>,
    pub(crate) xcb_x_print_print_get_printer_list_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_x_print_print_get_printer_list_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_x_print_print_get_printer_list_reply_t>,
    pub(crate) xcb_x_print_print_get_printer_list_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, printer_name_len: u32, locale_len: u32, printer_name: *const xcb_x_print_string8_t, locale: *const xcb_x_print_string8_t) -> xcb_x_print_print_get_printer_list_cookie_t>,
    pub(crate) xcb_x_print_print_get_screen_of_context: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_x_print_print_get_screen_of_context_cookie_t>,
    pub(crate) xcb_x_print_print_get_screen_of_context_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_x_print_print_get_screen_of_context_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_x_print_print_get_screen_of_context_reply_t>,
    pub(crate) xcb_x_print_print_get_screen_of_context_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_x_print_print_get_screen_of_context_cookie_t>,
    pub(crate) xcb_x_print_print_input_selected: LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: xcb_x_print_pcontext_t) -> xcb_x_print_print_input_selected_cookie_t>,
    pub(crate) xcb_x_print_print_input_selected_all_events_list: LazySymbol<unsafe fn(R: *const xcb_x_print_print_input_selected_reply_t) -> *mut u32>,
    pub(crate) xcb_x_print_print_input_selected_all_events_list_end: LazySymbol<unsafe fn(R: *const xcb_x_print_print_input_selected_reply_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_x_print_print_input_selected_all_events_list_length: LazySymbol<unsafe fn(R: *const xcb_x_print_print_input_selected_reply_t) -> c_int>,
    pub(crate) xcb_x_print_print_input_selected_event_list: LazySymbol<unsafe fn(R: *const xcb_x_print_print_input_selected_reply_t) -> *mut u32>,
    pub(crate) xcb_x_print_print_input_selected_event_list_end: LazySymbol<unsafe fn(R: *const xcb_x_print_print_input_selected_reply_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_x_print_print_input_selected_event_list_length: LazySymbol<unsafe fn(R: *const xcb_x_print_print_input_selected_reply_t) -> c_int>,
    pub(crate) xcb_x_print_print_input_selected_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_x_print_print_input_selected_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_x_print_print_input_selected_reply_t>,
    pub(crate) xcb_x_print_print_input_selected_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: xcb_x_print_pcontext_t) -> xcb_x_print_print_input_selected_cookie_t>,
    pub(crate) xcb_x_print_print_put_document_data: LazySymbol<unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t, len_data: u32, len_fmt: u16, len_options: u16, data: *const u8, doc_format_len: u32, doc_format: *const xcb_x_print_string8_t, options_len: u32, options: *const xcb_x_print_string8_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_put_document_data_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t, len_data: u32, len_fmt: u16, len_options: u16, data: *const u8, doc_format_len: u32, doc_format: *const xcb_x_print_string8_t, options_len: u32, options: *const xcb_x_print_string8_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_query_screens: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_x_print_print_query_screens_cookie_t>,
    pub(crate) xcb_x_print_print_query_screens_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_x_print_print_query_screens_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_x_print_print_query_screens_reply_t>,
    pub(crate) xcb_x_print_print_query_screens_roots: LazySymbol<unsafe fn(R: *const xcb_x_print_print_query_screens_reply_t) -> *mut xcb_window_t>,
    pub(crate) xcb_x_print_print_query_screens_roots_end: LazySymbol<unsafe fn(R: *const xcb_x_print_print_query_screens_reply_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_x_print_print_query_screens_roots_length: LazySymbol<unsafe fn(R: *const xcb_x_print_print_query_screens_reply_t) -> c_int>,
    pub(crate) xcb_x_print_print_query_screens_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_x_print_print_query_screens_cookie_t>,
    pub(crate) xcb_x_print_print_query_version: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_x_print_print_query_version_cookie_t>,
    pub(crate) xcb_x_print_print_query_version_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_x_print_print_query_version_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_x_print_print_query_version_reply_t>,
    pub(crate) xcb_x_print_print_query_version_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_x_print_print_query_version_cookie_t>,
    pub(crate) xcb_x_print_print_rehash_printer_list: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_rehash_printer_list_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_select_input: LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: xcb_x_print_pcontext_t, event_mask: u32, event_list: *const u32) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_select_input_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: xcb_x_print_pcontext_t, event_mask: u32, event_list: *const u32) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_set_attributes: LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: xcb_x_print_pcontext_t, string_len: u32, pool: u8, rule: u8, attributes_len: u32, attributes: *const xcb_x_print_string8_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_set_attributes_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: xcb_x_print_pcontext_t, string_len: u32, pool: u8, rule: u8, attributes_len: u32, attributes: *const xcb_x_print_string8_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_set_context: LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: u32) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_set_context_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: u32) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_set_image_resolution: LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: xcb_x_print_pcontext_t, image_resolution: u16) -> xcb_x_print_print_set_image_resolution_cookie_t>,
    pub(crate) xcb_x_print_print_set_image_resolution_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_x_print_print_set_image_resolution_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_x_print_print_set_image_resolution_reply_t>,
    pub(crate) xcb_x_print_print_set_image_resolution_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, context: xcb_x_print_pcontext_t, image_resolution: u16) -> xcb_x_print_print_set_image_resolution_cookie_t>,
    pub(crate) xcb_x_print_print_start_doc: LazySymbol<unsafe fn(c: *mut xcb_connection_t, driver_mode: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_start_doc_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, driver_mode: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_start_job: LazySymbol<unsafe fn(c: *mut xcb_connection_t, output_mode: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_start_job_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, output_mode: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_start_page: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_print_start_page_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_x_print_printer_description: LazySymbol<unsafe fn(R: *const xcb_x_print_printer_t) -> *mut xcb_x_print_string8_t>,
    pub(crate) xcb_x_print_printer_description_end: LazySymbol<unsafe fn(R: *const xcb_x_print_printer_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_x_print_printer_description_length: LazySymbol<unsafe fn(R: *const xcb_x_print_printer_t) -> c_int>,
    pub(crate) xcb_x_print_printer_end: LazySymbol<unsafe fn(i: *mut xcb_x_print_printer_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_x_print_printer_name: LazySymbol<unsafe fn(R: *const xcb_x_print_printer_t) -> *mut xcb_x_print_string8_t>,
    pub(crate) xcb_x_print_printer_name_end: LazySymbol<unsafe fn(R: *const xcb_x_print_printer_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_x_print_printer_name_length: LazySymbol<unsafe fn(R: *const xcb_x_print_printer_t) -> c_int>,
    pub(crate) xcb_x_print_printer_next: LazySymbol<unsafe fn(i: *mut xcb_x_print_printer_iterator_t)>,
    pub(crate) xcb_x_print_string8_end: LazySymbol<unsafe fn(i: *mut xcb_x_print_string8_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_x_print_string8_next: LazySymbol<unsafe fn(i: *mut xcb_x_print_string8_iterator_t)>,
}