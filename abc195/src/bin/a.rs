fn main() {
  proconio::input! { m: usize, h: usize, };
  if 0 == h % m {
    println!("Yes");
  } else {
    println!("No");
  }
}
