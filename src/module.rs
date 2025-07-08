use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use inflector::Inflector;

pub fn add_module(project: &str, name: &str) {
    let parts: Vec<&str> = name.split('/').collect();
    let module = parts.last().unwrap();
    let folder = format!("{}/src/{}", project, parts[..parts.len()-1].join("/"));
    let mod_path = format!("{}/mod.rs", folder);
    let file_path = format!("{}/{}.rs", folder, module);

    fs::create_dir_all(&folder).unwrap();
    File::create(&file_path).unwrap();

    if Path::new(&mod_path).exists() {
        let mut mod_file = OpenOptions::new().append(true).open(&mod_path).unwrap();
        writeln!(mod_file, "pub mod {};", module).unwrap();
    } else {
        let mut mod_file = File::create(&mod_path).unwrap();
        writeln!(mod_file, "pub mod {};", module).unwrap();
    }
}

pub fn add_service(project: &str, name: &str, trait_name: Option<String>) {
    let file_path = format!("{}/src/services/{}.rs", project, name);
    let mut file = File::create(&file_path).unwrap();

    let struct_name = name.to_class_case();
    let trait_impl = if let Some(trait_name) = trait_name {
        format!(
            "
impl {} for {} {{
    // TODO: Implement trait methods
}}",
            trait_name, struct_name
        )
    } else {
        String::new()
    };

    writeln!(
        file,
        "pub struct {} {{}}\n\nimpl {} {{\n    pub fn new() -> Self {{ Self {{}} }}\n}}{}\n",
        struct_name, struct_name, trait_impl
    ).unwrap();

    let mod_path = format!("{}/src/services/mod.rs", project);
    let mut mod_file = OpenOptions::new().append(true).open(mod_path).unwrap();
    writeln!(mod_file, "pub mod {};", name).unwrap();
}
