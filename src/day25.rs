use itertools::Itertools;

fn near(a: &[i32], b: &[i32]) -> bool {
    a.iter()
        .zip(b.iter())
        .map(|(i, j)| (i - j).abs())
        .sum::<i32>()
        <= 3
}

fn find(r: &mut [usize], i: usize) -> usize {
    while r[i] != r[r[i]] {
        r[i] = r[r[i]];
    }
    r[i]
}

fn union(r: &mut [usize], i: usize, j: usize) {
    r[find(r, i)] = find(r, j);
}

fn main() {
    let input = common::get_input(25).unwrap();
    let points = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|w| w.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    let mut r = (0..points.len()).collect_vec();
    for (i, p) in points.iter().enumerate() {
        for (j, q) in points.iter().skip(i + 1).enumerate() {
            if near(p, q) {
                union(&mut r, i, j + i + 1);
            }
        }
    }
    let mut rs = (0..points.len()).map(|p| find(&mut r, p)).collect_vec();
    rs.sort();
    rs.dedup();
    println!("{}", rs.len());
}
