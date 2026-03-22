use num_format::{Locale, ToFormattedString};
use std::{fmt::Display, usize};

#[derive(Debug, Clone, Copy)]
pub struct Stats {
    count: usize,
    mean: f64,
    m2: f64, // accumulated squared deviations
    min: usize,
    max: usize,
}
impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "count: {}, mean: {:>.3}, min: {} max: {}",
            self.count.to_formatted_string(&Locale::es_US),
            self.mean,
            self.min,
            self.max
        )
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            count: Default::default(),
            mean: Default::default(),
            m2: Default::default(),
            min: usize::MAX,
            max: Default::default(),
        }
    }
}

impl Stats {
    pub fn update(&mut self, x: usize) {
        self.count += 1;
        let x = x as f64;
        let delta = x - self.mean;
        self.mean += delta / self.count as f64;
        let delta2 = x - self.mean;
        self.m2 += delta * delta2;
        self.min = self.min.min(x as usize);
        self.max = self.max.max(x as usize);
    }

    pub fn variance(&self) -> f64 {
        self.m2 / (self.count - 1) as f64
    }
    pub fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }

    pub fn merge(self, other: Stats) -> Stats {
        let count = self.count + other.count;
        let delta = other.mean - self.mean;
        let mean = self.mean + delta * other.count as f64 / count as f64;
        let m2 =
            self.m2 + other.m2 + delta * delta * (self.count * other.count) as f64 / count as f64;
        Stats {
            count,
            mean,
            m2,
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }
}
