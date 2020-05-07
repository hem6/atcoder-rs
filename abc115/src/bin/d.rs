#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        x: usize,
    }

    let mut len_a: Vec<usize> = (1..=n)
        .scan(1, |st, _| {
            *st = *st * 2 + 3;
            Some(*st)
        })
        .collect();
    len_a.insert(0, 1);

    let mut count_p: Vec<usize> = (1..=n)
        .scan(1, |st, _| {
            *st = *st * 2 + 1;
            Some(*st)
        })
        .collect();
    count_p.insert(0, 1);

    let ans = solve(n, x, &len_a, &count_p);
    println!("{}", ans);
}

fn solve(n: usize, x: usize, len_a: &Vec<usize>, count_p: &Vec<usize>) -> usize {
    if x == 0 {
        0
    } else if n == 0 {
        1
    } else if x <= len_a[n] / 2 {
        solve(n - 1, x - 1, &len_a, &count_p)
    } else if x == len_a[n] / 2 + 1 {
        count_p[n - 1] + 1
    } else {
        count_p[n - 1] + 1 + solve(n - 1, x - len_a[n - 1] - 2, &len_a, &count_p)
    }
}
