use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn valid_passports(batch: &str) -> usize {
    let batch = batch.trim();
    if batch.is_empty() {
        return 0;
    }

    batch
        .split("\n\n")
        .filter(|passport| is_valid(*passport, false))
        .count()
}

pub fn valid_passports_and_fields(batch: &str) -> usize {
    let batch = batch.trim();
    if batch.is_empty() {
        return 0;
    }

    batch
        .split("\n\n")
        .filter(|passport| is_valid(*passport, true))
        .count()
}

lazy_static! {
    static ref EXPECTED_FIELD_KEYS: HashSet<&'static str> =
        HashSet::from_iter(vec!["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"].into_iter());
    static ref WHITESPACE_REGEX: Regex = Regex::new(r"\s+").unwrap();
}

fn is_valid(passport: &str, validate_fields: bool) -> bool {
    let encountered_fields = {
        let mut set = HashSet::new();
        for field in WHITESPACE_REGEX.split(passport.trim()) {
            let key_and_value: Option<(_, _)> = field.split(':').collect_tuple();
            match key_and_value {
                Some((key, value)) => {
                    if validate_fields && !field_is_valid(key, value) {
                        return false;
                    }
                    set.insert(key);
                }
                None => {
                    todo!()
                }
            }
        }
        set
    };
    EXPECTED_FIELD_KEYS.is_subset(&encountered_fields)
}

lazy_static! {
    static ref HGT_CM_REGEX: Regex = Regex::new(r"^(?P<num>[1-9][0-9]{2})cm$").unwrap();
    static ref HGT_IN_REGEX: Regex = Regex::new(r"^(?P<num>[1-9][0-9])in$").unwrap();
    static ref HCL_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref VALID_EYE_COLOURS: HashSet<&'static str> =
        HashSet::from_iter(vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].into_iter());
}

fn field_is_valid(key: &str, value: &str) -> bool {
    match key {
        "byr" => value
            .parse::<u32>()
            .map_or(false, |byr| (1920..=2002).contains(&byr)),
        "iyr" => value
            .parse::<u32>()
            .map_or(false, |iyr| (2010..=2020).contains(&iyr)),
        "eyr" => value
            .parse::<u32>()
            .map_or(false, |eyr| (2020..=2030).contains(&eyr)),
        "hgt" => {
            if let Some(centimetres) = extract_num_as_u32(value, &*HGT_CM_REGEX) {
                return (150..=193).contains(&centimetres);
            }

            if let Some(inches) = extract_num_as_u32(value, &*HGT_IN_REGEX) {
                return (59..=76).contains(&inches);
            }

            return false;
        }
        "hcl" => {
            return HCL_REGEX.is_match(value);
        }
        "ecl" => {
            return VALID_EYE_COLOURS.contains(&value);
        }
        "pid" => {
            return value.len() == 9 && value.parse::<u32>().map_or(false, |num| num <= 999_999_999)
        }
        _ => true,
    }
}

fn extract_num_as_u32(value: &str, regex: &Regex) -> Option<u32> {
    return regex
        .captures(value)
        .and_then(|cap| cap.name("num").map(|num| num.as_str()))
        .map(|num| num.parse::<u32>().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use proptest::prelude::*;
    use std::collections::HashMap;

    fn joined_in_random_order(key_value_pairs: &[(&str, &str)]) -> String {
        let randomizer_map = {
            let mut result = HashMap::new();
            for (k, v) in key_value_pairs {
                result.entry(*k).or_insert(*v);
            }
            result
        };
        randomizer_map
            .iter()
            .map(|(&key, &value)| format!("{}:{}", key, value))
            .intersperse(" ".to_string())
            .join("")
    }

    prop_compose! {
        fn arb_birth_year()(byr in 1920..=2002u32) -> String {
            byr.to_string()
        }
    }

    prop_compose! {
        fn arb_issue_year()(iyr in 2010..=2020u32) -> String {
            iyr.to_string()
        }
    }

    prop_compose! {
        fn arb_expiration_year()(eyr in 2020..=2030u32) -> String {
            eyr.to_string()
        }
    }

    prop_compose! {
        fn arb_height_in_centimetres()(hgt in 150..=193u32) -> String {
            hgt.to_string() + "cm"
        }
    }

    prop_compose! {
        fn arb_height_in_inches()(hgt in 59..=76u32) -> String {
            hgt.to_string() + "in"
        }
    }

    prop_compose! {
        fn arb_height()
                (cm in arb_height_in_centimetres(),
                inches in arb_height_in_inches(),
                is_cm in any::<bool>())
            -> String {

            if is_cm {
                cm
            } else {
                inches
            }
        }
    }

    prop_compose! {
        fn arb_hair_colour()(hcl in r"#[0-9a-f]{6}") -> String {
            hcl.to_string()
        }
    }

    prop_compose! {
        fn arb_eye_colour()(ecl in r"(amb|blu|brn|gry|grn|hzl|oth)") -> String {
            ecl.to_string()
        }
    }

    prop_compose! {
        fn arb_passport_id()(pid in r"[0-9]{9}") -> String {
            pid.to_string()
        }
    }

    prop_compose! {
        fn arb_country_id()(pid in r"[0-9a-zA-Z]+") -> String {
            pid.to_string()
        }
    }

    prop_compose! {
        fn arb_passport()
                (ecl in arb_eye_colour(),
                pid in arb_passport_id(),
                eyr in arb_expiration_year(),
                hcl in arb_hair_colour(),
                byr in arb_birth_year(),
                iyr in arb_issue_year(),
                cid in arb_country_id(),
                hgt in arb_height())
            -> String {

            joined_in_random_order(
                &[("ecl", &ecl), ("pid", &pid), ("eyr", &eyr),
                ("hcl", &hcl), ("byr", &byr), ("iyr", &iyr),
                ("cid", &cid), ("hgt", &hgt)])
        }
    }

    prop_compose! {
        fn arb_passport_without_cid()
                (ecl in arb_eye_colour(),
                pid in arb_passport_id(),
                eyr in arb_expiration_year(),
                hcl in arb_hair_colour(),
                byr in arb_birth_year(),
                iyr in arb_issue_year(),
                hgt in arb_height())
            -> String {

            joined_in_random_order(
                &[("ecl", &ecl), ("pid", &pid), ("eyr", &eyr),
                ("hcl", &hcl), ("byr", &byr), ("iyr", &iyr),
                ("hgt", &hgt)])
        }
    }

    prop_compose! {
        fn arb_passport_with_or_without_cid()
                (ecl in arb_eye_colour(),
                pid in arb_passport_id(),
                eyr in arb_expiration_year(),
                hcl in arb_hair_colour(),
                byr in arb_birth_year(),
                iyr in arb_issue_year(),
                cid in arb_country_id(),
                include_cid in any::<bool>(),
                hgt in arb_height())
            -> String {

            let mut passport_fields: Vec<(&str, &str)> =
                vec![("ecl", &ecl[..]), ("pid", &pid[..]), ("eyr", &eyr[..]),
                    ("hcl", &hcl[..]), ("byr", &byr[..]), ("iyr", &iyr[..]),
                    ("hgt", &hgt[..])];
            if include_cid {
                passport_fields.push(("cid", &cid[..]));
            }
            joined_in_random_order(passport_fields.as_slice())
        }
    }

    prop_compose! {
        fn arb_batch_of_passports()
                (vec in prop::collection::vec(arb_passport_with_or_without_cid(), 2..100))
            -> (String, usize) {

            return (vec.iter().join("\n\n"), vec.len());
        }
    }

    mod given_empty_or_blank_batch {
        use super::*;

        mod when_searching_for_valid_passports {
            use super::*;

            proptest! {
                #[test]
                fn then_it_returns_0(batch in r"\s*") {
                    let result = valid_passports(&batch);

                    prop_assert_eq!(result, 0);
                }
            }
        }

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            proptest! {
                #[test]
                fn then_it_returns_0(batch in r"\s*") {
                    let result = valid_passports_and_fields(&batch);

                    prop_assert_eq!(result, 0);
                }
            }
        }
    }

    mod given_single_valid_passport {
        use super::*;

        mod when_searching_for_valid_passports {
            use super::*;

            proptest! {
                #[test]
                fn then_it_returns_1(passport in arb_passport()) {
                    let result = valid_passports(&passport);

                    prop_assert_eq!(result, 1);
                }
            }
        }

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            proptest! {
                #[test]
                fn then_it_returns_1(passport in arb_passport()) {
                    let result = valid_passports_and_fields(&passport);

                    prop_assert_eq!(result, 1);
                }
            }
        }
    }

    mod given_single_valid_passport_with_newline {
        use super::*;

        mod when_searching_for_valid_passports {
            use super::*;

            #[test]
            fn then_it_returns_1() {
                let batch = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
                    byr:1937 iyr:2017 cid:147 hgt:183cm";

                let result = valid_passports(&batch);

                assert_eq!(result, 1);
            }
        }

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            #[test]
            fn then_it_returns_1() {
                let batch = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
                    byr:1937 iyr:2017 cid:147 hgt:183cm";

                let result = valid_passports_and_fields(&batch);

                assert_eq!(result, 1);
            }
        }
    }

    mod given_single_valid_passport_with_extra_whitespaces {
        use super::*;

        mod when_searching_for_valid_passports {
            use super::*;

            #[test]
            fn then_it_returns_1() {
                let batch = "  ecl:gry pid:860033327    eyr:2020 hcl:#fffffd\n\
                    byr:1937\niyr:2017 cid:147\nhgt:183cm ";

                let result = valid_passports(&batch);

                assert_eq!(result, 1);
            }
        }

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            #[test]
            fn then_it_returns_1() {
                let batch = "  ecl:gry pid:860033327    eyr:2020 hcl:#fffffd\n\
                    byr:1937\niyr:2017 cid:147\nhgt:183cm ";

                let result = valid_passports_and_fields(&batch);

                assert_eq!(result, 1);
            }
        }
    }

    mod given_single_valid_passport_without_optional_cid_field {
        use super::*;

        mod when_searching_for_valid_passports {
            use super::*;

            proptest! {
                #[test]
                fn then_it_returns_1(passport in arb_passport_without_cid()) {
                    let result = valid_passports(&passport);

                    prop_assert_eq!(result, 1);
                }
            }
        }

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            proptest! {
                #[test]
                fn then_it_returns_1(passport in arb_passport_without_cid()) {
                    let result = valid_passports_and_fields(&passport);

                    prop_assert_eq!(result, 1);
                }
            }
        }
    }

    mod given_single_valid_passport_with_a_repeating_field {
        use super::*;

        mod when_searching_for_valid_passports {
            use super::*;

            #[test]
            fn then_it_returns_1() {
                let passport = "ecl:gry ecl:blu pid:860033327 eyr:2020 hcl:#fffffd \
                    byr:1937 iyr:2017 cid:147 hgt:183cm";

                let result = valid_passports(&passport);

                assert_eq!(result, 1);
            }
        }

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            #[test]
            fn then_it_returns_1() {
                let passport = "ecl:gry ecl:blu pid:860033327 eyr:2020 hcl:#fffffd \
                    byr:1937 iyr:2017 cid:147 hgt:183cm";

                let result = valid_passports_and_fields(&passport);

                assert_eq!(result, 1);
            }
        }
    }

    mod given_single_valid_passport_with_an_unrecognized_field {
        use super::*;

        mod when_searching_for_valid_passports {
            use super::*;

            #[test]
            fn then_it_returns_1() {
                let passport = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd \
                    byr:1937 iyr:2017 cid:147 hgt:183cm foo:bar";

                let result = valid_passports(&passport);

                assert_eq!(result, 1);
            }
        }

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            #[test]
            fn then_it_returns_1() {
                let passport = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd \
                    byr:1937 iyr:2017 cid:147 hgt:183cm foo:bar";

                let result = valid_passports_and_fields(&passport);

                assert_eq!(result, 1);
            }
        }
    }

    mod given_single_passport_with_missing_mandatory_field {
        use super::*;

        mod when_searching_for_valid_passports {
            use super::*;

            #[test]
            fn then_it_returns_0() {
                let passport = "ecl:gry ecl:blu pid:860033327 eyr:2020 hcl:#fffffd \
                    byr:1937 iyr:2017 cid:147";

                let result = valid_passports(&passport);

                assert_eq!(result, 0);
            }
        }

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            #[test]
            fn then_it_returns_0() {
                let passport = "ecl:gry ecl:blu pid:860033327 eyr:2020 hcl:#fffffd \
                    byr:1937 iyr:2017 cid:147";

                let result = valid_passports_and_fields(&passport);

                assert_eq!(result, 0);
            }
        }
    }

    mod given_single_passport_with_too_small_byr {
        use super::*;

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            #[test]
            fn then_it_returns_0() {
                let passport = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd \
                    byr:1919 iyr:2017 cid:147 hgt:183cm";

                let result = valid_passports_and_fields(&passport);

                assert_eq!(result, 0);
            }
        }
    }

    mod given_single_passport_with_too_large_byr {
        use super::*;

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            #[test]
            fn then_it_returns_0() {
                let passport = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd \
                    byr:2003 iyr:2017 cid:147 hgt:183cm";

                let result = valid_passports_and_fields(&passport);

                assert_eq!(result, 0);
            }
        }
    }

    mod given_single_passport_with_too_small_iyr {
        use super::*;

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            #[test]
            fn then_it_returns_0() {
                let passport = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd \
                    byr:2002 iyr:2009 cid:147 hgt:183cm";

                let result = valid_passports_and_fields(&passport);

                assert_eq!(result, 0);
            }
        }
    }

    mod given_single_passport_with_too_large_iyr {
        use super::*;

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            #[test]
            fn then_it_returns_0() {
                let passport = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd \
                    byr:2002 iyr:2021 cid:147 hgt:183cm";

                let result = valid_passports_and_fields(&passport);

                assert_eq!(result, 0);
            }
        }
    }

    mod given_single_passport_with_too_small_eyr {
        use super::*;

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            #[test]
            fn then_it_returns_0() {
                let passport = "ecl:gry pid:860033327 eyr:2019 hcl:#fffffd \
                    byr:2002 iyr:2010 cid:147 hgt:183cm";

                let result = valid_passports_and_fields(&passport);

                assert_eq!(result, 0);
            }
        }
    }

    mod given_single_passport_with_too_large_eyr {
        use super::*;

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            #[test]
            fn then_it_returns_0() {
                let passport = "ecl:gry pid:860033327 eyr:2031 hcl:#fffffd \
                    byr:2002 iyr:2010 cid:147 hgt:183cm";

                let result = valid_passports_and_fields(&passport);

                assert_eq!(result, 0);
            }
        }
    }

    mod given_single_passport_with_too_small_hgt_cm {
        use super::*;

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            #[test]
            fn then_it_returns_0() {
                let passport = "ecl:gry pid:860033327 eyr:2030 hcl:#fffffd \
                    byr:2002 iyr:2010 cid:147 hgt:149cm";

                let result = valid_passports_and_fields(&passport);

                assert_eq!(result, 0);
            }
        }
    }

    mod given_single_passport_with_too_large_hgt_cm {
        use super::*;

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            #[test]
            fn then_it_returns_0() {
                let passport = "ecl:gry pid:860033327 eyr:2030 hcl:#fffffd \
                    byr:2002 iyr:2010 cid:147 hgt:194cm";

                let result = valid_passports_and_fields(&passport);

                assert_eq!(result, 0);
            }
        }
    }

    mod given_single_passport_with_too_small_hgt_in {
        use super::*;

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            #[test]
            fn then_it_returns_0() {
                let passport = "ecl:gry pid:860033327 eyr:2030 hcl:#fffffd \
                    byr:2002 iyr:2010 cid:147 hgt:58in";

                let result = valid_passports_and_fields(&passport);

                assert_eq!(result, 0);
            }
        }
    }

    mod given_single_passport_with_too_large_hgt_in {
        use super::*;

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            #[test]
            fn then_it_returns_0() {
                let passport = "ecl:gry pid:860033327 eyr:2030 hcl:#fffffd \
                    byr:2002 iyr:2010 cid:147 hgt:77in";

                let result = valid_passports_and_fields(&passport);

                assert_eq!(result, 0);
            }
        }
    }

    mod given_single_passport_with_hgt_missing_cm_or_in_suffix {
        use super::*;

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            #[test]
            fn then_it_returns_0() {
                let passport = "ecl:gry pid:860033327 eyr:2030 hcl:#fffffd \
                    byr:2002 iyr:2010 cid:147 hgt:76";

                let result = valid_passports_and_fields(&passport);

                assert_eq!(result, 0);
            }
        }
    }

    mod given_single_passport_with_hcl_missing_hash {
        use super::*;

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            #[test]
            fn then_it_returns_0() {
                let passport = "ecl:gry pid:860033327 eyr:2030 hcl:fffffd \
                    byr:2002 iyr:2010 cid:147 hgt:76in";

                let result = valid_passports_and_fields(&passport);

                assert_eq!(result, 0);
            }
        }
    }

    mod given_single_passport_with_hcl_missing_all_hex_chars {
        use super::*;

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            #[test]
            fn then_it_returns_0() {
                let passport = "ecl:gry pid:860033327 eyr:2030 hcl:#fffff \
                    byr:2002 iyr:2010 cid:147 hgt:76in";

                let result = valid_passports_and_fields(&passport);

                assert_eq!(result, 0);
            }
        }
    }

    mod given_single_passport_with_hcl_with_non_hex_char {
        use super::*;

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            #[test]
            fn then_it_returns_0() {
                let passport = "ecl:gry pid:860033327 eyr:2030 hcl:#fffffg \
                    byr:2002 iyr:2010 cid:147 hgt:76in";

                let result = valid_passports_and_fields(&passport);

                assert_eq!(result, 0);
            }
        }
    }

    mod given_single_passport_with_invalid_ecl {
        use super::*;

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            prop_compose! {
                fn arb_invalid_eye_colour()(iecl in "[a-z]{3}") -> String {
                    if vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&iecl.as_str()) {
                        return "aaa".to_string();
                    }
                    return iecl.to_string();
                }
            }

            proptest! {
                #[test]
                fn then_it_returns_0(invalid_ecl in arb_invalid_eye_colour()) {
                    let passport = format!("ecl:{} pid:860033327 eyr:2030 hcl:#fffffd \
                        byr:2002 iyr:2010 cid:147 hgt:76in", invalid_ecl);

                    let result = valid_passports_and_fields(&passport);

                    prop_assert_eq!(result, 0);
                }
            }
        }
    }

    mod given_single_passport_with_invalid_pid {
        use super::*;

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            prop_compose! {
                fn arb_invalid_passport_id()(invalid_pid in r"\S+") -> String {
                    if invalid_pid.contains(":") {
                        return "a".to_string();
                    }
                    if invalid_pid.len() == 9 && invalid_pid.chars().all(|c| c < '0' || c > '9') {
                        return "a".to_string();
                    }
                    return invalid_pid;
                }
            }

            proptest! {
                #[test]
                fn then_it_returns_0(invalid_pid in arb_invalid_passport_id()) {
                    let passport = format!("ecl:gry pid:{} eyr:2030 hcl:#fffffd \
                        byr:2002 iyr:2010 cid:147 hgt:76in", invalid_pid);

                    let result = valid_passports_and_fields(&passport);

                    prop_assert_eq!(result, 0);
                }
            }
        }
    }

    mod given_x_valid_passports {
        use super::*;

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            proptest! {
                #[test]
                fn then_it_returns_x((batch, len) in arb_batch_of_passports()) {
                    let result = valid_passports_and_fields(&batch);

                    prop_assert_eq!(result, len);
                }
            }
        }
    }

    mod given_example_batch_file_from_part_1_of_advent_of_code_2020 {
        use super::*;

        mod when_searching_for_valid_passports {
            use super::*;

            #[test]
            fn then_it_returns_2() {
                let batch = indoc! { "
                    ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
                    byr:1937 iyr:2017 cid:147 hgt:183cm

                    iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
                    hcl:#cfa07d byr:1929

                    hcl:#ae17e1 iyr:2013
                    eyr:2024
                    ecl:brn pid:760753108 byr:1931
                    hgt:179cm

                    hcl:#cfa07d eyr:2025 pid:166559648
                    iyr:2011 ecl:brn hgt:59in
                " };

                let result = valid_passports(&batch);

                assert_eq!(result, 2);
            }
        }
    }

    mod given_actual_batch_file_from_advent_of_code_2020 {
        use super::*;

        mod when_searching_for_valid_passports {
            use super::*;

            #[test]
            fn then_it_returns_226() -> anyhow::Result<()> {
                let batch = std::fs::read_to_string("tests/input.txt")?;

                let result = valid_passports(&batch);

                assert_eq!(result, 226);

                Ok(())
            }
        }

        mod when_searching_for_valid_passports_and_fields {
            use super::*;

            #[test]
            fn then_it_returns_160() -> anyhow::Result<()> {
                let batch = std::fs::read_to_string("tests/input.txt")?;

                let result = valid_passports_and_fields(&batch);

                assert_eq!(result, 160);

                Ok(())
            }
        }
    }
}
