use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("gcc")
        .args(&["src/lib.c", "-c", "-fPIC", "-o"])
        .arg(&format!("{}/cacheflush.o", out_dir))
        .status()
        .unwrap();
    Command::new("ar")
        .args(&["crus", "libcacheflush.a", "cacheflush.o"])
        .current_dir(&Path::new(&out_dir))
        .status()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=dylib=cacheflush");
    println!("cargo:rerun-if-changed=src/lib.c");
}
