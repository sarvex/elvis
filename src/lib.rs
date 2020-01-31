#![feature(try_trait)]
#![feature(box_syntax)]
mod err;
mod serde;
mod tree;
mod values;
mod widgets;

pub use crate::{
    err::Error,
    serde::Serde,
    tree::Tree,
    values::{color::Colors, unit::Unit},
    widgets::text::{Text, TextStyle},
};

// elvis platform features
#[cfg(feature = "web")]
pub mod web;
mod features {
    #[cfg(feature = "web")]
    pub use crate::web;
}
