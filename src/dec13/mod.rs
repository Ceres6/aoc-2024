// m1*a + n1*b = x
// m2*a + n2*b = y

use std::{fs::File, io::{BufReader, BufRead}};

use itertools::Itertools;
use regex::Regex;

const PATH: &str = "src/dec13/input.txt";

pub(crate) fn minimum_tokens(offset: isize) -> anyhow::Result<isize> {
  let file = File::open(PATH)?;
  let contents = BufReader::new(file);
  let number = contents.lines().chunks(4).into_iter().map(|lines| {
    let mut lines = lines.map(|line| line.unwrap_or_default());
    let button_a = lines.next().unwrap();
    let (m1, m2) = (button_a[12..14].parse().unwrap(), button_a[18..20].parse().unwrap());
    let button_b = lines.next().unwrap();
    let (n1, n2) = (button_b[12..14].parse().unwrap(), button_b[18..20].parse().unwrap());
    let prize = lines.next().unwrap();
    let re = Regex::new(r"(\d+)").unwrap();
    let mut caps = re.captures_iter(&prize);
    let (x, y): (isize, isize) = (caps.next().unwrap()[1].parse().unwrap(), caps.next().unwrap()[1].parse().unwrap());
    let system = EquationSystem::new(m1, n1, m2, n2, x + offset, y + offset);
    solve_equation_system(system)
  }).sum();
  Ok(number)
}
fn solve_equation_system(system: EquationSystem) -> isize {
    match system.compatibility() {
        Compatibility::Determinate => calculate_unique_price(system),
        Compatibility::Incompatible => 0,
        Compatibility::Indeterminate => calculate_optimal_solution(),
    }
}

fn calculate_unique_price(system: EquationSystem) -> isize {
    let (a, b) = system.calculate_unique_integer_solution().unwrap_or_default();
    3 * a + b
}

fn calculate_optimal_solution() -> isize {
    todo!()
}

struct EquationSystem {
    m1: isize,
    n1: isize,
    m2: isize,
    n2: isize,
    x: isize,
    y: isize,
}

impl EquationSystem {
    fn new(m1: isize, n1: isize, m2: isize, n2: isize, x: isize, y: isize) -> Self {
        EquationSystem {
            m1,
            n1,
            m2,
            n2,
            x,
            y,
        }
    }

    fn compatibility(&self) -> Compatibility {
        let EquationSystem {
            m1,
            n1,
            m2,
            n2,
            x,
            y,
        } = self;
        let (numerator, denominator) = (m2 * x - m1 * y, n1 * m2 - n2 * m1);
        if numerator == 0 && denominator == 0 {
            Compatibility::Indeterminate
        } else if denominator == 0 {
            Compatibility::Incompatible
        } else {
            Compatibility::Determinate
        }
    }

    fn calculate_unique_integer_solution(&self) -> Option<(isize, isize)> {
        let EquationSystem {
            m1,
            n1,
            m2,
            n2,
            x,
            y,
        } = self;

        if (m2 * x - m1 * y) % (n1 * m2 - n2 * m1) != 0 {
            return None;
        }
        let b = (m2 * x - m1 * y) / (n1 * m2 - n2 * m1);
        if b < 0 || (x - n1 * b) % m1 != 0 {
            return None;
        }
        let a = (x - n1 * b) / m1;
        if a < 0 {
          return None
        }
        Some((a, b))
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Compatibility {
    Determinate,
    Indeterminate,
    Incompatible,
}

#[cfg(test)]
mod test {
use super::*;

  #[test]
  fn test_compatible_integer_solution() {
    let system = EquationSystem::new(94, 22, 34, 67, 8400, 5400);
    assert_eq!(system.compatibility(), Compatibility::Determinate);
    assert_eq!(system.calculate_unique_integer_solution(), Some((80, 40)));
    
  }
}