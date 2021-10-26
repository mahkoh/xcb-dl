//! This crate contains bindings for libxcb, libxcb-xinput, etc.
//! Symbols are loaded dynamically at runtime; linking at compile time is not supported.
//!
//! # Features
//!
//! Only bindings for libxcb are available by default.
//! Enable the appropriately named features to enable bindings for libxcb-xinput etc.
//!
//! # Handling Missing Symbols
//!
//! Symbols are loaded lazily when they are accessed.
//! If this fails, we panic.
//! Enable the `has_symbol` feature to handle missing symbols more gracefully.
//! This enables methods of the form `has_X` that return whether a symbol can be loaded.
//!
//! ```
//! # unsafe {
//! let lib = xcb_dl::Xcb::load().unwrap();
//! if !lib.has_xcb_total_written() {
//!     eprintln!("Cannot monitor total number of bytes written.");
//! }
//! # }
//! ```
//!
//! # Constructing Structs
//!
//! Future versions of this crate might add new fields to structs or remove padding fields.
//! This is not considered a breaking change.
//! Construct objects using `Default::default()` to ensure that your code remains valid:
//!
//! ```
//! let format = xcb_dl::ffi::xcb_format_t {
//!     depth: 0,
//!     bits_per_pixel: 0,
//!     scanline_pad: 0,
//!     ..Default::default()
//! };
//! ```
//!
//! # Semantic Documentation
//!
//! This crate contains almost no semantic documentation.
//! Documentation of the X core protocol and extensions is available at [freedesktop] and [x.org].
//! Documentation of the libxcb functions that are not auto-generated from the protocols is available
//!     in the [libxcb] repository. Check `src/xcb.h` and `src/xcbext.h`.
//!
//! [freedesktop]:  https://gitlab.freedesktop.org/xorg/proto/xorgproto
//! [x.org]: https://www.x.org/releases/X11R7.7/doc/
//! [libxcb]: https://gitlab.freedesktop.org/xorg/lib/libxcb
//!
//! # libxcb Architecture
//!
//! This sections provides a brief overview of the auto-generated functions in libxcb.
//! Consult the [libxcb][libxcb2] documentation for more details.
//!
//! [libxcb2]: https://xcb.freedesktop.org/
//!
//! The terminology in this section is not the official libxcb terminology.
//!
//! libxcb sends **requests** to and receives **messages** from the X server.
//! There are three types of messages:
//!
//! - **value replies**: A success message sent in response to a request.
//! - **error replies**: An error sent in response to a request.
//! - **events**: An event such as a key press.
//!
//! libxcb maintains two queues for incoming messages:
//!
//! - the **event queue**: Contains messages that can be retrieved via `xcb_poll_for_event` etc.
//! - the **reply queue**: Contains messages that can only be retrieved by the code that sent the request.
//!
//! Events are always placed in the event queue.
//!
//! Value replies are placed in the reply queue unless the code that sent the request specified that
//!     the reply should be discarded.
//! In this case the reply is discarded.
//!
//! Error replies are placed in the reply queue unless the code that sent the request specified that
//!     the reply should be discarded.
//! **In this case the error reply is placed in the event queue.**
//!
//! There are two types of requests:
//!
//! - **void requests**: These do not generate value replies but can generate error replies.
//! - **value requests**: These can generate either value replies or error replies.
//!
//! By default, replies to void requests are discarded.
//! That is, error replies are placed in the event queue.
//! For every void request, libxcb provides a function with the suffix `_checked` that causes error
//!     replies to be placed in the reply queue.
//!
//! By default, replies to value requests are not discarded.
//! That is, both value replies and error replies are placed in the reply queue.
//! For every value request, libxcb provides a function with the suffix `_unchecked` that causes
//!     value replies to be discarded and error replies to the placed in the event queue.
//!
//! Messages placed in the reply queue must be handled by the user.
//! Otherwise they will never be discarded.
//!
//! Messages in the reply queue can be discarded by calling `xcb_discard_reply`.
//!
//! For void requests, error replies can be retrieved by calling `xcb_request_check`.
//!
//! For every value request, libxcb provides a function with the suffix `_reply` that can
//!     be used to retrieve the reply if it was placed in the reply queue.
//!
//! ## Memory Management
//!
//! libxcb never takes ownership of memory passed to it.
//!
//! The functions that return messages pass ownership of the message to the caller.
//! These messages can be freed with `libc::free`.
//!
//! ## Non-homogeneous Requests
//!
//! Some requests contain variably sized data and possibly non-homogeneous arrays.
//! To simplify creating such requests, libxcb provides auxiliary functions with the suffix `_aux`.
//! These functions take fixed-sized arguments and internally create a serialized request.
//!
//! ## Non-homogeneous Replies
//!
//! Some replies contain variably sized data and possibly non-homogeneous arrays.
//!
//! The data structures of libxcb do not contain fields for this data.
//! Instead, libxcb provides accessor functions to retrieve pointers to these fields.
//!
//! If the fields are non-homogeneous arrays, these accessor functions return iterators.
//! The pointer in these iterators can be advanced by calling an appropriately named `_next` function.
//!
//! If the fields are homogeneous arrays, libxcb provides `_length` functions that return the number
//!     of elements in the array.
//!
//! In some cases these accessor functions return `*mut c_void`.
//! In these cases libxcb usually provides an `_unpack` function that deserializes the data into
//!     a struct.
#![allow(
    non_camel_case_types,
    non_snake_case,
    clippy::missing_safety_doc,
    clippy::type_complexity,
    clippy::too_many_arguments
)]

pub use libs::*;

#[macro_use]
mod macros;
mod headers;
mod lazy;
mod libs;

pub mod ffi {
    pub use crate::headers::*;
    use std::fmt::{Debug, Formatter};

    impl Debug for xcb_client_message_data_t {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            unsafe {
                f.debug_struct("xcb_client_message_data_t")
                    .field("data8", &self.data8)
                    .finish()
            }
        }
    }

    #[cfg(feature = "xcb_xinput_types")]
    impl Debug for xcb_input_event_for_send_t {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            unsafe {
                f.debug_struct("xcb_input_event_for_send_t")
                    .field("response_type", &self.event_header.response_type)
                    .finish_non_exhaustive()
            }
        }
    }

    #[cfg(feature = "xcb_xkb_types")]
    impl Debug for xcb_xkb_action_t {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            unsafe {
                f.debug_struct("xcb_xkb_action_t")
                    .field("type", &self.type_)
                    .finish_non_exhaustive()
            }
        }
    }

    #[cfg(feature = "xcb_xkb_types")]
    impl Debug for xcb_xkb_behavior_t {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            unsafe {
                f.debug_struct("xcb_xkb_behavior_t")
                    .field("type", &self.type_)
                    .finish_non_exhaustive()
            }
        }
    }

    #[cfg(feature = "xcb_randr_types")]
    impl Debug for xcb_randr_notify_data_t {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("xcb_randr_notify_data_t")
                .finish_non_exhaustive()
        }
    }
}
