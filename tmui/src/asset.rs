#[derive(rust_embed::RustEmbed)]
#[folder = "resources/svg"]
#[include = "*.svg"]
pub(crate) struct Asset;