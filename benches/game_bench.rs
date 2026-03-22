use brrr::{PROBS, game::game};
use criterion::{Criterion, criterion_group, criterion_main};
use rand::rngs::SmallRng;
use rand_distr::Bernoulli;

fn bench_game(c: &mut Criterion) {
    let mut group = c.benchmark_group("game");

    let mut r: SmallRng = rand::make_rng();
    // bernouli distributions. this may be inefficient
    let berns = PROBS.map(|p| Bernoulli::new(p).unwrap());
    group.bench_function("small_rng", |b| b.iter(|| game(&mut r, &berns)));

    group.finish();
}

criterion_group!(benches, bench_game);
criterion_main!(benches);
