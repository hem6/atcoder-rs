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
        map: [Chars;h],
    }

    // (x, y, score)
    let mut q: VecDeque<(usize, usize, isize)> = VecDeque::new();

    let mut ans = 0;

    for i in 0..w {
        for j in 0..h {
            let mut seen: Vec<Vec<bool>> = vec![vec![false; w]; h];
            q.push_back((i, j, 0));

            while let Some((x, y, score)) = q.pop_front() {
                if seen[y][x] || map[y][x] == '#' {
                    ans = max(ans, score - 1);
                    continue;
                }

                seen[y][x] = true;

                if x > 0 {
                    q.push_back((x - 1, y, score + 1));
                }
                if x < w - 1 {
                    q.push_back((x + 1, y, score + 1));
                }
                if y > 0 {
                    q.push_back((x, y - 1, score + 1));
                }
                if y < h - 1 {
                    q.push_back((x, y + 1, score + 1));
                }
            }
        }
    }

    println!("{}", ans);
}
