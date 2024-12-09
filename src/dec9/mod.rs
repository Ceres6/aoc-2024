use std::{cmp::Ordering, fs::read_to_string};

use anyhow::{Context, Ok};

const PATH: &str = "src/dec9/input.txt";

pub(crate) fn compact_checksum() -> anyhow::Result<usize> {
    let file = read_to_string(PATH)?;
    let num_chars = file.chars().count();
    let mut direct = file
        .chars()
        .map(|c| c.to_digit(10).unwrap_or_default() as usize)
        .enumerate();
    let mut reverse = file
        .chars()
        .map(|c| c.to_digit(10).unwrap_or_default() as usize)
        .rev()
        .zip((0..num_chars).rev());
    let (mut i, mut n) = direct.next().context("Failed to fetch next char")?;
    let (mut n_rev, mut i_rev) = reverse.next().context("Failed to fetch next char")?;
    if i_rev % 2 == 1 {
        (n_rev, i_rev) = reverse.next().context("Failed to fetch next char")?;
    }
    let mut pos = 0;
    let mut checksum = 0;
    while i < i_rev {
        if i % 2 == 0 {
            checksum += ((2 * pos + n - 1) * n * i) / 4;
            pos += n;
            (i, n) = direct.next().context("Failed to fetch next char")?;
        } else {
            match n.cmp(&n_rev) {
                Ordering::Less => {
                    n_rev -= n;
                    checksum += ((2 * pos + n - 1) * n * i_rev) / 4;
                    pos += n;
                    (i, n) = direct.next().context("Failed to fetch next char")?;
                }
                Ordering::Greater => {
                    n -= n_rev;
                    checksum += ((2 * pos + n_rev - 1) * n_rev * i_rev) / 4;
                    pos += n_rev;
                    reverse.next();
                    (n_rev, i_rev) = reverse.next().context("Failed to fetch next char")?;
                }
                Ordering::Equal => {
                    checksum += ((2 * pos + n - 1) * n * i_rev) / 4;
                    pos += n;
                    (i, n) = direct.next().context("Failed to fetch next char")?;
                    reverse.next();
                    (n_rev, i_rev) = reverse.next().context("Failed to fetch next char")?;
                }
            }
        }
    }
    if i == i_rev {
        checksum += ((2 * pos + n_rev - 1) * n_rev * i) / 4;
    }
    Ok(checksum)
}

pub(crate) fn unfragmented_compact_checksum() -> anyhow::Result<usize> {
    let file = read_to_string(PATH)?;
    let num_chars = file.chars().count();
    let n_spaces = num_chars / 2;
    let mut spaces = Vec::with_capacity(n_spaces);
    let mut space_ptrs = [None; 9];
    let mut pos_rev = file
        .chars()
        .map(|c| c.to_digit(10).unwrap_or_default() as usize)
        .enumerate()
        .fold(0, |pos, (i, n)| {
            if i % 2 == 1 {
                spaces.push((n, pos));
                (0..n).rev().any(|j| {
                    if space_ptrs[j].is_none() {
                        space_ptrs[j] = Some(i / 2);
                        false
                    } else {
                        true
                    }
                });
            }
            pos + n
        });
    pos_rev -= 1;
    let checksum = file
        .chars()
        .map(|c| c.to_digit(10).unwrap_or_default() as usize)
        .rev()
        .zip((0..num_chars).rev())
        .fold(0, |mut checksum, (n_rev, i_rev)| {
            if i_rev % 2 == 0 {
                update_ptr(&mut space_ptrs, &spaces, n_rev);
                if let Some(space_ptr) = &space_ptrs[n_rev - 1] {
                    let (space_size, space_pos) = &mut spaces[*space_ptr];
                    if *space_pos < pos_rev {
                        checksum += ((2 * *space_pos + n_rev - 1) * n_rev * i_rev) / 4;
                        *space_size -= n_rev;
                        *space_pos += n_rev;
                    } else {
                        checksum += ((2 * pos_rev - n_rev + 1) * n_rev * i_rev) / 4;
                    }
                } else {
                    checksum += ((2 * pos_rev - n_rev + 1) * n_rev * i_rev) / 4;
                }
            }
            pos_rev = pos_rev.saturating_sub(n_rev);
            checksum
        });
    Ok(checksum)
}

fn update_ptr(space_ptrs: &mut [Option<usize>; 9], spaces: &[(usize, usize)], to_update: usize) {
    if space_ptrs[to_update - 1].is_none() {
        return;
    }
    let ptr = space_ptrs[to_update - 1].unwrap();
    let (space_size, _) = spaces[ptr];
    if space_size >= to_update {
        return;
    }
    space_ptrs[to_update - 1] = spaces
        .iter()
        .enumerate()
        .skip(ptr)
        .find(|(_, (space_size, _))| *space_size >= to_update)
        .map(|(i, _)| i);
}
