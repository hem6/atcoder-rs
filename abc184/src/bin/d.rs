#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    let mut dp = vec![vec![vec![0.0; 101]; 101]; 101];

    let mut q: VecDeque<(usize, usize, usize)> = VecDeque::new();
    q.push_back((99, 99, 99));

    while let Some((x, y, z)) = q.pop_front() {
        if dp[x][y][z] != 0.0 {
            continue;
        }

        dp[x][y][z] = (dp[x + 1][y][z] + 1.0) * x as f64 / (x + y + z) as f64
            + (dp[x][y + 1][z] + 1.0) * y as f64 / (x + y + z) as f64
            + (dp[x][y][z + 1] + 1.0) * z as f64 / (x + y + z) as f64;

        if x > 0 && dp[x - 1][y][z] == 0.0 {
            q.push_back((x - 1, y, z));
        }
        if y > 0 && dp[x][y - 1][z] == 0.0 {
            q.push_back((x, y - 1, z));
        }
        if z > 0 && dp[x][y][z - 1] == 0.0 {
            q.push_back((x, y, z - 1));
        }
    }

    println!("{}", dp[a][b][c]);
}
