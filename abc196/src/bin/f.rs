fn main() {
  use proconio::marker::Chars;
  proconio::input! {
    s: Chars,
    t: Chars,
  };
  let mut dp = vec![vec![0; s.len() + 1]; t.len() + 1];
}
