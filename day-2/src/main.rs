use clap::{crate_version, App};
use itertools::Itertools;
use std::io::{BufReader, Lines};
use std::ops::Range;
use std::str::FromStr;
use std::{fs::File, io, io::BufRead, path::Path};

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    });
}

fn run_app() -> Result<(), String> {
    let matches = App::new("day-2")
        .version(crate_version!())
        .author("Jonathan Bluett-Duncan <jbluettduncan@gmail.com>")
        .about("Solution for Advent of Code 2020, puzzle 2")
        .args_from_usage(
            "<INPUT> 'Sets the input file of passwords and their policies to use'
            --alternative-policy 'Changes policy interpretation according to part 2'",
        )
        .get_matches();

    let passwords_and_policies_file = matches.value_of("INPUT").unwrap();
    let alternative_policy = matches.is_present("alternative-policy");

    let lines = match read_lines(&passwords_and_policies_file) {
        Ok(lines) => lines,
        Err(_) => {
            return Err(format!(
                r#"File "{}" cannot be found."#,
                passwords_and_policies_file
            ));
        }
    };

    if alternative_policy {
        print_invalid_passwords_by_positions_policy(lines)?;
    } else {
        print_invalid_passwords_by_count_range_policy(lines)?;
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn print_invalid_passwords_by_count_range_policy(
    lines: Lines<BufReader<File>>,
) -> Result<(), String> {
    let mut passwords_and_policies = Vec::new();
    for (index, line) in lines.enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(_) => return Err(format!("Line {} cannot be read.", index)),
        };

        let line = line.trim();
        if !line.is_empty() {
            passwords_and_policies.push(match PasswordAndCountRangePolicy::from_str(line) {
                Ok(p) => p,
                Err(err) => {
                    return Err(format!(
                        r#"Line {} is not a password-and-policy string: "{}". Reason: {}."#,
                        index, line, err
                    ));
                }
            });
        }
    }

    let invalid_passwords_count = passwords_and_policies
        .iter()
        .filter(|&p| p.is_valid())
        .count();
    println!("{:?}", invalid_passwords_count);

    Ok(())
}

fn print_invalid_passwords_by_positions_policy(
    lines: Lines<BufReader<File>>,
) -> Result<(), String> {
    let mut passwords_and_policies = Vec::new();
    for (index, line) in lines.enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(_) => return Err(format!("Line {} cannot be read.", index)),
        };

        let line = line.trim();
        if !line.is_empty() {
            passwords_and_policies.push(match PasswordAndPositionsPolicy::from_str(line) {
                Ok(p) => p,
                Err(err) => {
                    return Err(format!(
                        r#"Line {} is not a password-and-policy string: "{}". Reason: {}."#,
                        index, line, err
                    ));
                }
            });
        }
    }

    let invalid_passwords_count = passwords_and_policies
        .iter()
        .filter(|&p| p.is_valid())
        .count();
    println!("{:?}", invalid_passwords_count);

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct PasswordAndCountRangePolicy {
    password: String,
    policy: CountRangePolicy,
}

impl PasswordAndCountRangePolicy {
    // fn factory(alternative_policy: bool) -> fn(input: &str) -> Result<PasswordAndCountRangePolicy, String> {
    //     fn count_range_policy(s: &str) -> Result<PasswordAndCountRangePolicy, String> {
    //         todo!()
    //         // return s.parse::<CountRangePolicy>();
    //     }
    //
    //     fn position_policy(s: &str) -> Result<PasswordAndCountRangePolicy, String> {
    //         todo!()
    //     }
    //
    //     return if alternative_policy {
    //         |s: &str| {
    //             count_range_policy(s)
    //         }
    //     } else {
    //         |s: &str| {
    //             count_range_policy(s)
    //         }
    //     }
    // }

    fn is_valid(&self) -> bool {
        let expected_letter_count = &self
            .password
            .chars()
            .filter(|&c| c == self.policy.expected_letter)
            .count();
        self.policy
            .expected_letter_count_range
            .contains(expected_letter_count)
    }
}

impl FromStr for PasswordAndCountRangePolicy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(": ").map(|p| p.trim()).collect_vec();
        if parts.len() != 2 {
            return Err(
                r#"Expected to follow pattern "<digit>-<digit>: <letter>: <password>""#.to_string(),
            );
        }

        let policy = match parts[0].parse::<CountRangePolicy>() {
            Ok(policy) => policy,
            Err(_) => {
                return Err(
                    r#"Expected to follow pattern "<digit>-<digit>: <letter>: <password>""#
                        .to_string(),
                );
            }
        };

        Ok(PasswordAndCountRangePolicy {
            password: parts[1].to_string(),
            policy,
        })
    }
}

// trait Policy {
//     fn letter(&self) -> char;
//     fn first_digit(&self) -> usize;
//     fn second_digit(&self) -> usize;
// }

#[derive(Debug, PartialEq, Eq, Hash)]
struct CountRangePolicy {
    expected_letter: char,
    expected_letter_count_range: Range<usize>,
}

impl FromStr for CountRangePolicy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 2 {
            return Err(
                r#"Expected to follow pattern "<digit>-<digit>: <letter>: <password>""#.to_string(),
            );
        }

        let expected_letter = match parts[1].parse::<char>() {
            Ok(letter) => letter,
            Err(_) => {
                return Err(
                    r#"Expected to follow pattern "<digit>-<digit>: <letter>: <password>""#
                        .to_string(),
                );
            }
        };

        let digits: Result<Vec<usize>, _> = parts[0]
            .split(|c| c == ' ' || c == '-')
            .map(|part| part.parse::<usize>())
            .collect();
        let digits = match digits {
            Ok(p) => p,
            Err(_) => {
                return Err(
                    r#"Expected to follow pattern "<digit>-<digit>: <letter>: <password>""#
                        .to_string(),
                );
            }
        };
        if digits.len() != 2 {
            return Err(
                r#"Expected to follow pattern "<digit>-<digit>: <letter>: <password>""#.to_string(),
            );
        }
        let expected_letter_count_range = Range {
            start: digits[0],
            end: digits[1] + 1,
        };

        Ok(CountRangePolicy {
            expected_letter,
            expected_letter_count_range,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct PasswordAndPositionsPolicy {
    password: String,
    policy: PositionsPolicy,
}

impl PasswordAndPositionsPolicy {
    fn is_valid(&self) -> bool {
        let password_first_letter = &self
            .password
            .chars()
            .nth(self.policy.first_expected_position - 1);
        let password_second_letter = &self
            .password
            .chars()
            .nth(self.policy.second_expected_position - 1);
        return password_first_letter
            .map(|l| l == self.policy.expected_letter)
            .unwrap_or(false)
            ^ password_second_letter
                .map(|l| l == self.policy.expected_letter)
                .unwrap_or(false);
    }
}

impl FromStr for PasswordAndPositionsPolicy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(": ").map(|p| p.trim()).collect_vec();
        if parts.len() != 2 {
            return Err(
                r#"Expected to follow pattern "<digit>-<digit>: <letter>: <password>""#.to_string(),
            );
        }

        let policy = match parts[0].parse::<PositionsPolicy>() {
            Ok(policy) => policy,
            Err(_) => {
                return Err(
                    r#"Expected to follow pattern "<digit>-<digit>: <letter>: <password>""#
                        .to_string(),
                );
            }
        };

        Ok(PasswordAndPositionsPolicy {
            password: parts[1].to_string(),
            policy,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct PositionsPolicy {
    expected_letter: char,
    first_expected_position: usize,
    second_expected_position: usize,
}

impl FromStr for PositionsPolicy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 2 {
            return Err(
                r#"Expected to follow pattern "<digit>-<digit>: <letter>: <password>""#.to_string(),
            );
        }

        let expected_letter = match parts[1].parse::<char>() {
            Ok(letter) => letter,
            Err(_) => {
                return Err(
                    r#"Expected to follow pattern "<digit>-<digit>: <letter>: <password>""#
                        .to_string(),
                );
            }
        };

        let digits: Result<Vec<usize>, _> = parts[0]
            .split(|c| c == ' ' || c == '-')
            .map(|part| part.parse::<usize>())
            .collect();
        let digits = match digits {
            Ok(p) => p,
            Err(_) => {
                return Err(
                    r#"Expected to follow pattern "<digit>-<digit>: <letter>: <password>""#
                        .to_string(),
                );
            }
        };
        if digits.len() != 2 {
            return Err(
                r#"Expected to follow pattern "<digit>-<digit>: <letter>: <password>""#.to_string(),
            );
        }

        Ok(PositionsPolicy {
            expected_letter,
            first_expected_position: digits[0],
            second_expected_position: digits[1],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_range_policy_from_str() {
        let policy = "1-3 a".parse::<CountRangePolicy>();
        assert!(policy.is_ok());
        let policy = policy.unwrap();
        assert_eq!(policy.expected_letter, 'a');
        assert_eq!(policy.expected_letter_count_range, (1..4));
    }

    #[test]
    fn positions_policy_from_str() {
        let policy = "1-3 a".parse::<PositionsPolicy>();
        assert!(policy.is_ok());
        let policy = policy.unwrap();
        assert_eq!(policy.expected_letter, 'a');
        assert_eq!(policy.first_expected_position, 1);
        assert_eq!(policy.second_expected_position, 3);
    }

    #[test]
    fn password_and_count_range_policy_from_str() {
        let password_and_policy = "1-3 a: abcde".parse::<PasswordAndCountRangePolicy>();
        assert!(password_and_policy.is_ok());
        let password_and_policy = password_and_policy.unwrap();
        assert_eq!(
            password_and_policy,
            PasswordAndCountRangePolicy {
                password: "abcde".to_string(),
                policy: CountRangePolicy {
                    expected_letter: 'a',
                    expected_letter_count_range: (1..4),
                },
            }
        )
    }

    #[test]
    fn password_and_positions_policy_from_str() {
        let password_and_policy = "1-3 a: abcde".parse::<PasswordAndPositionsPolicy>();
        assert!(password_and_policy.is_ok());
        let password_and_policy = password_and_policy.unwrap();
        assert_eq!(
            password_and_policy,
            PasswordAndPositionsPolicy {
                password: "abcde".to_string(),
                policy: PositionsPolicy {
                    expected_letter: 'a',
                    first_expected_position: 1,
                    second_expected_position: 3,
                },
            }
        )
    }

    #[test]
    fn password_and_count_range_policy_is_valid() {
        let a = "1-3 a: abcde"
            .parse::<PasswordAndCountRangePolicy>()
            .unwrap();
        assert!(a.is_valid());

        let b = "1-3 b: cdefg"
            .parse::<PasswordAndCountRangePolicy>()
            .unwrap();
        assert!(!b.is_valid());

        let c = "2-9 c: ccccccccc"
            .parse::<PasswordAndCountRangePolicy>()
            .unwrap();
        assert!(c.is_valid());
    }

    #[test]
    fn password_and_positions_policy_is_valid() {
        let a = "1-3 a: abcde"
            .parse::<PasswordAndPositionsPolicy>()
            .unwrap();
        assert!(a.is_valid());

        let b = "1-3 b: bcdef"
            .parse::<PasswordAndPositionsPolicy>()
            .unwrap();
        assert!(b.is_valid());

        let c = "1-3 b: aabcd"
            .parse::<PasswordAndPositionsPolicy>()
            .unwrap();
        assert!(c.is_valid());

        let d = "1-3 b: babcd"
            .parse::<PasswordAndPositionsPolicy>()
            .unwrap();
        assert!(!d.is_valid());

        let e = "2-9 c: ccccccccc"
            .parse::<PasswordAndPositionsPolicy>()
            .unwrap();
        assert!(!e.is_valid());
    }
}
