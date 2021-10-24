use std::os::raw::{c_char, c_int};

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_extension_t {
    pub name: *const c_char,
    pub global_id: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_protocol_request_t {
    pub count: usize,
    pub ext: *mut xcb_extension_t,
    pub opcode: u8,
    pub isvoid: u8,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub enum xcb_send_request_flags_t {
    XCB_REQUEST_CHECKED = 0x01,
    XCB_REQUEST_RAW = 0x02,
    XCB_REQUEST_DISCARD_REPLY = 0x04,
    XCB_REQUEST_REPLY_FDS = 0x08,
}
