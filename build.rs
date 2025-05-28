use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    let ts_lib_dir = PathBuf::from("ts-lib");
    let dist_file = ts_lib_dir.join("dist").join("index.js");

    println!("cargo:rerun-if-changed=ts-lib/package.json");
    println!("cargo:rerun-if-changed=ts-lib/tsconfig.json");
    println!("cargo:rerun-if-changed=ts-lib/src/");

    if dist_file.exists() {
        println!("✅ Found existing build: {:?}", dist_file);
        return;
    }

    // If building from a git checkout, skip automatic build (read-only)
    if env::var("CARGO").is_ok()
        && !fs::metadata(&ts_lib_dir)
            .map(|m| m.permissions().readonly())
            .unwrap_or(false)
    {
        println!("📦 Running `npm install` in {:?}", ts_lib_dir);
        let install_status = Command::new("npm")
            .arg("install")
            .current_dir(&ts_lib_dir)
            .status()
            .expect("❌ Failed to run `npm install`");
        assert!(install_status.success(), "`npm install` failed");

        println!("🔨 Running `npm run build` in {:?}", ts_lib_dir);
        let build_status = Command::new("npm")
            .args(&["run", "build"])
            .current_dir(&ts_lib_dir)
            .status()
            .expect("❌ Failed to run `npm run build`");
        assert!(build_status.success(), "`npm run build` failed");

        if !dist_file.exists() {
            panic!("❌ Build did not produce {:?}", dist_file);
        }

        println!("✅ Node backend built successfully: {:?}", dist_file);
    } else {
        println!("⚠️ Skipping ts-lib build: read-only environment (likely Git dependency).");
        println!(
            "💡 To use this crate, clone it locally and use `[dependencies.dop] path = \"../dop-sdk\"`."
        );
    }
}
