extern crate petgraph;

use itertools::Itertools;
use lazy_static::lazy_static;
use petgraph::prelude::*;
use petgraph::visit::{Bfs, Reversed};
use regex::Regex;
use std::cell::RefCell;
use std::collections::vec_deque::VecDeque;
use std::collections::HashSet;
use petgraph::algo::is_cyclic_directed;

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

        for rule in rules.split('\n').filter(|r| !r.trim().is_empty()) {
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
                    let is_many_bags = MANY_BAGS_REGEX.is_match(inner_bag);
                    let is_no_other_bags = inner_bag == "no other bag";

                    if is_many_bags {
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
                    } else if is_no_other_bags {
                        // do nothing
                    } else {
                        todo!()
                    }
                }
            } else {
                todo!()
            }
        }

        if is_cyclic_directed(&result.bag_relationships) {
            todo!()
        }

        Ok(result)
    }
}

impl<'a> BagRules<'a> {
    pub fn bags_eventually_containing(&self, bag: &'a str) -> BagsEventuallyContaining {
        BagsEventuallyContaining::new(&self.bag_relationships, bag)
    }

    pub fn individual_bags_contained_by(&self, bag: &'a str) -> u32 {
        let mut remaining = VecDeque::new();
        remaining.push_back(bag);
        let mut result = 0;

        // Do a breadth-first-style traversal of the graph.
        // This traversal assumes the graph is non-cyclic, which is
        // guaranteed by BagRules::new.
        while !remaining.is_empty() {
            let outer_bag = remaining.pop_front().unwrap();
            let edges = self.bag_relationships.edges(outer_bag);
            for (_, inner_bag, &num_inner_bags_in_outer_bag) in edges {
                result += num_inner_bags_in_outer_bag;
                for _ in 0..num_inner_bags_in_outer_bag {
                    remaining.push_back(inner_bag);
                }
            }
        }

        result
    }
}

pub struct BagsEventuallyContaining<'a, 'b> {
    reversed_bag_relationships: Reversed<&'b DiGraphMap<&'a str, u32>>,
    bfs: Bfs<&'a str, HashSet<&'a str>>,
    skipped: RefCell<bool>,
}

impl<'a, 'b> BagsEventuallyContaining<'a, 'b> {
    fn new(
        bag_relationships: &'b DiGraphMap<&'a str, u32>,
        bag: &'a str,
    ) -> BagsEventuallyContaining<'a, 'b> {
        let reversed_bag_relationships = Reversed(bag_relationships);
        let bfs = Bfs::new(reversed_bag_relationships, bag);
        let skipped = RefCell::new(false);

        BagsEventuallyContaining {
            reversed_bag_relationships,
            bfs,
            skipped,
        }
    }
}

impl<'a, 'b> Iterator for BagsEventuallyContaining<'a, 'b> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if !(*self.skipped.borrow()) {
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

        mod when_finding_individual_bags_contained_by_bright_white_bags {
            use super::*;

            #[test]
            fn then_result_is_1_bag() -> Result<(), Box<dyn Error>> {
                let rules = "bright white bags contain 1 shiny gold bag.";

                let bag_rules = BagRules::from(rules)?;
                let result = bag_rules.individual_bags_contained_by("bright white bag");

                assert_eq!(result, 1);

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

        mod when_finding_individual_bags_contained_by_bright_white_bags {
            use super::*;

            #[test]
            fn then_result_is_2_bags() -> Result<(), Box<dyn Error>> {
                let rules = "bright white bags contain 2 shiny gold bag.";

                let bag_rules = BagRules::from(rules)?;
                let result = bag_rules.individual_bags_contained_by("bright white bag");

                assert_eq!(result, 2);

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

        mod when_finding_individual_bags_contained_by_muted_yellow_bags {
            use super::*;

            #[test]
            fn then_result_is_11_bags() -> Result<(), Box<dyn Error>> {
                let rules = "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.";

                let bag_rules = BagRules::from(rules)?;
                let result = bag_rules.individual_bags_contained_by("muted yellow bag");

                assert_eq!(result, 11);

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

            mod when_finding_individual_bags_contained_by_light_red_bags {
                use super::*;

                #[test]
                fn then_result_is_2_bags() -> Result<(), Box<dyn Error>> {
                    let rules = indoc! { "
                        light red bags contain 1 bright white bag.
                        bright white bags contain 1 shiny gold bag.
                    " };

                    let bag_rules = BagRules::from(rules)?;
                    let result = bag_rules.individual_bags_contained_by("light red bag");

                    assert_eq!(result, 2);

                    Ok(())
                }
            }
        }
    }

    mod given_faded_blue_bags_contain_no_other_bags {
        use super::*;

        mod when_finding_bags_that_eventually_contain_shiny_gold_bags {
            use super::*;

            #[test]
            fn then_result_is_empty() -> Result<(), Box<dyn Error>> {
                let rules = "faded blue bags contain no other bags.";

                let bag_rules = BagRules::from(rules)?;
                let result = bag_rules.bags_eventually_containing("shiny gold bag");

                assert!(result.collect::<Vec<_>>().is_empty());

                Ok(())
            }
        }
    }

    mod given_shiny_gold_bags_contain_no_other_bags {
        use super::*;

        mod when_finding_bags_that_eventually_contain_shiny_gold_bags {
            use super::*;

            #[test]
            fn then_result_is_empty() -> Result<(), Box<dyn Error>> {
                let rules = "shiny gold bags contain no other bags.";

                let bag_rules = BagRules::from(rules)?;
                let result = bag_rules.bags_eventually_containing("shiny gold bag");

                assert!(result.collect::<Vec<_>>().is_empty());

                Ok(())
            }
        }

        mod when_finding_individual_bags_contained_by_shiny_gold_bags {
            use super::*;

            #[test]
            fn then_result_is_0() -> Result<(), Box<dyn Error>> {
                let rules = "shiny gold bags contain no other bags.";

                let bag_rules = BagRules::from(rules)?;
                let result = bag_rules.individual_bags_contained_by("shiny gold bag");

                assert_eq!(result, 0);

                Ok(())
            }
        }
    }

    mod given_shiny_gold_bags_contain_2_dark_red_bags {
        use super::*;

        mod and_dark_red_bags_contain_2_dark_orange_bags {
            use super::*;

            mod when_finding_individual_bags_contained_by_shiny_gold_bags {
                use super::*;

                #[test]
                fn then_result_is_6() -> Result<(), Box<dyn Error>> {
                    let rules = indoc! { "
                        shiny gold bags contain 2 dark red bags.
                        dark red bags contain 2 dark orange bags.
                        dark orange bags contain no other bags.
                    " };

                    let bag_rules = BagRules::from(rules)?;
                    let result = bag_rules.individual_bags_contained_by("shiny gold bag");

                    assert_eq!(result, 6);

                    Ok(())
                }
            }

            mod and_dark_orange_bags_contain_2_dark_yellow_bags {
                use super::*;

                mod when_finding_individual_bags_contained_by_shiny_gold_bags {
                    use super::*;

                    #[test]
                    fn then_result_is_14() -> Result<(), Box<dyn Error>> {
                        let rules = indoc! { "
                            shiny gold bags contain 2 dark red bags.
                            dark red bags contain 2 dark orange bags.
                            dark orange bags contain 2 dark yellow bags.
                            dark yellow bags contain no other bags.
                        " };

                        let bag_rules = BagRules::from(rules)?;
                        let result = bag_rules.individual_bags_contained_by("shiny gold bag");

                        assert_eq!(result, 14);

                        Ok(())
                    }
                }
            }
        }
    }
}
