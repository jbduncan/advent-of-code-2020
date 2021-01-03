use assert_cmd::Command;
use indoc::indoc;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

#[cfg(test)]
mod when_not_given_a_map {
    use super::*;

    #[test]
    fn then_returns_help_message() -> Result<(), Box<dyn std::error::Error>> {
        let mut command = Command::cargo_bin("day-3")?;

        command
            .assert()
            .failure()
            .stderr(predicate::function(|x: &str| {
                x.to_ascii_lowercase().contains("usage")
            }));

        Ok(())
    }
}

#[cfg(test)]
mod given_map_with_lots_of_trees {
    use super::*;

    mod when_sliding_down_map {
        use super::*;

        #[test]
        fn then_we_encounter_some_trees() -> Result<(), Box<dyn std::error::Error>> {
            let mut actual_map_file = NamedTempFile::new()?;
            writeln!(
                actual_map_file,
                indoc! { "
                    ..##.......
                    #...#...#..
                    .#....#..#.
                    ..#.#...#.#
                    .#...##..#.
                    ..#.##.....
                    .#.#.#....#
                    .#........#
                    #.##...#...
                    #...##....#
                    .#..#...#.#
                " }
            )?;
            let expected_trees_encountered = "7\n";

            let mut command = Command::cargo_bin("day-3")?;
            command.arg(actual_map_file.path());

            command
                .assert()
                .success()
                .stdout(predicate::eq(expected_trees_encountered));
            Ok(())
        }
    }

    mod when_sliding_down_map_in_all_directions {
        use super::*;

        #[test]
        fn then_we_encounter_some_trees() -> Result<(), Box<dyn std::error::Error>> {
            let mut actual_map_file = NamedTempFile::new()?;
            writeln!(
                actual_map_file,
                indoc! { "
                    ..##.......
                    #...#...#..
                    .#....#..#.
                    ..#.#...#.#
                    .#...##..#.
                    ..#.##.....
                    .#.#.#....#
                    .#........#
                    #.##...#...
                    #...##....#
                    .#..#...#.#
                " }
            )?;
            let expected_result = indoc! { "
                Right 1, down 1: 2
                Right 3, down 1: 7
                Right 5, down 1: 3
                Right 7, down 1: 4
                Right 1, down 2: 2
                Product: 336
            " };

            let mut command = Command::cargo_bin("day-3")?;
            command.arg(actual_map_file.path()).arg("--all-directions");

            command
                .assert()
                .success()
                .stdout(predicate::eq(expected_result));
            Ok(())
        }
    }
}
