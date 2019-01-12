use itertools::Itertools;
use std::collections::HashSet;

struct Claim {
    id: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

fn main() {
    let input = common::get_input(3).unwrap();
    let claims = input.trim_end().split('\n').collect_vec();
    let claims = claims
        .iter()
        .map(|s| {
            let cs = s.split(' ').collect_vec();
            let id: i32 = cs[0].trim_start_matches('#').parse().unwrap();
            let xy = cs[2]
                .trim_end_matches(':')
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect_vec();
            let wh = cs[3]
                .split('x')
                .map(|x| x.parse::<i32>().unwrap())
                .collect_vec();
            Claim {
                id,
                x: xy[0],
                y: xy[1],
                w: wh[0],
                h: wh[1],
            }
        })
        .collect_vec();

    let mut once = HashSet::new();
    let mut more = HashSet::new();
    for c in &claims {
        for i in c.x..c.x + c.w {
            for j in c.y..c.y + c.h {
                if !once.insert((i, j)) {
                    more.insert((i, j));
                }
            }
        }
    }
    println!("{}", more.len());

    for (i, ci) in claims.iter().enumerate() {
        let mut ok = true;
        for (j, cj) in claims.iter().enumerate() {
            if i != j
                && !(ci.x + ci.w <= cj.x
                    || cj.x + cj.w <= ci.x
                    || ci.y + ci.h <= cj.y
                    || cj.y + cj.h <= ci.y)
            {
                ok = false;
                break;
            }
        }
        if ok {
            println!("{}", ci.id);
        }
    }
}
