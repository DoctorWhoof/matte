use super::*;

#[test]
fn test_directional_methods_for_signed_integers() {
    // Test that up, left, right work for signed integers
    let up_i32: Vec2<i32> = Vec2::up();
    assert_eq!(up_i32, Vec2::new(0, -1));

    let left_i32: Vec2<i32> = Vec2::left();
    assert_eq!(left_i32, Vec2::new(-1, 0));

    let right_i32: Vec2<i32> = Vec2::right();
    assert_eq!(right_i32, Vec2::new(1, 0));

    // Test that they also work for floats
    let up_f32: Vec2<f32> = Vec2::up();
    assert_eq!(up_f32, Vec2::new(0.0, -1.0));

    let left_f32: Vec2<f32> = Vec2::left();
    assert_eq!(left_f32, Vec2::new(-1.0, 0.0));

    let right_f32: Vec2<f32> = Vec2::right();
    assert_eq!(right_f32, Vec2::new(1.0, 0.0));
}

#[test]
fn test_sub_assign_works_correctly() {
    let mut vec = Vec2::new(5.0, 3.0);
    vec -= Vec2::new(2.0, 1.0);
    assert_eq!(vec, Vec2::new(3.0, 2.0));

    let mut vec_i32 = Vec2::new(10, 7);
    vec_i32 -= Vec2::new(3, 2);
    assert_eq!(vec_i32, Vec2::new(7, 5));
}
