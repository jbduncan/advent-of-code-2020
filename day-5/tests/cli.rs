use assert_cmd::Command;
use indoc::indoc;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

#[cfg(test)]
mod cli_tests {
    use super::*;

    mod given_no_cli_args {
        use super::*;

        mod when_using_cli_app {
            use super::*;

            #[test]
            fn then_it_outputs_error_message() -> Result<(), Box<dyn std::error::Error>> {
                let mut command = Command::cargo_bin("day-5")?;

                command
                    .assert()
                    .failure()
                    .stdout(predicate::str::is_empty())
                    .stderr(predicate::str::is_match("<INPUT>").unwrap());

                Ok(())
            }
        }
    }

    mod given_example_boarding_passes_from_part_1 {
        use super::*;

        mod when_searching_for_boarding_pass_with_largest_seat_id {
            use super::*;

            #[test]
            fn then_it_outputs_a_seat_id_of_820() -> Result<(), Box<dyn std::error::Error>> {
                let mut input_file = NamedTempFile::new()?;
                writeln!(
                    input_file,
                    indoc! { "
                        FBFBBFFRLR
                        BFFFBBFRRR
                        FFFBBBFRRR
                        BBFFBBFRLL
                    " }
                )?;

                let mut command = Command::cargo_bin("day-5")?;
                command.arg(input_file.path()).arg("--highest-seat-id");

                command
                    .assert()
                    .success()
                    .stdout(predicate::eq("820\n"))
                    .stderr(predicate::str::is_empty());

                Ok(())
            }
        }

        mod when_searching_for_boarding_pass_my_seat_id {
            use super::*;

            #[test]
            fn then_it_outputs_a_seat_id_of_741() -> Result<(), Box<dyn std::error::Error>> {
                let mut input_file = NamedTempFile::new()?;
                writeln!(
                    input_file,
                    indoc! { "
                        FBFBBFFRLR
                        BFFFBBFRRR
                        FFFBBBFRRR
                        BBFFBBFRLL
                    " }
                )?;

                let mut command = Command::cargo_bin("day-5")?;
                command.arg("tests/input.txt");

                command
                    .assert()
                    .success()
                    .stdout(predicate::eq("741\n"))
                    .stderr(predicate::str::is_empty());

                Ok(())
            }
        }
    }

    mod given_no_boarding_passes {
        use super::*;

        mod when_searching_for_boarding_pass_with_largest_seat_id {
            use super::*;

            #[test]
            fn then_it_outputs_error_message() -> Result<(), Box<dyn std::error::Error>> {
                let input_file = NamedTempFile::new()?;

                let mut command = Command::cargo_bin("day-5")?;
                command.arg(input_file.path()).arg("--highest-seat-id");

                command
                    .assert()
                    .failure()
                    .stdout(predicate::str::is_empty())
                    .stderr(predicate::str::contains(
                        "Expected a positive number of boarding passes, but got none instead.",
                    ));

                Ok(())
            }
        }
    }

    mod given_a_single_invalid_boarding_pass {
        use super::*;

        mod when_searching_for_boarding_pass_with_largest_seat_id {
            use super::*;

            #[test]
            fn then_it_outputs_error_message() -> Result<(), Box<dyn std::error::Error>> {
                let mut input_file = NamedTempFile::new()?;
                let invalid_boarding_pass = "invalid-boarding-pass";
                writeln!(input_file, "{}", invalid_boarding_pass)?;

                let mut command = Command::cargo_bin("day-5")?;
                command.arg(input_file.path()).arg("--highest-seat-id");

                let error_message = format!(
                    "Expected all boarding passes to match regex \"{}\", but got \"{}\" instead.",
                    r"^[FB]{7}[LR]{3}$", invalid_boarding_pass
                );
                command
                    .assert()
                    .failure()
                    .stdout(predicate::str::is_empty())
                    .stderr(predicate::str::contains(error_message));

                Ok(())
            }
        }
    }
}
