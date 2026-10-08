//! The generic steps every feature file is written with.
//!
//! A scenario runs commands with "When I run …" and then says what must be true of the
//! exit code, of the two output streams, and of the workspace. In a command, `'…'` groups
//! a word with spaces, `<name>` stands for a short name remembered earlier, and `{home}`
//! for the scenario's own directory.

use std::time::{Duration, Instant};

use cucumber::{given, then, when};

use crate::world::{TrcliWorld, portable, split};

/// The arguments of a command line that starts with `trcli`.
fn arguments(world: &TrcliWorld, line: &str) -> Vec<String> {
    let mut words = split(&world.expand(line));
    assert_eq!(
        words.first().map(String::as_str),
        Some("trcli"),
        "a command starts with `trcli`: {line}"
    );
    words.remove(0);
    words
}

// ---------------------------------------------------------------- Given

/// Step: `a workspace named {string}`.
#[given(expr = "a workspace named {string}")]
async fn a_workspace_named(world: &mut TrcliWorld, name: String) {
    world
        .run(&["init".to_owned(), "--name".to_owned(), name])
        .await;
    assert_eq!(
        world.last.code, 0,
        "creating the workspace failed: {}",
        world.last.stderr
    );
}

/// Step: `I am in the directory {string}`.
#[given(expr = "I am in the directory {string}")]
#[when(expr = "I go to the directory {string}")]
async fn i_am_in_the_directory(world: &mut TrcliWorld, relative: String) {
    world.directory = world.path(&relative);
    std::fs::create_dir_all(&world.directory).expect("the directory");
}

/// Step: `the environment variable {word} is {string}`.
#[given(expr = "the environment variable {word} is {string}")]
async fn the_environment_variable_is(world: &mut TrcliWorld, name: String, value: String) {
    let value = world.expand(&value);
    world.environment.insert(name, value);
}

/// Step: `standard input is not a terminal`.
#[given("standard input is not a terminal")]
async fn standard_input_is_not_a_terminal(_world: &mut TrcliWorld) {
    // Always so here: every command is run with no standard input at all. The step says
    // it in the scenario, where it matters to the reader.
}

/// Step: `the file {string} contains {string}`.
#[given(expr = "the file {string} contains {string}")]
async fn the_file_contains(world: &mut TrcliWorld, relative: String, content: String) {
    let path = world.directory.join(relative);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("the directory");
    }
    std::fs::write(path, content.replace("\\n", "\n")).expect("the file");
}

/// Step: `my settings file contains {string}`.
#[given(expr = "my settings file contains {string}")]
async fn my_settings_file_contains(world: &mut TrcliWorld, content: String) {
    // The same directory serves as the settings directory of every system; see the world.
    for directory in [
        "user-settings/trcli",
        "user-settings/.config/trcli",
        "user-settings/Library/Application Support/trcli",
    ] {
        let directory = world.path(directory);
        std::fs::create_dir_all(&directory).expect("the settings directory");
        std::fs::write(directory.join("config.toml"), content.replace("\\n", "\n"))
            .expect("the settings file");
    }
}

/// Step: `the workspace is stored in format {int}`.
#[given(expr = "the workspace is stored in format {int}")]
async fn the_workspace_is_stored_in_format(world: &mut TrcliWorld, format: i64) {
    world
        .execute(&format!("UPDATE workspace SET format_version = {format}"))
        .await;
}

/// Step: `the workspace's database has been damaged`.
#[given("the workspace's database has been damaged")]
async fn the_database_has_been_damaged(world: &mut TrcliWorld) {
    let database = world.database().expect("a workspace database");
    std::fs::write(database, b"this is no longer a database").expect("the file");
}

/// Step: `the directory {string} cannot be written to`.
#[given(expr = "the directory {string} cannot be written to")]
async fn the_directory_cannot_be_written_to(world: &mut TrcliWorld, relative: String) {
    let directory = world.path(&relative);
    std::fs::create_dir_all(&directory).expect("the directory");
    set_read_only(&directory, true);
}

/// Makes a directory read-only, or writable again.
#[cfg(unix)]
fn set_read_only(directory: &std::path::Path, read_only: bool) {
    use std::os::unix::fs::PermissionsExt;
    let mode = if read_only { 0o555 } else { 0o755 };
    std::fs::set_permissions(directory, std::fs::Permissions::from_mode(mode))
        .expect("permissions");
}

/// Scenarios that need this are tagged `@unix` and do not run elsewhere.
#[cfg(not(unix))]
fn set_read_only(_directory: &std::path::Path, _read_only: bool) {}

// ---------------------------------------------------------------- When

/// Step: `I run {string}`.
#[when(expr = "I run {string}")]
#[given(expr = "I ran {string}")]
async fn i_run(world: &mut TrcliWorld, line: String) {
    let arguments = arguments(world, &line);
    world.run(&arguments).await;
}

/// Step: `I run {string} and remember the record as {string}`.
#[when(expr = "I run {string} and remember the record as {string}")]
#[given(expr = "I ran {string} and remember the record as {string}")]
async fn i_run_and_remember(world: &mut TrcliWorld, line: String, name: String) {
    let arguments = arguments(world, &line);
    world.run(&arguments).await;
    assert_eq!(world.last.code, 0, "`{line}` failed: {}", world.last.stderr);
    let handle = first_handle(&world.last.stdout)
        .unwrap_or_else(|| panic!("no short name in: {}", world.last.stdout));
    world.handles.insert(name, handle);
}

/// The first word of a text that has the shape of a short name: a prefix of 2 to 4
/// letters, a hyphen, and a code of 4 to 12 letters and digits.
fn first_handle(text: &str) -> Option<String> {
    text.split(|character: char| !(character.is_ascii_alphanumeric() || character == '-'))
        .find_map(|word| {
            let (prefix, code) = word.split_once('-')?;
            let is_handle = (2..=4).contains(&prefix.len())
                && prefix
                    .chars()
                    .all(|character| character.is_ascii_lowercase())
                && (4..=12).contains(&code.len())
                && code
                    .chars()
                    .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit());
            is_handle.then(|| word.to_owned())
        })
}

/// Step: `I run {string} and stop it with {word} once it holds the workspace`.
#[when(expr = "I run {string} and stop it with {word} once it holds the workspace")]
async fn i_run_and_stop_it(world: &mut TrcliWorld, line: String, signal: String) {
    let arguments = arguments(world, &line);
    world.before = world.snapshot().await;
    let mut command = world.trcli(&arguments);
    let child = command
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("trcli starts");
    world.wait_until_held().await;
    let started = Instant::now();
    let sent = std::process::Command::new("kill")
        .arg(format!("-{signal}"))
        .arg(child.id().to_string())
        .status();
    assert!(
        sent.is_ok_and(|status| status.success()),
        "the signal could not be sent"
    );
    world.last = child.wait_with_output().expect("trcli ends").into();
    // Stopping must be prompt: within a second (User Story 4, scenario 8).
    assert!(
        started.elapsed() < Duration::from_secs(1),
        "the command took {:?} to stop",
        started.elapsed()
    );
}

/// Step: `I run {string} while {string} holds the workspace`.
#[when(expr = "I run {string} while {string} holds the workspace")]
async fn i_run_while_another_holds_the_workspace(
    world: &mut TrcliWorld,
    line: String,
    other: String,
) {
    let holding = arguments(world, &other);
    let mut holder = world
        .trcli(&holding)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("trcli starts");
    world.wait_until_held().await;
    let arguments = arguments(world, &line);
    world.run(&arguments).await;
    let ended = holder.wait().expect("the first command ends");
    assert!(
        ended.success(),
        "the command that held the workspace failed"
    );
}

/// Step: `audit entry {int} is altered outside the tool`.
#[when(expr = "audit entry {int} is altered outside the tool")]
async fn an_audit_entry_is_altered(world: &mut TrcliWorld, sequence: i64) {
    world
        .execute(&format!(
            "UPDATE audit_entry SET actor = 'someone else' WHERE sequence = {sequence}"
        ))
        .await;
}

/// Step: `the last audit entry is removed outside the tool`.
#[when("the last audit entry is removed outside the tool")]
async fn the_last_audit_entry_is_removed(world: &mut TrcliWorld) {
    world
        .execute("DELETE FROM audit_entry WHERE sequence = (SELECT MAX(sequence) FROM audit_entry)")
        .await;
}

/// Step: `the directory {string} is renamed to {string}`.
#[when(expr = "the directory {string} is renamed to {string}")]
async fn the_directory_is_renamed(world: &mut TrcliWorld, from: String, to: String) {
    let (from, to) = (world.path(&from), world.path(&to));
    let relative = world
        .directory
        .strip_prefix(&from)
        .map(std::path::Path::to_path_buf)
        .ok();
    std::fs::rename(&from, &to).expect("the rename");
    if let Some(relative) = relative {
        world.directory = to.join(relative);
    }
}

// ---------------------------------------------------------------- Then

/// Step: `the exit code is {int}`.
#[then(expr = "the exit code is {int}")]
async fn the_exit_code_is(world: &mut TrcliWorld, code: i32) {
    assert_eq!(
        world.last.code, code,
        "stdout: {}\nstderr: {}",
        world.last.stdout, world.last.stderr
    );
}

/// Step: `stdout contains {string}`.
#[then(expr = "stdout contains {string}")]
async fn stdout_contains(world: &mut TrcliWorld, text: String) {
    let text = portable(&world.expand(&text));
    assert!(
        portable(&world.last.stdout).contains(&text),
        "`{text}` is not in stdout:\n{}",
        world.last.stdout
    );
}

/// Step: `stdout does not contain {string}`.
#[then(expr = "stdout does not contain {string}")]
async fn stdout_does_not_contain(world: &mut TrcliWorld, text: String) {
    let text = portable(&world.expand(&text));
    assert!(
        !portable(&world.last.stdout).contains(&text),
        "`{text}` is in stdout:\n{}",
        world.last.stdout
    );
}

/// Step: `stderr contains {string}`.
#[then(expr = "stderr contains {string}")]
async fn stderr_contains(world: &mut TrcliWorld, text: String) {
    let text = portable(&world.expand(&text));
    assert!(
        portable(&world.last.stderr).contains(&text),
        "`{text}` is not in stderr:\n{}",
        world.last.stderr
    );
}

/// Step: `stderr does not contain {string}`.
#[then(expr = "stderr does not contain {string}")]
async fn stderr_does_not_contain(world: &mut TrcliWorld, text: String) {
    let text = portable(&world.expand(&text));
    assert!(
        !portable(&world.last.stderr).contains(&text),
        "`{text}` is in stderr:\n{}",
        world.last.stderr
    );
}

/// Step: `stdout is empty`.
#[then("stdout is empty")]
async fn stdout_is_empty(world: &mut TrcliWorld) {
    assert!(
        world.last.stdout.is_empty(),
        "stdout is not empty:\n{}",
        world.last.stdout
    );
}

/// Step: `stderr is empty`.
#[then("stderr is empty")]
async fn stderr_is_empty(world: &mut TrcliWorld) {
    assert!(
        world.last.stderr.is_empty(),
        "stderr is not empty:\n{}",
        world.last.stderr
    );
}

/// The value at a dotted path of the JSON document on standard output, as text.
fn json_at(world: &TrcliWorld, path: &str) -> String {
    let document: serde_json::Value = serde_json::from_str(&world.last.stdout)
        .unwrap_or_else(|error| panic!("stdout is not JSON ({error}):\n{}", world.last.stdout));
    let mut value = &document;
    for segment in path.split('.') {
        value = match segment.parse::<usize>() {
            Ok(index) => value.get(index),
            Err(_) => value.get(segment),
        }
        .unwrap_or_else(|| panic!("`{path}` is not in:\n{}", world.last.stdout));
    }
    match value {
        serde_json::Value::String(text) => text.clone(),
        other => other.to_string(),
    }
}

/// Step: `stdout is JSON where {string} is {string}`.
#[then(expr = "stdout is JSON where {string} is {string}")]
async fn stdout_is_json_where(world: &mut TrcliWorld, path: String, expected: String) {
    let expected = world.expand(&expected);
    assert_eq!(
        json_at(world, &path),
        expected,
        "at `{path}` in:\n{}",
        world.last.stdout
    );
}

/// Step: `stdout is JSON where {string} contains {string}`.
#[then(expr = "stdout is JSON where {string} contains {string}")]
async fn stdout_is_json_where_contains(world: &mut TrcliWorld, path: String, expected: String) {
    let expected = world.expand(&expected);
    let found = json_at(world, &path);
    assert!(
        found.contains(&expected),
        "`{expected}` is not in `{found}` (at `{path}`)"
    );
}

/// Step: `stdout is one JSON document`.
#[then("stdout is one JSON document")]
async fn stdout_is_one_json_document(world: &mut TrcliWorld) {
    let parsed = serde_json::from_str::<serde_json::Value>(&world.last.stdout);
    assert!(
        parsed.is_ok(),
        "stdout is not one JSON document:\n{}",
        world.last.stdout
    );
    assert!(world.last.stdout.ends_with('\n') && !world.last.stdout.ends_with("\n\n"));
}

/// Step: `the output has no colour codes`.
#[then("the output has no colour codes")]
async fn the_output_has_no_colour_codes(world: &mut TrcliWorld) {
    assert!(
        !world.last.stdout.contains('\u{1b}'),
        "stdout has colour codes:\n{:?}",
        world.last.stdout
    );
    assert!(
        !world.last.stderr.contains('\u{1b}'),
        "stderr has colour codes:\n{:?}",
        world.last.stderr
    );
}

/// Step: `stdout has colour codes`.
#[then("stdout has colour codes")]
async fn stdout_has_colour_codes(world: &mut TrcliWorld) {
    assert!(
        world.last.stdout.contains("\u{1b}["),
        "stdout has no colour codes:\n{:?}",
        world.last.stdout
    );
}

/// Step: `no line of stdout is wider than {int} columns`.
#[then(expr = "no line of stdout is wider than {int} columns")]
async fn no_line_is_wider_than(world: &mut TrcliWorld, columns: usize) {
    for line in world.last.stdout.lines() {
        assert!(
            line.chars().count() <= columns,
            "this line is wider than {columns} columns:\n{line}"
        );
    }
}

/// Step: `nothing was changed`.
#[then("nothing was changed")]
async fn nothing_was_changed(world: &mut TrcliWorld) {
    let after = world.snapshot().await;
    assert_eq!(
        world.before.exists, after.exists,
        "a workspace appeared or disappeared"
    );
    assert_eq!(
        world.before.audit_entries, after.audit_entries,
        "the number of audit entries changed"
    );
    assert_eq!(
        world.before.rows, after.rows,
        "the workspace's records changed"
    );
    assert_eq!(
        world.before.settings, after.settings,
        "the workspace's settings file changed"
    );
}

/// Step: `the workspace has {int} audit entries`.
#[then(expr = "the workspace has {int} audit entries")]
async fn the_workspace_has_audit_entries(world: &mut TrcliWorld, expected: usize) {
    assert_eq!(world.snapshot().await.audit_entries, expected);
}

/// Step: `the file {string} exists`.
#[then(expr = "the file {string} exists")]
async fn the_file_exists(world: &mut TrcliWorld, relative: String) {
    let path = world.directory.join(world.expand(&relative));
    assert!(path.exists(), "{} does not exist", path.display());
}

/// Step: `the file {string} does not exist`.
#[then(expr = "the file {string} does not exist")]
async fn the_file_does_not_exist(world: &mut TrcliWorld, relative: String) {
    let path = world.directory.join(world.expand(&relative));
    assert!(!path.exists(), "{} exists", path.display());
}

/// Step: `the file {string} contains {string}`.
#[then(expr = "the file {string} contains {string}")]
async fn the_file_has(world: &mut TrcliWorld, relative: String, text: String) {
    let path = world.directory.join(world.expand(&relative));
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    let text = world.expand(&text);
    assert!(
        content.contains(&text),
        "`{text}` is not in {}:\n{content}",
        path.display()
    );
}

/// Step: `the directory {string} holds {int} file(s)`.
#[then(expr = "the directory {string} holds {int} file(s)")]
async fn the_directory_holds_files(world: &mut TrcliWorld, relative: String, expected: usize) {
    let directory = world.directory.join(relative);
    let count = std::fs::read_dir(&directory)
        .map(Iterator::count)
        .unwrap_or(0);
    assert_eq!(count, expected, "in {}", directory.display());
}

/// Step: `the directory {string} can be written to again`.
#[then(expr = "the directory {string} can be written to again")]
async fn the_directory_can_be_written_to_again(world: &mut TrcliWorld, relative: String) {
    set_read_only(&world.path(&relative), false);
}

/// Step: `stdout lists {string} before {string}`.
#[then(expr = "stdout lists {string} before {string}")]
async fn stdout_lists_before(world: &mut TrcliWorld, first: String, second: String) {
    let (first, second) = (world.expand(&first), world.expand(&second));
    let position = |text: &str| {
        world
            .last
            .stdout
            .find(text)
            .unwrap_or_else(|| panic!("`{text}` is not in stdout:\n{}", world.last.stdout))
    };
    assert!(
        position(&first) < position(&second),
        "`{first}` does not come before `{second}`:\n{}",
        world.last.stdout
    );
}

/// The options (`--word`) named in a help text.
fn options_in(help: &str) -> std::collections::BTreeSet<String> {
    help.split(|character: char| !(character.is_ascii_alphanumeric() || character == '-'))
        .filter(|word| word.starts_with("--") && word.len() > 2)
        .map(str::to_owned)
        .collect()
}

/// Step: `the options of {string} and {string} are the same`.
#[then(expr = "the options of {string} and {string} are the same")]
async fn the_options_are_the_same(world: &mut TrcliWorld, one: String, other: String) {
    let mut help = Vec::new();
    for line in [&one, &other] {
        let mut arguments = arguments(world, line);
        arguments.push("--help".to_owned());
        world.run(&arguments).await;
        assert_eq!(
            world.last.code, 0,
            "`{line} --help` failed: {}",
            world.last.stderr
        );
        help.push(options_in(&world.last.stdout));
    }
    assert!(!help[0].is_empty(), "`{one} --help` names no option");
    assert_eq!(
        help[0], help[1],
        "`{one}` and `{other}` do not take the same options"
    );
}
