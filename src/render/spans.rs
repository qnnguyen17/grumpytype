use std::borrow::Cow;

use tui::style::{Color, Modifier, Style};
use tui::text::Span;

pub(super) fn span_incorrect<'a, T>(content: T) -> Span<'a>
where
    T: Into<Cow<'a, str>>,
{
    Span::styled(
        content,
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
    )
}

pub(super) fn span_default<'a, T>(content: T) -> Span<'a>
where
    T: Into<Cow<'a, str>>,
{
    Span::styled(content, Style::default().fg(Color::Gray))
}

pub(super) fn span_correct<'a, T>(content: T) -> Span<'a>
where
    T: Into<Cow<'a, str>>,
{
    Span::styled(
        content,
        Style::default()
            .fg(Color::Blue)
            .add_modifier(Modifier::BOLD),
    )
}

pub(super) fn spans_highlight_red<'a, T>(spans: T) -> impl Iterator<Item = Span<'a>>
where
    T: IntoIterator<Item = Span<'a>>,
{
    spans
        .into_iter()
        .map(|span| Span::styled(span.content.clone(), span.style.bg(Color::Red)))
}
