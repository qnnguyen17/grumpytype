mod input;
mod render;
mod state;

use std::io;
use std::sync::mpsc::channel;
use std::thread;

use input::input_handling;
use render::render_loop;
use state::State;

fn main() -> Result<(), io::Error> {
    let state = State::new();

    let (sender, receiver) = channel();

    thread::spawn(|| {
        input_handling(sender).unwrap();
    });

    render_loop(state, receiver)?;

    Ok(())
}
