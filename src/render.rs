use std::cmp::max;
use std::io;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use itertools::zip;
use itertools::EitherOrBoth;
use itertools::Itertools;
use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::Rect;
use tui::style::Color;
use tui::style::Modifier;
use tui::style::Style;
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::Block;
use tui::widgets::Borders;
use tui::widgets::Paragraph;
use tui::widgets::Wrap;
use tui::Terminal;

use crate::input::handle_key;
use crate::state::State;
use crate::textgen::Dictionary;

fn render_typed_word<'a>(
    typed_text: &'a str,
    expected_text: &'a str,
    completed_typing: bool,
) -> Vec<Span<'a>> {
    typed_text
        .chars()
        .zip_longest(expected_text.chars())
        .map(|entry| match entry {
            EitherOrBoth::Left(t) => Span::styled(
                t.to_string(),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            EitherOrBoth::Right(e) => {
                let style = Style::default();
                let style = if completed_typing {
                    style.fg(Color::Red).add_modifier(Modifier::BOLD)
                } else {
                    style.fg(Color::Gray)
                };
                Span::styled(e.to_string(), style)
            }
            EitherOrBoth::Both(t, e) if t != e => Span::styled(
                t.to_string(),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            EitherOrBoth::Both(t, _) => Span::styled(
                t.to_string(),
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
        })
        .collect::<Vec<Span>>()
}

fn get_cursor_position(state: &State, area: &Rect) -> (u16, u16) {
    // Subtract 2 for each side of the border
    let type_area_width = area.width as usize - 2;

    let mut current_line = 0;
    let mut current_line_len = match state.typed_words.get(0) {
        Some(s) => s.len(),
        None => return (state.current_word.len() as u16 + 1, 1),
    };

    zip(state.typed_words[1..].iter(), state.text[1..].iter()).for_each(|(typed, expected)| {
        let word_len = max(typed.len(), expected.len());
        if current_line_len + 1 + word_len > type_area_width {
            // If length of the space + the next word exceeds the width, then go to the next line
            current_line_len = word_len;
            current_line += 1;
        } else {
            // Otherwise, move the cursor right by 1 word
            current_line_len += 1 + word_len;
        }
    });

    // Add the space that comes after the last fully typed word
    current_line_len += 1;

    let next_word_len = max(
        state.current_word.len(),
        state.text[state.typed_words.len()].len(),
    );

    if current_line_len + next_word_len > type_area_width {
        (state.current_word.len() as u16 + 1, current_line + 2)
    } else {
        (
            (current_line_len + state.current_word.len() + 1) as u16,
            current_line + 1,
        )
    }
}

fn load_words(state: &mut State, dictionary: &mut Dictionary) {
    let text = &mut state.text;
    while text.len() < 50 {
        text.push(dictionary.get_random_word());
    }
}

pub fn render_loop(
    mut state: State,
    mut dictionary: Dictionary,
    input_receiver: Receiver<Key>,
) -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    loop {
        match input_receiver.recv_timeout(Duration::from_millis(50)) {
            Ok(key) => handle_key(&mut state, key),
            Err(_) => {}
        }

        let quit = state.quit.load(Ordering::SeqCst);
        if quit {
            terminal.clear()?;
            break;
        }

        terminal.draw(|f| {
            load_words(&mut state, &mut dictionary);

            let size = f.size();
            let num_typed_words = state.typed_words.len();

            let mut spans = Vec::new();

            for (typed, expected) in zip(state.typed_words.iter(), state.text.iter()) {
                spans.extend(render_typed_word(typed, expected, true));
                spans.push(" ".into());
            }

            spans.extend(render_typed_word(
                &state.current_word,
                &state.text[num_typed_words],
                false,
            ));

            spans.push(" ".into());

            spans.push(Span::styled(
                state.text[num_typed_words + 1..].join(" "),
                Style::default().fg(Color::Gray),
            ));

            let spans = Spans::from(spans);

            let block = Block::default().title("grumpytype").borders(Borders::ALL);
            let paragraph = Paragraph::new(spans)
                .block(block)
                .wrap(Wrap { trim: false });
            f.render_widget(paragraph, size);

            let (x, y) = get_cursor_position(&state, &size);

            f.set_cursor(x, y);
        })?;
    }

    Ok(())
}
