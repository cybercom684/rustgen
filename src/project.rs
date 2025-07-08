use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

pub fn create_lib(name: &str) {
    Command::new("cargo")
        .args(["new", "--lib", name])
        .status()
        .expect("Failed to create project");

    let base = format!("{name}/src");
    let folders = ["models", "traits", "services"];

    for folder in folders {
        let path = format!("{base}/{folder}");
        fs::create_dir_all(&path).unwrap();
        File::create(format!("{path}/mod.rs")).unwrap();
    }

    let lib_path = format!("{base}/lib.rs");
    let mut lib = File::create(lib_path).unwrap();
    writeln!(lib, "pub mod models;\npub mod traits;\npub mod services;").unwrap();
}