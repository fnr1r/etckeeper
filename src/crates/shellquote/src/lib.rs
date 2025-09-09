mod consts;
pub mod escape;
pub mod eval;
pub mod ext;
mod tests;

pub use escape::{singlequote_escape, singlequote_unescape};
pub use ext::ExtStrEscapeSinglequote;
