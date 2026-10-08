//! Fitting text to the terminal (FR-029): how wide text is when shown, and shortening it
//! without ever cutting a character in two.
//!
//! Width is counted in terminal columns, not characters or bytes: an accented letter takes
//! one column, a wide character (most CJK) takes two.

use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// The width used when the terminal's cannot be known.
pub const DEFAULT_WIDTH: usize = 80;

/// The narrowest width output is laid out for; a narrower terminal wraps on its own.
pub const MINIMUM_WIDTH: usize = 20;

/// How many terminal columns a text takes.
pub fn display_width(text: &str) -> usize {
    UnicodeWidthStr::width(text)
}

/// Shortens a text to at most `columns` columns, ending it with `ellipsis` when something
/// was cut. A text that fits is returned as it is.
pub fn shorten(text: &str, columns: usize, ellipsis: &str) -> String {
    if display_width(text) <= columns {
        return text.to_owned();
    }
    let room = columns.saturating_sub(display_width(ellipsis));
    let mut shortened = String::new();
    let mut used = 0;
    for character in text.chars() {
        let width = UnicodeWidthChar::width(character).unwrap_or(0);
        if used + width > room {
            break;
        }
        shortened.push(character);
        used += width;
    }
    shortened.push_str(ellipsis);
    shortened
}

/// Pads a text with spaces on the right to `columns` columns.
pub fn pad(text: &str, columns: usize) -> String {
    let padding = columns.saturating_sub(display_width(text));
    format!("{text}{}", " ".repeat(padding))
}

/// The width to lay output out for: the `COLUMNS` variable when it holds a number (so that
/// a researcher, or a script, can say), else what the terminal reports, else the default.
pub fn terminal_width(columns_variable: Option<&str>, reported: Option<usize>) -> usize {
    let stated = columns_variable.and_then(|columns| columns.trim().parse::<usize>().ok());
    stated
        .or(reported)
        .unwrap_or(DEFAULT_WIDTH)
        .max(MINIMUM_WIDTH)
}

/// The width of the terminal standard output is connected to, if it is one.
pub fn reported_width() -> Option<usize> {
    terminal_size::terminal_size().map(|(width, _)| usize::from(width.0))
}

#[cfg(test)]
mod tests {
    //! Unit tests for width and shortening (T016).

    use super::{display_width, pad, shorten, terminal_width};

    #[test]
    fn width_counts_accented_and_wide_characters_correctly() {
        assert_eq!(display_width("acao"), 4);
        assert_eq!(display_width("Ação"), 4);
        assert_eq!(display_width("東京"), 4);
        // A letter followed by a combining accent is still one column.
        assert_eq!(display_width("e\u{301}"), 1);
    }

    #[test]
    fn shortening_ends_with_the_ellipsis_and_fits() {
        assert_eq!(
            shorten("Attention Is All You Need", 12, "…"),
            "Attention I…"
        );
        assert_eq!(
            shorten("Attention Is All You Need", 12, "..."),
            "Attention..."
        );
        assert_eq!(
            display_width(&shorten("Attention Is All You Need", 12, "…")),
            12
        );
        assert_eq!(shorten("short", 12, "…"), "short");
    }

    #[test]
    fn shortening_never_splits_a_character() {
        // Each of these characters is two columns wide: only whole ones are kept.
        let shortened = shorten("東京都立大学", 6, "…");
        assert_eq!(shortened, "東京…");
        assert!(display_width(&shortened) <= 6);
        assert_eq!(shorten("Ação e reação", 5, "…"), "Ação…");
    }

    #[test]
    fn padding_aligns_by_columns() {
        assert_eq!(pad("東京", 6), "東京  ");
        assert_eq!(pad("abc", 2), "abc");
    }

    #[test]
    fn the_width_is_what_was_stated_else_reported_else_the_default() {
        assert_eq!(terminal_width(Some("30"), Some(120)), 30);
        assert_eq!(terminal_width(None, Some(120)), 120);
        assert_eq!(terminal_width(Some("wide"), None), 80);
        assert_eq!(terminal_width(Some("3"), None), 20);
    }
}
