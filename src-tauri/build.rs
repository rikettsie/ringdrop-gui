fn main() {
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!(
        "cargo:rustc-env=RINGDROP_VERSION={}",
        ringdrop_version().expect("ringdrop entry not found in Cargo.lock")
    );
    tauri_build::build()
}

fn ringdrop_version() -> Option<String> {
    let lock = std::fs::read_to_string("Cargo.lock").ok()?;
    let mut in_ringdrop = false;
    for line in lock.lines() {
        let line = line.trim();
        if line == r#"name = "ringdrop""# {
            in_ringdrop = true;
        } else if in_ringdrop && line.starts_with("version = ") {
            return Some(
                line.trim_start_matches("version = ")
                    .trim_matches('"')
                    .to_string(),
            );
        } else if in_ringdrop && line.starts_with("[[") {
            break;
        }
    }
    None
}
