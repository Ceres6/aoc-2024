use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Context;

const PATH: &str = "src/dec6/input.txt";

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
struct GuardState {
    i: usize,
    j: usize,
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

#[derive(PartialEq, Eq, Hash, Debug)]
struct Position {
    i: usize,
    j: usize,
}

impl Position {
    fn new(i: usize, j: usize) -> Self {
        Position { i, j }
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

pub(crate) fn guard_position_count() -> anyhow::Result<usize> {
    let mut unique_positions = 1;

    let (mut map, mut guard_state) = create_map_and_initial_state()?;

    while let Ok(Position { i, j }) = guard_state.next_position() {
        let break_condition = map
            .get_mut(i)
            .and_then(|line| line.get_mut(j))
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
        .into_iter()
        .map(|position| is_loop(&map, &mut guard_state.clone(), position) as usize)
        .sum();
    Ok(loop_count)
}

fn create_map_and_initial_state() -> anyhow::Result<(Vec<Vec<char>>, GuardState)> {
    let file = File::open(PATH)?;
    let content = BufReader::new(file);
    let mut guard_state = GuardState::default();

    let map: Vec<Vec<char>> = content
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.unwrap_or_default()
                .chars()
                .enumerate()
                .map(|(j, char)| {
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

fn hash_positions(map: &[Vec<char>], guard_state: &mut GuardState) -> HashSet<Position> {
    let initial_position = Position::new(guard_state.i, guard_state.j);
    let mut positions: HashSet<Position> = HashSet::new();
    while let Ok(position) = guard_state.next_position() {
        let break_condition = map
            .get(position.i)
            .and_then(|line| line.get(position.j))
            .map(|next_position| {
                if *next_position == '#' {
                    guard_state.turn();
                } else {
                    guard_state.move_forward();
                    let new_position = Position::new(guard_state.i, guard_state.j);
                    if new_position != initial_position {
                        positions.insert(new_position);
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
    // println!();
    // println!();
    // println!();
    // println!("------ FUNCTION START ------");
    // println!("new obstacle {new_obstacle:?}");
    // println!("initial position {guard_state:?}");
    let mut states: HashSet<GuardState> = HashSet::new();
    let mut already_present = false;
    while let Ok(position) = guard_state.next_position() {
        // println!("----- LOOP ITERATION ----");
        // println!("{states:?}");
        // println!("{position:?}");
        if already_present {
            return true;
        }
        if new_obstacle == position {
            guard_state.turn();
            already_present = !states.insert(guard_state.clone());
            continue;
        }
        let break_condition = map
            .get(position.i)
            .and_then(|line| line.get(position.j))
            .map(|next_position| {
                if *next_position == '#' {
                    guard_state.turn();
                } else {
                    guard_state.move_forward();
                }
                // println!("states before safe {states:?}");
                // println!("guard state {guard_state:?}");
                already_present = !states.insert(guard_state.clone());
                // println!("already present {already_present}");
            })
            .is_none();
        if break_condition {
            break;
        }
    }
    false
}
