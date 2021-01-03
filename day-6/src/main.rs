use anyhow::{Context, Result};
use std::path::PathBuf;
use structopt::StructOpt;
use petgraph::graphmap::DiGraphMap;
use petgraph::visit::Bfs;
use petgraph::visit::Walker;

#[derive(StructOpt)]
struct Cli {
    /// Sets the input file to use
    #[structopt(parse(from_os_str))]
    input_file: PathBuf,

    /// Should we sum the number of questions answered by everyone in each group? If this flag is
    /// not specified, we will sum the number of questions answered by anyone in each group instead
    #[structopt(short, long)]
    everyone: bool,
}

fn main() -> Result<()> {
    let args: Cli = Cli::from_args();

    let groups = std::fs::read_to_string(&args.input_file)
        .with_context(|| format!("Failed to read file {}", &args.input_file.display()))?;

    println!(
        "{}",
        if !args.everyone {
            day_6::sum_unique_questions_answered_per_group(groups)
        } else {
            day_6::sum_questions_answered_by_everyone_per_group(groups)
        }
    );

    let mut graph = DiGraphMap::<i32, ()>::new();
    graph.add_edge(-1, 2, ());
    graph.add_edge(-1, 3, ());
    graph.add_edge(-1, 5, ());
    graph.add_edge(5, 8, ());

    let mut bfs = Bfs::new(&graph, -1);
    let results: Vec<_> = bfs.iter(&graph).collect();
    println!("{:?}", results);

    Ok(())
}
