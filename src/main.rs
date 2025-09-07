use tabled::{Table, Tabled};

mod days;

#[derive(Tabled)]
struct DayResult {
    day: usize,
    part_1: usize,
    part_2: usize,
}

fn main() {
    let res: Vec<DayResult> = days::run_all()
        .iter()
        .enumerate()
        .map(|(k, (part_1, part_2))| DayResult {
            day: k + 1,
            part_1: *part_1,
            part_2: *part_2,
        })
        .collect();

    let table = Table::new(res);
    println!("{table}");
}
