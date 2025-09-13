use ethers::prelude::*;
use std::sync::Arc;
use anyhow::Context;
use ethers::abi::Abi;

pub async fn get_amounts_out(
    provider: Arc<Provider<Http>>,
    router_addr: Address,
    amount_in: U256,
    path: Vec<Address>,
) -> anyhow::Result<Vec<U256>> {
    // ABI load from bytes
    let abi: Abi = serde_json::from_slice(include_bytes!("../abi/IUniswapV2Router02.json"))
        .context("parsing router ABI")?;

    let contract = Contract::new(router_addr, abi, provider);

    let amounts: Vec<U256> = contract
        .method::<(U256, Vec<Address>), Vec<U256>>("getAmountsOut", (amount_in, path))?
        .call()
        .await
        .context("contract call getAmountsOut")?;

    Ok(amounts)
}
