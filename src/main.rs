pub(crate) mod helpers;

fn main() -> std::io::Result<()> {

    use helpers::{
        collect_files,
        create_build_file,
    };

    // Initial compiler flags as recommended by the Open Source Security 
    // Foundation (OpenSSF) Best Practices Working Group, 2025-01-23
    let mut cflags: Vec<&str> = vec![
        "cflags =",
        "-O2",
        "-Wall",
        "-Wextra",
        "-Wformat",
        "-Wformat=2",
        "-Wconversion",
        "-Wsign-conversion",
        "-Wtrampolines",
        "-Wimplicit-fallthrough",
        "-Wbidi-chars=any",
        "-Werror=format-security",
        "-Werror=implicit",
        "-Werror=incompatible-pointer-types",
        "-Werror=int-conversion",
        "-fstrict-flex-arrays=3",
        "-fstack-clash-protection",
        "-fstack-protector-strong",
        "-fcf-protection=full",
        "-fno-delete-null-pointer-checks",
        "-fno-strict-overflow",
        "-fno-strict-aliasing",
        "-ftrivial-auto-var-init=zero",
        "-fexceptions",
    ];
    
    // Initial compiler flags as recommended by the Open Source Security 
    // Foundation (OpenSSF) Best Practices Working Group, 2025-01-23    
    let lflags: Vec<&str> = vec![
        "lflags =",
        "-Wl,-z,nodlopen",
        "-Wl,-z,noexecstack",
        "-Wl,-z,relro",
        "-Wl,-z,now",
        "-Wl,--as-needed",
        "-Wl,--no-copy-dt-needed-entries",
    ];

    let cur_dir: std::path::PathBuf = std::env::current_dir()?;
    let mut src_files: Vec<std::path::PathBuf> = Vec::new();
    let mut header_files: Vec<std::path::PathBuf> = Vec::new();
    collect_files(&cur_dir, &mut src_files, &mut header_files)?;

    let mut src_names: Vec<String> = src_files.iter()
    .filter_map(|file_path: &std::path::PathBuf| file_path.file_name().map(
        |name: &std::ffi::OsStr| name.to_string_lossy().to_string())
    )
    .collect();
    
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
    
    let args: Vec<String> = std::env::args().collect();
    
    let mut artifact: String = String::new();
    let mut valid: bool = false;
    
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--executable" => {
                println!("\"{}\" argument found. Building executable.", arg);
                cflags.push("-fPIE -pie");
                artifact = format!("build main: ld {}", src_names.join(" ").replace(".c", ".o"));
                valid = true;
                break;
            },
            "--static-lib" => {
                println!("\"{}\" argument found. Building static library.", arg);
                artifact = format!("build lib.o: ld {}", src_names.join(" ").replace(".c", ".o"));
                valid = true;
                break;
            },
            "--shared-lib" => {
                println!("\"{}\" argument found. Building shared library.", arg);
                cflags.push("-fPIC -shared");
                artifact = format!("build lib.so: ld {}", src_names.join(" ").replace(".c", ".o"));
                valid = true;
                break;
            },
            "--src-files" => {
                println!("Found source files: {}", src_names.join(" "));
                return Ok(());
            },
            "--header-paths" => {
                println!("Found paths for header files: {}", header_paths.join(" "));
                return Ok(());
            }
            "-h" | "--help" => {
                println!("Help information goes here.");
                return Ok(());
            },
            _ => println!("Unknown argument passed \"{}\", ignoring.", arg),
        }
    } 

    src_names.iter_mut().for_each(|file: &mut String| {
        *file = format!("build {}: cc {}", file.replace(".c", ".o"), file);
    });
    
    if valid {
        create_build_file(&cflags, &lflags, &src_names, &header_paths, &artifact)?;
    }

    else {
        print!("Error: no valid arguments were passed.")
    }
    
    Ok(())

}
