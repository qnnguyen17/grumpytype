mod spans;
mod text;

use std::io;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::Rect;
use tui::widgets::Block;
use tui::widgets::Borders;
use tui::widgets::Paragraph;
use tui::widgets::Wrap;
use tui::Terminal;

use crate::input::handle_key;
use crate::state::State;
use crate::textgen::Dictionary;

use self::text::get_cursor_position;
use self::text::render_text;
use self::text::word_display_len;

fn drop_first_line(state: &mut State, area: &Rect) {
    let all_words = &state.all_words;

    let mut n_words = 0;
    let mut current_line_len = all_words[0].len() as u16;

    while current_line_len <= area.width - 2 {
        n_words += 1;
        current_line_len +=
            word_display_len(&all_words[n_words], &state.typed_words[n_words]) as u16 + 1;
    }

    state.all_words = all_words[n_words..].to_vec();
    state.typed_words = state.typed_words[n_words..].to_vec();
}

// TODO: load based on the number of lines
fn load_words(state: &mut State, dictionary: &mut Dictionary) {
    let text = &mut state.all_words;
    while text.len() < 100 {
        text.push(dictionary.get_random_word());
    }
}

pub fn render_loop(
    mut state: State,
    mut dictionary: Dictionary,
    input_receiver: Receiver<Key>,
    num_text_lines_to_show: usize,
) -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    let mut last_cursor_x = 1;

    loop {
        if let Ok(key) = input_receiver.recv_timeout(Duration::from_millis(10)) {
            handle_key(&mut state, key);
        }

        if state.quit {
            terminal.clear()?;
            break;
        }

        terminal.draw(|f| {
            load_words(&mut state, &mut dictionary);

            let size = f.size();

            let spans = render_text(&state);

            let block = Block::default().title("grumpytype").borders(Borders::ALL);

            let paragraph = Paragraph::new(spans)
                .block(block)
                .wrap(Wrap { trim: false });

            f.render_widget(paragraph, size);

            let (x, y) = get_cursor_position(&state, size.width as usize - 2);

            f.set_cursor(x, y);

            if last_cursor_x > x && y > 2 {
                drop_first_line(&mut state, &size);
            }

            last_cursor_x = x;
        })?;
    }

    Ok(())
}
