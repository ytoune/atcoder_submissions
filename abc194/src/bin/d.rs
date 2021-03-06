fn main() {
  proconio::input! {
    n: usize,
  };
  let mut r = 0f64;
  let m = n as f64;
  for i in 1..n {
    let i = i as f64;
    r += m / i;
  }
  println!("{}", r);
}
