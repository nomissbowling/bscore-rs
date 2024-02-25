#![doc(html_root_url = "https://docs.rs/bscore/0.1.0")]
/// bscore bowling score for Rust
///

pub mod bgame;

#[cfg(test)]
mod tests {
  use super::*;
  use bgame::{bowling_score, bscore};

  #[test]
  fn test_scores() {
    assert_eq!(bscore("G/G/G/G/G/G/G/G/G/G/G", false).unwrap(), [100]);
    assert_eq!(bscore("xxxxxxxxxxxx", false).unwrap(), [300]);
    assert_eq!(bscore("xxxxxxxxxxxxxx", true).unwrap(), [300, 300, 300]);
    assert_eq!(bowling_score(false, "etc/scores.txt").unwrap(), ());
    // assert_eq!(bowling_score(true, "etc/scores.txt").unwrap(), ());
    // assert_eq!(bowling_score(true, "").unwrap(), ());
  }
}
