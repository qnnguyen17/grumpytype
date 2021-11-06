mod dictionary;
mod error;
mod input;
mod opt;
mod render;
mod state;
mod stats;

use std::sync::mpsc::channel;
use std::thread;

use structopt::StructOpt;

use dictionary::Dictionary;
use error::ApplicationError;
use input::input_handling;
use opt::CliOptions;
use render::{print_stats, render_loop};
use state::State;

fn main() -> Result<(), ApplicationError> {
    let opt = CliOptions::from_args();

    let mut state = State::default();

    let dictionary =
        Dictionary::from_file(opt.dictionary_path, opt.min_word_len, opt.max_word_len)?;

    let (sender, receiver) = channel();

    thread::spawn(|| {
        input_handling(sender).unwrap();
    });

    render_loop(
        &mut state,
        dictionary,
        receiver,
        opt.display_lines,
        opt.time_limit,
    )?;

    print_stats(&state.counters, opt.time_limit);

    Ok(())
}
