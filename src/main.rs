mod dictionary;
mod error;
mod input;
mod render;
mod state;
mod stats;

use std::sync::mpsc::channel;
use std::thread;

use dictionary::Dictionary;
use error::ApplicationError;
use input::input_handling;
use render::{print_stats, render_loop};
use state::State;

fn main() -> Result<(), ApplicationError> {
    let mut state = State::default();

    let dictionary = Dictionary::from_file("google-10000-english-usa.txt", 3, 7)?;

    let (sender, receiver) = channel();

    thread::spawn(|| {
        input_handling(sender).unwrap();
    });

    // TODO: accept args to determine these!
    let num_text_lines_to_show = 5;
    let time_limit_sec = 15;

    render_loop(
        &mut state,
        dictionary,
        receiver,
        num_text_lines_to_show,
        time_limit_sec,
    )?;

    print_stats(&state.counters, time_limit_sec);

    Ok(())
}
