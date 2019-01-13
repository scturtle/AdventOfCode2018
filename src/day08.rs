use itertools::Itertools;

struct Node {
    cs: Vec<Node>,
    meta: Vec<usize>,
}

fn build(vs: &[usize], st: usize) -> (Node, usize) {
    let cl = vs[st];
    let ml = vs[st + 1];
    let mut st = st + 2;
    let mut node = Node {
        cs: vec![],
        meta: vec![],
    };
    for _ in 0..cl {
        let (c, new_st) = build(vs, st);
        st = new_st;
        node.cs.push(c);
    }
    for i in 0..ml {
        node.meta.push(vs[st + i]);
    }
    (node, st + ml)
}

fn sum_meta(rt: &Node) -> usize {
    rt.cs.iter().map(|c| sum_meta(c)).sum::<usize>() + rt.meta.iter().sum::<usize>()
}

fn value(rt: &Node) -> usize {
    if rt.cs.is_empty() {
        rt.meta.iter().sum()
    } else {
        let cvs = rt.cs.iter().map(|c| value(c)).collect_vec();
        rt.meta
            .iter()
            .filter_map(|&m| {
                if m > 0 && m <= rt.cs.len() {
                    Some(cvs[m - 1])
                } else {
                    None
                }
            })
            .sum::<usize>()
    }
}

fn main() {
    let vs = common::get_input(8)
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect_vec();
    let root = build(&vs, 0).0;
    println!("{}", sum_meta(&root));
    println!("{}", value(&root));
}
