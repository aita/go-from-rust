use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("go")
        .args(&["build", "-buildmode=c-archive", "-o"])
        .arg(&format!("{}/libgohello.a", out_dir))
        .current_dir("go")
        .status()
        .unwrap();

    let bindings = bindgen::Builder::default()
        .header(format!("{}/libgohello.h", out_dir))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(format!("{}/bindings.rs", out_dir))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=gohello");
}
