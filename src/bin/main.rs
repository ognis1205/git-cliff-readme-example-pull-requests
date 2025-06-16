use clap::Parser;

use git_cliff_readme_example::core::hello;

#[derive(Parser)]
struct Cli {
    name: String,
}

pub fn main() {
    let cli = Cli::parse();
    hello(&cli.name)
}
