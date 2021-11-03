mod cursor;
mod spans;
mod text;

use std::cmp::min;
use std::io;
use std::io::Stdout;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

use termion::event::Key;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::text::Span;
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use tui::Frame;
use tui::Terminal;

use crate::dictionary::Dictionary;
use crate::input::handle_key;
use crate::state::{Counters, State};
use crate::stats::Stats;

use self::cursor::get_cursor_position;
use self::cursor::CursorPosition;
use self::text::{render_text, word_display_len};

fn get_typing_seconds(state: &State) -> Option<u64> {
    state
        .start_time
        .and_then(|start_time| Instant::now().checked_duration_since(start_time))
        .as_ref()
        .map(Duration::as_secs)
}

fn handle_timer(state: &mut State, time_limit_sec: u64) {
    if let Some(elapsed_seconds) = get_typing_seconds(state) {
        if elapsed_seconds >= time_limit_sec {
            state.quit = true;
        }
    }
}

fn drop_first_line(state: &mut State, text_area_without_border: &Rect) {
    let all_words = &state.all_words;

    let mut n_words = 0;
    let mut current_line_len = all_words[0].len() as u16;

    while current_line_len <= text_area_without_border.width {
        let expected_word = &all_words[n_words];
        let typed_word = &state.typed_words[n_words];
        n_words += 1;
        current_line_len += word_display_len(expected_word, typed_word) as u16 + 1;
    }

    state.all_words = all_words[n_words..].to_vec();
    state.typed_words = state.typed_words[n_words..].to_vec();
}

fn ui_layout(area: Rect, text_area_height: u16) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(8)
        .constraints([
            Constraint::Length(1),
            // Add 2 for the borders
            Constraint::Length(text_area_height + 2),
            Constraint::Min(0),
        ])
        .split(area)
}

fn draw_timer(
    frame: &mut Frame<TermionBackend<AlternateScreen<RawTerminal<Stdout>>>>,
    state: &State,
    time_limit_sec: u64,
    area: Rect,
) {
    if let Some(elapsed_seconds) = get_typing_seconds(state) {
        let timer_text = time_limit_sec - elapsed_seconds;
        let timer_text = timer_text.to_string();
        let paragraph = Paragraph::new(vec![Span::raw(timer_text).into()]);
        frame.render_widget(paragraph, area);
    }
}

fn draw_text_area(
    frame: &mut Frame<TermionBackend<AlternateScreen<RawTerminal<Stdout>>>>,
    state: &State,
    area: Rect,
) {
    let spans = render_text(state);
    let block = Block::default().borders(Borders::ALL);
    let paragraph = Paragraph::new(spans)
        .block(block)
        .wrap(Wrap { trim: false });
    frame.render_widget(paragraph, area);
}

fn draw_cursor(
    frame: &mut Frame<TermionBackend<AlternateScreen<RawTerminal<Stdout>>>>,
    state: &State,
    text_area_without_border: Rect,
) -> CursorPosition {
    let cursor_position = get_cursor_position(state, text_area_without_border);
    frame.set_cursor(cursor_position.x, cursor_position.y);
    cursor_position
}

fn drop_line_if_necessary(
    state: &mut State,
    cursor_position: CursorPosition,
    last_cursor_x: u16,
    num_text_lines_to_show: usize,
    text_area_without_border: Rect,
) {
    if last_cursor_x > cursor_position.x
        && cursor_position.y as usize
            > (num_text_lines_to_show / 2) + text_area_without_border.y as usize
    {
        drop_first_line(state, &text_area_without_border);
    }
}

pub fn render_loop(
    state: &mut State,
    mut dictionary: Dictionary,
    input_receiver: Receiver<Key>,
    num_text_lines_to_show: usize,
    time_limit_sec: u64,
) -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    let mut last_cursor_x = 1;

    loop {
        if let Ok(key) = input_receiver.recv_timeout(Duration::from_millis(10)) {
            handle_key(state, key);
        }

        handle_timer(state, time_limit_sec);

        if state.quit {
            terminal.clear()?;
            break;
        }

        terminal.draw(|mut f| {
            dictionary.load_words(state, 300);

            let size = f.size();

            let text_area_height = min(num_text_lines_to_show, size.height as usize);

            let layout = ui_layout(size, text_area_height as u16);

            let timer_area = layout[0];
            let text_area_and_border = layout[1];

            draw_timer(&mut f, state, time_limit_sec, timer_area);

            draw_text_area(&mut f, state, text_area_and_border);

            let text_area_without_border = Rect {
                x: text_area_and_border.x + 1,
                y: text_area_and_border.y + 1,
                width: text_area_and_border.width - 2,
                height: text_area_and_border.height - 2,
            };

            let cursor_position = draw_cursor(&mut f, state, text_area_without_border);
            drop_line_if_necessary(
                state,
                cursor_position,
                last_cursor_x,
                num_text_lines_to_show,
                text_area_without_border,
            );

            last_cursor_x = cursor_position.x;
        })?;
    }

    Ok(())
}

// TODO: maybe display this with a nice TUI
pub fn print_stats(counters: &Counters, time_limit_sec: u64) {
    let maybe_stats = Stats::from_counters(counters, time_limit_sec);
    if let Some(Stats { accuracy, wpm }) = maybe_stats {
        println!("accuracy: {:.2}%", accuracy * 100.0);
        println!("wpm: {:.2}", wpm);
    }
}
