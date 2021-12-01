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

fn handle_backspace(state: &mut State) {
    if !state.current_word.is_empty() {
        state.current_word.pop();
    } else {
        if state.typed_words.is_empty() {
            return;
        }

        let num_typed_words = state.typed_words.len();
        if state.all_words[num_typed_words - 1] != state.typed_words[num_typed_words - 1] {
            let previous_typed_word = state.typed_words.pop().unwrap();
            state.current_word = previous_typed_word;
        }
    }
}

pub fn handle_key(state: &mut State, k: Key) {
    match k {
        Key::Ctrl('c') => {
            state.quit = true;
        }
        Key::Ctrl('r') => {
            state.retry = true;
        }
        Key::Backspace => {
            handle_backspace(state);
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
