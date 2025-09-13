pub fn compute_profit(buy_price_usdc: f64, sell_price_usdc: f64, trade_size_weth: f64, gas_cost_usdc: f64) -> f64 {
    let gross = (sell_price_usdc - buy_price_usdc) * trade_size_weth;
    gross - gas_cost_usdc
}
