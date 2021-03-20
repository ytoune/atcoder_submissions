fn main() {
  proconio::input! {
    top: u64,
  };
  let mut ans = 0;
  for n in 1.. {
    let m: u64 = format!("{}{}", n, n).parse().unwrap();
    if m <= top {
      ans += 1;
    } else {
      println!("{}", ans);
      return;
    }
  }
}
