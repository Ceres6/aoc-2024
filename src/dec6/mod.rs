use std::{
    // collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Context;
// use fxhash::FxHashSet as HashSet;
use rayon::prelude::*;
// use gxhash::{HashSet, HashSetExt};
// use ahash::{HashSet, HashSetExt};

const PATH: &str = "src/dec6/input.txt";

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
struct GuardState {
    i: u16,
    j: u16,
    direction: Direction,
}

impl GuardState {
    fn turn(&mut self) {
        self.direction = self.direction.next();
    }

    fn next_position(&self) -> anyhow::Result<Position> {
        let mut i = self.i;
        let mut j = self.j;
        match self.direction {
            Direction::Up => i = i.checked_sub(1).context("overflow")?,
            Direction::Right => j += 1,
            Direction::Down => i += 1,
            Direction::Left => j = j.checked_sub(1).context("overflow")?,
        };
        Ok(Position::new(i, j))
    }

    fn move_forward(&mut self) {
        let new_position = self.next_position().unwrap();
        self.i = new_position.i;
        self.j = new_position.j;
    }
}

impl From<GuardState> for u32 {
    fn from(value: GuardState) -> Self {
        u32::from(value.direction) * 130 * 130 + (value.i * 130 + value.j) as u32
    }
}

impl From<GuardState> for usize {
    fn from(value: GuardState) -> Self {
        usize::from(value.direction) * 130 * 130 + (value.i * 130 + value.j) as usize
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Position {
    i: u16,
    j: u16,
}

impl Position {
    fn new(i: u16, j: u16) -> Self {
        Position { i, j }
    }
}

impl From<Position> for u16 {
    fn from(value: Position) -> Self {
        value.i * 130 + value.j
    }
}

impl From<u16> for Position {
    fn from(value: u16) -> Self {
        Position::new(value / 130, value % 130)
    }
}

impl From<Position> for usize {
    fn from(value: Position) -> Self {
       (value.i * 130 + value.j) as usize
    }
}

impl From<usize> for Position {
    fn from(value: usize) -> Self {
        Position::new((value / 130) as u16, (value % 130) as u16)
    }
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::Up => Self::Right,
            Direction::Right => Self::Down,
            Direction::Down => Self::Left,
            Direction::Left => Self::Up,
        }
    }
}

impl From<Direction> for u32 {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
}

impl From<Direction> for usize {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
}

pub(crate) fn guard_position_count() -> anyhow::Result<usize> {
    let mut unique_positions = 1;

    let (mut map, mut guard_state) = create_map_and_initial_state()?;

    while let Ok(Position { i, j }) = guard_state.next_position() {
        let break_condition = map
            .get_mut(i as usize)
            .and_then(|line| line.get_mut(j as usize))
            .map(|next_position| {
                match next_position {
                    '#' => guard_state.turn(),
                    '.' => {
                        guard_state.move_forward();
                        *next_position = 'X';
                        unique_positions += 1
                    }
                    _ => guard_state.move_forward(),
                };
            })
            .is_none();
        if break_condition {
            break;
        }
    }

    Ok(unique_positions)
}

pub(crate) fn count_possible_loops() -> anyhow::Result<usize> {
    let (map, guard_state) = create_map_and_initial_state()?;
    let positions = hash_positions(&map, &mut guard_state.clone());
    let loop_count = positions
        .into_par_iter()
        .enumerate()
        .filter(|(_, visited)| *visited)
        .map(|(position, _)| is_loop(&map, &mut guard_state.clone(), position.into()) as usize)
        .sum();
    Ok(loop_count)
}

fn create_map_and_initial_state() -> anyhow::Result<(Vec<Vec<char>>, GuardState)> {
    let file = File::open(PATH)?;
    let content = BufReader::new(file);
    let mut guard_state = GuardState::default();

    let map: Vec<Vec<char>> = content
        .lines()
        .zip(0..)
        .map(|(line, i)| {
            line.unwrap_or_default()
                .chars()
                .zip(0..)
                .map(|(char, j)| {
                    if char == '^' {
                        guard_state.i = i;
                        guard_state.j = j;
                    }
                    char
                })
                .collect()
        })
        .collect();

    Ok((map, guard_state))
}

fn hash_positions(map: &[Vec<char>], guard_state: &mut GuardState) -> [bool; 130 * 130] {
    let initial_position = Position::new(guard_state.i, guard_state.j);
    // let mut positions: HashSet<u16> = HashSet::default();
    // let mut positions: HashSet<u16> = HashSet::new();
    let mut positions = [false; 130 * 130];
    while let Ok(position) = guard_state.next_position() {
        let break_condition = map
            .get(position.i as usize)
            .and_then(|line| line.get(position.j as usize))
            .map(|next_position| {
                if *next_position == '#' {
                    guard_state.turn();
                } else {
                    guard_state.move_forward();
                    let new_position = Position::new(guard_state.i, guard_state.j);
                    if new_position != initial_position {
                        positions[usize::from(new_position)] = true;
                    }
                }
            })
            .is_none();
        if break_condition {
            break;
        }
    }
    positions
}

fn is_loop(map: &[Vec<char>], guard_state: &mut GuardState, new_obstacle: Position) -> bool {
    // let mut states: HashSet<u32> = HashSet::default();
    // let mut states: HashSet<u32> = HashSet::new();
    let mut states = [false; 130 * 130 * 4];
    let mut already_present = false;
    while let Ok(position) = guard_state.next_position() {
        if already_present {
            return true;
        }
        if new_obstacle == position {
            guard_state.turn();
            let state_index = usize::from(guard_state.clone());
            already_present = states[state_index];
            states[state_index] = true;
            // already_present = !states.insert(guard_state.clone().into());
            continue;
        }
        let break_condition = map
            .get(position.i as usize)
            .and_then(|line| line.get(position.j as usize))
            .map(|next_position| {
                if *next_position == '#' {
                    guard_state.turn();
                } else {
                    guard_state.move_forward();
                }
                // already_present = !states.insert(guard_state.clone().into());
                let state_index = usize::from(guard_state.clone());
                already_present = states[state_index];
                states[state_index] = true;
            })
            .is_none();
        if break_condition {
            break;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_loops() -> anyhow::Result<()> {
        assert_eq!(count_possible_loops()?, 1434);
        Ok(())
    }
}

// cargo flamegraph showed that most time is spent on hashing and iterating over hash
// benchmarking with criterion
// Changing the size of positions from usize to u16 improved from 1.33 to 1.27 -> Not very good
// Converting Position to u16 and GuardState to u32 before hashing improved from 1.27 to 0.700 -> noticeable
// Using gxHash improved from 0.700 to 0.353 -> noticeable
// Using aHash improved to .324 -> marginally better
// Using fxHash improve to 0.250 -> better
// Adding rayon improved from 0.150 to 0.037 -> almost 10x reduction (which makes sense as I'm using 10 cores computer)
// using boolean grid took .725 -> terribly worse
// using 1D boolean array with preallocated size for guard state took 9.5ms -> much better
// adding 1D boolean array with preallocated size for visited positions took 9.1ms -> slightly better
// Almost 148x from first try

