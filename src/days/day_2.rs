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
    input
        .lines()
        .filter(|line| {
            let nums: Vec<u8> = line
                .split_whitespace()
                .map(|i| i.parse::<u8>().unwrap())
                .collect();
            variants_are_safe(nums)
        })
        .collect::<Vec<_>>()
        .len()
}

fn variants_are_safe(input: Vec<u8>) -> bool {
    if is_safe(input.clone()) {
        return true;
    }

    for i in 0..input.len() {
        let mut temp = input.clone();
        temp.remove(i);
        if is_safe(temp) {
            return true;
        }
    }
    false
}

fn is_safe(input: Vec<u8>) -> bool {
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
        fn decreasing() {
            assert!(is_safe(vec![7, 6, 4, 2, 1]));
        }
        #[test]
        fn increase_too_large() {
            assert!(!is_safe(vec![1, 2, 7, 8, 9]));
        }
        #[test]
        fn decrease_too_large() {
            assert!(!is_safe(vec![9, 7, 6, 2, 1]));
        }
        #[test]
        fn increase_and_decrease() {
            assert!(!is_safe(vec![1, 3, 2, 4, 5]));
        }
        #[test]
        fn repeating_number() {
            assert!(!is_safe(vec![8, 6, 4, 4, 1]));
        }
        #[test]
        fn increasing() {
            assert!(is_safe(vec![1, 3, 6, 7, 9]));
        }
    }
    mod variants_are_safe {
        use super::*;

        #[test]
        fn safe_no_removing_1() {
            assert!(variants_are_safe(vec![7, 6, 4, 2, 1]));
        }
        #[test]
        fn removed_anyway_1() {
            assert!(!variants_are_safe(vec![1, 2, 7, 8, 9]));
        }
        #[test]
        fn removed_anyway_2() {
            assert!(!variants_are_safe(vec![9, 7, 6, 2, 1]));
        }
        #[test]
        fn removing_second_level() {
            assert!(variants_are_safe(vec![1, 3, 2, 4, 5]));
        }
        #[test]
        fn removing_third_level() {
            assert!(variants_are_safe(vec![8, 6, 4, 4, 1]));
        }
        #[test]
        fn safe_no_removing_2() {
            assert!(variants_are_safe(vec![1, 3, 6, 7, 9]));
        }
    }
}
