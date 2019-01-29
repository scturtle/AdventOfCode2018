use std::collections::HashSet;

fn main() {
    let _ = common::get_input(21).unwrap();
    let mut r0s = vec![];
    let mut r5 = 65536;
    let mut r2 = 16123384;
    loop {
        r2 += r5 & 255;
        r2 = ((r2 & 16777215) * 65899) & 16777215;
        if 256 > r5 {
            r0s.push(r2);
            if r0s.len() > /*max_iters=*/100000 {
                break;
            }
            r5 = r2 | 65536;
            r2 = 16123384;
        } else {
            r5 /= 256;
        }
    }
    println!("{}", r0s[0]);
    let mut part2 = 0;
    let mut saw = HashSet::new();
    for r0 in r0s {
        if !saw.contains(&r0) {
            part2 = r0;
            saw.insert(r0);
        }
    }
    println!("{}", part2);
}
