#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a:[u64;n],
    }

    let mut count: HashMap<u64, i64> = HashMap::new();
    for i in a {
        *count.entry(i).or_insert(0) += 1;
    }

    match count.len() {
        1 => {
            if count.get(&0).is_some() {
                println!("Yes");
            } else {
                println!("No");
            }
        }
        2 => {
            if n % 3 == 0 && count.get(&0) == Some(&(n as i64 / 3)) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
        3 => {
            let keys: Vec<&u64> = count.keys().collect();

            if n % 3 == 0
                && count.values().all(|&c| c as usize == n / 3)
                && keys[0] ^ keys[1] == *keys[2]
            {
                println!("Yes");
            } else {
                println!("No");
            }
        }
        _ => println!("No"),
    }
}
