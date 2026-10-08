//! How alike two short names are, to suggest close matches for a mistyped one (FR-011).
//!
//! This is the classic edit distance: the fewest single-character insertions, deletions,
//! and substitutions that turn one text into the other. Short names are a dozen
//! characters at most, so the plain two-row algorithm is more than fast enough.

/// The edit distance between two texts.
pub fn distance(one: &str, other: &str) -> usize {
    let other: Vec<char> = other.chars().collect();
    // `previous[j]` is the distance between the part of `one` seen so far and the first
    // `j` characters of `other`.
    let mut previous: Vec<usize> = (0..=other.len()).collect();
    for (row, from) in one.chars().enumerate() {
        let mut current = vec![row + 1];
        for (column, to) in other.iter().enumerate() {
            let substitution = previous[column] + usize::from(from != *to);
            let insertion = current[column] + 1;
            let deletion = previous[column + 1] + 1;
            current.push(substitution.min(insertion).min(deletion));
        }
        previous = current;
    }
    previous[other.len()]
}

/// Up to `limit` of `candidates` closest to `typed`, nearest first, leaving out those too
/// different to be what was meant (more than half of what was typed would have to change).
pub fn closest<'a>(
    typed: &str,
    candidates: impl IntoIterator<Item = &'a str>,
    limit: usize,
) -> Vec<&'a str> {
    let threshold = (typed.chars().count() / 2).max(2);
    let mut scored: Vec<(usize, &str)> = candidates
        .into_iter()
        .map(|candidate| (distance(typed, candidate), candidate))
        .filter(|(distance, _)| *distance <= threshold)
        .collect();
    scored.sort();
    scored
        .into_iter()
        .take(limit)
        .map(|(_, candidate)| candidate)
        .collect()
}

#[cfg(test)]
mod tests {
    //! Unit tests for edit distance.

    use super::{closest, distance};

    #[test]
    fn distance_counts_insertions_deletions_and_substitutions() {
        assert_eq!(distance("spc-7k3f", "spc-7k3f"), 0);
        assert_eq!(distance("spc-7k3f", "spc-7k3g"), 1);
        assert_eq!(distance("spc-7k3", "spc-7k3f"), 1);
        assert_eq!(distance("kitten", "sitting"), 3);
        assert_eq!(distance("", "abc"), 3);
    }

    #[test]
    fn the_closest_are_listed_nearest_first_up_to_the_limit() {
        let handles = ["spc-7k3f", "spc-7k3g", "spc-7kzz", "smp-0000", "spc-9999"];
        assert_eq!(
            closest("spc-7k3x", handles, 3),
            ["spc-7k3f", "spc-7k3g", "spc-7kzz"]
        );
    }

    #[test]
    fn what_is_too_different_is_not_suggested() {
        assert!(closest("spc-zzzz", ["ref-0000", "tsk-1111"], 3).is_empty());
    }
}
