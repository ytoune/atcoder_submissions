fn main() {
  use proconio::marker::Chars;
  proconio::input! { s: Chars };
  let mut ans = 0;
  use itertools::iproduct;
  for ns in iproduct!(0..10, 0..10, 0..10, 0..10).map(|(n1, n2, n3, n4)| [n1, n2, n3, n4]) {
    let has_bad = s.iter().enumerate().any(|(n, c)| match *c {
      'o' => ns.iter().all(|t| n != *t),
      'x' => ns.iter().any(|t| n == *t),
      '?' => false,
      _ => unreachable!(),
    });
    if has_bad {
      continue;
    }
    ans += 1;
  }
  println!("{}", ans);
}
