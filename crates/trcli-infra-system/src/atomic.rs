//! Writing a file so that it is whole or untouched (FR-066).
//!
//! The new content is written to a temporary name beside the file and then renamed over
//! it. A rename within one directory is atomic on every supported system, so a reader —
//! or the next command after a crash — sees either the old file or the new one, never a
//! part of one.

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// The temporary name used while `path` is being written.
fn temporary_name(path: &Path) -> PathBuf {
    let mut name = path.file_name().unwrap_or_default().to_os_string();
    name.push(format!(".tmp-{}", std::process::id()));
    path.with_file_name(name)
}

/// Replaces the content of `path` with `content`, atomically.
pub fn write_atomically(path: &Path, content: &[u8]) -> io::Result<()> {
    write_atomically_with(path, content, |from, to| fs::rename(from, to))
}

/// As [`write_atomically`], with the final step supplied by the caller. Tests use this to
/// make the rename fail and check that the original file is intact.
pub fn write_atomically_with(
    path: &Path,
    content: &[u8],
    rename: impl FnOnce(&Path, &Path) -> io::Result<()>,
) -> io::Result<()> {
    let temporary = temporary_name(path);
    let written = write_and_sync(&temporary, content).and_then(|()| rename(&temporary, path));
    if written.is_err() {
        // Nothing of the attempt may be left behind; the original was never touched.
        let _ = fs::remove_file(&temporary);
    }
    written
}

/// Writes a new file and makes sure it has reached the disk before it is renamed.
fn write_and_sync(path: &Path, content: &[u8]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(content)?;
    file.sync_all()
}

/// Copies `from` to `to` so that `to` is never seen half-copied.
pub fn copy_atomically(from: &Path, to: &Path) -> io::Result<()> {
    copy_atomically_with(from, to, |temporary, target| fs::rename(temporary, target))
}

/// As [`copy_atomically`], with the final step supplied by the caller.
pub fn copy_atomically_with(
    from: &Path,
    to: &Path,
    rename: impl FnOnce(&Path, &Path) -> io::Result<()>,
) -> io::Result<()> {
    let temporary = temporary_name(to);
    let copied = fs::copy(from, &temporary)
        .map(|_| ())
        .and_then(|()| rename(&temporary, to));
    if copied.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    copied
}

#[cfg(test)]
mod tests {
    //! Unit tests for atomic writing.

    use std::fs;
    use std::io;

    use super::{copy_atomically_with, write_atomically, write_atomically_with};

    #[test]
    fn the_content_is_replaced_and_no_temporary_file_remains() {
        let directory = tempfile::tempdir().expect("a temporary directory");
        let path = directory.path().join("config.toml");
        write_atomically(&path, b"first").expect("written");
        write_atomically(&path, b"second").expect("written");
        assert_eq!(fs::read(&path).expect("read"), b"second");
        assert_eq!(fs::read_dir(directory.path()).expect("list").count(), 1);
    }

    #[test]
    fn a_failure_at_the_rename_leaves_the_original_intact() {
        let directory = tempfile::tempdir().expect("a temporary directory");
        let path = directory.path().join("audit.head");
        write_atomically(&path, b"original").expect("written");
        let failed = write_atomically_with(&path, b"new content", |_, _| {
            Err(io::Error::other("interrupted"))
        });
        assert!(failed.is_err());
        assert_eq!(fs::read(&path).expect("read"), b"original");
        assert_eq!(
            fs::read_dir(directory.path()).expect("list").count(),
            1,
            "the temporary file is removed"
        );
    }

    #[test]
    fn a_failed_copy_leaves_no_half_made_copy() {
        let directory = tempfile::tempdir().expect("a temporary directory");
        let (from, to) = (
            directory.path().join("trcli.db"),
            directory.path().join("copy.db"),
        );
        fs::write(&from, b"database").expect("written");
        let failed = copy_atomically_with(&from, &to, |_, _| Err(io::Error::other("interrupted")));
        assert!(failed.is_err());
        assert!(!to.exists());
        assert_eq!(fs::read_dir(directory.path()).expect("list").count(), 1);
    }
}
