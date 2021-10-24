use crate::ffi::*;
use crate::lazy::*;
use libloading::{Error, Library};
use std::os::raw::*;

#[rustfmt::skip]
pub struct XcbSync {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_sync_alarm_end: LazySymbol<unsafe fn(i: *mut xcb_sync_alarm_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_sync_alarm_next: LazySymbol<unsafe fn(i: *mut xcb_sync_alarm_iterator_t)>,
    pub(crate) xcb_sync_await: LazySymbol<unsafe fn(c: *mut xcb_connection_t, wait_list_len: u32, wait_list: *const xcb_sync_waitcondition_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_await_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, wait_list_len: u32, wait_list: *const xcb_sync_waitcondition_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_await_fence: LazySymbol<unsafe fn(c: *mut xcb_connection_t, fence_list_len: u32, fence_list: *const xcb_sync_fence_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_await_fence_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, fence_list_len: u32, fence_list: *const xcb_sync_fence_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_change_alarm: LazySymbol<unsafe fn(c: *mut xcb_connection_t, id: xcb_sync_alarm_t, value_mask: u32, value_list: *const xcb_sync_change_alarm_value_list_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_change_alarm_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, id: xcb_sync_alarm_t, value_mask: u32, value_list: *const xcb_sync_change_alarm_value_list_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_change_counter: LazySymbol<unsafe fn(c: *mut xcb_connection_t, counter: xcb_sync_counter_t, amount: xcb_sync_int64_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_change_counter_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, counter: xcb_sync_counter_t, amount: xcb_sync_int64_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_counter_end: LazySymbol<unsafe fn(i: *mut xcb_sync_counter_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_sync_counter_next: LazySymbol<unsafe fn(i: *mut xcb_sync_counter_iterator_t)>,
    pub(crate) xcb_sync_create_alarm: LazySymbol<unsafe fn(c: *mut xcb_connection_t, id: xcb_sync_alarm_t, value_mask: u32, value_list: *const xcb_sync_create_alarm_value_list_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_create_alarm_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, id: xcb_sync_alarm_t, value_mask: u32, value_list: *const xcb_sync_create_alarm_value_list_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_create_counter: LazySymbol<unsafe fn(c: *mut xcb_connection_t, id: xcb_sync_counter_t, initial_value: xcb_sync_int64_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_create_counter_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, id: xcb_sync_counter_t, initial_value: xcb_sync_int64_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_create_fence: LazySymbol<unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t, fence: xcb_sync_fence_t, initially_triggered: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_create_fence_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t, fence: xcb_sync_fence_t, initially_triggered: u8) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_destroy_alarm: LazySymbol<unsafe fn(c: *mut xcb_connection_t, alarm: xcb_sync_alarm_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_destroy_alarm_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, alarm: xcb_sync_alarm_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_destroy_counter: LazySymbol<unsafe fn(c: *mut xcb_connection_t, counter: xcb_sync_counter_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_destroy_counter_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, counter: xcb_sync_counter_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_destroy_fence: LazySymbol<unsafe fn(c: *mut xcb_connection_t, fence: xcb_sync_fence_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_destroy_fence_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, fence: xcb_sync_fence_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_fence_end: LazySymbol<unsafe fn(i: *mut xcb_sync_fence_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_sync_fence_next: LazySymbol<unsafe fn(i: *mut xcb_sync_fence_iterator_t)>,
    pub(crate) xcb_sync_get_priority: LazySymbol<unsafe fn(c: *mut xcb_connection_t, id: u32) -> xcb_sync_get_priority_cookie_t>,
    pub(crate) xcb_sync_get_priority_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_sync_get_priority_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_sync_get_priority_reply_t>,
    pub(crate) xcb_sync_get_priority_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, id: u32) -> xcb_sync_get_priority_cookie_t>,
    pub(crate) xcb_sync_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_sync_initialize: LazySymbol<unsafe fn(c: *mut xcb_connection_t, desired_major_version: u8, desired_minor_version: u8) -> xcb_sync_initialize_cookie_t>,
    pub(crate) xcb_sync_initialize_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_sync_initialize_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_sync_initialize_reply_t>,
    pub(crate) xcb_sync_initialize_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, desired_major_version: u8, desired_minor_version: u8) -> xcb_sync_initialize_cookie_t>,
    pub(crate) xcb_sync_int64_end: LazySymbol<unsafe fn(i: *mut xcb_sync_int64_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_sync_int64_next: LazySymbol<unsafe fn(i: *mut xcb_sync_int64_iterator_t)>,
    pub(crate) xcb_sync_list_system_counters: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_sync_list_system_counters_cookie_t>,
    pub(crate) xcb_sync_list_system_counters_counters_iterator: LazySymbol<unsafe fn(R: *const xcb_sync_list_system_counters_reply_t) -> xcb_sync_systemcounter_iterator_t>,
    pub(crate) xcb_sync_list_system_counters_counters_length: LazySymbol<unsafe fn(R: *const xcb_sync_list_system_counters_reply_t) -> c_int>,
    pub(crate) xcb_sync_list_system_counters_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_sync_list_system_counters_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_sync_list_system_counters_reply_t>,
    pub(crate) xcb_sync_list_system_counters_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_sync_list_system_counters_cookie_t>,
    pub(crate) xcb_sync_query_alarm: LazySymbol<unsafe fn(c: *mut xcb_connection_t, alarm: xcb_sync_alarm_t) -> xcb_sync_query_alarm_cookie_t>,
    pub(crate) xcb_sync_query_alarm_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_sync_query_alarm_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_sync_query_alarm_reply_t>,
    pub(crate) xcb_sync_query_alarm_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, alarm: xcb_sync_alarm_t) -> xcb_sync_query_alarm_cookie_t>,
    pub(crate) xcb_sync_query_counter: LazySymbol<unsafe fn(c: *mut xcb_connection_t, counter: xcb_sync_counter_t) -> xcb_sync_query_counter_cookie_t>,
    pub(crate) xcb_sync_query_counter_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_sync_query_counter_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_sync_query_counter_reply_t>,
    pub(crate) xcb_sync_query_counter_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, counter: xcb_sync_counter_t) -> xcb_sync_query_counter_cookie_t>,
    pub(crate) xcb_sync_query_fence: LazySymbol<unsafe fn(c: *mut xcb_connection_t, fence: xcb_sync_fence_t) -> xcb_sync_query_fence_cookie_t>,
    pub(crate) xcb_sync_query_fence_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_sync_query_fence_cookie_t, error: *mut *mut xcb_generic_error_t) -> *mut xcb_sync_query_fence_reply_t>,
    pub(crate) xcb_sync_query_fence_unchecked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, fence: xcb_sync_fence_t) -> xcb_sync_query_fence_cookie_t>,
    pub(crate) xcb_sync_reset_fence: LazySymbol<unsafe fn(c: *mut xcb_connection_t, fence: xcb_sync_fence_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_reset_fence_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, fence: xcb_sync_fence_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_set_counter: LazySymbol<unsafe fn(c: *mut xcb_connection_t, counter: xcb_sync_counter_t, value: xcb_sync_int64_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_set_counter_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, counter: xcb_sync_counter_t, value: xcb_sync_int64_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_set_priority: LazySymbol<unsafe fn(c: *mut xcb_connection_t, id: u32, priority: i32) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_set_priority_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, id: u32, priority: i32) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_systemcounter_end: LazySymbol<unsafe fn(i: *mut xcb_sync_systemcounter_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_sync_systemcounter_name: LazySymbol<unsafe fn(R: *const xcb_sync_systemcounter_t) -> *mut c_char>,
    pub(crate) xcb_sync_systemcounter_name_end: LazySymbol<unsafe fn(R: *const xcb_sync_systemcounter_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_sync_systemcounter_name_length: LazySymbol<unsafe fn(R: *const xcb_sync_systemcounter_t) -> c_int>,
    pub(crate) xcb_sync_systemcounter_next: LazySymbol<unsafe fn(i: *mut xcb_sync_systemcounter_iterator_t)>,
    pub(crate) xcb_sync_trigger_end: LazySymbol<unsafe fn(i: *mut xcb_sync_trigger_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_sync_trigger_fence: LazySymbol<unsafe fn(c: *mut xcb_connection_t, fence: xcb_sync_fence_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_trigger_fence_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t, fence: xcb_sync_fence_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_sync_trigger_next: LazySymbol<unsafe fn(i: *mut xcb_sync_trigger_iterator_t)>,
    pub(crate) xcb_sync_waitcondition_end: LazySymbol<unsafe fn(i: *mut xcb_sync_waitcondition_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_sync_waitcondition_next: LazySymbol<unsafe fn(i: *mut xcb_sync_waitcondition_iterator_t)>,
}
