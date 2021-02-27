fn main() {
  proconio::input! {
    a: usize, b: usize,
  };
  let b = a - b;
  println!("{}", (b as f64 / a as f64) * 100.0);
}
