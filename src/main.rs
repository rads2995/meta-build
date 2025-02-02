pub(crate) mod helpers;

fn main() -> std::io::Result<()> {

    use helpers::{
        collect_files,
        create_build_file,
    };
        
    let args: Vec<String> = std::env::args().collect();
    let mut application: bool = true;
    if args.len() == 1 {
        println!(
            "Info: no input argument passed. Building binary executable."
        );
    }

    else if args.len() == 2 {
        if args[1] == "--static-lib" {
            println!(
                "Info: {} passed as input argument. Building static library.", args[1]
            );

        } 

        else if args[1] == "--shared-lib" {
            println!(
                "Info: {} passed as input argument. Building shared library.", args[1]
            );
        }
        
        else {
            panic!("Error: input argument not valid. For building a library, please use --static-lib or --shared-lib.")
        }

        application = false;
    }

    else {
        panic!("Error: incorrect number of input arguments.")
    }
    
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
        "-Wl,-z,nodlopen",
        "-Wl,-z,noexecstack",
        "-Wl,-z,relro",
        "-Wl,-z,now",
        "-fno-delete-null-pointer-checks",
        "-fno-strict-overflow",
        "-fno-strict-aliasing",
        "-ftrivial-auto-var-init=zero",
        "-fexceptions",
        "-fhardened",
        "-Wl,--as-needed",
        "-Wl,--no-copy-dt-needed-entries",
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
    
    let mut build: String = String::new();
    
    if application == true {
        cflags.push("-fPIE -pie");
        build = format!("build main: ld {}", src_names.join(" ").replace("c", "o"));
    }

    else {
        if args[1] == "--static-lib" {
            build = format!("build lib.o: ld {}", src_names.join(" ").replace("c", "o"));
        }

        else if args[1] == "--shared-lib" {
            cflags.push("-fPIC -shared");
            build = format!("build lib.so: ld {}", src_names.join(" ").replace("c", "o"));
        }
    }
    
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
