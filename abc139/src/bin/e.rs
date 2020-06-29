#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [[Usize1;n-1];n],
    }

    let mut table: Vec<Vec<usize>> = vec![vec![0; n]; n];
    let mut count = 1;

    for i in 0..n {
        for j in 0..n {
            table[i][j] = count;
            count += 1;
        }
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n * n + 1];

    for i in 0..n {
        let opponents = &a[i];
        let mut prev_match = 0;

        for &j in opponents {
            let match_no = if i < j { table[i][j] } else { table[j][i] };
            if prev_match > 0 {
                graph[prev_match].push(match_no);
            }
            prev_match = match_no;
        }
    }

    let mut seen: Vec<bool> = vec![false; n * n + 1];
    let mut dp: Vec<i64> = vec![0; n * n + 1];
    let mut calculated: Vec<bool> = vec![false; n * n + 1];

    let mut ans = 0;

    for i in 0..=n * n {
        if seen[i] {
            continue;
        }

        let score = dfs(i, &graph, &mut seen, &mut dp, &mut calculated);

        if score == -1 {
            println!("-1");
            return;
        }

        ans = max(ans, score + 1);
    }

    println!("{}", ans);
}

fn dfs(
    current: usize,
    graph: &Vec<Vec<usize>>,
    seen: &mut Vec<bool>,
    dp: &mut Vec<i64>,
    calculated: &mut Vec<bool>,
) -> i64 {
    if seen[current] {
        if !calculated[current] {
            return -1;
        } else {
            return dp[current];
        }
    }

    seen[current] = true;

    for &next in &graph[current] {
        let score = dfs(next, graph, seen, dp, calculated);

        if score == -1 {
            return -1;
        }

        dp[current] = max(dp[current], score + 1);
    }

    calculated[current] = true;
    dp[current]
}
