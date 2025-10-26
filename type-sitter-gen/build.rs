// See https://stackoverflow.com/questions/48967583/how-to-get-executables-full-target-triple-as-a-compile-time-constant-without-us
// and https://github.com/Shnatsel/current_platform/blob/master/src/build.rs
fn main() {
    forward_env("HOST");
    forward_env("TARGET");
    forward_env("OPT_LEVEL");
    forward_env("DEBUG");

    // By default Cargo only runs the build script when a file changes.
    // This makes it re-run on target change
    println!("cargo:rerun-if-changed-env=TARGET")
}

/// Pass the environment variable to env! in the crate
fn forward_env(name: &str) {
    println!(
        "cargo:rustc-env=TYPE_SITTER_{}={}",
        name,
        std::env::var(name).unwrap()
    );
}
