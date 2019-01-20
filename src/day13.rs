use itertools::{iproduct, Itertools};

#[derive(Debug, Clone)]
struct Cart {
    i: i32,
    j: i32,
    d: u8, // ^,v,<,>
    turn_cnt: u8,
    crash: bool,
}

impl Cart {
    fn step(&mut self, m: &[Vec<u8>]) {
        match self.d {
            b'^' => self.i -= 1,
            b'v' => self.i += 1,
            b'<' => self.j -= 1,
            b'>' => self.j += 1,
            _ => unreachable!(),
        }
        match (self.d, m[self.i as usize][self.j as usize]) {
            (b'^', b'\\') | (b'v', b'\\') => self.turn_left(),
            (b'^', b'/') | (b'v', b'/') => self.turn_right(),
            (b'<', b'\\') | (b'>', b'\\') => self.turn_right(),
            (b'<', b'/') | (b'>', b'/') => self.turn_left(),
            (_, b'+') => self.turn(),
            _ => {}
        }
    }
    fn turn_left(&mut self) {
        let order = b"^>v<";
        let cur = order.iter().position(|c| c == &self.d).unwrap();
        self.d = order[((cur as i32 + 3) % 4) as usize]
    }
    fn turn_right(&mut self) {
        let order = b"^>v<";
        let cur = order.iter().position(|c| c == &self.d).unwrap();
        self.d = order[((cur as i32 + 1) % 4) as usize]
    }
    fn turn(&mut self) {
        match self.turn_cnt {
            0 => self.turn_left(),
            1 => {}
            2 => self.turn_right(),
            _ => unreachable!(),
        }
        self.turn_cnt = (self.turn_cnt + 1) % 3;
    }
}

fn main() {
    let input = common::get_input(13).unwrap();
    let map = input.lines().map(|s| s.bytes().collect_vec()).collect_vec();
    let (w, h) = (map[0].len(), map.len());
    let mut init = vec![];
    for (i, j) in iproduct!(0..h, 0..w) {
        let d = map[i][j];
        if b"^v<>".iter().any(|&b| b == d) {
            init.push(Cart {
                i: i as i32,
                j: j as i32,
                d,
                turn_cnt: 0,
                crash: false,
            });
        }
    }
    let n = init.len();
    let mut carts = init.clone();
    'LOOP: loop {
        carts.sort_by_key(|cart| (cart.i, cart.j));
        for a in 0..n {
            carts[a].step(&map);
            for b in 0..n {
                if a != b && carts[a].i == carts[b].i && carts[a].j == carts[b].j {
                    println!("{},{}", carts[a].j, carts[a].i);
                    break 'LOOP;
                }
            }
        }
    }
    let mut carts = init.clone();
    let mut remain = n;
    loop {
        carts.sort_by_key(|cart| (cart.i, cart.j));
        for a in 0..n {
            if !carts[a].crash {
                carts[a].step(&map);
                for b in 0..n {
                    if a != b
                        && !carts[b].crash
                        && carts[a].i == carts[b].i
                        && carts[a].j == carts[b].j
                    {
                        carts[a].crash = true;
                        carts[b].crash = true;
                        remain -= 2;
                    }
                }
            }
        }
        if remain == 1 {
            for cart in &carts {
                if !cart.crash {
                    println!("{},{}", cart.j, cart.i);
                    return;
                }
            }
        }
    }
}
