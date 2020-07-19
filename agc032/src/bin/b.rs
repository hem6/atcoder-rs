#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
    }

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];
    let mut count = 0;

    for i in 1..n {
        for j in i + 1..=n {
            if n % 2 == 0 && i + j == n + 1 {
                continue;
            } else if n % 2 == 1 && i + j == n {
                continue;
            }
            graph[i].push(j);
            count += 1;
        }
    }

    println!("{}", count);
    for i in 1..=n {
        for &j in &graph[i] {
            println!("{} {}", i, j);
        }
    }
}
