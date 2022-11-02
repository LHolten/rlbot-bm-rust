use std::env;
use std::path::PathBuf;

fn main() {
    let dst = cmake::Config::new("RLBot-BM")
        .define("CMAKE_C_COMPILER", "clang")
        .define("CMAKE_CXX_COMPILER", "clang++")
        .build_target("all")
        .build();

    println!("cargo:rustc-link-search=native={}\\build", dst.display());
    println!("cargo:rustc-link-lib=static=RLBot-BM");
    println!("cargo:rustc-link-lib=dylib=stdc++");

    let bindings = bindgen::Builder::default()
        .header("RLBot-BM/src/RLBotBM_c.h")
        .derive_default(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .layout_tests(false)
        .newtype_enum("DropShotObj_TileState")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
