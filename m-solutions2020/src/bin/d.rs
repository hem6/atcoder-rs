#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [u64;n],
    }

    let mut money = 1000;
    let mut stock = 0;

    for i in 0..n - 1 {
        if a[i] < a[i + 1] && money >= a[i] {
            stock += money / a[i];
            money -= stock * a[i];
        }
        if a[i] > a[i + 1] && stock > 0 {
            money += stock * a[i];
            stock = 0;
        }
    }

    money += stock * a[n - 1];
    println!("{}", money);
}
