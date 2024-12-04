use std::fs::read;

const PATH: &str = "src/dec2/input.txt";

enum Direction {
    Increasing,
    Decreasing,
}

pub(crate) fn safe_count() -> i32 {
    let bytes = read(PATH).unwrap();
    let reports = String::from_utf8(bytes).unwrap();
    reports.lines().fold(0, |acc, report| {
        let mut prev_opt = None;
        let mut dir_opt = None;
        for word in report.split_ascii_whitespace() {
            let level = word.parse::<i32>().unwrap();
            if let Some(prev) = prev_opt {
                match dir_opt {
                    Some(Direction::Decreasing) => match prev - level {
                        1..=3 => (),
                        _ => return acc,
                    },
                    Some(Direction::Increasing) => match level - prev {
                        1..=3 => (),
                        _ => return acc,
                    },
                    None => match level - prev {
                        -3..=-1 => {
                            dir_opt = Some(Direction::Decreasing);
                        }
                        1..=3 => {
                            dir_opt = Some(Direction::Increasing);
                        }
                        _ => return acc,
                    },
                }
            }
            prev_opt = Some(level);
        }
        acc + 1
    })
}

pub(crate) fn dampened_count() -> i32 {
    let bytes = read(PATH).unwrap();
    let reports = String::from_utf8(bytes).unwrap();
    reports.lines().fold(0, |acc, report| {
        if is_safe(report.split_ascii_whitespace(), false)
        {
            acc + 1
        } else {
            for i in 0..report.split_ascii_whitespace().count() {
                if is_safe(
                    report
                        .split_ascii_whitespace()
                        .enumerate()
                        .filter(|(j, _)| *j != i)
                        .map(|(_, w)| w),
                    true,
                ) {
                    return acc + 1;
                }
            }
            acc
        }
    })
}

fn is_safe<'a, I: Iterator<Item = &'a str>>(report: I, mut already_dampened: bool) -> bool {
    let mut prev_opt = None;
    let mut dir_opt = None;
    for word in report {
        let level = word.parse::<i32>().unwrap();
        if let Some(prev) = prev_opt {
            match dir_opt {
                Some(Direction::Decreasing) => match prev - level {
                    1..=3 => (),
                    _ => {
                        if already_dampened {
                            return false;
                        } else {
                            already_dampened = true;
                            continue;
                        }
                    }
                },
                Some(Direction::Increasing) => match level - prev {
                    1..=3 => (),
                    _ => {
                        if already_dampened {
                            return false;
                        } else {
                            already_dampened = true;
                            continue;
                        }
                    }
                },
                None => match level - prev {
                    -3..=-1 => {
                        dir_opt = Some(Direction::Decreasing);
                    }
                    1..=3 => {
                        dir_opt = Some(Direction::Increasing);
                    }
                    _ => {
                        if already_dampened {
                            return false;
                        } else {
                            already_dampened = true;
                            continue;
                        }
                    }
                },
            }
        }
        prev_opt = Some(level);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_safe_count() {
        assert_eq!(safe_count(), 332);
    }

    #[test]
    fn test_dampened_count() {
        assert_eq!(dampened_count(), 398);
    }
}