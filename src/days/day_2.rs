pub fn run(day: usize) -> (usize, usize) {
    super::run(day, part_1, part_2)
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let nums: Vec<u8> = line
                .split_whitespace()
                .map(|i| i.parse::<u8>().unwrap())
                .collect();
            is_safe(nums)
        })
        .collect::<Vec<_>>()
        .len()
}

fn part_2(input: &str) -> usize {
    0
}

pub fn is_safe(input: Vec<u8>) -> bool {
    let increasing = input.is_sorted_by(|a, b| a < b);
    let decreasing = input.is_sorted_by(|a, b| a > b);
    if !increasing && !decreasing {
        return false;
    }

    for w in input.windows(2) {
        let diff = w[0].abs_diff(w[1]);
        if !(1..=3).contains(&diff) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    mod is_safe {
        use super::*;

        #[test]
        fn increasing() {
            assert_eq!(true, is_safe(vec![1, 3, 6, 7, 9]));
        }
        #[test]
        fn decreasing() {
            assert_eq!(true, is_safe(vec![7, 6, 4, 2, 1]));
        }
        #[test]
        fn increase_too_large() {
            assert_eq!(false, is_safe(vec![1, 2, 7, 8, 9]));
        }
        #[test]
        fn decrease_too_large() {
            assert_eq!(false, is_safe(vec![9, 7, 6, 2, 1]));
        }
        #[test]
        fn increase_and_decrease() {
            assert_eq!(false, is_safe(vec![1, 3, 2, 4, 5]));
        }
        #[test]
        fn repeating_number() {
            assert_eq!(false, is_safe(vec![8, 6, 4, 4, 1]));
        }
    }
}
