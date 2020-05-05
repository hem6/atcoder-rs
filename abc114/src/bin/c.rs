#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;

    dfs(0, 7, n, &mut ans);
    dfs(0, 5, n, &mut ans);
    dfs(0, 3, n, &mut ans);

    println!("{}", ans);
}

fn dfs(depth: usize, v: usize, n: usize, ans: &mut usize) {
    if depth > 9 {
        return;
    }

    let mut flags: HashMap<usize, bool> = HashMap::new();
    flags.insert(7, false);
    flags.insert(5, false);
    flags.insert(3, false);

    let mut num = v;
    while num > 0 {
        flags.insert(num % 10, true);
        num /= 10;
    }

    if flags.iter().all(|(_, &f)| f) && v <= n {
        *ans += 1;
    }

    dfs(depth + 1, v * 10 + 7, n, ans);
    dfs(depth + 1, v * 10 + 5, n, ans);
    dfs(depth + 1, v * 10 + 3, n, ans);
}
