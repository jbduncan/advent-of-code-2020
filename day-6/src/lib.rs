use itertools::Itertools;
use std::collections::HashSet;

pub fn sum_unique_questions_answered_per_group<T: AsRef<str>>(groups: T) -> usize {
    groups
        .as_ref()
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|&c| c >= 'a' && c <= 'z')
                .unique()
                .count()
        })
        .sum()
}

pub fn sum_questions_answered_by_everyone_per_group<T: AsRef<str>>(groups: T) -> usize {
    groups
        .as_ref()
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|questions| {
                    questions
                        .chars()
                        .filter(|&c| c >= 'a' && c <= 'z')
                        .collect::<HashSet<char>>()
                })
                .fold1(|first_char_set, second_char_set| {
                    first_char_set
                        .intersection(&second_char_set)
                        .map(|c| *c)
                        .collect::<HashSet<char>>()
                })
                .map(|common_chars| common_chars.len())
                .unwrap_or(0)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    mod given_empty_or_blank_string_groups {
        use super::*;

        mod when_summing_unique_questions_answered_per_group {
            use super::*;
            use test_case::test_case;

            #[test_case("".to_string() ; "where string has length of 0")]
            #[test_case(" ".to_string() ; "where string has 1 space")]
            #[test_case(" ".repeat(2) ; "where string has 2 spaces")]
            #[test_case(" ".repeat(42) ; "where string has 42 spaces")]
            #[test_case("\n".to_string() ; "where string has 1 newline")]
            #[test_case("\n".repeat(2) ; "where string has 2 newlines")]
            #[test_case("\n".repeat(42) ; "where string has 42 newlines")]
            #[test_case(" \n ".to_string() ; "where string has spaces separated by newline")]
            #[test_case(
                " \n\n ".to_string() ;
                "where string has spaces separated by 2 newlines")]
            #[test_case(
                " \n\n      \n \n     \n\n\n".to_string() ;
                "where string is mix of spaces and newlines")]
            fn then_return_0(groups: String) {
                let result = sum_unique_questions_answered_per_group(groups);

                assert_eq!(result, 0);
            }
        }

        mod when_summing_questions_answered_by_everyone_per_group {
            use super::*;
            use test_case::test_case;

            #[test_case("".to_string() ; "where string has length of 0")]
            #[test_case(" ".to_string() ; "where string has 1 space")]
            #[test_case(" ".repeat(2) ; "where string has 2 spaces")]
            #[test_case(" ".repeat(42) ; "where string has 42 spaces")]
            #[test_case("\n".to_string() ; "where string has 1 newline")]
            #[test_case("\n".repeat(2) ; "where string has 2 newlines")]
            #[test_case("\n".repeat(42) ; "where string has 42 newlines")]
            #[test_case(" \n ".to_string() ; "where string has spaces separated by newline")]
            #[test_case(
                " \n\n ".to_string() ;
                "where string has spaces separated by 2 newlines")]
            #[test_case(
                " \n\n      \n \n     \n\n\n".to_string() ;
                "where string is mix of spaces and newlines")]
            fn then_return_0(groups: String) {
                let result = sum_questions_answered_by_everyone_per_group(groups);

                assert_eq!(result, 0);
            }
        }
    }

    mod given_single_group_with_one_person_and_one_answered_question {
        use super::*;

        mod when_summing_unique_questions_answered_per_group {
            use super::*;
            use test_case::test_case;

            #[test_case("a" ; "where question is 'a'")]
            #[test_case("b" ; "where question is 'b'")]
            #[test_case("c" ; "where question is 'c'")]
            #[test_case("z" ; "where question is 'z'")]
            fn then_return_1(groups: &str) {
                let result = sum_unique_questions_answered_per_group(groups);

                assert_eq!(result, 1);
            }
        }

        mod when_summing_questions_answered_by_everyone_per_group {
            use super::*;
            use test_case::test_case;

            #[test_case("a" ; "where question is 'a'")]
            #[test_case("b" ; "where question is 'b'")]
            #[test_case("c" ; "where question is 'c'")]
            #[test_case("z" ; "where question is 'z'")]
            fn then_return_1(groups: &str) {
                let result = sum_questions_answered_by_everyone_per_group(groups);

                assert_eq!(result, 1);
            }
        }
    }

    mod given_single_group_with_one_person_and_two_answered_questions {
        use super::*;

        mod when_summing_unique_questions_answered_per_group {
            use super::*;

            #[test]
            fn then_return_2() {
                let groups = "ab";

                let result = sum_unique_questions_answered_per_group(groups);

                assert_eq!(result, 2);
            }
        }

        mod when_summing_questions_answered_by_everyone_per_group {
            use super::*;

            #[test]
            fn then_return_2() {
                let groups = "ab";

                let result = sum_questions_answered_by_everyone_per_group(groups);

                assert_eq!(result, 2);
            }
        }
    }

    mod given_single_group_with_two_people_and_one_different_question_each {
        use super::*;

        mod when_summing_unique_questions_answered_per_group {
            use super::*;

            #[test]
            fn then_return_2() {
                let groups = indoc! { "
                    a
                    b"
                };

                let result = sum_unique_questions_answered_per_group(groups);

                assert_eq!(result, 2);
            }
        }

        mod when_summing_questions_answered_by_everyone_per_group {
            use super::*;

            #[test]
            fn then_return_0() {
                let groups = indoc! { "
                    a
                    b"
                };

                let result = sum_questions_answered_by_everyone_per_group(groups);

                assert_eq!(result, 0);
            }
        }
    }

    mod given_single_group_with_two_people_and_both_answered_same_question {
        use super::*;

        mod when_summing_unique_questions_answered_per_group {
            use super::*;

            #[test]
            fn then_return_1() {
                let groups = indoc! { "
                    a
                    a"
                };

                let result = sum_unique_questions_answered_per_group(groups);

                assert_eq!(result, 1);
            }
        }

        mod when_summing_questions_answered_by_everyone_per_group {
            use super::*;

            #[test]
            fn then_return_1() {
                let groups = indoc! { "
                    a
                    a"
                };

                let result = sum_questions_answered_by_everyone_per_group(groups);

                assert_eq!(result, 1);
            }
        }
    }

    mod given_two_groups_with_one_person_each_and_one_answered_question_each {
        use super::*;

        mod when_summing_unique_questions_answered_per_group {
            use super::*;

            #[test]
            fn then_return_2() {
                let groups = indoc! { "
                    a

                    b"
                };

                let result = sum_unique_questions_answered_per_group(groups);

                assert_eq!(result, 2);
            }
        }

        mod when_summing_questions_answered_by_everyone_per_group {
            use super::*;

            #[test]
            fn then_return_2() {
                let groups = indoc! { "
                    a

                    b"
                };

                let result = sum_questions_answered_by_everyone_per_group(groups);

                assert_eq!(result, 2);
            }
        }
    }

    mod given_two_groups_with_one_person_each_and_both_answered_same_question {
        use super::*;

        mod when_summing_unique_questions_answered_per_group {
            use super::*;

            #[test]
            fn then_return_2() {
                let groups = indoc! { "
                    a

                    a"
                };

                let result = sum_unique_questions_answered_per_group(groups);

                assert_eq!(result, 2);
            }
        }

        mod when_summing_questions_answered_by_everyone_per_group {
            use super::*;

            #[test]
            fn then_return_2() {
                let groups = indoc! { "
                    a

                    a"
                };

                let result = sum_questions_answered_by_everyone_per_group(groups);

                assert_eq!(result, 2);
            }
        }
    }

    mod given_two_groups_with_two_people_each_and_all_answered_different_questions {
        use super::*;

        mod when_summing_questions_answered_by_everyone_per_group {
            use super::*;

            #[test]
            fn then_return_0() {
                let groups = indoc! { "
                    a
                    b

                    c
                    d"
                };

                let result = sum_questions_answered_by_everyone_per_group(groups);

                assert_eq!(result, 0);
            }
        }
    }

    mod given_two_groups_with_two_people_each_and_all_answered_the_same_question {
        use super::*;

        mod when_summing_questions_answered_by_everyone_per_group {
            use super::*;

            #[test]
            fn then_return_2() {
                let groups = indoc! { "
                    a
                    a

                    a
                    a"
                };

                let result = sum_questions_answered_by_everyone_per_group(groups);

                assert_eq!(result, 2);
            }
        }
    }

    mod given_one_group_with_one_person_and_three_answered_questions {
        use super::*;

        mod when_summing_questions_answered_by_everyone_per_group {
            use super::*;

            #[test]
            fn then_return_3() {
                let groups = "abc";

                let result = sum_questions_answered_by_everyone_per_group(groups);

                assert_eq!(result, 3);
            }
        }
    }

    mod given_one_group_with_three_people_and_one_different_answered_question_each {
        use super::*;

        mod when_summing_questions_answered_by_everyone_per_group {
            use super::*;

            #[test]
            fn then_return_0() {
                let groups = indoc! { "
                    a
                    b
                    c
                " };

                let result = sum_questions_answered_by_everyone_per_group(groups);

                assert_eq!(result, 0);
            }
        }
    }

    mod given_one_group_with_two_people {
        use super::*;

        mod and_two_questions_answered_each_with_one_question_in_common {
            use super::*;

            mod when_summing_questions_answered_by_everyone_per_group {
                use super::*;

                #[test]
                fn then_return_1() {
                    let groups = indoc! { "
                        ab
                        ac
                    " };

                    let result = sum_questions_answered_by_everyone_per_group(groups);

                    assert_eq!(result, 1);
                }
            }
        }
    }

    mod given_one_group_with_four_people_and_all_answered_the_same_question {
        use super::*;

        mod when_summing_questions_answered_by_everyone_per_group {
            use super::*;

            #[test]
            fn then_return_1() {
                let groups = indoc! { "
                    a
                    a
                    a
                    a
                " };

                let result = sum_questions_answered_by_everyone_per_group(groups);

                assert_eq!(result, 1);
            }
        }
    }

    mod given_example_puzzle_input {
        use super::*;

        mod when_summing_questions_answered_by_everyone_per_group {
            use super::*;

            #[test]
            fn then_return_6() {
                let groups = indoc! { "
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
                " };

                let result = sum_questions_answered_by_everyone_per_group(groups);

                assert_eq!(result, 6);
            }
        }
    }

    mod given_actual_puzzle_input {
        use super::*;

        mod when_summing_unique_questions_answered_per_group {
            use super::*;

            #[test]
            fn then_return_6799() -> Result<(), Box<dyn std::error::Error>> {
                let groups = std::fs::read_to_string("puzzle-input.txt")?;

                let result = sum_unique_questions_answered_per_group(groups);

                assert_eq!(result, 6799);

                Ok(())
            }
        }

        mod when_summing_questions_answered_by_everyone_per_group {
            use super::*;

            #[test]
            fn then_return_3354() -> Result<(), Box<dyn std::error::Error>> {
                let groups = std::fs::read_to_string("puzzle-input.txt")?;

                let result = sum_questions_answered_by_everyone_per_group(groups);

                assert_eq!(result, 3354);

                Ok(())
            }
        }
    }
}
