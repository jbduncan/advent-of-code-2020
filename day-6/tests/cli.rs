use assert_cmd::Command;
use indoc::indoc;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

mod given_example_answered_questions_from_part_1 {
    use super::*;

    mod when_summing_num_unique_questions_answered_by_each_group {
        use super::*;

        #[test]
        fn then_it_outputs_11() -> Result<(), Box<dyn std::error::Error>> {
            let mut input_file = NamedTempFile::new()?;
            writeln!(
                input_file,
                indoc! { "
                    abc

                    a
                    b
                    c

                    ab
                    ac

                    a
                    a
                    a
                    a

                    b
                " }
            )?;

            let mut command = Command::cargo_bin("day-6")?;
            command.arg(input_file.path());

            command
                .assert()
                .success()
                .stdout(predicate::eq("11\n"))
                .stderr(predicate::str::is_empty());

            Ok(())
        }
    }

    mod when_summing_num_questions_answered_by_everyone_per_group {
        use super::*;

        #[test]
        fn then_it_outputs_6() -> Result<(), Box<dyn std::error::Error>> {
            let mut input_file = NamedTempFile::new()?;
            writeln!(
                input_file,
                indoc! { "
                    abc

                    a
                    b
                    c

                    ab
                    ac

                    a
                    a
                    a
                    a

                    b
                " }
            )?;

            let mut command = Command::cargo_bin("day-6")?;
            command.arg(input_file.path()).arg("--everyone");

            command
                .assert()
                .success()
                .stdout(predicate::eq("6\n"))
                .stderr(predicate::str::is_empty());

            Ok(())
        }
    }
}
