pub mod parser;
pub mod node;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(target_arch = "wasm32")]
pub use wasm::MarkupParserWrapper;
