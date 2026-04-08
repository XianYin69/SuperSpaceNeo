mod file;
mod open_file;
mod ffi;

use crate::file::FileInfo;
use crate::open_file::{OpenFile, OpenError};
use std::path;
use std::path::Path;

fn process_file_request<P: AsRef<Path>>(
    path: P,
    is_open: bool,
) -> (Option<FileInfo>, Option<Result<(), OpenError>>) {
    let file_info = match FileInfo::from_path(path.as_ref().to_path_buf()) {
        Ok(info) => Some(info),
        Err(e) => {
            eprint!("Error: {}", e);
            return (None, None);
        }
    };

    let mut open_result = None;
    if let Some(ref info) = file_info {
        if is_open {
            open_result = Some(info.open_this());
        }
    }

    (file_info, open_result)
}
fn main() {
    println!("Rust test");
}