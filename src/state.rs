// TODO: store only the text that _hasn't_ been written yet,
// and the actual _spans_ of the typed words. This should help performance!
// TODO: store the cursor position instead of calculating every time
#[derive(Debug)]
pub struct State {
    pub quit: bool,
    pub text: Vec<String>,
    pub typed_words: Vec<String>,
    pub current_word: String,
}

impl State {
    pub fn new() -> Self {
        Self {
            quit: false,
            text: "the quick brown fox jumped over the lazy dog lorem ipsum whatever foo bar bizz buzz"
                .split(' ')
                .map(Into::into)
                .collect(),
            typed_words: Vec::new(),
            current_word: "".into(),
        }
    }
}
