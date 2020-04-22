#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        d: usize,
        x: [[i32;d];n]
    }

    let mut ans = 0;

    for i in 0..n - 1 {
        for j in i + 1..n {
            let mut dst = 0;
            for k in 0..d {
                dst += (x[i][k] - x[j][k]).pow(2) as usize;
            }

            dbg!(d);

            for l in 0..=dst {
                if l.pow(2) == dst {
                    ans += 1;
                    break;
                } else if l.pow(2) > dst {
                    break;
                }
            }
        }
    }

    println!("{}", ans);
}
