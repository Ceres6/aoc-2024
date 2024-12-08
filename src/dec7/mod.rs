use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use rayon::prelude::*;
use anyhow::Context;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const PATH: &str = "src/dec7/input.txt";

pub(crate) fn feasible_equations() -> anyhow::Result<usize> {
    let file = File::open(PATH)?;
    let content = BufReader::new(file);

    let equations_sum = content
        .lines()
        .map_while(Result::ok)
        .par_bridge()
        .flat_map(get_target_and_operands)
        .map(|(target, operands)| {
            if Operation::iter()
                .any(|operation| recursive_operate_to_target(target, operands[0], operation, &operands[1..]))
            {
                target
            } else {
                0
            }
        })
        .sum();
    Ok(equations_sum)
}

fn get_target_and_operands(line: String) -> anyhow::Result<(usize, Vec<usize>)> {
    let mut split = line.split(": ");
    let target = split.next().context("No target")?.parse()?;
    let operand: Result<_, _> = split
        .next()
        .context("No operands")?
        .split(" ")
        .map(|n| n.parse::<usize>())
        .collect();
    Ok((target, operand?))
}

fn recursive_operate_to_target(
    target: usize,
    curr: usize,
    operation: Operation,
    rem: &[usize],
) -> bool {
    if rem.is_empty() {
        return false;
    }
    let new_curr = operation.operate(curr, rem[0]);
    if new_curr > target {
        return false;
    }
    if new_curr == target && rem.len() == 1 {
        return true;
    }
    let new_rem = &rem[1..];
    Operation::iter()
        .any(|operation| recursive_operate_to_target(target, new_curr, operation, new_rem))
}

#[derive(EnumIter)]
enum Operation {
    Sum,
    Multiplication,
    Union,
}

impl Operation {
    fn operate(self, a: usize, b: usize) -> usize {
        match self {
            Self::Multiplication => a * b,
            Self::Sum => a + b,
            Self::Union => a * 10usize.pow((b as f64).log10().floor() as u32 + 1) + b,
        }
    }
}
