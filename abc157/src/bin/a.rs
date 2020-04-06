use proconio::input;

fn main() {
    input! {
        n: f32,
    }

    println!("{}", (n / 2.0).ceil());
}
