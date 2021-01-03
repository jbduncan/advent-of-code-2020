use anyhow::Result;
use clap::{clap_app, crate_authors, crate_description, crate_name, crate_version};
use day_3::{trees_encountered, trees_encountered_in_direction, Direction};
use std::fs::read_to_string;
use std::path::PathBuf;

fn main() -> Result<()> {
    let matches = clap_app!(app =>
        (name: crate_name!())
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg INPUT: +required "Sets the input file to use")
        (@arg ALL_DIRECTIONS:
            --("all-directions")
            "Should we go in all directions, not just right 3 and down 1?")
    )
    .get_matches();

    let map_file_path = matches.value_of("INPUT").map(PathBuf::from).unwrap();
    let map = read_to_string(map_file_path)?;
    let sanitized_map = map.trim();

    if matches.is_present("ALL_DIRECTIONS") {
        let right_1_down_1 =
            trees_encountered_in_direction(&sanitized_map, Direction { right: 1, down: 1 })?;
        println!("Right 1, down 1: {}", right_1_down_1);
        let right_3_down_1 =
            trees_encountered_in_direction(&sanitized_map, Direction { right: 3, down: 1 })?;
        println!("Right 3, down 1: {}", right_3_down_1);
        let right_5_down_1 =
            trees_encountered_in_direction(&sanitized_map, Direction { right: 5, down: 1 })?;
        println!("Right 5, down 1: {}", right_5_down_1);
        let right_7_down_1 =
            trees_encountered_in_direction(&sanitized_map, Direction { right: 7, down: 1 })?;
        println!("Right 7, down 1: {}", right_7_down_1);
        let right_1_down_2 =
            trees_encountered_in_direction(&sanitized_map, Direction { right: 1, down: 2 })?;
        println!("Right 1, down 2: {}", right_1_down_2);
        let product =
            right_1_down_1 * right_3_down_1 * right_5_down_1 * right_7_down_1 * right_1_down_2;
        println!("Product: {}", product);
    } else {
        println!("{}", trees_encountered(&sanitized_map)?);
    }

    Ok(())
}
