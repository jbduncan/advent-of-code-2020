use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

lazy_static! {
    static ref BOARDING_PASS_REGEX: Regex = Regex::new(r"^[FB]{7}[LR]{3}$").unwrap();
}

pub fn largest_seat_id<T: AsRef<str>>(boarding_passes: &[T]) -> Result<u32, Day5Error> {
    if boarding_passes.is_empty() {
        return Err(Day5Error::NoBoardingPasses);
    }

    let validated_boarding_passes = validate_boarding_passes(boarding_passes)?;

    let largest_seat_id = validated_boarding_passes
        .into_iter()
        .map(|pass| seat_id_of(pass))
        .max()
        // at least one valid boarding pass exists at this point, so .unwrap() won't panic
        .unwrap();

    Ok(largest_seat_id)
}

pub fn my_seat_id<T: AsRef<str>>(boarding_passes: &[T]) -> Result<u32, Day5Error> {
    if boarding_passes.is_empty() {
        return Err(Day5Error::NoBoardingPasses);
    }

    let validated_boarding_passes = validate_boarding_passes(boarding_passes)?;

    fn not_at_front_of_plane(pass: &str) -> bool {
        pass[0..7] != "F".repeat(7)
    }

    fn not_at_back_of_plane(pass: &str) -> bool {
        pass[0..7] != "B".repeat(7)
    }

    fn missing_consecutive_seat_id(sorted_seat_ids: Vec<u32>) -> Option<u32> {
        sorted_seat_ids.iter()
            .tuple_windows::<(&u32, &u32)>()
            .skip_while(|(&a, &b)| b - a == 1)
            .map(|window| window.0 + 1)
            .next()
    }

    let mut possible_seat_ids = validated_boarding_passes
        .into_iter()
        .filter(|&pass| not_at_front_of_plane(pass))
        .filter(|&pass| not_at_back_of_plane(pass))
        .map(|pass| seat_id_of(pass))
        .collect_vec();
    possible_seat_ids.sort();
    let my_seat_id = missing_consecutive_seat_id(possible_seat_ids);

    if let Some(seat_id) = my_seat_id {
        return Ok(seat_id);
    }
    todo!() // We should return an error here saying something like:
            // "All seat IDs should be consecutive when sorted, except for one missing number.
            //  E.g. [1, 2, 4, 5] is valid."
}

fn validate_boarding_passes<T: AsRef<str>>(boarding_passes: &[T]) -> Result<Vec<&str>, Day5Error> {
    boarding_passes
        .iter()
        .map(|pass| pass.as_ref())
        .map(|pass| {
            return if BOARDING_PASS_REGEX.is_match(pass) {
                Ok(pass)
            } else {
                Err(Day5Error::InvalidBoardingPass {
                    boarding_pass: pass.to_owned(),
                })
            };
        })
        .collect::<Result<Vec<&str>, Day5Error>>()
}

fn seat_id_of(pass_with_largest_seat_id: &str) -> u32 {
    let mut instructions = pass_with_largest_seat_id.chars();

    let mut possible_rows = 0..127;
    for _ in 0..7 {
        let instr = instructions.next().unwrap();
        let delta = ((possible_rows.end - possible_rows.start) / 2) + 1;
        possible_rows = if instr == 'F' {
            let start = possible_rows.start;
            let end = possible_rows.end - delta;
            start..end
        } else {
            // instr == 'B'
            let start = possible_rows.start + delta;
            let end = possible_rows.end;
            start..end
        };
    }
    let row = possible_rows.start;

    let mut possible_columns = 0..7;
    for _ in 0..3 {
        let instr = instructions.next().unwrap();
        let delta = ((possible_columns.end - possible_columns.start) / 2) + 1;
        possible_columns = if instr == 'L' {
            let start = possible_columns.start;
            let end = possible_columns.end - delta;
            start..end
        } else {
            // instr == 'R'
            let start = possible_columns.start + delta;
            let end = possible_columns.end;
            start..end
        };
    }
    let column = possible_columns.start;

    (row * 8) + column
}

#[derive(Error, Debug, PartialEq)]
pub enum Day5Error {
    #[error("Expected a positive number of boarding passes, but got none instead.")]
    NoBoardingPasses,
    #[error(
        "Expected all boarding passes to match regex \"{}\", but got \"{boarding_pass}\" instead.",
        r"^[FB]{7}[LR]{3}$"
    )]
    InvalidBoardingPass { boarding_pass: String },
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::fs::*;

    // TODO: Write tests for 'when_finding_my_seat_id'

    mod given_no_boarding_passes {
        use super::*;

        mod when_finding_largest_seat_id {
            use super::*;

            #[test]
            fn then_it_returns_an_error() {
                let boarding_passes: Vec<&str> = vec![];

                let result = largest_seat_id(&boarding_passes);

                assert!(result.is_err());
                assert_eq!(result.unwrap_err(), Day5Error::NoBoardingPasses);
            }
        }
    }

    mod given_invalid_boarding_pass {
        use super::*;

        mod when_finding_largest_seat_id {
            use super::*;

            prop_compose! {
                fn arb_invalid_boarding_pass()(pass in r".*") -> String {
                    if BOARDING_PASS_REGEX.is_match(&pass) {
                        return "".to_string();
                    }
                    return pass;
                }
            }

            prop_compose! {
                fn arb_invalid_boarding_passes()
                        (passes in prop::collection::vec(arb_invalid_boarding_pass(), 1..100))
                    -> Vec<String> {

                    passes
                }
            }

            proptest! {
                #[test]
                fn then_it_returns_an_error(boarding_passes in arb_invalid_boarding_passes()) {
                    let boarding_passes = boarding_passes.as_slice();

                    let result = largest_seat_id(boarding_passes);

                    prop_assert!(result.is_err());
                    prop_assert_eq!(
                        result.unwrap_err(),
                        Day5Error::InvalidBoardingPass {
                            boarding_pass: boarding_passes[0].to_owned()
                        });
                }
            }
        }
    }

    mod given_example_boarding_pass_fbfbbffrlr {
        use super::*;

        mod when_finding_largest_seat_id {
            use super::*;

            #[test]
            fn then_it_returns_357() {
                let boarding_passes: Vec<&str> = vec!["FBFBBFFRLR"];

                let result = largest_seat_id(&boarding_passes);

                assert!(result.is_ok());
                assert_eq!(result.unwrap(), 357);
            }
        }
    }

    mod given_example_boarding_pass_bfffbbfrrr {
        use super::*;

        mod when_finding_largest_seat_id {
            use super::*;

            #[test]
            fn then_it_returns_357() {
                let boarding_passes: Vec<&str> = vec!["BFFFBBFRRR"];

                let result = largest_seat_id(&boarding_passes);

                assert!(result.is_ok());
                assert_eq!(result.unwrap(), 567);
            }
        }
    }

    mod given_example_boarding_pass_fffbbbfrrr {
        use super::*;

        mod when_finding_largest_seat_id {
            use super::*;

            #[test]
            fn then_it_returns_357() {
                let boarding_passes: Vec<&str> = vec!["FFFBBBFRRR"];

                let result = largest_seat_id(&boarding_passes);

                assert!(result.is_ok());
                assert_eq!(result.unwrap(), 119);
            }
        }
    }

    mod given_example_boarding_pass_bbffbbfrll {
        use super::*;

        mod when_finding_largest_seat_id {
            use super::*;

            #[test]
            fn then_it_returns_357() {
                let boarding_passes: Vec<&str> = vec!["BBFFBBFRLL"];

                let result = largest_seat_id(&boarding_passes);

                assert!(result.is_ok());
                assert_eq!(result.unwrap(), 820);
            }
        }
    }

    mod given_all_example_boarding_passes {
        use super::*;

        mod when_finding_largest_seat_id {
            use super::*;

            #[test]
            fn then_it_returns_820() {
                let boarding_passes = vec!["FBFBBFFRLR", "BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"];

                let result = largest_seat_id(&boarding_passes);

                assert!(result.is_ok());
                assert_eq!(result.unwrap(), 820);
            }
        }
    }

    mod given_all_actual_puzzle_input_boarding_passes {
        use super::*;

        mod when_finding_largest_seat_id {
            use super::*;

            #[test]
            fn then_it_returns_994() -> Result<(), Box<dyn std::error::Error>> {
                let actual_puzzle_input = read_to_string("tests/input.txt")?;
                let boarding_passes = actual_puzzle_input
                    .lines()
                    .map(|line| line.trim())
                    .filter(|line| !line.is_empty())
                    .collect_vec();

                let result = largest_seat_id(&boarding_passes);

                assert!(result.is_ok());
                assert_eq!(result.unwrap(), 994);

                Ok(())
            }
        }
    }
}
