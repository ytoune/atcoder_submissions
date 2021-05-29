fn main() {
  proconio::input! {
    n: usize,
    dogs: [(i64, char); 2 * n],
  };
  let dogs = {
    let mut dogseped = [vec![], vec![], vec![]];
    for d in dogs {
      let k = match d.1 {
        'R' => 0,
        'G' => 1,
        'B' => 2,
        _ => unreachable!(),
      };
      dogseped[k].push(d.0);
    }
    dogseped.sort_by_cached_key(|d| d.len() % 2);
    dogseped
  };
  if 0 == dogs[2].len() % 2 {
    println!("0");
    return;
  }
  let po = find(&dogs, 1, 2).unwrap();
  if 0 == po || dogs[0].is_empty() {
    println!("{}", po);
    return;
  }
  let p0 = find(&dogs, 0, 1).unwrap();
  let p1 = find(&dogs, 0, 2).unwrap();
  println!("{}", (p0 + p1).min(po));
}

fn find(dogs: &[Vec<i64>; 3], u0: usize, u1: usize) -> Option<i64> {
  let mut i: [usize; 3] = [0; 3];
  let mut ans: Option<i64> = None;
  loop {
    if let Some(po) = pick(dogs, u0, i[u0], u1, i[u1]) {
      if 0 == po {
        return Some(0);
      }
      ans = ans.map(|m| m.min(po)).or_else(|| Some(po));
      let mut min: Option<(i64, usize)> = None;
      if let Some(p) = pick(dogs, u0, i[u0] + 1, u1, i[u1]) {
        min = Some((p, u0));
      }
      if let Some(p) = pick(dogs, u0, i[u0], u1, i[u1] + 1) {
        min = min.map(|m| (p, u1).min(m)).or_else(|| Some((p, u1)));
      }
      if let Some(min) = min {
        i[min.1] += 1;
        continue;
      }
    }
    return ans;
  }
}

fn pick(dogs: &[Vec<i64>; 3], u0: usize, i0: usize, u1: usize, i1: usize) -> Option<i64> {
  dogs[u0]
    .get(i0)
    .and_then(|d0| dogs[u1].get(i1).map(|d1| (d0 - d1).abs()))
}
