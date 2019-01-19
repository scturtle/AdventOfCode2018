use itertools::iproduct;

fn main() {
    let serial_number = 5177;
    let mut m = vec![[0i32; 300]; 300];
    for (x, y) in iproduct!(1..=300, 1..=300) {
        let rack_id = x + 10i32;
        let power_level = rack_id * y;
        let t = (power_level + serial_number) * rack_id;
        let fuel = (t / 100) % 10 - 5;
        m[(x - 1) as usize][(y - 1) as usize] = fuel;
    }
    let (mx, my) = iproduct!(1..=300 - 3, 1..=300 - 3)
        .max_by_key(|(x, y)| {
            iproduct!(0..3, 0..3)
                .map(|(i, j)| m[(x - 1 + i) as usize][(y - 1 + j) as usize])
                .sum::<i32>()
        })
        .unwrap();
    println!("{},{}", mx, my);
    let mut max = std::i32::MIN;
    let mut res = (0, 0, 0);
    for (x, y) in iproduct!(1..=300 - 3, 1..=300 - 3) {
        // dbg!((x, y));
        let n: i32 = (300 + 1 - x).min(300 + 1 - y);
        for n in 1..=n {
            let sum = iproduct!(0..n, 0..n)
                .map(|(i, j)| unsafe {
                    m.get_unchecked((x - 1 + i) as usize)
                        .get_unchecked((y - 1 + j) as usize)
                })
                .sum::<i32>();
            if sum > max {
                max = sum;
                res = (x, y, n);
            }
        }
    }
    println!("{},{},{}", res.0, res.1, res.2);
}
