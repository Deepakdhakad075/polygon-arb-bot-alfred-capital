use std::fs::OpenOptions;
use std::io::Write;
use chrono::Utc;

pub fn log_opportunity(
    dex_buy: &str,
    dex_sell: &str,
    buy_price: f64,
    sell_price: f64,
    profit: f64,
) -> anyhow::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("opportunities.csv")?;
    let ts = Utc::now().to_rfc3339();
    let line = format!("{},{},{:.6},{:.6},{:.6}\n", ts, format!("{}->{}", dex_buy, dex_sell), buy_price, sell_price, profit);
    file.write_all(line.as_bytes())?;
    Ok(())
}
