fn main() {
  proconio::input! { n: u128 };
  let mut d = 0;
  let mut ans = 0;
  let mut range = (0, 1000);
  loop {
    if range.0 <= n && n < range.1 {
      ans += d * (n - range.0 + 1);
      println!("{}", ans);
      return;
    }
    ans += d * (range.1 - range.0);
    range = (range.1, range.1 * 1000);
    d += 1;
  }
}
