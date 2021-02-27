#[proconio::fastout]
fn main() {
  proconio::input! {
    t: usize,
    cases: [(u128, u128, u128, u128); t],
  };
  for (x, y, p, q) in cases {
    println!("{}", calc(x, y, p, q));
  }
}

fn calc(x: u128, y: u128, p: u128, q: u128) -> String {
  let mut nv: Option<u128> = Some(0);
  let mut mv: Option<u128> = Some(0);
  let x2 = x.checked_mul(2);
  let y2 = y.checked_mul(2);
  let x2y2 = x2.and_then(|x2| y2.and_then(|y2| x2.checked_add(y2)));
  let x2y = x2.and_then(|x2| x2.checked_add(y));
  let pq = p.checked_add(q);
  if let (Some(x2y2), Some(x2y), Some(pq)) = (x2y2, x2y, pq) {
    while let (Some(n), Some(m)) = (nv, mv) {
      let t1s = n.checked_mul(x2y2).and_then(|t| t.checked_add(x2y));
      let t1e = n.checked_add(1).and_then(|n1| x2y2.checked_mul(n1));
      let t2s = m.checked_mul(pq).and_then(|t| t.checked_add(p));
      let t2e = m.checked_add(1).and_then(|m1| pq.checked_mul(m1));
      if let (Some(t1s), Some(t1e), Some(t2s), Some(t2e)) = (t1s, t1e, t2s, t2e) {
        if t2e < t1s {
          mv = m.checked_add(1);
          continue;
        }
        if t1e < t2s {
          nv = n.checked_add(1);
          continue;
        }
        return format!("{}", if t1s > t2s { t1s } else { t2s });
      } else {
        break;
      }
    }
  }
  "infinity".into()
}
