use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Group {
    side: String,
    units: i32,
    hp: i32,
    immune_to: Vec<String>,
    weak_to: Vec<String>,
    damage: i32,
    damage_type: String,
    initiative: i32,
}

impl Group {
    fn from_cap(cap: regex::Captures<'_>, side: String) -> Group {
        let immune_pat = Regex::new(r"immune to ([^;]+)").unwrap();
        let weak_pat = Regex::new(r"weak to ([^;]+)").unwrap();
        let inner = cap.get(3).map_or("", |m| m.as_str());
        let immunes = immune_pat.captures(inner).map_or(vec![], |c| {
            c.get(1).map_or(vec![], |m| {
                m.as_str().split(", ").map(|w| w.to_owned()).collect_vec()
            })
        });
        let weaks = weak_pat.captures(inner).map_or(vec![], |c| {
            c.get(1).map_or(vec![], |m| {
                m.as_str().split(", ").map(|w| w.to_owned()).collect_vec()
            })
        });
        Group {
            side,
            units: cap[1].parse().unwrap(),
            hp: cap[2].parse().unwrap(),
            immune_to: immunes,
            weak_to: weaks,
            damage: cap[4].parse().unwrap(),
            damage_type: cap[5].to_owned(),
            initiative: cap[6].parse().unwrap(),
        }
    }
}

fn main() {
    let boost: i32 = std::env::args().nth(1).map(|s| s.parse().unwrap()).unwrap_or(0);
    let input = common::get_input(24).unwrap();
    let mut lines = input.lines();
    let pat = Regex::new(
r"(\d+) units each with (\d+) hit points(?: \((.*)\))? with an attack that does (\d+) (\w+) damage at initiative (\d+)"
        ).unwrap();
    let mut groups = vec![];
    lines.next();
    let mut side = "immune";
    while let Some(l) = lines.next() {
        if let Some(cap) = pat.captures(l) {
            groups.push(Group::from_cap(cap, side.to_owned()));
        } else {
            for g in &mut groups {
                g.damage += boost;
            }
            side = "infection";
            lines.next();
        }
    }
    // dbg!(&groups);
    // dbg!(groups.len());
    loop {
        // target selection
        let mut order = (0..groups.len()).collect_vec();
        order.sort_by_key(|&i| (-groups[i].units * groups[i].damage, -groups[i].initiative));
        let mut chosen = vec![false; groups.len()];
        let mut target = vec![None; groups.len()];
        for &i in &order {
            let g = &groups[i];
            if g.units <= 0 {
                continue;
            }
            let mut sel = None; // index
            let mut stat = (0, 0, 0, 0); // (damage, eff, init, multi)
            for j in 0..groups.len() {
                let t = &groups[j];
                if t.side == g.side || t.units <= 0 || chosen[j] {
                    continue;
                }
                let multi =
                    if t.immune_to.iter().any(|x| x == &g.damage_type) {
                        continue;
                    } else if t.weak_to.iter().any(|x| x == &g.damage_type) {
                        2
                    } else {
                        1
                    };
                let d = g.units * g.damage * multi;
                // if d < t.hp {continue;}
                let e = t.units * t.damage;
                if sel.is_none() ||
                    d > stat.0 ||
                    (d == stat.0 && e > stat.1) ||
                    (d == stat.0 && e == stat.1 && t.initiative > stat.2) {
                    sel = Some(j);
                    stat = (d, e, t.initiative, multi);
                    // eprintln!("{} would deal {} damage to {}", g.initiative, d, t.initiative);
                }
            }
            if let Some(j) = sel {
                chosen[j] = true;
            }
            target[i] = sel.map(|j| (j, stat.3));
        }
        // attacking
        order.sort_by_key(|&i| -groups[i].initiative);
        for i in order {
            // let init_i = groups[i].initiative;
            let side_i = groups[i].side.clone();
            let eff_i = groups[i].units * groups[i].damage;
            if groups[i].units <= 0 {
                continue;
            }
            if let Some((j, multi)) = target[i] {
                let t = &mut groups[j];
                assert!(t.side != side_i);
                if t.units <= 0 {
                    continue;
                }
                let d = eff_i * multi;
                let loss = d / t.hp;
                t.units -= loss;
                // eprintln!("{} deal {} damage to {} kill {} units", init_i, d, t.initiative, loss);
            }
        }
        groups.retain(|g| g.units > 0);
        // dbg!(&groups);
        // exit?
        let mut alive_sides = groups.iter().map(|g| g.side.clone()).collect_vec();
        alive_sides.sort();
        alive_sides.dedup();
        if alive_sides.len() < 2 {
            break;
        }
    }
    eprintln!("#units: {}", groups.iter().map(|g| g.units).sum::<i32>());
    eprintln!("boost: {} winner: {}", boost, groups[0].side);
}
