mod client;
mod error;
pub mod models;

pub use client::{Client, ClientSettings};
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
