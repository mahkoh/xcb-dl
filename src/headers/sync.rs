// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The name of the `Sync` extension.
pub const XCB_SYNC_NAME: &[u8] = b"SYNC";

/// The name of the `Sync` extension.
pub const XCB_SYNC_NAME_STR: &str = "SYNC";

/// The `Sync::ALARM` type.
pub type xcb_sync_alarm_t = u32;

/// An iterator over `Sync::ALARM` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_alarm_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_sync_alarm_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_sync_alarm_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Sync::ALARMSTATE` enum.
///
/// This enum has the following variants:
///
/// - [`Sync::ALARMSTATE::Active`](XCB_SYNC_ALARMSTATE_ACTIVE)
/// - [`Sync::ALARMSTATE::Inactive`](XCB_SYNC_ALARMSTATE_INACTIVE)
/// - [`Sync::ALARMSTATE::Destroyed`](XCB_SYNC_ALARMSTATE_DESTROYED)
pub type xcb_sync_alarmstate_t = u32;
/// The `Sync::ALARMSTATE::Active` enum variant.
///
/// This is a variant of [`xcb_sync_alarmstate_t`].
pub const XCB_SYNC_ALARMSTATE_ACTIVE: xcb_sync_alarmstate_t = 0;
/// The `Sync::ALARMSTATE::Inactive` enum variant.
///
/// This is a variant of [`xcb_sync_alarmstate_t`].
pub const XCB_SYNC_ALARMSTATE_INACTIVE: xcb_sync_alarmstate_t = 1;
/// The `Sync::ALARMSTATE::Destroyed` enum variant.
///
/// This is a variant of [`xcb_sync_alarmstate_t`].
pub const XCB_SYNC_ALARMSTATE_DESTROYED: xcb_sync_alarmstate_t = 2;

/// The `Sync::COUNTER` type.
pub type xcb_sync_counter_t = u32;

/// An iterator over `Sync::COUNTER` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_counter_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_sync_counter_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_sync_counter_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Sync::FENCE` type.
pub type xcb_sync_fence_t = u32;

/// An iterator over `Sync::FENCE` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_fence_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_sync_fence_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_sync_fence_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Sync::TESTTYPE` enum.
///
/// This enum has the following variants:
///
/// - [`Sync::TESTTYPE::PositiveTransition`](XCB_SYNC_TESTTYPE_POSITIVE_TRANSITION)
/// - [`Sync::TESTTYPE::NegativeTransition`](XCB_SYNC_TESTTYPE_NEGATIVE_TRANSITION)
/// - [`Sync::TESTTYPE::PositiveComparison`](XCB_SYNC_TESTTYPE_POSITIVE_COMPARISON)
/// - [`Sync::TESTTYPE::NegativeComparison`](XCB_SYNC_TESTTYPE_NEGATIVE_COMPARISON)
pub type xcb_sync_testtype_t = u32;
/// The `Sync::TESTTYPE::PositiveTransition` enum variant.
///
/// This is a variant of [`xcb_sync_testtype_t`].
pub const XCB_SYNC_TESTTYPE_POSITIVE_TRANSITION: xcb_sync_testtype_t = 0;
/// The `Sync::TESTTYPE::NegativeTransition` enum variant.
///
/// This is a variant of [`xcb_sync_testtype_t`].
pub const XCB_SYNC_TESTTYPE_NEGATIVE_TRANSITION: xcb_sync_testtype_t = 1;
/// The `Sync::TESTTYPE::PositiveComparison` enum variant.
///
/// This is a variant of [`xcb_sync_testtype_t`].
pub const XCB_SYNC_TESTTYPE_POSITIVE_COMPARISON: xcb_sync_testtype_t = 2;
/// The `Sync::TESTTYPE::NegativeComparison` enum variant.
///
/// This is a variant of [`xcb_sync_testtype_t`].
pub const XCB_SYNC_TESTTYPE_NEGATIVE_COMPARISON: xcb_sync_testtype_t = 3;

/// The `Sync::VALUETYPE` enum.
///
/// This enum has the following variants:
///
/// - [`Sync::VALUETYPE::Absolute`](XCB_SYNC_VALUETYPE_ABSOLUTE)
/// - [`Sync::VALUETYPE::Relative`](XCB_SYNC_VALUETYPE_RELATIVE)
pub type xcb_sync_valuetype_t = u32;
/// The `Sync::VALUETYPE::Absolute` enum variant.
///
/// This is a variant of [`xcb_sync_valuetype_t`].
pub const XCB_SYNC_VALUETYPE_ABSOLUTE: xcb_sync_valuetype_t = 0;
/// The `Sync::VALUETYPE::Relative` enum variant.
///
/// This is a variant of [`xcb_sync_valuetype_t`].
pub const XCB_SYNC_VALUETYPE_RELATIVE: xcb_sync_valuetype_t = 1;

/// The `Sync::CA` enum.
///
/// This enum has the following variants:
///
/// - [`Sync::CA::Counter`](XCB_SYNC_CA_COUNTER)
/// - [`Sync::CA::ValueType`](XCB_SYNC_CA_VALUE_TYPE)
/// - [`Sync::CA::Value`](XCB_SYNC_CA_VALUE)
/// - [`Sync::CA::TestType`](XCB_SYNC_CA_TEST_TYPE)
/// - [`Sync::CA::Delta`](XCB_SYNC_CA_DELTA)
/// - [`Sync::CA::Events`](XCB_SYNC_CA_EVENTS)
pub type xcb_sync_ca_t = u32;
/// The `Sync::CA::Counter` enum variant.
///
/// This is a variant of [`xcb_sync_ca_t`].
pub const XCB_SYNC_CA_COUNTER: xcb_sync_ca_t = 1;
/// The `Sync::CA::ValueType` enum variant.
///
/// This is a variant of [`xcb_sync_ca_t`].
pub const XCB_SYNC_CA_VALUE_TYPE: xcb_sync_ca_t = 2;
/// The `Sync::CA::Value` enum variant.
///
/// This is a variant of [`xcb_sync_ca_t`].
pub const XCB_SYNC_CA_VALUE: xcb_sync_ca_t = 4;
/// The `Sync::CA::TestType` enum variant.
///
/// This is a variant of [`xcb_sync_ca_t`].
pub const XCB_SYNC_CA_TEST_TYPE: xcb_sync_ca_t = 8;
/// The `Sync::CA::Delta` enum variant.
///
/// This is a variant of [`xcb_sync_ca_t`].
pub const XCB_SYNC_CA_DELTA: xcb_sync_ca_t = 16;
/// The `Sync::CA::Events` enum variant.
///
/// This is a variant of [`xcb_sync_ca_t`].
pub const XCB_SYNC_CA_EVENTS: xcb_sync_ca_t = 32;

/// The `Sync::INT64` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_int64_t {
    pub hi: i32,
    pub lo: u32,
}

impl Default for xcb_sync_int64_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Sync::INT64` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_int64_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_sync_int64_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_sync_int64_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Sync::SYSTEMCOUNTER` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `name`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_systemcounter_t {
    pub counter: xcb_sync_counter_t,
    pub resolution: xcb_sync_int64_t,
    pub name_len: u16,
}

impl Default for xcb_sync_systemcounter_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Sync::SYSTEMCOUNTER` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_systemcounter_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_sync_systemcounter_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_sync_systemcounter_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Sync::TRIGGER` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_trigger_t {
    pub counter: xcb_sync_counter_t,
    pub wait_type: u32,
    pub wait_value: xcb_sync_int64_t,
    pub test_type: u32,
}

impl Default for xcb_sync_trigger_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Sync::TRIGGER` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_trigger_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_sync_trigger_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_sync_trigger_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Sync::WAITCONDITION` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_waitcondition_t {
    pub trigger: xcb_sync_trigger_t,
    pub event_threshold: xcb_sync_int64_t,
}

impl Default for xcb_sync_waitcondition_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Sync::WAITCONDITION` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_waitcondition_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_sync_waitcondition_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_sync_waitcondition_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Sync::Counter` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_sync_counter_error_t`].
pub const XCB_SYNC_COUNTER: u8 = 0i32 as u8;

/// The `Sync::Counter` error.
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

impl Default for xcb_sync_counter_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Sync::Alarm` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_sync_alarm_error_t`].
pub const XCB_SYNC_ALARM: u8 = 1i32 as u8;

/// The `Sync::Alarm` error.
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

impl Default for xcb_sync_alarm_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Sync::Initialize` request.
///
/// Pass this cookie to [`xcb_sync_initialize_reply`] to retrieve the reply.
///
/// [`xcb_sync_initialize_reply`]: XcbSync::xcb_sync_initialize_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_initialize_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_sync_initialize_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Sync::Initialize` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbSync::xcb_sync_id()`], then the type of the request is
/// [`xcb_sync_initialize_request_t`].
pub const XCB_SYNC_INITIALIZE: u8 = 0i32 as u8;

/// The `Sync::Initialize` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_initialize_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub desired_major_version: u8,
    pub desired_minor_version: u8,
}

impl Default for xcb_sync_initialize_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Sync::Initialize` reply.
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

impl Default for xcb_sync_initialize_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Sync::ListSystemCounters` request.
///
/// Pass this cookie to [`xcb_sync_list_system_counters_reply`] to retrieve the reply.
///
/// [`xcb_sync_list_system_counters_reply`]: XcbSync::xcb_sync_list_system_counters_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_list_system_counters_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_sync_list_system_counters_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Sync::ListSystemCounters` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbSync::xcb_sync_id()`], then the type of the request is
/// [`xcb_sync_list_system_counters_request_t`].
pub const XCB_SYNC_LIST_SYSTEM_COUNTERS: u8 = 1i32 as u8;

/// The `Sync::ListSystemCounters` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_list_system_counters_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_sync_list_system_counters_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Sync::ListSystemCounters` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `counters`
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

impl Default for xcb_sync_list_system_counters_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Sync::CreateCounter` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbSync::xcb_sync_id()`], then the type of the request is
/// [`xcb_sync_create_counter_request_t`].
pub const XCB_SYNC_CREATE_COUNTER: u8 = 2i32 as u8;

/// The `Sync::CreateCounter` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_create_counter_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub id: xcb_sync_counter_t,
    pub initial_value: xcb_sync_int64_t,
}

impl Default for xcb_sync_create_counter_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Sync::DestroyCounter` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbSync::xcb_sync_id()`], then the type of the request is
/// [`xcb_sync_destroy_counter_request_t`].
pub const XCB_SYNC_DESTROY_COUNTER: u8 = 6i32 as u8;

/// The `Sync::DestroyCounter` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_destroy_counter_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub counter: xcb_sync_counter_t,
}

impl Default for xcb_sync_destroy_counter_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Sync::QueryCounter` request.
///
/// Pass this cookie to [`xcb_sync_query_counter_reply`] to retrieve the reply.
///
/// [`xcb_sync_query_counter_reply`]: XcbSync::xcb_sync_query_counter_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_query_counter_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_sync_query_counter_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Sync::QueryCounter` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbSync::xcb_sync_id()`], then the type of the request is
/// [`xcb_sync_query_counter_request_t`].
pub const XCB_SYNC_QUERY_COUNTER: u8 = 5i32 as u8;

/// The `Sync::QueryCounter` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_query_counter_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub counter: xcb_sync_counter_t,
}

impl Default for xcb_sync_query_counter_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Sync::QueryCounter` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_query_counter_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub counter_value: xcb_sync_int64_t,
}

impl Default for xcb_sync_query_counter_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Sync::Await` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbSync::xcb_sync_id()`], then the type of the request is
/// [`xcb_sync_await_request_t`].
pub const XCB_SYNC_AWAIT: u8 = 7i32 as u8;

/// The `Sync::Await` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `wait_list`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_await_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_sync_await_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Sync::ChangeCounter` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbSync::xcb_sync_id()`], then the type of the request is
/// [`xcb_sync_change_counter_request_t`].
pub const XCB_SYNC_CHANGE_COUNTER: u8 = 4i32 as u8;

/// The `Sync::ChangeCounter` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_change_counter_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub counter: xcb_sync_counter_t,
    pub amount: xcb_sync_int64_t,
}

impl Default for xcb_sync_change_counter_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Sync::SetCounter` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbSync::xcb_sync_id()`], then the type of the request is
/// [`xcb_sync_set_counter_request_t`].
pub const XCB_SYNC_SET_COUNTER: u8 = 3i32 as u8;

/// The `Sync::SetCounter` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_set_counter_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub counter: xcb_sync_counter_t,
    pub value: xcb_sync_int64_t,
}

impl Default for xcb_sync_set_counter_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Sync::value_list` switch.
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

impl Default for xcb_sync_create_alarm_value_list_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Sync::CreateAlarm` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbSync::xcb_sync_id()`], then the type of the request is
/// [`xcb_sync_create_alarm_request_t`].
pub const XCB_SYNC_CREATE_ALARM: u8 = 8i32 as u8;

/// The `Sync::CreateAlarm` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `value_list`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_create_alarm_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub id: xcb_sync_alarm_t,
    pub value_mask: u32,
}

impl Default for xcb_sync_create_alarm_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Sync::value_list` switch.
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

impl Default for xcb_sync_change_alarm_value_list_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Sync::ChangeAlarm` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbSync::xcb_sync_id()`], then the type of the request is
/// [`xcb_sync_change_alarm_request_t`].
pub const XCB_SYNC_CHANGE_ALARM: u8 = 9i32 as u8;

/// The `Sync::ChangeAlarm` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `value_list`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_change_alarm_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub id: xcb_sync_alarm_t,
    pub value_mask: u32,
}

impl Default for xcb_sync_change_alarm_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Sync::DestroyAlarm` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbSync::xcb_sync_id()`], then the type of the request is
/// [`xcb_sync_destroy_alarm_request_t`].
pub const XCB_SYNC_DESTROY_ALARM: u8 = 11i32 as u8;

/// The `Sync::DestroyAlarm` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_destroy_alarm_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub alarm: xcb_sync_alarm_t,
}

impl Default for xcb_sync_destroy_alarm_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Sync::QueryAlarm` request.
///
/// Pass this cookie to [`xcb_sync_query_alarm_reply`] to retrieve the reply.
///
/// [`xcb_sync_query_alarm_reply`]: XcbSync::xcb_sync_query_alarm_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_query_alarm_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_sync_query_alarm_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Sync::QueryAlarm` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbSync::xcb_sync_id()`], then the type of the request is
/// [`xcb_sync_query_alarm_request_t`].
pub const XCB_SYNC_QUERY_ALARM: u8 = 10i32 as u8;

/// The `Sync::QueryAlarm` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_query_alarm_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub alarm: xcb_sync_alarm_t,
}

impl Default for xcb_sync_query_alarm_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Sync::QueryAlarm` reply.
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

impl Default for xcb_sync_query_alarm_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Sync::SetPriority` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbSync::xcb_sync_id()`], then the type of the request is
/// [`xcb_sync_set_priority_request_t`].
pub const XCB_SYNC_SET_PRIORITY: u8 = 12i32 as u8;

/// The `Sync::SetPriority` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_set_priority_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub id: u32,
    pub priority: i32,
}

impl Default for xcb_sync_set_priority_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Sync::GetPriority` request.
///
/// Pass this cookie to [`xcb_sync_get_priority_reply`] to retrieve the reply.
///
/// [`xcb_sync_get_priority_reply`]: XcbSync::xcb_sync_get_priority_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_get_priority_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_sync_get_priority_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Sync::GetPriority` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbSync::xcb_sync_id()`], then the type of the request is
/// [`xcb_sync_get_priority_request_t`].
pub const XCB_SYNC_GET_PRIORITY: u8 = 13i32 as u8;

/// The `Sync::GetPriority` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_get_priority_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub id: u32,
}

impl Default for xcb_sync_get_priority_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Sync::GetPriority` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_get_priority_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub priority: i32,
}

impl Default for xcb_sync_get_priority_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Sync::CreateFence` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbSync::xcb_sync_id()`], then the type of the request is
/// [`xcb_sync_create_fence_request_t`].
pub const XCB_SYNC_CREATE_FENCE: u8 = 14i32 as u8;

/// The `Sync::CreateFence` request.
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

impl Default for xcb_sync_create_fence_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Sync::TriggerFence` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbSync::xcb_sync_id()`], then the type of the request is
/// [`xcb_sync_trigger_fence_request_t`].
pub const XCB_SYNC_TRIGGER_FENCE: u8 = 15i32 as u8;

/// The `Sync::TriggerFence` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_trigger_fence_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub fence: xcb_sync_fence_t,
}

impl Default for xcb_sync_trigger_fence_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Sync::ResetFence` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbSync::xcb_sync_id()`], then the type of the request is
/// [`xcb_sync_reset_fence_request_t`].
pub const XCB_SYNC_RESET_FENCE: u8 = 16i32 as u8;

/// The `Sync::ResetFence` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_reset_fence_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub fence: xcb_sync_fence_t,
}

impl Default for xcb_sync_reset_fence_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Sync::DestroyFence` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbSync::xcb_sync_id()`], then the type of the request is
/// [`xcb_sync_destroy_fence_request_t`].
pub const XCB_SYNC_DESTROY_FENCE: u8 = 17i32 as u8;

/// The `Sync::DestroyFence` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_destroy_fence_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub fence: xcb_sync_fence_t,
}

impl Default for xcb_sync_destroy_fence_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Sync::QueryFence` request.
///
/// Pass this cookie to [`xcb_sync_query_fence_reply`] to retrieve the reply.
///
/// [`xcb_sync_query_fence_reply`]: XcbSync::xcb_sync_query_fence_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_query_fence_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_sync_query_fence_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Sync::QueryFence` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbSync::xcb_sync_id()`], then the type of the request is
/// [`xcb_sync_query_fence_request_t`].
pub const XCB_SYNC_QUERY_FENCE: u8 = 18i32 as u8;

/// The `Sync::QueryFence` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_query_fence_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub fence: xcb_sync_fence_t,
}

impl Default for xcb_sync_query_fence_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Sync::QueryFence` reply.
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

impl Default for xcb_sync_query_fence_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Sync::AwaitFence` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbSync::xcb_sync_id()`], then the type of the request is
/// [`xcb_sync_await_fence_request_t`].
pub const XCB_SYNC_AWAIT_FENCE: u8 = 19i32 as u8;

/// The `Sync::AwaitFence` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `fence_list`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_sync_await_fence_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_sync_await_fence_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Sync::CounterNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_sync_counter_notify_event_t`].
pub const XCB_SYNC_COUNTER_NOTIFY: u8 = 0i32 as u8;

/// The `Sync::CounterNotify` event.
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

impl Default for xcb_sync_counter_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Sync::AlarmNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_sync_alarm_notify_event_t`].
pub const XCB_SYNC_ALARM_NOTIFY: u8 = 1i32 as u8;

/// The `Sync::AlarmNotify` event.
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

impl Default for xcb_sync_alarm_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_sync")]
pub(crate) struct XcbSyncSync {
    xcb_sync_id: LazySymbol<*mut xcb_extension_t>,
    xcb_sync_alarm_next: LazySymbol<unsafe fn(i: *mut xcb_sync_alarm_iterator_t)>,
    xcb_sync_alarm_end:
        LazySymbol<unsafe fn(i: xcb_sync_alarm_iterator_t) -> xcb_generic_iterator_t>,
    xcb_sync_counter_next: LazySymbol<unsafe fn(i: *mut xcb_sync_counter_iterator_t)>,
    xcb_sync_counter_end:
        LazySymbol<unsafe fn(i: xcb_sync_counter_iterator_t) -> xcb_generic_iterator_t>,
    xcb_sync_fence_next: LazySymbol<unsafe fn(i: *mut xcb_sync_fence_iterator_t)>,
    xcb_sync_fence_end:
        LazySymbol<unsafe fn(i: xcb_sync_fence_iterator_t) -> xcb_generic_iterator_t>,
    xcb_sync_int64_next: LazySymbol<unsafe fn(i: *mut xcb_sync_int64_iterator_t)>,
    xcb_sync_int64_end:
        LazySymbol<unsafe fn(i: xcb_sync_int64_iterator_t) -> xcb_generic_iterator_t>,
    xcb_sync_systemcounter_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_sync_systemcounter_name:
        LazySymbol<unsafe fn(r: *const xcb_sync_systemcounter_t) -> *mut c_char>,
    xcb_sync_systemcounter_name_length:
        LazySymbol<unsafe fn(r: *const xcb_sync_systemcounter_t) -> c_int>,
    xcb_sync_systemcounter_name_end:
        LazySymbol<unsafe fn(r: *const xcb_sync_systemcounter_t) -> xcb_generic_iterator_t>,
    xcb_sync_systemcounter_next: LazySymbol<unsafe fn(i: *mut xcb_sync_systemcounter_iterator_t)>,
    xcb_sync_systemcounter_end:
        LazySymbol<unsafe fn(i: xcb_sync_systemcounter_iterator_t) -> xcb_generic_iterator_t>,
    xcb_sync_trigger_next: LazySymbol<unsafe fn(i: *mut xcb_sync_trigger_iterator_t)>,
    xcb_sync_trigger_end:
        LazySymbol<unsafe fn(i: xcb_sync_trigger_iterator_t) -> xcb_generic_iterator_t>,
    xcb_sync_waitcondition_next: LazySymbol<unsafe fn(i: *mut xcb_sync_waitcondition_iterator_t)>,
    xcb_sync_waitcondition_end:
        LazySymbol<unsafe fn(i: xcb_sync_waitcondition_iterator_t) -> xcb_generic_iterator_t>,
    xcb_sync_initialize: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            desired_major_version: u8,
            desired_minor_version: u8,
        ) -> xcb_sync_initialize_cookie_t,
    >,
    xcb_sync_initialize_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            desired_major_version: u8,
            desired_minor_version: u8,
        ) -> xcb_sync_initialize_cookie_t,
    >,
    xcb_sync_initialize_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_sync_initialize_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_sync_initialize_reply_t,
    >,
    xcb_sync_list_system_counters_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_sync_list_system_counters:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_sync_list_system_counters_cookie_t>,
    xcb_sync_list_system_counters_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_sync_list_system_counters_cookie_t>,
    xcb_sync_list_system_counters_counters_length:
        LazySymbol<unsafe fn(r: *const xcb_sync_list_system_counters_reply_t) -> c_int>,
    xcb_sync_list_system_counters_counters_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_sync_list_system_counters_reply_t,
        ) -> xcb_sync_systemcounter_iterator_t,
    >,
    xcb_sync_list_system_counters_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_sync_list_system_counters_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_sync_list_system_counters_reply_t,
    >,
    xcb_sync_create_counter_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            id: xcb_sync_counter_t,
            initial_value: xcb_sync_int64_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_sync_create_counter: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            id: xcb_sync_counter_t,
            initial_value: xcb_sync_int64_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_sync_destroy_counter_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, counter: xcb_sync_counter_t) -> xcb_void_cookie_t,
    >,
    xcb_sync_destroy_counter: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, counter: xcb_sync_counter_t) -> xcb_void_cookie_t,
    >,
    xcb_sync_query_counter: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            counter: xcb_sync_counter_t,
        ) -> xcb_sync_query_counter_cookie_t,
    >,
    xcb_sync_query_counter_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            counter: xcb_sync_counter_t,
        ) -> xcb_sync_query_counter_cookie_t,
    >,
    xcb_sync_query_counter_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_sync_query_counter_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_sync_query_counter_reply_t,
    >,
    xcb_sync_await_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, wait_list_len: u32) -> c_int>,
    xcb_sync_await_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            wait_list_len: u32,
            wait_list: *const xcb_sync_waitcondition_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_sync_await: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            wait_list_len: u32,
            wait_list: *const xcb_sync_waitcondition_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_sync_await_wait_list:
        LazySymbol<unsafe fn(r: *const xcb_sync_await_request_t) -> *mut xcb_sync_waitcondition_t>,
    xcb_sync_await_wait_list_length:
        LazySymbol<unsafe fn(r: *const xcb_sync_await_request_t) -> c_int>,
    xcb_sync_await_wait_list_iterator: LazySymbol<
        unsafe fn(r: *const xcb_sync_await_request_t) -> xcb_sync_waitcondition_iterator_t,
    >,
    xcb_sync_change_counter_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            counter: xcb_sync_counter_t,
            amount: xcb_sync_int64_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_sync_change_counter: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            counter: xcb_sync_counter_t,
            amount: xcb_sync_int64_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_sync_set_counter_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            counter: xcb_sync_counter_t,
            value: xcb_sync_int64_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_sync_set_counter: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            counter: xcb_sync_counter_t,
            value: xcb_sync_int64_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_sync_create_alarm_value_list_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            value_mask: u32,
            _aux: *const xcb_sync_create_alarm_value_list_t,
        ) -> c_int,
    >,
    xcb_sync_create_alarm_value_list_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            value_mask: u32,
            _aux: *mut xcb_sync_create_alarm_value_list_t,
        ) -> c_int,
    >,
    xcb_sync_create_alarm_value_list_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, value_mask: u32) -> c_int>,
    xcb_sync_create_alarm_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_sync_create_alarm_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            id: xcb_sync_alarm_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_sync_create_alarm: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            id: xcb_sync_alarm_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_sync_create_alarm_aux_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            id: xcb_sync_alarm_t,
            value_mask: u32,
            value_list: *const xcb_sync_create_alarm_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_sync_create_alarm_aux: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            id: xcb_sync_alarm_t,
            value_mask: u32,
            value_list: *const xcb_sync_create_alarm_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_sync_create_alarm_value_list:
        LazySymbol<unsafe fn(r: *const xcb_sync_create_alarm_request_t) -> *mut c_void>,
    xcb_sync_change_alarm_value_list_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            value_mask: u32,
            _aux: *const xcb_sync_change_alarm_value_list_t,
        ) -> c_int,
    >,
    xcb_sync_change_alarm_value_list_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            value_mask: u32,
            _aux: *mut xcb_sync_change_alarm_value_list_t,
        ) -> c_int,
    >,
    xcb_sync_change_alarm_value_list_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, value_mask: u32) -> c_int>,
    xcb_sync_change_alarm_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_sync_change_alarm_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            id: xcb_sync_alarm_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_sync_change_alarm: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            id: xcb_sync_alarm_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_sync_change_alarm_aux_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            id: xcb_sync_alarm_t,
            value_mask: u32,
            value_list: *const xcb_sync_change_alarm_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_sync_change_alarm_aux: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            id: xcb_sync_alarm_t,
            value_mask: u32,
            value_list: *const xcb_sync_change_alarm_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_sync_change_alarm_value_list:
        LazySymbol<unsafe fn(r: *const xcb_sync_change_alarm_request_t) -> *mut c_void>,
    xcb_sync_destroy_alarm_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, alarm: xcb_sync_alarm_t) -> xcb_void_cookie_t,
    >,
    xcb_sync_destroy_alarm: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, alarm: xcb_sync_alarm_t) -> xcb_void_cookie_t,
    >,
    xcb_sync_query_alarm: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            alarm: xcb_sync_alarm_t,
        ) -> xcb_sync_query_alarm_cookie_t,
    >,
    xcb_sync_query_alarm_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            alarm: xcb_sync_alarm_t,
        ) -> xcb_sync_query_alarm_cookie_t,
    >,
    xcb_sync_query_alarm_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_sync_query_alarm_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_sync_query_alarm_reply_t,
    >,
    xcb_sync_set_priority_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, id: u32, priority: i32) -> xcb_void_cookie_t,
    >,
    xcb_sync_set_priority: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, id: u32, priority: i32) -> xcb_void_cookie_t,
    >,
    xcb_sync_get_priority:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, id: u32) -> xcb_sync_get_priority_cookie_t>,
    xcb_sync_get_priority_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, id: u32) -> xcb_sync_get_priority_cookie_t>,
    xcb_sync_get_priority_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_sync_get_priority_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_sync_get_priority_reply_t,
    >,
    xcb_sync_create_fence_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            fence: xcb_sync_fence_t,
            initially_triggered: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_sync_create_fence: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            fence: xcb_sync_fence_t,
            initially_triggered: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_sync_trigger_fence_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, fence: xcb_sync_fence_t) -> xcb_void_cookie_t,
    >,
    xcb_sync_trigger_fence: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, fence: xcb_sync_fence_t) -> xcb_void_cookie_t,
    >,
    xcb_sync_reset_fence_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, fence: xcb_sync_fence_t) -> xcb_void_cookie_t,
    >,
    xcb_sync_reset_fence: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, fence: xcb_sync_fence_t) -> xcb_void_cookie_t,
    >,
    xcb_sync_destroy_fence_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, fence: xcb_sync_fence_t) -> xcb_void_cookie_t,
    >,
    xcb_sync_destroy_fence: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, fence: xcb_sync_fence_t) -> xcb_void_cookie_t,
    >,
    xcb_sync_query_fence: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            fence: xcb_sync_fence_t,
        ) -> xcb_sync_query_fence_cookie_t,
    >,
    xcb_sync_query_fence_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            fence: xcb_sync_fence_t,
        ) -> xcb_sync_query_fence_cookie_t,
    >,
    xcb_sync_query_fence_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_sync_query_fence_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_sync_query_fence_reply_t,
    >,
    xcb_sync_await_fence_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, fence_list_len: u32) -> c_int>,
    xcb_sync_await_fence_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            fence_list_len: u32,
            fence_list: *const xcb_sync_fence_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_sync_await_fence: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            fence_list_len: u32,
            fence_list: *const xcb_sync_fence_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_sync_await_fence_fence_list:
        LazySymbol<unsafe fn(r: *const xcb_sync_await_fence_request_t) -> *mut xcb_sync_fence_t>,
    xcb_sync_await_fence_fence_list_length:
        LazySymbol<unsafe fn(r: *const xcb_sync_await_fence_request_t) -> c_int>,
    xcb_sync_await_fence_fence_list_end:
        LazySymbol<unsafe fn(r: *const xcb_sync_await_fence_request_t) -> xcb_generic_iterator_t>,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self.sync.$f.get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .sync
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_sync")]
impl XcbSync {
    /// The libxcb identifier of the `Sync` extension.
    #[inline]
    pub fn xcb_sync_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_sync_id) }
    }

    /// Returns `true` iff the symbol `xcb_sync_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_id(&self) -> bool {
        has_sym!(self, xcb_sync_id)
    }

    /// Advances a `xcb_sync_alarm_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_sync_alarm_next(&self, i: *mut xcb_sync_alarm_iterator_t) {
        sym!(self, xcb_sync_alarm_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_sync_alarm_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_alarm_next(&self) -> bool {
        has_sym!(self, xcb_sync_alarm_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_sync_alarm_iterator_t`.
    #[inline]
    pub unsafe fn xcb_sync_alarm_end(
        &self,
        i: xcb_sync_alarm_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_sync_alarm_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_sync_alarm_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_alarm_end(&self) -> bool {
        has_sym!(self, xcb_sync_alarm_end)
    }

    /// Advances a `xcb_sync_counter_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_sync_counter_next(&self, i: *mut xcb_sync_counter_iterator_t) {
        sym!(self, xcb_sync_counter_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_sync_counter_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_counter_next(&self) -> bool {
        has_sym!(self, xcb_sync_counter_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_sync_counter_iterator_t`.
    #[inline]
    pub unsafe fn xcb_sync_counter_end(
        &self,
        i: xcb_sync_counter_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_sync_counter_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_sync_counter_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_counter_end(&self) -> bool {
        has_sym!(self, xcb_sync_counter_end)
    }

    /// Advances a `xcb_sync_fence_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_sync_fence_next(&self, i: *mut xcb_sync_fence_iterator_t) {
        sym!(self, xcb_sync_fence_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_sync_fence_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_fence_next(&self) -> bool {
        has_sym!(self, xcb_sync_fence_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_sync_fence_iterator_t`.
    #[inline]
    pub unsafe fn xcb_sync_fence_end(
        &self,
        i: xcb_sync_fence_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_sync_fence_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_sync_fence_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_fence_end(&self) -> bool {
        has_sym!(self, xcb_sync_fence_end)
    }

    /// Advances a `xcb_sync_int64_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_sync_int64_next(&self, i: *mut xcb_sync_int64_iterator_t) {
        sym!(self, xcb_sync_int64_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_sync_int64_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_int64_next(&self) -> bool {
        has_sym!(self, xcb_sync_int64_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_sync_int64_iterator_t`.
    #[inline]
    pub unsafe fn xcb_sync_int64_end(
        &self,
        i: xcb_sync_int64_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_sync_int64_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_sync_int64_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_int64_end(&self) -> bool {
        has_sym!(self, xcb_sync_int64_end)
    }

    /// Computes the size of a `xcb_sync_systemcounter_t` object.
    #[inline]
    pub unsafe fn xcb_sync_systemcounter_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_sync_systemcounter_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_sync_systemcounter_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_systemcounter_sizeof(&self) -> bool {
        has_sym!(self, xcb_sync_systemcounter_sizeof)
    }

    /// Returns a pointer to the `name` field of a `xcb_sync_systemcounter_t` struct.
    #[inline]
    pub unsafe fn xcb_sync_systemcounter_name(
        &self,
        r: *const xcb_sync_systemcounter_t,
    ) -> *mut c_char {
        sym!(self, xcb_sync_systemcounter_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_sync_systemcounter_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_systemcounter_name(&self) -> bool {
        has_sym!(self, xcb_sync_systemcounter_name)
    }

    /// Returns the number of elements of the `name` field of a `xcb_sync_systemcounter_t` struct.
    #[inline]
    pub unsafe fn xcb_sync_systemcounter_name_length(
        &self,
        r: *const xcb_sync_systemcounter_t,
    ) -> c_int {
        sym!(self, xcb_sync_systemcounter_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_sync_systemcounter_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_systemcounter_name_length(&self) -> bool {
        has_sym!(self, xcb_sync_systemcounter_name_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `name` field of a `xcb_sync_systemcounter_t` struct.
    #[inline]
    pub unsafe fn xcb_sync_systemcounter_name_end(
        &self,
        r: *const xcb_sync_systemcounter_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_sync_systemcounter_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_sync_systemcounter_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_systemcounter_name_end(&self) -> bool {
        has_sym!(self, xcb_sync_systemcounter_name_end)
    }

    /// Advances a `xcb_sync_systemcounter_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_sync_systemcounter_next(&self, i: *mut xcb_sync_systemcounter_iterator_t) {
        sym!(self, xcb_sync_systemcounter_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_sync_systemcounter_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_systemcounter_next(&self) -> bool {
        has_sym!(self, xcb_sync_systemcounter_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_sync_systemcounter_iterator_t`.
    #[inline]
    pub unsafe fn xcb_sync_systemcounter_end(
        &self,
        i: xcb_sync_systemcounter_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_sync_systemcounter_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_sync_systemcounter_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_systemcounter_end(&self) -> bool {
        has_sym!(self, xcb_sync_systemcounter_end)
    }

    /// Advances a `xcb_sync_trigger_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_sync_trigger_next(&self, i: *mut xcb_sync_trigger_iterator_t) {
        sym!(self, xcb_sync_trigger_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_sync_trigger_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_trigger_next(&self) -> bool {
        has_sym!(self, xcb_sync_trigger_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_sync_trigger_iterator_t`.
    #[inline]
    pub unsafe fn xcb_sync_trigger_end(
        &self,
        i: xcb_sync_trigger_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_sync_trigger_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_sync_trigger_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_trigger_end(&self) -> bool {
        has_sym!(self, xcb_sync_trigger_end)
    }

    /// Advances a `xcb_sync_waitcondition_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_sync_waitcondition_next(&self, i: *mut xcb_sync_waitcondition_iterator_t) {
        sym!(self, xcb_sync_waitcondition_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_sync_waitcondition_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_waitcondition_next(&self) -> bool {
        has_sym!(self, xcb_sync_waitcondition_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_sync_waitcondition_iterator_t`.
    #[inline]
    pub unsafe fn xcb_sync_waitcondition_end(
        &self,
        i: xcb_sync_waitcondition_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_sync_waitcondition_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_sync_waitcondition_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_waitcondition_end(&self) -> bool {
        has_sym!(self, xcb_sync_waitcondition_end)
    }

    /// Sends a `Sync::Initialize` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_sync_initialize_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_sync_initialize_reply`]: Self::xcb_sync_initialize_reply
    #[inline]
    pub unsafe fn xcb_sync_initialize(
        &self,
        c: *mut xcb_connection_t,
        desired_major_version: u8,
        desired_minor_version: u8,
    ) -> xcb_sync_initialize_cookie_t {
        sym!(self, xcb_sync_initialize)(c, desired_major_version, desired_minor_version)
    }

    /// Returns `true` iff the symbol `xcb_sync_initialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_initialize(&self) -> bool {
        has_sym!(self, xcb_sync_initialize)
    }

    /// Sends a `Sync::Initialize` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_sync_initialize_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_sync_initialize_reply`]: Self::xcb_sync_initialize_reply
    #[inline]
    pub unsafe fn xcb_sync_initialize_unchecked(
        &self,
        c: *mut xcb_connection_t,
        desired_major_version: u8,
        desired_minor_version: u8,
    ) -> xcb_sync_initialize_cookie_t {
        sym!(self, xcb_sync_initialize_unchecked)(c, desired_major_version, desired_minor_version)
    }

    /// Returns `true` iff the symbol `xcb_sync_initialize_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_initialize_unchecked(&self) -> bool {
        has_sym!(self, xcb_sync_initialize_unchecked)
    }

    /// Waits for the reply to a `Sync::Initialize` request.
    #[inline]
    pub unsafe fn xcb_sync_initialize_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_sync_initialize_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_sync_initialize_reply_t {
        sym!(self, xcb_sync_initialize_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_sync_initialize_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_initialize_reply(&self) -> bool {
        has_sym!(self, xcb_sync_initialize_reply)
    }

    /// Computes the size of a `xcb_sync_list_system_counters_reply_t` object.
    #[inline]
    pub unsafe fn xcb_sync_list_system_counters_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_sync_list_system_counters_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_sync_list_system_counters_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_list_system_counters_sizeof(&self) -> bool {
        has_sym!(self, xcb_sync_list_system_counters_sizeof)
    }

    /// Sends a `Sync::ListSystemCounters` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_sync_list_system_counters_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_sync_list_system_counters_reply`]: Self::xcb_sync_list_system_counters_reply
    #[inline]
    pub unsafe fn xcb_sync_list_system_counters(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_sync_list_system_counters_cookie_t {
        sym!(self, xcb_sync_list_system_counters)(c)
    }

    /// Returns `true` iff the symbol `xcb_sync_list_system_counters` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_list_system_counters(&self) -> bool {
        has_sym!(self, xcb_sync_list_system_counters)
    }

    /// Sends a `Sync::ListSystemCounters` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_sync_list_system_counters_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_sync_list_system_counters_reply`]: Self::xcb_sync_list_system_counters_reply
    #[inline]
    pub unsafe fn xcb_sync_list_system_counters_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_sync_list_system_counters_cookie_t {
        sym!(self, xcb_sync_list_system_counters_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_sync_list_system_counters_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_list_system_counters_unchecked(&self) -> bool {
        has_sym!(self, xcb_sync_list_system_counters_unchecked)
    }

    /// Returns the number of elements of the `counters` field of a `xcb_sync_list_system_counters_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_sync_list_system_counters_counters_length(
        &self,
        r: *const xcb_sync_list_system_counters_reply_t,
    ) -> c_int {
        sym!(self, xcb_sync_list_system_counters_counters_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_sync_list_system_counters_counters_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_list_system_counters_counters_length(&self) -> bool {
        has_sym!(self, xcb_sync_list_system_counters_counters_length)
    }

    /// Returns an iterator over the elements of the
    /// `counters` field of a `xcb_sync_list_system_counters_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_sync_list_system_counters_counters_iterator(
        &self,
        r: *const xcb_sync_list_system_counters_reply_t,
    ) -> xcb_sync_systemcounter_iterator_t {
        sym!(self, xcb_sync_list_system_counters_counters_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_sync_list_system_counters_counters_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_list_system_counters_counters_iterator(&self) -> bool {
        has_sym!(self, xcb_sync_list_system_counters_counters_iterator)
    }

    /// Waits for the reply to a `Sync::ListSystemCounters` request.
    #[inline]
    pub unsafe fn xcb_sync_list_system_counters_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_sync_list_system_counters_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_sync_list_system_counters_reply_t {
        sym!(self, xcb_sync_list_system_counters_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_sync_list_system_counters_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_list_system_counters_reply(&self) -> bool {
        has_sym!(self, xcb_sync_list_system_counters_reply)
    }

    /// Sends a `Sync::CreateCounter` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_sync_create_counter_checked(
        &self,
        c: *mut xcb_connection_t,
        id: xcb_sync_counter_t,
        initial_value: xcb_sync_int64_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_create_counter_checked)(c, id, initial_value)
    }

    /// Returns `true` iff the symbol `xcb_sync_create_counter_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_create_counter_checked(&self) -> bool {
        has_sym!(self, xcb_sync_create_counter_checked)
    }

    /// Sends a `Sync::CreateCounter` request (unchecked).
    #[inline]
    pub unsafe fn xcb_sync_create_counter(
        &self,
        c: *mut xcb_connection_t,
        id: xcb_sync_counter_t,
        initial_value: xcb_sync_int64_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_create_counter)(c, id, initial_value)
    }

    /// Returns `true` iff the symbol `xcb_sync_create_counter` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_create_counter(&self) -> bool {
        has_sym!(self, xcb_sync_create_counter)
    }

    /// Sends a `Sync::DestroyCounter` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_sync_destroy_counter_checked(
        &self,
        c: *mut xcb_connection_t,
        counter: xcb_sync_counter_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_destroy_counter_checked)(c, counter)
    }

    /// Returns `true` iff the symbol `xcb_sync_destroy_counter_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_destroy_counter_checked(&self) -> bool {
        has_sym!(self, xcb_sync_destroy_counter_checked)
    }

    /// Sends a `Sync::DestroyCounter` request (unchecked).
    #[inline]
    pub unsafe fn xcb_sync_destroy_counter(
        &self,
        c: *mut xcb_connection_t,
        counter: xcb_sync_counter_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_destroy_counter)(c, counter)
    }

    /// Returns `true` iff the symbol `xcb_sync_destroy_counter` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_destroy_counter(&self) -> bool {
        has_sym!(self, xcb_sync_destroy_counter)
    }

    /// Sends a `Sync::QueryCounter` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_sync_query_counter_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_sync_query_counter_reply`]: Self::xcb_sync_query_counter_reply
    #[inline]
    pub unsafe fn xcb_sync_query_counter(
        &self,
        c: *mut xcb_connection_t,
        counter: xcb_sync_counter_t,
    ) -> xcb_sync_query_counter_cookie_t {
        sym!(self, xcb_sync_query_counter)(c, counter)
    }

    /// Returns `true` iff the symbol `xcb_sync_query_counter` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_query_counter(&self) -> bool {
        has_sym!(self, xcb_sync_query_counter)
    }

    /// Sends a `Sync::QueryCounter` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_sync_query_counter_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_sync_query_counter_reply`]: Self::xcb_sync_query_counter_reply
    #[inline]
    pub unsafe fn xcb_sync_query_counter_unchecked(
        &self,
        c: *mut xcb_connection_t,
        counter: xcb_sync_counter_t,
    ) -> xcb_sync_query_counter_cookie_t {
        sym!(self, xcb_sync_query_counter_unchecked)(c, counter)
    }

    /// Returns `true` iff the symbol `xcb_sync_query_counter_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_query_counter_unchecked(&self) -> bool {
        has_sym!(self, xcb_sync_query_counter_unchecked)
    }

    /// Waits for the reply to a `Sync::QueryCounter` request.
    #[inline]
    pub unsafe fn xcb_sync_query_counter_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_sync_query_counter_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_sync_query_counter_reply_t {
        sym!(self, xcb_sync_query_counter_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_sync_query_counter_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_query_counter_reply(&self) -> bool {
        has_sym!(self, xcb_sync_query_counter_reply)
    }

    /// Computes the size of a `xcb_sync_await_request_t` object.
    #[inline]
    pub unsafe fn xcb_sync_await_sizeof(
        &self,
        _buffer: *const c_void,
        wait_list_len: u32,
    ) -> c_int {
        sym!(self, xcb_sync_await_sizeof)(_buffer, wait_list_len)
    }

    /// Returns `true` iff the symbol `xcb_sync_await_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_await_sizeof(&self) -> bool {
        has_sym!(self, xcb_sync_await_sizeof)
    }

    /// Sends a `Sync::Await` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_sync_await_checked(
        &self,
        c: *mut xcb_connection_t,
        wait_list_len: u32,
        wait_list: *const xcb_sync_waitcondition_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_await_checked)(c, wait_list_len, wait_list)
    }

    /// Returns `true` iff the symbol `xcb_sync_await_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_await_checked(&self) -> bool {
        has_sym!(self, xcb_sync_await_checked)
    }

    /// Sends a `Sync::Await` request (unchecked).
    #[inline]
    pub unsafe fn xcb_sync_await(
        &self,
        c: *mut xcb_connection_t,
        wait_list_len: u32,
        wait_list: *const xcb_sync_waitcondition_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_await)(c, wait_list_len, wait_list)
    }

    /// Returns `true` iff the symbol `xcb_sync_await` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_await(&self) -> bool {
        has_sym!(self, xcb_sync_await)
    }

    /// Returns a pointer to the `wait_list` field of a `xcb_sync_await_request_t` struct.
    #[inline]
    pub unsafe fn xcb_sync_await_wait_list(
        &self,
        r: *const xcb_sync_await_request_t,
    ) -> *mut xcb_sync_waitcondition_t {
        sym!(self, xcb_sync_await_wait_list)(r)
    }

    /// Returns `true` iff the symbol `xcb_sync_await_wait_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_await_wait_list(&self) -> bool {
        has_sym!(self, xcb_sync_await_wait_list)
    }

    /// Returns the number of elements of the `wait_list` field of a `xcb_sync_await_request_t` struct.
    #[inline]
    pub unsafe fn xcb_sync_await_wait_list_length(
        &self,
        r: *const xcb_sync_await_request_t,
    ) -> c_int {
        sym!(self, xcb_sync_await_wait_list_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_sync_await_wait_list_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_await_wait_list_length(&self) -> bool {
        has_sym!(self, xcb_sync_await_wait_list_length)
    }

    /// Returns an iterator over the elements of the
    /// `wait_list` field of a `xcb_sync_await_request_t` struct.
    #[inline]
    pub unsafe fn xcb_sync_await_wait_list_iterator(
        &self,
        r: *const xcb_sync_await_request_t,
    ) -> xcb_sync_waitcondition_iterator_t {
        sym!(self, xcb_sync_await_wait_list_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_sync_await_wait_list_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_await_wait_list_iterator(&self) -> bool {
        has_sym!(self, xcb_sync_await_wait_list_iterator)
    }

    /// Sends a `Sync::ChangeCounter` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_sync_change_counter_checked(
        &self,
        c: *mut xcb_connection_t,
        counter: xcb_sync_counter_t,
        amount: xcb_sync_int64_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_change_counter_checked)(c, counter, amount)
    }

    /// Returns `true` iff the symbol `xcb_sync_change_counter_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_change_counter_checked(&self) -> bool {
        has_sym!(self, xcb_sync_change_counter_checked)
    }

    /// Sends a `Sync::ChangeCounter` request (unchecked).
    #[inline]
    pub unsafe fn xcb_sync_change_counter(
        &self,
        c: *mut xcb_connection_t,
        counter: xcb_sync_counter_t,
        amount: xcb_sync_int64_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_change_counter)(c, counter, amount)
    }

    /// Returns `true` iff the symbol `xcb_sync_change_counter` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_change_counter(&self) -> bool {
        has_sym!(self, xcb_sync_change_counter)
    }

    /// Sends a `Sync::SetCounter` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_sync_set_counter_checked(
        &self,
        c: *mut xcb_connection_t,
        counter: xcb_sync_counter_t,
        value: xcb_sync_int64_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_set_counter_checked)(c, counter, value)
    }

    /// Returns `true` iff the symbol `xcb_sync_set_counter_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_set_counter_checked(&self) -> bool {
        has_sym!(self, xcb_sync_set_counter_checked)
    }

    /// Sends a `Sync::SetCounter` request (unchecked).
    #[inline]
    pub unsafe fn xcb_sync_set_counter(
        &self,
        c: *mut xcb_connection_t,
        counter: xcb_sync_counter_t,
        value: xcb_sync_int64_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_set_counter)(c, counter, value)
    }

    /// Returns `true` iff the symbol `xcb_sync_set_counter` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_set_counter(&self) -> bool {
        has_sym!(self, xcb_sync_set_counter)
    }

    /// Serializes a `xcb_sync_create_alarm_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_sync_create_alarm_value_list_serialize(
        &self,
        _buffer: *mut *mut c_void,
        value_mask: u32,
        _aux: *const xcb_sync_create_alarm_value_list_t,
    ) -> c_int {
        sym!(self, xcb_sync_create_alarm_value_list_serialize)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_sync_create_alarm_value_list_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_create_alarm_value_list_serialize(&self) -> bool {
        has_sym!(self, xcb_sync_create_alarm_value_list_serialize)
    }

    /// Unpacks a `xcb_sync_create_alarm_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_sync_create_alarm_value_list_unpack(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
        _aux: *mut xcb_sync_create_alarm_value_list_t,
    ) -> c_int {
        sym!(self, xcb_sync_create_alarm_value_list_unpack)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_sync_create_alarm_value_list_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_create_alarm_value_list_unpack(&self) -> bool {
        has_sym!(self, xcb_sync_create_alarm_value_list_unpack)
    }

    /// Computes the size of a `xcb_sync_create_alarm_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_sync_create_alarm_value_list_sizeof(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
    ) -> c_int {
        sym!(self, xcb_sync_create_alarm_value_list_sizeof)(_buffer, value_mask)
    }

    /// Returns `true` iff the symbol `xcb_sync_create_alarm_value_list_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_create_alarm_value_list_sizeof(&self) -> bool {
        has_sym!(self, xcb_sync_create_alarm_value_list_sizeof)
    }

    /// Computes the size of a `xcb_sync_create_alarm_request_t` object.
    #[inline]
    pub unsafe fn xcb_sync_create_alarm_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_sync_create_alarm_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_sync_create_alarm_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_create_alarm_sizeof(&self) -> bool {
        has_sym!(self, xcb_sync_create_alarm_sizeof)
    }

    /// Sends a `Sync::CreateAlarm` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    ///
    /// There is an auxiliary version of this function: [`xcb_sync_create_alarm_aux_checked`].
    ///
    /// [`xcb_sync_create_alarm_aux_checked`]: Self::xcb_sync_create_alarm_aux_checked
    #[inline]
    pub unsafe fn xcb_sync_create_alarm_checked(
        &self,
        c: *mut xcb_connection_t,
        id: xcb_sync_alarm_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_create_alarm_checked)(c, id, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_sync_create_alarm_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_create_alarm_checked(&self) -> bool {
        has_sym!(self, xcb_sync_create_alarm_checked)
    }

    /// Sends a `Sync::CreateAlarm` request (unchecked).
    ///
    /// There is an auxiliary version of this function: [`xcb_sync_create_alarm_aux`].
    ///
    /// [`xcb_sync_create_alarm_aux`]: Self::xcb_sync_create_alarm_aux
    #[inline]
    pub unsafe fn xcb_sync_create_alarm(
        &self,
        c: *mut xcb_connection_t,
        id: xcb_sync_alarm_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_create_alarm)(c, id, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_sync_create_alarm` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_create_alarm(&self) -> bool {
        has_sym!(self, xcb_sync_create_alarm)
    }

    /// Sends a `Sync::CreateAlarm` request (checked) (aux).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_sync_create_alarm_aux_checked(
        &self,
        c: *mut xcb_connection_t,
        id: xcb_sync_alarm_t,
        value_mask: u32,
        value_list: *const xcb_sync_create_alarm_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_create_alarm_aux_checked)(c, id, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_sync_create_alarm_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_create_alarm_aux_checked(&self) -> bool {
        has_sym!(self, xcb_sync_create_alarm_aux_checked)
    }

    /// Sends a `Sync::CreateAlarm` request (unchecked) (aux).
    #[inline]
    pub unsafe fn xcb_sync_create_alarm_aux(
        &self,
        c: *mut xcb_connection_t,
        id: xcb_sync_alarm_t,
        value_mask: u32,
        value_list: *const xcb_sync_create_alarm_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_create_alarm_aux)(c, id, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_sync_create_alarm_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_create_alarm_aux(&self) -> bool {
        has_sym!(self, xcb_sync_create_alarm_aux)
    }

    /// Returns a pointer to the `value_list` field of a `xcb_sync_create_alarm_request_t` struct.
    #[inline]
    pub unsafe fn xcb_sync_create_alarm_value_list(
        &self,
        r: *const xcb_sync_create_alarm_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_sync_create_alarm_value_list)(r)
    }

    /// Returns `true` iff the symbol `xcb_sync_create_alarm_value_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_create_alarm_value_list(&self) -> bool {
        has_sym!(self, xcb_sync_create_alarm_value_list)
    }

    /// Serializes a `xcb_sync_change_alarm_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_sync_change_alarm_value_list_serialize(
        &self,
        _buffer: *mut *mut c_void,
        value_mask: u32,
        _aux: *const xcb_sync_change_alarm_value_list_t,
    ) -> c_int {
        sym!(self, xcb_sync_change_alarm_value_list_serialize)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_sync_change_alarm_value_list_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_change_alarm_value_list_serialize(&self) -> bool {
        has_sym!(self, xcb_sync_change_alarm_value_list_serialize)
    }

    /// Unpacks a `xcb_sync_change_alarm_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_sync_change_alarm_value_list_unpack(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
        _aux: *mut xcb_sync_change_alarm_value_list_t,
    ) -> c_int {
        sym!(self, xcb_sync_change_alarm_value_list_unpack)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_sync_change_alarm_value_list_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_change_alarm_value_list_unpack(&self) -> bool {
        has_sym!(self, xcb_sync_change_alarm_value_list_unpack)
    }

    /// Computes the size of a `xcb_sync_change_alarm_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_sync_change_alarm_value_list_sizeof(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
    ) -> c_int {
        sym!(self, xcb_sync_change_alarm_value_list_sizeof)(_buffer, value_mask)
    }

    /// Returns `true` iff the symbol `xcb_sync_change_alarm_value_list_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_change_alarm_value_list_sizeof(&self) -> bool {
        has_sym!(self, xcb_sync_change_alarm_value_list_sizeof)
    }

    /// Computes the size of a `xcb_sync_change_alarm_request_t` object.
    #[inline]
    pub unsafe fn xcb_sync_change_alarm_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_sync_change_alarm_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_sync_change_alarm_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_change_alarm_sizeof(&self) -> bool {
        has_sym!(self, xcb_sync_change_alarm_sizeof)
    }

    /// Sends a `Sync::ChangeAlarm` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    ///
    /// There is an auxiliary version of this function: [`xcb_sync_change_alarm_aux_checked`].
    ///
    /// [`xcb_sync_change_alarm_aux_checked`]: Self::xcb_sync_change_alarm_aux_checked
    #[inline]
    pub unsafe fn xcb_sync_change_alarm_checked(
        &self,
        c: *mut xcb_connection_t,
        id: xcb_sync_alarm_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_change_alarm_checked)(c, id, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_sync_change_alarm_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_change_alarm_checked(&self) -> bool {
        has_sym!(self, xcb_sync_change_alarm_checked)
    }

    /// Sends a `Sync::ChangeAlarm` request (unchecked).
    ///
    /// There is an auxiliary version of this function: [`xcb_sync_change_alarm_aux`].
    ///
    /// [`xcb_sync_change_alarm_aux`]: Self::xcb_sync_change_alarm_aux
    #[inline]
    pub unsafe fn xcb_sync_change_alarm(
        &self,
        c: *mut xcb_connection_t,
        id: xcb_sync_alarm_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_change_alarm)(c, id, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_sync_change_alarm` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_change_alarm(&self) -> bool {
        has_sym!(self, xcb_sync_change_alarm)
    }

    /// Sends a `Sync::ChangeAlarm` request (checked) (aux).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_sync_change_alarm_aux_checked(
        &self,
        c: *mut xcb_connection_t,
        id: xcb_sync_alarm_t,
        value_mask: u32,
        value_list: *const xcb_sync_change_alarm_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_change_alarm_aux_checked)(c, id, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_sync_change_alarm_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_change_alarm_aux_checked(&self) -> bool {
        has_sym!(self, xcb_sync_change_alarm_aux_checked)
    }

    /// Sends a `Sync::ChangeAlarm` request (unchecked) (aux).
    #[inline]
    pub unsafe fn xcb_sync_change_alarm_aux(
        &self,
        c: *mut xcb_connection_t,
        id: xcb_sync_alarm_t,
        value_mask: u32,
        value_list: *const xcb_sync_change_alarm_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_change_alarm_aux)(c, id, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_sync_change_alarm_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_change_alarm_aux(&self) -> bool {
        has_sym!(self, xcb_sync_change_alarm_aux)
    }

    /// Returns a pointer to the `value_list` field of a `xcb_sync_change_alarm_request_t` struct.
    #[inline]
    pub unsafe fn xcb_sync_change_alarm_value_list(
        &self,
        r: *const xcb_sync_change_alarm_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_sync_change_alarm_value_list)(r)
    }

    /// Returns `true` iff the symbol `xcb_sync_change_alarm_value_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_change_alarm_value_list(&self) -> bool {
        has_sym!(self, xcb_sync_change_alarm_value_list)
    }

    /// Sends a `Sync::DestroyAlarm` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_sync_destroy_alarm_checked(
        &self,
        c: *mut xcb_connection_t,
        alarm: xcb_sync_alarm_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_destroy_alarm_checked)(c, alarm)
    }

    /// Returns `true` iff the symbol `xcb_sync_destroy_alarm_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_destroy_alarm_checked(&self) -> bool {
        has_sym!(self, xcb_sync_destroy_alarm_checked)
    }

    /// Sends a `Sync::DestroyAlarm` request (unchecked).
    #[inline]
    pub unsafe fn xcb_sync_destroy_alarm(
        &self,
        c: *mut xcb_connection_t,
        alarm: xcb_sync_alarm_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_destroy_alarm)(c, alarm)
    }

    /// Returns `true` iff the symbol `xcb_sync_destroy_alarm` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_destroy_alarm(&self) -> bool {
        has_sym!(self, xcb_sync_destroy_alarm)
    }

    /// Sends a `Sync::QueryAlarm` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_sync_query_alarm_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_sync_query_alarm_reply`]: Self::xcb_sync_query_alarm_reply
    #[inline]
    pub unsafe fn xcb_sync_query_alarm(
        &self,
        c: *mut xcb_connection_t,
        alarm: xcb_sync_alarm_t,
    ) -> xcb_sync_query_alarm_cookie_t {
        sym!(self, xcb_sync_query_alarm)(c, alarm)
    }

    /// Returns `true` iff the symbol `xcb_sync_query_alarm` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_query_alarm(&self) -> bool {
        has_sym!(self, xcb_sync_query_alarm)
    }

    /// Sends a `Sync::QueryAlarm` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_sync_query_alarm_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_sync_query_alarm_reply`]: Self::xcb_sync_query_alarm_reply
    #[inline]
    pub unsafe fn xcb_sync_query_alarm_unchecked(
        &self,
        c: *mut xcb_connection_t,
        alarm: xcb_sync_alarm_t,
    ) -> xcb_sync_query_alarm_cookie_t {
        sym!(self, xcb_sync_query_alarm_unchecked)(c, alarm)
    }

    /// Returns `true` iff the symbol `xcb_sync_query_alarm_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_query_alarm_unchecked(&self) -> bool {
        has_sym!(self, xcb_sync_query_alarm_unchecked)
    }

    /// Waits for the reply to a `Sync::QueryAlarm` request.
    #[inline]
    pub unsafe fn xcb_sync_query_alarm_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_sync_query_alarm_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_sync_query_alarm_reply_t {
        sym!(self, xcb_sync_query_alarm_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_sync_query_alarm_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_query_alarm_reply(&self) -> bool {
        has_sym!(self, xcb_sync_query_alarm_reply)
    }

    /// Sends a `Sync::SetPriority` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_sync_set_priority_checked(
        &self,
        c: *mut xcb_connection_t,
        id: u32,
        priority: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_set_priority_checked)(c, id, priority)
    }

    /// Returns `true` iff the symbol `xcb_sync_set_priority_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_set_priority_checked(&self) -> bool {
        has_sym!(self, xcb_sync_set_priority_checked)
    }

    /// Sends a `Sync::SetPriority` request (unchecked).
    #[inline]
    pub unsafe fn xcb_sync_set_priority(
        &self,
        c: *mut xcb_connection_t,
        id: u32,
        priority: i32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_set_priority)(c, id, priority)
    }

    /// Returns `true` iff the symbol `xcb_sync_set_priority` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_set_priority(&self) -> bool {
        has_sym!(self, xcb_sync_set_priority)
    }

    /// Sends a `Sync::GetPriority` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_sync_get_priority_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_sync_get_priority_reply`]: Self::xcb_sync_get_priority_reply
    #[inline]
    pub unsafe fn xcb_sync_get_priority(
        &self,
        c: *mut xcb_connection_t,
        id: u32,
    ) -> xcb_sync_get_priority_cookie_t {
        sym!(self, xcb_sync_get_priority)(c, id)
    }

    /// Returns `true` iff the symbol `xcb_sync_get_priority` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_get_priority(&self) -> bool {
        has_sym!(self, xcb_sync_get_priority)
    }

    /// Sends a `Sync::GetPriority` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_sync_get_priority_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_sync_get_priority_reply`]: Self::xcb_sync_get_priority_reply
    #[inline]
    pub unsafe fn xcb_sync_get_priority_unchecked(
        &self,
        c: *mut xcb_connection_t,
        id: u32,
    ) -> xcb_sync_get_priority_cookie_t {
        sym!(self, xcb_sync_get_priority_unchecked)(c, id)
    }

    /// Returns `true` iff the symbol `xcb_sync_get_priority_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_get_priority_unchecked(&self) -> bool {
        has_sym!(self, xcb_sync_get_priority_unchecked)
    }

    /// Waits for the reply to a `Sync::GetPriority` request.
    #[inline]
    pub unsafe fn xcb_sync_get_priority_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_sync_get_priority_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_sync_get_priority_reply_t {
        sym!(self, xcb_sync_get_priority_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_sync_get_priority_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_get_priority_reply(&self) -> bool {
        has_sym!(self, xcb_sync_get_priority_reply)
    }

    /// Sends a `Sync::CreateFence` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_sync_create_fence_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        fence: xcb_sync_fence_t,
        initially_triggered: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_create_fence_checked)(c, drawable, fence, initially_triggered)
    }

    /// Returns `true` iff the symbol `xcb_sync_create_fence_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_create_fence_checked(&self) -> bool {
        has_sym!(self, xcb_sync_create_fence_checked)
    }

    /// Sends a `Sync::CreateFence` request (unchecked).
    #[inline]
    pub unsafe fn xcb_sync_create_fence(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        fence: xcb_sync_fence_t,
        initially_triggered: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_create_fence)(c, drawable, fence, initially_triggered)
    }

    /// Returns `true` iff the symbol `xcb_sync_create_fence` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_create_fence(&self) -> bool {
        has_sym!(self, xcb_sync_create_fence)
    }

    /// Sends a `Sync::TriggerFence` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_sync_trigger_fence_checked(
        &self,
        c: *mut xcb_connection_t,
        fence: xcb_sync_fence_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_trigger_fence_checked)(c, fence)
    }

    /// Returns `true` iff the symbol `xcb_sync_trigger_fence_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_trigger_fence_checked(&self) -> bool {
        has_sym!(self, xcb_sync_trigger_fence_checked)
    }

    /// Sends a `Sync::TriggerFence` request (unchecked).
    #[inline]
    pub unsafe fn xcb_sync_trigger_fence(
        &self,
        c: *mut xcb_connection_t,
        fence: xcb_sync_fence_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_trigger_fence)(c, fence)
    }

    /// Returns `true` iff the symbol `xcb_sync_trigger_fence` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_trigger_fence(&self) -> bool {
        has_sym!(self, xcb_sync_trigger_fence)
    }

    /// Sends a `Sync::ResetFence` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_sync_reset_fence_checked(
        &self,
        c: *mut xcb_connection_t,
        fence: xcb_sync_fence_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_reset_fence_checked)(c, fence)
    }

    /// Returns `true` iff the symbol `xcb_sync_reset_fence_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_reset_fence_checked(&self) -> bool {
        has_sym!(self, xcb_sync_reset_fence_checked)
    }

    /// Sends a `Sync::ResetFence` request (unchecked).
    #[inline]
    pub unsafe fn xcb_sync_reset_fence(
        &self,
        c: *mut xcb_connection_t,
        fence: xcb_sync_fence_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_reset_fence)(c, fence)
    }

    /// Returns `true` iff the symbol `xcb_sync_reset_fence` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_reset_fence(&self) -> bool {
        has_sym!(self, xcb_sync_reset_fence)
    }

    /// Sends a `Sync::DestroyFence` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_sync_destroy_fence_checked(
        &self,
        c: *mut xcb_connection_t,
        fence: xcb_sync_fence_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_destroy_fence_checked)(c, fence)
    }

    /// Returns `true` iff the symbol `xcb_sync_destroy_fence_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_destroy_fence_checked(&self) -> bool {
        has_sym!(self, xcb_sync_destroy_fence_checked)
    }

    /// Sends a `Sync::DestroyFence` request (unchecked).
    #[inline]
    pub unsafe fn xcb_sync_destroy_fence(
        &self,
        c: *mut xcb_connection_t,
        fence: xcb_sync_fence_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_destroy_fence)(c, fence)
    }

    /// Returns `true` iff the symbol `xcb_sync_destroy_fence` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_destroy_fence(&self) -> bool {
        has_sym!(self, xcb_sync_destroy_fence)
    }

    /// Sends a `Sync::QueryFence` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_sync_query_fence_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_sync_query_fence_reply`]: Self::xcb_sync_query_fence_reply
    #[inline]
    pub unsafe fn xcb_sync_query_fence(
        &self,
        c: *mut xcb_connection_t,
        fence: xcb_sync_fence_t,
    ) -> xcb_sync_query_fence_cookie_t {
        sym!(self, xcb_sync_query_fence)(c, fence)
    }

    /// Returns `true` iff the symbol `xcb_sync_query_fence` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_query_fence(&self) -> bool {
        has_sym!(self, xcb_sync_query_fence)
    }

    /// Sends a `Sync::QueryFence` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_sync_query_fence_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_sync_query_fence_reply`]: Self::xcb_sync_query_fence_reply
    #[inline]
    pub unsafe fn xcb_sync_query_fence_unchecked(
        &self,
        c: *mut xcb_connection_t,
        fence: xcb_sync_fence_t,
    ) -> xcb_sync_query_fence_cookie_t {
        sym!(self, xcb_sync_query_fence_unchecked)(c, fence)
    }

    /// Returns `true` iff the symbol `xcb_sync_query_fence_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_query_fence_unchecked(&self) -> bool {
        has_sym!(self, xcb_sync_query_fence_unchecked)
    }

    /// Waits for the reply to a `Sync::QueryFence` request.
    #[inline]
    pub unsafe fn xcb_sync_query_fence_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_sync_query_fence_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_sync_query_fence_reply_t {
        sym!(self, xcb_sync_query_fence_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_sync_query_fence_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_query_fence_reply(&self) -> bool {
        has_sym!(self, xcb_sync_query_fence_reply)
    }

    /// Computes the size of a `xcb_sync_await_fence_request_t` object.
    #[inline]
    pub unsafe fn xcb_sync_await_fence_sizeof(
        &self,
        _buffer: *const c_void,
        fence_list_len: u32,
    ) -> c_int {
        sym!(self, xcb_sync_await_fence_sizeof)(_buffer, fence_list_len)
    }

    /// Returns `true` iff the symbol `xcb_sync_await_fence_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_await_fence_sizeof(&self) -> bool {
        has_sym!(self, xcb_sync_await_fence_sizeof)
    }

    /// Sends a `Sync::AwaitFence` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_sync_await_fence_checked(
        &self,
        c: *mut xcb_connection_t,
        fence_list_len: u32,
        fence_list: *const xcb_sync_fence_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_await_fence_checked)(c, fence_list_len, fence_list)
    }

    /// Returns `true` iff the symbol `xcb_sync_await_fence_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_await_fence_checked(&self) -> bool {
        has_sym!(self, xcb_sync_await_fence_checked)
    }

    /// Sends a `Sync::AwaitFence` request (unchecked).
    #[inline]
    pub unsafe fn xcb_sync_await_fence(
        &self,
        c: *mut xcb_connection_t,
        fence_list_len: u32,
        fence_list: *const xcb_sync_fence_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_sync_await_fence)(c, fence_list_len, fence_list)
    }

    /// Returns `true` iff the symbol `xcb_sync_await_fence` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_await_fence(&self) -> bool {
        has_sym!(self, xcb_sync_await_fence)
    }

    /// Returns a pointer to the `fence_list` field of a `xcb_sync_await_fence_request_t` struct.
    #[inline]
    pub unsafe fn xcb_sync_await_fence_fence_list(
        &self,
        r: *const xcb_sync_await_fence_request_t,
    ) -> *mut xcb_sync_fence_t {
        sym!(self, xcb_sync_await_fence_fence_list)(r)
    }

    /// Returns `true` iff the symbol `xcb_sync_await_fence_fence_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_await_fence_fence_list(&self) -> bool {
        has_sym!(self, xcb_sync_await_fence_fence_list)
    }

    /// Returns the number of elements of the `fence_list` field of a `xcb_sync_await_fence_request_t` struct.
    #[inline]
    pub unsafe fn xcb_sync_await_fence_fence_list_length(
        &self,
        r: *const xcb_sync_await_fence_request_t,
    ) -> c_int {
        sym!(self, xcb_sync_await_fence_fence_list_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_sync_await_fence_fence_list_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_await_fence_fence_list_length(&self) -> bool {
        has_sym!(self, xcb_sync_await_fence_fence_list_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `fence_list` field of a `xcb_sync_await_fence_request_t` struct.
    #[inline]
    pub unsafe fn xcb_sync_await_fence_fence_list_end(
        &self,
        r: *const xcb_sync_await_fence_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_sync_await_fence_fence_list_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_sync_await_fence_fence_list_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_sync_await_fence_fence_list_end(&self) -> bool {
        has_sym!(self, xcb_sync_await_fence_fence_list_end)
    }
}

#[cfg(feature = "xcb_sync")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbSync::load().unwrap() };
        assert!(lib.has_xcb_sync_id());
        assert!(lib.has_xcb_sync_alarm_next());
        assert!(lib.has_xcb_sync_alarm_end());
        assert!(lib.has_xcb_sync_counter_next());
        assert!(lib.has_xcb_sync_counter_end());
        assert!(lib.has_xcb_sync_fence_next());
        assert!(lib.has_xcb_sync_fence_end());
        assert!(lib.has_xcb_sync_int64_next());
        assert!(lib.has_xcb_sync_int64_end());
        assert!(lib.has_xcb_sync_systemcounter_sizeof());
        assert!(lib.has_xcb_sync_systemcounter_name());
        assert!(lib.has_xcb_sync_systemcounter_name_length());
        assert!(lib.has_xcb_sync_systemcounter_name_end());
        assert!(lib.has_xcb_sync_systemcounter_next());
        assert!(lib.has_xcb_sync_systemcounter_end());
        assert!(lib.has_xcb_sync_trigger_next());
        assert!(lib.has_xcb_sync_trigger_end());
        assert!(lib.has_xcb_sync_waitcondition_next());
        assert!(lib.has_xcb_sync_waitcondition_end());
        assert!(lib.has_xcb_sync_initialize());
        assert!(lib.has_xcb_sync_initialize_unchecked());
        assert!(lib.has_xcb_sync_initialize_reply());
        assert!(lib.has_xcb_sync_list_system_counters_sizeof());
        assert!(lib.has_xcb_sync_list_system_counters());
        assert!(lib.has_xcb_sync_list_system_counters_unchecked());
        assert!(lib.has_xcb_sync_list_system_counters_counters_length());
        assert!(lib.has_xcb_sync_list_system_counters_counters_iterator());
        assert!(lib.has_xcb_sync_list_system_counters_reply());
        assert!(lib.has_xcb_sync_create_counter_checked());
        assert!(lib.has_xcb_sync_create_counter());
        assert!(lib.has_xcb_sync_destroy_counter_checked());
        assert!(lib.has_xcb_sync_destroy_counter());
        assert!(lib.has_xcb_sync_query_counter());
        assert!(lib.has_xcb_sync_query_counter_unchecked());
        assert!(lib.has_xcb_sync_query_counter_reply());
        assert!(lib.has_xcb_sync_await_sizeof());
        assert!(lib.has_xcb_sync_await_checked());
        assert!(lib.has_xcb_sync_await());
        assert!(lib.has_xcb_sync_await_wait_list());
        assert!(lib.has_xcb_sync_await_wait_list_length());
        assert!(lib.has_xcb_sync_await_wait_list_iterator());
        assert!(lib.has_xcb_sync_change_counter_checked());
        assert!(lib.has_xcb_sync_change_counter());
        assert!(lib.has_xcb_sync_set_counter_checked());
        assert!(lib.has_xcb_sync_set_counter());
        assert!(lib.has_xcb_sync_create_alarm_value_list_serialize());
        assert!(lib.has_xcb_sync_create_alarm_value_list_unpack());
        assert!(lib.has_xcb_sync_create_alarm_value_list_sizeof());
        assert!(lib.has_xcb_sync_create_alarm_sizeof());
        assert!(lib.has_xcb_sync_create_alarm_checked());
        assert!(lib.has_xcb_sync_create_alarm());
        assert!(lib.has_xcb_sync_create_alarm_aux_checked());
        assert!(lib.has_xcb_sync_create_alarm_aux());
        assert!(lib.has_xcb_sync_create_alarm_value_list());
        assert!(lib.has_xcb_sync_change_alarm_value_list_serialize());
        assert!(lib.has_xcb_sync_change_alarm_value_list_unpack());
        assert!(lib.has_xcb_sync_change_alarm_value_list_sizeof());
        assert!(lib.has_xcb_sync_change_alarm_sizeof());
        assert!(lib.has_xcb_sync_change_alarm_checked());
        assert!(lib.has_xcb_sync_change_alarm());
        assert!(lib.has_xcb_sync_change_alarm_aux_checked());
        assert!(lib.has_xcb_sync_change_alarm_aux());
        assert!(lib.has_xcb_sync_change_alarm_value_list());
        assert!(lib.has_xcb_sync_destroy_alarm_checked());
        assert!(lib.has_xcb_sync_destroy_alarm());
        assert!(lib.has_xcb_sync_query_alarm());
        assert!(lib.has_xcb_sync_query_alarm_unchecked());
        assert!(lib.has_xcb_sync_query_alarm_reply());
        assert!(lib.has_xcb_sync_set_priority_checked());
        assert!(lib.has_xcb_sync_set_priority());
        assert!(lib.has_xcb_sync_get_priority());
        assert!(lib.has_xcb_sync_get_priority_unchecked());
        assert!(lib.has_xcb_sync_get_priority_reply());
        assert!(lib.has_xcb_sync_create_fence_checked());
        assert!(lib.has_xcb_sync_create_fence());
        assert!(lib.has_xcb_sync_trigger_fence_checked());
        assert!(lib.has_xcb_sync_trigger_fence());
        assert!(lib.has_xcb_sync_reset_fence_checked());
        assert!(lib.has_xcb_sync_reset_fence());
        assert!(lib.has_xcb_sync_destroy_fence_checked());
        assert!(lib.has_xcb_sync_destroy_fence());
        assert!(lib.has_xcb_sync_query_fence());
        assert!(lib.has_xcb_sync_query_fence_unchecked());
        assert!(lib.has_xcb_sync_query_fence_reply());
        assert!(lib.has_xcb_sync_await_fence_sizeof());
        assert!(lib.has_xcb_sync_await_fence_checked());
        assert!(lib.has_xcb_sync_await_fence());
        assert!(lib.has_xcb_sync_await_fence_fence_list());
        assert!(lib.has_xcb_sync_await_fence_fence_list_length());
        assert!(lib.has_xcb_sync_await_fence_fence_list_end());
    }
}
