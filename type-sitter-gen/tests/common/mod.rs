use std::fs::{create_dir, write};
use std::path::{Path, PathBuf};

pub struct Common {
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
}

pub fn setup_common(lang: &str) -> Common {
    let input_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../vendor/tree-sitter-{}", lang));
    let output_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../type-sitter-lib/tests/{}", lang));

    // Recreate `expected_path` if it doesn't exist.
    if !output_dir.exists() {
        create_dir(&output_dir).expect("Failed to create expected directory");
        write(
            output_dir.join("mod.rs"),
            "pub mod nodes;\npub mod queries;",
        )
        .expect("Failed to create expected mod.rs file");
    }

    Common {
        input_dir,
        output_dir,
    }
}
