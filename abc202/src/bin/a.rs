fn main() {
  proconio::input! { a: usize, b: usize, c: usize };
  let a = 7 - a;
  let b = 7 - b;
  let c = 7 - c;
  println!("{}", a + b + c);
}
