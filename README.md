Polygon Arbitrage Opportunity Detector Bot
1. Overview

This is a Rust-based bot that detects potential arbitrage opportunities on the Polygon network. Arbitrage, in this context, is the process of buying a token on one DEX at a lower price and selling it on another DEX at a higher price to earn a profit.

The bot periodically fetches token prices from two decentralized exchanges (DEXes) on Polygon, calculates potential profits, and logs profitable opportunities.

2. Features

Fetch token prices from multiple DEXes (e.g., QuickSwap, SushiSwap, Uniswap V2 routers)

Detect arbitrage opportunities based on a configurable profit threshold

Simulate profit calculation considering gas costs and trade size

Logs opportunities to CSV for analysis

Configurable via .env file

Optional: Can be extended to save opportunities in a SQLite database.

3. Technology Stack

Blockchain: Polygon Network

Programming Language: Rust

DEX Interaction: ethers library in Rust (Interacts with Uniswap V2 Router ABIs)

Data Logging: CSV file (opportunities.csv)

Optional DB: SQLite (rusqlite crate)

Async Runtime: tokio

4. Folder Structure
polygon-arb-bot/
├── Cargo.toml
├── .env                  # Configuration file (RPC URL, DEX routers, tokens, thresholds)
├── abi/
│   └── IUniswapV2Router02.json
├── src/
│   ├── main.rs           # Entry point
│   ├── config.rs         # Loads config from .env
│   ├── dex.rs            # Fetches prices from DEXes
│   ├── arb.rs            # Detects arbitrage opportunities
│   └── logger.rs         # Logs opportunities to CSV
└── opportunities.csv     # Logged arbitrage opportunities

5. Configuration

Create a .env file based on .env.example:
# RPC_URL
RPC_URL=https://polygon-mainnet.g.alchemy.com/v2/YOUR_API_KEY
# DEX router addresses (QuickSwap & SushiSwap)
DEX1_ROUTER=0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff
DEX2_ROUTER=0x1b02da8cb0d097eb8d57a175b88c7d8b47997506
# Tokens
WETH=0x7ceb23fd6bc0add59e62ac25578270cff1b9f619
USDC=0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174
# other Setting
TRADE_SIZE=1.0
GAS_COST=2.0
PROFIT_THRESHOLD=5.0
POLL_INTERVAL_SEC=30


TRADE_SIZE: Amount of token to trade per arbitrage simulation

GAS_COST: Estimated gas cost for a trade

PROFIT_THRESHOLD: Minimum profit (in USDC) to log an opportunity

POLL_INTERVAL_SEC: How often the bot checks prices

6. How It Works (Flow)

Load Configuration: Reads .env for RPC URL, DEX routers, token addresses, trade size, gas cost, and thresholds.

Fetch Prices:

dex.rs calls each DEX router contract for the token pair (WETH/USDC)

Returns current prices for each DEX

Detect Arbitrage:

arb.rs compares prices from DEX1 and DEX2

Calculates potential profit (accounting for gas)

Checks if profit ≥ PROFIT_THRESHOLD

Log Opportunities:

Profitable opportunities are written to opportunities.csv

Each entry contains:

Timestamp

DEX names

Buy price, Sell price

Simulated profit in USDC

Loop: Repeats every POLL_INTERVAL_SEC seconds

7. Example Output
DEX1 price (USDC): 4610.63, DEX2 price (USDC): 4563.59
Arbitrage opportunity BUY DEX2 SELL DEX1 profit 45.03 USDC
DEX1 price (USDC): 4612.10, DEX2 price (USDC): 4565.00
Arbitrage opportunity BUY DEX2 SELL DEX1 profit 47.10 USDC


CSV output (opportunities.csv):

Timestamp	Buy DEX	Sell DEX	Buy Price	Sell Price	Profit (USDC)
2025-09-13 23:00:00	DEX2	DEX1	4563.59	4610.63	45.03
2025-09-13 23:00:30	DEX2	DEX1	4565.00	4612.10	47.10
8. Running the Bot
cargo run --release


The bot will fetch prices, detect arbitrage, and log opportunities automatically.

Ensure .env is configured with your RPC URL and router addresses.

9. Optional: SQLite Integration

Add rusqlite dependency:

rusqlite = "0.29"


Create a table arbitrage_opportunities to save each opportunity for easier querying later.

Modify logger.rs to insert into SQLite in addition to CSV.

10. Future Improvements

Execute actual arbitrage trades (requires private key management and transaction signing)

Support multiple token pairs (WETH/USDC, WBTC/USDC, etc.)

Web dashboard to visualize arbitrage opportunities

Push notifications for profitable trades

11. Notes

Ensure .gitignore includes:

target/
.env



CSV logs (opportunities.csv) can be shared for review or testing.