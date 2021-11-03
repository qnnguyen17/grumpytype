use std::time::Instant;

#[derive(Debug, Default)]
pub struct Counters {
    pub attempted_word_count: usize,
    pub correctly_typed_word_count: usize,
}

// TODO: store only the text that _hasn't_ been written yet,
// and the actual _spans_ of the typed words. This should help performance!
// TODO: store the cursor position instead of calculating every time
#[derive(Debug, Default)]
pub struct State {
    pub start_time: Option<Instant>,
    pub quit: bool,
    pub all_words: Vec<String>,
    pub typed_words: Vec<String>,
    pub current_word: String,
    pub counters: Counters,
}
