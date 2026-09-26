use super::*;
use core::ops::{Add, AddAssign, Sub, SubAssign};

impl<T> Add<Vec2<T>> for Vec2<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy + PartialOrd,
{
    type Output = Self;

    fn add(self, other: Vec2<T>) -> Self::Output {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
}

impl<T> Sub<Vec2<T>> for Vec2<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy + PartialOrd,
{
    type Output = Self;

    fn sub(self, other: Vec2<T>) -> Self::Output {
        Vec2 { x: self.x - other.x, y: self.y - other.y }
    }
}

impl<T> AddAssign<Vec2<T>> for Vec2<T>
where
    T: Add<Output = T> + AddAssign + Copy + PartialOrd,
{
    fn add_assign(&mut self, other: Vec2<T>) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T> SubAssign<Vec2<T>> for Vec2<T>
where
    T: Sub<Output = T> + SubAssign + Copy + PartialOrd,
{
    fn sub_assign(&mut self, other: Vec2<T>) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
