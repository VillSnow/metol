mod common;
mod desc_types;
mod new_type;

pub mod client;
pub mod mi_api_types;
pub mod mi_entities;
pub mod receiving_ws_msg;
pub mod sending_ws_msg;
pub mod utils;

pub use client::Client;
pub use common::Real;
pub use desc_types::MiChannel;
pub use new_type::{IntType, StringType};

#[cfg(feature = "parser")]
pub mod parser;
