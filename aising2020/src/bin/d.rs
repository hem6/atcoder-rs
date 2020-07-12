#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        x: Chars,
    }

    let mut popcount_x = 0;
    for i in 0..n {
        if x[i] == '1' {
            popcount_x += 1;
        }
    }

    let mut mod_plus = 0;
    let mut mod_minus = 0;

    for i in 0..n {
        let d = if x[i] == '1' { 1 } else { 0 };
        mod_plus = (mod_plus * 2 + d) % (popcount_x + 1);
        if popcount_x > 1 {
            mod_minus = (mod_minus * 2 + d) % (popcount_x - 1);
        }
    }

    let mut xs: Vec<u64> = vec![0; n];
    for i in 0..n {
        if x[i] == '0' {
            let res = (mod_plus + mod_pow(2, n - i - 1, popcount_x + 1)) % (popcount_x + 1);
            xs[i] = res as u64;
        } else if popcount_x > 1 {
            let diff = mod_pow(2, n - i - 1, popcount_x - 1) % (popcount_x - 1);
            let res = if mod_minus < diff {
                mod_minus + popcount_x - 1 - diff
            } else {
                mod_minus - diff
            };
            xs[i] = res as u64;
        } else {
            xs[i] = n as u64; // xi == 0になる目印
        }
    }

    let mut ops: Vec<i64> = vec![0; n + 1];
    ops[n] = -1;

    for i in 0..n {
        let mut i2 = i as u32;
        let mut op_count = 0;

        while i2 > 0 {
            if ops[i2 as usize] > 0 {
                op_count += ops[i2 as usize];
                i2 = 0;
            } else {
                i2 = i2 % i2.count_ones();
                op_count += 1;
            }
        }

        ops[i as usize] = op_count;
    }

    for i in 0..n {
        println!("{}", ops[xs[i] as usize] + 1);
    }
}

// aのn乗を求める。繰り返し二乗法によりO(logN)で計算できる
fn mod_pow(a: usize, n: usize, mod_n: usize) -> usize {
    if n == 0 {
        return 1;
    }

    let mut res = mod_pow(a, n / 2, mod_n);
    res = res * res % mod_n;

    if n % 2 == 1 {
        res = res * a % mod_n;
    }

    res
}
