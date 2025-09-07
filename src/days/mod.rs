include!(concat!(env!("OUT_DIR"), "/days_gen.rs"));
pub fn run(day: usize, part_1: fn(&str) -> usize, part_2: fn(&str) -> usize) -> (usize, usize) {
    let input = std::fs::read_to_string(format!("input/day_{day}.txt")).unwrap();
    (part_1(&input), part_2(&input))
}
