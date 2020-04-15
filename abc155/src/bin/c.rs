#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        ss: [String; n]
    }

    let mut poll: HashMap<String, i32> = HashMap::new();
    for s in ss {
        poll.entry(s).and_modify(|i| *i += 1).or_insert(1);
    }

    let mut ans: HashMap<i32, Vec<String>> = HashMap::new();
    for (k, v) in poll {
        let vec = ans.entry(v).or_insert(vec![]);
        vec.push(k);
    }

    let mut ans: Vec<(i32, Vec<String>)> = ans.into_iter().collect();
    ans.sort_by_key(|v| Reverse(v.0));

    let words = &mut ans[0].1;
    words.sort();
    for w in words {
        println!("{}", w);
    }
}
