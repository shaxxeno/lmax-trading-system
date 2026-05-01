# LMAX Trading System

A low-latency algorithmic trading engine written in Rust, built around the **LMAX Disruptor pattern**. This project is a systems programming learning exercise focused on understanding ring buffers, cache-line discipline, lock-free coordination, and real-time scheduling on Linux.

---

## Architecture

```
Feed thread
    │  TCP socket (MT5 / mock)
    ▼
Inbound Ring Buffer (4096 slots)
    │  lock-free, single producer / single consumer
    ▼
BLP thread  (Business Logic Processor)
    │  strategy → risk → signal → translate
    ▼
Outbound Ring Buffer (4096 slots)
    │  lock-free, single producer / 2 consumers
    ├──▶ OrderService thread   (paper execution)
    └──▶ JournalService thread (append-only CSV)
```

### Disruptor implementation

The ring buffer is implemented from scratch without external dependencies to understand the mechanics:

- **Slot sequence handshake** — each slot carries its own `AtomicU64` sequence. Producer stores `seq` with `Release` after writing; consumer spins with `Acquire` load until `slot.sequence == expected`. This pairs the memory barrier with the data, not a separate pointer.
- **Cache-line padding** — producer cursor and each consumer cursor live in `#[repr(C, align(64))]` structs to prevent false sharing across cores.
- **No heap allocation on hot path** — `MarketEvent` and `TradingEvent` are `Copy` types stored directly in ring buffer slots. No `Arc`, no `Box`, no allocation between tick receipt and signal publication.
- **Sequence math in `i64`** — wrap-point comparison uses signed 64-bit arithmetic to correctly handle the modular sequence space (mirrors the original LMAX Java implementation which uses signed `long`).

### Thread layout

| Thread | Core | Role |
|--------|------|------|
| Feed | 2 | Reads raw TCP ticks, publishes `MarketEvent` |
| BLP | 4 | Runs strategy + risk, publishes `TradingEvent` |
| OrderService | 6 | Consumes signals, executes paper orders |
| JournalService | 8 | Appends all non-Hold events to `journal.csv` |

All four threads run `SCHED_FIFO` priority 2 and are pinned to isolated physical cores with `core_affinity`.

### Strategy

**Asia Range Breakout** — a session-based mechanical strategy:
1. Records the high/low range formed during the Asia session (00:00–08:00 UTC)
2. On London open (08:00 UTC), monitors for a breakout above the Asia high (Buy) or below the Asia low (Sell)
3. Produces a single `Signal` per tick — `Buy`, `Sell`, or `Hold`

---

## Latency measurements

> **These are pure code latencies** — the time from consuming a `MarketEvent` off the inbound ring buffer to publishing a `TradingEvent` onto the outbound ring buffer. This includes strategy evaluation and risk sizing only. **No network, no NIC, no kernel bypass, no exchange RTT.**

| Metric | Value |
|--------|-------|
| min | 40 ns |
| avg | 119 ns |
| p99 | 131 ns |
| max | 5 751 ns |

Measurements taken at 100 000 ticks (1 ms/tick mock feed, 10 000-tick rolling window).

The max spike (5.7 µs) is a single IRQ routed to an isolated core by the kernel — not a hot-path regression.

### Measurement environment

**Hardware**
- CPU: AMD Ryzen AI 9 HX 370 w/ Radeon 890M
- Cores: 12 physical / 24 logical (SMT on)
- Boost: 5157 MHz
- Cache: L1d 48 KB per core · L2 1 MB per core · L3 24 MB (2 × 12 MB instances)

**OS / kernel**
- Fedora Linux, kernel `6.19.14-200.fc43.x86_64`

**Kernel tuning applied**
- `isolcpus=2,4,6,8` — CPUs removed from the general scheduler
- `nohz_full=2,4,6,8` — kernel timer tick disabled on isolated CPUs
- `rcu_nocbs=2,4,6,8` — RCU callbacks offloaded off isolated CPUs
- CPU governor: `performance` (no frequency scaling)
- CPU idle states: disabled (`cpupower idle-set -D 0`)
- Transparent huge pages: `never`
- Threads: `SCHED_FIFO` priority 2, pinned with `core_affinity`

---

## Building

```bash
cargo build --release
```

## Running

Terminal 1 — start the engine (root required for `SCHED_FIFO`):
```bash
sudo ./target/release/trading_engine_v2
```

Terminal 2 — start the mock MT5 tick feed:
```bash
./target/release/mock_feed
```

Latency stats print to stdout every 10 000 ticks (~10 seconds at 1 ms/tick).  
Trade journal is written to `journal.csv` in the working directory.

---

## Project layout

```
src/
├── core/
│   ├── config.rs          config.toml loader (Pydantic-style)
│   ├── logging.rs         tracing-subscriber setup
│   └── metrics.rs         rolling latency stats (min/avg/p99/max)
├── domain/
│   ├── entities/          trade, order, bar
│   ├── events/            MarketEvent, TradingEvent (Copy enums)
│   └── services/
│       ├── strategies/    Strategy trait + BreakoutStrategy
│       ├── risk/          RiskManager trait + BasicRiskManager
│       ├── position/      PositionTracker trait + BasicPositionTracker
│       ├── order_service.rs
│       └── journal_service.rs
├── engine/mod.rs          BusinessProcessor (BLP)
├── infrastructure/
│   ├── disruptor/         ring_buffer, producer, consumer
│   ├── feed/              Feed trait + Mt5Feed (TCP listener)
│   └── execution/         Executor trait + PaperExecutor
├── main.rs
└── bin/
    └── mock_feed.rs       synthetic tick sender for local testing
```
