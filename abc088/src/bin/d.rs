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
        map: [Chars; h],
    }

    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut dist: Vec<Vec<i32>> = vec![vec![-1; w]; h];

    q.push_back((0, 0));
    dist[0][0] = 0;

    while let Some((cy, cx)) = q.pop_front() {
        let mut next = Vec::with_capacity(4);

        if cy > 0 {
            next.push((cy - 1, cx));
        }
        if cy < h - 1 {
            next.push((cy + 1, cx));
        }
        if cx > 0 {
            next.push((cy, cx - 1));
        }
        if cx < w - 1 {
            next.push((cy, cx + 1));
        }

        for (ny, nx) in next {
            if dist[ny][nx] != -1 || map[ny][nx] == '#' {
                continue;
            }

            dist[ny][nx] = dist[cy][cx] + 1;
            q.push_back((ny, nx));
        }
    }

    if dist[h - 1][w - 1] == -1 {
        println!("-1");
        return;
    }

    let blank_cell = map
        .iter()
        .map(|chars| chars.iter().filter(|&c| *c == '.').count())
        .sum::<usize>();

    println!("{}", blank_cell - dist[h - 1][w - 1] as usize - 1);
}
