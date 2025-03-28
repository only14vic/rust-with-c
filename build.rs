use {
    dotenv::dotenv,
    std::{
        env::{self},
        ffi::OsStr,
        fs::create_dir_all,
        path::PathBuf,
        process::Command
    }
};

fn main() {
    dotenv().ok();

    //
    // Configuration
    //
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=include/include.h");
    println!("cargo:rerun-if-changed=cbindgen.toml");

    let out_path =
        PathBuf::from_iter([&env::var("CARGO_MANIFEST_DIR").unwrap(), "include"]);

    let target_dir = format!(
        "{}/{}",
        env::var("CARGO_TARGET_DIR").unwrap(),
        env::var("PROFILE").unwrap()
    );

    //
    // Linking libraries
    //
    println!("cargo::rustc-link-search={target_dir}");
    println!("cargo::rustc-link-lib=inih");
    //println!("cargo::rustc-link-lib=app_nostd");

    //
    // Binding C code
    //
    let bindings = bindgen::Builder::default()
        .blocklist_type("__BindgenBitfieldUnit")
        .blocklist_type("_IO_FILE")
        .blocklist_type("_IO_marker")
        .blocklist_type("_IO_codecvt")
        .blocklist_type("_IO_wide_data")
        .blocklist_type("_IO_lock_t")
        .blocklist_type("__off_t")
        .blocklist_type("__off64_t")
        .blocklist_type("FILE")
        .use_core()
        .header("include/include.h")
        .allowlist_item("ini_.*")
        .blocklist_function("ini_parse_file")
        .generate()
        .expect("Unable to generate bindings");

    create_dir_all(out_path.as_path())
        .expect(&format!("Couldn't create directory: {out_path:?}"));

    let bindings_file =
        PathBuf::from_iter([out_path.as_os_str(), OsStr::new("bindings.rs")]);

    bindings
        .write_to_file(&bindings_file)
        .expect("Couldn't write bindings!");

    let output = Command::new("rustup")
        .args(["run", "nightly", "rustfmt", bindings_file.to_str().unwrap()])
        .output()
        .expect("Could not format binding file.");

    assert!(
        output.status.success(),
        "Unsuccessful status code when running `rustfmt`: {output:?}",
    );

    //println!("cargo:warning={:?} was formatted successfully.", &out_path);

    //
    // Binding Rust code
    //
    let cbindgens_filename = PathBuf::from_iter([
        out_path.as_os_str(),
        OsStr::new(&format!(
            "lib{}.h",
            env::var("CARGO_PKG_NAME").unwrap().replace("-", "_")
        ))
    ]);

    cbindgen::Builder::new()
        .with_config(cbindgen::Config::from_file("cbindgen.toml").unwrap())
        .with_crate(env::var("CARGO_MANIFEST_DIR").unwrap())
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(cbindgens_filename);
}
