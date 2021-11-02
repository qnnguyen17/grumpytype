use std::io;
use std::sync::mpsc::Sender;

use termion::event::Key;
use termion::input::TermRead;

use crate::state::State;

pub(crate) fn input_handling(input_sender: Sender<Key>) -> Result<(), io::Error> {
    let keys = io::stdin().keys();
    // TODO: error
    for k in keys {
        input_sender.send(k?).unwrap();
    }
    Ok(())
}

pub fn handle_key(state: &mut State, k: Key) {
    match k {
        Key::Ctrl('c') => {
            state.quit = true;
        }
        Key::Backspace => {
            state.current_word.pop();
        }
        Key::Char(c) => {
            if c == ' ' {
                state.typed_words.push(state.current_word.clone());
                state.current_word = "".into();
            } else if c.is_ascii_alphabetic() {
                state.current_word.push(c);
            }
        }
        _ => {}
    }
}
