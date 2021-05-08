fn main() {
  proconio::input! {
    a: [i64],
  };
  use std::collections::*;
  let mut map = HashMap::<i64, u128>::new();
  for num in a.iter().map(|&num| num % 200) {
    map
      .entry(num)
      .and_modify(|n| {
        *n += 1;
      })
      .or_insert(1);
  }
  let ans = map.values().map(|&n| n * (n - 1) / 2).sum::<u128>();
  println!("{}", ans);
}
