#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        map: [[usize;n];n]
    }

    let mut res = 0;

    let pattern: Vec<usize> = (1..n).collect();
    for ptn in pattern.iter().permutations(n - 1) {
        let mut cost = 0;
        let mut bef = 0;

        for &i in ptn {
            cost += map[bef][i];
            bef = i;
        }

        cost += map[bef][0];
        if cost == k {
            res += 1;
        }
    }

    println!("{}", res);
}
