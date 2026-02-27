#![deny(clippy::all)]
#![feature(impl_trait_in_bindings)]
#![feature(nonpoison_mutex)]
#![feature(sync_nonpoison)]
pub mod manifest;
pub mod ssl;
pub mod versions;

pub use manifest::{CHUNK_SIZE, MAX_FILE_COUNT};

#[cfg(test)]
pub mod tests;
