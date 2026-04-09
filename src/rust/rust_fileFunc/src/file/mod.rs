use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::fs;
use crate::open_file::{OpenError, OpenFile};

#[repr(C)]
pub struct FileInfo {
    pub path: PathBuf,
    pub file_name: String,
    pub size: u64,
    pub is_readonly: bool,
    pub created: Option<SystemTime>,
    pub modified: Option<SystemTime>,
}

impl FileInfo {
    pub fn open_this(&self) -> Result<(), OpenError> {
        OpenFile::open_static(self)
    }
    pub fn from_path<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let path_ref = path.as_ref();
        let metadata = fs::metadata(path_ref)?;

        let file_name = path_ref
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or_else(|| path_ref.to_str()
                .unwrap_or("Unknown"))
            .to_string();

        Ok(FileInfo {
            path: path_ref.to_path_buf(),
            file_name,
            size: metadata.len(),
            is_readonly: metadata.permissions().readonly(),
            created: metadata.created().ok(),
            modified: metadata.modified().ok(),
        })
    }
}