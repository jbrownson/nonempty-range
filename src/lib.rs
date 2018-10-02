use std::ops::Range;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct NonemptyRange {
  start: usize,
  size_minus_one: usize
}

impl IntoIterator for NonemptyRange {
  type Item = usize;
  type IntoIter = Range<usize>;
  fn into_iter(self) -> Self::IntoIter { self.start..self.start + self.size_minus_one + 1 }
}

#[test]
fn test_nonempty_range_into_iterator() {
  assert!(
    NonemptyRange { start: 5, size_minus_one: 2 }
      .into_iter()
      .eq([5, 6, 7].into_iter().cloned())
  )
}