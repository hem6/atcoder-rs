use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [(Usize1, usize); m]
    }

    let mut t: Vec<Option<usize>> = vec![None; n];

    for (s, c) in a {
        if let Some(v) = t[s] {
            if v != c {
                println!("-1");
                return;
            }
        }

        t[s] = Some(c);
    }

    if t.len() > 1 && t[0] == Some(0) {
        println!("-1");
        return;
    }

    let mut t2: Vec<usize> = t.iter().map(|op| op.unwrap_or(0)).collect();

    let mut ans = 0;
    if t2.len() > 1 && t2[0] == 0 {
        t2[0] = 1;
    }

    for i in 0..n {
        ans = ans * 10 + t2[i];
    }

    println!("{}", ans);
}
