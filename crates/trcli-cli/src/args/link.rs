//! `trcli link …` and `trcli tag list`, as in `contracts/cli-link.md`. Tagging and noting
//! one record are verbs of the record's own noun; see `shared_verbs`.

use clap::{Args, Subcommand};

/// The example shown by `trcli link --help`.
pub const LINK_EXAMPLE: &str = "\
Example:
  trcli link add rq-4m2p ref-7k3f --relation addresses
  trcli link list rq-4m2p

Guide: docs/usage/records.md";

/// The example shown by `trcli tag --help`.
pub const TAG_EXAMPLE: &str = "\
Example:
  trcli tag list
  trcli tag list --kind reference

Guide: docs/usage/records.md";

/// The verbs of `trcli link`.
#[derive(Clone, Debug, Subcommand)]
pub enum LinkCommand {
    /// Link two records of any kind; the link is shown from both
    #[command(after_long_help = "Example:\n  trcli link add rq-4m2p ref-7k3f --relation addresses")]
    Add(LinkArgs),

    /// Remove the link between two records
    #[command(after_long_help = "Example:\n  trcli link rm rq-4m2p ref-7k3f --relation addresses")]
    Rm(LinkArgs),

    /// List the links of a record, in both directions
    #[command(after_long_help = "Example:\n  trcli link list rq-4m2p")]
    List(LinkListArgs),
}

/// The arguments of `trcli link add` and `trcli link rm`.
#[derive(Clone, Debug, Args)]
pub struct LinkArgs {
    /// Short name of the first record, or a unique beginning of it
    #[arg(value_name = "REF")]
    pub one: String,

    /// Short name of the second record, or a unique beginning of it
    #[arg(value_name = "OTHER_REF")]
    pub other: String,

    /// How the two relate (1 to 50 characters) [default: related]
    #[arg(long, value_name = "TEXT")]
    pub relation: Option<String>,
}

/// The argument of `trcli link list`.
#[derive(Clone, Debug, Args)]
pub struct LinkListArgs {
    /// Short name of the record, or a unique beginning of it
    #[arg(value_name = "REF")]
    pub reference: String,
}

/// The verbs of `trcli tag`.
#[derive(Clone, Debug, Subcommand)]
pub enum TagCommand {
    /// List every tag with the number of records carrying it
    #[command(after_long_help = "Example:\n  trcli tag list\n  trcli tag list --kind reference")]
    List(TagListArgs),
}

/// The options of `trcli tag list`.
#[derive(Clone, Debug, Args)]
pub struct TagListArgs {
    /// Only count records of this kind
    #[arg(long, value_name = "KIND")]
    pub kind: Option<String>,
}
