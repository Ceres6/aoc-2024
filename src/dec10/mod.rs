use std::fs::read_to_string;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const PATH: &str = "src/dec10/input.txt";

pub(crate) fn sum_trailhead_scores(unique_end: bool) -> anyhow::Result<usize> {
    let contents = read_to_string(PATH)?;
    let grid: Vec<Vec<usize>> = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap_or_default() as usize)
                .collect()
        })
        .collect();
    let total_score = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, &cell)| if cell == 0 { Some(j) } else { None })
                .map(|j| count_trails(&grid, i, j, unique_end))
                .sum::<usize>()
        })
        .sum();
    Ok(total_score)
}

fn count_trails(grid: &[Vec<usize>], i0: usize, j0: usize, unique_end: bool) -> usize {
    if unique_end {
        let mut visited = [[false; 19]; 19];
        traverse_trails(grid, 0, i0, j0, i0, j0, &mut Some(&mut visited))
    } else {
        traverse_trails(grid, 0, i0, j0, i0, j0, &mut None)
    }
}

fn traverse_trails(
    grid: &[Vec<usize>],
    next_altitude: usize,
    i0: usize,
    j0: usize,
    i: usize,
    j: usize,
    visited: &mut Option<&mut [[bool; 19]; 19]>,
) -> usize {
    if grid[i][j] != next_altitude {
        return 0;
    }
    let (i_offset, j_offset) = (i + 9 - i0, j + 9 - j0);
    if let Some(visited) = visited {

        if visited[i_offset][j_offset] {
            return 0;
        }
        visited[i_offset][j_offset] = true;
    }
    if next_altitude == 9 {
        return 1;
    }
    Direction::iter()
        .map(|direction| {
            if let Some((i, j)) = direction.move_forward(grid, i, j) {
                traverse_trails(grid, next_altitude + 1, i0, j0, i, j, visited)
            } else {
                0
            }
        })
        .sum()
}

#[derive(EnumIter, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn move_forward(self, grid: &[Vec<usize>], i: usize, j: usize) -> Option<(usize, usize)> {
        let height = grid.len();
        let width = grid[0].len();
        let result = match self {
            Direction::Up => (i.checked_sub(1), Some(j)),
            Direction::Down => (Some(i + 1), Some(j)),
            Direction::Left => (Some(i), j.checked_sub(1)),
            Direction::Right => (Some(i), Some(j + 1)),
        };
        match result {
            (Some(i), Some(j)) if i < height && j < width => Some((i, j)),
            _ => None,
        }
    }
}