use proconio::input;

fn main() {
    input! {
        a:i32,
        b:i32,
        c:i32,
        x:i32,
        y:i32
    }

    let simple = x * a + y * b;
    let a_half = x * 2 * c + std::cmp::max((y - x) * b, 0);
    let b_half = y * 2 * c + std::cmp::max((x - y) * a, 0);

    println!("{}", [simple, a_half, b_half].iter().min().unwrap());
}
