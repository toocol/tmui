use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "resources/"]
#[include = "*.svg"]
pub struct Asset;