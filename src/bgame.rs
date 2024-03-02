//! bgame
//!

use std::fmt;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read, BufRead, BufReader};
use std::collections::VecDeque;

/// BScore
#[derive(Debug)]
pub struct BScore {
  /// score
  pub s: Vec<String>,
  /// pin
  pub p: i32
}

/// Display
impl fmt::Display for BScore {
  /// format
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.s.iter().map(|l|
      format!("{}\x0A", l)).collect::<Vec<_>>().join(""))
  }
}

/// BScore
impl BScore {
  /// constructor
  pub fn new(s: Vec<String>, p: i32) -> BScore {
    BScore{s, p}
  }
}

/// Disp
trait Disp {
  /// format
  fn disp(&self, f: &mut fmt::Formatter, p: bool) -> fmt::Result;
}

/// Disp for Vec BScore
impl Disp for Vec<BScore> {
  /// disp format
  fn disp(&self, f: &mut fmt::Formatter, p: bool) -> fmt::Result {
    write!(f, "{}", self.iter().map(|s|
      match p {
      false => s.to_string(),
      true => format!("{}{}\x0A", s, s.p)
      }).collect::<Vec<_>>().join(""))
  }
}

/// VecBScore
#[derive(Debug)]
pub struct VecBScore {
  /// scores Vec BScore
  pub v: Vec<BScore>,
  /// pin flag (false: only s, true: s and p)
  pub p: bool
}

/// Display
impl fmt::Display for VecBScore {
  /// format
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.v.disp(f, self.p)
  }
}

/// VecBScore
impl VecBScore {
  /// constructor
  pub fn new(v: Vec<BScore>) -> VecBScore {
    VecBScore{v, p: false}
  }
}

/// BFrame
#[derive(Debug, Clone)]
pub struct BFrame {
  /// num
  pub n: usize,
  /// pin
  pub p: i32,
  /// first
  pub f: i32,
  /// second
  pub s: i32,
  /// mark (0: no, 1: spare, 2: strike)
  pub m: i32
}

/// Display
impl fmt::Display for BFrame {
  /// format
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // write!(f, "{:?}", self)
    write!(f, "({}, {})", self.f, self.s)
  }
}

/// BFrame
impl BFrame {
  /// constructor
  pub fn new() -> BFrame {
    BFrame{n: 0, p: -1, f: -1, s: -1, m: 0}
  }

  /// calc
  /// - q: VecDeque BFrame
  /// - f: frame num
  /// - Result: score (single frame)
  pub fn calc(&self, q: &VecDeque<BFrame>, f: usize) ->
    Result<i32, Box<dyn Error>> {
    let mut p = self.f;
    if p < 10 {
      if self.s < 0 { return Err("no throw second".into()); }
      p += self.s;
    }
    if self.m > 0 {
      let u = q.get(f + 1).ok_or("no throw after mark")?;
      p += u.f;
      if self.m == 2 {
        if u.f < 10 {
          if u.s < 0 { return Err("no throw second after x".into()); }
          p += u.s;
        } else {
          let v = q.get(f + 2).ok_or("no throw after xx")?;
          p += v.f;
        }
      }
    }
    Ok(p)
  }

  /// c (to char as &str)
  /// - q: VecDeque BFrame
  /// - f: frame num
  /// - p: phase (0: first, 1: second, 2: after xx)
  /// - Result: char of single throw
  pub fn c(&self, q: &VecDeque<BFrame>, f: usize, p: i32) -> &str {
    if f < 9 && p == 2 { return ""; }
    let dum = BFrame::new();
    let e = q.get(f + 1).or(Some(&dum)).unwrap(); // eleventh dummy
    let t = q.get(f + 2).or(Some(&dum)).unwrap(); // twelfth dummy
    let d = if p == 0 { self.f } else {
      if p == 1 {
        if self.f < 10 { self.s } else { if f < 9 { self.f } else { e.f } }
      } else {
        if self.m == 0 { -1 } else {
          if self.m == 1 { e.f } else {
            if e.f < 10 { e.s } else { t.f }
          }
        }
      }
    };
    if d == 0 {
      if f == 9 {
        if p == 1 && self.m == 2 { return "G"; }
        if p == 2 && self.m == 1 { return "G"; }
        if p == 2 && self.m == 2 && e.m == 2 { return "G"; }
      }
      return if p == 0 { "G" } else { "-" };
    }
    if p == 2 && self.m > 0 && e.m == 1 { return "/"; }
    if self.m == 1 && p == 1 { return "/"; }
    if d == 10 { return if f < 9 && p == 0 { " " } else { "x" }; }
    if d < 0 { return ""; }
    "123456789".split("").nth(d as usize).unwrap() // not "0123456789"
  }

  /// d (display)
  /// - q: VecDeque BFrame
  /// - Result: string of single frame
  pub fn d(&self, q: &VecDeque<BFrame>) -> Result<String, Box<dyn Error>> {
    Ok((0..3).into_iter().map(|p|
      self.c(q, self.n, p)).collect::<Vec<_>>().join(""))
  }
}

/// BGame
#[derive(Debug)]
pub struct BGame {
  /// que VecDeque BFrame
  pub q: VecDeque<BFrame>
}

/// Display
impl fmt::Display for BGame {
  /// format
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self.q)
  }
}

/// BGame
impl BGame {
  /// construct
  pub fn new() -> BGame {
    BGame{q: VecDeque::<BFrame>::new()}
  }

  /// construct BFrame and append to vecdeque
  /// - p: mut phase (false: first throw, true: second throw)
  /// - d: decimal pins
  pub fn frm(&mut self, p: &mut bool, d: i32) {
    let mut w: BFrame;
    if !*p {
      w = BFrame::new();
      w.f = d;
      *p = d < 10;
    } else {
      w = self.q.pop_back().unwrap(); // always exists
      w.s = d;
      *p = false;
    }
    if w.f == 10 { w.m = 2; }
    else if w.f + w.s == 10 { w.m = 1; }
    else { w.m = 0; }
    self.q.push_back(w);
  }

  /// calc score
  /// - Result: score
  pub fn calc_score(&mut self) -> Result<BScore, Box<dyn Error>> {
    let mut p = 0i32;
    let mut s: Vec<String> = vec![];
    for i in 0..self.q.len() { // not use enumerate (i, f) to avoid iter_mut
      let mut f = self.q[i].clone(); // get clone to avoid iter_mut
      let t = if i == 9 { "" } else { " " };
      f.n = i;
      f.p = f.calc(&self.q, i)? + if i > 0 { self.q[i - 1].p } else { 0 };
      p = f.p;
      // s.push(format!("{}{}", t, f));
      s.push(format!("{}{}", t, f.d(&self.q)?));
      *self.q.get_mut(i).ok_or("not found")? = f; // must be after access to f
      if i == 9 { break; }
    }
    Ok(BScore::new(vec![
      s.into_iter().collect::<Vec<_>>().join(" "),
      (0..10).into_iter().map(|i|
        format!("{:3}", self.q[i].p)).collect::<Vec<_>>().join(" ")
    ], p))
  }
}

/// bscore
/// - txt: single line (trim comments etc)
/// - mode: false (normal), true (shift score when extra frames)
/// - Result: [`single`] or [`multi`] scores
pub fn bscore(txt: &str, mode: bool) -> Result<Vec<i32>, Box<dyn Error>> {
  let scores = getscore(txt, mode)?;
  print!("{}", scores);
  Ok(scores.v.iter().map(|s| s.p).collect())
}

/// get score
/// - txt: single line (trim comments etc)
/// - mode: false (normal), true (shift score when extra frames)
/// - Result: [`single`] or [`multi`] scores
pub fn getscore(txt: &str, mode: bool) -> Result<VecBScore, Box<dyn Error>> {
  // println!("{}", txt);
  let mut g = BGame::new();
  let mut p = false;
  for c in txt.chars() {
    if '0' <= c && c <= '9' { g.frm(&mut p, c as i32 - '0' as i32); }
    if c == '-' || c == 'G' || c == 'F' { g.frm(&mut p, 0); }
    if c == '/' {
      if !p { return Err("first / is not allowed".into()); }
      else { g.frm(&mut p, 10 - g.q[g.q.len() - 1].f); }
    }
    if c == 'x' || c == 'X' {
      if p { return Err("second x is not allowed".into()); }
      else { g.frm(&mut p, 10); }
    }
  }
  let mut v: Vec<BScore> = vec![];
  let mut first = true;
  loop {
    let f = match g.q.get(9) {
    None => if first { return Err("too few frames".into()); } else { break; },
    Some(f) => f
    };
    let _ = match f.calc(&g.q, 9) {
    Err(e) => if first { return Err(e); } else { break; },
    Ok(p) => p
    };
    v.push(g.calc_score()?);
    if !mode { break; }
    if let Some(_) = g.q.pop_front() { first = false; } else { break; }
  }
  Ok(VecBScore::new(v))
}

/// parse lines
/// - mode: false (normal), true (shift score when extra frames)
/// - f: io::Stdin or File (trait io::Read)
pub fn parselines<T: Read>(mode: bool, f: T) -> Result<(), Box<dyn Error>> {
  let mut rdr = BufReader::new(f);
  let mut r = String::new();
  while rdr.read_line(&mut r)? > 0 {
    let l = r.trim(); // trim_end();
    if l.len() > 0 { // .chars().count()
      match l.find('#') {
      None => { bscore(l, mode)?; }, // line
      Some(i) => if i > 0 { bscore(&l[..i], mode)?; } // cut comment
      }
    }
    r.clear();
  }
  Ok(())
}

/// bowling score
/// - mode: false (normal), true (shift score when extra frames)
/// - fname: file name or "" from io::stdin()
pub fn bowling_score(mode: bool, fname: &str) -> Result<(), Box<dyn Error>> {
  match fname {
  "" => parselines(mode, io::stdin()),
  _ => parselines(mode, File::open(fname)?)
  }
}
