mod input;
mod render;
mod state;
mod textgen;

use std::io;
use std::sync::mpsc::channel;
use std::thread;

use textgen::Dictionary;
use input::input_handling;
use render::render_loop;
use state::State;

fn main() -> Result<(), io::Error> {
    let state = State::default();

    let dictionary = Dictionary::from_file("google-10000-english-usa.txt")?;

    let (sender, receiver) = channel();

    thread::spawn(|| {
        input_handling(sender).unwrap();
    });

    render_loop(state, dictionary, receiver)?;

    Ok(())
}
