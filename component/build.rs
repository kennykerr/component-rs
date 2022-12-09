use std::fs::*;
use std::process::*;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=src/component.idl");
    let _ = std::fs::remove_file("src/bindings.rs");
    let _ = std::fs::remove_file("component.winmd");

    Command::new("midlrt.exe")
        .arg("/winrt")
        .arg("/nomidl")
        .arg("/h")
        .arg("nul")
        .arg("/metadata_dir")
        .arg(std::env::current_dir()?)
        .arg("/reference")
        .arg("windows.winmd")
        .arg("/winmd")
        .arg("component.winmd")
        .arg("src/component.idl")
        .status()?;

    let files = vec![
        windows_metadata::reader::File::new("Windows.winmd")?,
        windows_metadata::reader::File::new("component.winmd")?,
    ];

    write(
        "src/bindings.rs",
        windows_bindgen::component("Component", &files),
    )?;

    Command::new("rustfmt").arg("src/bindings.rs").status()?;
    Ok(())
}
