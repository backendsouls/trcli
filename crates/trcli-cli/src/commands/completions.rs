//! `trcli completions <shell>` (FR-059): a completion script generated from the same
//! definitions the tool parses with, so it cannot fall out of step with them.

use crate::args::completions::CompletionsArgs;
use crate::cli::command;
use crate::output::Reply;
use crate::render::views::Text;

/// Builds the completion script for the shell that was named.
pub fn run(arguments: &CompletionsArgs) -> Reply {
    let mut script = Vec::new();
    clap_complete::generate(arguments.shell, &mut command(), "trcli", &mut script);
    Reply::new(Text { text: String::from_utf8_lossy(&script).into_owned() })
}

#[cfg(test)]
mod tests {
    //! Unit tests for completion scripts.

    use clap_complete::Shell;

    use super::run;
    use crate::args::completions::CompletionsArgs;

    #[test]
    fn a_script_is_generated_for_every_supported_shell_and_names_the_commands() {
        for shell in [Shell::Bash, Shell::Zsh, Shell::Fish, Shell::PowerShell] {
            let script = run(&CompletionsArgs { shell }).view.to_json()["text"].as_str().unwrap_or_default().to_owned();
            assert!(script.contains("trcli"), "{shell}");
            assert!(script.contains("workspace"), "{shell}");
            assert!(script.contains("--output") || script.contains("output"), "{shell}");
        }
    }
}
