use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
        queries: [(usize, usize, String);q],
    }

    let mut alphabet: HashMap<char, BTreeSet<usize>> = HashMap::new();
    for i in 0..n {
        alphabet.entry(s[i]).or_insert(BTreeSet::new()).insert(i);
    }

    for (qtype, i, j) in queries {
        if qtype == 1 {
            let i = i - 1;
            let c = j.chars().next().unwrap();

            let bef = s[i];
            s[i] = c;

            let bef_set = alphabet.entry(bef).or_insert(BTreeSet::new());
            bef_set.remove(&i);

            let aft_set = alphabet.entry(c).or_insert(BTreeSet::new());
            aft_set.insert(i);
        } else {
            let l = i - 1;
            let r = j.parse::<usize>().unwrap() - 1;

            let mut count = 0;

            for (_, btset) in &alphabet {
                if btset.range(l..=r).next().is_some() {
                    count += 1;
                }
            }

            println!("{}", count);
        }
    }
}
