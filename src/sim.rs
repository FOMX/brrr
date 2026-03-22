use rand::rngs::SmallRng;
use rand_distr::Bernoulli;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{PROBS, game::game, stats::Stats};

/// each game consists of 99 rounds
/// each round has a cap
/// the distribution of the total amount of turns is of interest.
pub fn stats(n: usize) -> Stats {
    (0..n)
        .into_par_iter()
        .map(|_| {
            let mut r: SmallRng = rand::make_rng();
            let berns = PROBS.map(|p| Bernoulli::new(p).unwrap());
            game(&mut r, &berns)
        })
        .fold(Stats::default, |mut s, x| {
            s.update(x);
            s
        })
        .reduce(Stats::default, |a, b| a.merge(b))
}
