#[proconio::fastout]
fn main() {
  proconio::input! {
    n: usize, m: usize,
    s: [String; n],
    t: [String; m],
  };
  let t = {
    use std::collections::HashSet;
    let mut t2 = HashSet::new();
    for u in t {
      t2.insert(u);
    }
    t2
  };
  for tmp in s {
    let ans = if t.contains(&tmp) { "Yes" } else { "No" };
    println!("{}", ans);
  }
}
