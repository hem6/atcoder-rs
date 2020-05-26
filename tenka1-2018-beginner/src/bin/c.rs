#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        mut a: [i64;n],
    }

    let mut small_start = vec![-1];
    let mut sign = 1;
    for _ in 0..n - 2 {
        small_start.push(sign * 2);
        sign *= -1;
    }
    small_start.push(sign);
    small_start.sort();

    let mut large_start = vec![1];
    let mut sign = -1;
    for _ in 0..n - 2 {
        large_start.push(sign * 2);
        sign *= -1;
    }
    large_start.push(sign);
    large_start.sort();

    a.sort();

    let mut ans_1 = 0;
    let mut ans_2 = 0;

    for i in 0..n {
        ans_1 += a[i] * large_start[i];
        ans_2 += a[i] * small_start[i];
    }

    println!("{}", max(ans_1, ans_2));
}
