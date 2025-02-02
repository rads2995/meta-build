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

pub(crate) fn create_build_file(cflags: &Vec<&str>, lflags: &Vec<&str>, src_names: &[String], header_paths: &[String], artifact: &String) -> std::io::Result<()> {

    use std::io::Write;
    
    let mut file: std::fs::File = std::fs::File::create("build.ninja")?;
    writeln!(file, "{} {}", cflags.join(" "), header_paths.join(" "))?;
    writeln!(file, "{}", lflags.join(" "))?;
    writeln!(file, "rule cc")?;
    writeln!(file, "  depfile = $out.d")?;
    writeln!(file, "  command = gcc -MD -MF $out.d $cflags -c $in -o $out")?;
    writeln!(file, "rule ld")?;
    writeln!(file, "  command = gcc $in -o $out $lflags")?;
    writeln!(file, "{}", src_names.join("\n"))?;
    writeln!(file, "{}", artifact)?;

    Ok(())
}
