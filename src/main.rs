use brrr::sim::stats;
use tracing::info;

fn main() {
    tracing_subscriber::fmt::init();
    info!("init: model");
    // TODO: aggregate samples
    for _ in 0..10 {
        info!("{}", stats(100_000_000));
    }
}
