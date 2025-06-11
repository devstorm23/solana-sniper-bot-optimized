use serde::{Deserialize, Serialize};
use std::fs;
use anyhow::Result;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub wallet: WalletConfig,
    pub trading: TradingConfig,
    pub dex: DexConfig,
    pub risk: RiskConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WalletConfig {
    pub private_key: String,
    pub public_key: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TradingConfig {
    pub max_buy_amount: f64,
    pub min_liquidity: f64,
    pub slippage_tolerance: f64,
    pub gas_limit: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DexConfig {
    pub raydium_enabled: bool,
    pub jupiter_enabled: bool,
    pub orca_enabled: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RiskConfig {
    pub stop_loss: f64,
    pub take_profit: f64,
    pub max_trades_per_hour: u32,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
    
    pub fn default() -> Self {
        Config {
            wallet: WalletConfig {
                private_key: "your-private-key-here".to_string(),
                public_key: "your-public-key-here".to_string(),
            },
            trading: TradingConfig {
                max_buy_amount: 1.0,
                min_liquidity: 10.0,
                slippage_tolerance: 5.0,
                gas_limit: 200000,
            },
            dex: DexConfig {
                raydium_enabled: true,
                jupiter_enabled: true,
                orca_enabled: false,
            },
            risk: RiskConfig {
                stop_loss: 20.0,
                take_profit: 50.0,
                max_trades_per_hour: 10,
            },
        }
    }
}
