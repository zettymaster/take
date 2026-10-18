use clap::{Parser, ValueEnum};
use take::take;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Unit {
    Amount,
    Mb,
    Gb,
}

#[derive(Debug, Parser)]
struct CliArgs {
    #[arg(short, long, default_value_t = true)]
    sort: bool,
    #[arg(short = 'u', long = "unit", value_enum, default_value_t = Unit::Amount)]
    unit: Unit,
    #[arg(short, long)]
    extension: Option<String>,

    amount: u64,
    source: String,
    destination: String,
}

fn main() {
    let args = CliArgs::parse();

    // TODO
    take(args.source, args.destination, args.amount).unwrap();
}
