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

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=gohello");
}
