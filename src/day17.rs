use regex::Regex;
use std::io::prelude::*;

fn main() {
    let input = common::get_input(17).unwrap();
    let pat1 = Regex::new(r"(.)=(\d+), .=(\d+)..(\d+)").unwrap();
    let mut tiles = vec![];
    for l in input.lines() {
        let cap = pat1.captures(l).unwrap();
        let s: usize = cap[2].parse().unwrap();
        let t1: usize = cap[3].parse().unwrap();
        let t2: usize = cap[4].parse().unwrap();
        for t in t1..=t2 {
            let p = if &cap[1] == "x" { (s, t) } else { (t, s) };
            tiles.push(p);
        }
    }
    let minx = tiles.iter().map(|&(x, _)| x).min().unwrap();
    let maxx = tiles.iter().map(|&(x, _)| x).max().unwrap();
    let miny = tiles.iter().map(|&(_, y)| y).min().unwrap();
    let maxy = tiles.iter().map(|&(_, y)| y).max().unwrap();
    let (minx, maxx) = (minx - 1, maxx + 1); // NOTE
    let width = maxx - minx + 1;
    let height = maxy - miny + 1;
    let mut m = vec![vec![b'.'; width]; height];
    for (x, y) in tiles {
        m[y - miny][x - minx] = b'#';
    }
    let mut f = std::fs::File::create("17.log").unwrap();

    m[0][500 - minx] = b'|';
    let mut q = vec![(500 - minx, 0)];
    while let Some((x, mut y)) = q.pop() {
        // down
        while y + 1 < height && m[y + 1][x] == b'.' {
            y += 1;
            m[y][x] = b'|';
        }
        // out of screen
        if y + 1 == height {
            continue;
        }
        // NOTE skip the flow surface
        if m[y + 1][x] == b'-' {
            continue;
        }
        m[y][x] = b'~';
        // flow back
        loop {
            // extend to left
            let mut lx = x;
            while lx > 0 && m[y][lx - 1] != b'#' && m[y + 1][lx] != b'.' && m[y + 1][lx] != b'|' {
                lx -= 1;
                m[y][lx] = b'~';
            }
            // extend to right
            let mut rx = x;
            while rx + 1 < width && m[y][rx + 1] != b'#' && m[y + 1][rx] != b'.' && m[y + 1][rx] != b'|' {
                rx += 1;
                m[y][rx] = b'~';
            }
            // back one step
            if lx > 0 && rx + 1 < width && m[y][lx - 1] != b'.' && m[y][rx + 1] != b'.' {
                y -= 1;
                m[y][x] = b'~';
                continue;
            }
            // mark the flow surface
            for x in lx..=rx {
                m[y][x] = b'-';
            }
            // new spring point
            if m[y + 1][rx] == b'.' {
                q.push((rx, y));
            }
            if m[y + 1][lx] == b'.' {
                q.push((lx, y));
            }
            break;
        }
    }
    f.write_all(&m.join(&b'\n')).unwrap();
    println!(
        "{}",
        m.iter()
            .map(|v| v.iter().filter(|&&b| b != b'.' && b != b'#').count())
            .sum::<usize>()
    );
    println!(
        "{}",
        m.iter()
            .map(|v| v.iter().filter(|&&b| b == b'~').count())
            .sum::<usize>()
    );
}
