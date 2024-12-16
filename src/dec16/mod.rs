use std::{fs::read_to_string, usize};

use anyhow::Context;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const PATH: &str = "src/dec16/input.txt";

const GRID_DIMENSION: usize = 141;

pub(crate) fn lowest_score_path() -> anyhow::Result<usize> {
    let content = read_to_string(PATH)?;
    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let mut visited = [[None; GRID_DIMENSION]; GRID_DIMENSION];
    traverse(grid.len() - 2, 1, &Direction::Right, 0, &grid, &mut visited);
    let min_points = visited[1][grid[1].len() - 2].context("No path found");
    let mut tiles = 0;
    best_path(
        1,
        grid[1].len() - 2,
        usize::MAX,
        visited[1][grid[1].len() - 2].unwrap() + 1,
        &mut tiles,
        &mut visited,
    );
    println!("tiles {tiles}");
    min_points
}

fn traverse(
    i: usize,
    j: usize,
    direction: &Direction,
    acc: usize,
    grid: &[Vec<char>],
    visited: &mut [[Option<usize>; GRID_DIMENSION]; GRID_DIMENSION],
) {
    if grid[i][j] == '#' {
        return;
    }
    if let Some(points) = visited[i][j] {
        if points <= acc {
            return;
        }
    }
    visited[i][j] = Some(acc);
    {
        let (i, j) = direction.move_forward(i, j);
        traverse(i, j, direction, acc + 1, grid, visited);
    }
    Direction::iter()
        .filter(|d| d != direction)
        .for_each(|direction| {
            let (i, j) = direction.move_forward(i, j);
            traverse(i, j, &direction, acc + 1001, grid, visited);
        });
}

fn best_path(
    i: usize,
    j: usize,
    previous_previous_points: usize,
    previous_points: usize,
    tiles: &mut usize,
    visited: &mut [[Option<usize>; GRID_DIMENSION]; GRID_DIMENSION],
) {
    if let Some(points) = visited[i][j] {
        if points + 1 != previous_points
            && points + 1001 != previous_points
            && (points + 2 != previous_previous_points || points != previous_points + 999)
        {
            return;
        }
        *tiles += 1;
        visited[i][j] = None;
        Direction::iter().for_each(|d| {
            let (i, j) = d.move_forward(i, j);
            best_path(i, j, previous_points, points, tiles, visited);
        });
    }
}

#[derive(PartialEq, EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn move_forward(&self, i: usize, j: usize) -> (usize, usize) {
        match self {
            Direction::Up => (i - 1, j),
            Direction::Down => (i + 1, j),
            Direction::Left => (i, j - 1),
            Direction::Right => (i, j + 1),
        }
    }
}
