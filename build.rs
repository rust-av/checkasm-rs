// SPDX-License-Identifier: MIT

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn rerun_dir<P: AsRef<Path>>(dir: P) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        println!("cargo:rerun-if-changed={}", path.display());

        if path.is_dir() {
            rerun_dir(path)?;
        }
    }

    Ok(())
}

fn make_config_file<'a, P, I>(path: P, define_sigil: &'a str, defines: I) -> std::io::Result<()>
where
    P: AsRef<Path>,
    I: IntoIterator<Item = &'a str>,
{
    let mut config_file = fs::File::create(path)?;
    for define in defines {
        write!(config_file, "{}define {}\n", define_sigil, define)?;
    }
    Ok(())
}

fn build_nasm_files() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("config.asm");

    let mut defines = vec![
        "private_prefix checkasm",
        "ARCH_X86_32 0",
        "ARCH_X86_64 1",
        "PIC 1",
        "STACK_ALIGNMENT 16",
        "HAVE_AVX512ICL 1",
    ];

    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "macos" {
        defines.push("PREFIX 1");
    }

    make_config_file(&dest_path, "%", defines).expect("Cannot generate the config file");

    let asm_files = &["src/x86/checkasm.asm"];

    nasm_rs::Build::new()
        .files(asm_files)
        .include(out_dir)
        .include("src")
        .compile("checkasm-x86_64")
        .expect("NASM build failed. Make sure you have nasm installed. https://nasm.us");

    println!("cargo:rustc-link-lib=static=checkasm-x86_64");
    rerun_dir("src/x86").unwrap();
}

fn build_asm_files() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let dest_path = Path::new(&out_dir).join("config.h");
    let mut defines = vec![
        "PRIVATE_PREFIX checkasm_",
        "ARCH_AARCH64 1",
        "ARCH_ARM 0",
        "CONFIG_LOG 1",
        "HAVE_ASM 1",
    ];

    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "macos" {
        defines.push("PREFIX 1");
    }

    make_config_file(dest_path, "#", defines).expect("Cannot generate the config file");

    let asm_files = &["src/arm/checkasm_64.S"];

    cc::Build::new()
        .files(asm_files)
        .include(".")
        .include(&out_dir)
        .compile("checkasm-aarch64");

    rerun_dir("src/arm").unwrap();
}

fn main() {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    match arch.as_str() {
        "x86_64" => build_nasm_files(),
        "aarch64" => build_asm_files(),
        _ => {} // platform not supported yet
    }
}
