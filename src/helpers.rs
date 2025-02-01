use std::{
    fs, 
    ffi,
    io
};

pub(crate) fn collect_files(path: &std::path::PathBuf, src_files: &mut Vec<String>) -> io::Result<()> {

    for entry in fs::read_dir(path)? {
        let entry: fs::DirEntry = entry?;
        let path: std::path::PathBuf = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|ext: &ffi::OsStr| ext.to_str()) {
                if ext == "c" {
                    if let Some(path_str) = path.to_str() {
                        src_files.push(path_str.to_string());
                    }
                }
            }
        }

        else if path.is_dir() {
            collect_files(&path, src_files);
        }
    }

    Ok(())
}    
