pub use color_eyre::*;
use lazy_static::lazy_static;
use serde::{Deserialize, Deserializer};
pub use tracing::*;

pub use cool::*;
pub use extension::*;
pub use trace::*;

mod cool;
mod error;
mod extension;
pub mod installer;
pub mod result;
pub mod shell;
pub mod state;
pub mod tasks;
mod trace;

lazy_static! {
    pub static ref DEFAULT_TERA_CONTEXT: tera::Context = tera::Context::default();
}

pub fn render_str<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(d).map(|s| s.render(&DEFAULT_TERA_CONTEXT, false).unwrap())
}
