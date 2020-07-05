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
        k: usize,
        map: [Chars;h],
    }

    let mut ans = 0;

    for i in 0..1 << h {
        for j in 0..1 << w {
            let mut k_count = 0;

            for cur_h in 0..h {
                if i >> cur_h & 1 == 1 {
                    continue;
                }
                for cur_w in 0..w {
                    if j >> cur_w & 1 == 1 {
                        continue;
                    }

                    if map[cur_h][cur_w] == '#' {
                        k_count += 1;
                    }
                }
            }

            if k_count == k {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
