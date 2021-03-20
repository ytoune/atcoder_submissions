fn main() {
  proconio::input! {
    x: String,
  };

  if x.chars().any(|c| c == '.') {
    let x = x.split('.').collect::<Vec<_>>();
    println!("{}", x[0]);
  } else {
    println!("{}", x);
  }
}
