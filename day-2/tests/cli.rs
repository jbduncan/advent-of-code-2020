use assert_cmd::prelude::*;
use indoc::indoc;
use predicates::prelude::*;
use std::io::prelude::*;
use std::process::Command;
use tempfile::NamedTempFile;

#[cfg(test)]
mod given_passwords_and_policies {
    use super::*;

    mod when_day_2_processes_them {
        use super::*;

        #[test]
        fn then_it_should_return_number_of_valid_passwords(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let mut actual_password_and_policies_file = NamedTempFile::new()?;
            writeln!(
                actual_password_and_policies_file,
                indoc! { "
                    1-3 a: abcde
                    1-3 b: cdefg
                    2-9 c: ccccccccc
                " }
            )?;
            let expected_result = "2\n";

            let mut cmd = Command::cargo_bin("day-2")?;
            cmd.arg(actual_password_and_policies_file.path());

            cmd.assert()
                .success()
                .stdout(predicate::eq(expected_result));
            Ok(())
        }
    }

    mod when_day_2_processes_them_with_alternative_policy_interpretation {
        use super::*;

        #[test]
        fn then_it_should_return_number_of_valid_passwords(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let mut actual_password_and_policies_file = NamedTempFile::new()?;
            writeln!(
                actual_password_and_policies_file,
                indoc! { "
                    1-3 a: abcde
                    1-3 b: cdefg
                    2-9 c: ccccccccc
                " }
            )?;
            let expected_result = "1\n";

            let mut cmd = Command::cargo_bin("day-2")?;
            cmd.arg(actual_password_and_policies_file.path())
                .arg("--alternative-policy");

            cmd.assert()
                .success()
                .stdout(predicate::eq(expected_result));
            Ok(())
        }
    }
}
