fn last_n(v: &[u8], n: usize) -> u64 {
    let v_n = v.len();
    (0..n).fold(0u64, |r, i| 10 * r + u64::from(v[v_n - n + i]))
}

fn produce(v: &mut Vec<u8>, e1: &mut usize, e2: &mut usize) {
    let s = v[*e1] + v[*e2];
    if s >= 10 {
        v.push(s / 10);
    }
    v.push(s % 10);
    *e1 = (*e1 + v[*e1] as usize + 1) % v.len();
    *e2 = (*e2 + v[*e2] as usize + 1) % v.len();
}

fn main() {
    let mut v: Vec<u8> = vec![3, 7];
    let (mut e1, mut e2) = (0, 1);
    let n = 360_781;
    while v.len() < 10 + n {
        produce(&mut v, &mut e1, &mut e2);
    }
    println!("{}", last_n(&v, 10));

    let nn = n.to_string().len();
    v = vec![3, 7];
    e1 = 0;
    e2 = 1;
    loop {
        produce(&mut v, &mut e1, &mut e2);
        if v.len() > nn {
            if last_n(&v, nn + 1) / 10 == n as u64 {
                v.pop();
                break;
            } else if last_n(&v, nn) == n as u64 {
                break;
            }
        }
    }
    println!("{}", v.len() - nn);
}
