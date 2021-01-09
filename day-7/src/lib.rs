extern crate petgraph;

use itertools::Itertools;
use lazy_static::lazy_static;
use petgraph::prelude::*;
use petgraph::visit::{Bfs, Reversed};
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashSet;

lazy_static! {
    static ref BAG_REGEX: Regex = Regex::new("^[a-z]+ [a-z]+ bags?$").unwrap();
    static ref MANY_BAGS_REGEX: Regex = Regex::new("^[1-9][0-9]* [a-z]+ [a-z]+ bag(s)?$").unwrap();
}

pub struct BagRules<'a> {
    bag_relationships: DiGraphMap<&'a str, u32>,
}

impl<'a> BagRules<'a> {
    pub fn from(rules: &'a str) -> anyhow::Result<BagRules<'a>> {
        let mut result = BagRules {
            bag_relationships: DiGraphMap::new(),
        };

        for rule in rules.split("\n").filter(|r| !r.trim().is_empty()) {
            if let Some((outer_bag, inner_bags)) = rule.splitn(2, " contain ").collect_tuple() {
                if !BAG_REGEX.is_match(outer_bag) {
                    todo!()
                }
                let outer_bag = outer_bag.trim_end_matches('s');

                for inner_bag in inner_bags
                    .trim_end_matches('.')
                    .split(", ")
                    .map(|i| i.trim_end_matches('s'))
                {
                    if !MANY_BAGS_REGEX.is_match(inner_bag) {
                        todo!()
                    }

                    if let Some((num, inner_bag)) = inner_bag.splitn(2, ' ').collect_tuple() {
                        let num = num.parse::<u32>();
                        if num.is_err() {
                            todo!()
                        }

                        result
                            .bag_relationships
                            .add_edge(outer_bag, inner_bag, num.unwrap());
                    } else {
                        todo!()
                    }
                }
            } else {
                todo!()
            }
        }

        return Ok(result);
    }
}

impl<'a> BagRules<'a> {
    pub fn bags_eventually_containing(&self, bag: &'a str) -> BagIterator {
        let bag_relationships = &self.bag_relationships;
        BagIterator::new(bag_relationships, bag)
    }
}

pub struct BagIterator<'a, 'b> {
    reversed_bag_relationships: Reversed<&'b DiGraphMap<&'a str, u32>>,
    bfs: Bfs<&'a str, HashSet<&'a str>>,
    skipped: RefCell<bool>,
}

impl<'a, 'b> BagIterator<'a, 'b> {
    fn new(
        bag_relationships: &'b DiGraphMap<&'a str, u32>,
        bag: &'a str,
    ) -> BagIterator<'a, 'b> {
        let reversed = Reversed(bag_relationships);
        let bfs = Bfs::new(reversed, bag);
        BagIterator {
            reversed_bag_relationships: reversed,
            bfs,
            skipped: RefCell::new(false),
        }
    }
}

impl<'a, 'b> Iterator for BagIterator<'a, 'b> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if *self.skipped.borrow() == false {
            let _ = self.bfs.next(&self.reversed_bag_relationships);
            *self.skipped.borrow_mut() = true;
        }
        self.bfs.next(&self.reversed_bag_relationships)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use std::collections::HashMap;
    use std::error::Error;
    use std::hash::Hash;

    fn eq_in_any_order<T>(a: &[T], b: &[T]) -> bool
    where
        T: Eq + Hash,
    {
        fn count<T>(items: &[T]) -> HashMap<&T, usize>
        where
            T: Eq + Hash,
        {
            let mut cnt = HashMap::new();
            for i in items {
                *cnt.entry(i).or_insert(0) += 1
            }
            cnt
        }

        count(a) == count(b)
    }

    mod given_bright_white_bags_contain_1_shiny_gold_bag {
        use super::*;

        mod when_finding_bags_that_eventually_contain_shiny_gold_bags {
            use super::*;

            #[test]
            fn then_result_is_bright_white_bag() -> Result<(), Box<dyn Error>> {
                let rules = "bright white bags contain 1 shiny gold bag.";

                let bag_rules = BagRules::from(rules)?;
                let result = bag_rules
                    .bags_eventually_containing("shiny gold bag")
                    .collect::<Vec<_>>();

                assert_eq!(result, &["bright white bag"]);

                Ok(())
            }
        }

        mod and_vivid_black_bags_contain_1_shiny_gold_bag {
            use super::*;

            mod when_finding_bags_that_eventually_contain_shiny_gold_bags {
                use super::*;

                #[test]
                fn then_result_is_bright_white_bag_and_vivid_black_bag(
                ) -> Result<(), Box<dyn Error>> {
                    let rules = indoc! { "
                        bright white bags contain 1 shiny gold bag.
                        vivid black bags contain 1 shiny gold bag.
                    " };

                    let bag_rules = BagRules::from(rules)?;
                    let result = bag_rules
                        .bags_eventually_containing("shiny gold bag")
                        .collect::<Vec<_>>();

                    assert!(eq_in_any_order(
                        result.as_slice(),
                        &["bright white bag", "vivid black bag"]
                    ));

                    Ok(())
                }
            }
        }
    }

    mod given_bright_white_bags_contain_2_shiny_gold_bags {
        use super::*;

        mod when_finding_bags_that_eventually_contain_shiny_gold_bags {
            use super::*;

            #[test]
            fn then_result_is_bright_white_bag() -> Result<(), Box<dyn Error>> {
                let rules = "bright white bags contain 2 shiny gold bags.";

                let bag_rules = BagRules::from(rules)?;
                let result = bag_rules.bags_eventually_containing("shiny gold bag");

                assert_eq!(result.collect::<Vec<_>>(), &["bright white bag"]);

                Ok(())
            }
        }
    }

    mod given_muted_yellow_bags_contain_2_shiny_gold_bags_and_9_faded_blue_bags {
        use super::*;

        mod when_finding_bags_that_eventually_contain_shiny_gold_bags {
            use super::*;

            #[test]
            fn then_result_is_muted_yellow_bag() -> Result<(), Box<dyn Error>> {
                let rules = "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.";

                let bag_rules = BagRules::from(rules)?;
                let result = bag_rules.bags_eventually_containing("shiny gold bag");

                assert_eq!(result.collect::<Vec<_>>(), &["muted yellow bag"]);

                Ok(())
            }
        }
    }

    mod given_light_red_bags_contain_1_bright_white_bag {
        use super::*;

        mod and_bright_white_bags_contain_1_shiny_gold_bag {
            use super::*;

            mod when_finding_bags_that_eventually_contain_shiny_gold_bags {
                use super::*;

                #[test]
                fn then_result_is_light_red_bag_and_muted_yellow_bag() -> Result<(), Box<dyn Error>>
                {
                    let rules = indoc! { "
                        light red bags contain 1 bright white bag.
                        bright white bags contain 1 shiny gold bag.
                    " };

                    let bag_rules = BagRules::from(rules)?;
                    let result = bag_rules.bags_eventually_containing("shiny gold bag");

                    assert!(eq_in_any_order(
                        result.collect::<Vec<_>>().as_slice(),
                        &["light red bag", "bright white bag"]
                    ));

                    Ok(())
                }
            }
        }
    }
}
