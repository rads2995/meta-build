pub(crate) fn collect_files(
    path: &std::path::PathBuf, 
    src_files: &mut Vec<std::path::PathBuf>,
    header_files: &mut Vec<std::path::PathBuf>
) -> std::io::Result<()> {

    for entry in std::fs::read_dir(path)? {
        let entry: std::fs::DirEntry = entry?;
        let path: std::path::PathBuf = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|ext: &std::ffi::OsStr| ext.to_str()) {
                if ext == "c" {
                    src_files.push(path);
                }

                else if ext == "h" {
                    header_files.push(path);
                }
            }
        }

        else if path.is_dir() {
            collect_files(&path, src_files, header_files)?;
        }
    }

    Ok(())
}
