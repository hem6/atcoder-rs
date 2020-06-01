#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        e: [(Usize1, Usize1, u64);n-1]
    }

    let mut color: Vec<bool> = vec![false; n];

    let mut tree: Vec<Vec<(usize, u64)>> = vec![vec![]; n];

    for &(from, to, w) in &e {
        tree[from].push((to, w));
        tree[to].push((from, w));
    }

    let mut seen: Vec<bool> = vec![false; n];
    let mut q: VecDeque<(bool, usize, u64)> = VecDeque::new();
    q.push_back((true, 0, 0));

    while let Some((par_color, i, w)) = q.pop_front() {
        seen[i] = true;

        if w % 2 == 0 {
            color[i] = par_color;
        } else {
            color[i] = !par_color;
        }

        for &(next, w) in &tree[i] {
            if seen[next] {
                continue;
            }
            q.push_back((color[i], next, w));
        }
    }

    for c in color {
        if c {
            println!("1");
        } else {
            println!("0");
        }
    }
}
