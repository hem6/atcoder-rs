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
        q: usize,
        a: [[usize;4];q],
    }

    let mut ans = 0;
    let vecs = dfs(n, m, vec![1]);

    for v in vecs {
        let mut score = 0;
        for i in 0..q {
            let (a, b, c, d) = (a[i][0] - 1, a[i][1] - 1, a[i][2], a[i][3]);
            if v[b] >= v[a] && v[b] - v[a] == c {
                score += d;
            }
        }
        ans = max(ans, score);
    }

    println!("{}", ans);
}

fn dfs(depth: usize, m: usize, v: Vec<usize>) -> Vec<Vec<usize>> {
    if depth == v.len() {
        return vec![v];
    }

    let mut new_v: Vec<Vec<usize>> = Vec::new();
    let cur = v[v.len() - 1];

    for i in cur..=m {
        let mut next_v = v.clone();
        next_v.push(i);
        new_v.append(&mut dfs(depth, m, next_v));
    }

    new_v
}
