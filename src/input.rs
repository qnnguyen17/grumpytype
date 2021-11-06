use std::io;
use std::sync::mpsc::Sender;
use std::time::Instant;

use termion::event::Key;
use termion::input::TermRead;

use crate::error::ApplicationError;
use crate::state::State;

pub fn input_handling(input_sender: Sender<Key>) -> Result<(), ApplicationError> {
    let keys = io::stdin().keys();
    for k in keys {
        let k = k.map_err(ApplicationError::InputKey)?;
        input_sender.send(k).map_err(ApplicationError::InputSend)?;
    }
    Ok(())
}

fn handle_space(state: &mut State) {
    state.counters.attempted_word_count += 1;

    let typed_word = &state.current_word;

    if typed_word == &state.all_words[state.typed_words.len()] {
        state.counters.correctly_typed_word_count += 1;
    }

    state.typed_words.push(typed_word.clone());
    state.current_word = "".into();
}

fn handle_alpha(state: &mut State, c: char) {
    if state.start_time.is_none() {
        state.start_time = Some(Instant::now());
    }
    state.current_word.push(c);
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
                handle_space(state);
            } else if c.is_ascii_alphabetic() {
                handle_alpha(state, c);
            }
        }
        _ => {}
    }
}
