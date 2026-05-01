mod core;
mod domain;
mod engine;
mod infrastructure;

extern crate libc;

use std::sync::Arc;
use std::thread;

use crate::{
    domain::{
        entities::trade::Signal,
        events::{market::MarketEvent, trading::TradingEvent},
        services::{
            journal_service::JournalService,
            order_service::OrderService,
            position::tracker::BasicPositionTracker,
            risk::basic::BasicRiskManager,
            strategies::{Strategy, breakout::BreakoutStrategy},
        },
    },
    engine::BusinessProcessor,
    infrastructure::{
        disruptor,
        execution::paper::PaperExecutor,
        feed::{Feed, mt5::Mt5Feed},
    },
};

struct NoopStrategy;

impl Strategy for NoopStrategy {
    fn name(&self) -> &str { "noop" }
    fn on_price(&mut self, _price: f64, _timestamp: i64) -> Signal { Signal::Hold }
}

fn pin(core_id: usize) {
    let cores = core_affinity::get_core_ids().unwrap();
    if let Some(core) = cores.get(core_id) {
        core_affinity::set_for_current(*core);
    }
}

fn set_realtime() {
    unsafe {
        let param = libc::sched_param { sched_priority: 2 };
        let ret = libc::sched_setscheduler(0, libc::SCHED_FIFO, &param);
        if ret != 0 {
            tracing::warn!("SCHED_FIFO failed: {}", std::io::Error::last_os_error());
        }
    }
}

fn main() -> anyhow::Result<()> {
    let settings = Arc::new(core::config::Settings::load()?);
    core::logging::init(&settings.logging);
    tracing::info!("Trading engine starting up");

    let (in_producer, mut in_consumers) = disruptor::build::<MarketEvent>(4096, 1);
    let (out_producer, mut out_consumers) = disruptor::build::<TradingEvent>(4096, 2);

    let feed = Mt5Feed::new(settings.exchange.port, settings.trading.symbols[0].clone());
    let strategy = BreakoutStrategy::new();

    let risk = Box::new(BasicRiskManager::new(
        settings.risk.max_position_pct,
        settings.risk.max_drawdown_pct,
        settings.trading.initial_balance,
    ));

    let tracker = Box::new(BasicPositionTracker::new());
    let executor = PaperExecutor::new(settings.trading.initial_balance);

    let blp = BusinessProcessor::new(
        in_consumers.remove(0),
        out_producer,
        strategy,
        risk,
        tracker,
        Arc::clone(&settings),
    );

    let order_svc = OrderService::new(out_consumers.remove(0), executor);
    let journal_svc = JournalService::new(out_consumers.remove(0), "journal.csv")?;

    let h1 = thread::spawn(move || { pin(2); set_realtime(); let mut f = feed; f.stream(in_producer); });
    let h2 = thread::spawn(move || { pin(4); set_realtime(); blp.run(); });
    let h3 = thread::spawn(move || { pin(6); set_realtime(); order_svc.run(); });
    let h4 = thread::spawn(move || { pin(8); set_realtime(); journal_svc.run(); });

    h1.join().unwrap();
    h2.join().unwrap();
    h3.join().unwrap();
    h4.join().unwrap();

    Ok(())
}
