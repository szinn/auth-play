pub mod error;
pub mod http;

pub use error::*;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../../frontend/build"]
struct Dist;
