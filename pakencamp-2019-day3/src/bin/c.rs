use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[u64;m];n],
    }

    let mut ans = 0;

    for i in 0..m - 1 {
        for j in i + 1..m {
            let mut total = 0;

            for k in 0..n {
                total += std::cmp::max(a[k][i], a[k][j]);
            }

            ans = std::cmp::max(ans, total);
        }
    }

    println!("{}", ans);
}
