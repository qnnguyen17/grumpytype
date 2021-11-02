// TODO: store only the text that _hasn't_ been written yet,
// and the actual _spans_ of the typed words. This should help performance!
// TODO: store the cursor position instead of calculating every time
#[derive(Debug, Default)]
pub struct State {
    pub quit: bool,
    pub text: Vec<String>,
    pub typed_words: Vec<String>,
    pub current_word: String,
}
