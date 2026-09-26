use super::signed_num::SignedNum;

/// Basic float operations needed for geometric calculations
/// (floor, ceil, round, abs, sqrt, powi) - commonly needed operations
pub trait FloatBasic: SignedNum {
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn round(self) -> Self;
    fn abs(self) -> Self;
    fn sqrt(self) -> Self;
    fn powi(self, exp: i32) -> Self;
    fn epsilon() -> Self;
}

/// Expensive trigonometric and mathematical operations
pub trait FloatTrig: FloatBasic {
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn exp(self) -> Self;
    fn pi() -> Self;
}

/// Full float trait (for backwards compatibility)
pub trait Float: FloatTrig {
    // Empty - just combines both traits
}