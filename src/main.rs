use std::{
    env,
    fs::File,
    io::Write, path,
};

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();
        
    // Initial compiler flags as recommended by the Open Source Security 
    // Foundation (OpenSSF) Best Practices Working Group, 2025-01-23
    // Mutable vector so that we can add or remove compiler flags!
    let mut cflags: Vec<&str> = vec![
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
    
    let cur_dir: std::path::PathBuf = env::current_dir()?;

    let mut src_files: Vec<String> = Vec::new();


    for iterator in std::fs::read_dir(cur_dir)? {
        let iterator: std::fs::DirEntry = iterator?;
        if iterator.path().is_dir() {

        }

        else if iterator.path().is_file() {
            if let Some("c") = iterator.path().extension().and_then(|ext: &std::ffi::OsStr| ext.to_str()) {
                if let Some(path_str) = iterator.path().to_str() {
                    src_files.push(path_str.to_string());
                }
            }
        }
    }

    dbg!(src_files);

    let build: &str = "build main.o: cc main.c \nbuild main: ld main.o";
    
    let mut file: File = File::create("build.ninja")?;
    writeln!(file, "{}", cflags.join(" "))?;
    writeln!(file, "rule cc")?;
    writeln!(file, "  command = gcc $cflags -c $in -o $out")?;
    writeln!(file, "rule ld")?;
    writeln!(file, "  command = gcc $cflags $in -o $out")?;
    writeln!(file, "{}", build)?;
    Ok(())
}
