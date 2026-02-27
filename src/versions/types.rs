use std::fmt::Debug;

use async_trait::async_trait;
use dyn_clone::DynClone;
use tokio::io::AsyncRead;

#[derive(Debug, Clone)]
pub struct VersionFile {
    pub relative_filename: String,
    pub permission: u32,
    pub size: u64,
}

pub trait MinimumFileObject: AsyncRead + Send + Unpin {}
impl<T: AsyncRead + Send + Unpin> MinimumFileObject for T {}

#[async_trait]
pub trait VersionBackend: DynClone {
    fn require_whole_files(&self) -> bool;
    async fn list_files(&mut self) -> anyhow::Result<Vec<VersionFile>>;
    async fn peek_file(&mut self, sub_path: String) -> anyhow::Result<VersionFile>;
    async fn reader(
        &self,
        file: &VersionFile,
        start: u64,
        end: u64,
    ) -> anyhow::Result<Box<dyn MinimumFileObject>>;
}

dyn_clone::clone_trait_object!(VersionBackend);
