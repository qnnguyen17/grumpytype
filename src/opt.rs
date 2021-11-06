use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct CliOptions {
    #[structopt(
        long,
        default_value = "google-10000-english-usa.txt",
        parse(from_os_str)
    )]
    pub dictionary_path: PathBuf,

    #[structopt(short, long, default_value = "5")]
    pub display_lines: usize,

    #[structopt(long, default_value = "3")]
    pub min_word_len: usize,

    #[structopt(long, default_value = "7")]
    pub max_word_len: usize,

    #[structopt(short, long, default_value = "15")]
    pub time_limit: u64,
}
