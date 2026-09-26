use super::*;

// Arithmetic operations
impl<T> Add<Rect<T>> for Rect<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy + PartialOrd,
{
    type Output = Self;

    fn add(self, other: Rect<T>) -> Self::Output {
        Rect {
            x: self.x + other.x,
            y: self.y + other.y,
            w: self.w,
            h: self.h,
        }
    }
}

impl<T> Sub<Rect<T>> for Rect<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy + PartialOrd,
{
    type Output = Self;

    fn sub(self, other: Rect<T>) -> Self::Output {
        Rect {
            x: self.x - other.x,
            y: self.y - other.y,
            w: self.w,
            h: self.h,
        }
    }
}

impl<T, V> Add<Vec2<V>> for Rect<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy + PartialOrd,
    V: Into<T>,
{
    type Output = Self;

    fn add(self, other: Vec2<V>) -> Self::Output {
        Rect {
            x: self.x + other.x.into(),
            y: self.y + other.y.into(),
            w: self.w,
            h: self.h,
        }
    }
}

impl<T, V> Sub<Vec2<V>> for Rect<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy + PartialOrd,
    V: Into<T>,
{
    type Output = Self;

    fn sub(self, other: Vec2<V>) -> Self::Output {
        Rect {
            x: self.x - other.x.into(),
            y: self.y - other.y.into(),
            w: self.w,
            h: self.h,
        }
    }
}

impl<T> Mul<T> for Rect<T>
where
    T: Mul<Output = T> + Copy + PartialOrd,
{
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        Rect { x: self.x * other, y: self.y * other, w: self.w, h: self.h }
    }
}
