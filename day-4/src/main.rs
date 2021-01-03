use anyhow::Result;
use clap::{clap_app, crate_authors, crate_description, crate_name, crate_version};
use day_4::{valid_passports, valid_passports_and_fields};
use std::fs::read_to_string;
use std::path::PathBuf;

fn main() -> Result<()> {
    let app = clap_app!(app =>
        (name: crate_name!())
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg INPUT: +required "Sets the input file to use")
        (@arg VERIFY_FIELDS:
            --("verify-fields")
            "Should we also verify that every password field follows the rules from part 2?")
    );
    let matches = app.get_matches();

    let file_path = matches.value_of("INPUT").map(PathBuf::from).unwrap();
    let file_contents = read_to_string(file_path)?;

    let verify_fields = matches.is_present("VERIFY_FIELDS");
    println!(
        "{}",
        if verify_fields {
            valid_passports_and_fields(&file_contents)
        } else {
            valid_passports(&file_contents)
        }
    );

    Ok(())
}
