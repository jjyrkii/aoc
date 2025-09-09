pub fn run(day: usize) -> (usize, usize) {
    super::run(day, part_1, part_2)
}

fn part_1(input: &str) -> usize {
    let grid: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let h = grid.len();
    let w = grid.first().map_or(0, |row| row.len());

    let inb = |r: isize, c: isize| r >= 0 && c >= 0 && (r as usize) < h && (c as usize) < w;

    let mut res = 0usize;
    for r in 0..h {
        for c in 0..w {
            if grid[r][c] == b'X' {
                for (dr, dc) in DIRS {
                    let mut ok = true;
                    let (mut rr, mut cc) = (r as isize, c as isize);
                    for &ch in [b'X', b'M', b'A', b'S'].iter() {
                        if !inb(rr, cc) || grid[rr as usize][cc as usize] != ch {
                            ok = false;
                            break;
                        }
                        rr += dr;
                        cc += dc;
                    }
                    if ok {
                        res += 1;
                    }
                }
            }
        }
    }
    res
}

fn part_2(input: &str) -> usize {
    let grid: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let h = grid.len();
    let w = grid.first().map_or(0, |row| row.len());

    let inb = |r: isize, c: isize| r >= 0 && c >= 0 && (r as usize) < h && (c as usize) < w;
    let mut res = 0usize;
    if h >= 3 && w >= 3 {
        for r in 1..h - 1 {
            for c in 1..w - 1 {
                if grid[r][c] == b'A' {
                    let nw = grid[r - 1][c - 1];
                    let ne = grid[r - 1][c + 1];
                    let sw = grid[r + 1][c - 1];
                    let se = grid[r + 1][c + 1];

                    let diag1_ok = (nw == b'M' && se == b'S') || (nw == b'S' && se == b'M');
                    let diag2_ok = (ne == b'M' && sw == b'S') || (ne == b'S' && sw == b'M');

                    if diag1_ok && diag2_ok {
                        res += 1;
                    }
                }
            }
        }
    }
    res
}
const DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[cfg(test)]
mod tests {
    use super::*;
}
