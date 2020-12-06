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

    let mut costs: Vec<Vec<i64>> = vec![vec![4_000_000; w]; h];
    let mut q: VecDeque<(usize, usize, i64)> = VecDeque::new();
    let mut gate: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for i in 0..h {
        for j in 0..w {
            if ('a'..='z').contains(&map[i][j]) {
                gate.entry(map[i][j]).or_insert(Vec::new()).push((i, j));
            } else if map[i][j] == 'S' {
                q.push_back((i, j, 0));
            }
        }
    }

    while let Some((y, x, cost)) = q.pop_front() {
        if cost >= costs[y][x] {
            continue;
        }

        if map[y][x] == 'G' {
            println!("{}", cost);
            return;
        }

        if 0 < y && map[y - 1][x] != '#' {
            q.push_back((y - 1, x, cost + 1));
        }
        if 0 < x && map[y][x - 1] != '#' {
            q.push_back((y, x - 1, cost + 1));
        }
        if y < h - 1 && map[y + 1][x] != '#' {
            q.push_back((y + 1, x, cost + 1));
        }
        if x < w - 1 && map[y][x + 1] != '#' {
            q.push_back((y, x + 1, cost + 1));
        }

        if ('a'..='z').contains(&map[y][x]) && costs[y][x] == 4_000_000 {
            for &(ny, nx) in gate.get(&map[y][x]).unwrap() {
                if ny == y && nx == x {
                    continue;
                }

                costs[ny][nx] = cost + 2;
                q.push_back((ny, nx, cost + 1));
            }
        }

        costs[y][x] = cost;
    }

    println!("-1");
}
