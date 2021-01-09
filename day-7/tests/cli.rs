use assert_cmd::Command;
use indoc::indoc;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

mod given_example_bag_rules {
    use super::*;

    mod when_finding_num_bags_that_can_eventually_contain_shiny_gold_bag {
        use super::*;

        #[test]
        fn then_it_outputs_4() -> Result<(), Box<dyn std::error::Error>> {
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
}
