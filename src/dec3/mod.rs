use std::{
    fs::read,
    ops::{Add, Mul},
};

use regex::{Matches, Regex};

const PATH: &str = "src/dec3/input.txt";

pub(crate) fn sum_mul(use_enabled: bool) -> anyhow::Result<u32> {
    let haystack = create_haystack()?;
    if use_enabled {
        parse_enabled(&haystack).map(parse_mul).try_sum()
    } else {
        parse_mul(&haystack)
    }
}

fn create_haystack() -> anyhow::Result<String> {
    let bytes = read(PATH)?;
    let haystack = String::from_utf8(bytes)?;
    Ok(haystack)
}

fn parse_mul(haystack: &str) -> anyhow::Result<u32> {
    let valid_pattern = Regex::new(r"mul\(\d{1,3},\d{1,3}\)")?;
    let number_pattern = Regex::new(r"\d+")?;
    valid_pattern
        .find_iter(haystack)
        .map(|m| number_pattern.find_iter(m.as_str()).try_product())
        .try_sum()
}

fn parse_enabled(contents: &str) -> impl Iterator<Item = &str> {
    contents
        .split("don't()")
        .enumerate()
        .flat_map(|(i, chunk)| {
            if i == 0 {
                // unnecessary skip to conform to types
                chunk.split("do()")
            } else {
                chunk.split("do()").skip(1)
            }
        })
}

trait TrySum<T> {
    fn try_sum(self) -> anyhow::Result<T>;
}

impl<I, T> TrySum<T> for I
where
    I: Iterator<Item = anyhow::Result<T>>,
    T: Default + Add<Output = T>,
{
    fn try_sum(mut self) -> anyhow::Result<T> {
        self.try_fold(T::default(), |acc, n| Ok(acc + n?))
    }
}

trait TryProduct<T> {
    fn try_product(self) -> anyhow::Result<T>;
}

impl<'a, T> TryProduct<T> for Matches<'a, 'a>
where
    T: Default + From<u32> + Mul<u32, Output = T>,
{
    fn try_product(mut self) -> anyhow::Result<T> {
        self.try_fold(T::from(1), |acc, n| Ok(acc * n.as_str().parse::<u32>()?))
    }
}

#[cfg(test)]
mod test {
    use crate::dec3::sum_mul;

    #[test]
    fn test_sum_mul_enabled() -> anyhow::Result<()> {
        Ok(assert_eq!(sum_mul(true)?, 113965544))
    }
}
