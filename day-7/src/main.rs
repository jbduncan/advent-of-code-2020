use anyhow::{Context, Result};
use day_7::BagRules;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// Sets the input file to use
    #[structopt(parse(from_os_str))]
    input_file: PathBuf,
}

fn main() -> Result<()> {
    let args: Cli = Cli::from_args();

    let input = std::fs::read_to_string(&args.input_file)
        .with_context(|| format!("Failed to read file {}", &args.input_file.display()))?;

    let bag_rules = BagRules::from(&input)? /*input.parse::<BagRules>()?*/;
    println!(
        "{}",
        bag_rules
            .bags_eventually_containing("shiny gold bag")
            .count()
    );

    Ok(())
}
