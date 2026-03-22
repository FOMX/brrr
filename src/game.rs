use rand::rngs::Xoshiro128PlusPlus;
use rand_distr::{Bernoulli, Distribution};

use crate::CAPS;

// each game consists of 99 rounds
/// each round has a cap
/// the distribution of the total amount of turns is of interest.
pub fn game_xoshiro128_plus_plus_rng(
    mut r: &mut Xoshiro128PlusPlus,
    berns: &[Bernoulli; 12],
) -> usize {
    CAPS.iter()
        .enumerate()
        .flat_map(|(i, &cap)| {
            // each cap repeats 20 times in 100 rounds, but we only want 99
            // so drop the last iteration of the last cap (i == 4)
            let repeats = if i == 4 { 19 } else { 20 };
            std::iter::repeat_n(cap, repeats)
        })
        .map(|cap| {
            berns[..cap]
                .iter()
                .position(|b| b.sample(&mut r))
                .map(|a| a + 1)
                .unwrap_or(cap + 1)
        })
        .sum()
}
