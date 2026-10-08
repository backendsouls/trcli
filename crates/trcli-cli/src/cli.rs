//! The whole command line: the foundation's commands, the commands of the record kinds
//! this build has, and what parsing them gives (FR-024, FR-055, FR-056).
//!
//! clap checks the syntax, produces help, and suggests the nearest command or option for
//! a mistyped one. Its errors are turned into the `usage` problem here, so that they look
//! like every other problem, in both output forms.

use clap::error::ErrorKind;
use clap::{ArgMatches, Args, Command, FromArgMatches, Subcommand};
use trcli_application::outcome::{Problem, codes};
use trcli_domain::workspace::FormatVersion;

use crate::args::Commands;
use crate::args::global::GlobalArgs;

/// What is shown under the list of commands by `trcli --help`.
const AFTER_HELP: &str = "\
Run `trcli <command> --help` for a command's options and an example.
Usage guides with worked examples are in docs/usage/.";

/// The description shown by `trcli --help`.
const ABOUT: &str = "TRCLI, The Research CLI: keep the records of your research in one workspace.";

/// The version line: the tool's version and the workspace format it reads and writes
/// (FR-059).
pub fn version() -> String {
    format!(
        "{} (workspace format {})",
        env!("CARGO_PKG_VERSION"),
        FormatVersion::CURRENT
    )
}

/// The command tree of this build.
pub fn command() -> Command {
    let root = Command::new("trcli")
        // The name shown in usage lines is the tool's, not the file's: on Windows the
        // file is `trcli.exe`, and help must read the same on every system.
        .bin_name("trcli")
        .about(ABOUT)
        .version(version())
        .after_help(AFTER_HELP)
        .propagate_version(false)
        .subcommand_required(false)
        .arg_required_else_help(false);
    let root = Commands::augment_subcommands(GlobalArgs::augment_args(root));
    #[cfg(feature = "sample-kind")]
    let root = crate::sample::extend(root);
    root
}

/// What the researcher asked for.
#[derive(Clone, Debug)]
pub enum Parsed {
    /// Nothing: `trcli` alone.
    Nothing,
    /// One of the foundation's commands.
    Foundation(Commands),
    /// A command of a record kind, to be read by the feature that owns it.
    Kind {
        /// The command's noun.
        noun: String,
        /// What clap parsed under that noun.
        matches: ArgMatches,
    },
}

/// One parsed command line.
#[derive(Clone, Debug)]
pub struct Invocation {
    /// The global options.
    pub global: GlobalArgs,
    /// The command.
    pub parsed: Parsed,
    /// The words of the command, without options or values: `["workspace", "show"]`.
    /// This, and nothing else of the command line, is what telemetry records (FR-054).
    pub path: Vec<String>,
}

/// The names of the foundation's own top-level commands.
fn foundation_nouns() -> Vec<String> {
    let commands = Commands::augment_subcommands(Command::new("trcli"));
    commands
        .get_subcommands()
        .map(|command| command.get_name().to_owned())
        .collect()
}

/// Reads what clap matched into an [`Invocation`].
pub fn parse(matches: &ArgMatches) -> Result<Invocation, clap::Error> {
    let global = GlobalArgs::from_arg_matches(matches)?;
    let path = command_path(matches);
    let parsed = match matches.subcommand() {
        None => Parsed::Nothing,
        Some((noun, _)) if foundation_nouns().iter().any(|known| known == noun) => {
            Parsed::Foundation(Commands::from_arg_matches(matches)?)
        }
        Some((noun, inner)) => Parsed::Kind {
            noun: noun.to_owned(),
            matches: inner.clone(),
        },
    };
    Ok(Invocation {
        global,
        parsed,
        path,
    })
}

/// The chain of command names that was matched.
fn command_path(matches: &ArgMatches) -> Vec<String> {
    let mut path = Vec::new();
    let mut current = matches;
    while let Some((name, inner)) = current.subcommand() {
        path.push(name.to_owned());
        current = inner;
    }
    path
}

/// Whether a clap "error" is really a request for help or for the version.
pub fn is_information(error: &clap::Error) -> bool {
    matches!(
        error.kind(),
        ErrorKind::DisplayHelp | ErrorKind::DisplayVersion
    )
}

/// The `usage` problem for a malformed command line: what is wrong, how the command is
/// used, and clap's suggestion when it has one (FR-024).
pub fn usage_problem(error: &clap::Error) -> Problem {
    let text = error.render().to_string();
    let mut lines = text.lines().map(str::trim).filter(|line| !line.is_empty());
    let message = lines
        .next()
        .unwrap_or("the command line is not valid")
        .trim_start_matches("error: ")
        .to_owned();
    // What remains is clap's own explanation: the suggestion ("tip: …") and the usage.
    let details: Vec<String> = lines
        .filter(|line| !line.starts_with("For more information"))
        .map(str::to_owned)
        .collect();
    Problem::new(codes::USAGE, message)
        .with_items(details)
        .with_next_step("run the command with --help to see how it is used")
}

#[cfg(test)]
mod tests {
    //! Unit tests for the command line.

    use super::{Parsed, command, is_information, parse, usage_problem, version};
    use crate::args::Commands;

    /// Parses a command line given as words.
    fn parsed(words: &[&str]) -> Result<super::Invocation, clap::Error> {
        let matches = command().try_get_matches_from(std::iter::once(&"trcli").chain(words))?;
        parse(&matches)
    }

    #[test]
    fn the_command_tree_is_well_formed() {
        command().debug_assert();
    }

    #[test]
    fn usage_names_the_tool_whatever_the_file_is_called() {
        let error = command()
            .try_get_matches_from(["C:\\tools\\trcli.exe", "workspace", "explode"])
            .expect_err("unknown verb");
        let text = error.render().to_string();
        assert!(text.contains("Usage: trcli workspace"), "{text}");
    }

    #[test]
    fn no_arguments_is_not_an_error() {
        let invocation = parsed(&[]).expect("valid");
        assert!(matches!(invocation.parsed, Parsed::Nothing));
        assert!(invocation.path.is_empty());
    }

    #[test]
    fn global_options_are_accepted_before_and_after_the_command() {
        let before = parsed(&["--output", "json", "workspace", "show"]).expect("valid");
        let after =
            parsed(&["workspace", "show", "--output", "json", "-vv", "--yes"]).expect("valid");
        assert_eq!(before.global.output.as_deref(), Some("json"));
        assert_eq!(
            (
                after.global.output.as_deref(),
                after.global.verbose,
                after.global.yes
            ),
            (Some("json"), 2, true)
        );
        assert!(matches!(
            after.parsed,
            Parsed::Foundation(Commands::Workspace(_))
        ));
    }

    #[test]
    fn the_path_holds_command_words_only() {
        let invocation =
            parsed(&["init", "--name", "A secret project", "somewhere"]).expect("valid");
        assert_eq!(invocation.path, ["init"]);
        assert_eq!(
            parsed(&["config", "set", "output.color", "never"])
                .expect("valid")
                .path,
            ["config", "set"]
        );
    }

    #[test]
    fn a_mistyped_command_is_a_usage_problem_with_a_suggestion() {
        let error = parsed(&["worksapce", "show"]).expect_err("unknown command");
        assert!(!is_information(&error));
        let problem = usage_problem(&error);
        assert_eq!(problem.outcome().exit_code(), 2);
        assert!(problem.message.contains("worksapce"), "{}", problem.message);
        assert!(
            format!("{:?}", problem.details).contains("workspace"),
            "{:?}",
            problem.details
        );
    }

    #[test]
    fn a_mistyped_option_is_a_usage_problem_with_a_suggestion() {
        let problem = usage_problem(&parsed(&["init", "--nmae", "x"]).expect_err("unknown option"));
        assert!(
            format!("{:?}", problem.details).contains("--name"),
            "{:?}",
            problem.details
        );
    }

    #[test]
    fn help_and_version_are_information_not_errors() {
        assert!(is_information(&parsed(&["--help"]).expect_err("help")));
        assert!(is_information(
            &parsed(&["--version"]).expect_err("version")
        ));
        assert!(version().contains("workspace format 1"));
    }

    #[test]
    fn audit_has_no_way_to_add_edit_or_remove_an_entry() {
        for verb in ["add", "edit", "rm"] {
            assert!(
                parsed(&["audit", verb]).is_err(),
                "audit {verb} must not exist"
            );
        }
    }
}
