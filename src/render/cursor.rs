use itertools::zip;
use tui::layout::Rect;

use crate::state::State;

use super::text::word_display_len;

#[derive(Clone, Copy)]
pub(super) struct CursorPosition {
    pub x: u16,
    pub y: u16,
}

pub(super) fn get_cursor_position(state: &State, text_area_without_border: Rect) -> CursorPosition {
    let all_words = &state.all_words;
    let typed_words = &state.typed_words;
    let current_word = &state.current_word;

    let mut current_line_len = match typed_words.get(0) {
        Some(s) => word_display_len(s, &all_words[0]),
        None => {
            return CursorPosition {
                x: current_word.len() as u16 + text_area_without_border.x,
                y: text_area_without_border.y,
            }
        }
    };

    let mut current_line = 0;
    let typed_and_expected_zip = zip(typed_words[1..].iter(), all_words[1..].iter());

    typed_and_expected_zip.for_each(|(typed, expected)| {
        let word_len = word_display_len(typed, expected);
        if current_line_len + 1 + word_len > text_area_without_border.width as usize {
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

    if current_line_len + next_word_len > text_area_without_border.width as usize {
        // Go to next line
        CursorPosition {
            x: current_word.len() as u16 + text_area_without_border.x,
            y: current_line + 1 + text_area_without_border.y,
        }
    } else {
        CursorPosition {
            x: (current_line_len + current_word.len() + text_area_without_border.x as usize) as u16,
            y: current_line + text_area_without_border.y,
        }
    }
}
