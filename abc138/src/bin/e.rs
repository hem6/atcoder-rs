#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        s:Chars,
        t:Chars,
    }

    let mut map: HashMap<char, Vec<usize>> = HashMap::new();

    let n = s.len();
    for i in 0..n {
        let c = s[i];
        map.entry(c).or_insert(Vec::new()).push(i + 1);
    }

    let mut ans = 0;
    let mut cur = 0;

    for c in t {
        match map.get_mut(&c) {
            None => {
                println!("-1");
                return;
            }
            Some(q) => {
                let i = q.lower_bound(&(cur + 1));

                if i < q.len() {
                    ans += q[i] - cur;
                    cur = q[i];
                } else {
                    ans += n - cur + q[0];
                    cur = q[0];
                }
            }
        }
    }

    println!("{}", ans);
}
