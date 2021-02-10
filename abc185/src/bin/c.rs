#[proconio::fastout]
fn main() {
  proconio::input! {
    l: usize,
  };
  let mut ans = 1;
  for t in 1..=11 {
    ans *= l - t;
    ans /= t;
  }
  println!("{}", ans);
}
