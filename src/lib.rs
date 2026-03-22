//! The game is a tile draw game consisting of 99 rounds
//!
//! each round has 12 draws maximum
//! the game is modelled based on 12 tiles, where the success tile has a weight that
//! varies on each draw: (SUCCESS_WEIGHTS)
//!
//! and the fail tiles have a constant weight: (FAIL_WEIGHT)
//! the game starts with 11 fail tiles, and each subsequent draw reduces their count by 1
//! correspondingly the total weight of the fail tiles for each round is shown in: (FAIL_WEIGHTS)
//!
//! the TOTAL_WEIGHT then shows the SUCCESS_WEIGHTS + FAIL_WEIGHTS which cna be used to
//! calculate the probability (i.e. SUCCESS_WEIGHTS / TOTAL_WEIGHTS)
//!
//! this is hardcoded in PROBS
//!
//! additionally, each round  has a cap on the number of turns it can have which depends on the
//! round number
//! if the round reaches the cap, the next round will win (so the cap + 1)
//! rounds 1, 6, 11, .. have a cap of 5 (so will take at most 6 turns)
//! rounds 2, 7, 12, .. have a cap of 6
//! rounds 3, 8, 13, .. have a cap of 8
//! rounds 4, 9, 14, .. have a cap of 5
//! rounds 5, 10, 15, .. have a cap of 10
//!
//! the CAPS const has these caps.
//!
pub mod game;
pub mod sim;
pub mod stats;

/// probability of success = success weight / total weight
pub const PROBS: [f64; 12] = [
    1.0 / (1.0 + 10.0 * 11.0),
    1.0 / (1.0 + 10.0 * 10.0),
    1.0 / (1.0 + 10.0 * 9.0),
    5.0 / (5.0 + 10.0 * 8.0),
    5.0 / (5.0 + 10.0 * 7.0),
    5.0 / (5.0 + 10.0 * 6.0),
    10.0 / (10.0 + 10.0 * 5.0),
    10.0 / (10.0 + 10.0 * 4.0),
    10.0 / (10.0 + 10.0 * 3.0),
    30.0 / (30.0 + 10.0 * 2.0),
    30.0 / (30.0 + 10.0 * 1.0),
    1.0, // 30.0 / 30.0, + 10.0 * 0.0),
];

/// caps that apply to the round.
/// if the round reaches the cap, the next round will win (so the cap + 1)
/// the caps apply depending on the round number
/// rounds 1, 6, 11, .. have a cap of 5
/// rounds 2, 7, 12, .. have a cap of 6
/// rounds 3, 8, 13, .. have a cap of 8
/// rounds 4, 9, 14, .. have a cap of 5
/// rounds 5, 10, 15, .. have a cap of 10
///
/// so in round 1, the maximum nuimber of turns taken is 6
const CAPS: [usize; 5] = [5, 6, 8, 5, 10];
