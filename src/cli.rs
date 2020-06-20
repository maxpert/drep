use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(name = "ertugrul", about = "Runtime regex log filter")]
pub struct CliOpts {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    pub debug: bool,

    /// Path to watch
    #[structopt(short = "f", long = "filter", default_value="filters.regex", parse(from_os_str))]
    pub filters_path: PathBuf,
}