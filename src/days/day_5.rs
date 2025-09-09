use std::collections::{HashMap, HashSet};

pub fn run(day: usize) -> (usize, usize) {
    super::run(day, part_1, part_2)
}

fn part_1(input: &str) -> usize {
    let (rules, updates) = parse(input);
    updates
        .into_iter()
        .filter(|u| is_ordered(u, &rules))
        .map(|u| u[u.len() / 2])
        .sum()
}

fn part_2(input: &str) -> usize {
    let (rules, updates) = parse(input);
    updates
        .into_iter()
        .filter(|u| !is_ordered(u, &rules))
        .map(|u| {
            let fixed = topo_sort_restricted(&u, &rules);
            fixed[fixed.len() / 2]
        })
        .sum()
}

fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let (a, b) = input.split_once("\n\n").expect("rules + updates");
    let rules = a
        .lines()
        .map(|line| {
            let (x, y) = line.split_once('|').unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    let updates = b
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    (rules, updates)
}

fn is_ordered(update: &[usize], rules: &[(usize, usize)]) -> bool {
    let pos: HashMap<usize, usize> = update
        .iter()
        .copied()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect();
    for &(x, y) in rules {
        if let (Some(&ix), Some(&iy)) = (pos.get(&x), pos.get(&y))
            && ix > iy
        {
            return false;
        }
    }
    true
}

fn topo_sort_restricted(update: &[usize], rules: &[(usize, usize)]) -> Vec<usize> {
    let present: HashSet<usize> = update.iter().copied().collect();
    let mut indeg: HashMap<usize, usize> = present.iter().map(|&n| (n, 0)).collect();
    let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();

    for &(x, y) in rules {
        if present.contains(&x) && present.contains(&y) {
            adj.entry(x).or_default().push(y);
            *indeg.get_mut(&y).unwrap() += 1;
        }
    }

    let pos: HashMap<usize, usize> = update
        .iter()
        .copied()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect();

    let mut used: HashSet<usize> = HashSet::new();
    let mut out = Vec::with_capacity(update.len());

    while out.len() < update.len() {
        if let Some(&next) = update
            .iter()
            .filter(|&&n| !used.contains(&n) && indeg[&n] == 0)
            .min_by_key(|&&n| pos[&n])
        {
            used.insert(next);
            out.push(next);
            if let Some(nei) = adj.get(&next) {
                for &m in nei {
                    *indeg.get_mut(&m).unwrap() -= 1;
                }
            }
        } else {
            panic!("Cycle detected among pages: {:?}", update);
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const RULES: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13"#;

    const UPDATES: &str = r#"75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn sample_part1_and_part2() {
        let input = format!("{RULES}\n\n{UPDATES}");
        assert_eq!(part_1(&input), 143); // AoC sample
        assert_eq!(part_2(&input), 123);
    }
}
