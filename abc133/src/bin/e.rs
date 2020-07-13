#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;
const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        k: usize,
        e: [(Usize1, Usize1);n-1],
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];

    for v in e {
        let (from, to) = v;
        graph[from].push(to);
        graph[to].push(from);
    }

    let mut seen: Vec<bool> = vec![false; n];

    let ans = dfs(0, &graph, &mut seen, k);
    println!("{}", ans);
}

fn dfs(i: usize, g: &Vec<Vec<usize>>, seen: &mut Vec<bool>, k: usize) -> usize {
    let mut seen_count = if i == 0 { 0 } else { 1 };
    let mut unseen_count = if i == 0 { 1 } else { 0 };

    seen[i] = true;

    let mut res = 1;

    for &next in &g[i] {
        if seen[next] {
            seen_count += 1;
            continue;
        }
        unseen_count += 1;
        res = res * dfs(next, g, seen, k) % MOD;
    }

    for i in 0..unseen_count {
        res = res * (k - seen_count - i) % MOD;
    }

    res
}
