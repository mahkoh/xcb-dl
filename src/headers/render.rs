// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The name of the `Render` extension.
pub const XCB_RENDER_NAME: &[u8] = b"RENDER";

/// The name of the `Render` extension.
pub const XCB_RENDER_NAME_STR: &str = "RENDER";

/// The `Render::PictType` enum.
///
/// This enum has the following variants:
///
/// - [`Render::PictType::Indexed`](XCB_RENDER_PICT_TYPE_INDEXED)
/// - [`Render::PictType::Direct`](XCB_RENDER_PICT_TYPE_DIRECT)
pub type xcb_render_pict_type_t = u32;
/// The `Render::PictType::Indexed` enum variant.
///
/// This is a variant of [`xcb_render_pict_type_t`].
pub const XCB_RENDER_PICT_TYPE_INDEXED: xcb_render_pict_type_t = 0;
/// The `Render::PictType::Direct` enum variant.
///
/// This is a variant of [`xcb_render_pict_type_t`].
pub const XCB_RENDER_PICT_TYPE_DIRECT: xcb_render_pict_type_t = 1;

/// The `Render::Picture` enum.
///
/// This enum has the following variants:
///
/// - [`Render::Picture::None`](XCB_RENDER_PICTURE_NONE)
pub type xcb_render_picture_enum_t = u32;
/// The `Render::Picture::None` enum variant.
///
/// This is a variant of [`xcb_render_picture_enum_t`].
pub const XCB_RENDER_PICTURE_NONE: xcb_render_picture_enum_t = 0;

/// The `Render::PictOp` enum.
///
/// This enum has the following variants:
///
/// - [`Render::PictOp::Clear`](XCB_RENDER_PICT_OP_CLEAR)
/// - [`Render::PictOp::Src`](XCB_RENDER_PICT_OP_SRC)
/// - [`Render::PictOp::Dst`](XCB_RENDER_PICT_OP_DST)
/// - [`Render::PictOp::Over`](XCB_RENDER_PICT_OP_OVER)
/// - [`Render::PictOp::OverReverse`](XCB_RENDER_PICT_OP_OVER_REVERSE)
/// - [`Render::PictOp::In`](XCB_RENDER_PICT_OP_IN)
/// - [`Render::PictOp::InReverse`](XCB_RENDER_PICT_OP_IN_REVERSE)
/// - [`Render::PictOp::Out`](XCB_RENDER_PICT_OP_OUT)
/// - [`Render::PictOp::OutReverse`](XCB_RENDER_PICT_OP_OUT_REVERSE)
/// - [`Render::PictOp::Atop`](XCB_RENDER_PICT_OP_ATOP)
/// - [`Render::PictOp::AtopReverse`](XCB_RENDER_PICT_OP_ATOP_REVERSE)
/// - [`Render::PictOp::Xor`](XCB_RENDER_PICT_OP_XOR)
/// - [`Render::PictOp::Add`](XCB_RENDER_PICT_OP_ADD)
/// - [`Render::PictOp::Saturate`](XCB_RENDER_PICT_OP_SATURATE)
/// - [`Render::PictOp::DisjointClear`](XCB_RENDER_PICT_OP_DISJOINT_CLEAR)
/// - [`Render::PictOp::DisjointSrc`](XCB_RENDER_PICT_OP_DISJOINT_SRC)
/// - [`Render::PictOp::DisjointDst`](XCB_RENDER_PICT_OP_DISJOINT_DST)
/// - [`Render::PictOp::DisjointOver`](XCB_RENDER_PICT_OP_DISJOINT_OVER)
/// - [`Render::PictOp::DisjointOverReverse`](XCB_RENDER_PICT_OP_DISJOINT_OVER_REVERSE)
/// - [`Render::PictOp::DisjointIn`](XCB_RENDER_PICT_OP_DISJOINT_IN)
/// - [`Render::PictOp::DisjointInReverse`](XCB_RENDER_PICT_OP_DISJOINT_IN_REVERSE)
/// - [`Render::PictOp::DisjointOut`](XCB_RENDER_PICT_OP_DISJOINT_OUT)
/// - [`Render::PictOp::DisjointOutReverse`](XCB_RENDER_PICT_OP_DISJOINT_OUT_REVERSE)
/// - [`Render::PictOp::DisjointAtop`](XCB_RENDER_PICT_OP_DISJOINT_ATOP)
/// - [`Render::PictOp::DisjointAtopReverse`](XCB_RENDER_PICT_OP_DISJOINT_ATOP_REVERSE)
/// - [`Render::PictOp::DisjointXor`](XCB_RENDER_PICT_OP_DISJOINT_XOR)
/// - [`Render::PictOp::ConjointClear`](XCB_RENDER_PICT_OP_CONJOINT_CLEAR)
/// - [`Render::PictOp::ConjointSrc`](XCB_RENDER_PICT_OP_CONJOINT_SRC)
/// - [`Render::PictOp::ConjointDst`](XCB_RENDER_PICT_OP_CONJOINT_DST)
/// - [`Render::PictOp::ConjointOver`](XCB_RENDER_PICT_OP_CONJOINT_OVER)
/// - [`Render::PictOp::ConjointOverReverse`](XCB_RENDER_PICT_OP_CONJOINT_OVER_REVERSE)
/// - [`Render::PictOp::ConjointIn`](XCB_RENDER_PICT_OP_CONJOINT_IN)
/// - [`Render::PictOp::ConjointInReverse`](XCB_RENDER_PICT_OP_CONJOINT_IN_REVERSE)
/// - [`Render::PictOp::ConjointOut`](XCB_RENDER_PICT_OP_CONJOINT_OUT)
/// - [`Render::PictOp::ConjointOutReverse`](XCB_RENDER_PICT_OP_CONJOINT_OUT_REVERSE)
/// - [`Render::PictOp::ConjointAtop`](XCB_RENDER_PICT_OP_CONJOINT_ATOP)
/// - [`Render::PictOp::ConjointAtopReverse`](XCB_RENDER_PICT_OP_CONJOINT_ATOP_REVERSE)
/// - [`Render::PictOp::ConjointXor`](XCB_RENDER_PICT_OP_CONJOINT_XOR)
/// - [`Render::PictOp::Multiply`](XCB_RENDER_PICT_OP_MULTIPLY)
/// - [`Render::PictOp::Screen`](XCB_RENDER_PICT_OP_SCREEN)
/// - [`Render::PictOp::Overlay`](XCB_RENDER_PICT_OP_OVERLAY)
/// - [`Render::PictOp::Darken`](XCB_RENDER_PICT_OP_DARKEN)
/// - [`Render::PictOp::Lighten`](XCB_RENDER_PICT_OP_LIGHTEN)
/// - [`Render::PictOp::ColorDodge`](XCB_RENDER_PICT_OP_COLOR_DODGE)
/// - [`Render::PictOp::ColorBurn`](XCB_RENDER_PICT_OP_COLOR_BURN)
/// - [`Render::PictOp::HardLight`](XCB_RENDER_PICT_OP_HARD_LIGHT)
/// - [`Render::PictOp::SoftLight`](XCB_RENDER_PICT_OP_SOFT_LIGHT)
/// - [`Render::PictOp::Difference`](XCB_RENDER_PICT_OP_DIFFERENCE)
/// - [`Render::PictOp::Exclusion`](XCB_RENDER_PICT_OP_EXCLUSION)
/// - [`Render::PictOp::HSLHue`](XCB_RENDER_PICT_OP_HSL_HUE)
/// - [`Render::PictOp::HSLSaturation`](XCB_RENDER_PICT_OP_HSL_SATURATION)
/// - [`Render::PictOp::HSLColor`](XCB_RENDER_PICT_OP_HSL_COLOR)
/// - [`Render::PictOp::HSLLuminosity`](XCB_RENDER_PICT_OP_HSL_LUMINOSITY)
pub type xcb_render_pict_op_t = u32;
/// The `Render::PictOp::Clear` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_CLEAR: xcb_render_pict_op_t = 0;
/// The `Render::PictOp::Src` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_SRC: xcb_render_pict_op_t = 1;
/// The `Render::PictOp::Dst` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_DST: xcb_render_pict_op_t = 2;
/// The `Render::PictOp::Over` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_OVER: xcb_render_pict_op_t = 3;
/// The `Render::PictOp::OverReverse` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_OVER_REVERSE: xcb_render_pict_op_t = 4;
/// The `Render::PictOp::In` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_IN: xcb_render_pict_op_t = 5;
/// The `Render::PictOp::InReverse` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_IN_REVERSE: xcb_render_pict_op_t = 6;
/// The `Render::PictOp::Out` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_OUT: xcb_render_pict_op_t = 7;
/// The `Render::PictOp::OutReverse` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_OUT_REVERSE: xcb_render_pict_op_t = 8;
/// The `Render::PictOp::Atop` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_ATOP: xcb_render_pict_op_t = 9;
/// The `Render::PictOp::AtopReverse` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_ATOP_REVERSE: xcb_render_pict_op_t = 10;
/// The `Render::PictOp::Xor` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_XOR: xcb_render_pict_op_t = 11;
/// The `Render::PictOp::Add` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_ADD: xcb_render_pict_op_t = 12;
/// The `Render::PictOp::Saturate` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_SATURATE: xcb_render_pict_op_t = 13;
/// The `Render::PictOp::DisjointClear` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_DISJOINT_CLEAR: xcb_render_pict_op_t = 16;
/// The `Render::PictOp::DisjointSrc` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_DISJOINT_SRC: xcb_render_pict_op_t = 17;
/// The `Render::PictOp::DisjointDst` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_DISJOINT_DST: xcb_render_pict_op_t = 18;
/// The `Render::PictOp::DisjointOver` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_DISJOINT_OVER: xcb_render_pict_op_t = 19;
/// The `Render::PictOp::DisjointOverReverse` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_DISJOINT_OVER_REVERSE: xcb_render_pict_op_t = 20;
/// The `Render::PictOp::DisjointIn` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_DISJOINT_IN: xcb_render_pict_op_t = 21;
/// The `Render::PictOp::DisjointInReverse` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_DISJOINT_IN_REVERSE: xcb_render_pict_op_t = 22;
/// The `Render::PictOp::DisjointOut` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_DISJOINT_OUT: xcb_render_pict_op_t = 23;
/// The `Render::PictOp::DisjointOutReverse` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_DISJOINT_OUT_REVERSE: xcb_render_pict_op_t = 24;
/// The `Render::PictOp::DisjointAtop` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_DISJOINT_ATOP: xcb_render_pict_op_t = 25;
/// The `Render::PictOp::DisjointAtopReverse` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_DISJOINT_ATOP_REVERSE: xcb_render_pict_op_t = 26;
/// The `Render::PictOp::DisjointXor` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_DISJOINT_XOR: xcb_render_pict_op_t = 27;
/// The `Render::PictOp::ConjointClear` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_CONJOINT_CLEAR: xcb_render_pict_op_t = 32;
/// The `Render::PictOp::ConjointSrc` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_CONJOINT_SRC: xcb_render_pict_op_t = 33;
/// The `Render::PictOp::ConjointDst` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_CONJOINT_DST: xcb_render_pict_op_t = 34;
/// The `Render::PictOp::ConjointOver` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_CONJOINT_OVER: xcb_render_pict_op_t = 35;
/// The `Render::PictOp::ConjointOverReverse` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_CONJOINT_OVER_REVERSE: xcb_render_pict_op_t = 36;
/// The `Render::PictOp::ConjointIn` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_CONJOINT_IN: xcb_render_pict_op_t = 37;
/// The `Render::PictOp::ConjointInReverse` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_CONJOINT_IN_REVERSE: xcb_render_pict_op_t = 38;
/// The `Render::PictOp::ConjointOut` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_CONJOINT_OUT: xcb_render_pict_op_t = 39;
/// The `Render::PictOp::ConjointOutReverse` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_CONJOINT_OUT_REVERSE: xcb_render_pict_op_t = 40;
/// The `Render::PictOp::ConjointAtop` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_CONJOINT_ATOP: xcb_render_pict_op_t = 41;
/// The `Render::PictOp::ConjointAtopReverse` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_CONJOINT_ATOP_REVERSE: xcb_render_pict_op_t = 42;
/// The `Render::PictOp::ConjointXor` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_CONJOINT_XOR: xcb_render_pict_op_t = 43;
/// The `Render::PictOp::Multiply` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_MULTIPLY: xcb_render_pict_op_t = 48;
/// The `Render::PictOp::Screen` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_SCREEN: xcb_render_pict_op_t = 49;
/// The `Render::PictOp::Overlay` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_OVERLAY: xcb_render_pict_op_t = 50;
/// The `Render::PictOp::Darken` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_DARKEN: xcb_render_pict_op_t = 51;
/// The `Render::PictOp::Lighten` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_LIGHTEN: xcb_render_pict_op_t = 52;
/// The `Render::PictOp::ColorDodge` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_COLOR_DODGE: xcb_render_pict_op_t = 53;
/// The `Render::PictOp::ColorBurn` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_COLOR_BURN: xcb_render_pict_op_t = 54;
/// The `Render::PictOp::HardLight` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_HARD_LIGHT: xcb_render_pict_op_t = 55;
/// The `Render::PictOp::SoftLight` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_SOFT_LIGHT: xcb_render_pict_op_t = 56;
/// The `Render::PictOp::Difference` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_DIFFERENCE: xcb_render_pict_op_t = 57;
/// The `Render::PictOp::Exclusion` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_EXCLUSION: xcb_render_pict_op_t = 58;
/// The `Render::PictOp::HSLHue` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_HSL_HUE: xcb_render_pict_op_t = 59;
/// The `Render::PictOp::HSLSaturation` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_HSL_SATURATION: xcb_render_pict_op_t = 60;
/// The `Render::PictOp::HSLColor` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_HSL_COLOR: xcb_render_pict_op_t = 61;
/// The `Render::PictOp::HSLLuminosity` enum variant.
///
/// This is a variant of [`xcb_render_pict_op_t`].
pub const XCB_RENDER_PICT_OP_HSL_LUMINOSITY: xcb_render_pict_op_t = 62;

/// The `Render::PolyEdge` enum.
///
/// This enum has the following variants:
///
/// - [`Render::PolyEdge::Sharp`](XCB_RENDER_POLY_EDGE_SHARP)
/// - [`Render::PolyEdge::Smooth`](XCB_RENDER_POLY_EDGE_SMOOTH)
pub type xcb_render_poly_edge_t = u32;
/// The `Render::PolyEdge::Sharp` enum variant.
///
/// This is a variant of [`xcb_render_poly_edge_t`].
pub const XCB_RENDER_POLY_EDGE_SHARP: xcb_render_poly_edge_t = 0;
/// The `Render::PolyEdge::Smooth` enum variant.
///
/// This is a variant of [`xcb_render_poly_edge_t`].
pub const XCB_RENDER_POLY_EDGE_SMOOTH: xcb_render_poly_edge_t = 1;

/// The `Render::PolyMode` enum.
///
/// This enum has the following variants:
///
/// - [`Render::PolyMode::Precise`](XCB_RENDER_POLY_MODE_PRECISE)
/// - [`Render::PolyMode::Imprecise`](XCB_RENDER_POLY_MODE_IMPRECISE)
pub type xcb_render_poly_mode_t = u32;
/// The `Render::PolyMode::Precise` enum variant.
///
/// This is a variant of [`xcb_render_poly_mode_t`].
pub const XCB_RENDER_POLY_MODE_PRECISE: xcb_render_poly_mode_t = 0;
/// The `Render::PolyMode::Imprecise` enum variant.
///
/// This is a variant of [`xcb_render_poly_mode_t`].
pub const XCB_RENDER_POLY_MODE_IMPRECISE: xcb_render_poly_mode_t = 1;

/// The `Render::CP` enum.
///
/// This enum has the following variants:
///
/// - [`Render::CP::Repeat`](XCB_RENDER_CP_REPEAT)
/// - [`Render::CP::AlphaMap`](XCB_RENDER_CP_ALPHA_MAP)
/// - [`Render::CP::AlphaXOrigin`](XCB_RENDER_CP_ALPHA_X_ORIGIN)
/// - [`Render::CP::AlphaYOrigin`](XCB_RENDER_CP_ALPHA_Y_ORIGIN)
/// - [`Render::CP::ClipXOrigin`](XCB_RENDER_CP_CLIP_X_ORIGIN)
/// - [`Render::CP::ClipYOrigin`](XCB_RENDER_CP_CLIP_Y_ORIGIN)
/// - [`Render::CP::ClipMask`](XCB_RENDER_CP_CLIP_MASK)
/// - [`Render::CP::GraphicsExposure`](XCB_RENDER_CP_GRAPHICS_EXPOSURE)
/// - [`Render::CP::SubwindowMode`](XCB_RENDER_CP_SUBWINDOW_MODE)
/// - [`Render::CP::PolyEdge`](XCB_RENDER_CP_POLY_EDGE)
/// - [`Render::CP::PolyMode`](XCB_RENDER_CP_POLY_MODE)
/// - [`Render::CP::Dither`](XCB_RENDER_CP_DITHER)
/// - [`Render::CP::ComponentAlpha`](XCB_RENDER_CP_COMPONENT_ALPHA)
pub type xcb_render_cp_t = u32;
/// The `Render::CP::Repeat` enum variant.
///
/// This is a variant of [`xcb_render_cp_t`].
pub const XCB_RENDER_CP_REPEAT: xcb_render_cp_t = 1;
/// The `Render::CP::AlphaMap` enum variant.
///
/// This is a variant of [`xcb_render_cp_t`].
pub const XCB_RENDER_CP_ALPHA_MAP: xcb_render_cp_t = 2;
/// The `Render::CP::AlphaXOrigin` enum variant.
///
/// This is a variant of [`xcb_render_cp_t`].
pub const XCB_RENDER_CP_ALPHA_X_ORIGIN: xcb_render_cp_t = 4;
/// The `Render::CP::AlphaYOrigin` enum variant.
///
/// This is a variant of [`xcb_render_cp_t`].
pub const XCB_RENDER_CP_ALPHA_Y_ORIGIN: xcb_render_cp_t = 8;
/// The `Render::CP::ClipXOrigin` enum variant.
///
/// This is a variant of [`xcb_render_cp_t`].
pub const XCB_RENDER_CP_CLIP_X_ORIGIN: xcb_render_cp_t = 16;
/// The `Render::CP::ClipYOrigin` enum variant.
///
/// This is a variant of [`xcb_render_cp_t`].
pub const XCB_RENDER_CP_CLIP_Y_ORIGIN: xcb_render_cp_t = 32;
/// The `Render::CP::ClipMask` enum variant.
///
/// This is a variant of [`xcb_render_cp_t`].
pub const XCB_RENDER_CP_CLIP_MASK: xcb_render_cp_t = 64;
/// The `Render::CP::GraphicsExposure` enum variant.
///
/// This is a variant of [`xcb_render_cp_t`].
pub const XCB_RENDER_CP_GRAPHICS_EXPOSURE: xcb_render_cp_t = 128;
/// The `Render::CP::SubwindowMode` enum variant.
///
/// This is a variant of [`xcb_render_cp_t`].
pub const XCB_RENDER_CP_SUBWINDOW_MODE: xcb_render_cp_t = 256;
/// The `Render::CP::PolyEdge` enum variant.
///
/// This is a variant of [`xcb_render_cp_t`].
pub const XCB_RENDER_CP_POLY_EDGE: xcb_render_cp_t = 512;
/// The `Render::CP::PolyMode` enum variant.
///
/// This is a variant of [`xcb_render_cp_t`].
pub const XCB_RENDER_CP_POLY_MODE: xcb_render_cp_t = 1024;
/// The `Render::CP::Dither` enum variant.
///
/// This is a variant of [`xcb_render_cp_t`].
pub const XCB_RENDER_CP_DITHER: xcb_render_cp_t = 2048;
/// The `Render::CP::ComponentAlpha` enum variant.
///
/// This is a variant of [`xcb_render_cp_t`].
pub const XCB_RENDER_CP_COMPONENT_ALPHA: xcb_render_cp_t = 4096;

/// The `Render::SubPixel` enum.
///
/// This enum has the following variants:
///
/// - [`Render::SubPixel::Unknown`](XCB_RENDER_SUB_PIXEL_UNKNOWN)
/// - [`Render::SubPixel::HorizontalRGB`](XCB_RENDER_SUB_PIXEL_HORIZONTAL_RGB)
/// - [`Render::SubPixel::HorizontalBGR`](XCB_RENDER_SUB_PIXEL_HORIZONTAL_BGR)
/// - [`Render::SubPixel::VerticalRGB`](XCB_RENDER_SUB_PIXEL_VERTICAL_RGB)
/// - [`Render::SubPixel::VerticalBGR`](XCB_RENDER_SUB_PIXEL_VERTICAL_BGR)
/// - [`Render::SubPixel::None`](XCB_RENDER_SUB_PIXEL_NONE)
pub type xcb_render_sub_pixel_t = u32;
/// The `Render::SubPixel::Unknown` enum variant.
///
/// This is a variant of [`xcb_render_sub_pixel_t`].
pub const XCB_RENDER_SUB_PIXEL_UNKNOWN: xcb_render_sub_pixel_t = 0;
/// The `Render::SubPixel::HorizontalRGB` enum variant.
///
/// This is a variant of [`xcb_render_sub_pixel_t`].
pub const XCB_RENDER_SUB_PIXEL_HORIZONTAL_RGB: xcb_render_sub_pixel_t = 1;
/// The `Render::SubPixel::HorizontalBGR` enum variant.
///
/// This is a variant of [`xcb_render_sub_pixel_t`].
pub const XCB_RENDER_SUB_PIXEL_HORIZONTAL_BGR: xcb_render_sub_pixel_t = 2;
/// The `Render::SubPixel::VerticalRGB` enum variant.
///
/// This is a variant of [`xcb_render_sub_pixel_t`].
pub const XCB_RENDER_SUB_PIXEL_VERTICAL_RGB: xcb_render_sub_pixel_t = 3;
/// The `Render::SubPixel::VerticalBGR` enum variant.
///
/// This is a variant of [`xcb_render_sub_pixel_t`].
pub const XCB_RENDER_SUB_PIXEL_VERTICAL_BGR: xcb_render_sub_pixel_t = 4;
/// The `Render::SubPixel::None` enum variant.
///
/// This is a variant of [`xcb_render_sub_pixel_t`].
pub const XCB_RENDER_SUB_PIXEL_NONE: xcb_render_sub_pixel_t = 5;

/// The `Render::Repeat` enum.
///
/// This enum has the following variants:
///
/// - [`Render::Repeat::None`](XCB_RENDER_REPEAT_NONE)
/// - [`Render::Repeat::Normal`](XCB_RENDER_REPEAT_NORMAL)
/// - [`Render::Repeat::Pad`](XCB_RENDER_REPEAT_PAD)
/// - [`Render::Repeat::Reflect`](XCB_RENDER_REPEAT_REFLECT)
pub type xcb_render_repeat_t = u32;
/// The `Render::Repeat::None` enum variant.
///
/// This is a variant of [`xcb_render_repeat_t`].
pub const XCB_RENDER_REPEAT_NONE: xcb_render_repeat_t = 0;
/// The `Render::Repeat::Normal` enum variant.
///
/// This is a variant of [`xcb_render_repeat_t`].
pub const XCB_RENDER_REPEAT_NORMAL: xcb_render_repeat_t = 1;
/// The `Render::Repeat::Pad` enum variant.
///
/// This is a variant of [`xcb_render_repeat_t`].
pub const XCB_RENDER_REPEAT_PAD: xcb_render_repeat_t = 2;
/// The `Render::Repeat::Reflect` enum variant.
///
/// This is a variant of [`xcb_render_repeat_t`].
pub const XCB_RENDER_REPEAT_REFLECT: xcb_render_repeat_t = 3;

/// The `Render::GLYPH` type.
pub type xcb_render_glyph_t = u32;

/// An iterator over `Render::GLYPH` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_glyph_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_render_glyph_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_render_glyph_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::GLYPHSET` type.
pub type xcb_render_glyphset_t = u32;

/// An iterator over `Render::GLYPHSET` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_glyphset_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_render_glyphset_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_render_glyphset_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::PICTURE` type.
pub type xcb_render_picture_t = u32;

/// An iterator over `Render::PICTURE` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_picture_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_render_picture_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_render_picture_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::PICTFORMAT` type.
pub type xcb_render_pictformat_t = u32;

/// An iterator over `Render::PICTFORMAT` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pictformat_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_render_pictformat_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_render_pictformat_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::FIXED` type.
pub type xcb_render_fixed_t = i32;

/// An iterator over `Render::FIXED` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_fixed_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_render_fixed_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_render_fixed_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::PictFormat` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_render_pict_format_error_t`].
pub const XCB_RENDER_PICT_FORMAT: u8 = 0i32 as u8;

/// The `Render::PictFormat` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pict_format_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_render_pict_format_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::Picture` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_render_picture_error_t`].
pub const XCB_RENDER_PICTURE: u8 = 1i32 as u8;

/// The `Render::Picture` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_picture_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_render_picture_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::PictOp` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_render_pict_op_error_t`].
pub const XCB_RENDER_PICT_OP: u8 = 2i32 as u8;

/// The `Render::PictOp` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pict_op_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_render_pict_op_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::GlyphSet` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_render_glyph_set_error_t`].
pub const XCB_RENDER_GLYPH_SET: u8 = 3i32 as u8;

/// The `Render::GlyphSet` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_glyph_set_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_render_glyph_set_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::Glyph` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_render_glyph_error_t`].
pub const XCB_RENDER_GLYPH: u8 = 4i32 as u8;

/// The `Render::Glyph` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_glyph_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_render_glyph_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::DIRECTFORMAT` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_directformat_t {
    pub red_shift: u16,
    pub red_mask: u16,
    pub green_shift: u16,
    pub green_mask: u16,
    pub blue_shift: u16,
    pub blue_mask: u16,
    pub alpha_shift: u16,
    pub alpha_mask: u16,
}

impl Default for xcb_render_directformat_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Render::DIRECTFORMAT` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_directformat_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_render_directformat_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_render_directformat_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::PICTFORMINFO` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pictforminfo_t {
    pub id: xcb_render_pictformat_t,
    pub type_: u8,
    pub depth: u8,
    pub pad0: [u8; 2],
    pub direct: xcb_render_directformat_t,
    pub colormap: xcb_colormap_t,
}

impl Default for xcb_render_pictforminfo_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Render::PICTFORMINFO` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pictforminfo_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_render_pictforminfo_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_render_pictforminfo_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::PICTVISUAL` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pictvisual_t {
    pub visual: xcb_visualid_t,
    pub format: xcb_render_pictformat_t,
}

impl Default for xcb_render_pictvisual_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Render::PICTVISUAL` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pictvisual_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_render_pictvisual_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_render_pictvisual_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::PICTDEPTH` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `visuals`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pictdepth_t {
    pub depth: u8,
    pub pad0: u8,
    pub num_visuals: u16,
    pub pad1: [u8; 4],
}

impl Default for xcb_render_pictdepth_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Render::PICTDEPTH` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pictdepth_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_render_pictdepth_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_render_pictdepth_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::PICTSCREEN` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `depths`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pictscreen_t {
    pub num_depths: u32,
    pub fallback: xcb_render_pictformat_t,
}

impl Default for xcb_render_pictscreen_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Render::PICTSCREEN` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pictscreen_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_render_pictscreen_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_render_pictscreen_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::INDEXVALUE` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_indexvalue_t {
    pub pixel: u32,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub alpha: u16,
}

impl Default for xcb_render_indexvalue_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Render::INDEXVALUE` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_indexvalue_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_render_indexvalue_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_render_indexvalue_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::COLOR` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_color_t {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub alpha: u16,
}

impl Default for xcb_render_color_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Render::COLOR` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_color_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_render_color_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_render_color_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::POINTFIX` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pointfix_t {
    pub x: xcb_render_fixed_t,
    pub y: xcb_render_fixed_t,
}

impl Default for xcb_render_pointfix_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Render::POINTFIX` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_pointfix_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_render_pointfix_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_render_pointfix_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::LINEFIX` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_linefix_t {
    pub p1: xcb_render_pointfix_t,
    pub p2: xcb_render_pointfix_t,
}

impl Default for xcb_render_linefix_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Render::LINEFIX` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_linefix_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_render_linefix_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_render_linefix_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::TRIANGLE` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_triangle_t {
    pub p1: xcb_render_pointfix_t,
    pub p2: xcb_render_pointfix_t,
    pub p3: xcb_render_pointfix_t,
}

impl Default for xcb_render_triangle_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Render::TRIANGLE` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_triangle_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_render_triangle_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_render_triangle_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::TRAPEZOID` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_trapezoid_t {
    pub top: xcb_render_fixed_t,
    pub bottom: xcb_render_fixed_t,
    pub left: xcb_render_linefix_t,
    pub right: xcb_render_linefix_t,
}

impl Default for xcb_render_trapezoid_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Render::TRAPEZOID` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_trapezoid_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_render_trapezoid_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_render_trapezoid_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::GLYPHINFO` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_glyphinfo_t {
    pub width: u16,
    pub height: u16,
    pub x: i16,
    pub y: i16,
    pub x_off: i16,
    pub y_off: i16,
}

impl Default for xcb_render_glyphinfo_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Render::GLYPHINFO` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_glyphinfo_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_render_glyphinfo_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_render_glyphinfo_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Render::QueryVersion` request.
///
/// Pass this cookie to [`xcb_render_query_version_reply`] to retrieve the reply.
///
/// [`xcb_render_query_version_reply`]: XcbRender::xcb_render_query_version_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_query_version_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_render_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::QueryVersion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_query_version_request_t`].
pub const XCB_RENDER_QUERY_VERSION: u8 = 0i32 as u8;

/// The `Render::QueryVersion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client_major_version: u32,
    pub client_minor_version: u32,
}

impl Default for xcb_render_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::QueryVersion` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
    pub pad1: [u8; 16],
}

impl Default for xcb_render_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Render::QueryPictFormats` request.
///
/// Pass this cookie to [`xcb_render_query_pict_formats_reply`] to retrieve the reply.
///
/// [`xcb_render_query_pict_formats_reply`]: XcbRender::xcb_render_query_pict_formats_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_query_pict_formats_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_render_query_pict_formats_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::QueryPictFormats` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_query_pict_formats_request_t`].
pub const XCB_RENDER_QUERY_PICT_FORMATS: u8 = 1i32 as u8;

/// The `Render::QueryPictFormats` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_query_pict_formats_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_render_query_pict_formats_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::QueryPictFormats` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `formats`
/// - `screens`
/// - `subpixels`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_query_pict_formats_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_formats: u32,
    pub num_screens: u32,
    pub num_depths: u32,
    pub num_visuals: u32,
    pub num_subpixel: u32,
    pub pad1: [u8; 4],
}

impl Default for xcb_render_query_pict_formats_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Render::QueryPictIndexValues` request.
///
/// Pass this cookie to [`xcb_render_query_pict_index_values_reply`] to retrieve the reply.
///
/// [`xcb_render_query_pict_index_values_reply`]: XcbRender::xcb_render_query_pict_index_values_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_query_pict_index_values_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_render_query_pict_index_values_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::QueryPictIndexValues` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_query_pict_index_values_request_t`].
pub const XCB_RENDER_QUERY_PICT_INDEX_VALUES: u8 = 2i32 as u8;

/// The `Render::QueryPictIndexValues` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_query_pict_index_values_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub format: xcb_render_pictformat_t,
}

impl Default for xcb_render_query_pict_index_values_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::QueryPictIndexValues` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `values`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_query_pict_index_values_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_values: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_render_query_pict_index_values_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::value_list` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_create_picture_value_list_t {
    pub repeat: u32,
    pub alphamap: xcb_render_picture_t,
    pub alphaxorigin: i32,
    pub alphayorigin: i32,
    pub clipxorigin: i32,
    pub clipyorigin: i32,
    pub clipmask: xcb_pixmap_t,
    pub graphicsexposure: u32,
    pub subwindowmode: u32,
    pub polyedge: u32,
    pub polymode: u32,
    pub dither: xcb_atom_t,
    pub componentalpha: u32,
}

impl Default for xcb_render_create_picture_value_list_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::CreatePicture` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_create_picture_request_t`].
pub const XCB_RENDER_CREATE_PICTURE: u8 = 4i32 as u8;

/// The `Render::CreatePicture` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `value_list`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_create_picture_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub pid: xcb_render_picture_t,
    pub drawable: xcb_drawable_t,
    pub format: xcb_render_pictformat_t,
    pub value_mask: u32,
}

impl Default for xcb_render_create_picture_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::value_list` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_change_picture_value_list_t {
    pub repeat: u32,
    pub alphamap: xcb_render_picture_t,
    pub alphaxorigin: i32,
    pub alphayorigin: i32,
    pub clipxorigin: i32,
    pub clipyorigin: i32,
    pub clipmask: xcb_pixmap_t,
    pub graphicsexposure: u32,
    pub subwindowmode: u32,
    pub polyedge: u32,
    pub polymode: u32,
    pub dither: xcb_atom_t,
    pub componentalpha: u32,
}

impl Default for xcb_render_change_picture_value_list_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::ChangePicture` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_change_picture_request_t`].
pub const XCB_RENDER_CHANGE_PICTURE: u8 = 5i32 as u8;

/// The `Render::ChangePicture` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `value_list`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_change_picture_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub picture: xcb_render_picture_t,
    pub value_mask: u32,
}

impl Default for xcb_render_change_picture_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::SetPictureClipRectangles` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_set_picture_clip_rectangles_request_t`].
pub const XCB_RENDER_SET_PICTURE_CLIP_RECTANGLES: u8 = 6i32 as u8;

/// The `Render::SetPictureClipRectangles` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `rectangles`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_set_picture_clip_rectangles_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub picture: xcb_render_picture_t,
    pub clip_x_origin: i16,
    pub clip_y_origin: i16,
}

impl Default for xcb_render_set_picture_clip_rectangles_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::FreePicture` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_free_picture_request_t`].
pub const XCB_RENDER_FREE_PICTURE: u8 = 7i32 as u8;

/// The `Render::FreePicture` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_free_picture_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub picture: xcb_render_picture_t,
}

impl Default for xcb_render_free_picture_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::Composite` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_composite_request_t`].
pub const XCB_RENDER_COMPOSITE: u8 = 8i32 as u8;

/// The `Render::Composite` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_composite_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub op: u8,
    pub pad0: [u8; 3],
    pub src: xcb_render_picture_t,
    pub mask: xcb_render_picture_t,
    pub dst: xcb_render_picture_t,
    pub src_x: i16,
    pub src_y: i16,
    pub mask_x: i16,
    pub mask_y: i16,
    pub dst_x: i16,
    pub dst_y: i16,
    pub width: u16,
    pub height: u16,
}

impl Default for xcb_render_composite_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::Trapezoids` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_trapezoids_request_t`].
pub const XCB_RENDER_TRAPEZOIDS: u8 = 10i32 as u8;

/// The `Render::Trapezoids` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `traps`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_trapezoids_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub op: u8,
    pub pad0: [u8; 3],
    pub src: xcb_render_picture_t,
    pub dst: xcb_render_picture_t,
    pub mask_format: xcb_render_pictformat_t,
    pub src_x: i16,
    pub src_y: i16,
}

impl Default for xcb_render_trapezoids_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::Triangles` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_triangles_request_t`].
pub const XCB_RENDER_TRIANGLES: u8 = 11i32 as u8;

/// The `Render::Triangles` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `triangles`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_triangles_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub op: u8,
    pub pad0: [u8; 3],
    pub src: xcb_render_picture_t,
    pub dst: xcb_render_picture_t,
    pub mask_format: xcb_render_pictformat_t,
    pub src_x: i16,
    pub src_y: i16,
}

impl Default for xcb_render_triangles_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::TriStrip` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_tri_strip_request_t`].
pub const XCB_RENDER_TRI_STRIP: u8 = 12i32 as u8;

/// The `Render::TriStrip` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `points`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_tri_strip_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub op: u8,
    pub pad0: [u8; 3],
    pub src: xcb_render_picture_t,
    pub dst: xcb_render_picture_t,
    pub mask_format: xcb_render_pictformat_t,
    pub src_x: i16,
    pub src_y: i16,
}

impl Default for xcb_render_tri_strip_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::TriFan` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_tri_fan_request_t`].
pub const XCB_RENDER_TRI_FAN: u8 = 13i32 as u8;

/// The `Render::TriFan` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `points`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_tri_fan_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub op: u8,
    pub pad0: [u8; 3],
    pub src: xcb_render_picture_t,
    pub dst: xcb_render_picture_t,
    pub mask_format: xcb_render_pictformat_t,
    pub src_x: i16,
    pub src_y: i16,
}

impl Default for xcb_render_tri_fan_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::CreateGlyphSet` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_create_glyph_set_request_t`].
pub const XCB_RENDER_CREATE_GLYPH_SET: u8 = 17i32 as u8;

/// The `Render::CreateGlyphSet` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_create_glyph_set_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub gsid: xcb_render_glyphset_t,
    pub format: xcb_render_pictformat_t,
}

impl Default for xcb_render_create_glyph_set_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::ReferenceGlyphSet` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_reference_glyph_set_request_t`].
pub const XCB_RENDER_REFERENCE_GLYPH_SET: u8 = 18i32 as u8;

/// The `Render::ReferenceGlyphSet` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_reference_glyph_set_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub gsid: xcb_render_glyphset_t,
    pub existing: xcb_render_glyphset_t,
}

impl Default for xcb_render_reference_glyph_set_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::FreeGlyphSet` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_free_glyph_set_request_t`].
pub const XCB_RENDER_FREE_GLYPH_SET: u8 = 19i32 as u8;

/// The `Render::FreeGlyphSet` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_free_glyph_set_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub glyphset: xcb_render_glyphset_t,
}

impl Default for xcb_render_free_glyph_set_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::AddGlyphs` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_add_glyphs_request_t`].
pub const XCB_RENDER_ADD_GLYPHS: u8 = 20i32 as u8;

/// The `Render::AddGlyphs` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `glyphids`
/// - `glyphs`
/// - `data_len`
/// - `data`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_add_glyphs_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub glyphset: xcb_render_glyphset_t,
    pub glyphs_len: u32,
}

impl Default for xcb_render_add_glyphs_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::FreeGlyphs` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_free_glyphs_request_t`].
pub const XCB_RENDER_FREE_GLYPHS: u8 = 22i32 as u8;

/// The `Render::FreeGlyphs` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `glyphs`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_free_glyphs_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub glyphset: xcb_render_glyphset_t,
}

impl Default for xcb_render_free_glyphs_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::CompositeGlyphs8` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_composite_glyphs_8_request_t`].
pub const XCB_RENDER_COMPOSITE_GLYPHS_8: u8 = 23i32 as u8;

/// The `Render::CompositeGlyphs8` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `glyphcmds`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_composite_glyphs_8_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub op: u8,
    pub pad0: [u8; 3],
    pub src: xcb_render_picture_t,
    pub dst: xcb_render_picture_t,
    pub mask_format: xcb_render_pictformat_t,
    pub glyphset: xcb_render_glyphset_t,
    pub src_x: i16,
    pub src_y: i16,
}

impl Default for xcb_render_composite_glyphs_8_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::CompositeGlyphs16` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_composite_glyphs_16_request_t`].
pub const XCB_RENDER_COMPOSITE_GLYPHS_16: u8 = 24i32 as u8;

/// The `Render::CompositeGlyphs16` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `glyphcmds`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_composite_glyphs_16_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub op: u8,
    pub pad0: [u8; 3],
    pub src: xcb_render_picture_t,
    pub dst: xcb_render_picture_t,
    pub mask_format: xcb_render_pictformat_t,
    pub glyphset: xcb_render_glyphset_t,
    pub src_x: i16,
    pub src_y: i16,
}

impl Default for xcb_render_composite_glyphs_16_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::CompositeGlyphs32` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_composite_glyphs_32_request_t`].
pub const XCB_RENDER_COMPOSITE_GLYPHS_32: u8 = 25i32 as u8;

/// The `Render::CompositeGlyphs32` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `glyphcmds`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_composite_glyphs_32_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub op: u8,
    pub pad0: [u8; 3],
    pub src: xcb_render_picture_t,
    pub dst: xcb_render_picture_t,
    pub mask_format: xcb_render_pictformat_t,
    pub glyphset: xcb_render_glyphset_t,
    pub src_x: i16,
    pub src_y: i16,
}

impl Default for xcb_render_composite_glyphs_32_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::FillRectangles` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_fill_rectangles_request_t`].
pub const XCB_RENDER_FILL_RECTANGLES: u8 = 26i32 as u8;

/// The `Render::FillRectangles` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `rects`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_fill_rectangles_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub op: u8,
    pub pad0: [u8; 3],
    pub dst: xcb_render_picture_t,
    pub color: xcb_render_color_t,
}

impl Default for xcb_render_fill_rectangles_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::CreateCursor` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_create_cursor_request_t`].
pub const XCB_RENDER_CREATE_CURSOR: u8 = 27i32 as u8;

/// The `Render::CreateCursor` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_create_cursor_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub cid: xcb_cursor_t,
    pub source: xcb_render_picture_t,
    pub x: u16,
    pub y: u16,
}

impl Default for xcb_render_create_cursor_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::TRANSFORM` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_transform_t {
    pub matrix11: xcb_render_fixed_t,
    pub matrix12: xcb_render_fixed_t,
    pub matrix13: xcb_render_fixed_t,
    pub matrix21: xcb_render_fixed_t,
    pub matrix22: xcb_render_fixed_t,
    pub matrix23: xcb_render_fixed_t,
    pub matrix31: xcb_render_fixed_t,
    pub matrix32: xcb_render_fixed_t,
    pub matrix33: xcb_render_fixed_t,
}

impl Default for xcb_render_transform_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Render::TRANSFORM` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_transform_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_render_transform_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_render_transform_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::SetPictureTransform` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_set_picture_transform_request_t`].
pub const XCB_RENDER_SET_PICTURE_TRANSFORM: u8 = 28i32 as u8;

/// The `Render::SetPictureTransform` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_set_picture_transform_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub picture: xcb_render_picture_t,
    pub transform: xcb_render_transform_t,
}

impl Default for xcb_render_set_picture_transform_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Render::QueryFilters` request.
///
/// Pass this cookie to [`xcb_render_query_filters_reply`] to retrieve the reply.
///
/// [`xcb_render_query_filters_reply`]: XcbRender::xcb_render_query_filters_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_query_filters_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_render_query_filters_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::QueryFilters` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_query_filters_request_t`].
pub const XCB_RENDER_QUERY_FILTERS: u8 = 29i32 as u8;

/// The `Render::QueryFilters` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_query_filters_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
}

impl Default for xcb_render_query_filters_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::QueryFilters` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `aliases`
/// - `filters`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_query_filters_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_aliases: u32,
    pub num_filters: u32,
    pub pad1: [u8; 16],
}

impl Default for xcb_render_query_filters_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::SetPictureFilter` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_set_picture_filter_request_t`].
pub const XCB_RENDER_SET_PICTURE_FILTER: u8 = 30i32 as u8;

/// The `Render::SetPictureFilter` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `filter`
/// - `values_len`
/// - `values`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_set_picture_filter_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub picture: xcb_render_picture_t,
    pub filter_len: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_render_set_picture_filter_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::ANIMCURSORELT` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_animcursorelt_t {
    pub cursor: xcb_cursor_t,
    pub delay: u32,
}

impl Default for xcb_render_animcursorelt_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Render::ANIMCURSORELT` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_animcursorelt_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_render_animcursorelt_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_render_animcursorelt_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::CreateAnimCursor` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_create_anim_cursor_request_t`].
pub const XCB_RENDER_CREATE_ANIM_CURSOR: u8 = 31i32 as u8;

/// The `Render::CreateAnimCursor` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `cursors`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_create_anim_cursor_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub cid: xcb_cursor_t,
}

impl Default for xcb_render_create_anim_cursor_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::SPANFIX` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_spanfix_t {
    pub l: xcb_render_fixed_t,
    pub r: xcb_render_fixed_t,
    pub y: xcb_render_fixed_t,
}

impl Default for xcb_render_spanfix_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Render::SPANFIX` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_spanfix_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_render_spanfix_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_render_spanfix_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Render::TRAP` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_trap_t {
    pub top: xcb_render_spanfix_t,
    pub bot: xcb_render_spanfix_t,
}

impl Default for xcb_render_trap_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Render::TRAP` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_trap_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_render_trap_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_render_trap_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::AddTraps` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_add_traps_request_t`].
pub const XCB_RENDER_ADD_TRAPS: u8 = 32i32 as u8;

/// The `Render::AddTraps` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `traps`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_add_traps_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub picture: xcb_render_picture_t,
    pub x_off: i16,
    pub y_off: i16,
}

impl Default for xcb_render_add_traps_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::CreateSolidFill` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_create_solid_fill_request_t`].
pub const XCB_RENDER_CREATE_SOLID_FILL: u8 = 33i32 as u8;

/// The `Render::CreateSolidFill` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_create_solid_fill_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub picture: xcb_render_picture_t,
    pub color: xcb_render_color_t,
}

impl Default for xcb_render_create_solid_fill_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::CreateLinearGradient` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_create_linear_gradient_request_t`].
pub const XCB_RENDER_CREATE_LINEAR_GRADIENT: u8 = 34i32 as u8;

/// The `Render::CreateLinearGradient` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `stops`
/// - `colors`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_create_linear_gradient_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub picture: xcb_render_picture_t,
    pub p1: xcb_render_pointfix_t,
    pub p2: xcb_render_pointfix_t,
    pub num_stops: u32,
}

impl Default for xcb_render_create_linear_gradient_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::CreateRadialGradient` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_create_radial_gradient_request_t`].
pub const XCB_RENDER_CREATE_RADIAL_GRADIENT: u8 = 35i32 as u8;

/// The `Render::CreateRadialGradient` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `stops`
/// - `colors`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_create_radial_gradient_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub picture: xcb_render_picture_t,
    pub inner: xcb_render_pointfix_t,
    pub outer: xcb_render_pointfix_t,
    pub inner_radius: xcb_render_fixed_t,
    pub outer_radius: xcb_render_fixed_t,
    pub num_stops: u32,
}

impl Default for xcb_render_create_radial_gradient_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Render::CreateConicalGradient` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbRender::xcb_render_id()`], then the type of the request is
/// [`xcb_render_create_conical_gradient_request_t`].
pub const XCB_RENDER_CREATE_CONICAL_GRADIENT: u8 = 36i32 as u8;

/// The `Render::CreateConicalGradient` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `stops`
/// - `colors`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_render_create_conical_gradient_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub picture: xcb_render_picture_t,
    pub center: xcb_render_pointfix_t,
    pub angle: xcb_render_fixed_t,
    pub num_stops: u32,
}

impl Default for xcb_render_create_conical_gradient_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_render")]
pub(crate) struct XcbRenderRender {
    xcb_render_id: LazySymbol<*mut xcb_extension_t>,
    xcb_render_glyph_next: LazySymbol<unsafe fn(i: *mut xcb_render_glyph_iterator_t)>,
    xcb_render_glyph_end:
        LazySymbol<unsafe fn(i: xcb_render_glyph_iterator_t) -> xcb_generic_iterator_t>,
    xcb_render_glyphset_next: LazySymbol<unsafe fn(i: *mut xcb_render_glyphset_iterator_t)>,
    xcb_render_glyphset_end:
        LazySymbol<unsafe fn(i: xcb_render_glyphset_iterator_t) -> xcb_generic_iterator_t>,
    xcb_render_picture_next: LazySymbol<unsafe fn(i: *mut xcb_render_picture_iterator_t)>,
    xcb_render_picture_end:
        LazySymbol<unsafe fn(i: xcb_render_picture_iterator_t) -> xcb_generic_iterator_t>,
    xcb_render_pictformat_next: LazySymbol<unsafe fn(i: *mut xcb_render_pictformat_iterator_t)>,
    xcb_render_pictformat_end:
        LazySymbol<unsafe fn(i: xcb_render_pictformat_iterator_t) -> xcb_generic_iterator_t>,
    xcb_render_fixed_next: LazySymbol<unsafe fn(i: *mut xcb_render_fixed_iterator_t)>,
    xcb_render_fixed_end:
        LazySymbol<unsafe fn(i: xcb_render_fixed_iterator_t) -> xcb_generic_iterator_t>,
    xcb_render_directformat_next: LazySymbol<unsafe fn(i: *mut xcb_render_directformat_iterator_t)>,
    xcb_render_directformat_end:
        LazySymbol<unsafe fn(i: xcb_render_directformat_iterator_t) -> xcb_generic_iterator_t>,
    xcb_render_pictforminfo_next: LazySymbol<unsafe fn(i: *mut xcb_render_pictforminfo_iterator_t)>,
    xcb_render_pictforminfo_end:
        LazySymbol<unsafe fn(i: xcb_render_pictforminfo_iterator_t) -> xcb_generic_iterator_t>,
    xcb_render_pictvisual_next: LazySymbol<unsafe fn(i: *mut xcb_render_pictvisual_iterator_t)>,
    xcb_render_pictvisual_end:
        LazySymbol<unsafe fn(i: xcb_render_pictvisual_iterator_t) -> xcb_generic_iterator_t>,
    xcb_render_pictdepth_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_render_pictdepth_visuals:
        LazySymbol<unsafe fn(r: *const xcb_render_pictdepth_t) -> *mut xcb_render_pictvisual_t>,
    xcb_render_pictdepth_visuals_length:
        LazySymbol<unsafe fn(r: *const xcb_render_pictdepth_t) -> c_int>,
    xcb_render_pictdepth_visuals_iterator:
        LazySymbol<unsafe fn(r: *const xcb_render_pictdepth_t) -> xcb_render_pictvisual_iterator_t>,
    xcb_render_pictdepth_next: LazySymbol<unsafe fn(i: *mut xcb_render_pictdepth_iterator_t)>,
    xcb_render_pictdepth_end:
        LazySymbol<unsafe fn(i: xcb_render_pictdepth_iterator_t) -> xcb_generic_iterator_t>,
    xcb_render_pictscreen_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_render_pictscreen_depths_length:
        LazySymbol<unsafe fn(r: *const xcb_render_pictscreen_t) -> c_int>,
    xcb_render_pictscreen_depths_iterator:
        LazySymbol<unsafe fn(r: *const xcb_render_pictscreen_t) -> xcb_render_pictdepth_iterator_t>,
    xcb_render_pictscreen_next: LazySymbol<unsafe fn(i: *mut xcb_render_pictscreen_iterator_t)>,
    xcb_render_pictscreen_end:
        LazySymbol<unsafe fn(i: xcb_render_pictscreen_iterator_t) -> xcb_generic_iterator_t>,
    xcb_render_indexvalue_next: LazySymbol<unsafe fn(i: *mut xcb_render_indexvalue_iterator_t)>,
    xcb_render_indexvalue_end:
        LazySymbol<unsafe fn(i: xcb_render_indexvalue_iterator_t) -> xcb_generic_iterator_t>,
    xcb_render_color_next: LazySymbol<unsafe fn(i: *mut xcb_render_color_iterator_t)>,
    xcb_render_color_end:
        LazySymbol<unsafe fn(i: xcb_render_color_iterator_t) -> xcb_generic_iterator_t>,
    xcb_render_pointfix_next: LazySymbol<unsafe fn(i: *mut xcb_render_pointfix_iterator_t)>,
    xcb_render_pointfix_end:
        LazySymbol<unsafe fn(i: xcb_render_pointfix_iterator_t) -> xcb_generic_iterator_t>,
    xcb_render_linefix_next: LazySymbol<unsafe fn(i: *mut xcb_render_linefix_iterator_t)>,
    xcb_render_linefix_end:
        LazySymbol<unsafe fn(i: xcb_render_linefix_iterator_t) -> xcb_generic_iterator_t>,
    xcb_render_triangle_next: LazySymbol<unsafe fn(i: *mut xcb_render_triangle_iterator_t)>,
    xcb_render_triangle_end:
        LazySymbol<unsafe fn(i: xcb_render_triangle_iterator_t) -> xcb_generic_iterator_t>,
    xcb_render_trapezoid_next: LazySymbol<unsafe fn(i: *mut xcb_render_trapezoid_iterator_t)>,
    xcb_render_trapezoid_end:
        LazySymbol<unsafe fn(i: xcb_render_trapezoid_iterator_t) -> xcb_generic_iterator_t>,
    xcb_render_glyphinfo_next: LazySymbol<unsafe fn(i: *mut xcb_render_glyphinfo_iterator_t)>,
    xcb_render_glyphinfo_end:
        LazySymbol<unsafe fn(i: xcb_render_glyphinfo_iterator_t) -> xcb_generic_iterator_t>,
    xcb_render_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u32,
            client_minor_version: u32,
        ) -> xcb_render_query_version_cookie_t,
    >,
    xcb_render_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u32,
            client_minor_version: u32,
        ) -> xcb_render_query_version_cookie_t,
    >,
    xcb_render_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_render_query_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_render_query_version_reply_t,
    >,
    xcb_render_query_pict_formats_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_render_query_pict_formats:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_render_query_pict_formats_cookie_t>,
    xcb_render_query_pict_formats_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_render_query_pict_formats_cookie_t>,
    xcb_render_query_pict_formats_formats: LazySymbol<
        unsafe fn(
            r: *const xcb_render_query_pict_formats_reply_t,
        ) -> *mut xcb_render_pictforminfo_t,
    >,
    xcb_render_query_pict_formats_formats_length:
        LazySymbol<unsafe fn(r: *const xcb_render_query_pict_formats_reply_t) -> c_int>,
    xcb_render_query_pict_formats_formats_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_render_query_pict_formats_reply_t,
        ) -> xcb_render_pictforminfo_iterator_t,
    >,
    xcb_render_query_pict_formats_screens_length:
        LazySymbol<unsafe fn(r: *const xcb_render_query_pict_formats_reply_t) -> c_int>,
    xcb_render_query_pict_formats_screens_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_render_query_pict_formats_reply_t,
        ) -> xcb_render_pictscreen_iterator_t,
    >,
    xcb_render_query_pict_formats_subpixels:
        LazySymbol<unsafe fn(r: *const xcb_render_query_pict_formats_reply_t) -> *mut u32>,
    xcb_render_query_pict_formats_subpixels_length:
        LazySymbol<unsafe fn(r: *const xcb_render_query_pict_formats_reply_t) -> c_int>,
    xcb_render_query_pict_formats_subpixels_end: LazySymbol<
        unsafe fn(r: *const xcb_render_query_pict_formats_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_render_query_pict_formats_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_render_query_pict_formats_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_render_query_pict_formats_reply_t,
    >,
    xcb_render_query_pict_index_values_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_render_query_pict_index_values: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            format: xcb_render_pictformat_t,
        ) -> xcb_render_query_pict_index_values_cookie_t,
    >,
    xcb_render_query_pict_index_values_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            format: xcb_render_pictformat_t,
        ) -> xcb_render_query_pict_index_values_cookie_t,
    >,
    xcb_render_query_pict_index_values_values: LazySymbol<
        unsafe fn(
            r: *const xcb_render_query_pict_index_values_reply_t,
        ) -> *mut xcb_render_indexvalue_t,
    >,
    xcb_render_query_pict_index_values_values_length:
        LazySymbol<unsafe fn(r: *const xcb_render_query_pict_index_values_reply_t) -> c_int>,
    xcb_render_query_pict_index_values_values_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_render_query_pict_index_values_reply_t,
        ) -> xcb_render_indexvalue_iterator_t,
    >,
    xcb_render_query_pict_index_values_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_render_query_pict_index_values_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_render_query_pict_index_values_reply_t,
    >,
    xcb_render_create_picture_value_list_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            value_mask: u32,
            _aux: *const xcb_render_create_picture_value_list_t,
        ) -> c_int,
    >,
    xcb_render_create_picture_value_list_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            value_mask: u32,
            _aux: *mut xcb_render_create_picture_value_list_t,
        ) -> c_int,
    >,
    xcb_render_create_picture_value_list_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, value_mask: u32) -> c_int>,
    xcb_render_create_picture_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_render_create_picture_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            pid: xcb_render_picture_t,
            drawable: xcb_drawable_t,
            format: xcb_render_pictformat_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_create_picture: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            pid: xcb_render_picture_t,
            drawable: xcb_drawable_t,
            format: xcb_render_pictformat_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_create_picture_aux_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            pid: xcb_render_picture_t,
            drawable: xcb_drawable_t,
            format: xcb_render_pictformat_t,
            value_mask: u32,
            value_list: *const xcb_render_create_picture_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_create_picture_aux: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            pid: xcb_render_picture_t,
            drawable: xcb_drawable_t,
            format: xcb_render_pictformat_t,
            value_mask: u32,
            value_list: *const xcb_render_create_picture_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_create_picture_value_list:
        LazySymbol<unsafe fn(r: *const xcb_render_create_picture_request_t) -> *mut c_void>,
    xcb_render_change_picture_value_list_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            value_mask: u32,
            _aux: *const xcb_render_change_picture_value_list_t,
        ) -> c_int,
    >,
    xcb_render_change_picture_value_list_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            value_mask: u32,
            _aux: *mut xcb_render_change_picture_value_list_t,
        ) -> c_int,
    >,
    xcb_render_change_picture_value_list_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, value_mask: u32) -> c_int>,
    xcb_render_change_picture_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_render_change_picture_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_change_picture: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_change_picture_aux_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            value_mask: u32,
            value_list: *const xcb_render_change_picture_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_change_picture_aux: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            value_mask: u32,
            value_list: *const xcb_render_change_picture_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_change_picture_value_list:
        LazySymbol<unsafe fn(r: *const xcb_render_change_picture_request_t) -> *mut c_void>,
    xcb_render_set_picture_clip_rectangles_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, rectangles_len: u32) -> c_int>,
    xcb_render_set_picture_clip_rectangles_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            clip_x_origin: i16,
            clip_y_origin: i16,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_set_picture_clip_rectangles: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            clip_x_origin: i16,
            clip_y_origin: i16,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_set_picture_clip_rectangles_rectangles: LazySymbol<
        unsafe fn(
            r: *const xcb_render_set_picture_clip_rectangles_request_t,
        ) -> *mut xcb_rectangle_t,
    >,
    xcb_render_set_picture_clip_rectangles_rectangles_length:
        LazySymbol<unsafe fn(r: *const xcb_render_set_picture_clip_rectangles_request_t) -> c_int>,
    xcb_render_set_picture_clip_rectangles_rectangles_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_render_set_picture_clip_rectangles_request_t,
        ) -> xcb_rectangle_iterator_t,
    >,
    xcb_render_free_picture_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, picture: xcb_render_picture_t) -> xcb_void_cookie_t,
    >,
    xcb_render_free_picture: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, picture: xcb_render_picture_t) -> xcb_void_cookie_t,
    >,
    xcb_render_composite_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            mask: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            src_x: i16,
            src_y: i16,
            mask_x: i16,
            mask_y: i16,
            dst_x: i16,
            dst_y: i16,
            width: u16,
            height: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_composite: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            mask: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            src_x: i16,
            src_y: i16,
            mask_x: i16,
            mask_y: i16,
            dst_x: i16,
            dst_y: i16,
            width: u16,
            height: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_trapezoids_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, traps_len: u32) -> c_int>,
    xcb_render_trapezoids_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            src_x: i16,
            src_y: i16,
            traps_len: u32,
            traps: *const xcb_render_trapezoid_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_trapezoids: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            src_x: i16,
            src_y: i16,
            traps_len: u32,
            traps: *const xcb_render_trapezoid_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_trapezoids_traps: LazySymbol<
        unsafe fn(r: *const xcb_render_trapezoids_request_t) -> *mut xcb_render_trapezoid_t,
    >,
    xcb_render_trapezoids_traps_length:
        LazySymbol<unsafe fn(r: *const xcb_render_trapezoids_request_t) -> c_int>,
    xcb_render_trapezoids_traps_iterator: LazySymbol<
        unsafe fn(r: *const xcb_render_trapezoids_request_t) -> xcb_render_trapezoid_iterator_t,
    >,
    xcb_render_triangles_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, triangles_len: u32) -> c_int>,
    xcb_render_triangles_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            src_x: i16,
            src_y: i16,
            triangles_len: u32,
            triangles: *const xcb_render_triangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_triangles: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            src_x: i16,
            src_y: i16,
            triangles_len: u32,
            triangles: *const xcb_render_triangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_triangles_triangles: LazySymbol<
        unsafe fn(r: *const xcb_render_triangles_request_t) -> *mut xcb_render_triangle_t,
    >,
    xcb_render_triangles_triangles_length:
        LazySymbol<unsafe fn(r: *const xcb_render_triangles_request_t) -> c_int>,
    xcb_render_triangles_triangles_iterator: LazySymbol<
        unsafe fn(r: *const xcb_render_triangles_request_t) -> xcb_render_triangle_iterator_t,
    >,
    xcb_render_tri_strip_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, points_len: u32) -> c_int>,
    xcb_render_tri_strip_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            src_x: i16,
            src_y: i16,
            points_len: u32,
            points: *const xcb_render_pointfix_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_tri_strip: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            src_x: i16,
            src_y: i16,
            points_len: u32,
            points: *const xcb_render_pointfix_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_tri_strip_points: LazySymbol<
        unsafe fn(r: *const xcb_render_tri_strip_request_t) -> *mut xcb_render_pointfix_t,
    >,
    xcb_render_tri_strip_points_length:
        LazySymbol<unsafe fn(r: *const xcb_render_tri_strip_request_t) -> c_int>,
    xcb_render_tri_strip_points_iterator: LazySymbol<
        unsafe fn(r: *const xcb_render_tri_strip_request_t) -> xcb_render_pointfix_iterator_t,
    >,
    xcb_render_tri_fan_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, points_len: u32) -> c_int>,
    xcb_render_tri_fan_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            src_x: i16,
            src_y: i16,
            points_len: u32,
            points: *const xcb_render_pointfix_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_tri_fan: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            src_x: i16,
            src_y: i16,
            points_len: u32,
            points: *const xcb_render_pointfix_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_tri_fan_points:
        LazySymbol<unsafe fn(r: *const xcb_render_tri_fan_request_t) -> *mut xcb_render_pointfix_t>,
    xcb_render_tri_fan_points_length:
        LazySymbol<unsafe fn(r: *const xcb_render_tri_fan_request_t) -> c_int>,
    xcb_render_tri_fan_points_iterator: LazySymbol<
        unsafe fn(r: *const xcb_render_tri_fan_request_t) -> xcb_render_pointfix_iterator_t,
    >,
    xcb_render_create_glyph_set_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gsid: xcb_render_glyphset_t,
            format: xcb_render_pictformat_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_create_glyph_set: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gsid: xcb_render_glyphset_t,
            format: xcb_render_pictformat_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_reference_glyph_set_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gsid: xcb_render_glyphset_t,
            existing: xcb_render_glyphset_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_reference_glyph_set: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gsid: xcb_render_glyphset_t,
            existing: xcb_render_glyphset_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_free_glyph_set_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, glyphset: xcb_render_glyphset_t) -> xcb_void_cookie_t,
    >,
    xcb_render_free_glyph_set: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, glyphset: xcb_render_glyphset_t) -> xcb_void_cookie_t,
    >,
    xcb_render_add_glyphs_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, data_len: u32) -> c_int>,
    xcb_render_add_glyphs_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            glyphset: xcb_render_glyphset_t,
            glyphs_len: u32,
            glyphids: *const u32,
            glyphs: *const xcb_render_glyphinfo_t,
            data_len: u32,
            data: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_add_glyphs: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            glyphset: xcb_render_glyphset_t,
            glyphs_len: u32,
            glyphids: *const u32,
            glyphs: *const xcb_render_glyphinfo_t,
            data_len: u32,
            data: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_add_glyphs_glyphids:
        LazySymbol<unsafe fn(r: *const xcb_render_add_glyphs_request_t) -> *mut u32>,
    xcb_render_add_glyphs_glyphids_length:
        LazySymbol<unsafe fn(r: *const xcb_render_add_glyphs_request_t) -> c_int>,
    xcb_render_add_glyphs_glyphids_end:
        LazySymbol<unsafe fn(r: *const xcb_render_add_glyphs_request_t) -> xcb_generic_iterator_t>,
    xcb_render_add_glyphs_glyphs: LazySymbol<
        unsafe fn(r: *const xcb_render_add_glyphs_request_t) -> *mut xcb_render_glyphinfo_t,
    >,
    xcb_render_add_glyphs_glyphs_length:
        LazySymbol<unsafe fn(r: *const xcb_render_add_glyphs_request_t) -> c_int>,
    xcb_render_add_glyphs_glyphs_iterator: LazySymbol<
        unsafe fn(r: *const xcb_render_add_glyphs_request_t) -> xcb_render_glyphinfo_iterator_t,
    >,
    xcb_render_add_glyphs_data:
        LazySymbol<unsafe fn(r: *const xcb_render_add_glyphs_request_t) -> *mut u8>,
    xcb_render_add_glyphs_data_length:
        LazySymbol<unsafe fn(r: *const xcb_render_add_glyphs_request_t) -> c_int>,
    xcb_render_add_glyphs_data_end:
        LazySymbol<unsafe fn(r: *const xcb_render_add_glyphs_request_t) -> xcb_generic_iterator_t>,
    xcb_render_free_glyphs_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, glyphs_len: u32) -> c_int>,
    xcb_render_free_glyphs_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            glyphset: xcb_render_glyphset_t,
            glyphs_len: u32,
            glyphs: *const xcb_render_glyph_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_free_glyphs: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            glyphset: xcb_render_glyphset_t,
            glyphs_len: u32,
            glyphs: *const xcb_render_glyph_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_free_glyphs_glyphs: LazySymbol<
        unsafe fn(r: *const xcb_render_free_glyphs_request_t) -> *mut xcb_render_glyph_t,
    >,
    xcb_render_free_glyphs_glyphs_length:
        LazySymbol<unsafe fn(r: *const xcb_render_free_glyphs_request_t) -> c_int>,
    xcb_render_free_glyphs_glyphs_end:
        LazySymbol<unsafe fn(r: *const xcb_render_free_glyphs_request_t) -> xcb_generic_iterator_t>,
    xcb_render_composite_glyphs_8_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, glyphcmds_len: u32) -> c_int>,
    xcb_render_composite_glyphs_8_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            glyphset: xcb_render_glyphset_t,
            src_x: i16,
            src_y: i16,
            glyphcmds_len: u32,
            glyphcmds: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_composite_glyphs_8: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            glyphset: xcb_render_glyphset_t,
            src_x: i16,
            src_y: i16,
            glyphcmds_len: u32,
            glyphcmds: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_composite_glyphs_8_glyphcmds:
        LazySymbol<unsafe fn(r: *const xcb_render_composite_glyphs_8_request_t) -> *mut u8>,
    xcb_render_composite_glyphs_8_glyphcmds_length:
        LazySymbol<unsafe fn(r: *const xcb_render_composite_glyphs_8_request_t) -> c_int>,
    xcb_render_composite_glyphs_8_glyphcmds_end: LazySymbol<
        unsafe fn(r: *const xcb_render_composite_glyphs_8_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_render_composite_glyphs_16_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, glyphcmds_len: u32) -> c_int>,
    xcb_render_composite_glyphs_16_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            glyphset: xcb_render_glyphset_t,
            src_x: i16,
            src_y: i16,
            glyphcmds_len: u32,
            glyphcmds: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_composite_glyphs_16: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            glyphset: xcb_render_glyphset_t,
            src_x: i16,
            src_y: i16,
            glyphcmds_len: u32,
            glyphcmds: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_composite_glyphs_16_glyphcmds:
        LazySymbol<unsafe fn(r: *const xcb_render_composite_glyphs_16_request_t) -> *mut u8>,
    xcb_render_composite_glyphs_16_glyphcmds_length:
        LazySymbol<unsafe fn(r: *const xcb_render_composite_glyphs_16_request_t) -> c_int>,
    xcb_render_composite_glyphs_16_glyphcmds_end: LazySymbol<
        unsafe fn(r: *const xcb_render_composite_glyphs_16_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_render_composite_glyphs_32_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, glyphcmds_len: u32) -> c_int>,
    xcb_render_composite_glyphs_32_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            glyphset: xcb_render_glyphset_t,
            src_x: i16,
            src_y: i16,
            glyphcmds_len: u32,
            glyphcmds: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_composite_glyphs_32: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            src: xcb_render_picture_t,
            dst: xcb_render_picture_t,
            mask_format: xcb_render_pictformat_t,
            glyphset: xcb_render_glyphset_t,
            src_x: i16,
            src_y: i16,
            glyphcmds_len: u32,
            glyphcmds: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_composite_glyphs_32_glyphcmds:
        LazySymbol<unsafe fn(r: *const xcb_render_composite_glyphs_32_request_t) -> *mut u8>,
    xcb_render_composite_glyphs_32_glyphcmds_length:
        LazySymbol<unsafe fn(r: *const xcb_render_composite_glyphs_32_request_t) -> c_int>,
    xcb_render_composite_glyphs_32_glyphcmds_end: LazySymbol<
        unsafe fn(r: *const xcb_render_composite_glyphs_32_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_render_fill_rectangles_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, rects_len: u32) -> c_int>,
    xcb_render_fill_rectangles_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            dst: xcb_render_picture_t,
            color: xcb_render_color_t,
            rects_len: u32,
            rects: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_fill_rectangles: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            op: u8,
            dst: xcb_render_picture_t,
            color: xcb_render_color_t,
            rects_len: u32,
            rects: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_fill_rectangles_rects: LazySymbol<
        unsafe fn(r: *const xcb_render_fill_rectangles_request_t) -> *mut xcb_rectangle_t,
    >,
    xcb_render_fill_rectangles_rects_length:
        LazySymbol<unsafe fn(r: *const xcb_render_fill_rectangles_request_t) -> c_int>,
    xcb_render_fill_rectangles_rects_iterator: LazySymbol<
        unsafe fn(r: *const xcb_render_fill_rectangles_request_t) -> xcb_rectangle_iterator_t,
    >,
    xcb_render_create_cursor_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cid: xcb_cursor_t,
            source: xcb_render_picture_t,
            x: u16,
            y: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_create_cursor: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cid: xcb_cursor_t,
            source: xcb_render_picture_t,
            x: u16,
            y: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_transform_next: LazySymbol<unsafe fn(i: *mut xcb_render_transform_iterator_t)>,
    xcb_render_transform_end:
        LazySymbol<unsafe fn(i: xcb_render_transform_iterator_t) -> xcb_generic_iterator_t>,
    xcb_render_set_picture_transform_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            transform: xcb_render_transform_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_set_picture_transform: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            transform: xcb_render_transform_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_query_filters_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_render_query_filters: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
        ) -> xcb_render_query_filters_cookie_t,
    >,
    xcb_render_query_filters_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
        ) -> xcb_render_query_filters_cookie_t,
    >,
    xcb_render_query_filters_aliases:
        LazySymbol<unsafe fn(r: *const xcb_render_query_filters_reply_t) -> *mut u16>,
    xcb_render_query_filters_aliases_length:
        LazySymbol<unsafe fn(r: *const xcb_render_query_filters_reply_t) -> c_int>,
    xcb_render_query_filters_aliases_end:
        LazySymbol<unsafe fn(r: *const xcb_render_query_filters_reply_t) -> xcb_generic_iterator_t>,
    xcb_render_query_filters_filters_length:
        LazySymbol<unsafe fn(r: *const xcb_render_query_filters_reply_t) -> c_int>,
    xcb_render_query_filters_filters_iterator:
        LazySymbol<unsafe fn(r: *const xcb_render_query_filters_reply_t) -> xcb_str_iterator_t>,
    xcb_render_query_filters_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_render_query_filters_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_render_query_filters_reply_t,
    >,
    xcb_render_set_picture_filter_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, values_len: u32) -> c_int>,
    xcb_render_set_picture_filter_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            filter_len: u16,
            filter: *const c_char,
            values_len: u32,
            values: *const xcb_render_fixed_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_set_picture_filter: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            filter_len: u16,
            filter: *const c_char,
            values_len: u32,
            values: *const xcb_render_fixed_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_set_picture_filter_filter:
        LazySymbol<unsafe fn(r: *const xcb_render_set_picture_filter_request_t) -> *mut c_char>,
    xcb_render_set_picture_filter_filter_length:
        LazySymbol<unsafe fn(r: *const xcb_render_set_picture_filter_request_t) -> c_int>,
    xcb_render_set_picture_filter_filter_end: LazySymbol<
        unsafe fn(r: *const xcb_render_set_picture_filter_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_render_set_picture_filter_values: LazySymbol<
        unsafe fn(r: *const xcb_render_set_picture_filter_request_t) -> *mut xcb_render_fixed_t,
    >,
    xcb_render_set_picture_filter_values_length:
        LazySymbol<unsafe fn(r: *const xcb_render_set_picture_filter_request_t) -> c_int>,
    xcb_render_set_picture_filter_values_end: LazySymbol<
        unsafe fn(r: *const xcb_render_set_picture_filter_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_render_animcursorelt_next:
        LazySymbol<unsafe fn(i: *mut xcb_render_animcursorelt_iterator_t)>,
    xcb_render_animcursorelt_end:
        LazySymbol<unsafe fn(i: xcb_render_animcursorelt_iterator_t) -> xcb_generic_iterator_t>,
    xcb_render_create_anim_cursor_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, cursors_len: u32) -> c_int>,
    xcb_render_create_anim_cursor_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cid: xcb_cursor_t,
            cursors_len: u32,
            cursors: *const xcb_render_animcursorelt_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_create_anim_cursor: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cid: xcb_cursor_t,
            cursors_len: u32,
            cursors: *const xcb_render_animcursorelt_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_create_anim_cursor_cursors: LazySymbol<
        unsafe fn(
            r: *const xcb_render_create_anim_cursor_request_t,
        ) -> *mut xcb_render_animcursorelt_t,
    >,
    xcb_render_create_anim_cursor_cursors_length:
        LazySymbol<unsafe fn(r: *const xcb_render_create_anim_cursor_request_t) -> c_int>,
    xcb_render_create_anim_cursor_cursors_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_render_create_anim_cursor_request_t,
        ) -> xcb_render_animcursorelt_iterator_t,
    >,
    xcb_render_spanfix_next: LazySymbol<unsafe fn(i: *mut xcb_render_spanfix_iterator_t)>,
    xcb_render_spanfix_end:
        LazySymbol<unsafe fn(i: xcb_render_spanfix_iterator_t) -> xcb_generic_iterator_t>,
    xcb_render_trap_next: LazySymbol<unsafe fn(i: *mut xcb_render_trap_iterator_t)>,
    xcb_render_trap_end:
        LazySymbol<unsafe fn(i: xcb_render_trap_iterator_t) -> xcb_generic_iterator_t>,
    xcb_render_add_traps_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, traps_len: u32) -> c_int>,
    xcb_render_add_traps_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            x_off: i16,
            y_off: i16,
            traps_len: u32,
            traps: *const xcb_render_trap_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_add_traps: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            x_off: i16,
            y_off: i16,
            traps_len: u32,
            traps: *const xcb_render_trap_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_add_traps_traps:
        LazySymbol<unsafe fn(r: *const xcb_render_add_traps_request_t) -> *mut xcb_render_trap_t>,
    xcb_render_add_traps_traps_length:
        LazySymbol<unsafe fn(r: *const xcb_render_add_traps_request_t) -> c_int>,
    xcb_render_add_traps_traps_iterator: LazySymbol<
        unsafe fn(r: *const xcb_render_add_traps_request_t) -> xcb_render_trap_iterator_t,
    >,
    xcb_render_create_solid_fill_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            color: xcb_render_color_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_create_solid_fill: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            color: xcb_render_color_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_create_linear_gradient_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_render_create_linear_gradient_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            p1: xcb_render_pointfix_t,
            p2: xcb_render_pointfix_t,
            num_stops: u32,
            stops: *const xcb_render_fixed_t,
            colors: *const xcb_render_color_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_create_linear_gradient: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            p1: xcb_render_pointfix_t,
            p2: xcb_render_pointfix_t,
            num_stops: u32,
            stops: *const xcb_render_fixed_t,
            colors: *const xcb_render_color_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_create_linear_gradient_stops: LazySymbol<
        unsafe fn(r: *const xcb_render_create_linear_gradient_request_t) -> *mut xcb_render_fixed_t,
    >,
    xcb_render_create_linear_gradient_stops_length:
        LazySymbol<unsafe fn(r: *const xcb_render_create_linear_gradient_request_t) -> c_int>,
    xcb_render_create_linear_gradient_stops_end: LazySymbol<
        unsafe fn(r: *const xcb_render_create_linear_gradient_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_render_create_linear_gradient_colors: LazySymbol<
        unsafe fn(r: *const xcb_render_create_linear_gradient_request_t) -> *mut xcb_render_color_t,
    >,
    xcb_render_create_linear_gradient_colors_length:
        LazySymbol<unsafe fn(r: *const xcb_render_create_linear_gradient_request_t) -> c_int>,
    xcb_render_create_linear_gradient_colors_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_render_create_linear_gradient_request_t,
        ) -> xcb_render_color_iterator_t,
    >,
    xcb_render_create_radial_gradient_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_render_create_radial_gradient_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            inner: xcb_render_pointfix_t,
            outer: xcb_render_pointfix_t,
            inner_radius: xcb_render_fixed_t,
            outer_radius: xcb_render_fixed_t,
            num_stops: u32,
            stops: *const xcb_render_fixed_t,
            colors: *const xcb_render_color_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_create_radial_gradient: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            inner: xcb_render_pointfix_t,
            outer: xcb_render_pointfix_t,
            inner_radius: xcb_render_fixed_t,
            outer_radius: xcb_render_fixed_t,
            num_stops: u32,
            stops: *const xcb_render_fixed_t,
            colors: *const xcb_render_color_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_create_radial_gradient_stops: LazySymbol<
        unsafe fn(r: *const xcb_render_create_radial_gradient_request_t) -> *mut xcb_render_fixed_t,
    >,
    xcb_render_create_radial_gradient_stops_length:
        LazySymbol<unsafe fn(r: *const xcb_render_create_radial_gradient_request_t) -> c_int>,
    xcb_render_create_radial_gradient_stops_end: LazySymbol<
        unsafe fn(r: *const xcb_render_create_radial_gradient_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_render_create_radial_gradient_colors: LazySymbol<
        unsafe fn(r: *const xcb_render_create_radial_gradient_request_t) -> *mut xcb_render_color_t,
    >,
    xcb_render_create_radial_gradient_colors_length:
        LazySymbol<unsafe fn(r: *const xcb_render_create_radial_gradient_request_t) -> c_int>,
    xcb_render_create_radial_gradient_colors_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_render_create_radial_gradient_request_t,
        ) -> xcb_render_color_iterator_t,
    >,
    xcb_render_create_conical_gradient_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_render_create_conical_gradient_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            center: xcb_render_pointfix_t,
            angle: xcb_render_fixed_t,
            num_stops: u32,
            stops: *const xcb_render_fixed_t,
            colors: *const xcb_render_color_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_create_conical_gradient: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            center: xcb_render_pointfix_t,
            angle: xcb_render_fixed_t,
            num_stops: u32,
            stops: *const xcb_render_fixed_t,
            colors: *const xcb_render_color_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_render_create_conical_gradient_stops: LazySymbol<
        unsafe fn(
            r: *const xcb_render_create_conical_gradient_request_t,
        ) -> *mut xcb_render_fixed_t,
    >,
    xcb_render_create_conical_gradient_stops_length:
        LazySymbol<unsafe fn(r: *const xcb_render_create_conical_gradient_request_t) -> c_int>,
    xcb_render_create_conical_gradient_stops_end: LazySymbol<
        unsafe fn(r: *const xcb_render_create_conical_gradient_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_render_create_conical_gradient_colors: LazySymbol<
        unsafe fn(
            r: *const xcb_render_create_conical_gradient_request_t,
        ) -> *mut xcb_render_color_t,
    >,
    xcb_render_create_conical_gradient_colors_length:
        LazySymbol<unsafe fn(r: *const xcb_render_create_conical_gradient_request_t) -> c_int>,
    xcb_render_create_conical_gradient_colors_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_render_create_conical_gradient_request_t,
        ) -> xcb_render_color_iterator_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self
            .render
            .$f
            .get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .render
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_render")]
impl XcbRender {
    /// The libxcb identifier of the `Render` extension.
    #[inline]
    pub fn xcb_render_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_render_id) }
    }

    /// Returns `true` iff the symbol `xcb_render_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_id(&self) -> bool {
        has_sym!(self, xcb_render_id)
    }

    /// Advances a `xcb_render_glyph_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_render_glyph_next(&self, i: *mut xcb_render_glyph_iterator_t) {
        sym!(self, xcb_render_glyph_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_glyph_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_glyph_next(&self) -> bool {
        has_sym!(self, xcb_render_glyph_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_render_glyph_iterator_t`.
    #[inline]
    pub unsafe fn xcb_render_glyph_end(
        &self,
        i: xcb_render_glyph_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_glyph_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_glyph_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_glyph_end(&self) -> bool {
        has_sym!(self, xcb_render_glyph_end)
    }

    /// Advances a `xcb_render_glyphset_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_render_glyphset_next(&self, i: *mut xcb_render_glyphset_iterator_t) {
        sym!(self, xcb_render_glyphset_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_glyphset_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_glyphset_next(&self) -> bool {
        has_sym!(self, xcb_render_glyphset_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_render_glyphset_iterator_t`.
    #[inline]
    pub unsafe fn xcb_render_glyphset_end(
        &self,
        i: xcb_render_glyphset_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_glyphset_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_glyphset_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_glyphset_end(&self) -> bool {
        has_sym!(self, xcb_render_glyphset_end)
    }

    /// Advances a `xcb_render_picture_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_render_picture_next(&self, i: *mut xcb_render_picture_iterator_t) {
        sym!(self, xcb_render_picture_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_picture_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_picture_next(&self) -> bool {
        has_sym!(self, xcb_render_picture_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_render_picture_iterator_t`.
    #[inline]
    pub unsafe fn xcb_render_picture_end(
        &self,
        i: xcb_render_picture_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_picture_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_picture_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_picture_end(&self) -> bool {
        has_sym!(self, xcb_render_picture_end)
    }

    /// Advances a `xcb_render_pictformat_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_render_pictformat_next(&self, i: *mut xcb_render_pictformat_iterator_t) {
        sym!(self, xcb_render_pictformat_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_pictformat_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_pictformat_next(&self) -> bool {
        has_sym!(self, xcb_render_pictformat_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_render_pictformat_iterator_t`.
    #[inline]
    pub unsafe fn xcb_render_pictformat_end(
        &self,
        i: xcb_render_pictformat_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_pictformat_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_pictformat_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_pictformat_end(&self) -> bool {
        has_sym!(self, xcb_render_pictformat_end)
    }

    /// Advances a `xcb_render_fixed_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_render_fixed_next(&self, i: *mut xcb_render_fixed_iterator_t) {
        sym!(self, xcb_render_fixed_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_fixed_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_fixed_next(&self) -> bool {
        has_sym!(self, xcb_render_fixed_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_render_fixed_iterator_t`.
    #[inline]
    pub unsafe fn xcb_render_fixed_end(
        &self,
        i: xcb_render_fixed_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_fixed_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_fixed_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_fixed_end(&self) -> bool {
        has_sym!(self, xcb_render_fixed_end)
    }

    /// Advances a `xcb_render_directformat_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_render_directformat_next(&self, i: *mut xcb_render_directformat_iterator_t) {
        sym!(self, xcb_render_directformat_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_directformat_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_directformat_next(&self) -> bool {
        has_sym!(self, xcb_render_directformat_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_render_directformat_iterator_t`.
    #[inline]
    pub unsafe fn xcb_render_directformat_end(
        &self,
        i: xcb_render_directformat_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_directformat_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_directformat_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_directformat_end(&self) -> bool {
        has_sym!(self, xcb_render_directformat_end)
    }

    /// Advances a `xcb_render_pictforminfo_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_render_pictforminfo_next(&self, i: *mut xcb_render_pictforminfo_iterator_t) {
        sym!(self, xcb_render_pictforminfo_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_pictforminfo_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_pictforminfo_next(&self) -> bool {
        has_sym!(self, xcb_render_pictforminfo_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_render_pictforminfo_iterator_t`.
    #[inline]
    pub unsafe fn xcb_render_pictforminfo_end(
        &self,
        i: xcb_render_pictforminfo_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_pictforminfo_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_pictforminfo_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_pictforminfo_end(&self) -> bool {
        has_sym!(self, xcb_render_pictforminfo_end)
    }

    /// Advances a `xcb_render_pictvisual_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_render_pictvisual_next(&self, i: *mut xcb_render_pictvisual_iterator_t) {
        sym!(self, xcb_render_pictvisual_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_pictvisual_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_pictvisual_next(&self) -> bool {
        has_sym!(self, xcb_render_pictvisual_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_render_pictvisual_iterator_t`.
    #[inline]
    pub unsafe fn xcb_render_pictvisual_end(
        &self,
        i: xcb_render_pictvisual_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_pictvisual_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_pictvisual_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_pictvisual_end(&self) -> bool {
        has_sym!(self, xcb_render_pictvisual_end)
    }

    /// Computes the size of a `xcb_render_pictdepth_t` object.
    #[inline]
    pub unsafe fn xcb_render_pictdepth_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_render_pictdepth_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_render_pictdepth_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_pictdepth_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_pictdepth_sizeof)
    }

    /// Returns a pointer to the `visuals` field of a `xcb_render_pictdepth_t` struct.
    #[inline]
    pub unsafe fn xcb_render_pictdepth_visuals(
        &self,
        r: *const xcb_render_pictdepth_t,
    ) -> *mut xcb_render_pictvisual_t {
        sym!(self, xcb_render_pictdepth_visuals)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_pictdepth_visuals` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_pictdepth_visuals(&self) -> bool {
        has_sym!(self, xcb_render_pictdepth_visuals)
    }

    /// Returns the number of elements of the `visuals` field of a `xcb_render_pictdepth_t` struct.
    #[inline]
    pub unsafe fn xcb_render_pictdepth_visuals_length(
        &self,
        r: *const xcb_render_pictdepth_t,
    ) -> c_int {
        sym!(self, xcb_render_pictdepth_visuals_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_pictdepth_visuals_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_pictdepth_visuals_length(&self) -> bool {
        has_sym!(self, xcb_render_pictdepth_visuals_length)
    }

    /// Returns an iterator over the elements of the
    /// `visuals` field of a `xcb_render_pictdepth_t` struct.
    #[inline]
    pub unsafe fn xcb_render_pictdepth_visuals_iterator(
        &self,
        r: *const xcb_render_pictdepth_t,
    ) -> xcb_render_pictvisual_iterator_t {
        sym!(self, xcb_render_pictdepth_visuals_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_pictdepth_visuals_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_pictdepth_visuals_iterator(&self) -> bool {
        has_sym!(self, xcb_render_pictdepth_visuals_iterator)
    }

    /// Advances a `xcb_render_pictdepth_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_render_pictdepth_next(&self, i: *mut xcb_render_pictdepth_iterator_t) {
        sym!(self, xcb_render_pictdepth_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_pictdepth_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_pictdepth_next(&self) -> bool {
        has_sym!(self, xcb_render_pictdepth_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_render_pictdepth_iterator_t`.
    #[inline]
    pub unsafe fn xcb_render_pictdepth_end(
        &self,
        i: xcb_render_pictdepth_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_pictdepth_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_pictdepth_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_pictdepth_end(&self) -> bool {
        has_sym!(self, xcb_render_pictdepth_end)
    }

    /// Computes the size of a `xcb_render_pictscreen_t` object.
    #[inline]
    pub unsafe fn xcb_render_pictscreen_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_render_pictscreen_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_render_pictscreen_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_pictscreen_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_pictscreen_sizeof)
    }

    /// Returns the number of elements of the `depths` field of a `xcb_render_pictscreen_t` struct.
    #[inline]
    pub unsafe fn xcb_render_pictscreen_depths_length(
        &self,
        r: *const xcb_render_pictscreen_t,
    ) -> c_int {
        sym!(self, xcb_render_pictscreen_depths_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_pictscreen_depths_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_pictscreen_depths_length(&self) -> bool {
        has_sym!(self, xcb_render_pictscreen_depths_length)
    }

    /// Returns an iterator over the elements of the
    /// `depths` field of a `xcb_render_pictscreen_t` struct.
    #[inline]
    pub unsafe fn xcb_render_pictscreen_depths_iterator(
        &self,
        r: *const xcb_render_pictscreen_t,
    ) -> xcb_render_pictdepth_iterator_t {
        sym!(self, xcb_render_pictscreen_depths_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_pictscreen_depths_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_pictscreen_depths_iterator(&self) -> bool {
        has_sym!(self, xcb_render_pictscreen_depths_iterator)
    }

    /// Advances a `xcb_render_pictscreen_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_render_pictscreen_next(&self, i: *mut xcb_render_pictscreen_iterator_t) {
        sym!(self, xcb_render_pictscreen_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_pictscreen_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_pictscreen_next(&self) -> bool {
        has_sym!(self, xcb_render_pictscreen_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_render_pictscreen_iterator_t`.
    #[inline]
    pub unsafe fn xcb_render_pictscreen_end(
        &self,
        i: xcb_render_pictscreen_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_pictscreen_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_pictscreen_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_pictscreen_end(&self) -> bool {
        has_sym!(self, xcb_render_pictscreen_end)
    }

    /// Advances a `xcb_render_indexvalue_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_render_indexvalue_next(&self, i: *mut xcb_render_indexvalue_iterator_t) {
        sym!(self, xcb_render_indexvalue_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_indexvalue_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_indexvalue_next(&self) -> bool {
        has_sym!(self, xcb_render_indexvalue_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_render_indexvalue_iterator_t`.
    #[inline]
    pub unsafe fn xcb_render_indexvalue_end(
        &self,
        i: xcb_render_indexvalue_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_indexvalue_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_indexvalue_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_indexvalue_end(&self) -> bool {
        has_sym!(self, xcb_render_indexvalue_end)
    }

    /// Advances a `xcb_render_color_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_render_color_next(&self, i: *mut xcb_render_color_iterator_t) {
        sym!(self, xcb_render_color_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_color_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_color_next(&self) -> bool {
        has_sym!(self, xcb_render_color_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_render_color_iterator_t`.
    #[inline]
    pub unsafe fn xcb_render_color_end(
        &self,
        i: xcb_render_color_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_color_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_color_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_color_end(&self) -> bool {
        has_sym!(self, xcb_render_color_end)
    }

    /// Advances a `xcb_render_pointfix_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_render_pointfix_next(&self, i: *mut xcb_render_pointfix_iterator_t) {
        sym!(self, xcb_render_pointfix_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_pointfix_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_pointfix_next(&self) -> bool {
        has_sym!(self, xcb_render_pointfix_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_render_pointfix_iterator_t`.
    #[inline]
    pub unsafe fn xcb_render_pointfix_end(
        &self,
        i: xcb_render_pointfix_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_pointfix_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_pointfix_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_pointfix_end(&self) -> bool {
        has_sym!(self, xcb_render_pointfix_end)
    }

    /// Advances a `xcb_render_linefix_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_render_linefix_next(&self, i: *mut xcb_render_linefix_iterator_t) {
        sym!(self, xcb_render_linefix_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_linefix_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_linefix_next(&self) -> bool {
        has_sym!(self, xcb_render_linefix_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_render_linefix_iterator_t`.
    #[inline]
    pub unsafe fn xcb_render_linefix_end(
        &self,
        i: xcb_render_linefix_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_linefix_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_linefix_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_linefix_end(&self) -> bool {
        has_sym!(self, xcb_render_linefix_end)
    }

    /// Advances a `xcb_render_triangle_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_render_triangle_next(&self, i: *mut xcb_render_triangle_iterator_t) {
        sym!(self, xcb_render_triangle_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_triangle_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_triangle_next(&self) -> bool {
        has_sym!(self, xcb_render_triangle_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_render_triangle_iterator_t`.
    #[inline]
    pub unsafe fn xcb_render_triangle_end(
        &self,
        i: xcb_render_triangle_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_triangle_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_triangle_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_triangle_end(&self) -> bool {
        has_sym!(self, xcb_render_triangle_end)
    }

    /// Advances a `xcb_render_trapezoid_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_render_trapezoid_next(&self, i: *mut xcb_render_trapezoid_iterator_t) {
        sym!(self, xcb_render_trapezoid_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_trapezoid_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_trapezoid_next(&self) -> bool {
        has_sym!(self, xcb_render_trapezoid_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_render_trapezoid_iterator_t`.
    #[inline]
    pub unsafe fn xcb_render_trapezoid_end(
        &self,
        i: xcb_render_trapezoid_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_trapezoid_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_trapezoid_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_trapezoid_end(&self) -> bool {
        has_sym!(self, xcb_render_trapezoid_end)
    }

    /// Advances a `xcb_render_glyphinfo_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_render_glyphinfo_next(&self, i: *mut xcb_render_glyphinfo_iterator_t) {
        sym!(self, xcb_render_glyphinfo_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_glyphinfo_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_glyphinfo_next(&self) -> bool {
        has_sym!(self, xcb_render_glyphinfo_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_render_glyphinfo_iterator_t`.
    #[inline]
    pub unsafe fn xcb_render_glyphinfo_end(
        &self,
        i: xcb_render_glyphinfo_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_glyphinfo_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_glyphinfo_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_glyphinfo_end(&self) -> bool {
        has_sym!(self, xcb_render_glyphinfo_end)
    }

    /// Sends a `Render::QueryVersion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_render_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_render_query_version_reply`]: Self::xcb_render_query_version_reply
    #[inline]
    pub unsafe fn xcb_render_query_version(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u32,
        client_minor_version: u32,
    ) -> xcb_render_query_version_cookie_t {
        sym!(self, xcb_render_query_version)(c, client_major_version, client_minor_version)
    }

    /// Returns `true` iff the symbol `xcb_render_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_version(&self) -> bool {
        has_sym!(self, xcb_render_query_version)
    }

    /// Sends a `Render::QueryVersion` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_render_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_render_query_version_reply`]: Self::xcb_render_query_version_reply
    #[inline]
    pub unsafe fn xcb_render_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u32,
        client_minor_version: u32,
    ) -> xcb_render_query_version_cookie_t {
        sym!(self, xcb_render_query_version_unchecked)(
            c,
            client_major_version,
            client_minor_version,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_render_query_version_unchecked)
    }

    /// Waits for the reply to a `Render::QueryVersion` request.
    #[inline]
    pub unsafe fn xcb_render_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_render_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_render_query_version_reply_t {
        sym!(self, xcb_render_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_render_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_render_query_version_reply)
    }

    /// Computes the size of a `xcb_render_query_pict_formats_reply_t` object.
    #[inline]
    pub unsafe fn xcb_render_query_pict_formats_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_render_query_pict_formats_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_render_query_pict_formats_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_pict_formats_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_query_pict_formats_sizeof)
    }

    /// Sends a `Render::QueryPictFormats` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_render_query_pict_formats_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_render_query_pict_formats_reply`]: Self::xcb_render_query_pict_formats_reply
    #[inline]
    pub unsafe fn xcb_render_query_pict_formats(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_render_query_pict_formats_cookie_t {
        sym!(self, xcb_render_query_pict_formats)(c)
    }

    /// Returns `true` iff the symbol `xcb_render_query_pict_formats` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_pict_formats(&self) -> bool {
        has_sym!(self, xcb_render_query_pict_formats)
    }

    /// Sends a `Render::QueryPictFormats` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_render_query_pict_formats_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_render_query_pict_formats_reply`]: Self::xcb_render_query_pict_formats_reply
    #[inline]
    pub unsafe fn xcb_render_query_pict_formats_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_render_query_pict_formats_cookie_t {
        sym!(self, xcb_render_query_pict_formats_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_render_query_pict_formats_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_pict_formats_unchecked(&self) -> bool {
        has_sym!(self, xcb_render_query_pict_formats_unchecked)
    }

    /// Returns a pointer to the `formats` field of a `xcb_render_query_pict_formats_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_render_query_pict_formats_formats(
        &self,
        r: *const xcb_render_query_pict_formats_reply_t,
    ) -> *mut xcb_render_pictforminfo_t {
        sym!(self, xcb_render_query_pict_formats_formats)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_query_pict_formats_formats` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_pict_formats_formats(&self) -> bool {
        has_sym!(self, xcb_render_query_pict_formats_formats)
    }

    /// Returns the number of elements of the `formats` field of a `xcb_render_query_pict_formats_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_render_query_pict_formats_formats_length(
        &self,
        r: *const xcb_render_query_pict_formats_reply_t,
    ) -> c_int {
        sym!(self, xcb_render_query_pict_formats_formats_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_query_pict_formats_formats_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_pict_formats_formats_length(&self) -> bool {
        has_sym!(self, xcb_render_query_pict_formats_formats_length)
    }

    /// Returns an iterator over the elements of the
    /// `formats` field of a `xcb_render_query_pict_formats_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_render_query_pict_formats_formats_iterator(
        &self,
        r: *const xcb_render_query_pict_formats_reply_t,
    ) -> xcb_render_pictforminfo_iterator_t {
        sym!(self, xcb_render_query_pict_formats_formats_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_query_pict_formats_formats_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_pict_formats_formats_iterator(&self) -> bool {
        has_sym!(self, xcb_render_query_pict_formats_formats_iterator)
    }

    /// Returns the number of elements of the `screens` field of a `xcb_render_query_pict_formats_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_render_query_pict_formats_screens_length(
        &self,
        r: *const xcb_render_query_pict_formats_reply_t,
    ) -> c_int {
        sym!(self, xcb_render_query_pict_formats_screens_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_query_pict_formats_screens_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_pict_formats_screens_length(&self) -> bool {
        has_sym!(self, xcb_render_query_pict_formats_screens_length)
    }

    /// Returns an iterator over the elements of the
    /// `screens` field of a `xcb_render_query_pict_formats_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_render_query_pict_formats_screens_iterator(
        &self,
        r: *const xcb_render_query_pict_formats_reply_t,
    ) -> xcb_render_pictscreen_iterator_t {
        sym!(self, xcb_render_query_pict_formats_screens_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_query_pict_formats_screens_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_pict_formats_screens_iterator(&self) -> bool {
        has_sym!(self, xcb_render_query_pict_formats_screens_iterator)
    }

    /// Returns a pointer to the `subpixels` field of a `xcb_render_query_pict_formats_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_render_query_pict_formats_subpixels(
        &self,
        r: *const xcb_render_query_pict_formats_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_render_query_pict_formats_subpixels)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_query_pict_formats_subpixels` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_pict_formats_subpixels(&self) -> bool {
        has_sym!(self, xcb_render_query_pict_formats_subpixels)
    }

    /// Returns the number of elements of the `subpixels` field of a `xcb_render_query_pict_formats_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_render_query_pict_formats_subpixels_length(
        &self,
        r: *const xcb_render_query_pict_formats_reply_t,
    ) -> c_int {
        sym!(self, xcb_render_query_pict_formats_subpixels_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_query_pict_formats_subpixels_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_pict_formats_subpixels_length(&self) -> bool {
        has_sym!(self, xcb_render_query_pict_formats_subpixels_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `subpixels` field of a `xcb_render_query_pict_formats_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_render_query_pict_formats_subpixels_end(
        &self,
        r: *const xcb_render_query_pict_formats_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_query_pict_formats_subpixels_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_query_pict_formats_subpixels_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_pict_formats_subpixels_end(&self) -> bool {
        has_sym!(self, xcb_render_query_pict_formats_subpixels_end)
    }

    /// Waits for the reply to a `Render::QueryPictFormats` request.
    #[inline]
    pub unsafe fn xcb_render_query_pict_formats_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_render_query_pict_formats_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_render_query_pict_formats_reply_t {
        sym!(self, xcb_render_query_pict_formats_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_render_query_pict_formats_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_pict_formats_reply(&self) -> bool {
        has_sym!(self, xcb_render_query_pict_formats_reply)
    }

    /// Computes the size of a `xcb_render_query_pict_index_values_reply_t` object.
    #[inline]
    pub unsafe fn xcb_render_query_pict_index_values_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_render_query_pict_index_values_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_render_query_pict_index_values_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_pict_index_values_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_query_pict_index_values_sizeof)
    }

    /// Sends a `Render::QueryPictIndexValues` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_render_query_pict_index_values_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_render_query_pict_index_values_reply`]: Self::xcb_render_query_pict_index_values_reply
    #[inline]
    pub unsafe fn xcb_render_query_pict_index_values(
        &self,
        c: *mut xcb_connection_t,
        format: xcb_render_pictformat_t,
    ) -> xcb_render_query_pict_index_values_cookie_t {
        sym!(self, xcb_render_query_pict_index_values)(c, format)
    }

    /// Returns `true` iff the symbol `xcb_render_query_pict_index_values` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_pict_index_values(&self) -> bool {
        has_sym!(self, xcb_render_query_pict_index_values)
    }

    /// Sends a `Render::QueryPictIndexValues` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_render_query_pict_index_values_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_render_query_pict_index_values_reply`]: Self::xcb_render_query_pict_index_values_reply
    #[inline]
    pub unsafe fn xcb_render_query_pict_index_values_unchecked(
        &self,
        c: *mut xcb_connection_t,
        format: xcb_render_pictformat_t,
    ) -> xcb_render_query_pict_index_values_cookie_t {
        sym!(self, xcb_render_query_pict_index_values_unchecked)(c, format)
    }

    /// Returns `true` iff the symbol `xcb_render_query_pict_index_values_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_pict_index_values_unchecked(&self) -> bool {
        has_sym!(self, xcb_render_query_pict_index_values_unchecked)
    }

    /// Returns a pointer to the `values` field of a `xcb_render_query_pict_index_values_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_render_query_pict_index_values_values(
        &self,
        r: *const xcb_render_query_pict_index_values_reply_t,
    ) -> *mut xcb_render_indexvalue_t {
        sym!(self, xcb_render_query_pict_index_values_values)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_query_pict_index_values_values` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_pict_index_values_values(&self) -> bool {
        has_sym!(self, xcb_render_query_pict_index_values_values)
    }

    /// Returns the number of elements of the `values` field of a `xcb_render_query_pict_index_values_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_render_query_pict_index_values_values_length(
        &self,
        r: *const xcb_render_query_pict_index_values_reply_t,
    ) -> c_int {
        sym!(self, xcb_render_query_pict_index_values_values_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_query_pict_index_values_values_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_pict_index_values_values_length(&self) -> bool {
        has_sym!(self, xcb_render_query_pict_index_values_values_length)
    }

    /// Returns an iterator over the elements of the
    /// `values` field of a `xcb_render_query_pict_index_values_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_render_query_pict_index_values_values_iterator(
        &self,
        r: *const xcb_render_query_pict_index_values_reply_t,
    ) -> xcb_render_indexvalue_iterator_t {
        sym!(self, xcb_render_query_pict_index_values_values_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_query_pict_index_values_values_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_pict_index_values_values_iterator(&self) -> bool {
        has_sym!(self, xcb_render_query_pict_index_values_values_iterator)
    }

    /// Waits for the reply to a `Render::QueryPictIndexValues` request.
    #[inline]
    pub unsafe fn xcb_render_query_pict_index_values_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_render_query_pict_index_values_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_render_query_pict_index_values_reply_t {
        sym!(self, xcb_render_query_pict_index_values_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_render_query_pict_index_values_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_pict_index_values_reply(&self) -> bool {
        has_sym!(self, xcb_render_query_pict_index_values_reply)
    }

    /// Serializes a `xcb_render_create_picture_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_render_create_picture_value_list_serialize(
        &self,
        _buffer: *mut *mut c_void,
        value_mask: u32,
        _aux: *const xcb_render_create_picture_value_list_t,
    ) -> c_int {
        sym!(self, xcb_render_create_picture_value_list_serialize)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_render_create_picture_value_list_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_picture_value_list_serialize(&self) -> bool {
        has_sym!(self, xcb_render_create_picture_value_list_serialize)
    }

    /// Unpacks a `xcb_render_create_picture_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_render_create_picture_value_list_unpack(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
        _aux: *mut xcb_render_create_picture_value_list_t,
    ) -> c_int {
        sym!(self, xcb_render_create_picture_value_list_unpack)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_render_create_picture_value_list_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_picture_value_list_unpack(&self) -> bool {
        has_sym!(self, xcb_render_create_picture_value_list_unpack)
    }

    /// Computes the size of a `xcb_render_create_picture_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_render_create_picture_value_list_sizeof(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
    ) -> c_int {
        sym!(self, xcb_render_create_picture_value_list_sizeof)(_buffer, value_mask)
    }

    /// Returns `true` iff the symbol `xcb_render_create_picture_value_list_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_picture_value_list_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_create_picture_value_list_sizeof)
    }

    /// Computes the size of a `xcb_render_create_picture_request_t` object.
    #[inline]
    pub unsafe fn xcb_render_create_picture_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_render_create_picture_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_render_create_picture_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_picture_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_create_picture_sizeof)
    }

    /// Sends a `Render::CreatePicture` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    ///
    /// There is an auxiliary version of this function: [`xcb_render_create_picture_aux_checked`].
    ///
    /// [`xcb_render_create_picture_aux_checked`]: Self::xcb_render_create_picture_aux_checked
    #[inline]
    pub unsafe fn xcb_render_create_picture_checked(
        &self,
        c: *mut xcb_connection_t,
        pid: xcb_render_picture_t,
        drawable: xcb_drawable_t,
        format: xcb_render_pictformat_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_create_picture_checked)(
            c, pid, drawable, format, value_mask, value_list,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_create_picture_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_picture_checked(&self) -> bool {
        has_sym!(self, xcb_render_create_picture_checked)
    }

    /// Sends a `Render::CreatePicture` request (unchecked).
    ///
    /// There is an auxiliary version of this function: [`xcb_render_create_picture_aux`].
    ///
    /// [`xcb_render_create_picture_aux`]: Self::xcb_render_create_picture_aux
    #[inline]
    pub unsafe fn xcb_render_create_picture(
        &self,
        c: *mut xcb_connection_t,
        pid: xcb_render_picture_t,
        drawable: xcb_drawable_t,
        format: xcb_render_pictformat_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_create_picture)(c, pid, drawable, format, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_render_create_picture` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_picture(&self) -> bool {
        has_sym!(self, xcb_render_create_picture)
    }

    /// Sends a `Render::CreatePicture` request (checked) (aux).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_create_picture_aux_checked(
        &self,
        c: *mut xcb_connection_t,
        pid: xcb_render_picture_t,
        drawable: xcb_drawable_t,
        format: xcb_render_pictformat_t,
        value_mask: u32,
        value_list: *const xcb_render_create_picture_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_create_picture_aux_checked)(
            c, pid, drawable, format, value_mask, value_list,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_create_picture_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_picture_aux_checked(&self) -> bool {
        has_sym!(self, xcb_render_create_picture_aux_checked)
    }

    /// Sends a `Render::CreatePicture` request (unchecked) (aux).
    #[inline]
    pub unsafe fn xcb_render_create_picture_aux(
        &self,
        c: *mut xcb_connection_t,
        pid: xcb_render_picture_t,
        drawable: xcb_drawable_t,
        format: xcb_render_pictformat_t,
        value_mask: u32,
        value_list: *const xcb_render_create_picture_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_create_picture_aux)(c, pid, drawable, format, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_render_create_picture_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_picture_aux(&self) -> bool {
        has_sym!(self, xcb_render_create_picture_aux)
    }

    /// Returns a pointer to the `value_list` field of a `xcb_render_create_picture_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_create_picture_value_list(
        &self,
        r: *const xcb_render_create_picture_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_render_create_picture_value_list)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_create_picture_value_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_picture_value_list(&self) -> bool {
        has_sym!(self, xcb_render_create_picture_value_list)
    }

    /// Serializes a `xcb_render_change_picture_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_render_change_picture_value_list_serialize(
        &self,
        _buffer: *mut *mut c_void,
        value_mask: u32,
        _aux: *const xcb_render_change_picture_value_list_t,
    ) -> c_int {
        sym!(self, xcb_render_change_picture_value_list_serialize)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_render_change_picture_value_list_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_change_picture_value_list_serialize(&self) -> bool {
        has_sym!(self, xcb_render_change_picture_value_list_serialize)
    }

    /// Unpacks a `xcb_render_change_picture_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_render_change_picture_value_list_unpack(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
        _aux: *mut xcb_render_change_picture_value_list_t,
    ) -> c_int {
        sym!(self, xcb_render_change_picture_value_list_unpack)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_render_change_picture_value_list_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_change_picture_value_list_unpack(&self) -> bool {
        has_sym!(self, xcb_render_change_picture_value_list_unpack)
    }

    /// Computes the size of a `xcb_render_change_picture_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_render_change_picture_value_list_sizeof(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
    ) -> c_int {
        sym!(self, xcb_render_change_picture_value_list_sizeof)(_buffer, value_mask)
    }

    /// Returns `true` iff the symbol `xcb_render_change_picture_value_list_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_change_picture_value_list_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_change_picture_value_list_sizeof)
    }

    /// Computes the size of a `xcb_render_change_picture_request_t` object.
    #[inline]
    pub unsafe fn xcb_render_change_picture_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_render_change_picture_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_render_change_picture_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_change_picture_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_change_picture_sizeof)
    }

    /// Sends a `Render::ChangePicture` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    ///
    /// There is an auxiliary version of this function: [`xcb_render_change_picture_aux_checked`].
    ///
    /// [`xcb_render_change_picture_aux_checked`]: Self::xcb_render_change_picture_aux_checked
    #[inline]
    pub unsafe fn xcb_render_change_picture_checked(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_change_picture_checked)(c, picture, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_render_change_picture_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_change_picture_checked(&self) -> bool {
        has_sym!(self, xcb_render_change_picture_checked)
    }

    /// Sends a `Render::ChangePicture` request (unchecked).
    ///
    /// There is an auxiliary version of this function: [`xcb_render_change_picture_aux`].
    ///
    /// [`xcb_render_change_picture_aux`]: Self::xcb_render_change_picture_aux
    #[inline]
    pub unsafe fn xcb_render_change_picture(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_change_picture)(c, picture, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_render_change_picture` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_change_picture(&self) -> bool {
        has_sym!(self, xcb_render_change_picture)
    }

    /// Sends a `Render::ChangePicture` request (checked) (aux).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_change_picture_aux_checked(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        value_mask: u32,
        value_list: *const xcb_render_change_picture_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_change_picture_aux_checked)(c, picture, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_render_change_picture_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_change_picture_aux_checked(&self) -> bool {
        has_sym!(self, xcb_render_change_picture_aux_checked)
    }

    /// Sends a `Render::ChangePicture` request (unchecked) (aux).
    #[inline]
    pub unsafe fn xcb_render_change_picture_aux(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        value_mask: u32,
        value_list: *const xcb_render_change_picture_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_change_picture_aux)(c, picture, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_render_change_picture_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_change_picture_aux(&self) -> bool {
        has_sym!(self, xcb_render_change_picture_aux)
    }

    /// Returns a pointer to the `value_list` field of a `xcb_render_change_picture_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_change_picture_value_list(
        &self,
        r: *const xcb_render_change_picture_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_render_change_picture_value_list)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_change_picture_value_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_change_picture_value_list(&self) -> bool {
        has_sym!(self, xcb_render_change_picture_value_list)
    }

    /// Computes the size of a `xcb_render_set_picture_clip_rectangles_request_t` object.
    #[inline]
    pub unsafe fn xcb_render_set_picture_clip_rectangles_sizeof(
        &self,
        _buffer: *const c_void,
        rectangles_len: u32,
    ) -> c_int {
        sym!(self, xcb_render_set_picture_clip_rectangles_sizeof)(_buffer, rectangles_len)
    }

    /// Returns `true` iff the symbol `xcb_render_set_picture_clip_rectangles_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_set_picture_clip_rectangles_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_set_picture_clip_rectangles_sizeof)
    }

    /// Sends a `Render::SetPictureClipRectangles` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_set_picture_clip_rectangles_checked(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        clip_x_origin: i16,
        clip_y_origin: i16,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_set_picture_clip_rectangles_checked)(
            c,
            picture,
            clip_x_origin,
            clip_y_origin,
            rectangles_len,
            rectangles,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_set_picture_clip_rectangles_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_set_picture_clip_rectangles_checked(&self) -> bool {
        has_sym!(self, xcb_render_set_picture_clip_rectangles_checked)
    }

    /// Sends a `Render::SetPictureClipRectangles` request (unchecked).
    #[inline]
    pub unsafe fn xcb_render_set_picture_clip_rectangles(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        clip_x_origin: i16,
        clip_y_origin: i16,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_set_picture_clip_rectangles)(
            c,
            picture,
            clip_x_origin,
            clip_y_origin,
            rectangles_len,
            rectangles,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_set_picture_clip_rectangles` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_set_picture_clip_rectangles(&self) -> bool {
        has_sym!(self, xcb_render_set_picture_clip_rectangles)
    }

    /// Returns a pointer to the `rectangles` field of a `xcb_render_set_picture_clip_rectangles_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_set_picture_clip_rectangles_rectangles(
        &self,
        r: *const xcb_render_set_picture_clip_rectangles_request_t,
    ) -> *mut xcb_rectangle_t {
        sym!(self, xcb_render_set_picture_clip_rectangles_rectangles)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_set_picture_clip_rectangles_rectangles` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_set_picture_clip_rectangles_rectangles(&self) -> bool {
        has_sym!(self, xcb_render_set_picture_clip_rectangles_rectangles)
    }

    /// Returns the number of elements of the `rectangles` field of a `xcb_render_set_picture_clip_rectangles_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_set_picture_clip_rectangles_rectangles_length(
        &self,
        r: *const xcb_render_set_picture_clip_rectangles_request_t,
    ) -> c_int {
        sym!(
            self,
            xcb_render_set_picture_clip_rectangles_rectangles_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_render_set_picture_clip_rectangles_rectangles_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_set_picture_clip_rectangles_rectangles_length(&self) -> bool {
        has_sym!(
            self,
            xcb_render_set_picture_clip_rectangles_rectangles_length
        )
    }

    /// Returns an iterator over the elements of the
    /// `rectangles` field of a `xcb_render_set_picture_clip_rectangles_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_set_picture_clip_rectangles_rectangles_iterator(
        &self,
        r: *const xcb_render_set_picture_clip_rectangles_request_t,
    ) -> xcb_rectangle_iterator_t {
        sym!(
            self,
            xcb_render_set_picture_clip_rectangles_rectangles_iterator
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_render_set_picture_clip_rectangles_rectangles_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_set_picture_clip_rectangles_rectangles_iterator(&self) -> bool {
        has_sym!(
            self,
            xcb_render_set_picture_clip_rectangles_rectangles_iterator
        )
    }

    /// Sends a `Render::FreePicture` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_free_picture_checked(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_free_picture_checked)(c, picture)
    }

    /// Returns `true` iff the symbol `xcb_render_free_picture_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_free_picture_checked(&self) -> bool {
        has_sym!(self, xcb_render_free_picture_checked)
    }

    /// Sends a `Render::FreePicture` request (unchecked).
    #[inline]
    pub unsafe fn xcb_render_free_picture(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_free_picture)(c, picture)
    }

    /// Returns `true` iff the symbol `xcb_render_free_picture` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_free_picture(&self) -> bool {
        has_sym!(self, xcb_render_free_picture)
    }

    /// Sends a `Render::Composite` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_composite_checked(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        mask: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        src_x: i16,
        src_y: i16,
        mask_x: i16,
        mask_y: i16,
        dst_x: i16,
        dst_y: i16,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_composite_checked)(
            c, op, src, mask, dst, src_x, src_y, mask_x, mask_y, dst_x, dst_y, width, height,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_composite_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_composite_checked(&self) -> bool {
        has_sym!(self, xcb_render_composite_checked)
    }

    /// Sends a `Render::Composite` request (unchecked).
    #[inline]
    pub unsafe fn xcb_render_composite(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        mask: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        src_x: i16,
        src_y: i16,
        mask_x: i16,
        mask_y: i16,
        dst_x: i16,
        dst_y: i16,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_composite)(
            c, op, src, mask, dst, src_x, src_y, mask_x, mask_y, dst_x, dst_y, width, height,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_composite` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_composite(&self) -> bool {
        has_sym!(self, xcb_render_composite)
    }

    /// Computes the size of a `xcb_render_trapezoids_request_t` object.
    #[inline]
    pub unsafe fn xcb_render_trapezoids_sizeof(
        &self,
        _buffer: *const c_void,
        traps_len: u32,
    ) -> c_int {
        sym!(self, xcb_render_trapezoids_sizeof)(_buffer, traps_len)
    }

    /// Returns `true` iff the symbol `xcb_render_trapezoids_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_trapezoids_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_trapezoids_sizeof)
    }

    /// Sends a `Render::Trapezoids` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_trapezoids_checked(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        src_x: i16,
        src_y: i16,
        traps_len: u32,
        traps: *const xcb_render_trapezoid_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_trapezoids_checked)(
            c,
            op,
            src,
            dst,
            mask_format,
            src_x,
            src_y,
            traps_len,
            traps,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_trapezoids_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_trapezoids_checked(&self) -> bool {
        has_sym!(self, xcb_render_trapezoids_checked)
    }

    /// Sends a `Render::Trapezoids` request (unchecked).
    #[inline]
    pub unsafe fn xcb_render_trapezoids(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        src_x: i16,
        src_y: i16,
        traps_len: u32,
        traps: *const xcb_render_trapezoid_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_trapezoids)(
            c,
            op,
            src,
            dst,
            mask_format,
            src_x,
            src_y,
            traps_len,
            traps,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_trapezoids` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_trapezoids(&self) -> bool {
        has_sym!(self, xcb_render_trapezoids)
    }

    /// Returns a pointer to the `traps` field of a `xcb_render_trapezoids_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_trapezoids_traps(
        &self,
        r: *const xcb_render_trapezoids_request_t,
    ) -> *mut xcb_render_trapezoid_t {
        sym!(self, xcb_render_trapezoids_traps)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_trapezoids_traps` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_trapezoids_traps(&self) -> bool {
        has_sym!(self, xcb_render_trapezoids_traps)
    }

    /// Returns the number of elements of the `traps` field of a `xcb_render_trapezoids_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_trapezoids_traps_length(
        &self,
        r: *const xcb_render_trapezoids_request_t,
    ) -> c_int {
        sym!(self, xcb_render_trapezoids_traps_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_trapezoids_traps_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_trapezoids_traps_length(&self) -> bool {
        has_sym!(self, xcb_render_trapezoids_traps_length)
    }

    /// Returns an iterator over the elements of the
    /// `traps` field of a `xcb_render_trapezoids_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_trapezoids_traps_iterator(
        &self,
        r: *const xcb_render_trapezoids_request_t,
    ) -> xcb_render_trapezoid_iterator_t {
        sym!(self, xcb_render_trapezoids_traps_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_trapezoids_traps_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_trapezoids_traps_iterator(&self) -> bool {
        has_sym!(self, xcb_render_trapezoids_traps_iterator)
    }

    /// Computes the size of a `xcb_render_triangles_request_t` object.
    #[inline]
    pub unsafe fn xcb_render_triangles_sizeof(
        &self,
        _buffer: *const c_void,
        triangles_len: u32,
    ) -> c_int {
        sym!(self, xcb_render_triangles_sizeof)(_buffer, triangles_len)
    }

    /// Returns `true` iff the symbol `xcb_render_triangles_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_triangles_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_triangles_sizeof)
    }

    /// Sends a `Render::Triangles` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_triangles_checked(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        src_x: i16,
        src_y: i16,
        triangles_len: u32,
        triangles: *const xcb_render_triangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_triangles_checked)(
            c,
            op,
            src,
            dst,
            mask_format,
            src_x,
            src_y,
            triangles_len,
            triangles,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_triangles_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_triangles_checked(&self) -> bool {
        has_sym!(self, xcb_render_triangles_checked)
    }

    /// Sends a `Render::Triangles` request (unchecked).
    #[inline]
    pub unsafe fn xcb_render_triangles(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        src_x: i16,
        src_y: i16,
        triangles_len: u32,
        triangles: *const xcb_render_triangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_triangles)(
            c,
            op,
            src,
            dst,
            mask_format,
            src_x,
            src_y,
            triangles_len,
            triangles,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_triangles` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_triangles(&self) -> bool {
        has_sym!(self, xcb_render_triangles)
    }

    /// Returns a pointer to the `triangles` field of a `xcb_render_triangles_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_triangles_triangles(
        &self,
        r: *const xcb_render_triangles_request_t,
    ) -> *mut xcb_render_triangle_t {
        sym!(self, xcb_render_triangles_triangles)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_triangles_triangles` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_triangles_triangles(&self) -> bool {
        has_sym!(self, xcb_render_triangles_triangles)
    }

    /// Returns the number of elements of the `triangles` field of a `xcb_render_triangles_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_triangles_triangles_length(
        &self,
        r: *const xcb_render_triangles_request_t,
    ) -> c_int {
        sym!(self, xcb_render_triangles_triangles_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_triangles_triangles_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_triangles_triangles_length(&self) -> bool {
        has_sym!(self, xcb_render_triangles_triangles_length)
    }

    /// Returns an iterator over the elements of the
    /// `triangles` field of a `xcb_render_triangles_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_triangles_triangles_iterator(
        &self,
        r: *const xcb_render_triangles_request_t,
    ) -> xcb_render_triangle_iterator_t {
        sym!(self, xcb_render_triangles_triangles_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_triangles_triangles_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_triangles_triangles_iterator(&self) -> bool {
        has_sym!(self, xcb_render_triangles_triangles_iterator)
    }

    /// Computes the size of a `xcb_render_tri_strip_request_t` object.
    #[inline]
    pub unsafe fn xcb_render_tri_strip_sizeof(
        &self,
        _buffer: *const c_void,
        points_len: u32,
    ) -> c_int {
        sym!(self, xcb_render_tri_strip_sizeof)(_buffer, points_len)
    }

    /// Returns `true` iff the symbol `xcb_render_tri_strip_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_tri_strip_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_tri_strip_sizeof)
    }

    /// Sends a `Render::TriStrip` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_tri_strip_checked(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        src_x: i16,
        src_y: i16,
        points_len: u32,
        points: *const xcb_render_pointfix_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_tri_strip_checked)(
            c,
            op,
            src,
            dst,
            mask_format,
            src_x,
            src_y,
            points_len,
            points,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_tri_strip_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_tri_strip_checked(&self) -> bool {
        has_sym!(self, xcb_render_tri_strip_checked)
    }

    /// Sends a `Render::TriStrip` request (unchecked).
    #[inline]
    pub unsafe fn xcb_render_tri_strip(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        src_x: i16,
        src_y: i16,
        points_len: u32,
        points: *const xcb_render_pointfix_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_tri_strip)(
            c,
            op,
            src,
            dst,
            mask_format,
            src_x,
            src_y,
            points_len,
            points,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_tri_strip` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_tri_strip(&self) -> bool {
        has_sym!(self, xcb_render_tri_strip)
    }

    /// Returns a pointer to the `points` field of a `xcb_render_tri_strip_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_tri_strip_points(
        &self,
        r: *const xcb_render_tri_strip_request_t,
    ) -> *mut xcb_render_pointfix_t {
        sym!(self, xcb_render_tri_strip_points)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_tri_strip_points` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_tri_strip_points(&self) -> bool {
        has_sym!(self, xcb_render_tri_strip_points)
    }

    /// Returns the number of elements of the `points` field of a `xcb_render_tri_strip_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_tri_strip_points_length(
        &self,
        r: *const xcb_render_tri_strip_request_t,
    ) -> c_int {
        sym!(self, xcb_render_tri_strip_points_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_tri_strip_points_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_tri_strip_points_length(&self) -> bool {
        has_sym!(self, xcb_render_tri_strip_points_length)
    }

    /// Returns an iterator over the elements of the
    /// `points` field of a `xcb_render_tri_strip_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_tri_strip_points_iterator(
        &self,
        r: *const xcb_render_tri_strip_request_t,
    ) -> xcb_render_pointfix_iterator_t {
        sym!(self, xcb_render_tri_strip_points_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_tri_strip_points_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_tri_strip_points_iterator(&self) -> bool {
        has_sym!(self, xcb_render_tri_strip_points_iterator)
    }

    /// Computes the size of a `xcb_render_tri_fan_request_t` object.
    #[inline]
    pub unsafe fn xcb_render_tri_fan_sizeof(
        &self,
        _buffer: *const c_void,
        points_len: u32,
    ) -> c_int {
        sym!(self, xcb_render_tri_fan_sizeof)(_buffer, points_len)
    }

    /// Returns `true` iff the symbol `xcb_render_tri_fan_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_tri_fan_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_tri_fan_sizeof)
    }

    /// Sends a `Render::TriFan` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_tri_fan_checked(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        src_x: i16,
        src_y: i16,
        points_len: u32,
        points: *const xcb_render_pointfix_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_tri_fan_checked)(
            c,
            op,
            src,
            dst,
            mask_format,
            src_x,
            src_y,
            points_len,
            points,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_tri_fan_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_tri_fan_checked(&self) -> bool {
        has_sym!(self, xcb_render_tri_fan_checked)
    }

    /// Sends a `Render::TriFan` request (unchecked).
    #[inline]
    pub unsafe fn xcb_render_tri_fan(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        src_x: i16,
        src_y: i16,
        points_len: u32,
        points: *const xcb_render_pointfix_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_tri_fan)(
            c,
            op,
            src,
            dst,
            mask_format,
            src_x,
            src_y,
            points_len,
            points,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_tri_fan` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_tri_fan(&self) -> bool {
        has_sym!(self, xcb_render_tri_fan)
    }

    /// Returns a pointer to the `points` field of a `xcb_render_tri_fan_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_tri_fan_points(
        &self,
        r: *const xcb_render_tri_fan_request_t,
    ) -> *mut xcb_render_pointfix_t {
        sym!(self, xcb_render_tri_fan_points)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_tri_fan_points` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_tri_fan_points(&self) -> bool {
        has_sym!(self, xcb_render_tri_fan_points)
    }

    /// Returns the number of elements of the `points` field of a `xcb_render_tri_fan_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_tri_fan_points_length(
        &self,
        r: *const xcb_render_tri_fan_request_t,
    ) -> c_int {
        sym!(self, xcb_render_tri_fan_points_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_tri_fan_points_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_tri_fan_points_length(&self) -> bool {
        has_sym!(self, xcb_render_tri_fan_points_length)
    }

    /// Returns an iterator over the elements of the
    /// `points` field of a `xcb_render_tri_fan_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_tri_fan_points_iterator(
        &self,
        r: *const xcb_render_tri_fan_request_t,
    ) -> xcb_render_pointfix_iterator_t {
        sym!(self, xcb_render_tri_fan_points_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_tri_fan_points_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_tri_fan_points_iterator(&self) -> bool {
        has_sym!(self, xcb_render_tri_fan_points_iterator)
    }

    /// Sends a `Render::CreateGlyphSet` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_create_glyph_set_checked(
        &self,
        c: *mut xcb_connection_t,
        gsid: xcb_render_glyphset_t,
        format: xcb_render_pictformat_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_create_glyph_set_checked)(c, gsid, format)
    }

    /// Returns `true` iff the symbol `xcb_render_create_glyph_set_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_glyph_set_checked(&self) -> bool {
        has_sym!(self, xcb_render_create_glyph_set_checked)
    }

    /// Sends a `Render::CreateGlyphSet` request (unchecked).
    #[inline]
    pub unsafe fn xcb_render_create_glyph_set(
        &self,
        c: *mut xcb_connection_t,
        gsid: xcb_render_glyphset_t,
        format: xcb_render_pictformat_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_create_glyph_set)(c, gsid, format)
    }

    /// Returns `true` iff the symbol `xcb_render_create_glyph_set` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_glyph_set(&self) -> bool {
        has_sym!(self, xcb_render_create_glyph_set)
    }

    /// Sends a `Render::ReferenceGlyphSet` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_reference_glyph_set_checked(
        &self,
        c: *mut xcb_connection_t,
        gsid: xcb_render_glyphset_t,
        existing: xcb_render_glyphset_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_reference_glyph_set_checked)(c, gsid, existing)
    }

    /// Returns `true` iff the symbol `xcb_render_reference_glyph_set_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_reference_glyph_set_checked(&self) -> bool {
        has_sym!(self, xcb_render_reference_glyph_set_checked)
    }

    /// Sends a `Render::ReferenceGlyphSet` request (unchecked).
    #[inline]
    pub unsafe fn xcb_render_reference_glyph_set(
        &self,
        c: *mut xcb_connection_t,
        gsid: xcb_render_glyphset_t,
        existing: xcb_render_glyphset_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_reference_glyph_set)(c, gsid, existing)
    }

    /// Returns `true` iff the symbol `xcb_render_reference_glyph_set` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_reference_glyph_set(&self) -> bool {
        has_sym!(self, xcb_render_reference_glyph_set)
    }

    /// Sends a `Render::FreeGlyphSet` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_free_glyph_set_checked(
        &self,
        c: *mut xcb_connection_t,
        glyphset: xcb_render_glyphset_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_free_glyph_set_checked)(c, glyphset)
    }

    /// Returns `true` iff the symbol `xcb_render_free_glyph_set_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_free_glyph_set_checked(&self) -> bool {
        has_sym!(self, xcb_render_free_glyph_set_checked)
    }

    /// Sends a `Render::FreeGlyphSet` request (unchecked).
    #[inline]
    pub unsafe fn xcb_render_free_glyph_set(
        &self,
        c: *mut xcb_connection_t,
        glyphset: xcb_render_glyphset_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_free_glyph_set)(c, glyphset)
    }

    /// Returns `true` iff the symbol `xcb_render_free_glyph_set` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_free_glyph_set(&self) -> bool {
        has_sym!(self, xcb_render_free_glyph_set)
    }

    /// Computes the size of a `xcb_render_add_glyphs_request_t` object.
    #[inline]
    pub unsafe fn xcb_render_add_glyphs_sizeof(
        &self,
        _buffer: *const c_void,
        data_len: u32,
    ) -> c_int {
        sym!(self, xcb_render_add_glyphs_sizeof)(_buffer, data_len)
    }

    /// Returns `true` iff the symbol `xcb_render_add_glyphs_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_add_glyphs_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_add_glyphs_sizeof)
    }

    /// Sends a `Render::AddGlyphs` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_add_glyphs_checked(
        &self,
        c: *mut xcb_connection_t,
        glyphset: xcb_render_glyphset_t,
        glyphs_len: u32,
        glyphids: *const u32,
        glyphs: *const xcb_render_glyphinfo_t,
        data_len: u32,
        data: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_add_glyphs_checked)(
            c, glyphset, glyphs_len, glyphids, glyphs, data_len, data,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_add_glyphs_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_add_glyphs_checked(&self) -> bool {
        has_sym!(self, xcb_render_add_glyphs_checked)
    }

    /// Sends a `Render::AddGlyphs` request (unchecked).
    #[inline]
    pub unsafe fn xcb_render_add_glyphs(
        &self,
        c: *mut xcb_connection_t,
        glyphset: xcb_render_glyphset_t,
        glyphs_len: u32,
        glyphids: *const u32,
        glyphs: *const xcb_render_glyphinfo_t,
        data_len: u32,
        data: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_add_glyphs)(c, glyphset, glyphs_len, glyphids, glyphs, data_len, data)
    }

    /// Returns `true` iff the symbol `xcb_render_add_glyphs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_add_glyphs(&self) -> bool {
        has_sym!(self, xcb_render_add_glyphs)
    }

    /// Returns a pointer to the `glyphids` field of a `xcb_render_add_glyphs_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_add_glyphs_glyphids(
        &self,
        r: *const xcb_render_add_glyphs_request_t,
    ) -> *mut u32 {
        sym!(self, xcb_render_add_glyphs_glyphids)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_add_glyphs_glyphids` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_add_glyphs_glyphids(&self) -> bool {
        has_sym!(self, xcb_render_add_glyphs_glyphids)
    }

    /// Returns the number of elements of the `glyphids` field of a `xcb_render_add_glyphs_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_add_glyphs_glyphids_length(
        &self,
        r: *const xcb_render_add_glyphs_request_t,
    ) -> c_int {
        sym!(self, xcb_render_add_glyphs_glyphids_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_add_glyphs_glyphids_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_add_glyphs_glyphids_length(&self) -> bool {
        has_sym!(self, xcb_render_add_glyphs_glyphids_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `glyphids` field of a `xcb_render_add_glyphs_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_add_glyphs_glyphids_end(
        &self,
        r: *const xcb_render_add_glyphs_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_add_glyphs_glyphids_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_add_glyphs_glyphids_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_add_glyphs_glyphids_end(&self) -> bool {
        has_sym!(self, xcb_render_add_glyphs_glyphids_end)
    }

    /// Returns a pointer to the `glyphs` field of a `xcb_render_add_glyphs_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_add_glyphs_glyphs(
        &self,
        r: *const xcb_render_add_glyphs_request_t,
    ) -> *mut xcb_render_glyphinfo_t {
        sym!(self, xcb_render_add_glyphs_glyphs)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_add_glyphs_glyphs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_add_glyphs_glyphs(&self) -> bool {
        has_sym!(self, xcb_render_add_glyphs_glyphs)
    }

    /// Returns the number of elements of the `glyphs` field of a `xcb_render_add_glyphs_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_add_glyphs_glyphs_length(
        &self,
        r: *const xcb_render_add_glyphs_request_t,
    ) -> c_int {
        sym!(self, xcb_render_add_glyphs_glyphs_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_add_glyphs_glyphs_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_add_glyphs_glyphs_length(&self) -> bool {
        has_sym!(self, xcb_render_add_glyphs_glyphs_length)
    }

    /// Returns an iterator over the elements of the
    /// `glyphs` field of a `xcb_render_add_glyphs_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_add_glyphs_glyphs_iterator(
        &self,
        r: *const xcb_render_add_glyphs_request_t,
    ) -> xcb_render_glyphinfo_iterator_t {
        sym!(self, xcb_render_add_glyphs_glyphs_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_add_glyphs_glyphs_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_add_glyphs_glyphs_iterator(&self) -> bool {
        has_sym!(self, xcb_render_add_glyphs_glyphs_iterator)
    }

    /// Returns a pointer to the `data` field of a `xcb_render_add_glyphs_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_add_glyphs_data(
        &self,
        r: *const xcb_render_add_glyphs_request_t,
    ) -> *mut u8 {
        sym!(self, xcb_render_add_glyphs_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_add_glyphs_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_add_glyphs_data(&self) -> bool {
        has_sym!(self, xcb_render_add_glyphs_data)
    }

    /// Returns the number of elements of the `data` field of a `xcb_render_add_glyphs_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_add_glyphs_data_length(
        &self,
        r: *const xcb_render_add_glyphs_request_t,
    ) -> c_int {
        sym!(self, xcb_render_add_glyphs_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_add_glyphs_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_add_glyphs_data_length(&self) -> bool {
        has_sym!(self, xcb_render_add_glyphs_data_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `data` field of a `xcb_render_add_glyphs_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_add_glyphs_data_end(
        &self,
        r: *const xcb_render_add_glyphs_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_add_glyphs_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_add_glyphs_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_add_glyphs_data_end(&self) -> bool {
        has_sym!(self, xcb_render_add_glyphs_data_end)
    }

    /// Computes the size of a `xcb_render_free_glyphs_request_t` object.
    #[inline]
    pub unsafe fn xcb_render_free_glyphs_sizeof(
        &self,
        _buffer: *const c_void,
        glyphs_len: u32,
    ) -> c_int {
        sym!(self, xcb_render_free_glyphs_sizeof)(_buffer, glyphs_len)
    }

    /// Returns `true` iff the symbol `xcb_render_free_glyphs_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_free_glyphs_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_free_glyphs_sizeof)
    }

    /// Sends a `Render::FreeGlyphs` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_free_glyphs_checked(
        &self,
        c: *mut xcb_connection_t,
        glyphset: xcb_render_glyphset_t,
        glyphs_len: u32,
        glyphs: *const xcb_render_glyph_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_free_glyphs_checked)(c, glyphset, glyphs_len, glyphs)
    }

    /// Returns `true` iff the symbol `xcb_render_free_glyphs_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_free_glyphs_checked(&self) -> bool {
        has_sym!(self, xcb_render_free_glyphs_checked)
    }

    /// Sends a `Render::FreeGlyphs` request (unchecked).
    #[inline]
    pub unsafe fn xcb_render_free_glyphs(
        &self,
        c: *mut xcb_connection_t,
        glyphset: xcb_render_glyphset_t,
        glyphs_len: u32,
        glyphs: *const xcb_render_glyph_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_free_glyphs)(c, glyphset, glyphs_len, glyphs)
    }

    /// Returns `true` iff the symbol `xcb_render_free_glyphs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_free_glyphs(&self) -> bool {
        has_sym!(self, xcb_render_free_glyphs)
    }

    /// Returns a pointer to the `glyphs` field of a `xcb_render_free_glyphs_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_free_glyphs_glyphs(
        &self,
        r: *const xcb_render_free_glyphs_request_t,
    ) -> *mut xcb_render_glyph_t {
        sym!(self, xcb_render_free_glyphs_glyphs)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_free_glyphs_glyphs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_free_glyphs_glyphs(&self) -> bool {
        has_sym!(self, xcb_render_free_glyphs_glyphs)
    }

    /// Returns the number of elements of the `glyphs` field of a `xcb_render_free_glyphs_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_free_glyphs_glyphs_length(
        &self,
        r: *const xcb_render_free_glyphs_request_t,
    ) -> c_int {
        sym!(self, xcb_render_free_glyphs_glyphs_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_free_glyphs_glyphs_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_free_glyphs_glyphs_length(&self) -> bool {
        has_sym!(self, xcb_render_free_glyphs_glyphs_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `glyphs` field of a `xcb_render_free_glyphs_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_free_glyphs_glyphs_end(
        &self,
        r: *const xcb_render_free_glyphs_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_free_glyphs_glyphs_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_free_glyphs_glyphs_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_free_glyphs_glyphs_end(&self) -> bool {
        has_sym!(self, xcb_render_free_glyphs_glyphs_end)
    }

    /// Computes the size of a `xcb_render_composite_glyphs_8_request_t` object.
    #[inline]
    pub unsafe fn xcb_render_composite_glyphs_8_sizeof(
        &self,
        _buffer: *const c_void,
        glyphcmds_len: u32,
    ) -> c_int {
        sym!(self, xcb_render_composite_glyphs_8_sizeof)(_buffer, glyphcmds_len)
    }

    /// Returns `true` iff the symbol `xcb_render_composite_glyphs_8_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_composite_glyphs_8_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_composite_glyphs_8_sizeof)
    }

    /// Sends a `Render::CompositeGlyphs8` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_composite_glyphs_8_checked(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        glyphset: xcb_render_glyphset_t,
        src_x: i16,
        src_y: i16,
        glyphcmds_len: u32,
        glyphcmds: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_composite_glyphs_8_checked)(
            c,
            op,
            src,
            dst,
            mask_format,
            glyphset,
            src_x,
            src_y,
            glyphcmds_len,
            glyphcmds,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_composite_glyphs_8_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_composite_glyphs_8_checked(&self) -> bool {
        has_sym!(self, xcb_render_composite_glyphs_8_checked)
    }

    /// Sends a `Render::CompositeGlyphs8` request (unchecked).
    #[inline]
    pub unsafe fn xcb_render_composite_glyphs_8(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        glyphset: xcb_render_glyphset_t,
        src_x: i16,
        src_y: i16,
        glyphcmds_len: u32,
        glyphcmds: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_composite_glyphs_8)(
            c,
            op,
            src,
            dst,
            mask_format,
            glyphset,
            src_x,
            src_y,
            glyphcmds_len,
            glyphcmds,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_composite_glyphs_8` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_composite_glyphs_8(&self) -> bool {
        has_sym!(self, xcb_render_composite_glyphs_8)
    }

    /// Returns a pointer to the `glyphcmds` field of a `xcb_render_composite_glyphs_8_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_composite_glyphs_8_glyphcmds(
        &self,
        r: *const xcb_render_composite_glyphs_8_request_t,
    ) -> *mut u8 {
        sym!(self, xcb_render_composite_glyphs_8_glyphcmds)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_composite_glyphs_8_glyphcmds` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_composite_glyphs_8_glyphcmds(&self) -> bool {
        has_sym!(self, xcb_render_composite_glyphs_8_glyphcmds)
    }

    /// Returns the number of elements of the `glyphcmds` field of a `xcb_render_composite_glyphs_8_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_composite_glyphs_8_glyphcmds_length(
        &self,
        r: *const xcb_render_composite_glyphs_8_request_t,
    ) -> c_int {
        sym!(self, xcb_render_composite_glyphs_8_glyphcmds_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_composite_glyphs_8_glyphcmds_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_composite_glyphs_8_glyphcmds_length(&self) -> bool {
        has_sym!(self, xcb_render_composite_glyphs_8_glyphcmds_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `glyphcmds` field of a `xcb_render_composite_glyphs_8_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_composite_glyphs_8_glyphcmds_end(
        &self,
        r: *const xcb_render_composite_glyphs_8_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_composite_glyphs_8_glyphcmds_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_composite_glyphs_8_glyphcmds_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_composite_glyphs_8_glyphcmds_end(&self) -> bool {
        has_sym!(self, xcb_render_composite_glyphs_8_glyphcmds_end)
    }

    /// Computes the size of a `xcb_render_composite_glyphs_16_request_t` object.
    #[inline]
    pub unsafe fn xcb_render_composite_glyphs_16_sizeof(
        &self,
        _buffer: *const c_void,
        glyphcmds_len: u32,
    ) -> c_int {
        sym!(self, xcb_render_composite_glyphs_16_sizeof)(_buffer, glyphcmds_len)
    }

    /// Returns `true` iff the symbol `xcb_render_composite_glyphs_16_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_composite_glyphs_16_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_composite_glyphs_16_sizeof)
    }

    /// Sends a `Render::CompositeGlyphs16` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_composite_glyphs_16_checked(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        glyphset: xcb_render_glyphset_t,
        src_x: i16,
        src_y: i16,
        glyphcmds_len: u32,
        glyphcmds: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_composite_glyphs_16_checked)(
            c,
            op,
            src,
            dst,
            mask_format,
            glyphset,
            src_x,
            src_y,
            glyphcmds_len,
            glyphcmds,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_composite_glyphs_16_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_composite_glyphs_16_checked(&self) -> bool {
        has_sym!(self, xcb_render_composite_glyphs_16_checked)
    }

    /// Sends a `Render::CompositeGlyphs16` request (unchecked).
    #[inline]
    pub unsafe fn xcb_render_composite_glyphs_16(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        glyphset: xcb_render_glyphset_t,
        src_x: i16,
        src_y: i16,
        glyphcmds_len: u32,
        glyphcmds: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_composite_glyphs_16)(
            c,
            op,
            src,
            dst,
            mask_format,
            glyphset,
            src_x,
            src_y,
            glyphcmds_len,
            glyphcmds,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_composite_glyphs_16` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_composite_glyphs_16(&self) -> bool {
        has_sym!(self, xcb_render_composite_glyphs_16)
    }

    /// Returns a pointer to the `glyphcmds` field of a `xcb_render_composite_glyphs_16_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_composite_glyphs_16_glyphcmds(
        &self,
        r: *const xcb_render_composite_glyphs_16_request_t,
    ) -> *mut u8 {
        sym!(self, xcb_render_composite_glyphs_16_glyphcmds)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_composite_glyphs_16_glyphcmds` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_composite_glyphs_16_glyphcmds(&self) -> bool {
        has_sym!(self, xcb_render_composite_glyphs_16_glyphcmds)
    }

    /// Returns the number of elements of the `glyphcmds` field of a `xcb_render_composite_glyphs_16_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_composite_glyphs_16_glyphcmds_length(
        &self,
        r: *const xcb_render_composite_glyphs_16_request_t,
    ) -> c_int {
        sym!(self, xcb_render_composite_glyphs_16_glyphcmds_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_composite_glyphs_16_glyphcmds_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_composite_glyphs_16_glyphcmds_length(&self) -> bool {
        has_sym!(self, xcb_render_composite_glyphs_16_glyphcmds_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `glyphcmds` field of a `xcb_render_composite_glyphs_16_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_composite_glyphs_16_glyphcmds_end(
        &self,
        r: *const xcb_render_composite_glyphs_16_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_composite_glyphs_16_glyphcmds_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_composite_glyphs_16_glyphcmds_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_composite_glyphs_16_glyphcmds_end(&self) -> bool {
        has_sym!(self, xcb_render_composite_glyphs_16_glyphcmds_end)
    }

    /// Computes the size of a `xcb_render_composite_glyphs_32_request_t` object.
    #[inline]
    pub unsafe fn xcb_render_composite_glyphs_32_sizeof(
        &self,
        _buffer: *const c_void,
        glyphcmds_len: u32,
    ) -> c_int {
        sym!(self, xcb_render_composite_glyphs_32_sizeof)(_buffer, glyphcmds_len)
    }

    /// Returns `true` iff the symbol `xcb_render_composite_glyphs_32_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_composite_glyphs_32_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_composite_glyphs_32_sizeof)
    }

    /// Sends a `Render::CompositeGlyphs32` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_composite_glyphs_32_checked(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        glyphset: xcb_render_glyphset_t,
        src_x: i16,
        src_y: i16,
        glyphcmds_len: u32,
        glyphcmds: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_composite_glyphs_32_checked)(
            c,
            op,
            src,
            dst,
            mask_format,
            glyphset,
            src_x,
            src_y,
            glyphcmds_len,
            glyphcmds,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_composite_glyphs_32_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_composite_glyphs_32_checked(&self) -> bool {
        has_sym!(self, xcb_render_composite_glyphs_32_checked)
    }

    /// Sends a `Render::CompositeGlyphs32` request (unchecked).
    #[inline]
    pub unsafe fn xcb_render_composite_glyphs_32(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        src: xcb_render_picture_t,
        dst: xcb_render_picture_t,
        mask_format: xcb_render_pictformat_t,
        glyphset: xcb_render_glyphset_t,
        src_x: i16,
        src_y: i16,
        glyphcmds_len: u32,
        glyphcmds: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_composite_glyphs_32)(
            c,
            op,
            src,
            dst,
            mask_format,
            glyphset,
            src_x,
            src_y,
            glyphcmds_len,
            glyphcmds,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_composite_glyphs_32` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_composite_glyphs_32(&self) -> bool {
        has_sym!(self, xcb_render_composite_glyphs_32)
    }

    /// Returns a pointer to the `glyphcmds` field of a `xcb_render_composite_glyphs_32_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_composite_glyphs_32_glyphcmds(
        &self,
        r: *const xcb_render_composite_glyphs_32_request_t,
    ) -> *mut u8 {
        sym!(self, xcb_render_composite_glyphs_32_glyphcmds)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_composite_glyphs_32_glyphcmds` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_composite_glyphs_32_glyphcmds(&self) -> bool {
        has_sym!(self, xcb_render_composite_glyphs_32_glyphcmds)
    }

    /// Returns the number of elements of the `glyphcmds` field of a `xcb_render_composite_glyphs_32_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_composite_glyphs_32_glyphcmds_length(
        &self,
        r: *const xcb_render_composite_glyphs_32_request_t,
    ) -> c_int {
        sym!(self, xcb_render_composite_glyphs_32_glyphcmds_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_composite_glyphs_32_glyphcmds_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_composite_glyphs_32_glyphcmds_length(&self) -> bool {
        has_sym!(self, xcb_render_composite_glyphs_32_glyphcmds_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `glyphcmds` field of a `xcb_render_composite_glyphs_32_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_composite_glyphs_32_glyphcmds_end(
        &self,
        r: *const xcb_render_composite_glyphs_32_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_composite_glyphs_32_glyphcmds_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_composite_glyphs_32_glyphcmds_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_composite_glyphs_32_glyphcmds_end(&self) -> bool {
        has_sym!(self, xcb_render_composite_glyphs_32_glyphcmds_end)
    }

    /// Computes the size of a `xcb_render_fill_rectangles_request_t` object.
    #[inline]
    pub unsafe fn xcb_render_fill_rectangles_sizeof(
        &self,
        _buffer: *const c_void,
        rects_len: u32,
    ) -> c_int {
        sym!(self, xcb_render_fill_rectangles_sizeof)(_buffer, rects_len)
    }

    /// Returns `true` iff the symbol `xcb_render_fill_rectangles_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_fill_rectangles_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_fill_rectangles_sizeof)
    }

    /// Sends a `Render::FillRectangles` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_fill_rectangles_checked(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        dst: xcb_render_picture_t,
        color: xcb_render_color_t,
        rects_len: u32,
        rects: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_fill_rectangles_checked)(c, op, dst, color, rects_len, rects)
    }

    /// Returns `true` iff the symbol `xcb_render_fill_rectangles_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_fill_rectangles_checked(&self) -> bool {
        has_sym!(self, xcb_render_fill_rectangles_checked)
    }

    /// Sends a `Render::FillRectangles` request (unchecked).
    #[inline]
    pub unsafe fn xcb_render_fill_rectangles(
        &self,
        c: *mut xcb_connection_t,
        op: u8,
        dst: xcb_render_picture_t,
        color: xcb_render_color_t,
        rects_len: u32,
        rects: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_fill_rectangles)(c, op, dst, color, rects_len, rects)
    }

    /// Returns `true` iff the symbol `xcb_render_fill_rectangles` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_fill_rectangles(&self) -> bool {
        has_sym!(self, xcb_render_fill_rectangles)
    }

    /// Returns a pointer to the `rects` field of a `xcb_render_fill_rectangles_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_fill_rectangles_rects(
        &self,
        r: *const xcb_render_fill_rectangles_request_t,
    ) -> *mut xcb_rectangle_t {
        sym!(self, xcb_render_fill_rectangles_rects)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_fill_rectangles_rects` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_fill_rectangles_rects(&self) -> bool {
        has_sym!(self, xcb_render_fill_rectangles_rects)
    }

    /// Returns the number of elements of the `rects` field of a `xcb_render_fill_rectangles_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_fill_rectangles_rects_length(
        &self,
        r: *const xcb_render_fill_rectangles_request_t,
    ) -> c_int {
        sym!(self, xcb_render_fill_rectangles_rects_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_fill_rectangles_rects_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_fill_rectangles_rects_length(&self) -> bool {
        has_sym!(self, xcb_render_fill_rectangles_rects_length)
    }

    /// Returns an iterator over the elements of the
    /// `rects` field of a `xcb_render_fill_rectangles_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_fill_rectangles_rects_iterator(
        &self,
        r: *const xcb_render_fill_rectangles_request_t,
    ) -> xcb_rectangle_iterator_t {
        sym!(self, xcb_render_fill_rectangles_rects_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_fill_rectangles_rects_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_fill_rectangles_rects_iterator(&self) -> bool {
        has_sym!(self, xcb_render_fill_rectangles_rects_iterator)
    }

    /// Sends a `Render::CreateCursor` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_create_cursor_checked(
        &self,
        c: *mut xcb_connection_t,
        cid: xcb_cursor_t,
        source: xcb_render_picture_t,
        x: u16,
        y: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_create_cursor_checked)(c, cid, source, x, y)
    }

    /// Returns `true` iff the symbol `xcb_render_create_cursor_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_cursor_checked(&self) -> bool {
        has_sym!(self, xcb_render_create_cursor_checked)
    }

    /// Sends a `Render::CreateCursor` request (unchecked).
    #[inline]
    pub unsafe fn xcb_render_create_cursor(
        &self,
        c: *mut xcb_connection_t,
        cid: xcb_cursor_t,
        source: xcb_render_picture_t,
        x: u16,
        y: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_create_cursor)(c, cid, source, x, y)
    }

    /// Returns `true` iff the symbol `xcb_render_create_cursor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_cursor(&self) -> bool {
        has_sym!(self, xcb_render_create_cursor)
    }

    /// Advances a `xcb_render_transform_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_render_transform_next(&self, i: *mut xcb_render_transform_iterator_t) {
        sym!(self, xcb_render_transform_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_transform_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_transform_next(&self) -> bool {
        has_sym!(self, xcb_render_transform_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_render_transform_iterator_t`.
    #[inline]
    pub unsafe fn xcb_render_transform_end(
        &self,
        i: xcb_render_transform_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_transform_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_transform_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_transform_end(&self) -> bool {
        has_sym!(self, xcb_render_transform_end)
    }

    /// Sends a `Render::SetPictureTransform` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_set_picture_transform_checked(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        transform: xcb_render_transform_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_set_picture_transform_checked)(c, picture, transform)
    }

    /// Returns `true` iff the symbol `xcb_render_set_picture_transform_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_set_picture_transform_checked(&self) -> bool {
        has_sym!(self, xcb_render_set_picture_transform_checked)
    }

    /// Sends a `Render::SetPictureTransform` request (unchecked).
    #[inline]
    pub unsafe fn xcb_render_set_picture_transform(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        transform: xcb_render_transform_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_set_picture_transform)(c, picture, transform)
    }

    /// Returns `true` iff the symbol `xcb_render_set_picture_transform` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_set_picture_transform(&self) -> bool {
        has_sym!(self, xcb_render_set_picture_transform)
    }

    /// Computes the size of a `xcb_render_query_filters_reply_t` object.
    #[inline]
    pub unsafe fn xcb_render_query_filters_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_render_query_filters_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_render_query_filters_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_filters_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_query_filters_sizeof)
    }

    /// Sends a `Render::QueryFilters` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_render_query_filters_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_render_query_filters_reply`]: Self::xcb_render_query_filters_reply
    #[inline]
    pub unsafe fn xcb_render_query_filters(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_render_query_filters_cookie_t {
        sym!(self, xcb_render_query_filters)(c, drawable)
    }

    /// Returns `true` iff the symbol `xcb_render_query_filters` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_filters(&self) -> bool {
        has_sym!(self, xcb_render_query_filters)
    }

    /// Sends a `Render::QueryFilters` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_render_query_filters_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_render_query_filters_reply`]: Self::xcb_render_query_filters_reply
    #[inline]
    pub unsafe fn xcb_render_query_filters_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_render_query_filters_cookie_t {
        sym!(self, xcb_render_query_filters_unchecked)(c, drawable)
    }

    /// Returns `true` iff the symbol `xcb_render_query_filters_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_filters_unchecked(&self) -> bool {
        has_sym!(self, xcb_render_query_filters_unchecked)
    }

    /// Returns a pointer to the `aliases` field of a `xcb_render_query_filters_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_render_query_filters_aliases(
        &self,
        r: *const xcb_render_query_filters_reply_t,
    ) -> *mut u16 {
        sym!(self, xcb_render_query_filters_aliases)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_query_filters_aliases` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_filters_aliases(&self) -> bool {
        has_sym!(self, xcb_render_query_filters_aliases)
    }

    /// Returns the number of elements of the `aliases` field of a `xcb_render_query_filters_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_render_query_filters_aliases_length(
        &self,
        r: *const xcb_render_query_filters_reply_t,
    ) -> c_int {
        sym!(self, xcb_render_query_filters_aliases_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_query_filters_aliases_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_filters_aliases_length(&self) -> bool {
        has_sym!(self, xcb_render_query_filters_aliases_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `aliases` field of a `xcb_render_query_filters_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_render_query_filters_aliases_end(
        &self,
        r: *const xcb_render_query_filters_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_query_filters_aliases_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_query_filters_aliases_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_filters_aliases_end(&self) -> bool {
        has_sym!(self, xcb_render_query_filters_aliases_end)
    }

    /// Returns the number of elements of the `filters` field of a `xcb_render_query_filters_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_render_query_filters_filters_length(
        &self,
        r: *const xcb_render_query_filters_reply_t,
    ) -> c_int {
        sym!(self, xcb_render_query_filters_filters_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_query_filters_filters_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_filters_filters_length(&self) -> bool {
        has_sym!(self, xcb_render_query_filters_filters_length)
    }

    /// Returns an iterator over the elements of the
    /// `filters` field of a `xcb_render_query_filters_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_render_query_filters_filters_iterator(
        &self,
        r: *const xcb_render_query_filters_reply_t,
    ) -> xcb_str_iterator_t {
        sym!(self, xcb_render_query_filters_filters_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_query_filters_filters_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_filters_filters_iterator(&self) -> bool {
        has_sym!(self, xcb_render_query_filters_filters_iterator)
    }

    /// Waits for the reply to a `Render::QueryFilters` request.
    #[inline]
    pub unsafe fn xcb_render_query_filters_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_render_query_filters_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_render_query_filters_reply_t {
        sym!(self, xcb_render_query_filters_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_render_query_filters_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_query_filters_reply(&self) -> bool {
        has_sym!(self, xcb_render_query_filters_reply)
    }

    /// Computes the size of a `xcb_render_set_picture_filter_request_t` object.
    #[inline]
    pub unsafe fn xcb_render_set_picture_filter_sizeof(
        &self,
        _buffer: *const c_void,
        values_len: u32,
    ) -> c_int {
        sym!(self, xcb_render_set_picture_filter_sizeof)(_buffer, values_len)
    }

    /// Returns `true` iff the symbol `xcb_render_set_picture_filter_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_set_picture_filter_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_set_picture_filter_sizeof)
    }

    /// Sends a `Render::SetPictureFilter` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_set_picture_filter_checked(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        filter_len: u16,
        filter: *const c_char,
        values_len: u32,
        values: *const xcb_render_fixed_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_set_picture_filter_checked)(
            c, picture, filter_len, filter, values_len, values,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_set_picture_filter_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_set_picture_filter_checked(&self) -> bool {
        has_sym!(self, xcb_render_set_picture_filter_checked)
    }

    /// Sends a `Render::SetPictureFilter` request (unchecked).
    #[inline]
    pub unsafe fn xcb_render_set_picture_filter(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        filter_len: u16,
        filter: *const c_char,
        values_len: u32,
        values: *const xcb_render_fixed_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_set_picture_filter)(
            c, picture, filter_len, filter, values_len, values,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_set_picture_filter` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_set_picture_filter(&self) -> bool {
        has_sym!(self, xcb_render_set_picture_filter)
    }

    /// Returns a pointer to the `filter` field of a `xcb_render_set_picture_filter_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_set_picture_filter_filter(
        &self,
        r: *const xcb_render_set_picture_filter_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_render_set_picture_filter_filter)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_set_picture_filter_filter` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_set_picture_filter_filter(&self) -> bool {
        has_sym!(self, xcb_render_set_picture_filter_filter)
    }

    /// Returns the number of elements of the `filter` field of a `xcb_render_set_picture_filter_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_set_picture_filter_filter_length(
        &self,
        r: *const xcb_render_set_picture_filter_request_t,
    ) -> c_int {
        sym!(self, xcb_render_set_picture_filter_filter_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_set_picture_filter_filter_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_set_picture_filter_filter_length(&self) -> bool {
        has_sym!(self, xcb_render_set_picture_filter_filter_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `filter` field of a `xcb_render_set_picture_filter_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_set_picture_filter_filter_end(
        &self,
        r: *const xcb_render_set_picture_filter_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_set_picture_filter_filter_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_set_picture_filter_filter_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_set_picture_filter_filter_end(&self) -> bool {
        has_sym!(self, xcb_render_set_picture_filter_filter_end)
    }

    /// Returns a pointer to the `values` field of a `xcb_render_set_picture_filter_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_set_picture_filter_values(
        &self,
        r: *const xcb_render_set_picture_filter_request_t,
    ) -> *mut xcb_render_fixed_t {
        sym!(self, xcb_render_set_picture_filter_values)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_set_picture_filter_values` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_set_picture_filter_values(&self) -> bool {
        has_sym!(self, xcb_render_set_picture_filter_values)
    }

    /// Returns the number of elements of the `values` field of a `xcb_render_set_picture_filter_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_set_picture_filter_values_length(
        &self,
        r: *const xcb_render_set_picture_filter_request_t,
    ) -> c_int {
        sym!(self, xcb_render_set_picture_filter_values_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_set_picture_filter_values_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_set_picture_filter_values_length(&self) -> bool {
        has_sym!(self, xcb_render_set_picture_filter_values_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `values` field of a `xcb_render_set_picture_filter_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_set_picture_filter_values_end(
        &self,
        r: *const xcb_render_set_picture_filter_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_set_picture_filter_values_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_set_picture_filter_values_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_set_picture_filter_values_end(&self) -> bool {
        has_sym!(self, xcb_render_set_picture_filter_values_end)
    }

    /// Advances a `xcb_render_animcursorelt_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_render_animcursorelt_next(
        &self,
        i: *mut xcb_render_animcursorelt_iterator_t,
    ) {
        sym!(self, xcb_render_animcursorelt_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_animcursorelt_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_animcursorelt_next(&self) -> bool {
        has_sym!(self, xcb_render_animcursorelt_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_render_animcursorelt_iterator_t`.
    #[inline]
    pub unsafe fn xcb_render_animcursorelt_end(
        &self,
        i: xcb_render_animcursorelt_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_animcursorelt_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_animcursorelt_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_animcursorelt_end(&self) -> bool {
        has_sym!(self, xcb_render_animcursorelt_end)
    }

    /// Computes the size of a `xcb_render_create_anim_cursor_request_t` object.
    #[inline]
    pub unsafe fn xcb_render_create_anim_cursor_sizeof(
        &self,
        _buffer: *const c_void,
        cursors_len: u32,
    ) -> c_int {
        sym!(self, xcb_render_create_anim_cursor_sizeof)(_buffer, cursors_len)
    }

    /// Returns `true` iff the symbol `xcb_render_create_anim_cursor_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_anim_cursor_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_create_anim_cursor_sizeof)
    }

    /// Sends a `Render::CreateAnimCursor` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_create_anim_cursor_checked(
        &self,
        c: *mut xcb_connection_t,
        cid: xcb_cursor_t,
        cursors_len: u32,
        cursors: *const xcb_render_animcursorelt_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_create_anim_cursor_checked)(c, cid, cursors_len, cursors)
    }

    /// Returns `true` iff the symbol `xcb_render_create_anim_cursor_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_anim_cursor_checked(&self) -> bool {
        has_sym!(self, xcb_render_create_anim_cursor_checked)
    }

    /// Sends a `Render::CreateAnimCursor` request (unchecked).
    #[inline]
    pub unsafe fn xcb_render_create_anim_cursor(
        &self,
        c: *mut xcb_connection_t,
        cid: xcb_cursor_t,
        cursors_len: u32,
        cursors: *const xcb_render_animcursorelt_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_create_anim_cursor)(c, cid, cursors_len, cursors)
    }

    /// Returns `true` iff the symbol `xcb_render_create_anim_cursor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_anim_cursor(&self) -> bool {
        has_sym!(self, xcb_render_create_anim_cursor)
    }

    /// Returns a pointer to the `cursors` field of a `xcb_render_create_anim_cursor_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_create_anim_cursor_cursors(
        &self,
        r: *const xcb_render_create_anim_cursor_request_t,
    ) -> *mut xcb_render_animcursorelt_t {
        sym!(self, xcb_render_create_anim_cursor_cursors)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_create_anim_cursor_cursors` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_anim_cursor_cursors(&self) -> bool {
        has_sym!(self, xcb_render_create_anim_cursor_cursors)
    }

    /// Returns the number of elements of the `cursors` field of a `xcb_render_create_anim_cursor_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_create_anim_cursor_cursors_length(
        &self,
        r: *const xcb_render_create_anim_cursor_request_t,
    ) -> c_int {
        sym!(self, xcb_render_create_anim_cursor_cursors_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_create_anim_cursor_cursors_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_anim_cursor_cursors_length(&self) -> bool {
        has_sym!(self, xcb_render_create_anim_cursor_cursors_length)
    }

    /// Returns an iterator over the elements of the
    /// `cursors` field of a `xcb_render_create_anim_cursor_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_create_anim_cursor_cursors_iterator(
        &self,
        r: *const xcb_render_create_anim_cursor_request_t,
    ) -> xcb_render_animcursorelt_iterator_t {
        sym!(self, xcb_render_create_anim_cursor_cursors_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_create_anim_cursor_cursors_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_anim_cursor_cursors_iterator(&self) -> bool {
        has_sym!(self, xcb_render_create_anim_cursor_cursors_iterator)
    }

    /// Advances a `xcb_render_spanfix_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_render_spanfix_next(&self, i: *mut xcb_render_spanfix_iterator_t) {
        sym!(self, xcb_render_spanfix_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_spanfix_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_spanfix_next(&self) -> bool {
        has_sym!(self, xcb_render_spanfix_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_render_spanfix_iterator_t`.
    #[inline]
    pub unsafe fn xcb_render_spanfix_end(
        &self,
        i: xcb_render_spanfix_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_spanfix_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_spanfix_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_spanfix_end(&self) -> bool {
        has_sym!(self, xcb_render_spanfix_end)
    }

    /// Advances a `xcb_render_trap_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_render_trap_next(&self, i: *mut xcb_render_trap_iterator_t) {
        sym!(self, xcb_render_trap_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_trap_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_trap_next(&self) -> bool {
        has_sym!(self, xcb_render_trap_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_render_trap_iterator_t`.
    #[inline]
    pub unsafe fn xcb_render_trap_end(
        &self,
        i: xcb_render_trap_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_trap_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_render_trap_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_trap_end(&self) -> bool {
        has_sym!(self, xcb_render_trap_end)
    }

    /// Computes the size of a `xcb_render_add_traps_request_t` object.
    #[inline]
    pub unsafe fn xcb_render_add_traps_sizeof(
        &self,
        _buffer: *const c_void,
        traps_len: u32,
    ) -> c_int {
        sym!(self, xcb_render_add_traps_sizeof)(_buffer, traps_len)
    }

    /// Returns `true` iff the symbol `xcb_render_add_traps_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_add_traps_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_add_traps_sizeof)
    }

    /// Sends a `Render::AddTraps` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_add_traps_checked(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        x_off: i16,
        y_off: i16,
        traps_len: u32,
        traps: *const xcb_render_trap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_add_traps_checked)(c, picture, x_off, y_off, traps_len, traps)
    }

    /// Returns `true` iff the symbol `xcb_render_add_traps_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_add_traps_checked(&self) -> bool {
        has_sym!(self, xcb_render_add_traps_checked)
    }

    /// Sends a `Render::AddTraps` request (unchecked).
    #[inline]
    pub unsafe fn xcb_render_add_traps(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        x_off: i16,
        y_off: i16,
        traps_len: u32,
        traps: *const xcb_render_trap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_add_traps)(c, picture, x_off, y_off, traps_len, traps)
    }

    /// Returns `true` iff the symbol `xcb_render_add_traps` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_add_traps(&self) -> bool {
        has_sym!(self, xcb_render_add_traps)
    }

    /// Returns a pointer to the `traps` field of a `xcb_render_add_traps_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_add_traps_traps(
        &self,
        r: *const xcb_render_add_traps_request_t,
    ) -> *mut xcb_render_trap_t {
        sym!(self, xcb_render_add_traps_traps)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_add_traps_traps` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_add_traps_traps(&self) -> bool {
        has_sym!(self, xcb_render_add_traps_traps)
    }

    /// Returns the number of elements of the `traps` field of a `xcb_render_add_traps_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_add_traps_traps_length(
        &self,
        r: *const xcb_render_add_traps_request_t,
    ) -> c_int {
        sym!(self, xcb_render_add_traps_traps_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_add_traps_traps_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_add_traps_traps_length(&self) -> bool {
        has_sym!(self, xcb_render_add_traps_traps_length)
    }

    /// Returns an iterator over the elements of the
    /// `traps` field of a `xcb_render_add_traps_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_add_traps_traps_iterator(
        &self,
        r: *const xcb_render_add_traps_request_t,
    ) -> xcb_render_trap_iterator_t {
        sym!(self, xcb_render_add_traps_traps_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_add_traps_traps_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_add_traps_traps_iterator(&self) -> bool {
        has_sym!(self, xcb_render_add_traps_traps_iterator)
    }

    /// Sends a `Render::CreateSolidFill` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_create_solid_fill_checked(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        color: xcb_render_color_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_create_solid_fill_checked)(c, picture, color)
    }

    /// Returns `true` iff the symbol `xcb_render_create_solid_fill_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_solid_fill_checked(&self) -> bool {
        has_sym!(self, xcb_render_create_solid_fill_checked)
    }

    /// Sends a `Render::CreateSolidFill` request (unchecked).
    #[inline]
    pub unsafe fn xcb_render_create_solid_fill(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        color: xcb_render_color_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_create_solid_fill)(c, picture, color)
    }

    /// Returns `true` iff the symbol `xcb_render_create_solid_fill` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_solid_fill(&self) -> bool {
        has_sym!(self, xcb_render_create_solid_fill)
    }

    /// Computes the size of a `xcb_render_create_linear_gradient_request_t` object.
    #[inline]
    pub unsafe fn xcb_render_create_linear_gradient_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_render_create_linear_gradient_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_render_create_linear_gradient_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_linear_gradient_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_create_linear_gradient_sizeof)
    }

    /// Sends a `Render::CreateLinearGradient` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_create_linear_gradient_checked(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        p1: xcb_render_pointfix_t,
        p2: xcb_render_pointfix_t,
        num_stops: u32,
        stops: *const xcb_render_fixed_t,
        colors: *const xcb_render_color_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_create_linear_gradient_checked)(
            c, picture, p1, p2, num_stops, stops, colors,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_create_linear_gradient_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_linear_gradient_checked(&self) -> bool {
        has_sym!(self, xcb_render_create_linear_gradient_checked)
    }

    /// Sends a `Render::CreateLinearGradient` request (unchecked).
    #[inline]
    pub unsafe fn xcb_render_create_linear_gradient(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        p1: xcb_render_pointfix_t,
        p2: xcb_render_pointfix_t,
        num_stops: u32,
        stops: *const xcb_render_fixed_t,
        colors: *const xcb_render_color_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_create_linear_gradient)(c, picture, p1, p2, num_stops, stops, colors)
    }

    /// Returns `true` iff the symbol `xcb_render_create_linear_gradient` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_linear_gradient(&self) -> bool {
        has_sym!(self, xcb_render_create_linear_gradient)
    }

    /// Returns a pointer to the `stops` field of a `xcb_render_create_linear_gradient_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_create_linear_gradient_stops(
        &self,
        r: *const xcb_render_create_linear_gradient_request_t,
    ) -> *mut xcb_render_fixed_t {
        sym!(self, xcb_render_create_linear_gradient_stops)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_create_linear_gradient_stops` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_linear_gradient_stops(&self) -> bool {
        has_sym!(self, xcb_render_create_linear_gradient_stops)
    }

    /// Returns the number of elements of the `stops` field of a `xcb_render_create_linear_gradient_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_create_linear_gradient_stops_length(
        &self,
        r: *const xcb_render_create_linear_gradient_request_t,
    ) -> c_int {
        sym!(self, xcb_render_create_linear_gradient_stops_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_create_linear_gradient_stops_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_linear_gradient_stops_length(&self) -> bool {
        has_sym!(self, xcb_render_create_linear_gradient_stops_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `stops` field of a `xcb_render_create_linear_gradient_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_create_linear_gradient_stops_end(
        &self,
        r: *const xcb_render_create_linear_gradient_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_create_linear_gradient_stops_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_create_linear_gradient_stops_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_linear_gradient_stops_end(&self) -> bool {
        has_sym!(self, xcb_render_create_linear_gradient_stops_end)
    }

    /// Returns a pointer to the `colors` field of a `xcb_render_create_linear_gradient_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_create_linear_gradient_colors(
        &self,
        r: *const xcb_render_create_linear_gradient_request_t,
    ) -> *mut xcb_render_color_t {
        sym!(self, xcb_render_create_linear_gradient_colors)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_create_linear_gradient_colors` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_linear_gradient_colors(&self) -> bool {
        has_sym!(self, xcb_render_create_linear_gradient_colors)
    }

    /// Returns the number of elements of the `colors` field of a `xcb_render_create_linear_gradient_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_create_linear_gradient_colors_length(
        &self,
        r: *const xcb_render_create_linear_gradient_request_t,
    ) -> c_int {
        sym!(self, xcb_render_create_linear_gradient_colors_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_create_linear_gradient_colors_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_linear_gradient_colors_length(&self) -> bool {
        has_sym!(self, xcb_render_create_linear_gradient_colors_length)
    }

    /// Returns an iterator over the elements of the
    /// `colors` field of a `xcb_render_create_linear_gradient_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_create_linear_gradient_colors_iterator(
        &self,
        r: *const xcb_render_create_linear_gradient_request_t,
    ) -> xcb_render_color_iterator_t {
        sym!(self, xcb_render_create_linear_gradient_colors_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_create_linear_gradient_colors_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_linear_gradient_colors_iterator(&self) -> bool {
        has_sym!(self, xcb_render_create_linear_gradient_colors_iterator)
    }

    /// Computes the size of a `xcb_render_create_radial_gradient_request_t` object.
    #[inline]
    pub unsafe fn xcb_render_create_radial_gradient_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_render_create_radial_gradient_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_render_create_radial_gradient_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_radial_gradient_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_create_radial_gradient_sizeof)
    }

    /// Sends a `Render::CreateRadialGradient` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_create_radial_gradient_checked(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        inner: xcb_render_pointfix_t,
        outer: xcb_render_pointfix_t,
        inner_radius: xcb_render_fixed_t,
        outer_radius: xcb_render_fixed_t,
        num_stops: u32,
        stops: *const xcb_render_fixed_t,
        colors: *const xcb_render_color_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_create_radial_gradient_checked)(
            c,
            picture,
            inner,
            outer,
            inner_radius,
            outer_radius,
            num_stops,
            stops,
            colors,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_create_radial_gradient_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_radial_gradient_checked(&self) -> bool {
        has_sym!(self, xcb_render_create_radial_gradient_checked)
    }

    /// Sends a `Render::CreateRadialGradient` request (unchecked).
    #[inline]
    pub unsafe fn xcb_render_create_radial_gradient(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        inner: xcb_render_pointfix_t,
        outer: xcb_render_pointfix_t,
        inner_radius: xcb_render_fixed_t,
        outer_radius: xcb_render_fixed_t,
        num_stops: u32,
        stops: *const xcb_render_fixed_t,
        colors: *const xcb_render_color_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_create_radial_gradient)(
            c,
            picture,
            inner,
            outer,
            inner_radius,
            outer_radius,
            num_stops,
            stops,
            colors,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_create_radial_gradient` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_radial_gradient(&self) -> bool {
        has_sym!(self, xcb_render_create_radial_gradient)
    }

    /// Returns a pointer to the `stops` field of a `xcb_render_create_radial_gradient_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_create_radial_gradient_stops(
        &self,
        r: *const xcb_render_create_radial_gradient_request_t,
    ) -> *mut xcb_render_fixed_t {
        sym!(self, xcb_render_create_radial_gradient_stops)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_create_radial_gradient_stops` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_radial_gradient_stops(&self) -> bool {
        has_sym!(self, xcb_render_create_radial_gradient_stops)
    }

    /// Returns the number of elements of the `stops` field of a `xcb_render_create_radial_gradient_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_create_radial_gradient_stops_length(
        &self,
        r: *const xcb_render_create_radial_gradient_request_t,
    ) -> c_int {
        sym!(self, xcb_render_create_radial_gradient_stops_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_create_radial_gradient_stops_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_radial_gradient_stops_length(&self) -> bool {
        has_sym!(self, xcb_render_create_radial_gradient_stops_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `stops` field of a `xcb_render_create_radial_gradient_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_create_radial_gradient_stops_end(
        &self,
        r: *const xcb_render_create_radial_gradient_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_create_radial_gradient_stops_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_create_radial_gradient_stops_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_radial_gradient_stops_end(&self) -> bool {
        has_sym!(self, xcb_render_create_radial_gradient_stops_end)
    }

    /// Returns a pointer to the `colors` field of a `xcb_render_create_radial_gradient_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_create_radial_gradient_colors(
        &self,
        r: *const xcb_render_create_radial_gradient_request_t,
    ) -> *mut xcb_render_color_t {
        sym!(self, xcb_render_create_radial_gradient_colors)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_create_radial_gradient_colors` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_radial_gradient_colors(&self) -> bool {
        has_sym!(self, xcb_render_create_radial_gradient_colors)
    }

    /// Returns the number of elements of the `colors` field of a `xcb_render_create_radial_gradient_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_create_radial_gradient_colors_length(
        &self,
        r: *const xcb_render_create_radial_gradient_request_t,
    ) -> c_int {
        sym!(self, xcb_render_create_radial_gradient_colors_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_create_radial_gradient_colors_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_radial_gradient_colors_length(&self) -> bool {
        has_sym!(self, xcb_render_create_radial_gradient_colors_length)
    }

    /// Returns an iterator over the elements of the
    /// `colors` field of a `xcb_render_create_radial_gradient_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_create_radial_gradient_colors_iterator(
        &self,
        r: *const xcb_render_create_radial_gradient_request_t,
    ) -> xcb_render_color_iterator_t {
        sym!(self, xcb_render_create_radial_gradient_colors_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_create_radial_gradient_colors_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_radial_gradient_colors_iterator(&self) -> bool {
        has_sym!(self, xcb_render_create_radial_gradient_colors_iterator)
    }

    /// Computes the size of a `xcb_render_create_conical_gradient_request_t` object.
    #[inline]
    pub unsafe fn xcb_render_create_conical_gradient_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_render_create_conical_gradient_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_render_create_conical_gradient_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_conical_gradient_sizeof(&self) -> bool {
        has_sym!(self, xcb_render_create_conical_gradient_sizeof)
    }

    /// Sends a `Render::CreateConicalGradient` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_render_create_conical_gradient_checked(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        center: xcb_render_pointfix_t,
        angle: xcb_render_fixed_t,
        num_stops: u32,
        stops: *const xcb_render_fixed_t,
        colors: *const xcb_render_color_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_create_conical_gradient_checked)(
            c, picture, center, angle, num_stops, stops, colors,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_create_conical_gradient_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_conical_gradient_checked(&self) -> bool {
        has_sym!(self, xcb_render_create_conical_gradient_checked)
    }

    /// Sends a `Render::CreateConicalGradient` request (unchecked).
    #[inline]
    pub unsafe fn xcb_render_create_conical_gradient(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        center: xcb_render_pointfix_t,
        angle: xcb_render_fixed_t,
        num_stops: u32,
        stops: *const xcb_render_fixed_t,
        colors: *const xcb_render_color_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_render_create_conical_gradient)(
            c, picture, center, angle, num_stops, stops, colors,
        )
    }

    /// Returns `true` iff the symbol `xcb_render_create_conical_gradient` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_conical_gradient(&self) -> bool {
        has_sym!(self, xcb_render_create_conical_gradient)
    }

    /// Returns a pointer to the `stops` field of a `xcb_render_create_conical_gradient_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_create_conical_gradient_stops(
        &self,
        r: *const xcb_render_create_conical_gradient_request_t,
    ) -> *mut xcb_render_fixed_t {
        sym!(self, xcb_render_create_conical_gradient_stops)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_create_conical_gradient_stops` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_conical_gradient_stops(&self) -> bool {
        has_sym!(self, xcb_render_create_conical_gradient_stops)
    }

    /// Returns the number of elements of the `stops` field of a `xcb_render_create_conical_gradient_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_create_conical_gradient_stops_length(
        &self,
        r: *const xcb_render_create_conical_gradient_request_t,
    ) -> c_int {
        sym!(self, xcb_render_create_conical_gradient_stops_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_create_conical_gradient_stops_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_conical_gradient_stops_length(&self) -> bool {
        has_sym!(self, xcb_render_create_conical_gradient_stops_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `stops` field of a `xcb_render_create_conical_gradient_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_create_conical_gradient_stops_end(
        &self,
        r: *const xcb_render_create_conical_gradient_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_render_create_conical_gradient_stops_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_create_conical_gradient_stops_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_conical_gradient_stops_end(&self) -> bool {
        has_sym!(self, xcb_render_create_conical_gradient_stops_end)
    }

    /// Returns a pointer to the `colors` field of a `xcb_render_create_conical_gradient_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_create_conical_gradient_colors(
        &self,
        r: *const xcb_render_create_conical_gradient_request_t,
    ) -> *mut xcb_render_color_t {
        sym!(self, xcb_render_create_conical_gradient_colors)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_create_conical_gradient_colors` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_conical_gradient_colors(&self) -> bool {
        has_sym!(self, xcb_render_create_conical_gradient_colors)
    }

    /// Returns the number of elements of the `colors` field of a `xcb_render_create_conical_gradient_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_create_conical_gradient_colors_length(
        &self,
        r: *const xcb_render_create_conical_gradient_request_t,
    ) -> c_int {
        sym!(self, xcb_render_create_conical_gradient_colors_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_create_conical_gradient_colors_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_conical_gradient_colors_length(&self) -> bool {
        has_sym!(self, xcb_render_create_conical_gradient_colors_length)
    }

    /// Returns an iterator over the elements of the
    /// `colors` field of a `xcb_render_create_conical_gradient_request_t` struct.
    #[inline]
    pub unsafe fn xcb_render_create_conical_gradient_colors_iterator(
        &self,
        r: *const xcb_render_create_conical_gradient_request_t,
    ) -> xcb_render_color_iterator_t {
        sym!(self, xcb_render_create_conical_gradient_colors_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_render_create_conical_gradient_colors_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_render_create_conical_gradient_colors_iterator(&self) -> bool {
        has_sym!(self, xcb_render_create_conical_gradient_colors_iterator)
    }
}

#[cfg(feature = "xcb_render")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbRender::load().unwrap() };
        assert!(lib.has_xcb_render_id());
        assert!(lib.has_xcb_render_glyph_next());
        assert!(lib.has_xcb_render_glyph_end());
        assert!(lib.has_xcb_render_glyphset_next());
        assert!(lib.has_xcb_render_glyphset_end());
        assert!(lib.has_xcb_render_picture_next());
        assert!(lib.has_xcb_render_picture_end());
        assert!(lib.has_xcb_render_pictformat_next());
        assert!(lib.has_xcb_render_pictformat_end());
        assert!(lib.has_xcb_render_fixed_next());
        assert!(lib.has_xcb_render_fixed_end());
        assert!(lib.has_xcb_render_directformat_next());
        assert!(lib.has_xcb_render_directformat_end());
        assert!(lib.has_xcb_render_pictforminfo_next());
        assert!(lib.has_xcb_render_pictforminfo_end());
        assert!(lib.has_xcb_render_pictvisual_next());
        assert!(lib.has_xcb_render_pictvisual_end());
        assert!(lib.has_xcb_render_pictdepth_sizeof());
        assert!(lib.has_xcb_render_pictdepth_visuals());
        assert!(lib.has_xcb_render_pictdepth_visuals_length());
        assert!(lib.has_xcb_render_pictdepth_visuals_iterator());
        assert!(lib.has_xcb_render_pictdepth_next());
        assert!(lib.has_xcb_render_pictdepth_end());
        assert!(lib.has_xcb_render_pictscreen_sizeof());
        assert!(lib.has_xcb_render_pictscreen_depths_length());
        assert!(lib.has_xcb_render_pictscreen_depths_iterator());
        assert!(lib.has_xcb_render_pictscreen_next());
        assert!(lib.has_xcb_render_pictscreen_end());
        assert!(lib.has_xcb_render_indexvalue_next());
        assert!(lib.has_xcb_render_indexvalue_end());
        assert!(lib.has_xcb_render_color_next());
        assert!(lib.has_xcb_render_color_end());
        assert!(lib.has_xcb_render_pointfix_next());
        assert!(lib.has_xcb_render_pointfix_end());
        assert!(lib.has_xcb_render_linefix_next());
        assert!(lib.has_xcb_render_linefix_end());
        assert!(lib.has_xcb_render_triangle_next());
        assert!(lib.has_xcb_render_triangle_end());
        assert!(lib.has_xcb_render_trapezoid_next());
        assert!(lib.has_xcb_render_trapezoid_end());
        assert!(lib.has_xcb_render_glyphinfo_next());
        assert!(lib.has_xcb_render_glyphinfo_end());
        assert!(lib.has_xcb_render_query_version());
        assert!(lib.has_xcb_render_query_version_unchecked());
        assert!(lib.has_xcb_render_query_version_reply());
        assert!(lib.has_xcb_render_query_pict_formats_sizeof());
        assert!(lib.has_xcb_render_query_pict_formats());
        assert!(lib.has_xcb_render_query_pict_formats_unchecked());
        assert!(lib.has_xcb_render_query_pict_formats_formats());
        assert!(lib.has_xcb_render_query_pict_formats_formats_length());
        assert!(lib.has_xcb_render_query_pict_formats_formats_iterator());
        assert!(lib.has_xcb_render_query_pict_formats_screens_length());
        assert!(lib.has_xcb_render_query_pict_formats_screens_iterator());
        assert!(lib.has_xcb_render_query_pict_formats_subpixels());
        assert!(lib.has_xcb_render_query_pict_formats_subpixels_length());
        assert!(lib.has_xcb_render_query_pict_formats_subpixels_end());
        assert!(lib.has_xcb_render_query_pict_formats_reply());
        assert!(lib.has_xcb_render_query_pict_index_values_sizeof());
        assert!(lib.has_xcb_render_query_pict_index_values());
        assert!(lib.has_xcb_render_query_pict_index_values_unchecked());
        assert!(lib.has_xcb_render_query_pict_index_values_values());
        assert!(lib.has_xcb_render_query_pict_index_values_values_length());
        assert!(lib.has_xcb_render_query_pict_index_values_values_iterator());
        assert!(lib.has_xcb_render_query_pict_index_values_reply());
        assert!(lib.has_xcb_render_create_picture_value_list_serialize());
        assert!(lib.has_xcb_render_create_picture_value_list_unpack());
        assert!(lib.has_xcb_render_create_picture_value_list_sizeof());
        assert!(lib.has_xcb_render_create_picture_sizeof());
        assert!(lib.has_xcb_render_create_picture_checked());
        assert!(lib.has_xcb_render_create_picture());
        assert!(lib.has_xcb_render_create_picture_aux_checked());
        assert!(lib.has_xcb_render_create_picture_aux());
        assert!(lib.has_xcb_render_create_picture_value_list());
        assert!(lib.has_xcb_render_change_picture_value_list_serialize());
        assert!(lib.has_xcb_render_change_picture_value_list_unpack());
        assert!(lib.has_xcb_render_change_picture_value_list_sizeof());
        assert!(lib.has_xcb_render_change_picture_sizeof());
        assert!(lib.has_xcb_render_change_picture_checked());
        assert!(lib.has_xcb_render_change_picture());
        assert!(lib.has_xcb_render_change_picture_aux_checked());
        assert!(lib.has_xcb_render_change_picture_aux());
        assert!(lib.has_xcb_render_change_picture_value_list());
        assert!(lib.has_xcb_render_set_picture_clip_rectangles_sizeof());
        assert!(lib.has_xcb_render_set_picture_clip_rectangles_checked());
        assert!(lib.has_xcb_render_set_picture_clip_rectangles());
        assert!(lib.has_xcb_render_set_picture_clip_rectangles_rectangles());
        assert!(lib.has_xcb_render_set_picture_clip_rectangles_rectangles_length());
        assert!(lib.has_xcb_render_set_picture_clip_rectangles_rectangles_iterator());
        assert!(lib.has_xcb_render_free_picture_checked());
        assert!(lib.has_xcb_render_free_picture());
        assert!(lib.has_xcb_render_composite_checked());
        assert!(lib.has_xcb_render_composite());
        assert!(lib.has_xcb_render_trapezoids_sizeof());
        assert!(lib.has_xcb_render_trapezoids_checked());
        assert!(lib.has_xcb_render_trapezoids());
        assert!(lib.has_xcb_render_trapezoids_traps());
        assert!(lib.has_xcb_render_trapezoids_traps_length());
        assert!(lib.has_xcb_render_trapezoids_traps_iterator());
        assert!(lib.has_xcb_render_triangles_sizeof());
        assert!(lib.has_xcb_render_triangles_checked());
        assert!(lib.has_xcb_render_triangles());
        assert!(lib.has_xcb_render_triangles_triangles());
        assert!(lib.has_xcb_render_triangles_triangles_length());
        assert!(lib.has_xcb_render_triangles_triangles_iterator());
        assert!(lib.has_xcb_render_tri_strip_sizeof());
        assert!(lib.has_xcb_render_tri_strip_checked());
        assert!(lib.has_xcb_render_tri_strip());
        assert!(lib.has_xcb_render_tri_strip_points());
        assert!(lib.has_xcb_render_tri_strip_points_length());
        assert!(lib.has_xcb_render_tri_strip_points_iterator());
        assert!(lib.has_xcb_render_tri_fan_sizeof());
        assert!(lib.has_xcb_render_tri_fan_checked());
        assert!(lib.has_xcb_render_tri_fan());
        assert!(lib.has_xcb_render_tri_fan_points());
        assert!(lib.has_xcb_render_tri_fan_points_length());
        assert!(lib.has_xcb_render_tri_fan_points_iterator());
        assert!(lib.has_xcb_render_create_glyph_set_checked());
        assert!(lib.has_xcb_render_create_glyph_set());
        assert!(lib.has_xcb_render_reference_glyph_set_checked());
        assert!(lib.has_xcb_render_reference_glyph_set());
        assert!(lib.has_xcb_render_free_glyph_set_checked());
        assert!(lib.has_xcb_render_free_glyph_set());
        assert!(lib.has_xcb_render_add_glyphs_sizeof());
        assert!(lib.has_xcb_render_add_glyphs_checked());
        assert!(lib.has_xcb_render_add_glyphs());
        assert!(lib.has_xcb_render_add_glyphs_glyphids());
        assert!(lib.has_xcb_render_add_glyphs_glyphids_length());
        assert!(lib.has_xcb_render_add_glyphs_glyphids_end());
        assert!(lib.has_xcb_render_add_glyphs_glyphs());
        assert!(lib.has_xcb_render_add_glyphs_glyphs_length());
        assert!(lib.has_xcb_render_add_glyphs_glyphs_iterator());
        assert!(lib.has_xcb_render_add_glyphs_data());
        assert!(lib.has_xcb_render_add_glyphs_data_length());
        assert!(lib.has_xcb_render_add_glyphs_data_end());
        assert!(lib.has_xcb_render_free_glyphs_sizeof());
        assert!(lib.has_xcb_render_free_glyphs_checked());
        assert!(lib.has_xcb_render_free_glyphs());
        assert!(lib.has_xcb_render_free_glyphs_glyphs());
        assert!(lib.has_xcb_render_free_glyphs_glyphs_length());
        assert!(lib.has_xcb_render_free_glyphs_glyphs_end());
        assert!(lib.has_xcb_render_composite_glyphs_8_sizeof());
        assert!(lib.has_xcb_render_composite_glyphs_8_checked());
        assert!(lib.has_xcb_render_composite_glyphs_8());
        assert!(lib.has_xcb_render_composite_glyphs_8_glyphcmds());
        assert!(lib.has_xcb_render_composite_glyphs_8_glyphcmds_length());
        assert!(lib.has_xcb_render_composite_glyphs_8_glyphcmds_end());
        assert!(lib.has_xcb_render_composite_glyphs_16_sizeof());
        assert!(lib.has_xcb_render_composite_glyphs_16_checked());
        assert!(lib.has_xcb_render_composite_glyphs_16());
        assert!(lib.has_xcb_render_composite_glyphs_16_glyphcmds());
        assert!(lib.has_xcb_render_composite_glyphs_16_glyphcmds_length());
        assert!(lib.has_xcb_render_composite_glyphs_16_glyphcmds_end());
        assert!(lib.has_xcb_render_composite_glyphs_32_sizeof());
        assert!(lib.has_xcb_render_composite_glyphs_32_checked());
        assert!(lib.has_xcb_render_composite_glyphs_32());
        assert!(lib.has_xcb_render_composite_glyphs_32_glyphcmds());
        assert!(lib.has_xcb_render_composite_glyphs_32_glyphcmds_length());
        assert!(lib.has_xcb_render_composite_glyphs_32_glyphcmds_end());
        assert!(lib.has_xcb_render_fill_rectangles_sizeof());
        assert!(lib.has_xcb_render_fill_rectangles_checked());
        assert!(lib.has_xcb_render_fill_rectangles());
        assert!(lib.has_xcb_render_fill_rectangles_rects());
        assert!(lib.has_xcb_render_fill_rectangles_rects_length());
        assert!(lib.has_xcb_render_fill_rectangles_rects_iterator());
        assert!(lib.has_xcb_render_create_cursor_checked());
        assert!(lib.has_xcb_render_create_cursor());
        assert!(lib.has_xcb_render_transform_next());
        assert!(lib.has_xcb_render_transform_end());
        assert!(lib.has_xcb_render_set_picture_transform_checked());
        assert!(lib.has_xcb_render_set_picture_transform());
        assert!(lib.has_xcb_render_query_filters_sizeof());
        assert!(lib.has_xcb_render_query_filters());
        assert!(lib.has_xcb_render_query_filters_unchecked());
        assert!(lib.has_xcb_render_query_filters_aliases());
        assert!(lib.has_xcb_render_query_filters_aliases_length());
        assert!(lib.has_xcb_render_query_filters_aliases_end());
        assert!(lib.has_xcb_render_query_filters_filters_length());
        assert!(lib.has_xcb_render_query_filters_filters_iterator());
        assert!(lib.has_xcb_render_query_filters_reply());
        assert!(lib.has_xcb_render_set_picture_filter_sizeof());
        assert!(lib.has_xcb_render_set_picture_filter_checked());
        assert!(lib.has_xcb_render_set_picture_filter());
        assert!(lib.has_xcb_render_set_picture_filter_filter());
        assert!(lib.has_xcb_render_set_picture_filter_filter_length());
        assert!(lib.has_xcb_render_set_picture_filter_filter_end());
        assert!(lib.has_xcb_render_set_picture_filter_values());
        assert!(lib.has_xcb_render_set_picture_filter_values_length());
        assert!(lib.has_xcb_render_set_picture_filter_values_end());
        assert!(lib.has_xcb_render_animcursorelt_next());
        assert!(lib.has_xcb_render_animcursorelt_end());
        assert!(lib.has_xcb_render_create_anim_cursor_sizeof());
        assert!(lib.has_xcb_render_create_anim_cursor_checked());
        assert!(lib.has_xcb_render_create_anim_cursor());
        assert!(lib.has_xcb_render_create_anim_cursor_cursors());
        assert!(lib.has_xcb_render_create_anim_cursor_cursors_length());
        assert!(lib.has_xcb_render_create_anim_cursor_cursors_iterator());
        assert!(lib.has_xcb_render_spanfix_next());
        assert!(lib.has_xcb_render_spanfix_end());
        assert!(lib.has_xcb_render_trap_next());
        assert!(lib.has_xcb_render_trap_end());
        assert!(lib.has_xcb_render_add_traps_sizeof());
        assert!(lib.has_xcb_render_add_traps_checked());
        assert!(lib.has_xcb_render_add_traps());
        assert!(lib.has_xcb_render_add_traps_traps());
        assert!(lib.has_xcb_render_add_traps_traps_length());
        assert!(lib.has_xcb_render_add_traps_traps_iterator());
        assert!(lib.has_xcb_render_create_solid_fill_checked());
        assert!(lib.has_xcb_render_create_solid_fill());
        assert!(lib.has_xcb_render_create_linear_gradient_sizeof());
        assert!(lib.has_xcb_render_create_linear_gradient_checked());
        assert!(lib.has_xcb_render_create_linear_gradient());
        assert!(lib.has_xcb_render_create_linear_gradient_stops());
        assert!(lib.has_xcb_render_create_linear_gradient_stops_length());
        assert!(lib.has_xcb_render_create_linear_gradient_stops_end());
        assert!(lib.has_xcb_render_create_linear_gradient_colors());
        assert!(lib.has_xcb_render_create_linear_gradient_colors_length());
        assert!(lib.has_xcb_render_create_linear_gradient_colors_iterator());
        assert!(lib.has_xcb_render_create_radial_gradient_sizeof());
        assert!(lib.has_xcb_render_create_radial_gradient_checked());
        assert!(lib.has_xcb_render_create_radial_gradient());
        assert!(lib.has_xcb_render_create_radial_gradient_stops());
        assert!(lib.has_xcb_render_create_radial_gradient_stops_length());
        assert!(lib.has_xcb_render_create_radial_gradient_stops_end());
        assert!(lib.has_xcb_render_create_radial_gradient_colors());
        assert!(lib.has_xcb_render_create_radial_gradient_colors_length());
        assert!(lib.has_xcb_render_create_radial_gradient_colors_iterator());
        assert!(lib.has_xcb_render_create_conical_gradient_sizeof());
        assert!(lib.has_xcb_render_create_conical_gradient_checked());
        assert!(lib.has_xcb_render_create_conical_gradient());
        assert!(lib.has_xcb_render_create_conical_gradient_stops());
        assert!(lib.has_xcb_render_create_conical_gradient_stops_length());
        assert!(lib.has_xcb_render_create_conical_gradient_stops_end());
        assert!(lib.has_xcb_render_create_conical_gradient_colors());
        assert!(lib.has_xcb_render_create_conical_gradient_colors_length());
        assert!(lib.has_xcb_render_create_conical_gradient_colors_iterator());
    }
}
