// See https://stackoverflow.com/questions/48967583/how-to-get-executables-full-target-triple-as-a-compile-time-constant-without-us
fn main() {
    forward_env("HOST");
    forward_env("TARGET");
    forward_env("OPT_LEVEL");
    forward_env("DEBUG");
}

/// Pass the environment variable to env! in the crate
fn forward_env(name: &str) {
    println!("cargo:rustc-env={}={}", name, std::env::var(name).unwrap());
}