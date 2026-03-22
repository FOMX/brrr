use rand::rngs::Xoshiro128PlusPlus;
use rand_distr::{Bernoulli, Distribution};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{CAPS, PROBS, game::{game_flattened, game_restructured, game_xoshiro128_plus_plus_rng}, stats::Stats};


/// each game consists of 99 rounds
/// each round has a cap
/// the distribution of the total amount of turns is of interest.
pub fn games(n: usize) -> f64 {
    // rng
    let mut r = rand::rng();

    // bernouli distributions. this may be inefficient
    let berns = PROBS.map(|p| Bernoulli::new(p).unwrap());

    let total: f64 = (0..n)
        .map(|_| {
            // track the turns taken to complete the game
            let mut turns = 0;

            // create a cycle to avoid modulo calcs
            let mut caps = CAPS.iter().cycle();

            // for 99 games
            for _ in 0..99 {
                let cap = *caps.next().unwrap();
                let turn = (0..=cap)
                    .find(|&attempt| berns[attempt].sample(&mut r))
                    .map(|attempt| attempt + 1)
                    .unwrap_or(cap + 1); // shouldn't happen given berns[cap] = 1.0
                turns += turn;
            }
            turns as f64
        })
        .sum();

    total / n as f64
}

/// each game consists of 99 rounds
/// each round has a cap
/// the distribution of the total amount of turns is of interest.
pub fn games_restructured(n: usize) -> f64 {
    // rng
    let mut r = rand::rng();

    // bernouli distributions. this may be inefficient
    let berns = PROBS.map(|p| Bernoulli::new(p).unwrap());

    let total: f64 = (0..n)
        .map(|_| game_restructured(&mut r, &berns) as f64)
        .sum();

    total / n as f64
}

/// each game consists of 99 rounds
/// each round has a cap
/// the distribution of the total amount of turns is of interest.
pub fn games_flattened(n: usize) -> f64 {
    // rng
    let mut r = rand::rng();

    // bernouli distributions. this may be inefficient
    let berns = PROBS.map(|p| Bernoulli::new(p).unwrap());

    let total: f64 = (0..n).map(|_| game_flattened(&mut r, &berns) as f64).sum();

    total / n as f64
}

/// each game consists of 99 rounds
/// each round has a cap
/// the distribution of the total amount of turns is of interest.
pub fn games_xoshiro128_plus_plus_rng(n: usize) -> f64 {
    // rng
    let mut r: Xoshiro128PlusPlus = rand::make_rng();

    // bernouli distributions. this may be inefficient
    let berns = PROBS.map(|p| Bernoulli::new(p).unwrap());

    let total: f64 = (0..n)
        .map(|_| game_xoshiro128_plus_plus_rng(&mut r, &berns) as f64)
        .sum();

    total / n as f64
}

/// each game consists of 99 rounds
/// each round has a cap
/// the distribution of the total amount of turns is of interest.
pub fn stats_xoshiro128_plus_plus_rng(n: usize) -> Stats {
    let stats = (0..n)
        .into_par_iter()
        .map(|_| {
            let mut r: Xoshiro128PlusPlus = rand::make_rng();
            let berns = PROBS.map(|p| Bernoulli::new(p).unwrap());
            game_xoshiro128_plus_plus_rng(&mut r, &berns)
        })
        .fold(Stats::default, |mut s, x| {
            s.update(x);
            s
        })
        .reduce(Stats::default, |a, b| a.merge(b));

    stats
}
