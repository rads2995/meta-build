pub(crate) mod helpers;

use std::io::Write;

use helpers::collect_files;

fn main() -> std::io::Result<()> {

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

    let build: &str = "build main.o: cc main.c \nbuild main: ld main.o";
    
    header_files.iter_mut().for_each(|file_path: &mut std::path::PathBuf| {
        if let Some(parent_dir) = file_path.parent() {
            *file_path = parent_dir.to_path_buf();
        }
    });

    header_files.dedup();

    // Convert header_files (PathBuf) into Vec<&str>
    let header_paths: Vec<String> = header_files.iter()
        .map(|file_path: &std::path::PathBuf| format!("-I{}", file_path.to_string_lossy().into_owned()))
        .collect();

    let mut file: std::fs::File = std::fs::File::create("build.ninja")?;
    writeln!(file, "{} {}", cflags.join(" "), header_paths.join(" "))?;
    writeln!(file, "rule cc")?;
    writeln!(file, "  depfile = $out.d")?;
    writeln!(file, "  command = gcc -MD -MF $out.d $cflags -I. -c $in -o $out")?;
    writeln!(file, "rule ld")?;
    writeln!(file, "  command = gcc $cflags $in -o $out")?;
    writeln!(file, "{}", build)?;
    Ok(())
}
