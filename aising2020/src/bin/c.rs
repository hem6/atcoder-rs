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

    let mut ans: Vec<u64> = vec![0; n + 1];

    for x in 1..=100 {
        for y in 1..=100 {
            for z in 1..=100 {
                let a = x * x + y * y + z * z + x * y + y * z + z * x;
                if n < a {
                    continue;
                }
                ans[a as usize] += 1;
            }
        }
    }

    for i in 1..=n {
        println!("{}", ans[i]);
    }
}
