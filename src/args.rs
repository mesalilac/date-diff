use clap::Parser;

/// Calculate the difference between two dates
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct DateDiffArgs {
    /// Date formats: yyyy-mm-dd or yyyy/mm/dd
    pub start_date: String,
    /// Date formats: yyyy-mm-dd or yyyy/mm/dd
    pub end_date: String,

    #[clap(long, short)]
    pub seconds: bool,

    #[clap(long, short = 'M')]
    pub minutes: bool,

    #[clap(long, short)]
    pub hours: bool,

    #[clap(long, short)]
    pub days: bool,

    #[clap(long, short)]
    pub weeks: bool,

    #[clap(long, short)]
    pub months: bool,

    #[clap(long, short)]
    pub years: bool,
}
