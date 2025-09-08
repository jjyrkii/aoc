use regex::Regex;

pub fn run(day: usize) -> (usize, usize) {
    super::run(day, part_1, part_2)
}

fn part_1(input: &str) -> usize {
    get_multiplications(input).iter().map(|(a, b)| a * b).sum()
}

fn part_2(input: &str) -> usize {
    get_instructions(input)
        .iter()
        .map(|m| match m {
            Instruction::Mul(a, b) => a * b,
            _ => 0,
        })
        .sum()
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Do,
    Dont,
    Mul(usize, usize),
}

fn get_instructions(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"mul\(\d*,\d*\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    let mut res = vec![];
    for m in re.find_iter(input) {
        let s = m.as_str();
        if s == "don't()" {
            enabled = false;
            res.push(Instruction::Dont);
        } else if s == "do()" {
            enabled = true;
            res.push(Instruction::Do);
        } else if enabled {
            let slice = m
                .as_str()
                .strip_prefix("mul(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split_once(',')
                .unwrap();
            res.push(Instruction::Mul(
                slice.0.parse().unwrap(),
                slice.1.parse().unwrap(),
            ));
        }
    }
    res
}

fn get_multiplications(input: &str) -> Vec<(usize, usize)> {
    let re = Regex::new(r"mul\(\d*,\d*\)").unwrap();

    let res: Vec<(usize, usize)> = re
        .find_iter(input)
        .map(|m| {
            let slice = m
                .as_str()
                .strip_prefix("mul(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split_once(',')
                .unwrap();
            (slice.0.parse().unwrap(), slice.1.parse().unwrap())
        })
        .collect();

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_multiplications() {
        let expected = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(expected, get_multiplications(input));
    }

    #[test]
    fn test_get_instructions() {
        let expected = vec![
            Instruction::Mul(2, 4),
            Instruction::Dont,
            Instruction::Do,
            Instruction::Mul(8, 5),
        ];
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(expected, get_instructions(input));
    }
}
