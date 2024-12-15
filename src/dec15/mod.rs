use std::fs::read_to_string;

const PATH: &str = "src/dec15/input.txt";

pub(crate) fn sum_box_positions() -> anyhow::Result<usize> {
    let contents = read_to_string(PATH)?;
    let mut split = contents.split("\n\n");
    let grid = split.next().unwrap();
    let moves = split.next().unwrap();
    let mut robot = Robot::new();
    let mut grid: Vec<Vec<char>> = grid
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, position)| {
                    if position == '@' {
                        robot.i = i;
                        robot.j = j;
                    }
                    position
                })
                .collect()
        })
        .collect();
    moves.lines().for_each(|line| {
        line.chars().for_each(|direction| {
            robot.try_move(&mut grid, &Direction::try_from_option(direction).unwrap())
        })
    });
    let result = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &position)| position == 'O')
                .map(|(j, _)| 100 * i + j)
                .sum::<usize>()
        })
        .sum();
    // let path = Path::new("src/dec15/output.txt");
    // let mut file = File::create(path)?;
    // for line in grid {
    //     let line_string: String = line.iter().collect();
    //     writeln!(file, "{}", line_string)?;
    // }
    Ok(result)
}

pub(crate) fn sum_wide_box_positions() -> anyhow::Result<usize> {
    let contents = read_to_string(PATH)?;
    let mut split = contents.split("\n\n");
    let grid = split.next().unwrap();
    let moves = split.next().unwrap();
    let mut robot = Robot::new();
    let mut grid: Vec<Vec<char>> = grid
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let mut row = Vec::with_capacity(line.len() * 2);
            line.chars()
                .enumerate()
                .for_each(|(j, position)| match position {
                    '@' => {
                        robot.i = i;
                        robot.j = 2 * j;
                        row.extend(['@', '.'])
                    }
                    'O' => row.extend(['[', ']']),
                    '#' => row.extend(['#', '#']),
                    _ => row.extend(['.', '.']),
                });
            row
        })
        .collect();

    moves.lines().for_each(|line| {
        line.chars().enumerate().for_each(|direction| {
            robot.try_move_wide(&mut grid, &Direction::try_from_option(direction.1).unwrap());
            // let var_name = &format!("src/dec15/output{}.txt", direction.0);
            // let path = Path::new(&var_name[..]);
            // let mut file = File::create(path).unwrap();
            // for line in grid.iter() {
            //     let line_string: String = line.iter().collect();
            //     writeln!(file, "{}", line_string).unwrap();
            // }
        })
    });
    let result = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &position)| position == '[')
                .map(|(j, _)| 100 * i + j)
                .sum::<usize>()
        })
        .sum();
    // let path = Path::new("src/dec15/output.txt");
    // let mut file = File::create(path)?;
    // for line in grid {
    //     let line_string: String = line.iter().collect();
    //     writeln!(file, "{}", line_string)?;
    // }
    Ok(result)
}

fn can_move(i: usize, j: usize, direction: &Direction, grid: &[Vec<char>], is_pair: bool) -> bool {
    match grid[i][j] {
        '.' => true,
        ']' => {
            let (i2, j2) = direction.move_forward(grid, i, j).unwrap();
            if *direction == Direction::Left || is_pair {
                return can_move(i2, j2, direction, grid, !is_pair);
            }
            can_move(i, j - 1, direction, grid, true) && can_move(i2, j2, direction, grid, false)
        }
        '[' => {
            let (i2, j2) = direction.move_forward(grid, i, j).unwrap();
            if *direction == Direction::Right || is_pair {
                return can_move(i2, j2, direction, grid, !is_pair);
            }
            can_move(i, j + 1, direction, grid, true) && can_move(i2, j2, direction, grid, false)
        }
        _ => false,
    }
}

fn do_move(
    prev: char,
    i: usize,
    j: usize,
    direction: &Direction,
    grid: &mut [Vec<char>],
    is_pair: bool,
) {
    match grid[i][j] {
        ']' => {
            let (i2, j2) = direction.move_forward(grid, i, j).unwrap();
            if *direction == Direction::Left || is_pair {
                do_move(']', i2, j2, direction, grid, !is_pair)
            } else {
                do_move('.', i, j - 1, direction, grid, true);
                do_move(']', i2, j2, direction, grid, false)
            }
        }
        '[' => {
            let (i2, j2) = direction.move_forward(grid, i, j).unwrap();
            if *direction == Direction::Right || is_pair {
                do_move('[', i2, j2, direction, grid, !is_pair)
            } else {
                do_move('.', i, j + 1, direction, grid, true);
                do_move('[', i2, j2, direction, grid, false)
            }
        }
        _ => (),
    }
    grid[i][j] = prev
}

struct Robot {
    i: usize,
    j: usize,
}

impl Robot {
    fn new() -> Self {
        Robot { i: 0, j: 0 }
    }
    fn try_move(&mut self, grid: &mut [Vec<char>], direction: &Direction) {
        if let Some((i, j)) = direction.move_forward(grid, self.i, self.j) {
            match grid[i][j] {
                '.' => {
                    grid[self.i][self.j] = '.';
                    self.i = i;
                    self.j = j;
                    grid[i][j] = '@';
                }
                'O' => {
                    let (mut i_n, mut j_n) = (i, j);
                    while let Some((i_n1, j_n1)) = direction.move_forward(grid, i_n, j_n) {
                        i_n = i_n1;
                        j_n = j_n1;
                        match grid[i_n][j_n] {
                            '.' => {
                                grid[self.i][self.j] = '.';
                                self.i = i;
                                self.j = j;
                                grid[i][j] = '@';
                                grid[i_n][j_n] = 'O';
                                return;
                            }
                            'O' => continue,
                            _ => return,
                        }
                    }
                }
                _ => (),
            }
        }
    }

    fn try_move_wide(&mut self, grid: &mut [Vec<char>], direction: &Direction) {
        if let Some((i, j)) = direction.move_forward(grid, self.i, self.j) {
            if can_move(i, j, direction, grid, false) {
                grid[self.i][self.j] = '.';
                self.i = i;
                self.j = j;
                do_move('@', i, j, direction, grid, false);
            }
        }
    }
}

#[derive(PartialEq)]
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
    fn try_from_option(value: char) -> Option<Self> {
        match value {
            '>' => Some(Self::Right),
            '<' => Some(Self::Left),
            '^' => Some(Self::Up),
            'v' => Some(Self::Down),
            _ => None,
        }
    }
}
