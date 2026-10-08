//! Special characters, with a plain alternative for terminals that cannot show them
//! (`output.symbols`). Nothing becomes ambiguous in the plain set.

/// The characters output is decorated with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SymbolSet {
    /// Ends a text that was shortened.
    pub ellipsis: &'static str,
    /// Stands between a value before and after.
    pub arrow: &'static str,
    /// Stands between two linked records.
    pub link: &'static str,
    /// Marks something that worked.
    pub check: &'static str,
    /// Marks a list item.
    pub bullet: &'static str,
    /// The frames of the indication that work is progressing.
    pub spinner: &'static [&'static str],
}

impl SymbolSet {
    /// Symbols that need a terminal able to show Unicode.
    pub const UNICODE: SymbolSet = SymbolSet {
        ellipsis: "…",
        arrow: "→",
        link: "⟷",
        check: "✓",
        bullet: "•",
        spinner: &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
    };

    /// Plain characters only.
    pub const ASCII: SymbolSet = SymbolSet {
        ellipsis: "...",
        arrow: "->",
        link: "<->",
        check: "ok",
        bullet: "-",
        spinner: &["|", "/", "-", "\\"],
    };

    /// The set named by the `output.symbols` setting.
    pub fn named(name: &str) -> Self {
        if name == "ascii" {
            Self::ASCII
        } else {
            Self::UNICODE
        }
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the symbol sets.

    use super::SymbolSet;

    #[test]
    fn the_plain_set_has_only_plain_characters() {
        let plain = SymbolSet::named("ascii");
        let all = [
            plain.ellipsis,
            plain.arrow,
            plain.link,
            plain.check,
            plain.bullet,
        ];
        assert!(
            all.iter()
                .chain(plain.spinner)
                .all(|symbol| symbol.is_ascii())
        );
        assert_eq!(SymbolSet::named("unicode"), SymbolSet::UNICODE);
    }
}
