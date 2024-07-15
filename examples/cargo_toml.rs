use cargo_toml::Manifest;



fn main ()->anyhow::Result<()> {

    let toml = include_bytes!("/Users/kingzcheung/rust/bdzer/Cargo.toml");
    let manifest = Manifest::from_slice(toml)?;


    Ok(())
}