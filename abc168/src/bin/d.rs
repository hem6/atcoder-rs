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
        e: [[Usize1;2];m],
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for v in e {
        let (from, to) = (v[0], v[1]);
        graph[from].push(to);
        graph[to].push(from);
    }

    let mut seen: Vec<Option<usize>> = vec![None; n];
    seen[0] = Some(0);
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(0);

    while let Some(n) = q.pop_front() {
        for &next in &graph[n] {
            if seen[next].is_some() {
                continue;
            }
            q.push_back(next);
            seen[next] = Some(n);
        }
    }

    println!("Yes");
    for i in 1..n {
        if let Some(n) = seen[i] {
            println!("{}", n + 1);
        }
    }
}
