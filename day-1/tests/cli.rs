#[cfg(test)]
mod given_expense_report {
    mod when_searching_for_two_expenses_that_sum_to_2020 {

        use assert_cmd::prelude::*;
        use indoc::indoc;
        use predicates::prelude::*;
        use std::io::prelude::*;
        use std::process::Command;
        use tempfile::NamedTempFile;

        #[test]
        fn then_prints_product_of_those_expenses() -> Result<(), Box<dyn std::error::Error>> {
            let mut actual_expense_report_file = NamedTempFile::new()?;
            writeln!(
                actual_expense_report_file,
                indoc! { "
                    1721
                    979
                    366
                    299
                    675
                    1456
                " }
            )?;
            let expected_result = "514579\n";

            let mut cmd = Command::cargo_bin("day-1")?;
            cmd.arg(actual_expense_report_file.path()).arg("2");

            cmd.assert()
                .success()
                .stdout(predicate::eq(expected_result));
            Ok(())
        }
    }

    mod when_searching_for_three_expenses_that_sum_to_2020 {

        use assert_cmd::prelude::*;
        use indoc::indoc;
        use predicates::prelude::*;
        use std::io::prelude::*;
        use std::process::Command;
        use tempfile::NamedTempFile;

        #[test]
        fn then_prints_product_of_those_expenses() -> Result<(), Box<dyn std::error::Error>> {
            let mut actual_expense_report_file = NamedTempFile::new()?;
            writeln!(
                actual_expense_report_file,
                indoc! { "
                    1721
                    979
                    366
                    299
                    675
                    1456
                " }
            )?;
            let expected_result = "514579\n";

            let mut cmd = Command::cargo_bin("day-1")?;
            cmd.arg(actual_expense_report_file.path()).arg("2");

            cmd.assert()
                .success()
                .stdout(predicate::eq(expected_result));
            Ok(())
        }
    }
}
