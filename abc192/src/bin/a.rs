fn main() {
  proconio::input! {
    x: usize,
  };
  println!("{}", 100 - (x % 100));
}
