use std::{collections::HashMap, fs::read_to_string};

use anyhow::Ok;

const PATH: &str = "src/dec11/input.txt";

pub(crate) fn count_stones() -> anyhow::Result<usize> {
    let starting_stones: Vec<usize> = read_to_string(PATH)?
        .split(" ")
        .map(|n| n.parse().unwrap())
        .collect();
    let mut cache = HashMap::new();
    starting_stones
        .iter()
        .map(|&stone| count_stone(stone, 75, &mut cache))
        .map(Ok)
        .sum()
}

fn count_stone(
    start: usize,
    iterations: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if iterations == 0 {
        return 1;
    }
    if let Some(&count) = cache.get(&(start, iterations)) {
        return count;
    }
    let count = if start == 0 {
        count_stone(1, iterations - 1, cache)
    } else {
        let num_digits = (start as f64).log10().floor() as u32 + 1;
        if num_digits % 2 == 0 {
            let power = 10usize.pow(num_digits / 2);
            count_stone(start / power, iterations - 1, cache)
                + count_stone(start % power, iterations - 1, cache)
        } else {
            count_stone(start * 2024, iterations - 1, cache)
        }
    };
    cache.insert((start, iterations), count);
    count
}
