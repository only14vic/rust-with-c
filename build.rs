use std::{env, path::PathBuf};

fn main() {
    // Not necessarily
    //println!("cargo::rustc-link-lib=ircclient");
    //println!("cargo::rustc-link-search=/usr/lib");

    println!("cargo:rerun-if-changed=build.rc");
    println!("cargo:rerun-if-changed=cbindgen.toml");
    println!("cargo:rerun-if-changed=/usr/include/libircclient/libircclient.h");

    let bindings = bindgen::Builder::default()
        .header("/usr/include/libircclient/libircclient.h")
        .allowlist_item("irc_.*")
        .use_core()
        .generate()
        .expect("Unable to generate bindings");

    let mut out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    out_path.push("irc_bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");

    cbindgen::Builder::new()
        .with_config(cbindgen::Config::from_file("cbindgen.toml").unwrap())
        .with_crate(env::var("CARGO_MANIFEST_DIR").unwrap())
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(format!(
            "lib{}.h",
            env::var("CARGO_PKG_NAME").unwrap().replace("-", "_")
        ));
}
