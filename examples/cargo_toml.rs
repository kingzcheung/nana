use std::fs;

use cargo_toml::Manifest;



fn main ()->anyhow::Result<()> {

    let toml = include_bytes!("/Users/kingzcheung/rust/bdzer/Cargo.toml");
    let mut manifest = Manifest::from_slice(toml)?;

    // dbg!(manifest.dependencies);
    manifest.dependencies.insert("agc".to_string(), cargo_toml::Dependency::Simple("0.0.0".to_owned()));
    
    let toml_str = toml::to_string(&manifest).unwrap();
    let dest_path = "./output/test_cargo.toml";
    fs::write(dest_path, toml_str).unwrap();
    Ok(())
}