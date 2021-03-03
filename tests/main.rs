use std::{
    env,
    io::{Error, ErrorKind, Result},
    path::{Path, PathBuf},
};

const TARGET: &str = "spirv-unknown-unknown";
const TARGET_DIR: &str = "target/compiletest";

fn main() {
    let manifest_dir = PathBuf::from("./");
    std::env::set_var("CARGO_MANIFEST_DIR", &manifest_dir);
    // Pull in rustc_codegen_spirv as a dynamic library in the same way
    // spirv-builder does.
    let codegen_backend_path = find_rustc_codegen_spirv();

    build_spirv_std(&manifest_dir, &codegen_backend_path);

    run_mode("ui", &codegen_backend_path);
}

/// Runs the given `mode` on the directory that matches that name, using the
/// backend provided by `codegen_backend_path`.
fn run_mode(mode: &'static str, codegen_backend_path: &Path) {
    let mut config = compiletest::Config::default();

    /// RUSTFLAGS passed to all test files.
    fn test_rustc_flags(codegen_backend_path: &Path, library_path: &[&Path]) -> String {
        [
            &*rust_flags(codegen_backend_path),
            &*library_path.iter().map(|p| format!("-L {}", p.display())).fold(String::new(), |a, b| b + " " + &a),
             "--edition 2018",
             "--extern spirv_std",
             "--crate-type dylib",
        ].join(" ")
    }

    let flags = test_rustc_flags(
        codegen_backend_path,
        &[
            &PathBuf::from(format!("./{}/spirv-std/spirv-unknown-unknown/debug/deps", TARGET_DIR)),
            &PathBuf::from(format!("./{}/debug", TARGET_DIR)),
        ]
    );

    config.target_rustcflags = Some(flags);
    config.mode = mode.parse().expect("Invalid mode");
    config.target = String::from(TARGET);
    config.src_base = PathBuf::from(format!("./tests/{}", mode));
    config.build_base = PathBuf::from(format!("./{}-results", TARGET_DIR));
    config.bless = std::env::args().any(|a| a == "--bless");
    config.clean_rmeta();

    compiletest::run_tests(&config);
}

/// Runs the processes needed to build `spirv-std`.
fn build_spirv_std(manifest_dir: &Path, codegen_backend_path: &Path) {
    let cargo_target_flag = format!("--target-dir={}", TARGET_DIR);

    std::process::Command::new("cargo")
        .args(&[
            "build",
            "-p=spirv-std-macros",
            &cargo_target_flag,
        ])
        .env("CARGO_MANIFEST_DIR", manifest_dir)
        .current_dir(manifest_dir)
        .status()
        .and_then(map_status_to_result)
        .unwrap();

    std::process::Command::new("cargo")
        .args(&[
            "build",
            "--manifest-path=crates/spirv-std/Cargo.toml",
            // Giving `spirv-std` it's own directory allows cargo to re-use the
            // `build-std=core` build output from previous runs.
            &(cargo_target_flag + "/spirv-std"),
            "-Zbuild-std=core",
            &*format!("--target={}", TARGET),
        ])
        .env("RUSTFLAGS", rust_flags(&codegen_backend_path))
        .env("CARGO_MANIFEST_DIR", manifest_dir)
        .current_dir(manifest_dir)
        .stderr(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .status()
        .and_then(map_status_to_result)
        .unwrap();
}

/// The RUSTFLAGS passed to all SPIR-V builds.
fn rust_flags(codegen_backend_path: &Path) -> String {
    [
        &*format!("-Zcodegen-backend={}", codegen_backend_path.display()),
        "-Coverflow-checks=off",
        "-Cdebug-assertions=off",
    ].join(" ")
}

/// Convience function to map process failure to results in Rust.
fn map_status_to_result(status: std::process::ExitStatus) -> Result<()> {
    match status.success() {
        true => Ok(()),
        false => Err(Error::new(
            ErrorKind::Other,
            format!(
                "process terminated with non-zero code: {}",
                status.code().unwrap_or(0)
            ),
        )),
    }
}

// https://github.com/rust-lang/cargo/blob/1857880b5124580c4aeb4e8bc5f1198f491d61b1/src/cargo/util/paths.rs#L29-L52
fn dylib_path_envvar() -> &'static str {
    if cfg!(windows) {
        "PATH"
    } else if cfg!(target_os = "macos") {
        "DYLD_FALLBACK_LIBRARY_PATH"
    } else {
        "LD_LIBRARY_PATH"
    }
}

fn dylib_path() -> Vec<PathBuf> {
    match env::var_os(dylib_path_envvar()) {
        Some(var) => env::split_paths(&var).collect(),
        None => Vec::new(),
    }
}

fn find_rustc_codegen_spirv() -> PathBuf {
    let filename = format!(
        "{}rustc_codegen_spirv{}",
        env::consts::DLL_PREFIX,
        env::consts::DLL_SUFFIX
    );
    for mut path in dylib_path() {
        path.push(&filename);
        if path.is_file() {
            return path;
        }
    }
    panic!("Could not find {} in library path", filename);
}

