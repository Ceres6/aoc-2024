use std::{collections::VecDeque, fs::read_to_string};

use derive_more::derive::{Add, AddAssign};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const PATH: &str = "src/dec12/input.txt";

pub(crate) fn fence_price(bulk_discount: bool) -> anyhow::Result<usize> {
    let grid: Vec<Vec<char>> = read_to_string(PATH)?
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let price = grid.iter().enumerate().fold(0, |acc, (i, row)| {
        acc + row.iter().enumerate().fold(0, |acc, (j, region)| {
            if visited[i][j] {
                return acc;
            }
            acc + calculate_region(&grid, region, i, j, &mut visited, bulk_discount)
        })
    });
    Ok(price)
}

fn calculate_region(
    grid: &[Vec<char>],
    region: &char,
    i: usize,
    j: usize,
    global_visited: &mut Vec<Vec<bool>>,
    bulk_discount: bool,
) -> usize {
    if bulk_discount {
        let mut local_visited = vec![vec![None; global_visited[0].len()]; global_visited.len()];
        let mut queue = VecDeque::from_iter(Direction::iter().map(|d| (i, j, d)));
        let mut total_geometry = Geometry::in_region();
        global_visited[i][j] = true;
        local_visited[i][j] = Some([false;4]);
        while let Some((i, j, d)) = queue.pop_front() {
            total_geometry += traverse_region_with_bulk_discount(
                grid,
                region,
                Cell::new(i, j),
                d,
                global_visited,
                &mut local_visited,
                &mut queue,
            );
        }
        total_geometry.price()
    } else {
        let mut local_visited = vec![vec![false; global_visited[0].len()]; global_visited.len()];
        traverse_region(grid, region, i, j, global_visited, &mut local_visited).price()
    }
}

fn traverse_region(
    grid: &[Vec<char>],
    region: &char,
    i: usize,
    j: usize,
    global_visited: &mut Vec<Vec<bool>>,
    local_visited: &mut Vec<Vec<bool>>,
) -> Geometry {
    if local_visited[i][j] {
        return Geometry::empty();
    }
    if *region != grid[i][j] {
        return Geometry::out_of_region();
    }
    global_visited[i][j] = true;
    local_visited[i][j] = true;
    Direction::iter().fold(Geometry::empty(), |acc, direction| {
        if let Some((i, j)) = direction.move_forward(grid, i, j) {
            acc + traverse_region(grid, region, i, j, global_visited, local_visited)
        } else {
            acc + Geometry::out_of_region()
        }
    }) + Geometry::in_region()
}

fn traverse_region_with_bulk_discount(
    grid: &[Vec<char>],
    region: &char,
    Cell { i: i0, j: j0 }: Cell,
    direction: Direction,
    global_visited: &mut [Vec<bool>],
    local_visited: &mut [Vec<Option<[bool; 4]>>],
    queue: &mut VecDeque<(usize, usize, Direction)>,
) -> Geometry {
    if let Some((i, j)) = direction.move_forward(grid, i0, j0) {
        if local_visited[i][j].is_some() {
            return Geometry::empty();
        }
        if *region != grid[i][j] {
            return compute_adjacent_wall(i0, j0, local_visited, &direction);
        }
        global_visited[i][j] = true;
        local_visited[i][j] = Some([false; 4]);
        Direction::iter().for_each(|direction| {
            queue.push_back((i, j, direction));
        });
        return Geometry::in_region();
    }
    compute_adjacent_wall(i0, j0, local_visited, &direction)
}

// FIXME: Should be counting vertices instead
fn compute_adjacent_wall(
    i: usize,
    j: usize,
    local_visited: &mut [Vec<Option<[bool; 4]>>],
    direction: &Direction,
) -> Geometry {
    local_visited[i][j].as_mut().unwrap()[usize::from(direction)] = true;
    if has_adjacent_wall(i, j, local_visited, direction) {
        return Geometry::empty();
    }
    Geometry::out_of_region()
}

fn has_adjacent_wall(
    i: usize,
    j: usize,
    local_visited: &[Vec<Option<[bool; 4]>>],
    direction: &Direction,
) -> bool {
    if let Some((i, j)) = direction.move_sideways_clockwise(local_visited, i, j) {
        if let Some(perimeter) = local_visited[i][j] {
            if perimeter[usize::from(direction)] {
                return true;
            }
        }
    }
    if let Some((i, j)) = direction.move_sideways_counterclockwise(local_visited, i, j) {
        if let Some(perimeter) = local_visited[i][j] {
            if perimeter[usize::from(direction)] {
                return true;
            }
        }
    }
    false
}

#[derive(Add, AddAssign, PartialEq, Debug)]
struct Geometry {
    area: usize,
    perimeter: usize,
}

impl Geometry {
    fn new(area: usize, perimeter: usize) -> Self {
        Geometry { area, perimeter }
    }

    fn empty() -> Self {
        Geometry::new(0, 0)
    }

    fn out_of_region() -> Self {
        Geometry::new(0, 1)
    }

    fn in_region() -> Self {
        Geometry::new(1, 0)
    }

    fn price(self) -> usize {
        self.area * self.perimeter
    }
}

#[derive(EnumIter, Debug)]
enum Direction {
    Right,
    Up,
    Down,
    Left,
}

impl Direction {
    fn move_forward<T>(&self, grid: &[Vec<T>], i: usize, j: usize) -> Option<(usize, usize)> {
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

    fn move_sideways_clockwise<T>(
        &self,
        grid: &[Vec<T>],
        i: usize,
        j: usize,
    ) -> Option<(usize, usize)> {
        let height = grid.len();
        let width = grid[0].len();
        let result = match self {
            Direction::Up => (Some(i), Some(j + 1)),
            Direction::Down => (Some(i), j.checked_sub(1)),
            Direction::Left => (i.checked_sub(1), Some(j)),
            Direction::Right => (Some(i + 1), Some(j)),
        };
        match result {
            (Some(i), Some(j)) if i < height && j < width => Some((i, j)),
            _ => None,
        }
    }
    fn move_sideways_counterclockwise<T>(
        &self,
        grid: &[Vec<T>],
        i: usize,
        j: usize,
    ) -> Option<(usize, usize)> {
        let height = grid.len();
        let width = grid[0].len();
        let result = match self {
            Direction::Up => (Some(i), j.checked_sub(1)),
            Direction::Down => (Some(i), Some(j + 1)),
            Direction::Left => (Some(i + 1), Some(j)),
            Direction::Right => (i.checked_sub(1), Some(j)),
        };
        match result {
            (Some(i), Some(j)) if i < height && j < width => Some((i, j)),
            _ => None,
        }
    }
}

impl From<&Direction> for usize {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::Down => 0,
            Direction::Left => 1,
            Direction::Right => 2,
            Direction::Up => 3,
        }
    }
}

struct Cell {
    i: usize,
    j: usize,
}

impl Cell {
    fn new(i: usize, j: usize) -> Self {
        Cell { i, j }
    }
}
