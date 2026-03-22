
use brrr::sim::stats_xoshiro128_plus_plus_rng;
use tracing::info;

fn main() {
    tracing_subscriber::fmt::init();
    info!("init: model");
    // TODO: aggregate samples
    for _ in 0..10 {
        info!("{}", stats_xoshiro128_plus_plus_rng(100_000_000));
    }
}
