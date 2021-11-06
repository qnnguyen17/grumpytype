use std::{io, sync::mpsc::SendError};

use termion::event::Key;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("dictionary loading failed")]
    DictionaryLoad(#[source] io::Error),

    #[error("failed getting key input")]
    InputKey(#[source] io::Error),

    #[error("failed to send key input on channel")]
    InputSend(#[source] SendError<Key>),

    #[error("failed to switch to raw mode output")]
    RawMode(#[source] io::Error),

    #[error("failed to clear terminal")]
    TerminalClear(#[source] io::Error),

    #[error("failed to draw in terminal")]
    TerminalDraw(#[source] io::Error),

    #[error("failed to instantiate terminal object")]
    TerminalInstantiation(#[source] io::Error),
}
