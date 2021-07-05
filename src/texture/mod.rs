use std::sync::Arc;

mod texture;
pub use self::texture::*;

pub type AnyTexture = Arc<dyn Texture + Sync + Send>;
