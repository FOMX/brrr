use rand::rngs::{SmallRng, ThreadRng, Xoshiro128PlusPlus};
use rand_distr::{Bernoulli, Distribution};

use crate::CAPS;

/// each game consists of 99 rounds
/// each round has a cap
/// the distribution of the total amount of turns is of interest.
pub fn game_original(mut r: &mut ThreadRng, berns: &[Bernoulli; 12]) -> usize {
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
    turns
}

/// each game consists of 99 rounds
/// each round has a cap
/// the distribution of the total amount of turns is of interest.
pub fn game_restructured(mut r: &mut ThreadRng, berns: &[Bernoulli; 12]) -> usize {
    // track the turns taken to complete the game
    let mut turns = 0;

    for (i, &cap) in CAPS.iter().enumerate() {
        // each cap repeats 20 times in 100 rounds, but we only want 99
        // so drop the last iteration of the last cap (i == 4)
        let repeats = if i == 4 { 19 } else { 20 };

        for _ in 0..repeats {
            let turn = (0..=cap)
                .find(|&attempt| berns[attempt].sample(&mut r))
                .map(|a| a + 1)
                .unwrap_or(cap + 1);
            turns += turn;
        }
    }
    turns
}

// each game consists of 99 rounds
/// each round has a cap
/// the distribution of the total amount of turns is of interest.
pub fn game_flattened(mut r: &mut ThreadRng, berns: &[Bernoulli; 12]) -> usize {
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

// each game consists of 99 rounds
/// each round has a cap
/// the distribution of the total amount of turns is of interest.
pub fn game_small_rng(mut r: &mut SmallRng, berns: &[Bernoulli; 12]) -> usize {
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