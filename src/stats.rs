use crate::state::Counters;

pub struct Stats {
    pub accuracy: f64,
    pub wpm: f64,
}

impl Stats {
    pub fn from_counters(counters: &Counters, time_limit_sec: u64) -> Option<Self> {
        if counters.attempted_word_count > 0 {
            let accuracy =
                counters.correctly_typed_word_count as f64 / counters.attempted_word_count as f64;
            let wpm = counters.correctly_typed_word_count as f64 * (60.0 / time_limit_sec as f64);

            Some(Self { accuracy, wpm })
        } else {
            None
        }
    }
}
