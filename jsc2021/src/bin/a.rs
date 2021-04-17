fn main() {
  proconio::input! {
    x: usize, y: usize, z: usize,
  };
  let t = y * z;
  let ans = t / x - (if 0 < t % x { 0 } else { 1 });
  println!("{}", ans);
}
