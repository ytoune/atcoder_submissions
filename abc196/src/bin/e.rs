#[proconio::fastout]
fn main() {
  proconio::input! {
    n: usize,
    at: [(i128, usize); n],
    q: usize,
    x: [i128; q],
  };
  for x in x.iter().copied() {
    let mut ans = x;
    for (a, t) in at.iter().copied() {
      ans = match t {
        1 => ans + a,
        2 => ans.max(a),
        3 => ans.min(a),
        _ => unreachable!(),
      };
    }
    println!("{}", ans);
  }
}
