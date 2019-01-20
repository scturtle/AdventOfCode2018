use itertools::{iproduct, Itertools};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
enum Role {
    Elf,
    Goblin,
}

impl Role {
    fn from_u8(b: u8) -> Role {
        match b {
            b'E' => Role::Elf,
            b'G' => Role::Goblin,
            _ => unreachable!(),
        }
    }
    fn to_u8(&self) -> u8 {
        match self {
            Role::Elf => b'E',
            Role::Goblin => b'G',
        }
    }
}

#[derive(Debug, Clone)]
struct Unit {
    role: Role,
    x: i32,
    y: i32,
    hp: i32,
}

type BfsResult = (i32, (i32, i32), (i32, i32));

impl Unit {
    fn around_xy(x: i32, y: i32) -> std::vec::IntoIter<(i32, i32)> {
        vec![(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)].into_iter()
    }
    fn around(&self) -> std::vec::IntoIter<(i32, i32)> {
        Unit::around_xy(self.x, self.y)
    }
    fn bfs(&self, map: &[Vec<u8>], all_units: &[Unit], x: i32, y: i32) -> Option<BfsResult> {
        let dests = self.around().collect_vec();
        let mut distance = 0;
        let mut cur = vec![(x, y)];
        let mut saw: HashSet<(i32, i32)> = cur.iter().cloned().collect();
        while !cur.is_empty() {
            let mut reached = cur
                .iter()
                .filter(|p| dests.iter().any(|q| p == &q))
                .collect_vec();
            if !reached.is_empty() {
                reached.sort();
                // first ordered by distance, then ordered by start point
                return reached.iter().map(|p| (distance, (x, y), **p)).nth(0);
            }
            let mut nxt = vec![];
            for (x, y) in cur {
                for p in Unit::around_xy(x, y) {
                    if !saw.contains(&p)
                        && map[p.0 as usize][p.1 as usize] != b'#'
                        // no collision with any alive unit
                        && !all_units
                            .iter()
                            .filter(|u| u.hp > 0)
                            .any(|u| p == (u.x, u.y))
                    {
                        saw.insert(p);
                        nxt.push(p);
                    }
                }
            }
            distance += 1;
            cur = nxt;
        }
        None
    }
    fn step(&self, map: &[Vec<u8>], all_units: &[Unit]) -> Option<(i32, i32)> {
        // already around alive enemy
        for (p, u) in iproduct!(self.around(), all_units.iter()) {
            if u.hp > 0 && u.role != self.role && p == (u.x, u.y) {
                return None;
            }
        }
        let mut squares = all_units
            .iter()
            // all alive enemies' open around squares
            .filter(|u| u.hp > 0 && u.role != self.role)
            .flat_map(|u| {
                u.around()
                    .filter(|&(x, y)| map[x as usize][y as usize] != b'#')
            })
            // but not occupied by any alive unit
            .filter(|&p| {
                all_units
                    .iter()
                    .filter(|u| u.hp > 0)
                    .all(|u| (u.x, u.y) != p)
            })
            .collect_vec();
        squares.sort();
        squares.dedup();
        // for each dest, find the best way back to around of this unit
        let mut ways = squares
            .iter()
            .filter_map(|&(x, y)| self.bfs(map, all_units, x, y))
            .collect_vec();
        ways.sort();
        ways.iter().map(|(_, _, p)| *p).nth(0)
    }
}

fn display_map(map: &mut [Vec<u8>], all_units: &[Unit]) {
    for u in all_units {
        if u.hp > 0 {
            map[u.x as usize][u.y as usize] = u.role.to_u8();
        }
    }
    println!(
        "{}",
        map.iter()
            .map(|l| std::str::from_utf8(l).unwrap())
            .collect_vec()
            .join("\n")
    );
    for u in all_units {
        if u.hp > 0 {
            map[u.x as usize][u.y as usize] = b'.';
        }
    }
}

fn units_status(units: &[Unit]) {
    for u in units {
        println!("({},{}) {:?} {}", u.x, u.y, u.role, u.hp);
    }
}

fn main() {
    let input = common::get_input(15).unwrap();
    let mut map = input.lines().map(|l| l.bytes().collect_vec()).collect_vec();
    let mut units = vec![];
    for (i, j) in iproduct!(0..map.len(), 0..map[0].len()) {
        let cell = map[i][j];
        if cell != b'.' && cell != b'#' {
            map[i][j] = b'.';
            units.push(Unit {
                role: Role::from_u8(cell),
                x: i as i32,
                y: j as i32,
                hp: 200,
            });
        }
    }
    display_map(&mut map, &units);
    units_status(&units);
    let mut round = 0;
    // custom elf attack power for part 2
    let elf_attack_power: i32 = std::env::var("p")
        .unwrap_or_else(|_| "3".to_owned())
        .parse()
        .unwrap();
    'OUTER: loop {
        round += 1;
        println!("round: {}", round);
        // order of move / attach
        units.sort_by_key(|u| (u.x, u.y));
        let n = units.len();
        for i in 0..n {
            if units[i].hp <= 0 {
                // NOTE
                continue;
            }
            let possible_move = units[i].step(&map, &units);
            if let Some((x, y)) = possible_move {
                // println!("({},{}) move to ({},{})", units[i].x, units[i].y, x, y);
                units[i].x = x;
                units[i].y = y;
            }
            let arounds = units[i].around().collect_vec();
            let mut enemies = units
                .iter()
                .enumerate()
                // alive enemy to attach around after moved
                .filter(|(_, u)| {
                    u.hp > 0 && u.role != units[i].role && arounds.iter().any(|&p| p == (u.x, u.y))
                })
                .collect_vec();
            // prefer lower hp, then reading order
            enemies.sort_by_key(|(_, u)| (u.hp, u.x, u.y));
            if let Some((j, _)) = enemies.into_iter().nth(0) {
                if units[i].role == Role::Goblin {
                    units[j].hp -= 3;
                } else {
                    units[j].hp -= elf_attack_power;
                };
                // to manually bisect the answer of part 2
                if units[j].hp <= 0 && units[j].role == Role::Elf && elf_attack_power > 3 {
                    panic!("elf died");
                }
                // println!("{},{} hit {},{} to {}", units[i].x, units[i].y, units[j].x, units[j].y, units[j].hp);
            }
            if !units.iter().any(|u| u.hp > 0 && u.role == Role::Elf)
                || !units.iter().any(|u| u.hp > 0 && u.role == Role::Goblin)
            {
                // NOTE: this is ridiculous
                if units[i + 1..].iter().any(|u| u.hp > 0) {
                    round -= 1;
                }
                break 'OUTER;
            }
        }
        units.retain(|u| u.hp > 0);
        display_map(&mut map, &units);
        units_status(&units);
    }
    units_status(&units);
    println!("{}", round * units.iter().map(|u| u.hp.max(0)).sum::<i32>());
}
