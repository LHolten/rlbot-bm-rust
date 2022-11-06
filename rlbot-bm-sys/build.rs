use std::env;
use std::path::PathBuf;

fn main() {
    if cfg!(feature = "dynamic") || cfg!(not(windows)) {
        println!("cargo:rustc-link-lib=dylib=RLBot-BM");
    } else {
        let dst = cmake::Config::new("RLBot-BM")
            .define("ENABLE_IPO", "OFF")
            .build_target("all")
            .build();

        println!("cargo:rustc-link-search=native={}\\build", dst.display());
        println!("cargo:rustc-link-lib=static=RLBot-BM");
    }

    println!("cargo:rustc-link-lib=dylib=stdc++");

    let bindings = bindgen::Builder::default()
        .header("RLBot-BM/src/RLBotBM_c.h")
        .derive_default(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .layout_tests(false)
        .newtype_enum("DropShotObj_TileState")
        .clang_arg("-fdeclspec")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
