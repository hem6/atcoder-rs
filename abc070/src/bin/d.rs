#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        e: [(Usize1, Usize1, u64);n-1],
        q: usize,
        k: Usize1,
        query: [(Usize1, Usize1);q]
    }

    let mut graph: Vec<Vec<(usize, u64)>> = vec![vec![]; n];
    for v in e {
        let (from, to, d) = (v.0, v.1, v.2);
        graph[from].push((to, d));
        graph[to].push((from, d));
    }

    let mut seen: Vec<u64> = vec![0; n];
    let mut q: VecDeque<(usize, u64)> = VecDeque::new();
    for &next in &graph[k] {
        q.push_back(next);

        while let Some((current, d)) = q.pop_front() {
            seen[current] = d;
            // なんらかの処理
            for &(next, next_d) in &graph[current] {
                if seen[next] > 0 || next == k {
                    continue;
                }
                q.push_back((next, d + next_d));
            }
        }
    }

    for &(from, to) in &query {
        println!("{}", seen[from] + seen[to]);
    }
}
