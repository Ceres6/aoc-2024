use itertools::iproduct;
use std::fs::read;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

const PATH: &str = "src/dec4/input.txt";

#[derive(Clone, Copy, EnumIter, Display)]
enum DirectionFlow {
    Positive,
    Negative,
    Neutral,
}

fn can_go_direction(
    vertical: DirectionFlow,
    horizontal: DirectionFlow,
    length: usize,
    v_pos: usize,
    h_pos: usize,
    width: usize,
    height: usize,
) -> bool {
    let can_go_horizontal = match horizontal {
        DirectionFlow::Positive => h_pos < width - length,
        DirectionFlow::Negative => h_pos >= length,
        DirectionFlow::Neutral => true,
    };
    let can_go_vertical = match vertical {
        DirectionFlow::Positive => v_pos < height - length,
        DirectionFlow::Negative => v_pos >= length,
        DirectionFlow::Neutral => true,
    };
    can_go_horizontal && can_go_vertical
}

fn is_match_in_direction(
    grid: &[Vec<char>],
    vertical: DirectionFlow,
    horizontal: DirectionFlow,
    v_pos: usize,
    h_pos: usize,
    word: &str,
) -> bool {
    let height = grid.len();
    let width = grid[0].len();
    if !can_go_direction(
        vertical,
        horizontal,
        word.chars().count(),
        v_pos,
        h_pos,
        width,
        height,
    ) {
        return false;
    }
    for (i, c) in word.char_indices() {
        let y = go_direction(v_pos, vertical, i + 1);
        let x = go_direction(h_pos, horizontal, i + 1);
        if grid[y][x] != c {
            return false;
        }
    }
    true
}

fn go_direction(start: usize, direction: DirectionFlow, amount: usize) -> usize {
    match direction {
        DirectionFlow::Negative => start - amount,
        DirectionFlow::Positive => start + amount,
        DirectionFlow::Neutral => start,
    }
}

pub(crate) fn count_xmas() -> anyhow::Result<u32> {
    let bytes = read(PATH)?;
    let contents = String::from_utf8(bytes)?;
    let grid: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();
    let mut occurrences = 0;
    let cells = iproduct!(0..height, 0..width);
    for (i, j) in cells {
        if grid[i][j] == 'X' {
            let directions = iproduct!(DirectionFlow::iter(), DirectionFlow::iter());
            for (vertical, horizontal) in directions {
                if is_match_in_direction(&grid, vertical, horizontal, i, j, "MAS") {
                    occurrences += 1;
                }
            }
        }
    }
    Ok(occurrences)
}

pub(crate) fn count_cross_mas() -> anyhow::Result<u32> {
    let bytes = read(PATH)?;
    let contents = String::from_utf8(bytes)?;
    let grid: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();
    let mut occurrences = 0;
    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == 'A' {
                let can_search = i >= 1 && i < height - 1 && j >= 1 && j < width - 1;
                if can_search
                    && ((grid[i - 1][j - 1] == 'M' && grid[i + 1][j + 1] == 'S')
                        || (grid[i - 1][j - 1] == 'S' && grid[i + 1][j + 1] == 'M'))
                    && ((grid[i - 1][j + 1] == 'M' && grid[i + 1][j - 1] == 'S')
                        || (grid[i - 1][j + 1] == 'S' && grid[i + 1][j - 1] == 'M'))
                {
                    occurrences += 1
                }
            }
        }
    }
    Ok(occurrences)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_xmas_count() {
        assert_eq!(count_xmas().unwrap(), 2569);
    }
}
