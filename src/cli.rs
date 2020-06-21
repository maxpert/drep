use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "drep", about = "Hot load regex log filter")]
pub struct CliOpts {
    /// Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,

    /// Path to watch
    #[structopt(short = "f", long = "filter", default_value = "filters.regex", parse(from_os_str))]
    pub filters_path: PathBuf,
}