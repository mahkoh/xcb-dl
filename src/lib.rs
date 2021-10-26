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
