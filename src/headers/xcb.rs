use crate::ffi::*;
use crate::lazy::LazySymbol;
use crate::*;
use std::os::raw::*;

/// xcb connection errors because of socket, pipe and other stream errors.
pub const XCB_CONN_ERROR: c_int = 1;
/// xcb connection shutdown because of extension not supported
pub const XCB_CONN_CLOSED_EXT_NOTSUPPORTED: c_int = 2;
/// malloc(), calloc() and realloc() error upon failure, for eg ENOMEM
pub const XCB_CONN_CLOSED_MEM_INSUFFICIENT: c_int = 3;
/// Connection closed, exceeding request length that server accepts.
pub const XCB_CONN_CLOSED_REQ_LEN_EXCEED: c_int = 4;
/// Connection closed, error during parsing display string.
pub const XCB_CONN_CLOSED_PARSE_ERR: c_int = 5;
/// Connection closed because the server does not have a screen
/// matching the display.
pub const XCB_CONN_CLOSED_INVALID_SCREEN: c_int = 6;
/// Connection closed because some FD passing operation failed
pub const XCB_CONN_CLOSED_FDPASSING_FAILED: c_int = 7;

/// XCB Connection structure.
pub enum xcb_connection_t {}

/// Generic iterator.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_generic_iterator_t {
    pub data: *mut c_void,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_generic_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Generic reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_generic_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
}

impl Default for xcb_generic_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Generic event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_generic_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub pad: [u32; 7],
    pub full_sequence: u32,
}

impl Default for xcb_generic_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Raw Generic event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_raw_generic_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub pad: [u32; 7],
}

impl Default for xcb_raw_generic_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// GE event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_ge_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub pad1: u16,
    pub pad: [u32; 5],
    pub full_sequence: u32,
}

impl Default for xcb_ge_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Generic error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_generic_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub resource_id: u32,
    pub minor_code: u16,
    pub major_code: u8,
    pub pad0: u8,
    pub pad: [u32; 5],
    pub full_sequence: u32,
}

impl Default for xcb_generic_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Generic cookie.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_void_cookie_t {
    /// sequence number
    pub sequence: c_uint,
}

impl Default for xcb_void_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// XCB_NONE is the universal null resource or null atom parameter value for many core X requests
pub const XCB_NONE: u32 = 0;
/// XCB_COPY_FROM_PARENT can be used for many xcb_create_window parameters
pub const XCB_COPY_FROM_PARENT: u32 = 0;
/// XCB_CURRENT_TIME can be used in most requests that take an xcb_timestamp_t
pub const XCB_CURRENT_TIME: u32 = 0;
/// XCB_NO_SYMBOL fills in unused entries in xcb_keysym_t tables
pub const XCB_NO_SYMBOL: u32 = 0;

/// Container for authorization information.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_auth_info_t {
    pub namelen: c_int,
    pub name: *mut c_char,
    pub datalen: c_int,
    pub data: *mut c_char,
}

impl Default for xcb_auth_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub enum xcb_special_event_t {}

pub(crate) struct XcbXcb {
    xcb_flush: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> c_int>,
    xcb_get_maximum_request_length: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> u32>,
    xcb_prefetch_maximum_request_length: LazySymbol<unsafe fn(c: *mut xcb_connection_t)>,
    xcb_wait_for_event: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t>,
    xcb_poll_for_event: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t>,
    xcb_poll_for_queued_event:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t>,
    xcb_poll_for_special_event: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            se: *mut xcb_special_event_t,
        ) -> *mut xcb_generic_event_t,
    >,
    xcb_wait_for_special_event: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            se: *mut xcb_special_event_t,
        ) -> *mut xcb_generic_event_t,
    >,
    xcb_register_for_special_xge: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            ext: *mut xcb_extension_t,
            eid: u32,
            stamp: *mut u32,
        ) -> *mut xcb_special_event_t,
    >,
    xcb_unregister_for_special_event:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, se: *mut xcb_special_event_t)>,
    xcb_request_check: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, cookie: xcb_void_cookie_t) -> *mut xcb_generic_error_t,
    >,
    xcb_discard_reply: LazySymbol<unsafe fn(c: *mut xcb_connection_t, sequence: c_uint)>,
    xcb_discard_reply64: LazySymbol<unsafe fn(c: *mut xcb_connection_t, sequence: u64)>,
    xcb_get_extension_data: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            ext: *mut xcb_extension_t,
        ) -> *const xcb_query_extension_reply_t,
    >,
    xcb_prefetch_extension_data:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, ext: *mut xcb_extension_t)>,
    xcb_get_setup: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> *const xcb_setup_t>,
    xcb_get_file_descriptor: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> c_int>,
    xcb_connection_has_error: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> c_int>,
    xcb_connect_to_fd:
        LazySymbol<unsafe fn(fd: c_int, auth_info: *mut xcb_auth_info_t) -> *mut xcb_connection_t>,
    xcb_disconnect: LazySymbol<unsafe fn(c: *mut xcb_connection_t)>,
    xcb_parse_display: LazySymbol<
        unsafe fn(
            name: *const c_char,
            host: *mut *mut c_char,
            display: *mut c_int,
            screen: *mut c_int,
        ) -> c_int,
    >,
    xcb_connect: LazySymbol<
        unsafe fn(displayname: *const c_char, screenp: *mut c_int) -> *mut xcb_connection_t,
    >,
    xcb_connect_to_display_with_auth_info: LazySymbol<
        unsafe fn(
            display: *const c_char,
            auth: *mut xcb_auth_info_t,
            screen: *mut c_int,
        ) -> *mut xcb_connection_t,
    >,
    xcb_generate_id: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> u32>,
    xcb_total_read: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> u64>,
    xcb_total_written: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> u64>,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        ($self.xcb.$f.get(&$self.lib, concat!(stringify!($f), "\0")))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            ($self
                .xcb
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0")))
        }
    };
}

impl Xcb {
    /// Forces any buffered output to be written to the server.
    #[inline]
    pub unsafe fn xcb_flush(&self, c: *mut xcb_connection_t) -> c_int {
        sym!(self, xcb_flush)(c)
    }

    /// Returns `true` iff the symbol `xcb_flush` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_flush(&self) -> bool {
        has_sym!(self, xcb_flush)
    }

    /// Returns the maximum request length that this server accepts.
    #[inline]
    pub unsafe fn xcb_get_maximum_request_length(&self, c: *mut xcb_connection_t) -> u32 {
        sym!(self, xcb_get_maximum_request_length)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_maximum_request_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_maximum_request_length(&self) -> bool {
        has_sym!(self, xcb_get_maximum_request_length)
    }

    /// Prefetch the maximum request length without blocking.
    #[inline]
    pub unsafe fn xcb_prefetch_maximum_request_length(&self, c: *mut xcb_connection_t) {
        sym!(self, xcb_prefetch_maximum_request_length)(c)
    }

    /// Returns `true` iff the symbol `xcb_prefetch_maximum_request_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_prefetch_maximum_request_length(&self) -> bool {
        has_sym!(self, xcb_prefetch_maximum_request_length)
    }

    /// Returns the next event or error from the server.
    #[inline]
    pub unsafe fn xcb_wait_for_event(&self, c: *mut xcb_connection_t) -> *mut xcb_generic_event_t {
        sym!(self, xcb_wait_for_event)(c)
    }

    /// Returns `true` iff the symbol `xcb_wait_for_event` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_wait_for_event(&self) -> bool {
        has_sym!(self, xcb_wait_for_event)
    }

    /// Returns the next event or error from the server.
    #[inline]
    pub unsafe fn xcb_poll_for_event(&self, c: *mut xcb_connection_t) -> *mut xcb_generic_event_t {
        sym!(self, xcb_poll_for_event)(c)
    }

    /// Returns `true` iff the symbol `xcb_poll_for_event` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poll_for_event(&self) -> bool {
        has_sym!(self, xcb_poll_for_event)
    }

    /// Returns the next event without reading from the connection.
    #[inline]
    pub unsafe fn xcb_poll_for_queued_event(
        &self,
        c: *mut xcb_connection_t,
    ) -> *mut xcb_generic_event_t {
        sym!(self, xcb_poll_for_queued_event)(c)
    }

    /// Returns `true` iff the symbol `xcb_poll_for_queued_event` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poll_for_queued_event(&self) -> bool {
        has_sym!(self, xcb_poll_for_queued_event)
    }

    /// Returns the next event from a special queue
    #[inline]
    pub unsafe fn xcb_poll_for_special_event(
        &self,
        c: *mut xcb_connection_t,
        se: *mut xcb_special_event_t,
    ) -> *mut xcb_generic_event_t {
        sym!(self, xcb_poll_for_special_event)(c, se)
    }

    /// Returns `true` iff the symbol `xcb_poll_for_special_event` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poll_for_special_event(&self) -> bool {
        has_sym!(self, xcb_poll_for_special_event)
    }

    /// Returns the next event from a special queue, blocking until one arrives
    #[inline]
    pub unsafe fn xcb_wait_for_special_event(
        &self,
        c: *mut xcb_connection_t,
        se: *mut xcb_special_event_t,
    ) -> *mut xcb_generic_event_t {
        sym!(self, xcb_wait_for_special_event)(c, se)
    }

    /// Returns `true` iff the symbol `xcb_wait_for_special_event` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_wait_for_special_event(&self) -> bool {
        has_sym!(self, xcb_wait_for_special_event)
    }

    /// Listen for a special event
    #[inline]
    pub unsafe fn xcb_register_for_special_xge(
        &self,
        c: *mut xcb_connection_t,
        ext: *mut xcb_extension_t,
        eid: u32,
        stamp: *mut u32,
    ) -> *mut xcb_special_event_t {
        sym!(self, xcb_register_for_special_xge)(c, ext, eid, stamp)
    }

    /// Returns `true` iff the symbol `xcb_register_for_special_xge` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_register_for_special_xge(&self) -> bool {
        has_sym!(self, xcb_register_for_special_xge)
    }

    /// Stop listening for a special event
    #[inline]
    pub unsafe fn xcb_unregister_for_special_event(
        &self,
        c: *mut xcb_connection_t,
        se: *mut xcb_special_event_t,
    ) {
        sym!(self, xcb_unregister_for_special_event)(c, se)
    }

    /// Returns `true` iff the symbol `xcb_unregister_for_special_event` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_unregister_for_special_event(&self) -> bool {
        has_sym!(self, xcb_unregister_for_special_event)
    }

    /// Return the error for a request, or NULL if none can ever arrive.
    #[inline]
    pub unsafe fn xcb_request_check(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_void_cookie_t,
    ) -> *mut xcb_generic_error_t {
        sym!(self, xcb_request_check)(c, cookie)
    }

    /// Returns `true` iff the symbol `xcb_request_check` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_request_check(&self) -> bool {
        has_sym!(self, xcb_request_check)
    }

    /// Discards the reply for a request.
    #[inline]
    pub unsafe fn xcb_discard_reply(&self, c: *mut xcb_connection_t, sequence: c_uint) {
        sym!(self, xcb_discard_reply)(c, sequence)
    }

    /// Returns `true` iff the symbol `xcb_discard_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_discard_reply(&self) -> bool {
        has_sym!(self, xcb_discard_reply)
    }

    /// Discards the reply for a request, given by a 64bit sequence number
    #[inline]
    pub unsafe fn xcb_discard_reply64(&self, c: *mut xcb_connection_t, sequence: u64) {
        sym!(self, xcb_discard_reply64)(c, sequence)
    }

    /// Returns `true` iff the symbol `xcb_discard_reply64` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_discard_reply64(&self) -> bool {
        has_sym!(self, xcb_discard_reply64)
    }

    /// Caches reply information from QueryExtension requests.
    #[inline]
    pub unsafe fn xcb_get_extension_data(
        &self,
        c: *mut xcb_connection_t,
        ext: *mut xcb_extension_t,
    ) -> *const xcb_query_extension_reply_t {
        sym!(self, xcb_get_extension_data)(c, ext)
    }

    /// Returns `true` iff the symbol `xcb_get_extension_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_extension_data(&self) -> bool {
        has_sym!(self, xcb_get_extension_data)
    }

    /// Prefetch of extension data into the extension cache
    #[inline]
    pub unsafe fn xcb_prefetch_extension_data(
        &self,
        c: *mut xcb_connection_t,
        ext: *mut xcb_extension_t,
    ) {
        sym!(self, xcb_prefetch_extension_data)(c, ext)
    }

    /// Returns `true` iff the symbol `xcb_prefetch_extension_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_prefetch_extension_data(&self) -> bool {
        has_sym!(self, xcb_prefetch_extension_data)
    }

    /// Access the data returned by the server.
    #[inline]
    pub unsafe fn xcb_get_setup(&self, c: *mut xcb_connection_t) -> *const xcb_setup_t {
        sym!(self, xcb_get_setup)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_setup` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_setup(&self) -> bool {
        has_sym!(self, xcb_get_setup)
    }

    /// Access the file descriptor of the connection.
    #[inline]
    pub unsafe fn xcb_get_file_descriptor(&self, c: *mut xcb_connection_t) -> c_int {
        sym!(self, xcb_get_file_descriptor)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_file_descriptor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_file_descriptor(&self) -> bool {
        has_sym!(self, xcb_get_file_descriptor)
    }

    /// Test whether the connection has shut down due to a fatal error.
    #[inline]
    pub unsafe fn xcb_connection_has_error(&self, c: *mut xcb_connection_t) -> c_int {
        sym!(self, xcb_connection_has_error)(c)
    }

    /// Returns `true` iff the symbol `xcb_connection_has_error` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_connection_has_error(&self) -> bool {
        has_sym!(self, xcb_connection_has_error)
    }

    /// Connects to the X server.
    #[inline]
    pub unsafe fn xcb_connect_to_fd(
        &self,
        fd: c_int,
        auth_info: *mut xcb_auth_info_t,
    ) -> *mut xcb_connection_t {
        sym!(self, xcb_connect_to_fd)(fd, auth_info)
    }

    /// Returns `true` iff the symbol `xcb_connect_to_fd` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_connect_to_fd(&self) -> bool {
        has_sym!(self, xcb_connect_to_fd)
    }

    /// Closes the connection.
    #[inline]
    pub unsafe fn xcb_disconnect(&self, c: *mut xcb_connection_t) {
        sym!(self, xcb_disconnect)(c)
    }

    /// Returns `true` iff the symbol `xcb_disconnect` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_disconnect(&self) -> bool {
        has_sym!(self, xcb_disconnect)
    }

    /// Parses a display string name in the form documented by X(7x).
    #[inline]
    pub unsafe fn xcb_parse_display(
        &self,
        name: *const c_char,
        host: *mut *mut c_char,
        display: *mut c_int,
        screen: *mut c_int,
    ) -> c_int {
        sym!(self, xcb_parse_display)(name, host, display, screen)
    }

    /// Returns `true` iff the symbol `xcb_parse_display` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_parse_display(&self) -> bool {
        has_sym!(self, xcb_parse_display)
    }

    /// Connects to the X server.
    #[inline]
    pub unsafe fn xcb_connect(
        &self,
        displayname: *const c_char,
        screenp: *mut c_int,
    ) -> *mut xcb_connection_t {
        sym!(self, xcb_connect)(displayname, screenp)
    }

    /// Returns `true` iff the symbol `xcb_connect` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_connect(&self) -> bool {
        has_sym!(self, xcb_connect)
    }

    /// Connects to the X server, using an authorization information.
    #[inline]
    pub unsafe fn xcb_connect_to_display_with_auth_info(
        &self,
        display: *const c_char,
        auth: *mut xcb_auth_info_t,
        screen: *mut c_int,
    ) -> *mut xcb_connection_t {
        sym!(self, xcb_connect_to_display_with_auth_info)(display, auth, screen)
    }

    /// Returns `true` iff the symbol `xcb_connect_to_display_with_auth_info` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_connect_to_display_with_auth_info(&self) -> bool {
        has_sym!(self, xcb_connect_to_display_with_auth_info)
    }

    /// Allocates an XID for a new object.
    #[inline]
    pub unsafe fn xcb_generate_id(&self, c: *mut xcb_connection_t) -> u32 {
        sym!(self, xcb_generate_id)(c)
    }

    /// Returns `true` iff the symbol `xcb_generate_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_generate_id(&self) -> bool {
        has_sym!(self, xcb_generate_id)
    }

    /// Obtain number of bytes read from the connection.
    #[inline]
    pub unsafe fn xcb_total_read(&self, c: *mut xcb_connection_t) -> u64 {
        sym!(self, xcb_total_read)(c)
    }

    /// Returns `true` iff the symbol `xcb_total_read` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_total_read(&self) -> bool {
        has_sym!(self, xcb_total_read)
    }

    /// Obtain number of bytes written to the connection.
    #[inline]
    pub unsafe fn xcb_total_written(&self, c: *mut xcb_connection_t) -> u64 {
        sym!(self, xcb_total_written)(c)
    }

    /// Returns `true` iff the symbol `xcb_total_written` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_total_written(&self) -> bool {
        has_sym!(self, xcb_total_written)
    }
}

#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::Xcb::load().unwrap() };
        assert!(lib.has_xcb_flush());
        assert!(lib.has_xcb_get_maximum_request_length());
        assert!(lib.has_xcb_prefetch_maximum_request_length());
        assert!(lib.has_xcb_wait_for_event());
        assert!(lib.has_xcb_poll_for_event());
        assert!(lib.has_xcb_poll_for_queued_event());
        assert!(lib.has_xcb_poll_for_special_event());
        assert!(lib.has_xcb_wait_for_special_event());
        assert!(lib.has_xcb_register_for_special_xge());
        assert!(lib.has_xcb_unregister_for_special_event());
        assert!(lib.has_xcb_request_check());
        assert!(lib.has_xcb_discard_reply());
        assert!(lib.has_xcb_discard_reply64());
        assert!(lib.has_xcb_get_extension_data());
        assert!(lib.has_xcb_prefetch_extension_data());
        assert!(lib.has_xcb_get_setup());
        assert!(lib.has_xcb_get_file_descriptor());
        assert!(lib.has_xcb_connection_has_error());
        assert!(lib.has_xcb_connect_to_fd());
        assert!(lib.has_xcb_disconnect());
        assert!(lib.has_xcb_parse_display());
        assert!(lib.has_xcb_connect());
        assert!(lib.has_xcb_connect_to_display_with_auth_info());
        assert!(lib.has_xcb_generate_id());
        assert!(lib.has_xcb_total_read());
        assert!(lib.has_xcb_total_written());
    }
}
