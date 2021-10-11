use crate::traits::{BitLength, Oppose, OppositionOf, Zeroable};

pub(crate) const fn are_same<T, U>() -> bool {
    trait SameTo<U> {
        const VALUE: bool;
    }

    impl<T, U> SameTo<U> for T {
        default const VALUE: bool = false;
    }

    impl<T> SameTo<T> for T {
        const VALUE: bool = true;
    }

    <T as SameTo<U>>::VALUE
}

pub(crate) const fn floor_log(value: usize, base: usize) -> Result<usize, &'static str> {
    if value == 0usize {
        Err("Logarithm of zero is undefined.")
    } else if value < base {
        Ok(0)
    } else {
        match floor_log(value / base, base) {
            Ok(value) => Ok(value + 1),
            error => error,
        }
    }
}

pub(crate) fn floor_log2<T: BitLength<Output = usize> + Zeroable>(value: T) -> usize {
    debug_assert!(!value.is_zero());
    value.bit_length() - 1
}

pub(crate) const fn is_signed<T: Oppose>() -> bool {
    are_same::<T, OppositionOf<T>>()
}

pub(crate) const fn is_unsigned<T: Oppose>() -> bool {
    !are_same::<T, OppositionOf<T>>()
}

pub(crate) const fn power(base: usize, exponent: usize) -> usize {
    match exponent {
        0 => 1,
        _ => base * power(base, exponent - 1),
    }
}
