// Bible resource
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Bible {
    pub name: String,
    file_path: PathBuf,
}
