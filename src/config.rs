use dotenv::dotenv;
use std::env;
use anyhow::Context;

#[derive(Debug, Clone)]
pub struct Config {
    pub rpc_url: String,
    pub dex1_router: String,
    pub dex2_router: String,
    pub weth: String,
    pub usdc: String,
    pub trade_size: f64,
    pub gas_cost: f64,
    pub profit_threshold: f64,
    pub poll_interval_sec: u64,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv().ok();
        let get = |k: &str| env::var(k).context(format!("{} not set in .env", k));
        Ok(Self {
            rpc_url: get("RPC_URL")?,
            dex1_router: get("DEX1_ROUTER")?,
            dex2_router: get("DEX2_ROUTER")?,
            weth: get("WETH")?,
            usdc: get("USDC")?,
            trade_size: get("TRADE_SIZE")?.parse::<f64>().context("TRADE_SIZE parse")?,
            gas_cost: get("GAS_COST")?.parse::<f64>().context("GAS_COST parse")?,
            profit_threshold: get("PROFIT_THRESHOLD")?.parse::<f64>().context("PROFIT_THRESHOLD parse")?,
            poll_interval_sec: get("POLL_INTERVAL_SEC")?.parse::<u64>().context("POLL_INTERVAL_SEC parse")?,
        })
    }
}
