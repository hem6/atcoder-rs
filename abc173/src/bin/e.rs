#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

const MOD: i64 = 1_000_000_007;
const MAX: i64 = 1_000_000_001;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [i64; n],
    }

    a.sort();

    let mut negatives: Vec<i64> = Vec::new();
    let mut positives: Vec<i64> = Vec::new();

    for v in a {
        if v < 0 {
            negatives.push(v);
        } else {
            positives.push(v);
        }
    }

    positives.reverse();

    let mut ans = 1;
    let mut cur = 0;
    let mut neg_cur = 0;
    let mut pos_cur = 0;

    if positives.len() == 0 && k % 2 == 1 || k == n {
        negatives.reverse();

        while cur < k {
            let neg_v = negatives.get(neg_cur).unwrap_or(&MAX);
            let pos_v = positives.get(pos_cur).unwrap_or(&MAX);

            if neg_v.abs() < pos_v.abs() {
                ans = ans * neg_v % MOD;
                neg_cur += 1;
            } else {
                ans = ans * pos_v % MOD;
                pos_cur += 1;
            }

            cur += 1;
        }
    } else {
        while cur < k {
            let neg_v1 = negatives.get(neg_cur);
            let neg_v2 = negatives.get(neg_cur + 1);
            let pos_v1 = positives.get(pos_cur);
            let pos_v2 = positives.get(pos_cur + 1);

            let neg_v = if neg_v1.and(neg_v2).is_some() {
                neg_v1.unwrap() * neg_v2.unwrap()
            } else {
                -MAX * MAX
            };

            let pos_v = if pos_v1.and(pos_v2).is_some() {
                pos_v1.unwrap() * pos_v2.unwrap()
            } else {
                -MAX * MAX
            };

            if neg_v < pos_v || cur == k - 1 {
                ans = ans * pos_v1.unwrap() % MOD;
                pos_cur += 1;
                cur += 1;
            } else {
                ans = ans * (neg_v % MOD) % MOD;
                neg_cur += 2;
                cur += 2;
            }
        }
    }

    if ans < 0 {
        println!("{}", ans + MOD);
    } else {
        println!("{}", ans);
    }
}
