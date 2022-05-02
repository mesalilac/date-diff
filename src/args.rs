use clap::Parser;

/// Calculate the difference between two dates
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct DateDiffArgs {
    pub start_date: String,
    pub end_date: String,

    #[clap(long, short)]
    pub second: bool,

    #[clap(long, short)]
    pub minute: bool,

    #[clap(long, short)]
    pub hour: bool,

    #[clap(long, short)]
    pub day: bool,

    #[clap(long, short)]
    pub week: bool,

    #[clap(long, short = 'M')]
    pub month: bool,

    #[clap(long, short)]
    pub year: bool,
}
