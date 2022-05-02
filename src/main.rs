mod args;

use args::DateDiffArgs;
use clap::Parser;

fn main() {
    let cli = DateDiffArgs::parse();
}
