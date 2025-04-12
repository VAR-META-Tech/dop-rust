use reqwest;
use serde_json::Value;
use std::process::{Child, Command};
use std::time::Duration;
use tokio::time::sleep;

pub struct DopEngine {
    child: Option<Child>,
}

impl DopEngine {
    pub fn new() -> Self {
        Self { child: None }
    }

    pub fn start(&mut self) {
        let child = Command::new("node")
            .arg("ts-lib/dist/index.js")
            .spawn()
            .expect("Failed to start Node Engine");

        self.child = Some(child);
    }

    pub async fn wait_for_api_ready(&self) {
        for _ in 0..10 {
            if reqwest::get("http://localhost:3000/health").await.is_ok() {
                println!("Node.js API is ready");
                return;
            }
            println!("Waiting for Node.js API...");
            sleep(Duration::from_secs(1)).await;
        }
        panic!("Node.js API failed to start");
    }

    pub async fn init_engine(&self) -> Result<(), Box<dyn std::error::Error>> {
        reqwest::get("http://localhost:3000/init").await?;
        Ok(())
    }

    pub async fn engine_status(&self) -> Result<String, Box<dyn std::error::Error>> {
        let res = reqwest::get("http://localhost:3000/status")
            .await?
            .text()
            .await?;
        Ok(res)
    }

    pub async fn get_engine_info(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let res = reqwest::get("http://localhost:3000/engine")
            .await?
            .json::<serde_json::Value>()
            .await?;
        Ok(res)
    }

    pub async fn close_engine(&self) -> Result<(), Box<dyn std::error::Error>> {
        reqwest::get("http://localhost:3000/close").await?;
        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some(child) = &mut self.child {
            child.kill().expect("Failed to kill Node Engine");
        }
    }
}
