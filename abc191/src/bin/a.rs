use proconio::input;

fn main() {
  input! {
    v: i64,
    t: i64,
    s: i64,
    d: i64,
  }
  if v * t <= d && d <= v * s {
    println!("No");
  } else {
    println!("Yes");
  }
}
