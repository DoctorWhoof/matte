use super::num::Num;
use core::ops::Neg;

/// A trait for numeric types that support negation (signed integers and floats).
pub trait SignedNum: Num + Neg<Output = Self> {}
