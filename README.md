# xcb-dl

[![crates.io](https://img.shields.io/crates/v/xcb-dl.svg)](http://crates.io/crates/xcb-dl)
[![docs.rs](https://docs.rs/xcb-dl/badge.svg)](http://docs.rs/xcb-dl)

This rust crate contains bindings for libxcb, libxcb-xinput, etc. Symbols are loaded
dynamically at runtime, linking at compile time is not supported.

## Hacking

Almost all code in this crate is auto-generated using the [generate.py](generate.py) python
script. Run [generate-all](generate-all) to re-generate all code.

generate.py takes as input an xml file from the [xcbproto] project. For consistency,
these files are vendored in the [xcbproto directory](xcbproto). The current version
is 1.14.1.

The python script a heavily modified version of the [c_client.py] script in the libxcb
source code.

[c_client.py]: https://gitlab.freedesktop.org/xorg/lib/libxcb/-/blob/master/src/c_client.py
[xcbproto]: https://gitlab.freedesktop.org/xorg/proto/xcbproto

## License

X11
