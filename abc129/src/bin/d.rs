#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        h: usize,
        w: usize,
        m: [Chars;h],
    }

    let mut map_x: Vec<Vec<usize>> = vec![vec![0; w]; h];

    for i in 0..h {
        let mut r = 0;
        let mut score = 0;
        for l in 0..w {
            while r < w && m[i][r] == '.' {
                score += 1;
                r += 1;
            }

            if l == r {
                score = 0;
                r += 1;
            }
            map_x[i][l] = score;
        }
    }

    let mut map_y: Vec<Vec<usize>> = vec![vec![0; w]; h];

    for i in 0..w {
        let mut b = 0;
        let mut score = 0;
        for t in 0..h {
            while b < h && m[b][i] == '.' {
                score += 1;
                b += 1;
            }

            if t == b {
                score = 0;
                b += 1;
            }
            map_y[t][i] = score;
        }
    }

    let mut ans = 0;

    for i in 0..h {
        for j in 0..w {
            if map_x[i][j] + map_y[i][j] == 0 {
                continue;
            }
            ans = max(ans, map_x[i][j] + map_y[i][j] - 1);
        }
    }

    println!("{}", ans);
}
