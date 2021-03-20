#[proconio::fastout]
fn main() {
  proconio::input! {
    n: usize,
    at: [(i128, usize); n],
    q: usize,
    x: [i128; q],
  };
  let (low, high, add) = {
    let mut low: Option<i128> = None;
    let mut high: Option<i128> = None;
    let mut add = 0;
    for (a, t) in at.iter().copied() {
      match t {
        1 => {
          low = low.map(|low| low + a);
          high = high.map(|high| high + a);
          add += a;
        }
        2 => {
          low = low.map(|low| low.max(a));
          high = high.map(|high| high.max(a)).or_else(|| Some(a));
        }
        3 => {
          low = low.map(|low| low.min(a)).or_else(|| Some(a));
          high = high.map(|high| high.min(a));
        }
        _ => unreachable!(),
      };
    }
    (low, high, add)
  };
  for x in x.iter().copied() {
    let mut ans = x + add;
    if let Some(h) = high {
      ans = ans.max(h);
    }
    if let Some(l) = low {
      ans = ans.min(l);
    }
    println!("{}", ans);
  }
}
