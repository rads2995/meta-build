pub(crate) mod helpers;

fn main() -> std::io::Result<()> {

    use helpers::{
        collect_files,
        create_build_file,
    };

    // Compiler and linker flags as recommended by the Open Source 
    // Security Foundation Best Practices Working Group, 2025-01-23
    // Note: these can be overridden by flags passed as arguments
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
    let mut ldflags: Vec<&str> = vec![
        "ldflags =",
        "-Wl,-z,nodlopen",
        "-Wl,-z,noexecstack",
        "-Wl,-z,relro",
        "-Wl,-z,now",
        "-Wl,--as-needed",
        "-Wl,--no-copy-dt-needed-entries",
    ];

    // Perform recursive search for files from current directory
    let cur_dir: std::path::PathBuf = std::env::current_dir()?;
    let mut src_files: Vec<std::path::PathBuf> = Vec::new();
    let mut header_files: Vec<std::path::PathBuf> = Vec::new();
    collect_files(&cur_dir, &mut src_files, &mut header_files)?;

    // Create vector of Strings from vector of source files path buffers
    let mut src_names: Vec<String> = src_files.iter()
    .filter_map(|file_path: &std::path::PathBuf| file_path.file_name().map(
        |name: &std::ffi::OsStr| name.to_string_lossy().to_string())
    )
    .collect();
    
    // Remove file names from vector of header files path buffers
    header_files.iter_mut().for_each(|file_path: &mut std::path::PathBuf| {
        if let Some(parent_dir) = file_path.parent() {
            *file_path = parent_dir.to_path_buf();
        }
    });

    // Remove duplicate directory entries for header files path buffers
    header_files.dedup();

    // Create vector of Strings from vector of header files path buffers
    let header_paths: Vec<String> = header_files.iter()
        .map(|file_path: &std::path::PathBuf| format!(
            "-I{}", file_path.to_string_lossy().into_owned())
        )
        .collect();

    let args: Vec<String> = std::env::args().collect();
    let mut artifact: String = String::new();
    
    // Logic flags to control flow of arguments to main function
    let mut valid: bool = false;
    let mut collect_cflags: bool = false;
    let mut collect_ldflags: bool = false;
    
    for arg in args.iter().skip(1) {
        if collect_cflags {
            if arg.contains("--") {
                collect_cflags = false;
            }

            else if arg.contains("-") {
                cflags.push(arg);
            }

            else {
                println!("Unknown --cflags argument passed \"{}\", ignoring.", arg);
            }
        }

        else if collect_ldflags {
            if arg.contains("--") {
                collect_ldflags = false;
            }

            else if arg.contains("-") {
                ldflags.push(arg);
            }
            
            else {
                println!("Unknown --ldflags argument passed \"{}\", ignoring.", arg);
            }
        }
        
        match arg.as_str() {
            "--cflags" => {
                collect_cflags = true;
            },
            "--ldflags" => {
                collect_ldflags = true;
            },
            "--executable" => {
                println!("\"{}\" argument found. Building executable and ignoring subsequent flags.", arg);
                cflags.push("-fPIE -pie");
                artifact = format!("build main: ld {}", src_names.join(" ").replace(".c", ".o"));
                valid = true;
                break;
            },
            "--static-lib" => {
                println!("\"{}\" argument found. Building static library and ignoring subsequent flags.", arg);
                artifact = format!("build lib.o: ld {}", src_names.join(" ").replace(".c", ".o"));
                valid = true;
                break;
            },
            "--shared-lib" => {
                println!("\"{}\" argument found. Building shared library and ignoring subsequent flags.", arg);
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
            "--help" => {
                println!("Help information goes here.");
                return Ok(());
            },
            _ => {
                if !collect_cflags && !collect_ldflags {
                    println!("Unknown argument passed \"{}\", ignoring.", arg);
                }
            },
        }
    } 

    src_names.iter_mut().for_each(|file: &mut String| {
        *file = format!("build {}: cc {}", file.replace(".c", ".o"), file);
    });
    
    if valid {
        create_build_file(&cflags, &ldflags, &src_names, &header_paths, &artifact)?;
    }

    else {
        print!("Error: no valid arguments were passed.")
    }
    
    Ok(())
}
