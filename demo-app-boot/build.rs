//! This build script copies the `link.ld` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `link.ld` is changed,
//! updating `link.ld` ensures a rebuild of the application with the
//! new memory settings.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Put `link.ld` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("link.ld"))
        .unwrap()
        .write_all(include_bytes!("link.ld"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // Command::new("arm-none-eabi-objcopy").args([
    //     "--prefix-symbols=MYMAGICPREFIX",
    //     &out.join("../../../libdemo_app.a").display().to_string(),
    //     &out.join("libdemo_app.a").display().to_string(),
    // ]).status().expect("objcopy failed");
    
    let status = Command::new("arm-none-eabi-ld").args([
        "-relocatable",
        "--whole-archive",
        // "--no-gc-sections",
        // "--export-dynamic-symbol", "run_me_from_ddr_too",
        "-o",
        &out.join("libdemo_app_intermediate.a").display().to_string(),
        &out.join("../../../libdemo_app.a").display().to_string(),
    ]).status().expect("ld reloc failed");
    assert!(status.success());
    println!("{}", out.join("libdemo_appmodified.a").display().to_string());

    let status = Command::new("arm-none-eabi-objcopy").args([
        "--redefine-sym", "rust_begin_unwind=rust_begin_unwind_lib",
        "--redefine-sym", "_ZN4core3fmt5write17hb5d6751d706a0b13E=_ZN4core3fmt5write17hb5d6751d706a0b13ERENAMED",
        // "--no-gc-sections",
        // "--export-dynamic-symbol", "run_me_from_ddr_too",
        &out.join("libdemo_app_intermediate.a").display().to_string(),
        &out.join("libdemo_appmodified.a").display().to_string(),
    ]).status().expect("objcopy failed");
    

    // Enable linking against final main app lib artifact
    // TODO: is there a better way to point to this file? Maybe move link attribute here and use a full path.
    let target_dir = out.join("../../../");
    println!("cargo:rustc-link-search=native={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `link.ld`
    // here, we ensure the build script is only re-run when
    // `link.ld` is changed.
    println!("cargo:rerun-if-changed=link.ld");
    println!("cargo:rerun-if-changed={}", out.join("../../../libdemo_app.a").display());
}
