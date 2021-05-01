fn main() {
  use proconio::marker::Chars;
  proconio::input! { ipt: Chars };
  let mut ans = 0;
  let mut cnt = 0;
  for c in ipt {
    match (c, cnt) {
      ('Z', _) => {
        cnt = 1;
      }
      ('O', 1) => {
        cnt = 2;
      }
      ('N', 2) => {
        cnt = 3;
      }
      ('e', 3) => {
        cnt = 0;
        ans += 1;
      }
      _ => {
        cnt = 0;
      }
    }
  }
  println!("{}", ans);
}
