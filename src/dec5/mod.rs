use anyhow::Context;
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

const PATH: &str = "src/dec5/input.txt";

#[derive(Default)]
struct Order {
    predecessors: HashSet<usize>,
    successors: HashSet<usize>,
}

pub(crate) fn middle_page_ordered_updates_sum() -> anyhow::Result<usize> {
    let content = read_to_string(PATH)?;
    let mut split_content = content.split("\n\n");
    let (raw_rules, updates) = (
        split_content.next().context("No rules")?,
        split_content.next().context("No updates")?,
    );
    let rules = create_rules(raw_rules);

    let result = updates
        .lines()
        .map(|l| {
            l.split(",")
                .map(|w| w.parse::<usize>().unwrap_or_default())
                .collect::<Vec<usize>>()
        })
        .map(|u| {
            let ordered = rules.clone().all(|[first, second]| {
                if let Some(first_pos) = u.iter().position(|&x| x == first) {
                    if let Some(second_pos) = u.iter().position(|&x| x == second) {
                        return first_pos < second_pos;
                    }
                };
                true
            });
            if ordered {
                let mid = u.len() / 2;
                return u[mid];
            }
            0
        })
        .sum();

    Ok(result)
}

pub(crate) fn middle_page_sum(ordered: bool) -> anyhow::Result<usize> {
    let content = read_to_string(PATH)?;
    let mut split_content = content.split("\n\n");
    let (raw_rules, raw_updates) = (
        split_content.next().context("No rules")?,
        split_content.next().context("No updates")?,
    );

    let rules = traverse_rules(create_rules(raw_rules));
    let updates = create_updates(raw_updates);
    let result = updates
        .map(|update| {
            let ordered_update = create_order(filter_updates(&rules, &update));
            let is_ordered = ordered_update == update;
            if is_ordered == ordered {
                let mid = ordered_update.len() / 2;
                ordered_update[mid]
            } else {
                0
            }
        })
        .sum();
    Ok(result)
}

fn create_rules<'a>(raw_rules: &'a str) -> impl Iterator<Item = [usize; 2]> + 'a + Clone {
    raw_rules.lines().map(|l| {
        let mut split = l.split("|");
        [
            split
                .next()
                .unwrap_or_default()
                .parse::<usize>()
                .unwrap_or_default(),
            split.next().unwrap_or_default().parse().unwrap_or_default(),
        ]
    })
}

fn create_updates<'a>(raw_updates: &'a str) -> impl Iterator<Item = std::vec::Vec<usize>> + 'a {
    raw_updates.lines().map(|l| {
        l.split(",")
            .map(|w| w.parse::<usize>().unwrap_or_default())
            .collect::<Vec<usize>>()
    })
}

fn traverse_rules<I: Iterator<Item = [usize; 2]>>(rules: I) -> HashMap<usize, Order> {
    let mut order: HashMap<usize, Order> = HashMap::new();
    rules.for_each(|[first, second]| {
        order.entry(first).or_default().successors.insert(second);
        order.entry(second).or_default().predecessors.insert(first);
    });
    order
}

fn filter_updates(order: &HashMap<usize, Order>, update: &[usize]) -> HashMap<usize, usize> {
    let mut precedences: HashMap<usize, usize> = HashMap::new();
    update.iter().for_each(|&page| {
        let default_order = Order::default();
        let entry = order.get(&page).unwrap_or(&default_order);
        precedences.entry(page).or_default();
        entry.predecessors.iter().for_each(|predecessor| {
            if precedences.contains_key(predecessor) {
                precedences.entry(*predecessor).and_modify(|p| *p += 1);
            }
        });
        entry.successors.iter().for_each(|successor| {
            if precedences.contains_key(successor) {
                precedences.entry(page).and_modify(|p| *p += 1);
            }
        });
    });
    precedences
}

fn create_order(precedences: HashMap<usize, usize>) -> Vec<usize> {
    let length = precedences.len();
    let mut order = vec![0; length];
    precedences
        .iter()
        .for_each(|(&number, &precedence)| order[length - 1 - precedence] = number);
    order
}

fn hash_rules<I: Iterator<Item = [usize; 2]>>(rules: I) -> HashMap<usize, HashSet<usize>> {
    let mut hashed_rules: HashMap<usize, HashSet<_>> = HashMap::new();
    rules.for_each(|[first, second]| {
        hashed_rules.entry(first).or_default().insert(second);
    });
    hashed_rules
}

pub(crate) fn page_custom_order(ordered: bool) -> anyhow::Result<usize> {
    let content = read_to_string(PATH)?;
    let mut split_content = content.split("\n\n");
    let (raw_rules, raw_updates) = (
        split_content.next().context("No rules")?,
        split_content.next().context("No updates")?,
    );
    let rules = hash_rules(create_rules(raw_rules));
    let updates = create_updates(raw_updates);
    let result = updates
        .map(|update| {
            let sorted_update: Vec<usize> = update.iter().sorted_unstable_by(|this, other| {
                if let Some(self_entry) = rules.get(this) {
                    if self_entry.contains(other) {
                        return Ordering::Less;
                    }
                }

                if let Some(other_entry) = rules.get(other) {
                    if other_entry.contains(this) {
                        return Ordering::Greater;
                    }
                }
                Ordering::Equal
            }).copied().collect();
            let is_ordered = sorted_update == update;
            if is_ordered == ordered {
              let mid = sorted_update.len() / 2;
              return sorted_update[mid];
            }
            0
        })
        .sum();

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ordered() -> anyhow::Result<()> {
        let expected = 5374;
        assert_eq!(middle_page_ordered_updates_sum()?, expected);
        assert_eq!(middle_page_sum(true)?, expected);
        assert_eq!(page_custom_order(true)?, expected);
        Ok(())
    }

    #[test]
    fn test_unordered() -> anyhow::Result<()> {
        let expected = 4260;
        assert_eq!(middle_page_sum(false)?, expected);
        assert_eq!(page_custom_order(false)?, expected);
        Ok(())
    }
}
