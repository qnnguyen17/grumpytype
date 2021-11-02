use std::cmp::max;

use itertools::{zip, EitherOrBoth, Itertools};
use tui::text::{Span, Spans};

use crate::state::State;

use super::spans::{span_correct, span_default, span_incorrect, spans_highlight_red};

pub(super) fn word_display_len(s1: &str, s2: &str) -> usize {
    max(s1.len(), s2.len())
}

fn render_word<'a>(
    typed_text: &'a str,
    expected_text: &'a str,
    completed_typing: bool,
) -> Vec<Span<'a>> {
    let spans = typed_text
        .chars()
        .zip_longest(expected_text.chars())
        .map(|entry| match entry {
            EitherOrBoth::Left(t) => span_incorrect(t.to_string()),
            EitherOrBoth::Right(e) if completed_typing => span_incorrect(e.to_string()),
            EitherOrBoth::Right(e) => span_default(e.to_string()),
            EitherOrBoth::Both(t, e) if t != e => span_incorrect(t.to_string()),
            EitherOrBoth::Both(t, _) => span_correct(t.to_string()),
        });
    if typed_text != expected_text && completed_typing {
        spans_highlight_red(spans).collect()
    } else {
        spans.collect()
    }
}

pub(super) fn get_cursor_position(state: &State, text_area_width: usize) -> (u16, u16) {
    let all_words = &state.all_words;
    let typed_words = &state.typed_words;
    let current_word = &state.current_word;

    let mut current_line = 0;
    let mut current_line_len = match typed_words.get(0) {
        Some(s) => word_display_len(s, &all_words[0]),
        None => return (current_word.len() as u16 + 1, 1),
    };

    let typed_and_expected_zip = zip(typed_words[1..].iter(), all_words[1..].iter());

    typed_and_expected_zip.for_each(|(typed, expected)| {
        let word_len = word_display_len(typed, expected);
        if current_line_len + 1 + word_len > text_area_width {
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

    let next_word_len = word_display_len(&current_word, &all_words[typed_words.len()]);

    if current_line_len + next_word_len > text_area_width {
        (current_word.len() as u16 + 1, current_line + 2)
    } else {
        (
            (current_line_len + current_word.len() + 1) as u16,
            current_line + 1,
        )
    }
}

pub(super) fn render_text(state: &State) -> Spans {
    let all_words = &state.all_words;
    let typed_words = &state.typed_words;
    let current_word = &state.current_word;

    let num_typed_words = typed_words.len();
    let mut spans = Vec::new();

    for (typed, expected) in zip(typed_words.iter(), all_words.iter()) {
        spans.extend(render_word(typed, expected, true));
        spans.push(" ".into());
    }

    spans.extend(render_word(
        &current_word,
        &all_words[num_typed_words],
        false,
    ));

    spans.push(" ".into());

    spans.push(span_default(all_words[num_typed_words + 1..].join(" ")));

    Spans::from(spans)
}
