use chrono::Utc;
use futures_util::StreamExt;
use serde::Deserialize;
use tokio_tungstenite::connect_async;

use super::Feed;
use crate::domain::events::market::MarketEvent;
use crate::infrastructure::disruptor::producer::Producer;

#[derive(Debug, Deserialize)]
struct BinanceTradeMessage {
    #[serde(rename = "p")]
    price: String,
    #[serde(rename = "s")]
    symbol: String,
}

pub struct BinanceFeed {
    symbol: String,
    ws_url: String,
}

impl BinanceFeed {
    pub fn new(symbol: String, ws_url: String) -> Self {
        BinanceFeed { symbol, ws_url }
    }
}

impl Feed for BinanceFeed {
    fn stream(&mut self, mut producer: Producer<MarketEvent>) {
        let rt = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");
        rt.block_on(async {
            let url = format!("{}/ws/{}@trade", self.ws_url, self.symbol.to_lowercase());
            let (ws_stream, _) = match connect_async(&url).await {
                Ok(s) => s,
                Err(e) => { tracing::error!("Binance connect failed: {}", e); return; }
            };
            tracing::info!("Connected to Binance stream: {}", self.symbol);
            let (_, mut read) = ws_stream.split();
            while let Some(msg) = read.next().await {
                let msg = match msg {
                    Ok(m) => m,
                    Err(e) => { tracing::error!("WS error: {}", e); break; }
                };
                let text = match msg.to_text() {
                    Ok(t) => t,
                    Err(_) => continue,
                };
                if let Ok(trade) = serde_json::from_str::<BinanceTradeMessage>(text) {
                    if let Ok(price) = trade.price.parse::<f64>() {
                        let ts = Utc::now().timestamp_nanos_opt().unwrap_or(0);
                        let event = MarketEvent::new(&trade.symbol, price, ts);
                        producer.publish(event);
                    }
                }
            }
        });
    }

    fn symbol(&self) -> &str {
        &self.symbol
    }
}
