use core::convert::TryFrom;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Base trait for all numeric types.
pub trait Num:
    Default
    + PartialEq
    + PartialOrd
    + Copy
    + AddAssign
    + MulAssign
    + SubAssign
    + DivAssign
    + Add<Output = Self>
    + Mul<Output = Self>
    + Sub<Output = Self>
    + Div<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
    fn two() -> Self;
    fn four() -> Self;
    fn from_usize_checked(value: usize) -> Option<Self>;
    fn try_from_usize(value: usize) -> Result<Self, <Self as TryFrom<usize>>::Error>
    where
        Self: TryFrom<usize>,
    {
        Self::try_from(value)
    }
    fn get_max(self, b: Self) -> Self;
    fn get_min(self, b: Self) -> Self;
    fn saturating_sub(self, rhs: Self) -> Self;
    fn saturating_add(self, rhs: Self) -> Self;
    fn from_f32(value: f32) -> Self;
    fn to_f32(self) -> f32;
}
