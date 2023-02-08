use std::{env, path::PathBuf};

use cbindgen::{Config, Language};

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let target_include_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../target/include");

    let output_file = target_include_dir.join("pghom.h").display().to_string();

    let config = Config {
        namespace: Some(String::from("ffi")),
        language: Language::C,
        usize_is_size_t: true,
        ..Default::default()
    };

    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file(&output_file);
}
