use std::fs::*;
use std::io::prelude::*;
use std::process::*;
use windows_bindgen::*;

fn main() {
    println!("cargo:rerun-if-changed=src/component.idl");
    let metadata_dir = format!("{}\\System32\\WinMetadata", env!("windir"));

    let status = Command::new("midlrt.exe")
        .arg("/winrt")
        .arg("/nomidl")
        .arg("/h")
        .arg("nul")
        .arg("/metadata_dir")
        .arg(&metadata_dir)
        .arg("/reference")
        .arg(format!("{}\\Windows.Foundation.winmd", metadata_dir))
        .arg("/winmd")
        .arg("component.winmd")
        .arg("src/component.idl")
        .status()
        .unwrap();

    if !status.success() {
        panic!();
    }

    // TODO: should be able to pass this directly to `bindgen`
    // https://github.com/microsoft/windows-rs/issues/1406
    copy("component.winmd", ".windows/winmd/component.winmd").unwrap();

    let gen = Gen {
        namespace: "Component",
        component: true,
        ..Default::default()
    };

    let mut bindings = File::create("src/bindings.rs").unwrap();
    bindings.write_all(gen_namespace(&gen).as_bytes()).unwrap();

    bindings
        .write_all(gen_namespace_impl(&gen).as_bytes())
        .unwrap();

    drop(bindings);

    Command::new("rustfmt")
        .arg("src/bindings.rs")
        .status()
        .unwrap();
}
