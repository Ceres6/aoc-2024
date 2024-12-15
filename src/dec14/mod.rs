use std::{
    fs::File,
    io::{BufRead, BufReader, Write}, path::Path,
};

use anyhow::Error;
use regex::Regex;
use rayon::prelude::*;

const PATH: &str = "src/dec14/input.txt";

pub(crate) fn safety_factor() -> anyhow::Result<isize> {
    let file = File::open(PATH)?;
    let contents = BufReader::new(file);
    contents
        .lines()
        .try_fold([0, 0, 0, 0], |mut acc, line| {
            let mut robot = Robot::from(&line?[..]);
            robot.move_times(100);
            if let Some(quadrant) = robot.get_cuadrant() {
                acc[quadrant] += 1;
            }
            Ok::<[isize; 4], Error>(acc)
        })
        .map(|result| result.iter().product())
}

fn render_tree(moves: isize) -> anyhow::Result<()> {
    let mut grid = [['.'; 101]; 103];
    let file = File::open(PATH)?;
    let contents = BufReader::new(file);
    contents.lines().try_for_each(|line| {
        let mut robot = Robot::from(&line?[..]);
        robot.move_times(moves);
        let Robot { x, y , ..} = robot;
        grid[y as usize][x as usize] = '#';
        Ok::<(), anyhow::Error>(())
    })?;
    if grid.iter().any(|line| line.iter().collect::<String>().contains("########")) {} else { return Ok(())};
    let filepath = format!("src/dec14/outputs/{}.txt", moves);
    let path = Path::new(&filepath[..]);
    let mut file = File::create(path)?;

    // Iterate through each inner vector (line)
    for line in grid {
        // Convert the line of chars to a string and write it to the file
        let line_string: String = line.iter().collect();
        writeln!(file, "{}", line_string)?;
    }
    Ok(())
}
#[derive(Debug, PartialEq, Clone)]
struct Robot {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

impl Robot {
    fn move_times(&mut self, times: isize) {
        let Robot { x, y, vx, vy } = self;
        *x = (((*x + *vx * times) % 101) + 101) % 101;
        *y = (((*y + *vy * times) % 103) + 103) % 103;
    }

    fn get_cuadrant(&self) -> Option<usize> {
        match (self.x, self.y) {
            (x, y) if x < 50 && y < 51 => Some(0),
            (x, y) if x > 50 && y < 51 => Some(1),
            (x, y) if x < 50 && y > 51 => Some(2),
            (x, y) if x > 50 && y > 51 => Some(3),
            _ => None,
        }
    }
}
impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"p=(?<x>\d+),(?<y>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();
        let caps = re.captures(value).unwrap();
        Robot {
            x: caps["x"].parse::<isize>().unwrap(),
            y: caps["y"].parse::<isize>().unwrap(),
            vx: caps["vx"].parse::<isize>().unwrap(),
            vy: caps["vy"].parse::<isize>().unwrap(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let haystack = "p=74,51 v=36,-94";
        let robot = Robot::from(haystack);
        // (5000..10000).par_bridge().for_each(|i| {render_tree(i).unwrap();});
        assert_eq!(
            robot,
            Robot {
                x: 74,
                y: 51,
                vx: 36,
                vy: -94
            }
        );
    }
}
