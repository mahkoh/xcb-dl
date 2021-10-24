use std::io::{self, Write};

use crate::codegen::{tit_cap, tit_dig_split};

pub fn type_name(typ: &str) -> String {
    match typ {
        "CARD8" => "u8".into(),
        "CARD16" => "u16".into(),
        "CARD32" => "u32".into(),
        "CARD64" => "u64".into(),
        "INT8" => "i8".into(),
        "INT16" => "i16".into(),
        "INT32" => "i32".into(),
        "BYTE" => "u8".into(),
        "BOOL" => "bool".into(),
        "char" => "c_char".into(),
        "float" => "f32".into(),
        "double" => "f64".into(),
        typ => tit_cap(typ),
    }
}

pub fn opname(name: &str) -> String {
    tit_dig_split(name).to_ascii_uppercase()
}

pub fn emit_opcode<Out: Write>(out: &mut Out, name: &str, opcode: i32) -> io::Result<()> {
    let opname = opname(name);
    let num_typ = if opcode < 0 { "i8" } else { "u8" };

    writeln!(out)?;
    writeln!(out, "pub const {}: {} = {};", &opname, &num_typ, opcode)?;

    Ok(())
}

pub fn emit_error<Out: Write>(out: &mut Out, ffi_typ: &str, rs_typ: &str) -> io::Result<()> {
    writeln!(out)?;
    writeln!(out, "pub struct {} {{", &rs_typ)?;
    writeln!(out, "    pub base: base::Error<{}>,", &ffi_typ)?;
    writeln!(out, "}}")?;

    Ok(())
}
