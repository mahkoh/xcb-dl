use crate::lazy::{LazySymbol, NamedLibrary};
use crate::{
    xcb_auth_info_t, xcb_connection_t, xcb_extension_t, xcb_generic_error_t, xcb_generic_event_t,
    xcb_special_event_t, xcb_void_cookie_t,
};
use libloading::{Error, Library};
use std::os::raw::{c_char, c_int, c_uint};

const XCB_LIB_NAME: &str = "libxcb.so.1";

#[rustfmt::skip]
pub struct Xcb {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_flush: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> c_int>,
    pub(crate) xcb_get_maximum_request_length: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> u32>,
    pub(crate) xcb_prefetch_maximum_request_length: LazySymbol<unsafe fn(c: *mut xcb_connection_t)>,
    pub(crate) xcb_wait_for_event: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t>,
    pub(crate) xcb_poll_for_event: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t>,
    pub(crate) xcb_poll_for_queued_event: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t>,
    pub(crate) xcb_poll_for_special_event: LazySymbol<unsafe fn(c: *mut xcb_connection_t, se: *mut xcb_special_event_t) -> *mut xcb_generic_event_t>,
    pub(crate) xcb_wait_for_special_event: LazySymbol<unsafe fn(c: *mut xcb_connection_t, se: *mut xcb_special_event_t) -> *mut xcb_generic_event_t>,
    pub(crate) xcb_register_for_special_xge: LazySymbol<unsafe fn(c: *mut xcb_connection_t, ext: *mut xcb_extension_t, eid: u32, stamp: *mut u32) -> *mut xcb_special_event_t>,
    pub(crate) xcb_unregister_for_special_event: LazySymbol<unsafe fn(c: *mut xcb_connection_t, se: *mut xcb_special_event_t)>,
    pub(crate) xcb_request_check: LazySymbol<unsafe fn(c: *mut xcb_connection_t, cookie: xcb_void_cookie_t) -> *mut xcb_generic_error_t>,
    pub(crate) xcb_discard_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, sequence: c_uint)>,
    pub(crate) xcb_discard_reply64: LazySymbol<unsafe fn(c: *mut xcb_connection_t, sequence: u64)>,
    // pub(crate) xcb_get_extension_data: LazySymbol<unsafe fn(c: *mut xcb_connection_t, ext: *mut xcb_extension_t) -> *const xcb_query_extension_reply_t>,
    pub(crate) xcb_prefetch_extension_data: LazySymbol<unsafe fn(c: *mut xcb_connection_t, ext: *mut xcb_extension_t)>,
    // pub(crate) xcb_get_setup: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> *const xcb_setup_t>,
    pub(crate) xcb_get_file_descriptor: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> c_int>,
    pub(crate) xcb_connection_has_error: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> c_int>,
    pub(crate) xcb_connect_to_fd: LazySymbol<unsafe fn(fd: c_int, auth_info: *mut xcb_auth_info_t) -> *mut xcb_connection_t>,
    pub(crate) xcb_disconnect: LazySymbol<unsafe fn(c: *mut xcb_connection_t)>,
    pub(crate) xcb_parse_display: LazySymbol<unsafe fn(name: *const c_char, host: *mut *mut c_char, display: *mut c_int, screen: *mut c_int) -> c_int>,
    pub(crate) xcb_connect: LazySymbol<unsafe fn(displayname: *const c_char, screenp: *mut c_int) -> *mut xcb_connection_t>,
    pub(crate) xcb_connect_to_display_with_auth_info: LazySymbol<unsafe fn(display: *const c_char, auth: *mut xcb_auth_info_t, screen: *mut c_int) -> *mut xcb_connection_t>,
    pub(crate) xcb_generate_id: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> u32>,
}

impl Xcb {
    #[inline]
    pub unsafe fn new() -> Result<Self, Error> {
        let lib = Library::new(XCB_LIB_NAME)?;
        Ok(Self {
            lib: NamedLibrary {
                name: XCB_LIB_NAME,
                lib,
            },
            xcb_flush: Default::default(),
            xcb_get_maximum_request_length: Default::default(),
            xcb_prefetch_maximum_request_length: Default::default(),
            xcb_wait_for_event: Default::default(),
            xcb_poll_for_event: Default::default(),
            xcb_poll_for_queued_event: Default::default(),
            xcb_poll_for_special_event: Default::default(),
            xcb_wait_for_special_event: Default::default(),
            xcb_register_for_special_xge: Default::default(),
            xcb_unregister_for_special_event: Default::default(),
            xcb_request_check: Default::default(),
            xcb_discard_reply: Default::default(),
            xcb_discard_reply64: Default::default(),
            // xcb_get_extension_data: Default::default(),
            xcb_prefetch_extension_data: Default::default(),
            // xcb_get_setup: Default::default(),
            xcb_get_file_descriptor: Default::default(),
            xcb_connection_has_error: Default::default(),
            xcb_connect_to_fd: Default::default(),
            xcb_disconnect: Default::default(),
            xcb_parse_display: Default::default(),
            xcb_connect: Default::default(),
            xcb_connect_to_display_with_auth_info: Default::default(),
            xcb_generate_id: Default::default(),
        })
    }
}
