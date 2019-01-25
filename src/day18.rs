use itertools::{iproduct, Itertools};

fn next(x: i32, y: i32, m: &[Vec<u8>]) -> u8 {
    let mut nb_trees = 0;
    let mut nb_lumberyards = 0;
    for (i, j) in iproduct!(x - 1..=x + 1, y - 1..=y + 1) {
        if i == x && j == y {
            continue;
        }
        if i < 0 || j < 0 {
            continue;
        }
        let (i, j) = (i as usize, j as usize);
        if j >= m.len() || i >= m[j].len() {
            continue;
        }
        match m[j][i] {
            b'|' => nb_trees += 1,
            b'#' => nb_lumberyards += 1,
            _ => {}
        }
    }
    let (x, y) = (x as usize, y as usize);
    match m[y][x] {
        b'.' if nb_trees >= 3 => b'|',
        b'|' if nb_lumberyards >= 3 => b'#',
        b'#' if nb_trees > 0 && nb_lumberyards > 0 => b'#',
        b'#' => b'.',
        c => c,
    }
}

fn main() {
    let input = common::get_input(18).unwrap();
    let mut m = input.lines().map(|s| s.bytes().collect_vec()).collect_vec();
    let mut m_next = m.clone();
    let (height, width) = (m.len(), m[0].len());
    let mut last = 0;
    for i in 0..(1000000000 - 500) % 28 + 500 {
        // NOTE: hard coded cycle count
        for (x, y) in iproduct!(0..width, 0..height) {
            m_next[y][x] = next(x as i32, y as i32, &m);
        }
        std::mem::swap(&mut m, &mut m_next);
        let n_trees = m
            .iter()
            .map(|v| v.iter().filter(|b| **b == b'|').count())
            .sum::<usize>() as i32;
        let n_lumberyards = m
            .iter()
            .map(|v| v.iter().filter(|b| **b == b'#').count())
            .sum::<usize>() as i32;
        let this = n_trees * n_lumberyards;
        println!("{}: {} {}", i, this, this - last);
        last = this;
    }
}
