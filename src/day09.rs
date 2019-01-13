use itertools::Itertools;
use std::collections::HashMap;

struct Node {
    val: i32,
    prev: usize,
    next: usize,
}

struct List {
    nodes: Vec<Node>,
    curr: usize,
}

impl List {
    fn new() -> List {
        let (val, prev, next) = (0, 0, 0);
        List {
            nodes: vec![Node { val, prev, next }],
            curr: 0,
        }
    }
    fn step(&mut self) {
        self.curr = self.nodes[self.curr].next
    }
    fn back(&mut self) {
        self.curr = self.nodes[self.curr].prev
    }
    fn val(&self) -> i32 {
        self.nodes[self.curr].val
    }
    fn add(&mut self, val: i32) {
        let next = self.nodes[self.curr].next;
        self.nodes.push(Node {
            val,
            prev: self.curr,
            next,
        });
        let added = self.nodes.len() - 1;
        self.nodes[self.curr].next = added;
        self.nodes[next].prev = added;
        self.curr = added;
    }
    fn del(&mut self) {
        let prev = self.nodes[self.curr].prev;
        let next = self.nodes[self.curr].next;
        self.nodes[prev].next = next;
        self.nodes[next].prev = prev;
        self.curr = next;
    }
}

fn main() {
    let input = common::get_input(9).unwrap();
    let (n_players, n_marble) = {
        let cs = input.split(' ').collect_vec();
        (cs[0].parse::<i32>().unwrap(), cs[6].parse::<i32>().unwrap())
    };
    for &n_marble in &[n_marble, n_marble * 100] {
        let mut scores: HashMap<i32, u64> = HashMap::new();
        let mut l = List::new();
        for m in 1..=n_marble {
            let p = ((m - 1) % n_players) + 1;
            if m % 23 == 0 {
                let score = scores.entry(p).or_insert(0);
                *score += m as u64;
                for _ in 0..7 {
                    l.back();
                }
                *score += l.val() as u64;
                l.del();
            } else {
                l.step();
                l.add(m);
            }
        }
        println!("{}", scores.iter().map(|(_, v)| v).max().unwrap());
    }
}
