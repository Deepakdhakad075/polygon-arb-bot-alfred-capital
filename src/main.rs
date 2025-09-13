mod config;
mod dex;
mod arb;
mod logger;

use ethers::prelude::*;
use std::sync::Arc;
use anyhow::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = config::Config::from_env()?;
    println!("Loaded config: {:?}", cfg);

    let provider = Arc::new(Provider::<Http>::try_from(cfg.rpc_url.clone())?);

    let dex1_addr: Address = cfg.dex1_router.parse()?;
    let dex2_addr: Address = cfg.dex2_router.parse()?;
    let weth_addr: Address = cfg.weth.parse()?;
    let usdc_addr: Address = cfg.usdc.parse()?;

    let trade_size_wei = {
        let wei_f = cfg.trade_size * 1e18;
        U256::from(wei_f as u128)
    };

    loop {
        let path = vec![weth_addr, usdc_addr];
        let amounts1 = dex::get_amounts_out(provider.clone(), dex1_addr, trade_size_wei, path.clone()).await;
        let amounts2 = dex::get_amounts_out(provider.clone(), dex2_addr, trade_size_wei, path.clone()).await;

        match (amounts1, amounts2) {
            (Ok(a1), Ok(a2)) => {
                let out1 = a1.get(1).cloned().unwrap_or(U256::zero());
                let out2 = a2.get(1).cloned().unwrap_or(U256::zero());

                let p1 = u256_to_f64_usdc(out1)?;
                let p2 = u256_to_f64_usdc(out2)?;

                println!("DEX1 price (USDC): {:.6}, DEX2 price (USDC): {:.6}", p1, p2);

                let profit_1_to_2 = arb::compute_profit(p1, p2, cfg.trade_size, cfg.gas_cost);
                if profit_1_to_2 > cfg.profit_threshold {
                    println!("Arbitrage opportunity BUY DEX1 SELL DEX2 profit {:.6} USDC", profit_1_to_2);
                    logger::log_opportunity("DEX1", "DEX2", p1, p2, profit_1_to_2)?;
                }

                let profit_2_to_1 = arb::compute_profit(p2, p1, cfg.trade_size, cfg.gas_cost);
                if profit_2_to_1 > cfg.profit_threshold {
                    println!("Arbitrage opportunity BUY DEX2 SELL DEX1 profit {:.6} USDC", profit_2_to_1);
                    logger::log_opportunity("DEX2", "DEX1", p2, p1, profit_2_to_1)?;
                }
            }
            (Err(e), _) | (_, Err(e)) => eprintln!("Error fetching prices: {:?}", e),
        }

        tokio::time::sleep(std::time::Duration::from_secs(cfg.poll_interval_sec)).await;
    }
}

fn u256_to_f64_usdc(v: U256) -> anyhow::Result<f64> {
    let x: u128 = v.as_u128();
    Ok(x as f64 / 1e6f64)
}
