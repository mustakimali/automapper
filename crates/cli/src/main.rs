use std::path::PathBuf;
use walkdir::WalkDir;

fn main() {
    let starting_path = std::env::args().nth(1).unwrap_or(".".to_string());

    let cargo_toml_path = nearest_cargo_toml_file(PathBuf::from(starting_path));
    println!("Generating rustdoc json for: {:?}", cargo_toml_path);

    let json_path = rustdoc_json::Builder::default()
        .toolchain("nightly")
        .manifest_path(&cargo_toml_path)
        .document_private_items(true)
        .all_features(true)
        .build()
        .expect("failed to build rustdoc JSON!");
    let dest_json = cargo_toml_path.parent().unwrap().join("rustdoc.json");

    println!("Copying {:?} to {:?}", json_path, dest_json);

    std::fs::copy(&json_path, dest_json).expect("failed to copy rustdoc JSON!");
}

/// Returns the root path of the crate that calls this function.
/// This is a cursed method
fn nearest_cargo_toml_file(starting_path: PathBuf) -> PathBuf {
    for entry in WalkDir::new(&starting_path)
        .into_iter()
        .filter_entry(|e| !e.file_name().eq_ignore_ascii_case("target"))
    {
        let Ok(entry) = entry else { continue };
        if !entry.file_type().is_file() {
            continue;
        }
        let Some(file_name) = entry.path().file_name() else {
            continue;
        };
        if !file_name.eq_ignore_ascii_case("Cargo.toml") {
            continue;
        }
        return entry.path().to_path_buf();
    }
    panic!("failed to find nearest Cargo.toml file!");
}