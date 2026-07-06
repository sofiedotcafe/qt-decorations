use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{env, fs};

fn main() {
    // 1. Determine Qt target modules based on features
    let is_qt6 = env::var("CARGO_FEATURE_QT6").is_ok();
    let qt_prefix = if is_qt6 { "Qt6" } else { "Qt5" };
    let modules = vec!["Core", "Gui", "WaylandClient"];

    // 2. Discover system library dependencies via pkg-config
    let base_libs: Vec<_> = modules
        .iter()
        .map(|mod_name| format!("{qt_prefix}{mod_name}"))
        .chain(std::iter::once("dbus-1".to_string()))
        .map(|lib| pkg_config::probe_library(&lib).unwrap_or_else(|e| panic!("{e}")))
        .collect();

    // 3. Locate source and output layout paths
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let src_dir = manifest_dir.join("src");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // 4. Run Qt Meta-Object Compiler (moc) execution pipelines
    let moc_path = env::var("QT_MOC").unwrap_or_else(|_| "moc".to_string());
    let moc_status = Command::new(&moc_path)
        .args([src_dir.join("plugin.cpp"), PathBuf::from("-o"), out_dir.join("plugin.moc")])
        .status()
        .expect("Failed to run Qt moc engine binary");

    if !moc_status.success() {
        panic!("Qt moc compilation task failed");
    }

    // 5. Gather compiler header parameters cleanly without flat loops
    let mut flags = vec!["-std=c++17".to_string(), format!("-I{}", src_dir.display()), format!("-I{}", out_dir.display())];
    let mut include_dirs = Vec::new();

    // --- FIX: Query the wrapped C++ compiler for its hidden standard includes ---
    let cxx = env::var("CXX").unwrap_or_else(|_| "c++".to_string());
    if let Ok(output) = Command::new(cxx)
        .args(["-v", "-E", "-x", "c++", "-"])
        .stdin(Stdio::null())
        .stderr(Stdio::piped())
        .stdout(Stdio::null())
        .output()
    {
        let stderr_str = String::from_utf8_lossy(&output.stderr);
        let mut in_include_section = false;
        for line in stderr_str.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("#include <...>") {
                in_include_section = true;
                continue;
            }
            if trimmed.starts_with("End of search list.") {
                break;
            }
            if in_include_section && !trimmed.is_empty() {
                flags.push(format!("-isystem{trimmed}"));
            }
        }
    }

    // Check for explicit standard library paths if exported out of Nix
    if let Ok(nix_cxx_flags) = env::var("NIX_CXXSTDLIB_COMPILE_FLAGS") {
        flags.extend(nix_cxx_flags.split_whitespace().map(String::from));
    }

    for lib in &base_libs {
        for path in &lib.include_paths {
            flags.push(format!("-I{}", path.display()));
            include_dirs.push(path.clone());

            if let Some(parent) = path.parent() {
                flags.push(format!("-I{}", parent.display()));
                include_dirs.push(parent.to_path_buf());
            }

            if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()).filter(|n| n.starts_with("Qt")) {
                if let Ok(entries) = fs::read_dir(path) {
                    for entry in entries.flatten().filter(|e| e.path().is_dir()) {
                        let sub_path = entry.path();
                        flags.push(format!("-I{}", sub_path.display()));
                        include_dirs.push(sub_path.clone());

                        let nested_private = sub_path.join(dir_name);
                        if nested_private.exists() {
                            flags.push(format!("-I{}", nested_private.display()));
                            include_dirs.push(nested_private);
                        }
                    }
                }
            }
        }
    }

    // 6. Bind compilation flags straight down into autocxx context engines
    let bindgen_args = format!("{} {}", env::var("BINDGEN_EXTRA_CLANG_ARGS").unwrap_or_default(), flags.join(" "));
    env::set_var("BINDGEN_EXTRA_CLANG_ARGS", bindgen_args.trim());

    let flag_refs: Vec<&str> = flags.iter().map(|s| s.as_str()).collect();
    let mut cc_builder = autocxx_build::Builder::new(src_dir.join("lib.rs"), &[&manifest_dir, &src_dir])
        .extra_clang_args(&flag_refs)
        .build()
        .unwrap();

    cc_builder
        .cpp(true)
        .std("c++17")
        .file(src_dir.join("plugin.cpp"))
        .include(&src_dir)
        .include(&out_dir);

    for path in include_dirs {
        cc_builder.include(path);
    }
    cc_builder.compile("qt_decorations_cpp_glue");

    // 7. Track filesystem changes to minimize unnecessary asset re-evaluation cycles
    for file in ["lib.rs", "bridge.h", "plugin.cpp"] {
        println!("cargo:rerun-if-changed={}", src_dir.join(file).display());
    }
}