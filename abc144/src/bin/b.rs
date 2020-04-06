use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    for i in 1..10 {
        for j in i..10 {
            if i * j == n {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
