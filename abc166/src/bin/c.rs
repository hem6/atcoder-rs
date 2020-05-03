#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        h: [usize;n],
        e: [[Usize1;2];m]
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];

    for v in e {
        let (from, to) = (v[0], v[1]);
        graph[from].push(to);
        graph[to].push(from);
    }

    let mut ans = 0;

    for i in 0..n {
        let mut good_flg = true;
        for &other in &graph[i] {
            if h[i] <= h[other] {
                good_flg = false;
                break;
            }
        }

        if good_flg {
            ans += 1;
        }
    }

    println!("{}", ans);
}
