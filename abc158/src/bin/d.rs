use std::collections::VecDeque;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut s: VecDeque<char> = s.trim_right().chars().collect();

    let mut q = String::new();
    std::io::stdin().read_line(&mut q).unwrap();
    let q: u64 = q.trim_right().parse().unwrap();

    let mut reversed = false;

    for _ in 0..q {
        let mut query = String::new();
        std::io::stdin().read_line(&mut query).unwrap();
        let query: Vec<&str> = query.trim_right().split_whitespace().collect();

        if query[0] == "1" {
            reversed = !reversed;
        }
        if query[0] == "2" {
            let push_front = query[1] == "1";
            if reversed ^ push_front {
                s.push_front(query[2].chars().next().unwrap());
            } else {
                s.push_back(query[2].chars().next().unwrap());
            }
        }
    }

    let mut res = String::new();
    if reversed {
        res.extend(s.iter().rev());
    } else {
        res.extend(s.iter());
    };

    println!("{}", res);
}
