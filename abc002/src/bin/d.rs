use proconio::input;

fn main() {
    input! {
        n:u32,
        m: usize,
        a: [[u32;2]; m]
    }

    let rs: Vec<u32> = a
        .iter()
        .map(|v| 1 << (v[0] - 1) | 1 << (v[1] - 1))
        .collect();

    let mut ans = 0;

    for i in 0..(1 << n) {
        let num = (i as u32).count_ones();
        if num <= ans {
            continue;
        }

        let count = rs.iter().filter(|r| i & **r == **r).count();
        if count == (num * (num - 1) / 2) as usize {
            ans = num;
        }
    }

    println!("{}", ans);
}
