use reqwest::Client;
use serde::Deserialize;
use std::{
    path::PathBuf,
    process::{Child, Command},
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, Deserialize)]
pub struct Chain {
    #[serde(rename = "type")]
    pub chain_type: u8,
    pub id: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MerkletreeScanUpdateEvent {
    #[serde(rename = "scanStatus")]
    pub scan_status: String,
    pub chain: Chain,
    pub progress: f64,
}

pub enum ScanType {
    UTXOMerkletree,
    TXIDMerkletree,
}

pub struct DopClient {
    pub(crate) child: Option<Child>,
    pub(crate) client: Client,
    pub(crate) port: u16,

    pub(crate) utxo_scan_callback:
        Arc<Mutex<Option<Box<dyn Fn(MerkletreeScanUpdateEvent) + Send + 'static>>>>,
    pub(crate) txid_scan_callback:
        Arc<Mutex<Option<Box<dyn Fn(MerkletreeScanUpdateEvent) + Send + 'static>>>>,
}

impl DopClient {
    pub fn new() -> Self {
        Self::with_port(3000)
    }

    pub fn with_port(port: u16) -> Self {
        Self {
            child: None,
            client: Client::new(),
            port,
            utxo_scan_callback: Arc::new(Mutex::new(None)),
            txid_scan_callback: Arc::new(Mutex::new(None)),
        }
    }

    pub(crate) fn base_url(&self) -> String {
        format!("http://localhost:{}", self.port)
    }

    pub fn start(&mut self) {
        let crate_dir = env!("CARGO_MANIFEST_DIR");
        let ts_lib_dir = PathBuf::from(crate_dir).join("ts-lib");
        let node_modules_dir = ts_lib_dir.join("node_modules");
        let dist_path = ts_lib_dir.join("dist/index.js");

        // If node_modules doesn't exist, run npm install
        if !node_modules_dir.exists() {
            println!("📦 node_modules not found — running `npm install`...");
            let status = Command::new("npm")
                .arg("install")
                .current_dir(&ts_lib_dir)
                .status()
                .expect("❌ Failed to run `npm install`");
            assert!(status.success(), "❌ `npm install` failed");
        }

        // If dist/index.js doesn't exist, run npm run build
        if !dist_path.exists() {
            println!("🔨 dist/index.js not found — running `npm run build`...");
            let status = Command::new("npm")
                .args(&["run", "build"])
                .current_dir(&ts_lib_dir)
                .status()
                .expect("❌ Failed to run `npm run build`");
            assert!(status.success(), "❌ `npm run build` failed");
        }

        // Confirm dist/index.js now exists
        assert!(
            dist_path.exists(),
            "❌ Build failed: dist/index.js still missing"
        );

        // Launch Node.js engine
        let child = Command::new("node")
            .arg(dist_path.to_str().unwrap())
            .spawn()
            .expect("❌ Failed to start Node Engine");

        println!("🚀 Node.js engine started from {:?}", dist_path);
        self.child = Some(child);
    }

    pub fn stop(&mut self) {
        if let Some(child) = self.child.as_mut() {
            if let Ok(Some(_)) = child.try_wait() {
                println!("Node.js process already exited.");
            } else {
                child.kill().expect("Failed to kill Node Engine");
                println!("Node.js process killed.");
            }
        }
    }
}

impl Drop for DopClient {
    fn drop(&mut self) {
        self.stop();
    }
}
