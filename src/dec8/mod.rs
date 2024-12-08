use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader},
};

use derive_more::derive::Add;
use itertools::Itertools;
use num_integer::gcd;

const PATH: &str = "src/dec8/input.txt";

pub(crate) fn count_antinodes() -> anyhow::Result<usize> {
    let file = File::open(PATH)?;
    let content = BufReader::new(file);
    let mut antenna_positions: HashMap<char, Vec<Point>> = HashMap::new();
    let mut antinode_positions = HashSet::new();

    content.lines().enumerate().try_for_each(|(i, line)| {
        line?.chars().enumerate().for_each(|(j, char)| {
            if char != '.' {
                antenna_positions
                    .entry(char)
                    .and_modify(|positions| positions.push(Point(i, j)))
                    .or_insert(vec![Point(i, j)]);
            }
        });
        Ok::<(), io::Error>(())
    })?;

    antenna_positions.values().for_each(|positions| {
        positions.iter().combinations(2).for_each(|combination| {
            let (first, second) = (combination[0], combination[1]);
            if let Some(antinode) = (*first + *first).try_sub(second) {
                antinode_positions.insert(antinode);
            }
            if let Some(antinode) = (*second + *second).try_sub(first) {
                antinode_positions.insert(antinode);
            }
        })
    });
    Ok(antinode_positions.len())
}

pub(crate) fn count_resonating_antinodes() -> anyhow::Result<usize> {
    let file = File::open(PATH)?;
    let content = BufReader::new(file);
    let mut antenna_positions: HashMap<char, Vec<Point>> = HashMap::new();
    let mut antinode_positions = HashSet::new();

    content.lines().enumerate().try_for_each(|(i, line)| {
        line?.chars().enumerate().for_each(|(j, char)| {
            if char != '.' {
                antenna_positions
                    .entry(char)
                    .and_modify(|positions| positions.push(Point(i, j)))
                    .or_insert(vec![Point(i, j)]);
            }
        });
        Ok::<(), io::Error>(())
    })?;

    antenna_positions.values().for_each(|positions| {
        positions.iter().combinations(2).for_each(|combination| {
            let (first, second) = (combination[0], combination[1]);
            antinode_positions.extend(first.calculate_line_points(second));
        })
    });
    Ok(antinode_positions.len())
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Add, Debug)]
struct Point(usize, usize);

impl Point {
    fn try_sub(&self, rhs: &Self) -> Option<Self> {
        match (self.0.checked_sub(rhs.0), self.1.checked_sub(rhs.1)) {
            (Some(i), Some(j)) if i < 50 && j < 50 => Some(Point(i, j)),
            _ => None,
        }
    }

    fn calculate_line_points(&self, rhs: &Self) -> Vec<Self> {
        let (i_diff, j_diff) = (
            self.0 as isize - rhs.0 as isize,
            self.1 as isize - rhs.1 as isize,
        );
        let denominator = gcd(i_diff, j_diff);
        let (i_min, j_min) = (i_diff / denominator, j_diff / denominator);
        let mut points = vec![];
        let (mut new_i, mut new_j) = (self.0 as isize, self.1 as isize);
        let limits = 0..50;
        while limits.contains(&new_i) && limits.contains(&new_j) {
            points.push(Point(new_i as usize, new_j as usize));
            (new_i, new_j) = (new_i - i_min, new_j - j_min);
        };
        (new_i, new_j) = (self.0 as isize + i_min, self.1 as isize + j_min);
        while limits.contains(&new_i) && limits.contains(&new_j) {
          points.push(Point(new_i as usize, new_j as usize));
          (new_i, new_j) = (new_i + i_min, new_j + j_min);
      };
      points
    }
}
