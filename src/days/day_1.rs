use std::collections::HashMap;

pub fn run(day: usize) -> (usize, usize) {
    super::run(day, part_1, part_2)
}

fn part_1(input: &str) -> usize {
    let lines = input.lines();
    let mut list_a: Vec<usize> = vec![];
    let mut list_b: Vec<usize> = vec![];
    for line in lines {
        let (a, b) = line.split_once("   ").unwrap();
        list_a.push(a.parse::<usize>().unwrap());
        list_b.push(b.parse::<usize>().unwrap());
    }
    list_a.sort();
    list_b.sort();

    list_a
        .iter()
        .enumerate()
        .map(|row| row.1.abs_diff(list_b[row.0]))
        .sum()
}

fn part_2(input: &str) -> usize {
    let pairs: Vec<(usize, usize)> = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .collect::<Vec<(&str, &str)>>()
        .iter()
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();
    let mut amounts: HashMap<usize, usize> = HashMap::new();
    for (_, right) in &pairs {
        let current = amounts.get(right).unwrap_or(&0);
        amounts.insert(*right, current + 1);
    }

    let mut res = 0;

    for (left, _) in pairs {
        let amount = amounts.get(&left).unwrap_or(&0usize);
        let score = left * amount;
        res += score;
    }
    res
}
