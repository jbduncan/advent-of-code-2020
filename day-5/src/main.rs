use anyhow::{Context, Result};
use clap::{clap_app, crate_authors, crate_description, crate_name, crate_version};
use day_5::{largest_seat_id, my_seat_id};
use std::fs::read_to_string;
use std::path::PathBuf;

fn main() -> Result<()> {
    let app = clap_app!(app =>
        (name: crate_name!())
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg INPUT: +required "Sets the input file to use")
        (@arg FIND_HIGHEST_SEAT_ID:
            --("highest-seat-id")
            "Should we find the highest seat ID? If this is not provided, \
             we'll find your seat ID instead.")
    );
    let matches = app.get_matches();

    let file_path = matches.value_of("INPUT").map(PathBuf::from).unwrap();
    let file_contents = read_to_string(file_path)?;
    let boarding_passes: Vec<&str> = file_contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();

    let find_highest_seat_id = matches.is_present("FIND_HIGHEST_SEAT_ID");
    if find_highest_seat_id {
        println!(
            "{}",
            largest_seat_id(&boarding_passes)
                .with_context(|| format!("Failed to find the largest seat ID."))?
        );
    } else {
        println!(
            "{}",
            my_seat_id(&boarding_passes)
                .with_context(|| format!("Failed to find your seat ID."))?
        );
    }

    Ok(())
}
