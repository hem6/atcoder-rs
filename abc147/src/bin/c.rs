#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;
use text_io::read;

fn main() {
    let n: usize = read!();

    let mut g: Vec<Vec<(usize, usize)>> = vec![vec![]; n];

    for i in 0..n {
        let a: usize = read!();
        for _ in 0..a {
            let x: usize = read!();
            let y: usize = read!();
            g[i].push((x - 1, y));
        }
    }

    let mut ans = 0;

    for i in 0..(1 << n) {
        let mut ok = true;

        for j in 0..n {
            if !ok {
                break;
            }
            if i & 1 << j > 0 {
                ok = g[j].iter().all(|&(x, y)| {
                    let honest = (i & 1 << x > 0) as usize;
                    honest == y
                });
            }
        }

        if ok {
            ans = max(ans, (i as usize).count_ones());
        }
    }

    println!("{}", ans);
}
