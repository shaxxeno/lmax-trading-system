use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub exchange: ExchangeSettings,
    pub trading:  TradingSettings,
    pub risk:     RiskSettings,
    pub logging:  LoggingSettings,
}

#[derive(Debug, Deserialize)]
pub struct ExchangeSettings {
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct TradingSettings {
    pub symbols:         Vec<String>,
    pub initial_balance: f64,
}

#[derive(Debug, Deserialize)]
pub struct RiskSettings {
    pub max_position_pct: f64,
    pub max_drawdown_pct: f64,
    pub max_open_trades:  usize,
}

#[derive(Debug, Deserialize)]
pub struct LoggingSettings {
    pub level:  String,
    pub format: String,
}

impl Settings {
    pub fn load() -> anyhow::Result<Self> {
        let settings = ::config::Config::builder()
            .add_source(::config::File::with_name("config"))
            .add_source(::config::Environment::with_prefix("TRADING"))
            .build()?
            .try_deserialize()?;
        Ok(settings)
    }
}
