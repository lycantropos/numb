pub(super) type Sign = i8;
pub(super) type WindowDigit = u8;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BigInt<Digit, const SEPARATOR: char, const SHIFT: usize> {
    pub(super) sign: Sign,
    pub(super) digits: Vec<Digit>,
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> BigInt<Digit, SEPARATOR, SHIFT> {
    pub(crate) fn digits(&self) -> &[Digit] {
        &self.digits
    }
}
