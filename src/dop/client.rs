use reqwest::Client;
use serde::Deserialize;
use std::{
    path::PathBuf, process::{Child, Command}, sync::{Arc, Mutex}
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
        // Find where this crate is located
        let crate_dir = env!("CARGO_MANIFEST_DIR");
        let script_path = PathBuf::from(crate_dir).join("ts-lib/dist/index.js");

        if !script_path.exists() {
            panic!("❌ Node script not found at {:?}", script_path);
        }

        let child = Command::new("node")
            .arg(script_path.to_str().unwrap())
            .spawn()
            .expect("❌ Failed to start Node Engine");

        println!("🚀 Node.js engine started from {:?}", script_path);
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
