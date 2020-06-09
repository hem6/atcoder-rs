#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        s:Chars,
        k: usize,
    }

    let alphabet = (b'a'..=b'z').map(char::from).collect::<Vec<char>>();
    let n = s.len();
    let mut cur = 0;

    for c in alphabet {
        let mut substrings: HashSet<String> = HashSet::new();

        for i in 0..n {
            if s[i] != c {
                continue;
            }

            for j in 0..5 {
                if i + j < n {
                    let substr: String = s[i..=i + j].iter().collect();
                    substrings.insert(substr);
                }
            }
        }

        let mut substrings: Vec<String> = substrings.into_iter().collect();
        substrings.sort();

        if cur + substrings.len() < k {
            cur += substrings.len();
        } else {
            let ans = &substrings[k - cur - 1];
            println!("{}", ans);
            return;
        }
    }
}
