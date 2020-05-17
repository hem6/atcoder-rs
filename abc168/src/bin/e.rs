#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        iwashis: [(f64, f64);n]
    }

    let mut a_b: HashMap<NotNan<f64>, usize> = HashMap::new();
    for i in 0..n {
        let &(a, b) = &iwashis[i];
        *a_b.entry(NotNan::new(a / b).unwrap()).or_insert(0) += 1;
    }

    let mut nakawarui: HashMap<NotNan<f64>, usize> = HashMap::new();
    for i in 0..n {
        let (a, b) = iwashis[i];
        let minus_b_a = NotNan::new(-b / a).unwrap();
        *nakawarui.entry(minus_b_a).or_insert(0) += *a_b.get(&minus_b_a).unwrap_or(&0);
    }

    dbg!(&nakawarui);

    let mut temp: HashMap<usize, usize> = HashMap::new();

    for i in nakawarui.values() {
        *temp.entry(*i).or_insert(0) += 1;
    }

    let mut ans = 1;
    for i in temp {}
    ans = ans * (i + 1) % 1_000_000_007;
    println!("{}", ans);
}
