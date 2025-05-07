use crate::dop::DopClient;
use anyhow::Result;

use super::MerkletreeScanUpdateEvent;

impl DopClient {
    pub fn set_utxo_scan_callback<F>(&self, callback: F)
    where
        F: Fn(MerkletreeScanUpdateEvent) + Send + 'static,
    {
        let mut cb = self.utxo_scan_callback.lock().unwrap();
        *cb = Some(Box::new(callback));

        // Notify TS server to set up its UTXO scan callback
        let client = self.client.clone();
        let url = format!("{}/setup-utxo-scan-callback", self.base_url());
        tokio::spawn(async move {
            if let Err(e) = client.post(&url).send().await {
                eprintln!("Failed to set up UTXO scan callback on TS server: {}", e);
            }
        });
    }

    pub fn set_txid_scan_callback<F>(&self, callback: F)
    where
        F: Fn(MerkletreeScanUpdateEvent) + Send + 'static,
    {
        let mut cb = self.txid_scan_callback.lock().unwrap();
        *cb = Some(Box::new(callback));

        // Notify TS server to set up its TXID scan callback
        let client = self.client.clone();
        let url = format!("{}/setup-txid-scan-callback", self.base_url());
        tokio::spawn(async move {
            if let Err(e) = client.post(&url).send().await {
                eprintln!("Failed to set up TXID scan callback on TS server: {}", e);
            }
        });
    }

    pub async fn start_scan_listeners(&self) -> Result<(), warp::Error> {
        use warp::Filter;

        let utxo_cb = self.utxo_scan_callback.clone();
        let txid_cb = self.txid_scan_callback.clone();

        let utxo_route = warp::post()
            .and(warp::path("utxo-scan-update"))
            .and(warp::body::json())
            .map(move |update: MerkletreeScanUpdateEvent| {
                if let Some(cb) = utxo_cb.lock().unwrap().as_ref() {
                    cb(update);
                }
                warp::reply()
            });

        let txid_route = warp::post()
            .and(warp::path("txid-scan-update"))
            .and(warp::body::json())
            .map(move |update: MerkletreeScanUpdateEvent| {
                if let Some(cb) = txid_cb.lock().unwrap().as_ref() {
                    cb(update);
                }
                warp::reply()
            });

        let routes = utxo_route.or(txid_route);

        tokio::spawn(async move {
            warp::serve(routes).run(([127, 0, 0, 1], 4000)).await;
        });

        Ok(())
    }
}
