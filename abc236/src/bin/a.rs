fn main() {
  use proconio::marker::{Chars, Usize1};
  proconio::input! {
    mut s: Chars,
    a: Usize1, b: Usize1,
  };
  let tmp = s[a];
  s[a] = s[b];
  s[b] = tmp;
  println!("{}", s.iter().collect::<String>());
}
