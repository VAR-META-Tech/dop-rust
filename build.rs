use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    println!("🔥 build.rs is running!");
    let ts_lib_dir = PathBuf::from("ts-lib");
    let dist_file = ts_lib_dir.join("dist").join("index.js");

    println!("cargo:rerun-if-changed=ts-lib/package.json");
    println!("cargo:rerun-if-changed=ts-lib/tsconfig.json");
    println!("cargo:rerun-if-changed=ts-lib/src");

    // If dist already exists, skip
    if dist_file.exists() {
        println!("✅ Skipping build: found {}", dist_file.display());
        return;
    }

    // Check if directory is writable (avoid failure in read-only Git builds)
    let can_write = fs::metadata(&ts_lib_dir)
        .map(|m| !m.permissions().readonly())
        .unwrap_or(false);

    if !can_write {
        println!("⚠️ ts-lib is read-only (likely a git dependency). Skipping npm install/build.");
        return;
    }

    // Run `npm install`
    println!("📦 Running `npm install` in {}", ts_lib_dir.display());
    let install_status = Command::new("npm")
        .arg("install")
        .current_dir(&ts_lib_dir)
        .status()
        .expect("❌ Failed to run `npm install`");
    assert!(install_status.success(), "`npm install` failed");

    // Run `npm run build`
    println!("🔨 Running `npm run build` in {}", ts_lib_dir.display());
    let build_status = Command::new("npm")
        .args(&["run", "build"])
        .current_dir(&ts_lib_dir)
        .status()
        .expect("❌ Failed to run `npm run build`");
    assert!(build_status.success(), "`npm run build` failed");

    // Check build result
    if !dist_file.exists() {
        panic!("❌ Build did not produce {}", dist_file.display());
    }

    println!("✅ Node backend built: {}", dist_file.display());
}
