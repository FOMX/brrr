use brrr::{PROBS, game::game_xoshiro128_plus_plus_rng};
use criterion::{Criterion, criterion_group, criterion_main};
use rand::rngs::{SmallRng, Xoshiro128PlusPlus};
use rand_distr::Bernoulli;

fn bench_game(c: &mut Criterion) {
    let mut group = c.benchmark_group("game");

    let mut r: Xoshiro128PlusPlus = rand::make_rng();
    // bernouli distributions. this may be inefficient
    let berns = PROBS.map(|p| Bernoulli::new(p).unwrap());
    group.bench_function("xoshiro128_rng", |b| {
        b.iter(|| game_xoshiro128_plus_plus_rng(&mut r, &berns))
    });

    group.finish();
}

criterion_group!(benches, bench_game);
criterion_main!(benches);
