use itertools::Itertools;
use regex::Regex;

struct P {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

fn main() {
    let input = common::get_input(10).unwrap();
    let pat = Regex::new(r"position=< *(-?\d+), *(-?\d+)> velocity=< *(-?\d+), *(-?\d+)>").unwrap();
    let mut pts = input
        .trim_end()
        .lines()
        .map(|s| {
            let cap = pat.captures_iter(s).next().unwrap();
            P {
                x: cap[1].parse().unwrap(),
                y: cap[2].parse().unwrap(),
                vx: cap[3].parse().unwrap(),
                vy: cap[4].parse().unwrap(),
            }
        })
        .collect_vec();
    let mut sec = 0;
    loop {
        for p in &mut pts {
            p.x += p.vx;
            p.y += p.vy;
        }
        let minx = pts.iter().map(|p| p.x).min().unwrap();
        let maxx = pts.iter().map(|p| p.x).max().unwrap();
        let miny = pts.iter().map(|p| p.y).min().unwrap();
        let maxy = pts.iter().map(|p| p.y).max().unwrap();
        sec += 1;
        if maxx - minx > 64 || maxy - miny > 20 {
            continue;
        }
        let mut sky = vec![vec!['.'; (maxx - minx + 1) as usize]; (maxy - miny + 1) as usize];
        for p in &pts {
            sky[(p.y - miny) as usize][(p.x - minx) as usize] = '#';
        }
        for l in sky {
            println!("{}", l.into_iter().collect::<String>());
        }
        println!("sec: {}", sec);
        let mut buf = String::new();
        let _ = std::io::stdin().read_line(&mut buf);
    }
}
