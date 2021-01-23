use assert_cmd::Command;
use indoc::indoc;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

mod given_first_example_bag_rules {
    use super::*;

    mod when_finding_num_bags_that_can_eventually_contain_shiny_gold_bag {
        use super::*;
        use std::error::Error;

        #[test]
        fn then_it_outputs_4() -> Result<(), Box<dyn Error>> {
            let mut input_file = NamedTempFile::new()?;
            writeln!(
                input_file,
                indoc! { "
                    light red bags contain 1 bright white bag, 2 muted yellow bags.
                    dark orange bags contain 3 bright white bags, 4 muted yellow bags.
                    bright white bags contain 1 shiny gold bag.
                    muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
                    shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
                    dark olive bags contain 3 faded blue bags, 4 dotted black bags.
                    vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
                    faded blue bags contain no other bags.
                    dotted black bags contain no other bags.
                " }
            )?;

            let mut command = Command::cargo_bin("day-7")?;
            command.arg(input_file.path());

            command
                .assert()
                .success()
                .stdout(predicate::eq("4\n"))
                .stderr(predicate::str::is_empty());

            Ok(())
        }
    }

    mod when_finding_num_bags_that_shiny_gold_bag_must_contain {
        use super::*;

        #[test]
        fn then_it_outputs_32() -> Result<(), Box<dyn std::error::Error>> {
            let mut input_file = NamedTempFile::new()?;
            writeln!(
                input_file,
                indoc! { "
                    light red bags contain 1 bright white bag, 2 muted yellow bags.
                    dark orange bags contain 3 bright white bags, 4 muted yellow bags.
                    bright white bags contain 1 shiny gold bag.
                    muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
                    shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
                    dark olive bags contain 3 faded blue bags, 4 dotted black bags.
                    vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
                    faded blue bags contain no other bags.
                    dotted black bags contain no other bags.
                " }
            )?;

            let mut command = Command::cargo_bin("day-7")?;
            command.arg(input_file.path()).arg("--part-2");

            command
                .assert()
                .success()
                .stdout(predicate::eq("32\n"))
                .stderr(predicate::str::is_empty());

            Ok(())
        }
    }
}

mod given_second_example_bag_rules {
    use super::*;

    mod when_finding_num_bags_that_shiny_gold_bag_must_contain {
        use super::*;

        #[test]
        fn then_it_outputs_126() -> Result<(), Box<dyn std::error::Error>> {
            let mut input_file = NamedTempFile::new()?;
            writeln!(
                input_file,
                indoc! { "
                    shiny gold bags contain 2 dark red bags.
                    dark red bags contain 2 dark orange bags.
                    dark orange bags contain 2 dark yellow bags.
                    dark yellow bags contain 2 dark green bags.
                    dark green bags contain 2 dark blue bags.
                    dark blue bags contain 2 dark violet bags.
                    dark violet bags contain no other bags.
                " }
            )?;

            let mut command = Command::cargo_bin("day-7")?;
            command.arg(input_file.path()).arg("--part-2");

            command
                .assert()
                .success()
                .stdout(predicate::eq("126\n"))
                .stderr(predicate::str::is_empty());

            Ok(())
        }
    }
}

mod given_actual_puzzle_input {
    use super::*;

    mod when_finding_num_bags_that_can_eventually_contain_shiny_gold_bag {
        use super::*;

        #[test]
        fn then_it_outputs_246() -> Result<(), Box<dyn std::error::Error>> {
            let input_file = "puzzle-input.txt";

            let mut command = Command::cargo_bin("day-7")?;
            command.arg(input_file);

            command
                .assert()
                .success()
                .stdout(predicate::eq("246\n"))
                .stderr(predicate::str::is_empty());

            Ok(())
        }
    }

    mod when_finding_num_bags_that_shiny_gold_bag_must_contain {
        use super::*;

        #[test]
        fn then_it_outputs_2976() -> Result<(), Box<dyn std::error::Error>> {
            let input_file = "puzzle-input.txt";

            let mut command = Command::cargo_bin("day-7")?;
            command.arg(input_file).arg("--part-2");

            command
                .assert()
                .success()
                .stdout(predicate::eq("2976\n"))
                .stderr(predicate::str::is_empty());

            Ok(())
        }
    }
}
