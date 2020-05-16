#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        a: [isize],
    }

    let mut m: HashMap<isize, usize> = HashMap::new();
    m.insert(0, 1);
    let mut sum = 0;
    for i in a {
        sum += i;
        *m.entry(sum).or_insert(0) += 1;
    }

    let mut ans = 0;
    for (_, v) in m {
        ans += v * (v - 1) / 2;
    }

    println!("{}", ans);
}
