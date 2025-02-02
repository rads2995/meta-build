pub(crate) mod helpers;

fn main() -> std::io::Result<()> {

    use helpers::{
        collect_files,
        create_build_file,
    };
    
    // TODO: implement something to do with input arguments
    let _args: Vec<String> = std::env::args().collect();
        
    // Initial compiler flags as recommended by the Open Source Security 
    // Foundation (OpenSSF) Best Practices Working Group, 2025-01-23
    // Mutable vector so that we can add or remove compiler flags!
    let cflags: Vec<&str> = vec![
        "cflags =",
        "-O2",
        "-Wall",
        "-Wformat",
        "-Wformat=2",
        "-Wconversion",
        "-Wimplicit-fallthrough",
        "-Werror=format-security",
        "-U_FORTIFY_SOURCE",
        "-D_FORTIFY_SOURCE=3",
        "-D_GLIBCXX_ASSERTIONS",
        "-fstrict-flex-arrays=3",
        "-fstack-clash-protection",
        "-fstack-protector-strong",
        "-Wl,-z,nodlopen",
        "-Wl,-z,noexecstack",
        "-Wl,-z,relro -Wl,-z,now",
        "-Wl,--as-needed",
        "-Wl,--no-copy-dt-needed-entries"
    ];
    
    let mut src_files: Vec<std::path::PathBuf> = Vec::new();
    let mut header_files: Vec<std::path::PathBuf> = Vec::new();
    let cur_dir: std::path::PathBuf = std::env::current_dir()?;
    
    collect_files(&cur_dir, &mut src_files, &mut header_files)?;

    let mut src_names: Vec<String> = src_files.iter()
    .filter_map(|file_path: &std::path::PathBuf| file_path.file_name().map(
        |name: &std::ffi::OsStr| name.to_string_lossy().to_string())
    )
    .collect();
    
    let build: String = format!("build main: ld {}", src_names.join(" ").replace("c", "o"));

    src_names.iter_mut().for_each(|file: &mut String| {
        *file = format!("build {}: cc {}", file.replace(".c", ".o"), file);
    });
    
    header_files.iter_mut().for_each(|file_path: &mut std::path::PathBuf| {
        if let Some(parent_dir) = file_path.parent() {
            *file_path = parent_dir.to_path_buf();
        }
    });
    
    header_files.dedup();

    let header_paths: Vec<String> = header_files.iter()
        .map(|file_path: &std::path::PathBuf| format!(
            "-I{}", file_path.to_string_lossy().into_owned())
        )
        .collect();

    create_build_file(&cflags, &src_names, &header_paths, &build)?;
    
    Ok(())

}
