use std::cmp::Ordering;

use traiter::numbers::Zeroable;

use super::types::BigInt;

impl<
        Digit: Clone + Eq + PartialOrd + Zeroable,
        const SEPARATOR: char,
        const SHIFT: usize,
    > Ord for BigInt<Digit, SEPARATOR, SHIFT>
{
    fn cmp(&self, other: &Self) -> Ordering {
        if self.lt(other) {
            Ordering::Less
        } else if self.gt(other) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
