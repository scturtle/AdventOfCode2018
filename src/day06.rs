use itertools::{iproduct, Itertools};
use std::collections::{HashSet, VecDeque};

fn main() {
    let centers = common::get_input(6)
        .unwrap()
        .trim_end()
        .split('\n')
        .map(|s| {
            let mut t = s.split(", ").map(|x| x.parse::<i32>().unwrap());
            (t.next().unwrap(), t.next().unwrap())
        })
        .collect_vec();

    let maxx = centers.iter().map(|v| v.0).max().unwrap();
    let maxy = centers.iter().map(|v| v.1).max().unwrap();

    let mut largest_area = 0;
    for (ii, c) in centers.iter().enumerate() {
        let mut q = VecDeque::new();
        q.push_back(*c);
        let mut saw = HashSet::new();
        saw.insert(*c);
        let mut infinite = false;
        'bfs: while let Some(t) = q.pop_front() {
            'step: for (i, j) in iproduct!(-1..=1, -1..=1) {
                let p = (t.0 + i, t.1 + j);
                if saw.contains(&p) {
                    continue;
                }
                for (jj, c2) in centers.iter().enumerate() {
                    if ii != jj
                        && (p.0 - c.0).abs() + (p.1 - c.1).abs()
                            >= (p.0 - c2.0).abs() + (p.1 - c2.1).abs()
                    {
                        continue 'step;
                    }
                }
                if p.0 < 0 || p.1 < 0 || p.0 > maxx || p.1 > maxy {
                    infinite = true;
                    break 'bfs;
                }
                q.push_back(p);
                saw.insert(p);
            }
        }
        let area = saw.len();
        if !infinite && area > largest_area {
            largest_area = area;
        }
    }
    println!("{}", largest_area);

    let mut cnt = 0;
    for (x, y) in iproduct!(0..=maxx /* + 10000*/, 0..=maxy /* + 10000*/) {
        let dis: i32 = centers
            .iter()
            .map(|&(cx, cy)| (x - cx).abs() + (y - cy).abs())
            .sum();
        if dis < 10000 {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
