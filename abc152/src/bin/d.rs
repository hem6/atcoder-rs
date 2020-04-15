#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: i32,
    }

    let ab: Vec<(i32, i32)> = (1..=n)
        .map(|i| {
            let a = i % 10;
            let mut b = i;
            while b / 10 > 0 {
                b = b / 10;
            }
            (a, b)
        })
        .collect();

    let mut freq: HashMap<(i32, i32), i32> = HashMap::new();

    for &p in &ab {
        freq.entry(p).and_modify(|i| *i += 1).or_insert(1);
    }

    let mut ans = 0;
    for &p in &ab {
        ans += freq.get(&(p.1, p.0)).unwrap_or(&0);
    }

    println!("{}", ans);
}
