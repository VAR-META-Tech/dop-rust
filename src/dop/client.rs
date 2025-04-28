use std::process::{Child, Command};
use reqwest::Client;

pub struct DopClient {
    pub(crate) child: Option<Child>,
    pub(crate) client: Client,
    pub(crate) port: u16,
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
        }
    }

    pub(crate) fn base_url(&self) -> String {
        format!("http://localhost:{}", self.port)
    }

    pub fn start(&mut self) {
        println!("Starting Node.js DOP Engine...");
        let child = Command::new("node")
            .arg("ts-lib/dist/index.js")
            .spawn()
            .expect("Failed to start Node Engine");
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
