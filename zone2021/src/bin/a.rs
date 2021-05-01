fn main() {
  proconio::input! { ipt: String };
  let ans = ipt.match_indices("ZONe").count();
  println!("{}", ans);
}
