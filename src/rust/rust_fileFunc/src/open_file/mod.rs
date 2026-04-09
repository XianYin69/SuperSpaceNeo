use std::process::Command;
use std::path::Path;
use crate::file::FileInfo;

#[derive(Debug)]
pub enum OpenError {
    FileNotFound,
    NoAssociatedProgram,
    PermissionDenied,
    Unknown(String),
}

pub struct OpenFile;

impl OpenFile {
    pub fn open_static(info: &FileInfo) -> Result<(), OpenError> {
        Self::execute_open(&info.path)
    }

    fn execute_open(path: &Path) -> Result<(), OpenError> {
        if !path.exists() {
            return Err(OpenError::FileNotFound);
        }

        let path_str = path.to_str().ok_or_else(|| OpenError::Unknown("Invalid path".to_string()))?;

        #[cfg(target_os = "windows")]
        let (cmd, args) = ("cmd", vec!["/C", "start", "", path_str]);
        #[cfg(target_os = "linux")]
        let (cmd, args) = ("xdg-open", vec![path_str]);

        match Command::new(cmd).args(&args).spawn() {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Err(OpenError::FileNotFound),
                std::io::ErrorKind::PermissionDenied => Err(OpenError::PermissionDenied),
                _ => Err(OpenError::Unknown(e.to_string())),
            }
        }
    }
}
