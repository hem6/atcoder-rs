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
        e: [(Usize1, Usize1, usize);m]
    }

    let mut tree: Vec<Vec<usize>> = vec![vec![]; n];

    for &(from, to, w) in &e {
        tree[from].push(to);
        tree[to].push(from);
    }

    let mut seen: Vec<bool> = vec![false; n];
    let mut q: VecDeque<usize> = VecDeque::new();

    let mut ans = 0;

    for i in 0..n {
        if seen[i] {
            continue;
        }

        q.push_back(i);
        ans += 1;
        while let Some(v) = q.pop_front() {
            seen[v] = true;

            for &next in &tree[v] {
                if seen[next] {
                    continue;
                }
                q.push_back(next);
            }
        }
    }

    println!("{}", ans);
}
