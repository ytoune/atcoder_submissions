fn main() {
  proconio::input! {
    arr: [usize; 4],
  };
  let r = arr.iter().min().unwrap();
  println!("{}", r);
}
