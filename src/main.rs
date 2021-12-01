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

    let mut dictionary =
        Dictionary::from_file(opt.dictionary_path, opt.min_word_len, opt.max_word_len)?;

    let (sender, receiver) = channel();

    thread::spawn(|| {
        input_handling(sender).unwrap();
    });

    loop {
        let mut state = State::default();
        render_loop(
            &mut state,
            &mut dictionary,
            &receiver,
            opt.display_lines,
            opt.time_limit,
        )?;

        if state.quit {
            break;
        }

        print_stats(&mut state, &receiver, opt.time_limit).unwrap();
        if state.quit {
            break;
        }
    }

    Ok(())
}
