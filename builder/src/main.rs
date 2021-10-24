extern crate quick_xml;

mod ast;
mod codegen;
mod output;
mod parse;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use ast::{Event, ExtInfo, OpCopy, OpCopyMap};
use codegen::{CodeGen, DepInfo};
use output::Output;
use parse::{Parser, Result};

fn xcb_mod_map(name: &str) -> &str {
    match name {
        "bigreq" => "big_requests",
        "ge" => "genericevent",
        "xselinux" => "selinux",
        "xprint" => "x_print",
        "xtest" => "test",
        _ => name,
    }
}

fn is_always(name: &str) -> bool {
    matches!(name, "xproto" | "big_requests" | "xc_misc")
}

fn has_feature(name: &str) -> bool {
    env::var(format!("CARGO_FEATURE_{}", name.to_ascii_uppercase())).is_ok()
}

fn main() {
    let xml_dir = Path::new("../../../c/xcbproto/src");
    let out_dir = Path::new("../src/headers/generated");

    let rustfmt = None;
    let gen_all = true;

    let mut dep_info = Vec::new();

    for xml_file in iter_xml(&xml_dir) {
        process_xcb_gen(&xml_file, out_dir, &rustfmt, gen_all, &mut dep_info).unwrap_or_else(
            |err| {
                panic!(
                    "Error during processing of {}: {:?}",
                    xml_file.display(),
                    err
                )
            },
        );
    }
}

fn iter_xml(xml_dir: &Path) -> impl Iterator<Item = PathBuf> {
    fs::read_dir(xml_dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| match p.extension() {
            Some(e) => e == "xml",
            _ => false,
        })
}

fn process_xcb_gen(
    xml_file: &Path,
    out_dir: &Path,
    rustfmt: &Option<PathBuf>,
    gen_all: bool,
    dep_info: &mut Vec<DepInfo>,
) -> Result<()> {
    let xcb_mod = xml_file.file_stem().unwrap();
    let xcb_mod = xcb_mod.to_str().unwrap();
    let xcb_mod = xcb_mod_map(xcb_mod);

    if dep_info.iter().any(|di| di.xcb_mod == xcb_mod) {
        return Ok(());
    }

    if !gen_all && !is_always(xcb_mod) && !has_feature(xcb_mod) {
        return Ok(());
    }

    let ffi_file = out_dir.join("ffi").join(&xcb_mod).with_extension("rs");
    let rs_file = out_dir.join(&xcb_mod).with_extension("rs");

    let ffi = Output::new(rustfmt, &ffi_file)
        .unwrap_or_else(|_| panic!("cannot create FFI output file: {}", ffi_file.display()));
    let rs = Output::new(rustfmt, &rs_file)
        .unwrap_or_else(|_| panic!("cannot create Rust output file: {}", rs_file.display()));

    let mut parser = Parser::from_file(xml_file);

    let mut imports = Vec::new();
    let mut events = Vec::new();
    let mut evcopies: OpCopyMap = OpCopyMap::new();
    let mut info: Option<(String, Option<ExtInfo>)> = None;

    for e in &mut parser {
        match e? {
            Event::Ignore => {}
            Event::Info(mod_name, ext_info) => {
                info = Some((mod_name, ext_info));
            }
            Event::Import(imp) => imports.push(imp),
            Event::Event {
                number,
                stru,
                no_seq_number,
                xge,
            } => {
                evcopies.insert(stru.name.clone(), Vec::new());
                events.push(Event::Event {
                    number,
                    stru,
                    no_seq_number,
                    xge,
                });
            }
            Event::EventCopy { name, number, ref_ } => {
                if let Some(copies) = evcopies.get_mut(&ref_) {
                    copies.push(OpCopy { name, number });
                } else {
                    events.push(Event::EventCopy { name, number, ref_ });
                }
            }
            ev => {
                events.push(ev);
            }
        }
    }

    let info = info.expect("no xcb protocol opening");

    let deps = {
        let mut deps = Vec::new();

        for i in imports.iter() {
            let xml_file = xml_file.with_file_name(&format!("{}.xml", i));

            // panic also from here to have the correct xml_file reported
            process_xcb_gen(&xml_file, out_dir, rustfmt, gen_all, dep_info).unwrap_or_else(|err| {
                panic!(
                    "Error during processing of {}: {:?}",
                    xml_file.display(),
                    err
                )
            });

            let i = xcb_mod_map(i);
            deps.push(
                dep_info
                    .iter()
                    .find(|di| di.xcb_mod == i)
                    .unwrap_or_else(|| panic!("can't find dependency {} of {}", i, xcb_mod))
                    .clone(),
            );
        }

        deps
    };

    let mut cg = CodeGen::new(xcb_mod, ffi, rs, deps, evcopies);

    cg.prologue(imports, &info.1)?;

    for ev in events {
        cg.event(ev)?;
    }

    cg.epilogue()?;

    dep_info.push(cg.into_depinfo());

    Ok(())
}
