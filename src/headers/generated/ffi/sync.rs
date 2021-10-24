use crate::*;
use std::os::raw::*;

pub const XCB_SYNC_MAJOR_VERSION: u32 = 3;
pub const XCB_SYNC_MINOR_VERSION: u32 = 1;

pub type xcb_sync_alarm_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_alarm_iterator_t {
    pub data: *mut xcb_sync_alarm_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_sync_alarmstate_t = u32;
pub const XCB_SYNC_ALARMSTATE_ACTIVE: xcb_sync_alarmstate_t = 0x00;
pub const XCB_SYNC_ALARMSTATE_INACTIVE: xcb_sync_alarmstate_t = 0x01;
pub const XCB_SYNC_ALARMSTATE_DESTROYED: xcb_sync_alarmstate_t = 0x02;

pub type xcb_sync_counter_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_counter_iterator_t {
    pub data: *mut xcb_sync_counter_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_sync_fence_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_fence_iterator_t {
    pub data: *mut xcb_sync_fence_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_sync_testtype_t = u32;
pub const XCB_SYNC_TESTTYPE_POSITIVE_TRANSITION: xcb_sync_testtype_t = 0x00;
pub const XCB_SYNC_TESTTYPE_NEGATIVE_TRANSITION: xcb_sync_testtype_t = 0x01;
pub const XCB_SYNC_TESTTYPE_POSITIVE_COMPARISON: xcb_sync_testtype_t = 0x02;
pub const XCB_SYNC_TESTTYPE_NEGATIVE_COMPARISON: xcb_sync_testtype_t = 0x03;

pub type xcb_sync_valuetype_t = u32;
pub const XCB_SYNC_VALUETYPE_ABSOLUTE: xcb_sync_valuetype_t = 0x00;
pub const XCB_SYNC_VALUETYPE_RELATIVE: xcb_sync_valuetype_t = 0x01;

pub type xcb_sync_ca_t = u32;
pub const XCB_SYNC_CA_COUNTER: xcb_sync_ca_t = 0x01;
pub const XCB_SYNC_CA_VALUE_TYPE: xcb_sync_ca_t = 0x02;
pub const XCB_SYNC_CA_VALUE: xcb_sync_ca_t = 0x04;
pub const XCB_SYNC_CA_TEST_TYPE: xcb_sync_ca_t = 0x08;
pub const XCB_SYNC_CA_DELTA: xcb_sync_ca_t = 0x10;
pub const XCB_SYNC_CA_EVENTS: xcb_sync_ca_t = 0x20;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_int64_t {
    pub hi: i32,
    pub lo: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_int64_iterator_t {
    pub data: *mut xcb_sync_int64_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_systemcounter_t {
    pub counter: xcb_sync_counter_t,
    pub resolution: xcb_sync_int64_t,
    pub name_len: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_systemcounter_iterator_t {
    pub data: *mut xcb_sync_systemcounter_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_trigger_t {
    pub counter: xcb_sync_counter_t,
    pub wait_type: u32,
    pub wait_value: xcb_sync_int64_t,
    pub test_type: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_trigger_iterator_t {
    pub data: *mut xcb_sync_trigger_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_waitcondition_t {
    pub trigger: xcb_sync_trigger_t,
    pub event_threshold: xcb_sync_int64_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_waitcondition_iterator_t {
    pub data: *mut xcb_sync_waitcondition_t,
    pub rem: c_int,
    pub index: c_int,
}

pub const XCB_SYNC_COUNTER: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_counter_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_counter: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}

pub const XCB_SYNC_ALARM: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_alarm_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_alarm: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}

pub const XCB_SYNC_INITIALIZE: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_initialize_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub desired_major_version: u8,
    pub desired_minor_version: u8,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_initialize_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_initialize_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u8,
    pub minor_version: u8,
    pub pad1: [u8; 22],
}

pub const XCB_SYNC_LIST_SYSTEM_COUNTERS: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_list_system_counters_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_list_system_counters_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_list_system_counters_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub counters_len: u32,
    pub pad1: [u8; 20],
}

pub const XCB_SYNC_CREATE_COUNTER: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_create_counter_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub id: xcb_sync_counter_t,
    pub initial_value: xcb_sync_int64_t,
}

pub const XCB_SYNC_DESTROY_COUNTER: u8 = 6;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_destroy_counter_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub counter: xcb_sync_counter_t,
}

pub const XCB_SYNC_QUERY_COUNTER: u8 = 5;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_query_counter_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub counter: xcb_sync_counter_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_query_counter_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_query_counter_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub counter_value: xcb_sync_int64_t,
}

pub const XCB_SYNC_AWAIT: u8 = 7;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_await_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

pub const XCB_SYNC_CHANGE_COUNTER: u8 = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_change_counter_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub counter: xcb_sync_counter_t,
    pub amount: xcb_sync_int64_t,
}

pub const XCB_SYNC_SET_COUNTER: u8 = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_set_counter_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub counter: xcb_sync_counter_t,
    pub value: xcb_sync_int64_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_create_alarm_value_list_t {
    pub counter: xcb_sync_counter_t,
    pub value_type: u32,
    pub value: xcb_sync_int64_t,
    pub test_type: u32,
    pub delta: xcb_sync_int64_t,
    pub events: u32,
}

pub const XCB_SYNC_CREATE_ALARM: u8 = 8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_create_alarm_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub id: xcb_sync_alarm_t,
    pub value_mask: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_change_alarm_value_list_t {
    pub counter: xcb_sync_counter_t,
    pub value_type: u32,
    pub value: xcb_sync_int64_t,
    pub test_type: u32,
    pub delta: xcb_sync_int64_t,
    pub events: u32,
}

pub const XCB_SYNC_CHANGE_ALARM: u8 = 9;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_change_alarm_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub id: xcb_sync_alarm_t,
    pub value_mask: u32,
}

pub const XCB_SYNC_DESTROY_ALARM: u8 = 11;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_destroy_alarm_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub alarm: xcb_sync_alarm_t,
}

pub const XCB_SYNC_QUERY_ALARM: u8 = 10;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_query_alarm_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub alarm: xcb_sync_alarm_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_query_alarm_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_query_alarm_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub trigger: xcb_sync_trigger_t,
    pub delta: xcb_sync_int64_t,
    pub events: u8,
    pub state: u8,
    pub pad1: [u8; 2],
}

pub const XCB_SYNC_SET_PRIORITY: u8 = 12;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_set_priority_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub id: u32,
    pub priority: i32,
}

pub const XCB_SYNC_GET_PRIORITY: u8 = 13;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_get_priority_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub id: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_get_priority_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_get_priority_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub priority: i32,
}

pub const XCB_SYNC_CREATE_FENCE: u8 = 14;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_create_fence_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub fence: xcb_sync_fence_t,
    pub initially_triggered: u8,
}

pub const XCB_SYNC_TRIGGER_FENCE: u8 = 15;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_trigger_fence_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub fence: xcb_sync_fence_t,
}

pub const XCB_SYNC_RESET_FENCE: u8 = 16;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_reset_fence_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub fence: xcb_sync_fence_t,
}

pub const XCB_SYNC_DESTROY_FENCE: u8 = 17;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_destroy_fence_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub fence: xcb_sync_fence_t,
}

pub const XCB_SYNC_QUERY_FENCE: u8 = 18;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_query_fence_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub fence: xcb_sync_fence_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_query_fence_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_query_fence_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub triggered: u8,
    pub pad1: [u8; 23],
}

pub const XCB_SYNC_AWAIT_FENCE: u8 = 19;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_await_fence_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

pub const XCB_SYNC_COUNTER_NOTIFY: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_counter_notify_event_t {
    pub response_type: u8,
    pub kind: u8,
    pub sequence: u16,
    pub counter: xcb_sync_counter_t,
    pub wait_value: xcb_sync_int64_t,
    pub counter_value: xcb_sync_int64_t,
    pub timestamp: xcb_timestamp_t,
    pub count: u16,
    pub destroyed: u8,
    pub pad0: u8,
}

pub const XCB_SYNC_ALARM_NOTIFY: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_alarm_notify_event_t {
    pub response_type: u8,
    pub kind: u8,
    pub sequence: u16,
    pub alarm: xcb_sync_alarm_t,
    pub counter_value: xcb_sync_int64_t,
    pub alarm_value: xcb_sync_int64_t,
    pub timestamp: xcb_timestamp_t,
    pub state: u8,
    pub pad0: [u8; 3],
}

impl XcbSync {
    #[inline]
    pub fn xcb_sync_id(&self) -> *mut xcb_extension_t {
        call!(self, xcb_sync_id)
    }

    #[inline]
    pub unsafe fn xcb_sync_alarm_next(&self, i: *mut xcb_sync_alarm_iterator_t) {
        call!(self, xcb_sync_alarm_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_sync_alarm_end(
        &self,
        i: *mut xcb_sync_alarm_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_sync_alarm_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_sync_counter_next(&self, i: *mut xcb_sync_counter_iterator_t) {
        call!(self, xcb_sync_counter_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_sync_counter_end(
        &self,
        i: *mut xcb_sync_counter_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_sync_counter_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_sync_fence_next(&self, i: *mut xcb_sync_fence_iterator_t) {
        call!(self, xcb_sync_fence_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_sync_fence_end(
        &self,
        i: *mut xcb_sync_fence_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_sync_fence_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_sync_int64_next(&self, i: *mut xcb_sync_int64_iterator_t) {
        call!(self, xcb_sync_int64_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_sync_int64_end(
        &self,
        i: *mut xcb_sync_int64_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_sync_int64_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_sync_systemcounter_name(
        &self,
        R: *const xcb_sync_systemcounter_t,
    ) -> *mut c_char {
        call!(self, xcb_sync_systemcounter_name)(R)
    }

    #[inline]
    pub unsafe fn xcb_sync_systemcounter_name_length(
        &self,
        R: *const xcb_sync_systemcounter_t,
    ) -> c_int {
        call!(self, xcb_sync_systemcounter_name_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_sync_systemcounter_name_end(
        &self,
        R: *const xcb_sync_systemcounter_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_sync_systemcounter_name_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_sync_systemcounter_next(&self, i: *mut xcb_sync_systemcounter_iterator_t) {
        call!(self, xcb_sync_systemcounter_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_sync_systemcounter_end(
        &self,
        i: *mut xcb_sync_systemcounter_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_sync_systemcounter_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_sync_trigger_next(&self, i: *mut xcb_sync_trigger_iterator_t) {
        call!(self, xcb_sync_trigger_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_sync_trigger_end(
        &self,
        i: *mut xcb_sync_trigger_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_sync_trigger_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_sync_waitcondition_next(&self, i: *mut xcb_sync_waitcondition_iterator_t) {
        call!(self, xcb_sync_waitcondition_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_sync_waitcondition_end(
        &self,
        i: *mut xcb_sync_waitcondition_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_sync_waitcondition_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_sync_initialize_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_sync_initialize_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_sync_initialize_reply_t {
        call!(self, xcb_sync_initialize_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_sync_initialize(
        &self,
        c: *mut xcb_connection_t,
        desired_major_version: u8,
        desired_minor_version: u8,
    ) -> xcb_sync_initialize_cookie_t {
        call!(self, xcb_sync_initialize)(c, desired_major_version, desired_minor_version)
    }

    #[inline]
    pub unsafe fn xcb_sync_initialize_unchecked(
        &self,
        c: *mut xcb_connection_t,
        desired_major_version: u8,
        desired_minor_version: u8,
    ) -> xcb_sync_initialize_cookie_t {
        call!(self, xcb_sync_initialize_unchecked)(c, desired_major_version, desired_minor_version)
    }

    #[inline]
    pub unsafe fn xcb_sync_list_system_counters_counters_length(
        &self,
        R: *const xcb_sync_list_system_counters_reply_t,
    ) -> c_int {
        call!(self, xcb_sync_list_system_counters_counters_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_sync_list_system_counters_counters_iterator(
        &self,
        R: *const xcb_sync_list_system_counters_reply_t,
    ) -> xcb_sync_systemcounter_iterator_t {
        call!(self, xcb_sync_list_system_counters_counters_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_sync_list_system_counters_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_sync_list_system_counters_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_sync_list_system_counters_reply_t {
        call!(self, xcb_sync_list_system_counters_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_sync_list_system_counters(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_sync_list_system_counters_cookie_t {
        call!(self, xcb_sync_list_system_counters)(c)
    }

    #[inline]
    pub unsafe fn xcb_sync_list_system_counters_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_sync_list_system_counters_cookie_t {
        call!(self, xcb_sync_list_system_counters_unchecked)(c)
    }

    #[inline]
    pub unsafe fn xcb_sync_create_counter(
        &self,
        c: *mut xcb_connection_t,
        id: xcb_sync_counter_t,
        initial_value: xcb_sync_int64_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_create_counter)(c, id, initial_value)
    }

    #[inline]
    pub unsafe fn xcb_sync_create_counter_checked(
        &self,
        c: *mut xcb_connection_t,
        id: xcb_sync_counter_t,
        initial_value: xcb_sync_int64_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_create_counter_checked)(c, id, initial_value)
    }

    #[inline]
    pub unsafe fn xcb_sync_destroy_counter(
        &self,
        c: *mut xcb_connection_t,
        counter: xcb_sync_counter_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_destroy_counter)(c, counter)
    }

    #[inline]
    pub unsafe fn xcb_sync_destroy_counter_checked(
        &self,
        c: *mut xcb_connection_t,
        counter: xcb_sync_counter_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_destroy_counter_checked)(c, counter)
    }

    #[inline]
    pub unsafe fn xcb_sync_query_counter_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_sync_query_counter_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_sync_query_counter_reply_t {
        call!(self, xcb_sync_query_counter_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_sync_query_counter(
        &self,
        c: *mut xcb_connection_t,
        counter: xcb_sync_counter_t,
    ) -> xcb_sync_query_counter_cookie_t {
        call!(self, xcb_sync_query_counter)(c, counter)
    }

    #[inline]
    pub unsafe fn xcb_sync_query_counter_unchecked(
        &self,
        c: *mut xcb_connection_t,
        counter: xcb_sync_counter_t,
    ) -> xcb_sync_query_counter_cookie_t {
        call!(self, xcb_sync_query_counter_unchecked)(c, counter)
    }

    #[inline]
    pub unsafe fn xcb_sync_await(
        &self,
        c: *mut xcb_connection_t,
        wait_list_len: u32,
        wait_list: *const xcb_sync_waitcondition_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_await)(c, wait_list_len, wait_list)
    }

    #[inline]
    pub unsafe fn xcb_sync_await_checked(
        &self,
        c: *mut xcb_connection_t,
        wait_list_len: u32,
        wait_list: *const xcb_sync_waitcondition_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_await_checked)(c, wait_list_len, wait_list)
    }

    #[inline]
    pub unsafe fn xcb_sync_change_counter(
        &self,
        c: *mut xcb_connection_t,
        counter: xcb_sync_counter_t,
        amount: xcb_sync_int64_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_change_counter)(c, counter, amount)
    }

    #[inline]
    pub unsafe fn xcb_sync_change_counter_checked(
        &self,
        c: *mut xcb_connection_t,
        counter: xcb_sync_counter_t,
        amount: xcb_sync_int64_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_change_counter_checked)(c, counter, amount)
    }

    #[inline]
    pub unsafe fn xcb_sync_set_counter(
        &self,
        c: *mut xcb_connection_t,
        counter: xcb_sync_counter_t,
        value: xcb_sync_int64_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_set_counter)(c, counter, value)
    }

    #[inline]
    pub unsafe fn xcb_sync_set_counter_checked(
        &self,
        c: *mut xcb_connection_t,
        counter: xcb_sync_counter_t,
        value: xcb_sync_int64_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_set_counter_checked)(c, counter, value)
    }

    #[inline]
    pub unsafe fn xcb_sync_create_alarm(
        &self,
        c: *mut xcb_connection_t,
        id: xcb_sync_alarm_t,
        value_mask: u32,
        value_list: *const xcb_sync_create_alarm_value_list_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_create_alarm)(c, id, value_mask, value_list)
    }

    #[inline]
    pub unsafe fn xcb_sync_create_alarm_checked(
        &self,
        c: *mut xcb_connection_t,
        id: xcb_sync_alarm_t,
        value_mask: u32,
        value_list: *const xcb_sync_create_alarm_value_list_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_create_alarm_checked)(c, id, value_mask, value_list)
    }

    #[inline]
    pub unsafe fn xcb_sync_change_alarm(
        &self,
        c: *mut xcb_connection_t,
        id: xcb_sync_alarm_t,
        value_mask: u32,
        value_list: *const xcb_sync_change_alarm_value_list_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_change_alarm)(c, id, value_mask, value_list)
    }

    #[inline]
    pub unsafe fn xcb_sync_change_alarm_checked(
        &self,
        c: *mut xcb_connection_t,
        id: xcb_sync_alarm_t,
        value_mask: u32,
        value_list: *const xcb_sync_change_alarm_value_list_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_change_alarm_checked)(c, id, value_mask, value_list)
    }

    #[inline]
    pub unsafe fn xcb_sync_destroy_alarm(
        &self,
        c: *mut xcb_connection_t,
        alarm: xcb_sync_alarm_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_destroy_alarm)(c, alarm)
    }

    #[inline]
    pub unsafe fn xcb_sync_destroy_alarm_checked(
        &self,
        c: *mut xcb_connection_t,
        alarm: xcb_sync_alarm_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_destroy_alarm_checked)(c, alarm)
    }

    #[inline]
    pub unsafe fn xcb_sync_query_alarm_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_sync_query_alarm_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_sync_query_alarm_reply_t {
        call!(self, xcb_sync_query_alarm_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_sync_query_alarm(
        &self,
        c: *mut xcb_connection_t,
        alarm: xcb_sync_alarm_t,
    ) -> xcb_sync_query_alarm_cookie_t {
        call!(self, xcb_sync_query_alarm)(c, alarm)
    }

    #[inline]
    pub unsafe fn xcb_sync_query_alarm_unchecked(
        &self,
        c: *mut xcb_connection_t,
        alarm: xcb_sync_alarm_t,
    ) -> xcb_sync_query_alarm_cookie_t {
        call!(self, xcb_sync_query_alarm_unchecked)(c, alarm)
    }

    #[inline]
    pub unsafe fn xcb_sync_set_priority(
        &self,
        c: *mut xcb_connection_t,
        id: u32,
        priority: i32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_set_priority)(c, id, priority)
    }

    #[inline]
    pub unsafe fn xcb_sync_set_priority_checked(
        &self,
        c: *mut xcb_connection_t,
        id: u32,
        priority: i32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_set_priority_checked)(c, id, priority)
    }

    #[inline]
    pub unsafe fn xcb_sync_get_priority_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_sync_get_priority_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_sync_get_priority_reply_t {
        call!(self, xcb_sync_get_priority_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_sync_get_priority(
        &self,
        c: *mut xcb_connection_t,
        id: u32,
    ) -> xcb_sync_get_priority_cookie_t {
        call!(self, xcb_sync_get_priority)(c, id)
    }

    #[inline]
    pub unsafe fn xcb_sync_get_priority_unchecked(
        &self,
        c: *mut xcb_connection_t,
        id: u32,
    ) -> xcb_sync_get_priority_cookie_t {
        call!(self, xcb_sync_get_priority_unchecked)(c, id)
    }

    #[inline]
    pub unsafe fn xcb_sync_create_fence(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        fence: xcb_sync_fence_t,
        initially_triggered: u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_create_fence)(c, drawable, fence, initially_triggered)
    }

    #[inline]
    pub unsafe fn xcb_sync_create_fence_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        fence: xcb_sync_fence_t,
        initially_triggered: u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_create_fence_checked)(c, drawable, fence, initially_triggered)
    }

    #[inline]
    pub unsafe fn xcb_sync_trigger_fence(
        &self,
        c: *mut xcb_connection_t,
        fence: xcb_sync_fence_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_trigger_fence)(c, fence)
    }

    #[inline]
    pub unsafe fn xcb_sync_trigger_fence_checked(
        &self,
        c: *mut xcb_connection_t,
        fence: xcb_sync_fence_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_trigger_fence_checked)(c, fence)
    }

    #[inline]
    pub unsafe fn xcb_sync_reset_fence(
        &self,
        c: *mut xcb_connection_t,
        fence: xcb_sync_fence_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_reset_fence)(c, fence)
    }

    #[inline]
    pub unsafe fn xcb_sync_reset_fence_checked(
        &self,
        c: *mut xcb_connection_t,
        fence: xcb_sync_fence_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_reset_fence_checked)(c, fence)
    }

    #[inline]
    pub unsafe fn xcb_sync_destroy_fence(
        &self,
        c: *mut xcb_connection_t,
        fence: xcb_sync_fence_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_destroy_fence)(c, fence)
    }

    #[inline]
    pub unsafe fn xcb_sync_destroy_fence_checked(
        &self,
        c: *mut xcb_connection_t,
        fence: xcb_sync_fence_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_destroy_fence_checked)(c, fence)
    }

    #[inline]
    pub unsafe fn xcb_sync_query_fence_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_sync_query_fence_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_sync_query_fence_reply_t {
        call!(self, xcb_sync_query_fence_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_sync_query_fence(
        &self,
        c: *mut xcb_connection_t,
        fence: xcb_sync_fence_t,
    ) -> xcb_sync_query_fence_cookie_t {
        call!(self, xcb_sync_query_fence)(c, fence)
    }

    #[inline]
    pub unsafe fn xcb_sync_query_fence_unchecked(
        &self,
        c: *mut xcb_connection_t,
        fence: xcb_sync_fence_t,
    ) -> xcb_sync_query_fence_cookie_t {
        call!(self, xcb_sync_query_fence_unchecked)(c, fence)
    }

    #[inline]
    pub unsafe fn xcb_sync_await_fence(
        &self,
        c: *mut xcb_connection_t,
        fence_list_len: u32,
        fence_list: *const xcb_sync_fence_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_await_fence)(c, fence_list_len, fence_list)
    }

    #[inline]
    pub unsafe fn xcb_sync_await_fence_checked(
        &self,
        c: *mut xcb_connection_t,
        fence_list_len: u32,
        fence_list: *const xcb_sync_fence_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_sync_await_fence_checked)(c, fence_list_len, fence_list)
    }
}

pub struct XcbSync {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_sync_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_sync_alarm_next: LazySymbol<unsafe fn(i: *mut xcb_sync_alarm_iterator_t)>,
    pub(crate) xcb_sync_alarm_end:
        LazySymbol<unsafe fn(i: *mut xcb_sync_alarm_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_sync_counter_next: LazySymbol<unsafe fn(i: *mut xcb_sync_counter_iterator_t)>,
    pub(crate) xcb_sync_counter_end:
        LazySymbol<unsafe fn(i: *mut xcb_sync_counter_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_sync_fence_next: LazySymbol<unsafe fn(i: *mut xcb_sync_fence_iterator_t)>,
    pub(crate) xcb_sync_fence_end:
        LazySymbol<unsafe fn(i: *mut xcb_sync_fence_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_sync_int64_next: LazySymbol<unsafe fn(i: *mut xcb_sync_int64_iterator_t)>,
    pub(crate) xcb_sync_int64_end:
        LazySymbol<unsafe fn(i: *mut xcb_sync_int64_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_sync_systemcounter_name:
        LazySymbol<unsafe fn(R: *const xcb_sync_systemcounter_t) -> *mut c_char>,
    pub(crate) xcb_sync_systemcounter_name_length:
        LazySymbol<unsafe fn(R: *const xcb_sync_systemcounter_t) -> c_int>,
    pub(crate) xcb_sync_systemcounter_name_end:
        LazySymbol<unsafe fn(R: *const xcb_sync_systemcounter_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_sync_systemcounter_next:
        LazySymbol<unsafe fn(i: *mut xcb_sync_systemcounter_iterator_t)>,
    pub(crate) xcb_sync_systemcounter_end:
        LazySymbol<unsafe fn(i: *mut xcb_sync_systemcounter_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_sync_trigger_next: LazySymbol<unsafe fn(i: *mut xcb_sync_trigger_iterator_t)>,
    pub(crate) xcb_sync_trigger_end:
        LazySymbol<unsafe fn(i: *mut xcb_sync_trigger_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_sync_waitcondition_next:
        LazySymbol<unsafe fn(i: *mut xcb_sync_waitcondition_iterator_t)>,
    pub(crate) xcb_sync_waitcondition_end:
        LazySymbol<unsafe fn(i: *mut xcb_sync_waitcondition_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_sync_initialize_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_sync_initialize_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_sync_initialize_reply_t,
    >,
    pub(crate) xcb_sync_initialize: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            desired_major_version: u8,
            desired_minor_version: u8,
        ) -> xcb_sync_initialize_cookie_t,
    >,
    pub(crate) xcb_sync_initialize_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            desired_major_version: u8,
            desired_minor_version: u8,
        ) -> xcb_sync_initialize_cookie_t,
    >,
    pub(crate) xcb_sync_list_system_counters_counters_length:
        LazySymbol<unsafe fn(R: *const xcb_sync_list_system_counters_reply_t) -> c_int>,
    pub(crate) xcb_sync_list_system_counters_counters_iterator: LazySymbol<
        unsafe fn(
            R: *const xcb_sync_list_system_counters_reply_t,
        ) -> xcb_sync_systemcounter_iterator_t,
    >,
    pub(crate) xcb_sync_list_system_counters_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_sync_list_system_counters_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_sync_list_system_counters_reply_t,
    >,
    pub(crate) xcb_sync_list_system_counters:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_sync_list_system_counters_cookie_t>,
    pub(crate) xcb_sync_list_system_counters_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_sync_list_system_counters_cookie_t>,
    pub(crate) xcb_sync_create_counter: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            id: xcb_sync_counter_t,
            initial_value: xcb_sync_int64_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_create_counter_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            id: xcb_sync_counter_t,
            initial_value: xcb_sync_int64_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_destroy_counter: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, counter: xcb_sync_counter_t) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_destroy_counter_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, counter: xcb_sync_counter_t) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_query_counter_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_sync_query_counter_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_sync_query_counter_reply_t,
    >,
    pub(crate) xcb_sync_query_counter: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            counter: xcb_sync_counter_t,
        ) -> xcb_sync_query_counter_cookie_t,
    >,
    pub(crate) xcb_sync_query_counter_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            counter: xcb_sync_counter_t,
        ) -> xcb_sync_query_counter_cookie_t,
    >,
    pub(crate) xcb_sync_await: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            wait_list_len: u32,
            wait_list: *const xcb_sync_waitcondition_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_await_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            wait_list_len: u32,
            wait_list: *const xcb_sync_waitcondition_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_change_counter: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            counter: xcb_sync_counter_t,
            amount: xcb_sync_int64_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_change_counter_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            counter: xcb_sync_counter_t,
            amount: xcb_sync_int64_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_set_counter: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            counter: xcb_sync_counter_t,
            value: xcb_sync_int64_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_set_counter_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            counter: xcb_sync_counter_t,
            value: xcb_sync_int64_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_create_alarm: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            id: xcb_sync_alarm_t,
            value_mask: u32,
            value_list: *const xcb_sync_create_alarm_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_create_alarm_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            id: xcb_sync_alarm_t,
            value_mask: u32,
            value_list: *const xcb_sync_create_alarm_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_change_alarm: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            id: xcb_sync_alarm_t,
            value_mask: u32,
            value_list: *const xcb_sync_change_alarm_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_change_alarm_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            id: xcb_sync_alarm_t,
            value_mask: u32,
            value_list: *const xcb_sync_change_alarm_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_destroy_alarm: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, alarm: xcb_sync_alarm_t) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_destroy_alarm_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, alarm: xcb_sync_alarm_t) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_query_alarm_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_sync_query_alarm_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_sync_query_alarm_reply_t,
    >,
    pub(crate) xcb_sync_query_alarm: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            alarm: xcb_sync_alarm_t,
        ) -> xcb_sync_query_alarm_cookie_t,
    >,
    pub(crate) xcb_sync_query_alarm_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            alarm: xcb_sync_alarm_t,
        ) -> xcb_sync_query_alarm_cookie_t,
    >,
    pub(crate) xcb_sync_set_priority: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, id: u32, priority: i32) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_set_priority_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, id: u32, priority: i32) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_get_priority_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_sync_get_priority_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_sync_get_priority_reply_t,
    >,
    pub(crate) xcb_sync_get_priority:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, id: u32) -> xcb_sync_get_priority_cookie_t>,
    pub(crate) xcb_sync_get_priority_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, id: u32) -> xcb_sync_get_priority_cookie_t>,
    pub(crate) xcb_sync_create_fence: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            fence: xcb_sync_fence_t,
            initially_triggered: u8,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_create_fence_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            fence: xcb_sync_fence_t,
            initially_triggered: u8,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_trigger_fence: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, fence: xcb_sync_fence_t) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_trigger_fence_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, fence: xcb_sync_fence_t) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_reset_fence: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, fence: xcb_sync_fence_t) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_reset_fence_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, fence: xcb_sync_fence_t) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_destroy_fence: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, fence: xcb_sync_fence_t) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_destroy_fence_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, fence: xcb_sync_fence_t) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_query_fence_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_sync_query_fence_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_sync_query_fence_reply_t,
    >,
    pub(crate) xcb_sync_query_fence: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            fence: xcb_sync_fence_t,
        ) -> xcb_sync_query_fence_cookie_t,
    >,
    pub(crate) xcb_sync_query_fence_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            fence: xcb_sync_fence_t,
        ) -> xcb_sync_query_fence_cookie_t,
    >,
    pub(crate) xcb_sync_await_fence: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            fence_list_len: u32,
            fence_list: *const xcb_sync_fence_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_sync_await_fence_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            fence_list_len: u32,
            fence_list: *const xcb_sync_fence_t,
        ) -> xcb_void_cookie_t,
    >,
}
