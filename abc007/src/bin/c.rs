#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        r: usize,
        c: usize,
        sy: Usize1,
        sx: Usize1,
        gy: Usize1,
        gx: Usize1,
        map: [Chars;r]
    }

    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((sy, sx));

    let mut dist: Vec<Vec<i32>> = vec![vec![-1; c as usize]; r as usize];
    dist[sy][sx] = 0;

    while let Some((cy, cx)) = q.pop_front() {
        let (uy, ux) = (cy - 1, cx);
        let (dy, dx) = (cy + 1, cx);
        let (ry, rx) = (cy, cx + 1);
        let (ly, lx) = (cy, cx - 1);

        if map[uy][ux] != '#' && dist[uy][ux] == -1 {
            dist[uy][ux] = dist[cy][cx] + 1;
            q.push_back((uy, ux));
        }
        if map[dy][dx] != '#' && dist[dy][dx] == -1 {
            dist[dy][dx] = dist[cy][cx] + 1;
            q.push_back((dy, dx));
        }
        if map[ry][rx] != '#' && dist[ry][rx] == -1 {
            dist[ry][rx] = dist[cy][cx] + 1;
            q.push_back((ry, rx));
        }
        if map[ly][lx] != '#' && dist[ly][lx] == -1 {
            dist[ly][lx] = dist[cy][cx] + 1;
            q.push_back((ly, lx));
        }
    }

    println!("{}", dist[gy][gx]);
}
