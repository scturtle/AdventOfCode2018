use itertools::{iproduct, Itertools};

fn main() {
    let input = common::get_input(22).unwrap();
    let mut lines = input.lines();
    let depth: u32 = lines
        .next()
        .unwrap()
        .split(' ')
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let target = lines
        .next()
        .unwrap()
        .split(' ')
        .nth(1)
        .unwrap()
        .split(',')
        .map(|t| t.parse::<usize>().unwrap())
        .collect_vec();

    let padx = 50;
    let pady = 50;
    let mut e = vec![vec![0u32; target[0] + padx]; target[1] + pady];
    for (i, j) in iproduct!(0..e[0].len(), 0..e.len()) {
        let g;
        if (i == 0 && j == 0) || (i == target[0] && j == target[1]) {
            g = 0;
        } else if i == 0 {
            g = j as u32 * 48271;
        } else if j == 0 {
            g = i as u32 * 16807;
        } else {
            g = e[j - 1][i] * e[j][i - 1];
        }
        e[j][i] = (g + depth) % 20183;
    }
    for (i, j) in iproduct!(0..e[0].len(), 0..e.len()) {
        e[j][i] %= 3;
    }
    assert!(e[target[1]][target[0]] == 0);
    let part1 = e[..=target[1]]
        .iter()
        .map(|v| v[..=target[0]].iter().sum::<u32>())
        .sum::<u32>()
        - e[target[1]][target[0]];
    dbg!(part1);

    let max = std::u32::MAX - 10;
    let mut n = vec![vec![max; target[0] + padx]; target[1] + pady];
    let mut t = vec![vec![max; target[0] + padx]; target[1] + pady];
    let mut c = vec![vec![max; target[0] + padx]; target[1] + pady];
    t[0][0] = 0;
    c[0][0] = 7;
    let mut part2 = max;
    loop {
        for (i, j) in iproduct!(0..e[0].len(), 0..e.len()) {
            let (i_, j_) = (i as i32, j as i32);
            for &(ni_, nj_) in &[(i_ - 1, j_), (i_, j_ - 1), (i_ + 1, j_), (i_, j_ + 1)] {
                if ni_ < 0 || nj_ < 0 || ni_ >= e[0].len() as i32 || nj_ >= e.len() as i32 {
                    continue;
                }
                let (ni, nj) = (ni_ as usize, nj_ as usize);
                if e[nj][ni] == 0 && e[j][i] == 0 {
                    c[nj][ni] = c[nj][ni].min(c[j][i] + 1).min(t[j][i] + 8);
                    t[nj][ni] = t[nj][ni].min(c[j][i] + 8).min(t[j][i] + 1);
                }
                if e[nj][ni] == 0 && e[j][i] == 1 {
                    c[nj][ni] = c[nj][ni].min(c[j][i] + 1).min(n[j][i] + 8);
                }
                if e[nj][ni] == 0 && e[j][i] == 2 {
                    t[nj][ni] = t[nj][ni].min(t[j][i] + 1).min(n[j][i] + 8);
                }
                if e[nj][ni] == 1 && e[j][i] == 0 {
                    c[nj][ni] = c[nj][ni].min(c[j][i] + 1).min(t[j][i] + 8);
                }
                if e[nj][ni] == 1 && e[j][i] == 1 {
                    c[nj][ni] = c[nj][ni].min(c[j][i] + 1).min(n[j][i] + 8);
                    n[nj][ni] = n[nj][ni].min(c[j][i] + 8).min(n[j][i] + 1);
                }
                if e[nj][ni] == 1 && e[j][i] == 2 {
                    n[nj][ni] = n[nj][ni].min(t[j][i] + 8).min(n[j][i] + 1);
                }
                if e[nj][ni] == 2 && e[j][i] == 0 {
                    t[nj][ni] = t[nj][ni].min(c[j][i] + 8).min(t[j][i] + 1);
                }
                if e[nj][ni] == 2 && e[j][i] == 1 {
                    n[nj][ni] = n[nj][ni].min(c[j][i] + 8).min(n[j][i] + 1);
                }
                if e[nj][ni] == 2 && e[j][i] == 2 {
                    t[nj][ni] = t[nj][ni].min(t[j][i] + 1).min(n[j][i] + 8);
                    n[nj][ni] = n[nj][ni].min(t[j][i] + 8).min(n[j][i] + 1);
                }
            }
        }
        if t[target[1]][target[0]] < part2 {
            part2 = t[target[1]][target[0]];
            dbg!(part2);
        }
    }
}
