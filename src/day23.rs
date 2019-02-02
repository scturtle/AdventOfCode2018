use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

struct Bot {
    c: Point,
    r: i32,
}

impl Point {
    fn dis(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Bot {
    fn dis(&self, p: &Point) -> i32 {
        (self.c.x - p.x).abs() + (self.c.y - p.y).abs() + (self.c.z - p.z).abs()
    }
    fn in_range(&self, other: &Point) -> bool {
        self.dis(other) <= self.r
    }
    fn corners(&self) -> Vec<Point> {
        vec![
            Point { x: self.c.x - self.r, y: self.c.y, z: self.c.z },
            Point { x: self.c.x + self.r, y: self.c.y, z: self.c.z },
            Point { x: self.c.x, y: self.c.y - self.r, z: self.c.z },
            Point { x: self.c.x, y: self.c.y + self.r, z: self.c.z },
            Point { x: self.c.x, y: self.c.y, z: self.c.z - self.r },
            Point { x: self.c.x, y: self.c.y, z: self.c.z + self.r },
        ]
    }
}

fn main() {
    let input = common::get_input(23).unwrap();
    let pat = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
    let mut bots = vec![];
    for l in input.lines() {
        let cap = pat.captures(l).unwrap();
        bots.push(Bot {
            c: Point {
                x: cap[1].parse().unwrap(),
                y: cap[2].parse().unwrap(),
                z: cap[3].parse().unwrap(),
            },
            r: cap[4].parse().unwrap(),
        });
    }
    let bot = bots.iter().max_by_key(|b| b.r).unwrap();
    println!("part1: {}", bots.iter().filter(|b| bot.in_range(&b.c)).count());

    let mut all_corners = bots
        .iter()
        .flat_map(|b| b.corners().into_iter())
        .collect_vec();
    all_corners.sort();
    all_corners.dedup();

    let count = |p: &Point| bots.iter().filter(|b| b.in_range(&p)).count();
    let mut corner_with_count = all_corners
        .into_iter()
        .map(|p| (p, bots.iter().filter(|b| b.in_range(&p)).count()))
        .collect_vec();
    corner_with_count.sort_by_key(|(_, c)| *c);
    corner_with_count.reverse();

    // greedy descent on top possible corners
    // very greedy but works unexpectedly
    for &(p, _) in corner_with_count.iter().take(30) {
        let mut p = p;
        let selected_bots = bots.iter().filter(|b| b.in_range(&p)).collect_vec();
        let ok = |np: &Point| selected_bots.iter().all(|b| b.in_range(np));
        'NEXT: loop {
            for &np in &(Bot { c: p, r: 1 }).corners() {
                if np.dis() < p.dis() && ok(&np) {
                    p = np;
                    continue 'NEXT;
                }
            }
            break;
        }
        println!("possible {} {}", count(&p), p.dis());
    }
}
