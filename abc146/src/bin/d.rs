#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        e: [[Usize1;2];n-1],
    }

    let mut graph: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
    for (i, v) in e.iter().enumerate() {
        let (from, to) = (v[0], v[1]);
        graph[from].push((to, i));
        graph[to].push((from, i));
    }

    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((0, 0));

    let mut edges: Vec<usize> = vec![0; n - 1];

    while let Some((node, edge)) = q.pop_front() {
        let mut next_color = 1;
        for &(to, i) in &graph[node] {
            if edges[i] > 0 {
                continue;
            }
            if next_color == edges[edge] {
                next_color += 1;
            }
            edges[i] = next_color;
            q.push_back((to, i));
            next_color += 1;
        }
    }

    println!("{}", edges.iter().max().unwrap());

    for i in edges {
        println!("{}", i);
    }
}
