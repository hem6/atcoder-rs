#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        mut a: [u64;n]
    }

    a.sort();
    let mut ans: Vec<u64> = Vec::new();
    let mut exclude: HashSet<u64> = HashSet::new();

    for i in a {
        let mut flag = false;

        if ans.last() == Some(&i) {
            exclude.insert(i);
            continue;
        }

        let divs = divisor(i);
        for v in divs {
            if ans.binary_search(&v).is_ok() {
                flag = true;
                break;
            }
        }

        if !flag {
            ans.push(i);
        }
    }

    println!("{}", ans.len() - exclude.len());
}

fn divisor(n: u64) -> Vec<u64> {
    let mut res = vec![];
    let mut i = 1;

    while i * i <= n {
        if n % i == 0 {
            res.push(i);
            if i != n / i {
                res.push(n / i);
            }
        }
        i += 1;
    }

    res.sort();
    res
}
