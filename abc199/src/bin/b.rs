fn main() {
  proconio::input! {
    n: usize,
    a: [u32; n],
    b: [u32; n],
  };
  let bot: Option<u32> = a.into_iter().max();
  let top: Option<u32> = b.into_iter().min();
  if let (Some(bot), Some(top)) = (bot, top) {
    let ans = if bot <= top { 1 + top - bot } else { 0 };
    println!("{}", ans);
  }
}
