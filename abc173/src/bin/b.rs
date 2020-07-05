#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        results: [String;n],
    }

    let mut count: HashMap<String, u64> = HashMap::new();

    for res in results {
        *count.entry(res).or_insert(0) += 1;
    }

    println!("AC x {}", count.get("AC").unwrap_or(&0));
    println!("WA x {}", count.get("WA").unwrap_or(&0));
    println!("TLE x {}", count.get("TLE").unwrap_or(&0));
    println!("RE x {}", count.get("RE").unwrap_or(&0));
}
