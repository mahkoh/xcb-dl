#![allow(non_camel_case_types)]

pub use headers::{ext::*, xcb::*};
pub use libs::xcb::Xcb;

#[macro_use]
mod macros;
mod headers;
mod lazy;
mod libs;
