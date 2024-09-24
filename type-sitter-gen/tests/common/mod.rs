use std::fs::{create_dir, write};
use std::path::{Path, PathBuf};

pub struct Common {
    pub input_dir: PathBuf,
    pub expected_dir: PathBuf
}

pub fn setup_common(lang: &str) -> Common {
    let input_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../vendor/tree-sitter-{}", lang));
    let expected_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../type-sitter-lib/tests/{}", lang));

    // Recreate `expected_path` if it doesn't exist.
    if !expected_dir.exists() {
        create_dir(&expected_dir).expect("Failed to create expected directory");
        write(expected_dir.join("mod.rs"), "pub mod nodes;\npub mod queries;")
            .expect("Failed to create expected mod.rs file");
    }

    Common { input_dir, expected_dir }
}