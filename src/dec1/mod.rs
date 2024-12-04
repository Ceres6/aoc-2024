use std::{
    collections::{HashMap, HashSet},
    fs,
};

const PATH: &str = "src/dec1/input.txt";

pub(crate) fn diff() -> i32 {
    let bytes = fs::read(PATH).unwrap();
    let mut firsts = vec![];
    let mut seconds = vec![];
    let content = String::from_utf8(bytes).unwrap();
    content.lines().for_each(|line| {
        let mut words = line.split_ascii_whitespace();
        firsts.push(words.next().unwrap().parse::<i32>().unwrap());
        seconds.push(words.next().unwrap().parse::<i32>().unwrap());
    });
    firsts.sort_unstable();
    seconds.sort_unstable();
    firsts
        .iter()
        .zip(seconds)
        .fold(0, |acc, (first, second)| acc + (first - second).abs())
}

pub(crate) fn similarity_score() -> i32 {
    let bytes = fs::read(PATH).unwrap();
    let mut firsts = HashSet::new();
    let mut seconds = HashMap::new();
    let content = String::from_utf8(bytes).unwrap();
    content.lines().for_each(|line| {
        let mut words = line.split_ascii_whitespace();
        firsts.insert(words.next().unwrap().parse::<i32>().unwrap());
        seconds
            .entry(words.next().unwrap().parse::<i32>().unwrap())
            .and_modify(|v| *v += 1)
            .or_insert(1);
    });
    firsts
        .iter()
        .fold(0, |acc, curr| acc + *curr * seconds.get(curr).unwrap_or(&0))
}
