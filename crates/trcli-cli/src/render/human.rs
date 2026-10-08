//! The form for people: lines, labelled fields, and tables sized to the terminal
//! (FR-029, FR-037, FR-038).
//!
//! A view draws itself by calling the methods of [`Human`]. Results accumulate as text
//! for standard output; remarks about the result — "nothing to list", "showing 50 of
//! 312" — accumulate separately, for standard error, so that a script reading standard
//! output gets the result and nothing else (FR-031).

use time::{OffsetDateTime, UtcOffset};
use trcli_application::view::Instant;

use super::symbols::SymbolSet;
use super::theme::{Meaning, Theme};
use super::width::{display_width, pad, shorten};

/// How one cell of a table is styled.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    /// The cell's text.
    pub text: String,
    /// The meaning that styles it, if any.
    pub meaning: Option<Meaning>,
}

impl Cell {
    /// A cell with no particular style.
    pub fn plain(text: impl Into<String>) -> Self {
        Self { text: text.into(), meaning: None }
    }

    /// A cell holding the short name of a record.
    pub fn handle(text: impl Into<String>) -> Self {
        Self { text: text.into(), meaning: Some(Meaning::Handle) }
    }

    /// A cell of secondary text.
    pub fn muted(text: impl Into<String>) -> Self {
        Self { text: text.into(), meaning: Some(Meaning::Muted) }
    }
}

/// What a view draws on.
#[derive(Debug)]
pub struct Human<'a> {
    /// The styles in effect on standard output.
    theme: &'a Theme,
    /// The special characters in effect.
    pub symbols: SymbolSet,
    /// The width output is laid out for.
    width: usize,
    /// The researcher's offset from UTC, for showing moments in local time.
    zone: UtcOffset,
    /// What will go to standard output.
    body: String,
    /// Remarks about the result, for standard error.
    notices: Vec<String>,
}

impl<'a> Human<'a> {
    /// A blank page of the given width.
    pub fn new(theme: &'a Theme, symbols: SymbolSet, width: usize, zone: UtcOffset) -> Self {
        Self { theme, symbols, width, zone, body: String::new(), notices: Vec::new() }
    }

    /// Splits the page into what goes to standard output and the remarks for standard error.
    pub fn finish(self) -> (String, Vec<String>) {
        (self.body, self.notices)
    }

    /// Text in the style of a meaning.
    pub fn paint(&self, meaning: Meaning, text: &str) -> String {
        self.theme.paint(meaning, text)
    }

    /// Adds one line.
    pub fn line(&mut self, text: impl AsRef<str>) {
        self.body.push_str(text.as_ref());
        self.body.push('\n');
    }

    /// Adds a heading.
    pub fn heading(&mut self, text: &str) {
        let painted = self.paint(Meaning::Heading, text);
        self.line(painted);
    }

    /// Adds a line that says something worked.
    pub fn success(&mut self, text: &str) {
        self.line(text);
    }

    /// Adds a remark about the result; it goes to standard error.
    pub fn notice(&mut self, text: impl Into<String>) {
        self.notices.push(text.into());
    }

    /// Adds labelled values, one per line, with the values aligned. A value that spans
    /// several lines is continued under itself.
    pub fn fields(&mut self, fields: &[(&str, String)]) {
        let label_width = fields.iter().map(|(label, _)| display_width(label)).max().unwrap_or(0);
        for (label, value) in fields {
            let mut lines = value.lines();
            let first = lines.next().unwrap_or_default();
            let painted = self.paint(Meaning::Muted, &pad(label, label_width));
            self.line(format!("  {painted}  {first}"));
            for continuation in lines {
                self.line(format!("  {}  {continuation}", " ".repeat(label_width)));
            }
        }
    }

    /// Adds a table. Columns are as wide as their widest cell; when the table is wider
    /// than the terminal, the column at `flexible` is shortened to make it fit.
    pub fn table(&mut self, headers: &[&str], rows: &[Vec<Cell>], flexible: usize) {
        let mut widths: Vec<usize> = headers.iter().map(|header| display_width(header)).collect();
        for row in rows {
            for (column, cell) in row.iter().enumerate() {
                widths[column] = widths[column].max(display_width(&cell.text));
            }
        }
        self.fit(&mut widths, flexible);
        let header: Vec<Cell> = headers.iter().map(|header| Cell { text: (*header).to_owned(), meaning: Some(Meaning::Heading) }).collect();
        self.row(&header, &widths);
        for row in rows {
            self.row(row, &widths);
        }
    }

    /// Narrows the flexible column until the table fits the terminal, keeping it readable.
    fn fit(&self, widths: &mut [usize], flexible: usize) {
        let gaps = 2 * widths.len().saturating_sub(1);
        let total: usize = widths.iter().sum::<usize>() + gaps;
        if total > self.width && flexible < widths.len() {
            let excess = total - self.width;
            widths[flexible] = widths[flexible].saturating_sub(excess).max(8).min(widths[flexible]);
        }
    }

    /// Adds one row of a table.
    fn row(&mut self, cells: &[Cell], widths: &[usize]) {
        let last = cells.len().saturating_sub(1);
        let mut line = String::new();
        for (column, cell) in cells.iter().enumerate() {
            let shortened = shorten(&cell.text, widths[column], self.symbols.ellipsis);
            // The last column is not padded, so that lines do not end in spaces.
            let padded = if column == last { shortened } else { pad(&shortened, widths[column]) };
            let painted = cell.meaning.map_or(padded.clone(), |meaning| self.paint(meaning, &padded));
            line.push_str(&painted);
            if column != last {
                line.push_str("  ");
            }
        }
        self.line(line.trim_end());
    }

    /// A moment in the researcher's local time: `2026-10-08 14:02`.
    pub fn moment(&self, instant: Instant) -> String {
        format_moment(instant.0, self.zone, false)
    }

    /// A moment in local time with its zone, for where the zone matters: `2026-10-08
    /// 14:02 +02:00`.
    pub fn moment_with_zone(&self, instant: Instant) -> String {
        format_moment(instant.0, self.zone, true)
    }

    /// The remark for a list cut short: how many are shown, how many there are, and how
    /// to see more (FR-037).
    pub fn limited(&mut self, shown: usize, total: u64) {
        if (shown as u64) < total {
            self.notice(format!("Showing {shown} of {total}. Use --limit <n> to see more."));
        }
    }
}

/// A moment as `YYYY-MM-DD HH:MM` in a zone, with the zone when asked (FR-038).
pub fn format_moment(moment: OffsetDateTime, zone: UtcOffset, with_zone: bool) -> String {
    let local = moment.to_offset(zone);
    let (year, month, day) = (local.year(), u8::from(local.month()), local.day());
    let text = format!("{year:04}-{month:02}-{day:02} {:02}:{:02}", local.hour(), local.minute());
    if !with_zone {
        return text;
    }
    if zone.is_utc() {
        return format!("{text} UTC");
    }
    let (hours, minutes, _) = zone.as_hms();
    let sign = if zone.is_negative() { '-' } else { '+' };
    format!("{text} {sign}{:02}:{:02}", hours.abs(), minutes.abs())
}

#[cfg(test)]
mod tests {
    //! Unit tests for the layout helpers.

    use time::{OffsetDateTime, UtcOffset};
    use trcli_application::settings::layers::Settings;

    use super::{Cell, Human, format_moment};
    use crate::render::symbols::SymbolSet;
    use crate::render::theme::Theme;

    /// What a table of two rows looks like at a width.
    fn table_at(width: usize, symbols: SymbolSet) -> String {
        let theme = Theme::plain();
        let mut out = Human::new(&theme, symbols, width, UtcOffset::UTC);
        let rows = vec![
            vec![Cell::handle("spc-7k3f"), Cell::plain("Attention Is All You Need, a very long title indeed")],
            vec![Cell::handle("spc-9abc"), Cell::plain("東京")],
        ];
        out.table(&["HANDLE", "TITLE"], &rows, 1);
        out.finish().0
    }

    #[test]
    fn a_table_is_aligned_and_fits_a_wide_terminal_untouched() {
        let table = table_at(120, SymbolSet::UNICODE);
        let lines: Vec<&str> = table.lines().collect();
        assert_eq!(lines[0], "HANDLE    TITLE");
        assert_eq!(lines[1], "spc-7k3f  Attention Is All You Need, a very long title indeed");
        assert_eq!(lines[2], "spc-9abc  東京");
    }

    #[test]
    fn a_narrow_terminal_shortens_the_flexible_column() {
        let table = table_at(30, SymbolSet::UNICODE);
        let long = table.lines().nth(1).expect("a row");
        assert!(long.ends_with('…'), "{long}");
        assert!(crate::render::width::display_width(long) <= 30, "{long}");
        assert!(table_at(30, SymbolSet::ASCII).lines().nth(1).expect("a row").ends_with("..."));
    }

    #[test]
    fn fields_are_aligned_and_a_long_value_continues_under_itself() {
        let theme = Theme::from_settings(&Settings::default(), false);
        let mut out = Human::new(&theme, SymbolSet::UNICODE, 80, UtcOffset::UTC);
        out.fields(&[("Name", "Doctorate".to_owned()), ("Description", "line one\nline two".to_owned())]);
        let (body, _) = out.finish();
        assert_eq!(body, "  Name         Doctorate\n  Description  line one\n               line two\n");
    }

    #[test]
    fn remarks_are_kept_apart_from_the_result() {
        let theme = Theme::plain();
        let mut out = Human::new(&theme, SymbolSet::UNICODE, 80, UtcOffset::UTC);
        out.line("a result");
        out.limited(50, 312);
        out.limited(3, 3);
        let (body, notices) = out.finish();
        assert_eq!(body, "a result\n");
        assert_eq!(notices, ["Showing 50 of 312. Use --limit <n> to see more."]);
    }

    #[test]
    fn moments_are_shown_in_one_form_in_local_time() {
        let moment = OffsetDateTime::from_unix_timestamp(1_791_468_120).expect("valid");
        let plus_two = UtcOffset::from_hms(2, 0, 0).expect("valid");
        let minus = UtcOffset::from_hms(-3, -30, 0).expect("valid");
        assert_eq!(format_moment(moment, UtcOffset::UTC, false), "2026-10-08 14:02");
        assert_eq!(format_moment(moment, UtcOffset::UTC, true), "2026-10-08 14:02 UTC");
        assert_eq!(format_moment(moment, plus_two, true), "2026-10-08 16:02 +02:00");
        assert_eq!(format_moment(moment, minus, true), "2026-10-08 10:32 -03:30");
    }
}
