use std::path::Path;
use std::process::Command;

fn main() {
    let ts_lib_dir = "ts-lib";
    let dist_file = "ts-lib/dist/index.js";

    println!("cargo:rerun-if-changed=ts-lib/package.json");
    println!("cargo:rerun-if-changed=ts-lib/tsconfig.json");
    println!("cargo:rerun-if-changed=ts-lib/src/");

    if Path::new(dist_file).exists() {
        println!("✅ Found existing build: {dist_file}");
        return;
    }

    println!("📦 Running `npm install` in {ts_lib_dir}...");
    let install_status = Command::new("npm")
        .arg("install")
        .current_dir(ts_lib_dir)
        .status()
        .expect("❌ Failed to execute `npm install`");
    assert!(install_status.success(), "`npm install` failed");

    println!("🔨 Running `npm run build` in {ts_lib_dir}...");
    let build_status = Command::new("npm")
        .args(&["run", "build"])
        .current_dir(ts_lib_dir)
        .status()
        .expect("❌ Failed to execute `npm run build`");
    assert!(build_status.success(), "`npm run build` failed");

    if !Path::new(dist_file).exists() {
        panic!("❌ Build did not produce {dist_file}");
    }

    println!("✅ Node backend built successfully: {dist_file}");
}
